use super::id::ResId;
use petompp_web_models::models::blog_data::BlogMetaData;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use yewdux::prelude::*;

#[derive(PartialEq, Clone, Debug, Store, Serialize, Deserialize, Default)]
#[store(storage = "local")]
pub struct LocalStore {
    data: BTreeMap<String, (String, Option<BlogMetaData>)>,
}

impl LocalStore {
    pub fn get(&self, key: &ResId, lang: &str) -> Option<&(String, Option<BlogMetaData>)> {
        self.data.get(&(key.to_string() + lang))
    }

    pub fn get_mut(
        &mut self,
        key: &ResId,
        lang: &str,
    ) -> Option<&mut (String, Option<BlogMetaData>)> {
        self.data.get_mut(&(key.to_string() + lang))
    }

    pub fn insert(&mut self, key: ResId, lang: &str, value: String, meta: Option<BlogMetaData>) {
        self.data.insert(key.to_string() + lang, (value, meta));
    }

    pub fn remove(&mut self, key: &ResId, lang: &str) {
        self.data.remove(&(key.to_string() + lang));
    }
}
