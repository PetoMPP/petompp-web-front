use super::client::{ApiClient, RequestError};
use petompp_web_models::{
    error::Error,
    models::blob::blob_meta::{BlobMetaData, BlobUpload},
};
use reqwasm::http::{Method, Request};
use serde::{de::DeserializeOwned, Deserialize};
use web_sys::RequestCache;

lazy_static::lazy_static! {
    static ref AZURE_STORAGE_URL: String =
        match std::option_env!("AZURE_STORAGE_URL").unwrap_or_default() {
            url if url.ends_with('/') => url.to_string(),
            url => format!("{}/", url)
        };
}

#[yewdux::async_trait(?Send)]
pub trait BlobClient {
    fn get_url(container: &str, filename: &str) -> String {
        format!("{}{}/{}", *AZURE_STORAGE_URL, container, filename)
    }
    async fn get_meta<TBlob: TryFrom<BlobMetaData> + DeserializeOwned>(
        container: &str,
        filename: &str,
    ) -> Result<TBlob, RequestError>;
    async fn get_meta_all<TBlob: TryFrom<BlobMetaData> + DeserializeOwned>(
        container: &str,
        prefix: Option<&str>,
    ) -> Result<Vec<TBlob>, RequestError>;
    async fn get_names(container: &str, prefix: Option<&str>) -> Result<Vec<String>, RequestError>;
    async fn get_content(container: &str, filename: &str) -> Result<Vec<u8>, RequestError>;
    async fn get_content_str(container: &str, filename: &str) -> Result<String, RequestError> {
        let content = Self::get_content(container, filename).await?;
        String::from_utf8(content).map_err(|e| RequestError::Parse(e.to_string()))
    }
    async fn create_or_update(
        token: &str,
        container: &str,
        upload: &BlobUpload,
    ) -> Result<String, RequestError>;
    async fn delete(token: &str, container: &str, filename: &str) -> Result<(), RequestError>;
}

#[derive(Deserialize)]
struct NameResp {
    #[serde(rename(deserialize = "Name"))]
    name: Vec<String>,
}

#[derive(Deserialize)]
struct FullResp<T> {
    #[serde(rename(deserialize = "Full"))]
    full: Vec<T>,
}

#[yewdux::async_trait(?Send)]
impl BlobClient for ApiClient {
    async fn get_meta<TBlob: TryFrom<BlobMetaData> + DeserializeOwned>(
        container: &str,
        filename: &str,
    ) -> Result<TBlob, RequestError> {
        Self::send_json(
            Method::GET,
            format!("api/v1/blob/{}/{}", container, filename).as_str(),
            None,
            Option::<&String>::None,
        )
        .await
    }
    async fn get_meta_all<TBlob: TryFrom<BlobMetaData> + DeserializeOwned>(
        container: &str,
        prefix: Option<&str>,
    ) -> Result<Vec<TBlob>, RequestError> {
        Self::send_json::<FullResp<TBlob>>(
            Method::GET,
            format!(
                "api/v1/blob/{}?data=full{}",
                container,
                prefix.map(|s| format!("&prefix={}", s)).unwrap_or_default()
            )
            .as_str(),
            None,
            Option::<&String>::None,
        )
        .await
        .map(|r| r.full)
    }
    async fn get_names(container: &str, prefix: Option<&str>) -> Result<Vec<String>, RequestError> {
        Self::send_json::<NameResp>(
            Method::GET,
            format!(
                "api/v1/blob/{}?data=name{}",
                container,
                prefix.map(|s| format!("&prefix={}", s)).unwrap_or_default()
            )
            .as_str(),
            None,
            Option::<&String>::None,
        )
        .await
        .map(|r| r.name)
    }
    async fn get_content(container: &str, filename: &str) -> Result<Vec<u8>, RequestError> {
        let response = &Request::new(Self::get_url(container, filename).as_str())
            .method(Method::GET)
            .cache(RequestCache::NoCache)
            .send()
            .await
            .map_err(|e| RequestError::Network(e.to_string()))?;

        if response.status() == 404 {
            return Err(RequestError::Endpoint(
                404,
                Error::Status(404, "Not found".to_string()),
            ));
        }

        response
            .binary()
            .await
            .map_err(|e| RequestError::Network(e.to_string()))
    }
    async fn create_or_update(
        token: &str,
        container: &str,
        upload: &BlobUpload,
    ) -> Result<String, RequestError> {
        Self::send_multipart(
            Method::POST,
            format!("api/v1/blob/{}", container).as_str(),
            Some(token),
            upload,
        )
        .await
    }
    async fn delete(token: &str, container: &str, filename: &str) -> Result<(), RequestError> {
        Self::send_json(
            Method::DELETE,
            format!("api/v1/blob/{}/{}", container, filename).as_str(),
            Some(token),
            Option::<&String>::None,
        )
        .await
        .map(|_: String| ())
    }
}
