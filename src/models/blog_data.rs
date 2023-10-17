use super::tag::Tags;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BlogMetaData {
    pub title: String,
    pub tags: Tags,
    pub created: DateTime<Utc>,
    pub updated: DateTime<Utc>,
    pub image: Option<String>,
}

impl BlogMetaData {
    pub fn new(title: impl ToString, tags: Tags, created: DateTime<Utc>) -> Self {
        Self {
            title: title.to_string(),
            tags,
            created: created.clone(),
            updated: created,
            image: Default::default(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BlogSummaryData {
    pub meta: BlogMetaData,
    pub summary: String,
}

impl BlogSummaryData {
    pub fn from_meta(meta: BlogMetaData, summary: impl ToString) -> Self {
        Self {
            meta,
            summary: summary.to_string(),
        }
    }
}
