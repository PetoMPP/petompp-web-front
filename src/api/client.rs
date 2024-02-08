use crate::{data::session::SessionStore, pages::login::LoginRedirect};
use petompp_web_models::error::Error;
use reqwasm::http::*;
use serde::{de::DeserializeOwned, Serialize};
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
}

trait Authorizable {
    fn authorize(self, token: &str) -> Self;
}

impl Authorizable for Request {
    fn authorize(self, token: &str) -> Self {
        self.header("Authorization", format!("Bearer {}", token).as_str())
    }
}

pub struct ApiClient;

impl ApiClient {
    fn get_url(path: &str) -> String {
        format!("{}{}", *API_URL, path)
    }

    pub async fn send_json<R: DeserializeOwned>(
        method: Method,
        path: &str,
        token: Option<&str>,
        body: Option<&impl Serialize>,
    ) -> Result<R, RequestError> {
        let mut request = Request::new(Self::get_url(path).as_str()).method(method);
        if let Some(token) = token {
            request = request.authorize(token);
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

    pub async fn send_multipart<R: DeserializeOwned>(
        method: Method,
        path: &str,
        token: Option<&str>,
        form: impl Into<FormData>,
    ) -> Result<R, RequestError> {
        let mut request = Request::new(Self::get_url(path).as_str()).method(method);
        if let Some(token) = token {
            request = request.authorize(token);
        }
        let form = form.into();
        let response = request
            .body(form)
            .send()
            .await
            .map_err(|e| RequestError::Network(e.to_string()))?;

        match Response::from_response(response).await? {
            Response::Success(data) => Ok(data),
            Response::Error(s, e) => Err(RequestError::Endpoint(s, e)),
        }
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
