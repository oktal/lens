//! Module that defines a format to export data in JSON.
//! We need to define our own format instead of using the default DataFusion JSON format
//! because DataFusion will output one JSON object per-line instead of outputing a JSON array

use std::any::Any;
use std::fmt;
use std::sync::Arc;

use async_trait::async_trait;
use bytes::Bytes;
use datafusion::arrow::array::RecordBatch;
use datafusion::arrow::datatypes::SchemaRef;
use datafusion::arrow::json;
use datafusion::common::file_options::json_writer::JsonWriterOptions;
use datafusion::common::{GetExt, Statistics};
use datafusion::datasource::file_format::FileFormatFactory;
use datafusion::datasource::file_format::{
    file_compression_type::FileCompressionType, json::JsonFormat, FileFormat,
};
use datafusion::datasource::physical_plan::{FileScanConfig, FileSinkConfig};
use datafusion::error::{DataFusionError, Result};
use datafusion::execution::context::SessionState;
use datafusion::execution::{SendableRecordBatchStream, TaskContext};
use datafusion::physical_expr::PhysicalSortRequirement;
use datafusion::physical_plan::insert::{DataSink, DataSinkExec};
use datafusion::physical_plan::metrics::MetricsSet;
use datafusion::physical_plan::{DisplayAs, DisplayFormatType, ExecutionPlan, PhysicalExpr};
use object_store::{ObjectMeta, ObjectStore};

use super::orchestration::stateless_multipart_put;

/// Implementation of a JSON file format based on the original DataFusion [`JsonFormat`] that will
/// output data as an array instead of object line separated
#[derive(Debug)]
struct JsonArrayFormat {
    inner: JsonFormat,
}

#[derive(Debug)]
struct JsonArrayFormatFactory;

impl JsonArrayFormat {
    fn new() -> Self {
        Self {
            inner: JsonFormat::default(),
        }
    }
}

impl GetExt for JsonArrayFormatFactory {
    fn get_ext(&self) -> String {
        "json".to_string()
    }
}

impl FileFormatFactory for JsonArrayFormatFactory {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn create(
        &self,
        _state: &SessionState,
        _format_options: &std::collections::HashMap<String, String>,
    ) -> Result<Arc<dyn FileFormat>> {
        Ok(self.default())
    }

    fn default(&self) -> Arc<dyn FileFormat> {
        Arc::new(JsonArrayFormat::new())
    }
}

#[async_trait]
impl FileFormat for JsonArrayFormat {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn get_ext(&self) -> String {
        self.inner.get_ext()
    }

    fn get_ext_with_compression(
        &self,
        file_compression_type: &FileCompressionType,
    ) -> Result<String> {
        self.inner.get_ext_with_compression(file_compression_type)
    }

    async fn infer_schema(
        &self,
        state: &SessionState,
        store: &Arc<dyn ObjectStore>,
        objects: &[ObjectMeta],
    ) -> Result<SchemaRef> {
        self.inner.infer_schema(state, store, objects).await
    }

    async fn infer_stats(
        &self,
        state: &SessionState,
        store: &Arc<dyn ObjectStore>,
        table_schema: SchemaRef,
        object: &ObjectMeta,
    ) -> Result<Statistics> {
        self.inner
            .infer_stats(state, store, table_schema, object)
            .await
    }

    async fn create_physical_plan(
        &self,
        state: &SessionState,
        conf: FileScanConfig,
        filters: Option<&Arc<dyn PhysicalExpr>>,
    ) -> Result<Arc<dyn ExecutionPlan>> {
        self.inner.create_physical_plan(state, conf, filters).await
    }

    async fn create_writer_physical_plan(
        &self,
        input: Arc<dyn ExecutionPlan>,
        _state: &SessionState,
        conf: FileSinkConfig,
        order_requirements: Option<Vec<PhysicalSortRequirement>>,
    ) -> Result<Arc<dyn ExecutionPlan>> {
        if conf.overwrite {
            return Err(DataFusionError::NotImplemented(
                "JSON files can not be overwritten yet".to_string(),
            ));
        }

        let writer_options = JsonWriterOptions::try_from(self.inner.options())?;
        let schema = Arc::clone(conf.output_schema());
        let sink = Arc::new(JsonArraySink::new(conf, writer_options));

        Ok(Arc::new(DataSinkExec::new(input, sink, schema, order_requirements)) as _)
    }
}

/// Define a struct for serializing Json records to a stream
pub struct JsonArraySerializer;

impl datafusion::datasource::file_format::write::BatchSerializer for JsonArraySerializer {
    fn serialize(&self, batch: RecordBatch, _initial: bool) -> Result<Bytes> {
        let mut buffer = Vec::with_capacity(4096);
        let mut writer = json::ArrayWriter::new(&mut buffer);
        writer.write(&batch)?;
        Ok(Bytes::from(buffer))
    }
}

/// Implements [`DataSink`] for writing to a Json file.
struct JsonArraySink {
    /// Config options for writing data
    config: FileSinkConfig,
    /// Writer options for underlying Json writer
    writer_options: JsonWriterOptions,
}

impl std::fmt::Debug for JsonArraySink {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("JsonSink").finish()
    }
}

impl DisplayAs for JsonArraySink {
    fn fmt_as(&self, _t: DisplayFormatType, _f: &mut fmt::Formatter<'_>) -> fmt::Result {
        Ok(())
    }
}

impl JsonArraySink {
    /// Create from config.
    pub fn new(config: FileSinkConfig, writer_options: JsonWriterOptions) -> Self {
        Self {
            config,
            writer_options,
        }
    }

    async fn multipartput_all(
        &self,
        data: SendableRecordBatchStream,
        context: &Arc<TaskContext>,
    ) -> Result<u64> {
        let get_serializer = move || Arc::new(JsonArraySerializer) as _;

        stateless_multipart_put(
            data,
            context,
            "json".into(),
            Box::new(get_serializer),
            &self.config,
            self.writer_options.compression.into(),
        )
        .await
    }
}

#[async_trait]
impl DataSink for JsonArraySink {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn metrics(&self) -> Option<MetricsSet> {
        None
    }

    async fn write_all(
        &self,
        data: SendableRecordBatchStream,
        context: &Arc<TaskContext>,
    ) -> Result<u64> {
        let total_count = self.multipartput_all(data, context).await?;
        Ok(total_count)
    }
}

pub(crate) fn factory() -> Arc<dyn FileFormatFactory> {
    Arc::new(JsonArrayFormatFactory)
}
