//! Module that defines commands related to AWS

use crate::{aws, lens::LensResult};

#[tauri::command]
pub async fn aws_sso_login(
    start_url: String,
    region: String,
    account_id: String,
    role_name: String,
) -> LensResult<(String, String)> {
    let credentials = aws::sso_login(start_url, region, account_id, role_name).await?;
    Ok(credentials)
}
