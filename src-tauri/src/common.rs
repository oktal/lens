//! Module that defines common types that are exchanged between the frontend and the backend

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq, Hash)]
pub struct StreamId(uuid::Uuid);

impl std::fmt::Display for StreamId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl StreamId {
    pub fn new() -> Self {
        Self(uuid::Uuid::new_v4())
    }
}

/// Represent a row read from a record batch
#[derive(Debug, Serialize)]
pub struct Row {
    /// Names of the columns present in this row
    pub columns: Vec<String>,

    /// Values of the corresponding columns
    pub values: Vec<String>,
}

/// Configuration for an object storage
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ObjectStoreConfig {
    #[serde(rename = "s3", rename_all = "camelCase")]
    AmazonS3 {
        access_key_id: String,
        secret_access_key: String,
        bucket: Option<String>,
        region: String,
    },

    #[serde(rename = "gcp", rename_all = "camelCase")]
    GoogleCloudStorage {
        service_account_path: Option<String>,
        service_acccount_key: Option<String>,
        application_credentials_path: Option<String>,
    },
}

/// Configuration for a data source
/// A data source is a location where data can be found and queried against
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatasourceConfig {
    pub url: String,
    pub store: ObjectStoreConfig,
}
