use std::{path, str::FromStr};

use super::{
    blob::BlobClient,
    client::{ApiClient, RequestError},
    resource::ResourceClient,
};
use crate::{
    data::resources::id::{BlobType, ResId},
    pages::editor::EditorData,
};
use petompp_web_models::models::{
    blob::{blog::BlogMetaData, project::ProjectMetaData},
    country::Country,
};

#[yewdux::async_trait(?Send)]
pub trait EditorClient: BlobClient + ResourceClient {
    async fn get_res_ids(token: &str)
        -> Result<(Vec<ResId>, Vec<ResId>, Vec<ResId>), RequestError>;
    async fn get_data(
        blob_type: &BlobType,
        lang: Country,
    ) -> Result<Option<EditorData>, RequestError>;
}

#[yewdux::async_trait(?Send)]
impl EditorClient for ApiClient {
    async fn get_res_ids(
        token: &str,
    ) -> Result<(Vec<ResId>, Vec<ResId>, Vec<ResId>), RequestError> {
        let (res_keys, blog_posts, projects) = futures::join!(
            Self::get_resource_keys(&token),
            Self::get_meta_all::<BlogMetaData>("blog", None),
            Self::get_meta_all::<ProjectMetaData>("project", None)
        );
        let res_keys = not_found_as_empty(res_keys)?;
        let mut blog_posts = not_found_as_empty(blog_posts)?;
        blog_posts.sort_by(|a, b| a.filename.cmp(&b.filename));
        blog_posts.dedup_by_key(|b| b.id().to_string());
        let mut projects = not_found_as_empty(projects)?;
        projects.sort_by(|a, b| a.filename.cmp(&b.filename));
        projects.dedup_by_key(|p| p.id().to_string());
        Ok((
            res_keys.into_iter().map(|k| ResId::ResKey(k)).collect(),
            blog_posts
                .into_iter()
                .map(|b| ResId::Blob(BlobType::Blog(b.id().to_string())))
                .collect(),
            projects
                .into_iter()
                .map(|p| ResId::Blob(BlobType::Project(p.id().to_string())))
                .collect(),
        ))
    }
    async fn get_data(
        blob_type: &BlobType,
        lang: Country,
    ) -> Result<Option<EditorData>, RequestError> {
        match blob_type {
            BlobType::Blog(id) => {
                let filename = format!("{}/{}.md", id, lang.key());
                let container = "blog";
                match ApiClient::get_meta(container, filename.as_str()).await {
                    Ok(m) => Ok(Some(EditorData::Blog((
                        ApiClient::get_content_str(container, filename.as_str()).await?,
                        m,
                    )))),
                    // does it exist in another language?
                    Err(RequestError::Endpoint(404, _)) => {
                        let path = path::PathBuf::from_str(&filename)
                            .map_err(|e| RequestError::Parse(e.to_string()))?;
                        let prefix = path
                            .parent()
                            .and_then(|p| p.to_str())
                            .ok_or(RequestError::Parse("file has no parent".to_string()))?;
                        Ok(ApiClient::get_names(container, Some(prefix))
                            .await
                            .map(|_| None)?)
                    }
                    Err(e) => Err(e),
                }
            }
            BlobType::Project(id) => {
                let filename = format!("{}/{}.md", id, lang.key());
                let container = "project";
                match ApiClient::get_meta(container, filename.as_str()).await {
                    Ok(m) => Ok(Some(EditorData::Project((
                        ApiClient::get_content_str(container, filename.as_str()).await?,
                        m,
                    )))),
                    // does it exist in another language?
                    Err(RequestError::Endpoint(404, _)) => {
                        let path = path::PathBuf::from_str(&filename)
                            .map_err(|e| RequestError::Parse(e.to_string()))?;
                        let prefix = path
                            .parent()
                            .and_then(|p| p.to_str())
                            .ok_or(RequestError::Parse("file has no parent".to_string()))?;
                        Ok(ApiClient::get_names(container, Some(prefix))
                            .await
                            .map(|_| None)?)
                    }
                    Err(e) => Err(e),
                }
            }
        }
    }
}

fn not_found_as_empty<T>(e: Result<Vec<T>, RequestError>) -> Result<Vec<T>, RequestError> {
    match e {
        Ok(v) => Ok(v),
        Err(RequestError::Endpoint(404, _)) => Ok(vec![]),
        Err(e) => Err(e),
    }
}
