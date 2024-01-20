use crate::pages::editor::EditorData;

use super::id::ResId;
use petompp_web_models::models::{blog_data::BlogMetaData, project_data::ProjectMetaData};
use serde::{Deserialize, Serialize};
use std::{collections::BTreeMap, str::FromStr};
use yewdux::prelude::*;

#[derive(PartialEq, Clone, Debug, Store, Serialize, Deserialize, Default)]
#[store(storage = "local")]
pub struct LocalStore {
    resources: BTreeMap<String, String>,
    blog_posts: BTreeMap<String, (String, BlogMetaData)>,
    projects: BTreeMap<String, (String, ProjectMetaData)>,
}

impl LocalStore {
    pub fn get(&self, key: &ResId, lang: &str) -> Option<EditorData> {
        let key = Self::key(key, lang);
        if let Some(value) = self.resources.get(&key) {
            return Some(EditorData::Resource(value.clone()));
        }
        if let Some((value, meta)) = self.blog_posts.get(&key) {
            return Some(EditorData::Blog((value.clone(), meta.clone())));
        }
        if let Some((value, meta)) = self.projects.get(&key) {
            return Some(EditorData::Project((value.clone(), meta.clone())));
        }
        None
    }

    pub fn get_all_resids(&self) -> Vec<ResId> {
        self.resources
            .keys()
            .chain(self.blog_posts.keys())
            .chain(self.projects.keys())
            .filter_map(|k| Self::key_lang(k).map(|(id, _)| id))
            .collect()
    }

    pub fn exists(&self, key: &ResId) -> bool {
        self.get_all_resids().iter().any(|x| x == key)
    }

    pub fn insert(&mut self, key: ResId, lang: &str, value: EditorData) {
        match value {
            EditorData::Resource(value) => {
                self.resources.insert(Self::key(&key, lang), value);
            }
            EditorData::Blog((value, meta)) => {
                self.blog_posts.insert(Self::key(&key, lang), (value, meta));
            }
            EditorData::Project((value, meta)) => {
                self.projects.insert(Self::key(&key, lang), (value, meta));
            }
        }
    }

    pub fn remove(&mut self, key: &ResId, lang: &str) {
        let key = Self::key(key, lang);
        self.resources.remove(&key);
        self.blog_posts.remove(&key);
        self.projects.remove(&key);
    }

    fn key(key: &ResId, lang: &str) -> String {
        key.to_string() + "." + lang
    }

    fn key_lang(key: &String) -> Option<(ResId, &str)> {
        let x = key
            .split('.')
            .fold((0usize, 0), |acc, x| (acc.0 + x.len() + 1, x.len()));
        let x = x.0 - x.1 - 1 - 1;
        Some((ResId::from_str(&key[..x]).ok()?, &key[x..]))
    }
}
