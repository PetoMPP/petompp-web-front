use super::client::{ApiClient, RequestError};
use petompp_web_models::models::{country::Country, resource_data::ResourceData};
use reqwasm::http::Method;

#[yewdux::async_trait(?Send)]
pub trait ResourceClient {
    async fn create_resource(
        token: &str,
        key: &str,
        lang: &Country,
        value: &str,
    ) -> Result<(), RequestError>;
    async fn get_resource(key: &str, lang: &Country) -> Result<(Country, String), RequestError>;
    async fn get_resource_keys(token: &str) -> Result<Vec<String>, RequestError>;
    async fn update_resource(
        token: &str,
        key: &str,
        lang: &Country,
        value: &str,
    ) -> Result<(), RequestError>;
    async fn delete_resource(token: &str, key: &str) -> Result<(), RequestError>;
    async fn delete_resource_lang(
        token: &str,
        key: &str,
        lang: &Country,
    ) -> Result<(), RequestError>;
}

#[yewdux::async_trait(?Send)]
impl ResourceClient for ApiClient {
    async fn create_resource(
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

    async fn get_resource(key: &str, lang: &Country) -> Result<(Country, String), RequestError> {
        Self::send_json(
            Method::GET,
            format!("api/v1/res/{}?lang={}", key, lang.key()).as_str(),
            None,
            Option::<&String>::None,
        )
        .await
    }

    async fn get_resource_keys(token: &str) -> Result<Vec<String>, RequestError> {
        Self::send_json(
            Method::GET,
            "api/v1/res/keys",
            Some(token),
            Option::<&String>::None,
        )
        .await
    }

    async fn update_resource(
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

    async fn delete_resource(token: &str, key: &str) -> Result<(), RequestError> {
        Self::send_json(
            Method::DELETE,
            format!("api/v1/res/{}", key).as_str(),
            Some(token),
            Option::<&String>::None,
        )
        .await
        .map(|_: String| ())
    }

    async fn delete_resource_lang(
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
}
