//! Module that defines a [`RecordBatchStream`] adapter to poll from a [`Stream`] with an
//! associated [`SchemaRef`]
//! This adapter differs from DataFusion' `RecordBatchStreamAdapter` by not requiring a [`SchemaRef`]
//! The [`SchemaRef`] will automatically be retrieved from the [`RecordBatch`] being polled from
//! the underlying [`Stream`]
use std::{
    sync::Arc,
    task::{ready, Poll},
};

use arrow::{
    array::RecordBatch,
    datatypes::{Schema, SchemaRef},
};
use datafusion::{error::DataFusionError, execution::RecordBatchStream};
use futures::Stream;
use pin_project::pin_project;

#[pin_project]
pub(super) struct SqlRecordBatchStream<S> {
    schema: Option<SchemaRef>,

    #[pin]
    stream: S,
}

impl<S> SqlRecordBatchStream<S> {
    pub fn new(stream: S) -> Self {
        Self {
            schema: None,
            stream,
        }
    }
}

impl<S, E> Stream for SqlRecordBatchStream<S>
where
    S: Stream<Item = Result<RecordBatch, E>>,
    E: Into<DataFusionError>,
{
    type Item = Result<RecordBatch, DataFusionError>;

    fn poll_next(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> Poll<Option<Self::Item>> {
        let this = self.project();

        let batch = ready!(this.stream.poll_next(cx));
        let Some(batch) = batch else {
            return Poll::Ready(None);
        };

        if let Ok(batch) = &batch {
            *this.schema = Some(batch.schema());
        }

        Poll::Ready(Some(batch.map_err(Into::into)))
    }
}

impl<S, E> RecordBatchStream for SqlRecordBatchStream<S>
where
    S: Stream<Item = Result<RecordBatch, E>>,
    E: Into<DataFusionError>,
{
    fn schema(&self) -> SchemaRef {
        self.schema.clone().unwrap_or(Arc::new(Schema::empty()))
    }
}
