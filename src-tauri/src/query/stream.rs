//! Module that provides a component to run queries in a streaming fashion

use std::collections::HashMap;

use datafusion::{
    arrow::{
        error::ArrowError,
        util::display::{ArrayFormatter, FormatOptions},
    },
    error::DataFusionError,
    execution::SendableRecordBatchStream,
    prelude::*,
};
use futures::StreamExt;
use thiserror::Error;
use tokio::sync::{mpsc, oneshot};

use crate::{
    common::{self, StreamId},
    lens::LensResult,
};

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
}

pub struct QueryStreamer {
    ctx: SessionContext,
    streams: HashMap<StreamId, SendableRecordBatchStream>,

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
                let _ = resp_tx.send(self.stream_create(sql).await);
            }

            QueryStreamRequest::Next { id, resp_tx } => {
                let _ = resp_tx.send(self.stream_next(id).await.map_err(Into::into));
            }
        }
    }

    async fn stream_create(&mut self, query: String) -> LensResult<StreamId> {
        let df = self.ctx.sql(&query).await?;
        let stream = df.execute_stream().await?;
        let stream_id = StreamId::new();
        self.streams.insert(stream_id.clone(), stream);
        Ok(stream_id)
    }

    async fn stream_next(&mut self, id: StreamId) -> StreamResult<Option<Vec<common::Row>>> {
        let Some(stream) = self.streams.get_mut(&id) else {
            return Err(StreamError::UnknownStream(id));
        };

        if let Some(item) = stream.next().await {
            let batch = item?;

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
        } else {
            Ok(None)
        }
    }
}
