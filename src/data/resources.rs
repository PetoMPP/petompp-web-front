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

impl TryInto<ResId> for ResourceId {
    type Error = String;

    fn try_into(self) -> Result<ResId, Self::Error> {
        match (self.key, self.blob) {
            (Some(key), None) => {
                let (path, lang) = key.split_once("@").ok_or("invalid key")?;
                Ok(ResId::ResKey((
                    path.to_string(),
                    Country::try_from(lang).map_err(|_| "invalid lang")?,
                )))
            }
            (None, Some(blob)) => {
                let (path, lang) = blob.split_once("@").ok_or("invalid blob")?;
                Ok(ResId::Blob((
                    path.to_string(),
                    Country::try_from(lang).map_err(|_| "invalid lang")?,
                )))
            }
            _ => Err("invalid resource id".to_string()),
        }
    }
}

impl From<ResId> for ResourceId {
    fn from(value: ResId) -> Self {
        match value {
            ResId::ResKey((p, l)) => Self {
                key: Some(format!("{}@{}", p, l.key())),
                blob: None,
            },
            ResId::Blob((p, l)) => Self {
                key: None,
                blob: Some(format!("{}@{}", p, l.key())),
            },
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ResId {
    ResKey((String, Country)),
    Blob((String, Country)),
}

impl ResId {
    pub fn with_lang(self, lang: Country) -> Self {
        match self {
            Self::ResKey((p, _)) => Self::ResKey((p, lang)),
            Self::Blob((p, _)) => Self::Blob((p, lang)),
        }
    }

    pub fn lang(&self) -> &Country {
        match self {
            Self::ResKey((_, l)) | Self::Blob((_, l)) => l,
        }
    }

    pub async fn get_value(&self) -> Result<String, RequestError> {
        match self {
            Self::ResKey((reskey, lang)) => {
                ApiClient::get_resource(reskey.as_str(), lang.key()).await
            }
            Self::Blob((path, lang)) => {
                BlobClient::get_post_content(format!("{}/{}.md", path, lang.key()).as_str()).await
            }
        }
    }
}
