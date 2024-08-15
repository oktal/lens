use std::time::Duration;

use anyhow::{anyhow, Context};
use aws_config::{BehaviorVersion, Region, SdkConfig};
use aws_sdk_sso::Client as SsoClient;
use aws_sdk_ssooidc::{
    operation::{
        create_token::{CreateTokenError, CreateTokenOutput},
        start_device_authorization::StartDeviceAuthorizationOutput,
    },
    Client as OidcClient,
};

async fn poll_token(
    client: &OidcClient,
    client_id: &str,
    client_secret: &str,
    auth: &StartDeviceAuthorizationOutput,
) -> anyhow::Result<CreateTokenOutput> {
    let interval = Duration::from_secs(auth.interval() as u64);

    Ok(loop {
        tokio::time::sleep(interval).await;

        match client
            .create_token()
            .client_id(client_id)
            .client_secret(client_secret)
            .device_code(auth.device_code().unwrap_or_default())
            .grant_type("urn:ietf:params:oauth:grant-type:device_code")
            .send()
            .await
        {
            Ok(response) => break response,
            Err(e) => {
                if let Some(CreateTokenError::AuthorizationPendingException(_)) =
                    e.as_service_error()
                {
                    continue;
                }

                return Err(e.into());
            }
        }
    })
}

async fn get_access_token(config: &SdkConfig, start_url: String) -> anyhow::Result<String> {
    let client = OidcClient::new(&config);

    let client_credentials = client
        .register_client()
        .client_name("datalens")
        .client_type("public")
        .send()
        .await
        .context("register oidc client")?;

    let client_id = client_credentials
        .client_id()
        .ok_or(anyhow!("missing client id"))?;

    let client_secret = client_credentials
        .client_secret()
        .ok_or(anyhow!("missing client secret"))?;

    let auth_response = client
        .start_device_authorization()
        .client_id(client_id)
        .client_secret(client_secret)
        .start_url(start_url)
        .send()
        .await
        .context("authorize device")?;

    let verification_uri = auth_response
        .verification_uri_complete()
        .ok_or(anyhow!("missing verification URI"))?;

    open::that(&verification_uri).with_context(|| format!("open {verification_uri}"))?;
    let token = poll_token(&client, client_id, client_secret, &auth_response).await?;
    let access_token = token
        .access_token()
        .map(ToString::to_string)
        .ok_or(anyhow!("missing access token"))?;

    Ok(access_token)
}

async fn get_credentials(
    config: &SdkConfig,
    access_token: String,
    account_id: String,
    role_name: String,
) -> anyhow::Result<(String, String)> {
    let client = SsoClient::new(&config);
    let account_roles = client
        .list_account_roles()
        .access_token(&access_token)
        .account_id(&account_id)
        .send()
        .await
        .context("list account roles")?;

    let account_role = account_roles
        .role_list()
        .iter()
        .find(|info| {
            info.role_name()
                .expect("missing role name")
                .eq_ignore_ascii_case(&role_name)
        })
        .ok_or(anyhow!("unknown role {role_name}"))?;

    let credentials = client
        .get_role_credentials()
        .role_name(account_role.role_name().expect("missing role name"))
        .account_id(account_id)
        .access_token(access_token)
        .send()
        .await
        .context("get role credentials")?;

    let credentials = credentials
        .role_credentials()
        .expect("missing role credentials");

    let access_key_id = credentials
        .access_key_id()
        .ok_or(anyhow!("missing access key id"))?
        .to_string();
    let secret_key = credentials
        .secret_access_key()
        .ok_or(anyhow!("missing secret key"))?
        .to_string();

    Ok((access_key_id, secret_key))
}

pub async fn sso_login(
    start_url: String,
    region: String,
    account_id: String,
    role_name: String,
) -> anyhow::Result<(String, String)> {
    let config = aws_config::defaults(BehaviorVersion::v2024_03_28())
        .region(Region::new(region))
        .load()
        .await;

    let access_token = get_access_token(&config, start_url).await?;
    get_credentials(&config, access_token, account_id, role_name).await
}
