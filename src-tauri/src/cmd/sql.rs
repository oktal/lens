//! Module that defines commands relative to SQL queries execution

use crate::common::{Row, StreamId};
use crate::lens::{Lens, LensResult};

#[tauri::command]
pub async fn sql(lens: tauri::State<'_, Lens>, query: String) -> LensResult<()> {
    lens.sql(&query).await?;
    Ok(())
}

#[tauri::command]
pub async fn sql_stream(lens: tauri::State<'_, Lens>, query: String) -> LensResult<StreamId> {
    lens.stream(&query).await
}

#[tauri::command]
pub async fn sql_next(
    lens: tauri::State<'_, Lens>,
    stream_id: StreamId,
) -> LensResult<Option<Vec<Row>>> {
    lens.stream_next(stream_id).await
}
