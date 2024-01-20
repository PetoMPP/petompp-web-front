use crate::api::client::{ApiClient, BlobClient, RequestError};
use petompp_web_models::models::country::Country;
use serde::{Deserialize, Serialize};
use std::{fmt::Display, str::FromStr};

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct ResourceId {
    /// "<reskey>@<lang>"
    key: Option<String>,
    /// "<blob-type>@<folder-path>@<lang>"
    blob: Option<String>,
}

impl TryInto<(ResId, Country)> for ResourceId {
    type Error = String;

    fn try_into(self) -> Result<(ResId, Country), Self::Error> {
        match (self.key, self.blob) {
            (Some(key), None) => {
                let (path, lang) = key.split_once('@').ok_or("invalid key")?;
                Ok((
                    ResId::ResKey(path.to_string()),
                    Country::try_from(lang).map_err(|_| "invalid lang")?,
                ))
            }
            (None, Some(blob)) => {
                let (blob_type, path, lang) = blob
                    .split_once('@')
                    .and_then(|(bt, rest)| rest.split_once('@').map(|(p, l)| (bt, p, l)))
                    .ok_or("invalid blob")?;
                Ok((
                    match blob_type {
                        "blog" => ResId::Blob(BlobType::Blog(path.to_string())),
                        "prj" => ResId::Blob(BlobType::Project(path.to_string())),
                        _ => return Err("invalid blob type".to_string()),
                    },
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
            ResId::Blob(blob_type) => Self {
                key: None,
                blob: match blob_type {
                    BlobType::Blog(p) => Some(format!("blog@{}@{}", p, value.1.key())),
                    BlobType::Project(p) => Some(format!("prj@{}@{}", p, value.1.key())),
                },
            },
        }
    }
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, PartialOrd, Ord, Eq)]
pub enum BlobType {
    Blog(String),
    Project(String),
}

impl Display for BlobType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&match self {
            Self::Blog(id) => format!("blog:{}", id),
            Self::Project(id) => format!("prj:{}", id),
        })
    }
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, PartialOrd, Ord, Eq)]
pub enum ResId {
    ResKey(String),
    Blob(BlobType),
}

impl Display for ResId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            ResId::ResKey(id) => f.write_str(&format!("reskey:{}", id)),
            ResId::Blob(id) => id.fmt(f),
        }
    }
}

impl FromStr for ResId {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (kind, id) = s.split_once(':').ok_or("invalid id")?;
        match kind {
            "reskey" => Ok(Self::ResKey(id.to_string())),
            "blog" => Ok(Self::Blob(BlobType::Blog(id.to_string()))),
            "prj" => Ok(Self::Blob(BlobType::Project(id.to_string()))),
            _ => Err("invalid id"),
        }
    }
}

impl ResId {
    pub fn id(&self) -> &str {
        match self {
            Self::ResKey(id)
            | Self::Blob(BlobType::Blog(id))
            | Self::Blob(BlobType::Project(id)) => id,
        }
    }

    pub async fn get_value(&self, lang: &Country) -> Result<String, RequestError> {
        match self {
            Self::ResKey(reskey) => ApiClient::get_resource(reskey.as_str(), lang)
                .await
                .map(|(_, v)| v),
            Self::Blob(blob_type) => match blob_type {
                BlobType::Blog(id) => {
                    BlobClient::get_post_content(format!("{}/{}.md", id, lang.key()).as_str()).await
                }
                BlobType::Project(id) => todo!(),
            },
        }
    }
}
