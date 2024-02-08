use super::client::{ApiClient, RequestError};
use petompp_web_models::models::{
    password_requirements::PasswordRequirements, user_settings_dto::UserSettingsDto,
    username_requirements::UsernameRequirements,
};
use reqwasm::http::Method;

#[yewdux::async_trait(?Send)]
pub trait SettingsClient {
    async fn get_user_settings(
    ) -> Result<(UsernameRequirements, PasswordRequirements), RequestError>;
}

#[yewdux::async_trait(?Send)]
impl SettingsClient for ApiClient {
    async fn get_user_settings(
    ) -> Result<(UsernameRequirements, PasswordRequirements), RequestError> {
        Self::send_json(
            Method::GET,
            "api/v1/settings/users",
            None,
            Option::<&String>::None,
        )
        .await
        .map(|dto: UserSettingsDto| {
            dto.try_into()
                .map_err(|_: ()| RequestError::Parse("Invalid data format".to_string()))
        })?
    }
}
