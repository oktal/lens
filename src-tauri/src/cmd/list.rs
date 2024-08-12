//! Module that defines commands to list previously created or already existing resources

use crate::{common::DatasourceConfig, lens::Lens};

#[tauri::command]
pub fn list_datasources(lens: tauri::State<'_, Lens>) -> Vec<DatasourceConfig> {
    lens.datasources()
}
