//! Module that provides a connection pool for SQL Server

use core::fmt;
use std::{collections::HashMap, sync::Arc};

use async_trait::async_trait;
use datafusion::error::DataFusionError;
use datafusion_table_providers::sql::db_connection_pool;
use datafusion_table_providers::sql::db_connection_pool::{
    dbconnection::{AsyncDbConnection, DbConnection},
    DbConnectionPool, JoinPushDown,
};
use secrecy::{ExposeSecret, SecretString};
use tiberius::{AuthMethod, Config, EncryptionLevel, ToSql};
use tokio::net::TcpStream;
use tokio_util::compat::TokioAsyncWriteCompatExt;

use super::conn::SqlServerConnection;
use super::connection_string::SqlServerConnectionString;

#[derive(Debug)]
pub enum Error {
    Io(std::io::Error),
    Sql(tiberius::error::Error),

    ConnectionString(super::connection_string::SqlServerConnectionStringError),

    InvalidPort,
    InvalidAuth,

    Connection(bb8::RunError<tiberius::error::Error>),
    Arrow(super::arrow::Error),
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

impl From<super::connection_string::SqlServerConnectionStringError> for Error {
    fn from(value: super::connection_string::SqlServerConnectionStringError) -> Self {
        Self::ConnectionString(value)
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
            Self::InvalidPort => write!(f, "invalid port"),
            Self::InvalidAuth => write!(f, "invalid authentication mode"),
            Self::Connection(err) => write!(f, "{err}"),
            Self::ConnectionString(err) => write!(f, "{err}"),
            Self::Arrow(err) => write!(f, "{err}"),
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

fn get_join_pushdown(params: &HashMap<String, SecretString>) -> Result<JoinPushDown> {
    let mut context_str = String::new();

    if let Some(connection_string) = params
        .get("connection_string")
        .map(ExposeSecret::expose_secret)
    {
        let conn_string = connection_string.parse::<SqlServerConnectionString>()?;
        context_str.push_str(&format!("host={},", conn_string.host));

        if let Some(instance) = conn_string.instance {
            context_str.push_str(&format!("instance={instance},"));
        }

        if let Some(port) = conn_string.port {
            context_str.push_str(&format!("port={port},"));
        }

        if let Some(database) = conn_string.database {
            context_str.push_str(&format!("database={database},"));
        }
    } else {
        if let Some(host) = params.get("host").map(ExposeSecret::expose_secret) {
            context_str.push_str(&format!("host={host},"));
        }

        if let Some(Ok(port)) = params
            .get("port")
            .map(ExposeSecret::expose_secret)
            .map(|p| u16::from_str_radix(p, 10))
        {
            context_str.push_str(&format!("port={port}"));
        }

        if let Some(database) = params.get("database").map(ExposeSecret::expose_secret) {
            context_str.push_str(&format!("database={database}"));
        }
    }

    Ok(JoinPushDown::AllowedFor(context_str))
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

pub struct SqlServerConnectionPool {
    pool: Arc<bb8::Pool<SqlServerConnectionManager>>,
    join_push_down: JoinPushDown,
}

pub type SqlServerPooledConnection = bb8::PooledConnection<'static, SqlServerConnectionManager>;

impl SqlServerConnectionPool {
    pub async fn new(params: HashMap<String, SecretString>) -> Result<Self> {
        let join_push_down = get_join_pushdown(&params)?;
        let config = Self::create_config(&params)?;
        let pool = bb8::Pool::builder()
            .build(SqlServerConnectionManager { config })
            .await?;
        Ok(Self {
            pool: Arc::new(pool),
            join_push_down,
        })
    }

    fn create_config(params: &HashMap<String, SecretString>) -> Result<Config> {
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
                let port = port.parse().map_err(|_| Error::InvalidPort)?;
                config.port(port);
            }

            if let Some(database) = params.get("database").map(ExposeSecret::expose_secret) {
                config.database(database);
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
                    return Err(Error::InvalidAuth);
                };

                config.authentication(auth);
            }

            Ok(config)
        }
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
        self.join_push_down.clone()
    }
}
