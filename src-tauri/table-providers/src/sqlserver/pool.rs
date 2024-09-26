//! A connection pool for SQL Server

use core::fmt;
use std::borrow::Cow;
use std::task::{ready, Poll};
use std::{collections::HashMap, sync::Arc};

use arrow::array::RecordBatch;
use arrow::datatypes::{Schema, SchemaRef};

use async_stream::stream;
use async_trait::async_trait;
use datafusion::execution::RecordBatchStream;
use datafusion::sql::TableReference;
use datafusion::{error::DataFusionError, execution::SendableRecordBatchStream};
use datafusion_table_providers::sql::db_connection_pool;
use datafusion_table_providers::sql::db_connection_pool::{
    dbconnection::{AsyncDbConnection, DbConnection},
    DbConnectionPool, JoinPushDown,
};
use futures::{Stream, StreamExt};
use pin_project::pin_project;
use secrecy::{ExposeSecret, SecretString};
use tiberius::{AuthMethod, ColumnData, Config, EncryptionLevel, IntoSql, Query, ToSql};
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

impl Into<DataFusionError> for Error {
    fn into(self) -> DataFusionError {
        DataFusionError::Execution(self.to_string())
    }
}

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
        let conn = self.pool.get_owned().await?;
        Ok(Box::new(SqlServerConnection::new(conn)))
    }

    fn join_push_down(&self) -> JoinPushDown {
        todo!()
    }
}

type ConnectionError = datafusion_table_providers::sql::db_connection_pool::dbconnection::Error;
type GenericError = datafusion_table_providers::sql::db_connection_pool::dbconnection::GenericError;

#[pin_project]
struct SqlRecordBatchStream<S> {
    schema: Option<SchemaRef>,

    #[pin]
    stream: S,
}

impl<S> SqlRecordBatchStream<S> {
    fn new(stream: S) -> Self {
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

fn to_owned<'a, T: ?Sized + ToOwned>(val: Cow<'a, T>) -> Cow<'static, T> {
    match val {
        Cow::Borrowed(val) => Cow::Owned(val.to_owned()),
        Cow::Owned(val) => Cow::Owned(val),
    }
}

/// A [`ColumnData`] that owns the underlying data, meaning that it will
/// transform Cow::Borrowed data to Cow::Owned when needed
struct OwnedColumnData(ColumnData<'static>);
impl<'a> From<ColumnData<'a>> for OwnedColumnData {
    fn from(value: ColumnData<'a>) -> Self {
        Self(match value {
            ColumnData::U8(val) => ColumnData::U8(val),
            ColumnData::I16(val) => ColumnData::I16(val),
            ColumnData::I32(val) => ColumnData::I32(val),
            ColumnData::I64(val) => ColumnData::I64(val),
            ColumnData::F32(val) => ColumnData::F32(val),
            ColumnData::F64(val) => ColumnData::F64(val),
            ColumnData::Bit(val) => ColumnData::Bit(val),
            ColumnData::Guid(val) => ColumnData::Guid(val),
            ColumnData::Numeric(val) => ColumnData::Numeric(val),
            ColumnData::DateTime(val) => ColumnData::DateTime(val),
            ColumnData::SmallDateTime(val) => ColumnData::SmallDateTime(val),
            ColumnData::Time(val) => ColumnData::Time(val),
            ColumnData::Date(val) => ColumnData::Date(val),
            ColumnData::DateTime2(val) => ColumnData::DateTime2(val),
            ColumnData::DateTimeOffset(val) => ColumnData::DateTimeOffset(val),
            ColumnData::String(val) => ColumnData::String(val.map(to_owned)),
            ColumnData::Binary(val) => ColumnData::Binary(val.map(to_owned)),
            ColumnData::Xml(val) => ColumnData::Xml(val.map(to_owned)),
        })
    }
}

impl IntoSql<'static> for OwnedColumnData {
    fn into_sql(self) -> ColumnData<'static> {
        self.0
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
        _projected_schema: Option<SchemaRef>,
    ) -> Result<SendableRecordBatchStream, GenericError> {
        let conn = Arc::clone(&self.conn);

        let sql = sql.to_string();
        let params = params
            .iter()
            .map(|p| OwnedColumnData::from(p.to_sql()))
            .collect::<Vec<_>>();

        let stream = stream! {
            let mut conn = conn.lock().await;
            let mut query = Query::new(sql.to_string());
            for param in params {
                query.bind(param);
            }
            let stream = query.query(&mut conn).await.map_err(Error::Sql)?;

            let mut chunks = stream.chunks(8192).map(|rows| {
                let rows = rows.into_iter().collect::<Result<Vec<_>, _>>()?;
                let rec = super::arrow::rows_to_arrow(rows)?;

                Ok::<_, Error>(rec)
            });

            while let Some(chunk) = chunks.next().await {
                yield chunk
            }
        }
        .boxed();

        Ok(Box::pin(SqlRecordBatchStream::new(stream)))
    }

    async fn execute(&self, sql: &str, params: &[&'a dyn ToSql]) -> Result<u64, GenericError> {
        let mut conn = self.conn.lock().await;
        let result = conn.execute(sql, params).await?;
        Ok(result.into_iter().sum())
    }
}
