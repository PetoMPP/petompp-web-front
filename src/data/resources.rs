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
