//! Module that defines tauri commands to create resources

use crate::{
    common::DatasourceConfig,
    lens::{Lens, LensResult},
};

#[tauri::command]
pub fn create_datasource(lens: tauri::State<'_, Lens>, config: DatasourceConfig) -> LensResult<()> {
    lens.register_datasource(config)?;
    Ok(())
}
