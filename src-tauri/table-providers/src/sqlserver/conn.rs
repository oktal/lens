//! Module that provides an [`AsyncDbConnection`] for SQL Server
use std::{borrow::Cow, sync::Arc};

use arrow::datatypes::SchemaRef;
use async_stream::stream;
use async_trait::async_trait;
use datafusion::{execution::SendableRecordBatchStream, sql::TableReference};
use datafusion_table_providers::sql::db_connection_pool::dbconnection::{
    AsyncDbConnection, DbConnection,
};
use futures::StreamExt;
use tiberius::{ColumnData, IntoSql, Query, ToSql};

use super::{pool::SqlServerPooledConnection, stream::SqlRecordBatchStream, Error};

pub struct SqlServerConnection {
    conn: Arc<tokio::sync::Mutex<SqlServerPooledConnection>>,
}

type ConnectionError = datafusion_table_providers::sql::db_connection_pool::dbconnection::Error;
type GenericError = datafusion_table_providers::sql::db_connection_pool::dbconnection::GenericError;

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
