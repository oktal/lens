//! Module that defines commands related to AWS

use std::collections::HashMap;

use aws_runtime::env_config::file::EnvConfigFiles;
use aws_types::os_shim_internal::{Env, Fs};

use crate::{aws, common::AwsSsoProfile, lens::LensResult};

const SSO_SESSION_KEY: &str = "sso_session";
const SSO_REGION_KEY: &str = "sso_region";
const SSO_START_URL_KEY: &str = "sso_start_url";
const SSO_ACCOUNT_ID_KEY: &str = "sso_account_id";
const SSO_ROLE_NAME_KEY: &str = "sso_role_name";

#[tauri::command]
pub async fn aws_sso_login(
    start_url: String,
    region: String,
    account_id: String,
    role_name: String,
) -> LensResult<(String, String, String)> {
    let credentials = aws::sso_login(start_url, region, account_id, role_name).await?;
    Ok(credentials)
}

#[tauri::command]
pub async fn list_aws_sso_profiles() -> LensResult<Vec<AwsSsoProfile>> {
    let profile_files = EnvConfigFiles::builder()
        .include_default_config_file(true)
        .build();

    let config = aws_config::profile::load(&Fs::real(), &Env::real(), &profile_files, None).await?;

    let sso_sessions = config
        .sso_sessions()
        .filter_map(|name| config.sso_session(name).map(|session| (name, session)))
        .collect::<HashMap<_, _>>();

    let sso_profiles = config
        .profiles()
        .filter_map(|name| {
            let profile = config.get_profile(name)?;
            let sso_session_name = profile.get(SSO_SESSION_KEY)?;
            let sso_session = sso_sessions.get(sso_session_name)?;

            let start_url = sso_session.get(SSO_START_URL_KEY)?;
            let region = sso_session.get(SSO_REGION_KEY)?;

            let account_id = profile.get(SSO_ACCOUNT_ID_KEY)?;
            let role_name = profile.get(SSO_ROLE_NAME_KEY)?;

            Some(AwsSsoProfile {
                name: name.to_string(),
                region: region.to_string(),
                start_url: start_url.to_string(),
                account_id: account_id.to_string(),
                role_name: role_name.to_string(),
            })
        })
        .collect();

    Ok(sso_profiles)
}
