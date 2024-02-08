use super::client::{ApiClient, RequestError};
use petompp_web_models::models::{credentials::Credentials, user::UserData};
use reqwasm::http::Method;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct LoginResponse {
    pub token: String,
    pub user: UserData,
}

#[yewdux::async_trait(?Send)]
pub trait UserClient {
    async fn login(credentials: Credentials) -> Result<LoginResponse, RequestError>;
    async fn register(credentials: Credentials) -> Result<(), RequestError>;
    async fn get_users(token: &str) -> Result<Vec<UserData>, RequestError>;
    async fn activate_user(token: &str, id: i32) -> Result<(), RequestError>;
    async fn delete_user(token: &str, id: i32) -> Result<(), RequestError>;
}

#[yewdux::async_trait(?Send)]
impl UserClient for ApiClient {
    async fn login(credentials: Credentials) -> Result<LoginResponse, RequestError> {
        Self::send_json(Method::POST, "api/v1/users/login", None, Some(&credentials)).await
    }

    async fn register(credentials: Credentials) -> Result<(), RequestError> {
        Self::send_json::<UserData>(Method::POST, "api/v1/users", None, Some(&credentials))
            .await
            .map(|_| ())
    }

    async fn get_users(token: &str) -> Result<Vec<UserData>, RequestError> {
        Self::send_json(
            Method::GET,
            "api/v1/users/all?range=all",
            Some(token),
            Option::<&String>::None,
        )
        .await
        .map(|u: Vec<Vec<UserData>>| u[0].clone())
    }

    async fn activate_user(token: &str, id: i32) -> Result<(), RequestError> {
        Self::send_json::<UserData>(
            Method::POST,
            format!("api/v1/users/{}/activate", id).as_str(),
            Some(token),
            Option::<&String>::None,
        )
        .await
        .map(|_| ())
    }

    async fn delete_user(token: &str, id: i32) -> Result<(), RequestError> {
        Self::send_json::<UserData>(
            Method::DELETE,
            format!("api/v1/users/{}", id).as_str(),
            Some(token),
            Option::<&String>::None,
        )
        .await
        .map(|_| ())
    }
}
