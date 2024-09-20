use core::fmt;
use std::ops::Sub;
use std::sync::Arc;

use arrow::array::{
    ArrayBuilder, BooleanBuilder, Date32Builder, Float32Builder, Float64Builder, Int16Builder,
    Int32Builder, Int64Builder, LargeStringBuilder, StringBuilder,
    Time64NanosecondBuilder, TimestampNanosecondBuilder, UInt8Builder, ArrayRef, BinaryBuilder,
};
use arrow::datatypes::{DataType, Field, Schema, TimeUnit};
use arrow::error::ArrowError;
use arrow::record_batch::{RecordBatch, RecordBatchOptions};
use chrono::{NaiveDate, NaiveDateTime, NaiveTime};
use datafusion_table_providers::sql::arrow_sql_gen::arrow::map_data_type_to_array_builder;
use futures::stream::TryStreamExt;
use tiberius::{Column, ColumnData, ColumnType, QueryItem, QueryStream, Uuid};

#[derive(Debug)]
pub enum Error {
    Stream(tiberius::error::Error),

    TypeMismatch {
        row_type: ColumnType,
        schema_type: ColumnType,
    },

    Arrow(ArrowError),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Stream(e) => write!(f, "stream error: {e}"),
            Self::TypeMismatch { row_type, schema_type } =>
                write!(f, "type mismatch for column. Expected type {schema_type:?} got {row_type:?}"),
            Self::Arrow(e) => write!(f, "arrow error: {e}")
        }
    }
}

impl std::error::Error for Error {}

pub type Result<T, E = Error> = std::result::Result<T, E>;

impl From<tiberius::error::Error> for Error {
    fn from(value: tiberius::error::Error) -> Self {
        Self::Stream(value)
    }
}

impl From<ArrowError> for Error {
    fn from(value: ArrowError) -> Self {
        Self::Arrow(value)
    }
}

// const `unwrap()` and `expect()` on `Option<T>` are not stable yet
const fn const_unwrap<T: Copy>(x: Option<T>) -> T {
    if let Some(x) = x {
        x
    } else {
        panic!("")
    }
}

const EPOCH: NaiveDate = const_unwrap(NaiveDate::from_ymd_opt(1970, 1, 1));

#[inline]
fn from_days(days: i64, start_year: i32) -> NaiveDate {
    NaiveDate::from_ymd_opt(start_year, 1, 1).unwrap() + chrono::Duration::days(days)
}

#[inline]
fn from_mins(mins: u32) -> NaiveTime {
    NaiveTime::from_num_seconds_from_midnight_opt(mins, 0).unwrap()
}

macro_rules! handle_primitive_type {
    ($builder:expr, $column_type:expr, $builder_ty:ty, $val:expr) => {{
        let Some(builder) = $builder.builder.as_any_mut().downcast_mut::<$builder_ty>() else {
            return Err(Error::TypeMismatch {
                row_type: $column_type,
                schema_type: $builder.column_type,
            });
        };
        match $val {
            Some(v) => builder.append_value(v),
            None => builder.append_null(),
        }
    }};
}

struct ColumnBuilder {
    name: String,
    column_type: ColumnType,
    data_type: DataType,
    builder: Box<dyn ArrayBuilder>,
}

impl ColumnBuilder {
    fn new(column: &Column) -> Result<Self> {
        let column_name = column.name();
        let column_type = column.column_type();

        let data_type = to_arrow_data_type(column_type);
        let builder = map_data_type_to_array_builder(&data_type);

        Ok(Self {
            name: column_name.to_string(),
            column_type,
            data_type,
            builder,
        })
    }
}

impl ColumnBuilder {
    fn append(&mut self, column: &Column, data: &ColumnData<'_>) -> Result<()> {
        if !self.name.eq_ignore_ascii_case(column.name()) {
            return Ok(());
        }

        let column_type = column.column_type();

        match data {
            ColumnData::U8(val) => {
                handle_primitive_type!(self, column_type, UInt8Builder, val.as_ref().copied())
            }
            ColumnData::I16(val) => {
                handle_primitive_type!(self, column_type, Int16Builder, val.as_ref().copied())
            }
            ColumnData::I32(val) => {
                handle_primitive_type!(self, column_type, Int32Builder, val.as_ref().copied())
            }
            ColumnData::I64(val) => {
                handle_primitive_type!(self, column_type, Int64Builder, val.as_ref().copied())
            }
            ColumnData::F32(val) => {
                handle_primitive_type!(self, column_type, Float32Builder, val.as_ref().copied())
            }
            ColumnData::F64(val) => {
                handle_primitive_type!(self, column_type, Float64Builder, val.as_ref().copied())
            }
            ColumnData::Bit(val) => {
                handle_primitive_type!(self, column_type, BooleanBuilder, val.as_ref().copied())
            }
            ColumnData::String(val) => {
                let str = val.as_ref().map(|str| str.to_string());
                handle_primitive_type!(self, column_type, StringBuilder, str)
            }
            ColumnData::Guid(val) => {
                let uuid = val.as_ref().map(Uuid::to_string);
                handle_primitive_type!(self, column_type, StringBuilder, uuid);
            }
            ColumnData::Binary(val) => {
                let bytes = val.as_ref().map(|bytes| bytes.to_vec());
                handle_primitive_type!(self, column_type, BinaryBuilder, bytes);
            }
            ColumnData::Numeric(_) => todo!(),
            ColumnData::Xml(val) => {
                let xml = val
                    .as_ref()
                    .map(|data| data.as_ref().clone())
                    .map(|data| data.into_string());
                handle_primitive_type!(self, column_type, LargeStringBuilder, xml);
            }
            ColumnData::DateTime(val) => {
                let dt = val.map(|dt| {
                    NaiveDateTime::new(
                        from_days(dt.days() as i64, 1900),
                        from_mins(dt.seconds_fragments()),
                    )
                });

                let epoch = dt.map(|dt| dt.and_utc().timestamp());
                handle_primitive_type!(self, column_type, TimestampNanosecondBuilder, epoch);
            }
            ColumnData::SmallDateTime(val) => {
                let dt = val.map(|dt| {
                    NaiveDateTime::new(
                        from_days(dt.days() as i64, 1900),
                        from_mins(dt.seconds_fragments() as u32 * 60),
                    )
                });

                let epoch = dt.map(|dt| dt.and_utc().timestamp());
                handle_primitive_type!(self, column_type, TimestampNanosecondBuilder, epoch);
            }
            ColumnData::Time(val) => {
                let time = val.map(|time| {
                    let ns = time.increments() as i64 * 10i64.pow(9 - time.scale() as u32);
                    ns
                });

                handle_primitive_type!(self, column_type, Time64NanosecondBuilder, time);
            }
            ColumnData::Date(val) => {
                let date = val.map(|date| from_days(date.days() as i64, 1));
                let days = date
                    .map(|date| date - EPOCH)
                    .map(|delta| delta.num_days() as i32);
                handle_primitive_type!(self, column_type, Date32Builder, days);
            }
            ColumnData::DateTime2(val) => {
                let dt = val.map(|dt2| {
                    let date = from_days(dt2.date().days() as i64, 1);
                    let ns =
                        dt2.time().increments() as i64 * 10i64.pow(9 - dt2.time().scale() as u32);
                    let time = NaiveTime::from_hms_opt(0, 0, 0).unwrap()
                        + chrono::Duration::nanoseconds(ns);
                    let naive = NaiveDateTime::new(date, time);

                    naive.and_utc()
                });
                let epoch = dt.map(|dt| dt.timestamp());
                handle_primitive_type!(self, column_type, TimestampNanosecondBuilder, epoch);
            }
            ColumnData::DateTimeOffset(val) => {
                let dt = val.map(|dto| {
                    let date = from_days(dto.datetime2().date().days() as i64, 1);
                    let ns = dto.datetime2().time().increments() as i64
                        * 10i64.pow(9 - dto.datetime2().time().scale() as u32);
                    let time = NaiveTime::from_hms_opt(0, 0, 0).unwrap()
                        + chrono::Duration::nanoseconds(ns);

                    let offset = chrono::Duration::minutes(dto.offset() as i64);
                    let naive = NaiveDateTime::new(date, time).sub(offset);
                    naive.and_utc()
                });
                let epoch = dt.map(|dt| dt.timestamp());
                handle_primitive_type!(self, column_type, TimestampNanosecondBuilder, epoch);
            }
        };

        Ok(())
    }

    fn finish(&mut self) -> ArrayRef {
        self.builder.finish()
    }
}

pub async fn stream_to_arrow<'a>(mut stream: QueryStream<'a>) -> Result<RecordBatch, Error> {
    let mut column_builders = Vec::new();
    let mut row_count = 0usize;

    while let Some(item) = stream.try_next().await? {
        match item {
            QueryItem::Metadata(metadata) => {
                column_builders = create_column_builders(metadata.columns())?;
            }

            QueryItem::Row(row) => {
                for ((column, data), builder) in row.cells().zip(column_builders.iter_mut()) {
                    builder.append(column, data)?;
                }

                row_count += 1;
            }
        }
    }

    to_record(column_builders, Some(row_count))
}

pub fn rows_to_arrow(rows: Vec<QueryItem>) -> Result<RecordBatch, Error> {
    let mut column_builders = Vec::new();
    let mut row_count = 0usize;

    for item in rows {
        match item {
            QueryItem::Metadata(metadata) => {
                column_builders = create_column_builders(metadata.columns())?;
            }

            QueryItem::Row(row) => {
                for ((column, data), builder) in row.cells().zip(column_builders.iter_mut()) {
                    builder.append(column, data)?;
                }

                row_count += 1;
            }
        }
    }

    to_record(column_builders, Some(row_count))
}

fn create_column_builders(columns: &[Column]) -> Result<Vec<ColumnBuilder>> {
    Ok(columns
        .iter()
        .map(ColumnBuilder::new)
        .collect::<Result<Vec<_>, _>>()?)
}

fn to_record(column_builders: Vec<ColumnBuilder>, row_count: Option<usize>) -> Result<RecordBatch> {
    let fields = column_builders.iter().map(|builder|
         Field::new(&builder.name, builder.data_type.clone(), true)
    ).collect::<Vec<_>>();
    let schema = Arc::new(Schema::new(fields));
    let columns = column_builders.into_iter().map(|mut builder| builder.finish()).collect();
    let options = RecordBatchOptions::new().with_row_count(row_count);
    RecordBatch::try_new_with_options(schema, columns, &options).map_err(Into::into)
}

fn to_arrow_data_type(column_type: ColumnType) -> DataType {
    match column_type {
        ColumnType::Null => DataType::Null,
        ColumnType::Bit => DataType::Boolean,
        ColumnType::Int1 => DataType::Int8,
        ColumnType::Int2 => DataType::Int16,
        ColumnType::Int4 => DataType::Int32,
        ColumnType::Int8 => DataType::Int64,
        ColumnType::Datetime4 => DataType::Timestamp(TimeUnit::Nanosecond, None),
        ColumnType::Float4 => DataType::Float32,
        ColumnType::Float8 => DataType::Float64,
        ColumnType::Money => DataType::Float64,
        ColumnType::Datetime => DataType::Timestamp(TimeUnit::Nanosecond, None),
        ColumnType::Money4 => DataType::Float32,
        ColumnType::Guid => DataType::Utf8,
        ColumnType::Intn => DataType::Int64,
        ColumnType::Bitn => DataType::Boolean,
        ColumnType::Decimaln => todo!(),
        ColumnType::Numericn => todo!(),
        ColumnType::Floatn => DataType::Float64,
        ColumnType::Datetimen => DataType::Timestamp(TimeUnit::Nanosecond, None),
        ColumnType::Daten => DataType::Date32,
        ColumnType::Timen => DataType::Time64(TimeUnit::Nanosecond),
        ColumnType::Datetime2 => DataType::Timestamp(TimeUnit::Nanosecond, None),
        ColumnType::DatetimeOffsetn => {
            DataType::Timestamp(TimeUnit::Nanosecond, Some("UTC".into()))
        }
        ColumnType::BigVarBin => DataType::LargeBinary,
        ColumnType::BigVarChar => DataType::LargeUtf8,
        ColumnType::BigBinary => DataType::LargeBinary,
        ColumnType::BigChar => DataType::LargeUtf8,
        ColumnType::NVarchar => DataType::Utf8,
        ColumnType::NChar => DataType::Utf8,
        ColumnType::Xml => DataType::LargeUtf8,
        ColumnType::Udt => DataType::Binary,
        ColumnType::Text => DataType::Utf8,
        ColumnType::Image => DataType::LargeBinary,
        ColumnType::NText => DataType::Utf8,
        ColumnType::SSVariant => DataType::Binary,
    }
}
