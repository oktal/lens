//! A module that defines data export file format
//! This module contains code that has been directly imported from DataFusion
//! that we need to override default DataFusion implementation

use std::sync::Arc;

use datafusion::datasource::file_format::file_compression_type::FileCompressionType;
use datafusion::error::Result;
use object_store::{buffered::BufWriter, path::Path, ObjectStore};
use tokio::io::AsyncWrite;

mod demux;
pub(super) mod json;
mod orchestration;

/// Returns an [`AsyncWrite`] which writes to the given object store location
/// with the specified compression.
/// We drop the `AbortableWrite` struct and the writer will not try to cleanup on failure.
/// Users can configure automatic cleanup with their cloud provider.
async fn create_writer(
    file_compression_type: FileCompressionType,
    location: &Path,
    object_store: Arc<dyn ObjectStore>,
) -> Result<Box<dyn AsyncWrite + Send + Unpin>> {
    let buf_writer = BufWriter::new(object_store, location.clone());
    file_compression_type.convert_async_writer(buf_writer)
}
