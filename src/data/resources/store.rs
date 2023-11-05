use super::id::ResId;
use petompp_web_models::models::blog_data::BlogMetaData;
use serde::{Deserialize, Serialize};
use yewdux::prelude::*;

#[derive(PartialEq, Clone, Debug, Store, Serialize, Deserialize)]
#[store(storage = "local")]
pub struct LocalStore {
    blog_posts: Vec<(ResId, String, BlogMetaData)>,
    resources: Vec<(ResId, String)>,
}

impl LocalStore {
    pub fn get_blog_post(&self, resid: &ResId) -> Option<(&String, &BlogMetaData)> {
        self.blog_posts
            .iter()
            .find(|(r, _, _)| r == resid)
            .map(|(_, s, m)| (s, m))
    }

    pub fn get_blog_post_mut(
        &mut self,
        resid: &ResId,
    ) -> Option<(&mut String, &mut BlogMetaData)> {
        self.blog_posts
            .iter_mut()
            .find(|(r, _, _)| r == resid)
            .map(|(_, s, m)| (s, m))
    }

    pub fn get_resource(&self, resid: &ResId) -> Option<&String> {
        self.resources
            .iter()
            .find(|(r, _)| r == resid)
            .map(|(_, s)| s)
    }

    pub fn get_resource_mut(&mut self, resid: &ResId) -> Option<&mut String> {
        self.resources
            .iter_mut()
            .find(|(r, _)| r == resid)
            .map(|(_, s)| s)
    }
}

impl Default for LocalStore {
    fn default() -> Self {
        Self {
            blog_posts: vec![(
                ResId::Blob("test".to_string()),
                "test".to_string(),
                BlogMetaData {
                    id: "test".to_string(),
                    ..Default::default()
                },
            )],
            resources: Default::default(),
        }
    }
}
