use std::sync::{Arc, Mutex};

use anyhow::Context;
use datafusion::{
    logical_expr::{DdlStatement, LogicalPlan},
    prelude::*,
    sql::{parser::Statement, TableReference},
};
use object_store::{aws::AmazonS3Builder, ObjectStore};
use tokio::sync::mpsc;
use url::Url;

use crate::{
    common::{DatasourceConfig, ExportOptions, ObjectStoreConfig, Row, StreamId, StreamInfo},
    query::stream::{QueryStreamRequest, QueryStreamer},
};

pub struct LensError(anyhow::Error);
pub type LensResult<T, E = LensError> = std::result::Result<T, E>;

pub struct Lens {
    ctx: SessionContext,
    stream_tx: mpsc::Sender<QueryStreamRequest>,

    datasources: Arc<Mutex<Vec<DatasourceConfig>>>,
}

impl<E> From<E> for LensError
where
    E: Into<anyhow::Error>,
{
    fn from(value: E) -> Self {
        Self(value.into())
    }
}

impl serde::Serialize for LensError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.0.to_string().as_ref())
    }
}

impl Lens {
    pub fn new() -> (Self, QueryStreamer) {
        // Setup session
        let config = SessionConfig::new()
            .with_information_schema(false)
            .with_create_default_catalog_and_schema(false);

        let ctx = SessionContext::new_with_config(config);

        // Replace default table factories by our own factory
        {
            let state_ref = ctx.state_ref();
            let mut state = state_ref.write();
            for factory in state.table_factories_mut().values_mut() {
                *factory = crate::table_provider::factory();
            }
        }

        let (query_exec, query_tx) = QueryStreamer::new(ctx.clone());
        (
            Self {
                ctx,
                stream_tx: query_tx,
                datasources: Arc::new(Mutex::new(vec![])),
            },
            query_exec,
        )
    }

    pub async fn sql(&self, query: &str) -> LensResult<DataFrame> {
        Ok(self
            .ctx
            .execute_logical_plan(self.create_logical_plan(query).await?)
            .await?)
    }

    pub async fn stream(&self, query: &str) -> LensResult<StreamId> {
        let (req, rx) = QueryStreamRequest::create(query);
        self.stream_tx.send(req).await?;
        let stream_id = rx.await?;

        stream_id
    }

    pub async fn stream_next(&self, stream_id: StreamId) -> LensResult<Option<Vec<Row>>> {
        let (req, rx) = QueryStreamRequest::next(stream_id);
        self.stream_tx.send(req).await?;
        let rows = rx.await?;
        rows
    }

    pub async fn stream_export(
        &self,
        stream_id: StreamId,
        options: ExportOptions,
    ) -> LensResult<usize> {
        let (req, rx) = QueryStreamRequest::export(stream_id, options);
        self.stream_tx.send(req).await?;
        let count = rx.await?;
        count
    }

    pub async fn stream_list(&self) -> LensResult<Vec<StreamInfo>> {
        let (req, rx) = QueryStreamRequest::list();
        self.stream_tx.send(req).await?;
        let infos = rx.await?;
        infos
    }

    pub fn context(&self) -> &SessionContext {
        &self.ctx
    }

    pub fn register_datasource(
        &self,
        source_config: DatasourceConfig,
    ) -> LensResult<Arc<dyn ObjectStore>> {
        let url = Url::parse(&source_config.url)
            .with_context(|| format!("invalid url {}", source_config.url))?;
        let object_store = Self::create_object_store(&source_config.store)?;

        self.ctx
            .register_object_store(&url, Arc::clone(&object_store));

        let mut datasources = self.datasources.lock().expect("lock poisoned");
        match datasources.iter_mut().find(|c| c.url == source_config.url) {
            Some(previous) => *previous = source_config,
            None => datasources.push(source_config),
        };

        Ok(object_store)
    }

    pub fn datasources(&self) -> Vec<DatasourceConfig> {
        self.datasources.lock().expect("lock poisoned").clone()
    }

    async fn create_logical_plan(&self, query: &str) -> LensResult<LogicalPlan> {
        // We need to create (and rewrite) our own logical plan instead of directly using `sql`
        // from DataFusion `SessionContext` because unfortunately, DataFusion does not properly
        // interpret table identifier from the "CREATE EXTERNAL TABLE <database>.<schema>.<table>"
        // statement.
        // When transforming a SQL statement to its LogicalPlan, DataFusion will convert
        // "<database>.<schema>.<table>" identifier to a TableReference::Bare { "database.schema.table" }
        // reference instead of TableReference::Full { "database", "schema", "table" }
        // We thus "rewrite" the logical plan prior to executing it with the TableReference that we
        // parsed from the initial statement
        let state = self.ctx.state();
        let dialect = state.config().options().sql_parser.dialect.as_str();

        let statement = state.sql_to_statement(query, dialect)?;

        let create_table_ref = if let Statement::CreateExternalTable(cet) = &statement {
            Some(TableReference::parse_str(&cet.name))
        } else {
            None
        };

        let plan = state.statement_to_plan(statement).await?;
        let plan = if let LogicalPlan::Ddl(DdlStatement::CreateExternalTable(mut cet)) = plan {
            if let Some(table_ref) = create_table_ref {
                cet.name = table_ref;
            }
            LogicalPlan::Ddl(DdlStatement::CreateExternalTable(cet))
        } else {
            plan
        };

        Ok(plan)
    }

    fn create_object_store(config: &ObjectStoreConfig) -> LensResult<Arc<dyn ObjectStore>> {
        Ok(Arc::new(match config {
            ObjectStoreConfig::AmazonS3 {
                access_key_id,
                secret_access_key,
                session_token,
                bucket,
                region,
            } => {
                let mut builder = AmazonS3Builder::new()
                    .with_access_key_id(access_key_id)
                    .with_secret_access_key(secret_access_key)
                    .with_region(region);

                if let Some(session_token) = session_token {
                    builder = builder.with_token(session_token);
                }

                if let Some(bucket) = bucket {
                    builder = builder.with_bucket_name(bucket);
                }

                builder.build()?
            }
            ObjectStoreConfig::GoogleCloudStorage {
                service_account_path,
                service_acccount_key,
                application_credentials_path,
            } => todo!(),
        }))
    }
}
