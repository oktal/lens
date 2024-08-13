//! Module that defines commands relative to SQL queries execution

use crate::lens::{Lens, LensResult};

#[tauri::command]
pub async fn sql(lens: tauri::State<'_, Lens>, query: String) -> LensResult<()> {
    lens.sql(&query).await?;
    Ok(())
}
