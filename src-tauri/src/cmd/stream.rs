//! Module that defines commands related to stream manipulation

use crate::{
    common::{ExportOptions, StreamId, StreamInfo},
    lens::{Lens, LensResult},
};

#[tauri::command]
pub async fn stream_export(
    lens: tauri::State<'_, Lens>,
    id: StreamId,
    options: ExportOptions,
) -> LensResult<usize> {
    lens.stream_export(id, options).await
}

#[tauri::command]
pub async fn stream_list(lens: tauri::State<'_, Lens>) -> LensResult<Vec<StreamInfo>> {
    lens.stream_list().await
}
