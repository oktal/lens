//! Module that provides a component to run queries in a streaming fashion

use std::{collections::HashMap, sync::Arc};

use datafusion::{
    arrow::{
        array::AsArray,
        datatypes::{SchemaRef, UInt64Type},
        error::ArrowError,
        util::display::{ArrayFormatter, FormatOptions},
    },
    config::CsvOptions,
    datasource::{file_format::format_as_file_type, provider_as_source},
    error::DataFusionError,
    execution::{context::SessionState, SendableRecordBatchStream},
    logical_expr::LogicalPlanBuilder,
    prelude::*,
    sql::TableReference,
};
use futures::StreamExt;
use thiserror::Error;
use tokio::sync::{mpsc, oneshot};

use crate::{
    common::{self, ExportFormat, ExportOptions, StreamId},
    lens::LensResult,
};

use super::memtable::MemTable;

/// Error that can occur during query stream manipulation
#[derive(Debug, Error)]
pub enum StreamError {
    #[error("unknown stream {0}")]
    UnknownStream(StreamId),

    #[error(transparent)]
    DataFusion(#[from] DataFusionError),

    #[error(transparent)]
    Arrow(#[from] ArrowError),
}

/// Result aliased type for stream operations
pub type StreamResult<T, E = StreamError> = std::result::Result<T, E>;

/// Describes an operation that can be executed through a [`QueryStreamer`]
pub enum QueryStreamRequest {
    /// Initate a new stream
    Initiate {
        /// Query to initiate the stream from
        sql: String,

        /// Channel on which to send the result of the newly initiated stream
        resp_tx: oneshot::Sender<LensResult<StreamId>>,
    },

    /// Retrieve the next records from a given [`StreamId`]
    Next {
        id: StreamId,

        /// Channel on which to send data fetched from the stream
        resp_tx: oneshot::Sender<LensResult<Option<Vec<common::Row>>>>,
    },

    /// Export the given [`StreamId`] to the location specified by [`ExportOptions`]
    Export {
        id: StreamId,

        options: ExportOptions,

        resp_tx: oneshot::Sender<LensResult<usize>>,
    },

    /// Close and remove from memory any data fetched by a given [`StreamId`]
    Close {
        id: StreamId,

        resp_tx: oneshot::Sender<LensResult<()>>,
    },

    /// List all the active streams
    List {
        resp_tx: oneshot::Sender<LensResult<Vec<common::StreamInfo>>>,
    },
}

impl QueryStreamRequest {
    pub fn create(sql: impl Into<String>) -> (Self, oneshot::Receiver<LensResult<StreamId>>) {
        let sql = sql.into();
        let (resp_tx, resp_rx) = oneshot::channel();

        (Self::Initiate { sql, resp_tx }, resp_rx)
    }

    pub fn next(
        id: StreamId,
    ) -> (
        Self,
        oneshot::Receiver<LensResult<Option<Vec<common::Row>>>>,
    ) {
        let (resp_tx, resp_rx) = oneshot::channel();
        (Self::Next { id, resp_tx }, resp_rx)
    }

    pub fn export(
        id: StreamId,
        options: ExportOptions,
    ) -> (Self, oneshot::Receiver<LensResult<usize>>) {
        let (resp_tx, resp_rx) = oneshot::channel();
        (
            Self::Export {
                id,
                options,
                resp_tx,
            },
            resp_rx,
        )
    }

    pub fn close(id: StreamId) -> (Self, oneshot::Receiver<LensResult<()>>) {
        let (resp_tx, resp_rx) = oneshot::channel();
        (Self::Close { id, resp_tx }, resp_rx)
    }

    pub fn list() -> (Self, oneshot::Receiver<LensResult<Vec<common::StreamInfo>>>) {
        let (resp_tx, resp_rx) = oneshot::channel();
        (Self::List { resp_tx }, resp_rx)
    }
}

struct StreamEntry {
    id: StreamId,
    query: String,
    stream: SendableRecordBatchStream,

    table: Arc<MemTable>,
}

impl StreamEntry {
    fn new(
        schema: SchemaRef,
        id: StreamId,
        query: String,
        stream: SendableRecordBatchStream,
    ) -> Self {
        Self {
            id,
            query,
            stream,
            table: Arc::new(MemTable::new(schema)),
        }
    }

    fn scan(&self, session: SessionState) -> StreamResult<DataFrame> {
        let table_ref = TableReference::bare(self.id.0.to_string());
        let plan = LogicalPlanBuilder::scan(
            table_ref,
            provider_as_source(Arc::clone(&self.table) as _),
            None,
        )?
        .build()?;

        Ok(DataFrame::new(session, plan))
    }
}

pub struct QueryStreamer {
    ctx: SessionContext,
    streams: HashMap<StreamId, StreamEntry>,

    reqs_rx: mpsc::Receiver<QueryStreamRequest>,
}

impl QueryStreamer {
    pub fn new(ctx: SessionContext) -> (Self, mpsc::Sender<QueryStreamRequest>) {
        let (reqs_tx, reqs_rx) = mpsc::channel(128);

        (
            Self {
                ctx,
                streams: HashMap::new(),
                reqs_rx,
            },
            reqs_tx,
        )
    }

    pub async fn run(mut self) {
        'outer: loop {
            tokio::select! {
                req = self.reqs_rx.recv() => {
                    let Some(req) = req else {
                        break 'outer;
                    };

                    self.handle_request(req).await;
                }
            }
        }
    }

    async fn handle_request(&mut self, req: QueryStreamRequest) {
        match req {
            QueryStreamRequest::Initiate { sql, resp_tx } => {
                let _ = resp_tx.send(self.create(sql).await);
            }

            QueryStreamRequest::Next { id, resp_tx } => {
                let _ = resp_tx.send(self.next(id).await.map_err(Into::into));
            }

            QueryStreamRequest::Export {
                id,
                options,
                resp_tx,
            } => {
                let _ = resp_tx.send(self.export(id, options).await.map_err(Into::into));
            }

            QueryStreamRequest::Close { id, resp_tx } => {
                let _ = resp_tx.send(self.close(id).map_err(Into::into));
            }

            QueryStreamRequest::List { resp_tx } => {
                let _ = resp_tx.send(self.list().map_err(Into::into));
            }
        }
    }

    async fn create(&mut self, query: String) -> LensResult<StreamId> {
        let df = self.ctx.sql(&query).await?;

        let schema = Arc::clone(df.schema().inner());

        let stream = df.execute_stream().await?;
        let stream_id = StreamId::new();

        let entry = StreamEntry::new(schema, stream_id, query, stream);

        self.streams.insert(stream_id, entry);
        Ok(stream_id)
    }

    async fn next(&mut self, id: StreamId) -> StreamResult<Option<Vec<common::Row>>> {
        let Some(entry) = self.streams.get_mut(&id) else {
            return Err(StreamError::UnknownStream(id));
        };

        let Some(batch) = entry.stream.next().await else {
            return Ok(None);
        };

        let batch = batch?;

        entry.table.insert(batch.clone());

        let schema = batch.schema();
        let columns = schema
            .fields()
            .iter()
            .map(|f| f.name())
            .cloned()
            .collect::<Vec<_>>();

        let options = FormatOptions::default().with_display_error(true);
        let formatters = batch
            .columns()
            .iter()
            .map(|c| ArrayFormatter::try_new(c.as_ref(), &options))
            .collect::<Result<Vec<_>, _>>()?;

        let rows = (0..batch.num_rows())
            .map(|row| common::Row {
                columns: columns.clone(),
                values: formatters
                    .iter()
                    .map(|f| f.value(row).to_string())
                    .collect(),
            })
            .collect();

        Ok(Some(rows))
    }

    async fn export(&mut self, id: StreamId, options: ExportOptions) -> StreamResult<usize> {
        let Some(entry) = self.streams.get(&id) else {
            return Err(StreamError::UnknownStream(id));
        };

        let df = entry.scan(self.ctx.state())?;

        let batches = match options.format {
            ExportFormat::Csv => {
                let csv_options = CsvOptions::default().with_has_header(true);
                let write_options = options.write_options.into();
                df.write_csv(&options.path, write_options, Some(csv_options))
                    .await?
            }
            ExportFormat::Parquet => {
                df.write_parquet(&options.path, options.write_options.into(), None)
                    .await?
            }
            ExportFormat::Json => {
                // NOTE(oktal): we do not directly call `DataFrame::write_json` because we can not
                // use our own serialization format.
                // Instead, we create a logicial plan with our own json serialization format
                let format = super::export::json::factory();
                let file_type = format_as_file_type(format);

                let plan = LogicalPlanBuilder::copy_to(
                    df.into_unoptimized_plan(),
                    options.path.clone(),
                    file_type,
                    Default::default(),
                    options.write_options.partition_by,
                )?
                .build()?;

                DataFrame::new(self.ctx.state(), plan).collect().await?
            }
        };

        let count = batches.first().and_then(|batch| {
            batch.column_by_name("count").and_then(|col| {
                col.as_primitive_opt::<UInt64Type>()
                    .and_then(|arr| arr.values().get(0).copied())
            })
        });

        Ok(count.unwrap_or(0) as usize)
    }

    fn close(&mut self, id: StreamId) -> StreamResult<()> {
        self.streams
            .remove(&id)
            .ok_or(StreamError::UnknownStream(id))?;
        Ok(())
    }

    fn list(&self) -> StreamResult<Vec<common::StreamInfo>> {
        Ok(self
            .streams
            .iter()
            .map(|(id, entry)| common::StreamInfo {
                id: *id,
                query: entry.query.clone(),
                rows: entry.table.num_rows(),
            })
            .collect())
    }
}
