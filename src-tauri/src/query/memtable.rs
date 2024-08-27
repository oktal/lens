//! A module that defines an implementation for an in-memory table that supports appending data

use std::{
    any::Any,
    fmt,
    sync::{Arc, RwLock},
};

use async_trait::async_trait;
use datafusion::{
    arrow::{array::RecordBatch, datatypes::SchemaRef},
    common::{project_schema, Constraints},
    datasource::{TableProvider, TableType},
    error::{DataFusionError, Result},
    execution::{context::SessionState, SendableRecordBatchStream, TaskContext},
    physical_expr::EquivalenceProperties,
    physical_plan::{
        memory::MemoryStream, DisplayAs, DisplayFormatType, ExecutionMode, ExecutionPlan,
        Partitioning, PlanProperties,
    },
    prelude::Expr,
};

#[derive(Debug)]
struct Shared {
    schema: SchemaRef,
    batches: Arc<RwLock<Vec<RecordBatch>>>,
}

pub struct MemTable {
    shared: Arc<Shared>,
}

#[derive(Debug)]
struct MemExecPlan {
    shared: Arc<Shared>,
    projection: Option<Vec<usize>>,
    schema: SchemaRef,
    properties: PlanProperties,
}

impl Shared {
    fn new(schema: SchemaRef) -> Arc<Self> {
        Arc::new(Self {
            schema,
            batches: Arc::new(RwLock::new(Vec::new())),
        })
    }
}

impl MemTable {
    pub fn new(schema: SchemaRef) -> Self {
        Self {
            shared: Shared::new(schema),
        }
    }

    pub fn insert(&self, batch: RecordBatch) {
        let mut batches = self.shared.batches.write().expect("lock poisoned");
        batches.push(batch);
    }
}

#[async_trait]
impl TableProvider for MemTable {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn schema(&self) -> SchemaRef {
        self.shared.schema.clone()
    }

    fn constraints(&self) -> Option<&Constraints> {
        None
    }

    fn table_type(&self) -> TableType {
        TableType::Temporary
    }

    async fn scan(
        &self,
        _state: &SessionState,
        projection: Option<&Vec<usize>>,
        _filters: &[Expr],
        _limit: Option<usize>,
    ) -> Result<Arc<dyn ExecutionPlan>> {
        Ok(Arc::new(MemExecPlan::new(
            Arc::clone(&self.shared),
            projection.cloned(),
        )?))
    }
}

impl MemExecPlan {
    fn new(shared: Arc<Shared>, projection: Option<Vec<usize>>) -> Result<Self> {
        let schema = project_schema(&shared.schema, projection.as_ref())?;

        let partitions_count = shared.batches.read().expect("lock poisoned").len();

        let properties = PlanProperties::new(
            EquivalenceProperties::new(schema.clone()),
            Partitioning::UnknownPartitioning(partitions_count),
            ExecutionMode::Bounded,
        );

        Ok(Self {
            shared,
            schema,
            properties,
            projection,
        })
    }

    fn partition(&self, idx: usize) -> Result<RecordBatch> {
        let partitions = self.shared.batches.read().expect("lock poisoned");

        partitions
            .get(idx)
            .ok_or(DataFusionError::Internal(format!(
                "partition {idx} out of bounds"
            )))
            .cloned()
    }
}

impl DisplayAs for MemExecPlan {
    fn fmt_as(&self, _t: DisplayFormatType, _f: &mut fmt::Formatter) -> fmt::Result {
        Ok(())
    }
}

impl ExecutionPlan for MemExecPlan {
    fn name(&self) -> &str {
        "MemExecPlan"
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn properties(&self) -> &PlanProperties {
        &self.properties
    }

    fn children(&self) -> Vec<&Arc<dyn ExecutionPlan>> {
        vec![]
    }

    fn with_new_children(
        self: Arc<Self>,
        children: Vec<Arc<dyn ExecutionPlan>>,
    ) -> Result<Arc<dyn ExecutionPlan>> {
        if children.is_empty() {
            return Ok(self);
        } else {
            return Err(DataFusionError::Internal("Invalid operation".to_string()));
        }
    }

    fn execute(
        &self,
        partition: usize,
        _context: Arc<TaskContext>,
    ) -> Result<SendableRecordBatchStream> {
        let partition = self.partition(partition)?;
        Ok(Box::pin(MemoryStream::try_new(
            vec![partition],
            Arc::clone(&self.schema),
            self.projection.clone(),
        )?))
    }
}
