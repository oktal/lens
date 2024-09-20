//! A connection pool for SQL Server

use core::fmt;
use std::{collections::HashMap, sync::Arc};

use arrow::datatypes::{Field, Schema, SchemaRef};

use async_stream::stream;
use async_trait::async_trait;
use datafusion::sql::TableReference;
use datafusion::{
    error::DataFusionError, execution::SendableRecordBatchStream,
    physical_plan::stream::RecordBatchStreamAdapter,
};
use datafusion_table_providers::sql::db_connection_pool;
use datafusion_table_providers::sql::db_connection_pool::{
    dbconnection::{AsyncDbConnection, DbConnection},
    DbConnectionPool, JoinPushDown,
};
use futures::{stream, StreamExt};
use secrecy::{ExposeSecret, SecretString};
use tiberius::{AuthMethod, Config, EncryptionLevel, ToSql, ColumnData};
use tokio::net::TcpStream;
use tokio_util::compat::TokioAsyncWriteCompatExt;

#[derive(Debug)]
pub enum Error {
    Io(std::io::Error),
    Sql(tiberius::error::Error),

    Connection(bb8::RunError<tiberius::error::Error>),

    Arrow(super::arrow::Error),

    InvalidParameter {
        name: Option<&'static str>,
        detail: String,
    },
}

pub type Result<T, E = Error> = std::result::Result<T, E>;

impl From<std::io::Error> for Error {
    fn from(value: std::io::Error) -> Self {
        Self::Io(value)
    }
}

impl From<tiberius::error::Error> for Error {
    fn from(value: tiberius::error::Error) -> Self {
        Self::Sql(value)
    }
}

impl From<bb8::RunError<tiberius::error::Error>> for Error {
    fn from(value: bb8::RunError<tiberius::error::Error>) -> Self {
        Self::Connection(value)
    }
}

impl From<super::arrow::Error> for Error {
    fn from(value: super::arrow::Error) -> Self {
        Self::Arrow(value)
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Io(err) => write!(f, "{err}"),
            Self::Sql(err) => write!(f, "{err}"),
            Self::Connection(err) => write!(f, "{err}"),
            Self::Arrow(err) => write!(f, "{err}"),
            Self::InvalidParameter { name, detail } => {
                if let Some(name) = name {
                    write!(f, "invalid parameter {name}: {detail}")
                } else {
                    write!(f, "invalid parameter: {detail}")
                }
            }
        }
    }
}

impl std::error::Error for Error {}

pub struct SqlServerConnectionManager {
    config: tiberius::Config,
}

#[async_trait]
impl bb8::ManageConnection for SqlServerConnectionManager {
    type Connection = tiberius::Client<tokio_util::compat::Compat<TcpStream>>;
    type Error = tiberius::error::Error;

    async fn connect(&self) -> Result<Self::Connection, Self::Error> {
        let tcp = TcpStream::connect(self.config.get_addr()).await?;
        tcp.set_nodelay(true)?;

        let client = tiberius::Client::connect(self.config.clone(), tcp.compat_write()).await?;
        Ok(client)
    }

    async fn is_valid(&self, conn: &mut Self::Connection) -> Result<(), Self::Error> {
        let _ = conn.simple_query("select 1").await?;
        Ok(())
    }

    fn has_broken(&self, _conn: &mut Self::Connection) -> bool {
        false
    }
}

pub type SqlServerPooledConnection = bb8::PooledConnection<'static, SqlServerConnectionManager>;

pub struct SqlServerConnectionPool {
    pool: Arc<bb8::Pool<SqlServerConnectionManager>>,
}

pub struct SqlServerConnection {
    conn: Arc<tokio::sync::Mutex<SqlServerPooledConnection>>,
}

impl SqlServerConnectionPool {
    pub async fn new(params: HashMap<String, SecretString>) -> Result<Self> {
        let config = Self::create_config(params)?;
        let pool = bb8::Pool::builder()
            .build(SqlServerConnectionManager { config })
            .await?;
        Ok(Self {
            pool: Arc::new(pool),
        })
    }

    fn create_config(params: HashMap<String, SecretString>) -> Result<Config> {
        if let Some(connection_string) = params
            .get("connection_string")
            .map(ExposeSecret::expose_secret)
        {
            Ok(Config::from_ado_string(&connection_string)?)
        } else {
            let mut config = Config::new();

            if let Some(host) = params.get("host").map(ExposeSecret::expose_secret) {
                config.host(host);
            }

            if let Some(port) = params.get("port").map(ExposeSecret::expose_secret) {
                let port = port.parse().map_err(|_| Error::InvalidParameter {
                    name: Some("port"),
                    detail: "invalid port".to_string(),
                })?;
                config.port(port)
            }

            if let Some(app_name) = params
                .get("application_name")
                .map(ExposeSecret::expose_secret)
            {
                config.application_name(app_name);
            }

            config.encryption(EncryptionLevel::On);

            if let (Some(username), Some(password)) = (
                params.get("username").map(ExposeSecret::expose_secret),
                params.get("password").map(ExposeSecret::expose_secret),
            ) {
                let auth_mode = params
                    .get("auth_mode")
                    .map(ExposeSecret::expose_secret)
                    .unwrap_or("sql_server");
                let auth = if auth_mode.eq_ignore_ascii_case("sql_server") {
                    AuthMethod::sql_server(username, password)
                } else {
                    return Err(Error::InvalidParameter {
                        name: Some("auth_mode"),
                        detail: "Invalid authentication mode: valid modes are sql_server, windows"
                            .to_string(),
                    });
                };

                config.authentication(auth);
            }

            Ok(config)
        }
    }
}

impl<'a> DbConnection<SqlServerPooledConnection, &'a dyn ToSql> for SqlServerConnection {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }

    fn as_async(&self) -> Option<&dyn AsyncDbConnection<SqlServerPooledConnection, &'a dyn ToSql>> {
        Some(self)
    }
}

#[async_trait]
impl DbConnectionPool<SqlServerPooledConnection, &'static dyn ToSql> for SqlServerConnectionPool {
    async fn connect(
        &self,
    ) -> Result<
        Box<dyn DbConnection<SqlServerPooledConnection, &'static dyn ToSql>>,
        db_connection_pool::Error,
    > {
        let pool = Arc::clone(&self.pool);
        // let conn = pool.get().await?;
        // Ok(Box::new(SqlServerConnection::new(conn)))

        todo!();
    }

    fn join_push_down(&self) -> JoinPushDown {
        todo!()
    }
}

type ConnectionError = datafusion_table_providers::sql::db_connection_pool::dbconnection::Error;
type GenericError = datafusion_table_providers::sql::db_connection_pool::dbconnection::GenericError;

struct OwnedColumnData<'a>(ColumnData<'a>);

impl<'a> ToSql for OwnedColumnData<'a> {
    fn to_sql(&self) -> ColumnData<'_> {
        self.0.clone()
    }
}

#[async_trait]
impl<'a> AsyncDbConnection<SqlServerPooledConnection, &'a dyn ToSql> for SqlServerConnection {
    fn new(conn: SqlServerPooledConnection) -> Self
    where
        Self: Sized,
    {
        Self {
            conn: Arc::new(tokio::sync::Mutex::new(conn)),
        }
    }

    async fn get_schema(
        &self,
        table_reference: &TableReference,
    ) -> Result<SchemaRef, ConnectionError> {
        let table_ref = table_reference.to_quoted_string();
        let mut conn = self.conn.lock().await;

        let stream = conn
            .query(format!("select * from {table_ref} limit 1"), &[])
            .await
            .map_err(|e| ConnectionError::UnableToGetSchema { source: e.into() })?;

        let record = super::arrow::stream_to_arrow(stream)
            .await
            .map_err(|e| ConnectionError::UnableToGetSchema { source: e.into() })?;

        Ok(record.schema())
    }

    async fn query_arrow(
        &self,
        sql: &str,
        params: &[&'a dyn ToSql],
        projected_schema: Option<SchemaRef>,
    ) -> Result<SendableRecordBatchStream, GenericError> {
        let conn = Arc::clone(&self.conn);
        let params = params.iter().map(|p| OwnedColumnData(p.to_sql())).collect::<Vec<_>>();

        let mut stream = Box::pin(stream! {
            let mut conn = conn.lock().await;

            let stream = conn
                .query(sql, &params)
                .await?;

            let mut chunks = stream.chunks(8192).boxed().map(|rows| {
                let rows = rows.into_iter().collect::<Result<Vec<_>, _>>()?;
                let rec = super::arrow::rows_to_arrow(rows)?;

                Ok::<_, Error>(rec)
            });

            while let Some(chunk) = chunks.next().await {
                yield chunk
            }
        });

        let Some(first_chunk) = stream.next().await else {
            return Ok(Box::pin(RecordBatchStreamAdapter::new(
                Arc::new(Schema::empty()),
                stream::empty(),
            )));
        };

        let first_chunk =
            first_chunk.map_err(|e| ConnectionError::UnableToQueryArrow { source: e.into() })?;
        let schema = first_chunk.schema();

        Ok(Box::pin(RecordBatchStreamAdapter::new(schema, {
            stream! {
                yield Ok(first_chunk);

                while let Some(chunk) = stream.next().await {
                    yield chunk.map_err(|e| DataFusionError::Execution(format!("failed to fetch batch: {e}")))
                }

            }
        })))
    }

    async fn execute(&self, sql: &str, params: &[&'a dyn ToSql]) -> Result<u64, GenericError> {
        todo!()
    }
}
