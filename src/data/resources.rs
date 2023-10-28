use crate::api::client::{ApiClient, BlobClient, RequestError};
use petompp_web_models::models::country::Country;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct ResourceId {
    /// "<reskey>@<lang>"
    key: Option<String>,
    /// "<folder-path>@<lang>"
    blob: Option<String>,
}

impl TryInto<(ResId, Country)> for ResourceId {
    type Error = String;

    fn try_into(self) -> Result<(ResId, Country), Self::Error> {
        match (self.key, self.blob) {
            (Some(key), None) => {
                let (path, lang) = key.split_once("@").ok_or("invalid key")?;
                Ok((
                    ResId::ResKey(path.to_string()),
                    Country::try_from(lang).map_err(|_| "invalid lang")?,
                ))
            }
            (None, Some(blob)) => {
                let (path, lang) = blob.split_once("@").ok_or("invalid blob")?;
                Ok((
                    ResId::Blob(path.to_string()),
                    Country::try_from(lang).map_err(|_| "invalid lang")?,
                ))
            }
            _ => Err("invalid resource id".to_string()),
        }
    }
}

impl From<(ResId, Country)> for ResourceId {
    fn from(value: (ResId, Country)) -> Self {
        match value.0 {
            ResId::ResKey(p) => Self {
                key: Some(format!("{}@{}", p, value.1.key())),
                blob: None,
            },
            ResId::Blob(p) => Self {
                key: None,
                blob: Some(format!("{}@{}", p, value.1.key())),
            },
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ResId {
    ResKey(String),
    Blob(String),
}

impl ResId {
    pub fn id(&self) -> &str {
        match self {
            Self::ResKey(id) | Self::Blob(id) => id,
        }
    }

    pub fn type_str(&self) -> &'static str {
        match self {
            Self::ResKey(_) => "resource",
            Self::Blob(_) => "blog post",
        }
    }

    pub async fn get_value(&self, lang: &Country) -> Result<String, RequestError> {
        match self {
            Self::ResKey(reskey) => ApiClient::get_resource(reskey.as_str(), lang).await,
            Self::Blob(path) => {
                BlobClient::get_post_content(format!("{}/{}.md", path, lang.key()).as_str()).await
            }
        }
    }
}
