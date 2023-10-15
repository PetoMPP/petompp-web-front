use serde::{Deserialize, Serialize};
use super::tag::Tags;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BlogMetaData {
    pub title: String,
    pub tags: Tags,
    pub created: String,
    pub updated: String,
    pub image: Option<String>,
}

impl BlogMetaData {
    pub fn new(title: impl ToString, tags: Tags, created: impl ToString) -> Self {
        Self {
            title: title.to_string(),
            tags,
            created: created.to_string(),
            updated: created.to_string(),
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
        Self { meta, summary: summary.to_string() }
    }
}
