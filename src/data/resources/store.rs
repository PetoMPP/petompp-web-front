use super::id::ResId;
use petompp_web_models::models::blog_data::BlogMetaData;
use serde::{Deserialize, Serialize};
use std::{collections::BTreeMap, str::FromStr};
use yewdux::prelude::*;

#[derive(PartialEq, Clone, Debug, Store, Serialize, Deserialize, Default)]
#[store(storage = "local")]
pub struct LocalStore {
    data: BTreeMap<String, (String, Option<BlogMetaData>)>,
}

impl LocalStore {
    pub fn get(&self, key: &ResId, lang: &str) -> Option<&(String, Option<BlogMetaData>)> {
        self.data.get(&(key.to_string() + "." + lang))
    }

    pub fn get_mut(
        &mut self,
        key: &ResId,
        lang: &str,
    ) -> Option<&mut (String, Option<BlogMetaData>)> {
        self.data.get_mut(&(key.to_string() + "." + lang))
    }

    pub fn get_all_resids(&self) -> Vec<ResId> {
        self.data
            .keys()
            .filter_map(|k| {
                let x = k
                    .split('.')
                    .fold((0usize, 0), |acc, x| (acc.0 + x.len() + 1, x.len()));
                let x = x.0 - x.1 - 1 - 1;
                ResId::from_str(&k[..x]).ok()
            })
            .collect()
    }

    pub fn exists(&self, key: &ResId) -> bool {
        self.get_all_resids().iter().any(|x| x == key)
    }

    pub fn insert(&mut self, key: ResId, lang: &str, value: String, meta: Option<BlogMetaData>) {
        self.data
            .insert(key.to_string() + "." + lang, (value, meta));
    }

    pub fn remove(&mut self, key: &ResId, lang: &str) {
        self.data.remove(&(key.to_string() + "." + lang));
    }
}
