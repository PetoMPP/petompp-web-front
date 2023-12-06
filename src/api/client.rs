use crate::{
    data::{resources::id::ResId, session::SessionStore},
    pages::login::LoginRedirect,
};
use petompp_web_models::{
    error::Error,
    models::{
        blog_data::{BlogData, BlogMetaData},
        country::Country,
        credentials::Credentials,
        resource_data::ResourceData,
        user::UserData,
    },
};
use reqwasm::http::*;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use serde_json::Value;
use std::{collections::HashMap, fmt::Display};
use yew::{html, virtual_dom::VNode};
use yewdux::prelude::Dispatch;

#[derive(Debug, Clone, PartialEq)]
pub enum RequestError {
    Endpoint(u16, Error),
    Parse(String),
    Network(String),
}

impl Display for RequestError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{:?}", self))
    }
}

impl RequestError {
    pub fn handle_failed_auth(
        &self,
        session_dispatch: Dispatch<SessionStore>,
    ) -> Result<(), VNode> {
        if let Self::Endpoint(401..=403, _) = self {
            session_dispatch.reduce(|_| {
                SessionStore {
                    token: None,
                    user: None,
                }
                .into()
            });
            return Err(html! {<LoginRedirect />});
        }
        Ok(())
    }
}

impl std::error::Error for RequestError {}

pub enum Response<T> {
    Success(T),
    Error(u16, Error),
}

impl<T: DeserializeOwned> Response<T> {
    async fn from_response(value: reqwasm::http::Response) -> Result<Self, RequestError> {
        let status_code = value.status();
        let body = value
            .json::<Value>()
            .await
            .map_err(|e| RequestError::Parse(e.to_string()))?;
        let status = body
            .get("status")
            .ok_or(RequestError::Parse("Missing status".to_string()))?
            .as_str()
            .ok_or(RequestError::Parse("Invalid status format".to_string()))?;
        let data = body
            .get("data")
            .ok_or(RequestError::Parse("Missing data".to_string()))?;
        match status {
            "success" => Ok(Response::Success(
                serde_json::from_value(data.clone())
                    .map_err(|e| RequestError::Parse(format!("Invalid data format, {}", e)))?,
            )),
            "error" => Ok(Response::Error(
                status_code,
                serde_json::from_value(data.clone())
                    .map_err(|e| RequestError::Parse(format!("Invalid data format, {}", e)))?,
            )),
            _ => Err(RequestError::Parse("Invalid status".to_string())),
        }
    }
}

lazy_static::lazy_static! {
    static ref API_URL: String = match std::option_env!("API_URL").unwrap_or_default() {
        url if url.ends_with('/') => url.to_string(),
        url => format!("{}/", url)
    };
    static ref AZURE_STORAGE_URL: String = match std::option_env!("AZURE_STORAGE_URL").unwrap_or_default() {
        url if url.ends_with('/') => url.to_string(),
        url => format!("{}/", url)
    };
}

#[derive(Serialize, Deserialize)]
pub struct LoginResponse {
    pub token: String,
    pub user: UserData,
}

pub struct ApiClient;

impl ApiClient {
    fn get_url(path: &str) -> String {
        format!("{}{}", *API_URL, path)
    }

    async fn send_json<R: DeserializeOwned>(
        method: Method,
        path: &str,
        token: Option<&str>,
        body: Option<&impl Serialize>,
    ) -> Result<R, RequestError> {
        let mut request = Request::new(Self::get_url(path).as_str()).method(method);
        if let Some(token) = token {
            request = request.header("Authorization", format!("Bearer {}", token).as_str());
        }
        if let Some(body) = body {
            request = request
                .body(serde_json::to_string(body).map_err(|e| RequestError::Parse(e.to_string()))?);
        }

        let response = request
            .send()
            .await
            .map_err(|e| RequestError::Network(e.to_string()))?;

        match Response::from_response(response).await? {
            Response::Success(data) => Ok(data),
            Response::Error(s, e) => Err(RequestError::Endpoint(s, e)),
        }
    }

    pub async fn login(credentials: Credentials) -> Result<LoginResponse, RequestError> {
        Self::send_json(Method::POST, "api/v1/users/login", None, Some(&credentials)).await
    }

    pub async fn register(credentials: Credentials) -> Result<(), RequestError> {
        Self::send_json::<UserData>(Method::POST, "api/v1/users", None, Some(&credentials))
            .await
            .map(|_| ())
    }

    pub async fn get_users(token: &str) -> Result<Vec<UserData>, RequestError> {
        Self::send_json(
            Method::GET,
            "api/v1/users/all?range=all",
            Some(token),
            Option::<&String>::None,
        )
        .await
        .map(|u: Vec<Vec<UserData>>| u[0].clone())
    }

    pub async fn activate_user(token: &str, id: i32) -> Result<(), RequestError> {
        Self::send_json::<UserData>(
            Method::POST,
            format!("api/v1/users/{}/activate", id).as_str(),
            Some(token),
            Option::<&String>::None,
        )
        .await
        .map(|_| ())
    }

    pub async fn delete_user(token: &str, id: i32) -> Result<(), RequestError> {
        Self::send_json::<UserData>(
            Method::DELETE,
            format!("api/v1/users/{}", id).as_str(),
            Some(token),
            Option::<&String>::None,
        )
        .await
        .map(|_| ())
    }

    pub async fn create_resource(
        token: &str,
        key: &str,
        lang: &Country,
        value: &str,
    ) -> Result<(), RequestError> {
        let resource = ResourceData::new_from_lang(key, lang, value);
        Self::send_json(
            Method::PUT,
            format!("api/v1/res/{}", key).as_str(),
            Some(token),
            Some(&resource),
        )
        .await
        .map(|_: ResourceData| ())
    }

    pub async fn get_resource(
        key: &str,
        lang: &Country,
    ) -> Result<(Country, String), RequestError> {
        Self::send_json(
            Method::GET,
            format!("api/v1/res/{}?lang={}", key, lang.key()).as_str(),
            None,
            Option::<&String>::None,
        )
        .await
    }

    pub async fn get_resource_keys(token: &str) -> Result<Vec<String>, RequestError> {
        Self::send_json(
            Method::GET,
            "api/v1/res/keys",
            Some(token),
            Option::<&String>::None,
        )
        .await
    }

    pub async fn update_resource(
        token: &str,
        key: &str,
        lang: &Country,
        value: &str,
    ) -> Result<(), RequestError> {
        let resource = ResourceData::new_from_lang(key, lang, value);
        Self::send_json(
            Method::POST,
            format!("api/v1/res/{}", key).as_str(),
            Some(token),
            Some(&resource),
        )
        .await
        .map(|_: ResourceData| ())
    }

    pub async fn delete_resource(token: &str, key: &str) -> Result<(), RequestError> {
        Self::send_json(
            Method::DELETE,
            format!("api/v1/res/{}", key).as_str(),
            Some(token),
            Option::<&String>::None,
        )
        .await
        .map(|_: String| ())
    }

    pub async fn delete_resource_lang(
        token: &str,
        key: &str,
        lang: &Country,
    ) -> Result<(), RequestError> {
        Self::send_json(
            Method::DELETE,
            format!("api/v1/res/{}?lang={}", key, lang.key()).as_str(),
            Some(token),
            Option::<&String>::None,
        )
        .await
        .map(|_: String| ())
    }

    pub async fn get_img_paths() -> Result<Vec<String>, RequestError> {
        Self::send_json(Method::GET, "api/v1/img/", None, Option::<&String>::None).await
    }

    pub async fn upload_img(
        token: &str,
        img: web_sys::File,
        folder: &str,
        name: Option<&str>,
    ) -> Result<String, RequestError> {
        let query = match name {
            Some(name) => format!("?folder={}&filename={}", folder, name),
            None => format!("?folder={}", folder),
        };
        let url = Self::get_url(format!("api/v1/img/{}", query).as_str());
        let resp = Request::new(url.as_str())
            .method(Method::PUT)
            .header("Authorization", format!("Bearer {}", token).as_str())
            .body(img)
            .send()
            .await
            .map_err(|e| RequestError::Network(e.to_string()))?;
        match Response::<String>::from_response(resp).await? {
            Response::Success(filename) => Ok(BlobClient::get_url(
                format!("image-upload/{}/{}", folder, filename).as_str(),
            )),
            Response::Error(s, e) => Err(RequestError::Endpoint(s, e)),
        }
    }

    pub async fn delete_img(token: &str, pattern: &str) -> Result<(), RequestError> {
        Self::send_json(
            Method::DELETE,
            format!("api/v1/img/?pattern={}", pattern).as_str(),
            Some(token),
            Option::<&String>::None,
        )
        .await
        .map(|_: usize| ())
    }

    pub async fn get_posts_meta(prefix: Option<String>) -> Result<Vec<BlogMetaData>, RequestError> {
        let path = match prefix {
            Some(prefix) => format!("api/v1/blog/meta/?prefix={}", prefix),
            None => "api/v1/blog/meta/".to_string(),
        };
        Self::send_json(Method::GET, path.as_str(), None, Option::<&String>::None).await
    }

    pub async fn get_post_meta(id: &str, lang: &str) -> Result<BlogMetaData, RequestError> {
        Self::send_json(
            Method::GET,
            format!("api/v1/blog/meta/{}/{}", id, lang).as_str(),
            None,
            Option::<&String>::None,
        )
        .await
    }

    pub async fn create_or_update_post(
        id: &str,
        lang: &str,
        token: &str,
        value: &BlogData,
    ) -> Result<(), RequestError> {
        Self::send_json(
            Method::POST,
            format!("api/v1/blog/{}/{}", id, lang).as_str(),
            Some(token),
            Some(&value),
        )
        .await
        .map(|_: String| ())
    }

    pub async fn delete_post(id: &str, lang: &str, token: &str) -> Result<(), RequestError> {
        Self::send_json(
            Method::DELETE,
            format!("api/v1/blog/{}/{}", id, lang).as_str(),
            Some(token),
            Option::<&String>::None,
        )
        .await
        .map(|_: String| ())
    }

    /// Ok((resources, posts))
    pub async fn get_res_ids(token: &str) -> Result<(Vec<ResId>, Vec<ResId>), RequestError> {
        Ok((
            ApiClient::get_resource_keys(token)
                .await?
                .into_iter()
                .map(ResId::ResKey)
                .collect::<Vec<_>>(),
            {
                let mut posts = ApiClient::get_posts_meta(None)
                    .await?
                    .into_iter()
                    .map(|r| ResId::Blob(r.id))
                    .collect::<Vec<_>>();
                posts.sort();
                posts.dedup();
                posts
            },
        ))
    }
}

pub struct LocalClient;

impl LocalClient {
    pub async fn get_locale(lang: &str) -> Result<HashMap<String, String>, RequestError> {
        let resp = Request::new(format!("/locales/{}.yml", lang).as_str())
            .method(Method::GET)
            .send()
            .await
            .map_err(|e| RequestError::Network(e.to_string()))?;
        let body = resp
            .binary()
            .await
            .map_err(|e| RequestError::Network(e.to_string()))?;

        serde_yaml::from_slice::<HashMap<String, String>>(&body)
            .map_err(|e| RequestError::Parse(e.to_string()))
    }
}

pub struct BlobClient;

impl BlobClient {
    pub fn get_url(filename: &str) -> String {
        format!("{}{}", *AZURE_STORAGE_URL, filename)
    }

    pub fn get_post_url(filename: &str) -> String {
        format!("{}blog/{}", *AZURE_STORAGE_URL, filename)
    }

    pub async fn get_post_content(filename: &str) -> Result<String, RequestError> {
        let response = &Request::new(Self::get_post_url(filename).as_str())
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
            .text()
            .await
            .map_err(|e| RequestError::Network(e.to_string()))
    }
}
