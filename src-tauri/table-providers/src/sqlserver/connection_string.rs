//! Module that defines helper to parse an ADO.NET connection string.
//! Unfortunately, `tiberius` `Config` does not publicly expose the fields
//! that it parsed from the connection string so we roll our own isntead
use std::{fmt, num::ParseIntError, str::FromStr};

use connection_string::AdoNetString;
use secrecy::SecretString;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum SqlServerConnectionStringError {
    Invalid,
    MissingHost,
    InvalidPort(ParseIntError),
}

impl From<connection_string::Error> for SqlServerConnectionStringError {
    fn from(value: connection_string::Error) -> Self {
        Self::Invalid
    }
}

impl fmt::Display for SqlServerConnectionStringError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SqlServerConnectionStringError::Invalid => {
                write!(f, "invalid connection string")
            }
            SqlServerConnectionStringError::MissingHost => write!(f, "missing Server"),
            SqlServerConnectionStringError::InvalidPort(e) => write!(f, "invalid port: {e}"),
        }
    }
}

impl std::error::Error for SqlServerConnectionStringError {}

struct ServerDefinition {
    host: String,
    port: Option<u16>,
    instance: Option<String>,
}

impl FromStr for ServerDefinition {
    type Err = SqlServerConnectionStringError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split(',');
        let host_with_instance = parts
            .next()
            .ok_or(SqlServerConnectionStringError::MissingHost)?;

        let mut host_with_instance = host_with_instance.split("\\");
        let host = host_with_instance
            .next()
            .ok_or(SqlServerConnectionStringError::MissingHost)?;
        let instance = host_with_instance.next().map(ToString::to_string);

        if let Some(port) = parts.next() {
            let port = port
                .parse()
                .map_err(SqlServerConnectionStringError::InvalidPort)?;

            Ok(Self {
                host: host.to_string(),
                port: Some(port),
                instance,
            })
        } else {
            Ok(Self {
                host: host.to_string(),
                port: None,
                instance,
            })
        }
    }
}

#[derive(Debug, Clone)]
pub(crate) struct SqlServerConnectionString {
    pub host: String,
    pub port: Option<u16>,
    pub instance: Option<String>,
    pub database: Option<String>,
    pub user: Option<String>,
    pub password: Option<SecretString>,
}

impl FromStr for SqlServerConnectionString {
    type Err = SqlServerConnectionStringError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let conn: AdoNetString = s.parse()?;

        let server = conn
            .get("server")
            .or_else(|| conn.get("data source"))
            .ok_or(SqlServerConnectionStringError::MissingHost)?;

        let ServerDefinition {
            host,
            port,
            instance,
        } = server.parse()?;

        let database = conn
            .get("database")
            .or_else(|| conn.get("initial catalog"))
            .or_else(|| conn.get("databasename"))
            .cloned();

        let user = conn
            .get("uid")
            .or_else(|| conn.get("username"))
            .or_else(|| conn.get("user"))
            .or_else(|| conn.get("user id"))
            .cloned();

        let password = conn.get("password").cloned().map(SecretString::from);

        Ok(Self {
            host,
            port,
            instance,
            database,
            user,
            password,
        })
    }
}

#[cfg(test)]
mod tests {
    use secrecy::ExposeSecret;

    use crate::sqlserver::connection_string::SqlServerConnectionStringError;

    use super::SqlServerConnectionString;

    #[test]
    fn parse_connection_string() {
        let conn: SqlServerConnectionString =
            "Server=MyServer\\BCDEMO,28000;Database=BC170;User Id=MySQLAccount;Password=MyPassword;"
                .parse()
                .expect("valid connection string");

        assert_eq!(conn.host, "MyServer");
        assert_eq!(conn.port, Some(28000));
        assert_eq!(conn.instance.as_deref(), Some("BCDEMO"));
        assert_eq!(conn.user.as_deref(), Some("MySQLAccount"));
        assert_eq!(
            conn.password.as_ref().map(ExposeSecret::expose_secret),
            Some("MyPassword")
        );
    }

    #[test]
    fn error_when_server_missing() {
        let conn = "Database=BC170".parse::<SqlServerConnectionString>();
        assert!(conn.is_err());
        assert_eq!(
            conn.unwrap_err(),
            SqlServerConnectionStringError::MissingHost
        )
    }

    #[test]
    fn error_when_port_is_invalid() {
        let conn = "Server=MyServer,abcd;Database=BC170".parse::<SqlServerConnectionString>();
        assert!(conn.is_err());
        assert!(matches!(
            conn.unwrap_err(),
            SqlServerConnectionStringError::InvalidPort(_)
        ));
    }
}
