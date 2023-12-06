use std::collections::BTreeMap;

use crate::{
    api::{self, client::ApiClient},
    async_event,
    components::atoms::{
        modal::show_error,
        text_input::{InputType, TextInput},
    },
    data::{
        locales::{localizable::Localizable, store::LocalesStore, tk::TK},
        session::SessionStore,
    },
    pages::page_base::PageBase,
    router::route::Route,
};
use petompp_web_models::models::credentials::Credentials;
use yew::prelude::*;
use yew_router::prelude::*;
use yewdux::prelude::use_store;

const PATH_QUERY_NAME: &str = "the-way-back-to-where-you-came-from";

#[function_component(LoginRedirect)]
pub fn login_redirect() -> Html {
    let navigator = use_navigator().unwrap();
    let location = use_location().unwrap();
    let mut pairs = BTreeMap::from_iter(vec![(
        PATH_QUERY_NAME.to_string(),
        location.path().to_string(),
    )]);
    if let Ok(query) = serde_urlencoded::from_str::<BTreeMap<String, String>>(
        location.query_str()[1..].to_string().as_str(),
    ) {
        pairs.extend(query);
    };
    use_effect(move || navigator.push_with_query(&Route::Login, &pairs).unwrap());

    html! {}
}

#[function_component(Login)]
pub fn login() -> Html {
    let form_data = use_mut_ref(Credentials::default);
    let error_state = use_state_eq(|| Option::None);
    let history = use_navigator().unwrap();
    let (locales_store, _) = use_store::<LocalesStore>();
    let (_, session_dispatch) = use_store::<SessionStore>();
    let location = use_location().unwrap();
    let returnto = location
        .query::<BTreeMap<String, String>>()
        .unwrap_or_default();

    let onchange_username = {
        let error_state = error_state.clone();
        let form_data = form_data.clone();
        Callback::from(move |e| {
            form_data.borrow_mut().name = e;
            error_state.set(Option::None);
        })
    };
    let onchange_password = {
        let error_state = error_state.clone();
        let form_data = form_data.clone();
        Callback::from(move |e| {
            form_data.borrow_mut().password = e;
            error_state.set(Option::None);
        })
    };
    let onsubmit = {
        async_event!(
            [prevent SubmitEvent] |form_data, error_state, history, session_dispatch, locales_store, returnto| {
                let creds = form_data.borrow().clone();
                match ApiClient::login(creds).await {
                    Ok(response) => {
                        session_dispatch.reduce(|_| {
                            SessionStore {
                                token: Some(response.token),
                                user: Some(response.user),
                            }
                            .into()
                        });
                        error_state.set(Option::None);
                        let mut returnto = returnto.clone();
                        let Some(path) = returnto.remove(PATH_QUERY_NAME) else {
                            history.push(&Route::Home);
                            return;
                        };
                        Route::navigate_from_str(&path, Some(&returnto), history.clone()).unwrap_or_else(|| history.push(&Route::Home));
                    }
                    Err(error) => match error {
                        api::client::RequestError::Endpoint(_, message) => error_state.set(Some(message.localize(&locales_store))),
                        api::client::RequestError::Parse(message) | api::client::RequestError::Network(message) => {
                            show_error(message, Some((&Route::Home, &history)))
                        }
                    },
                }
            }
        )
    };
    html! {
        <PageBase>
        <form class={"form-control mx-auto mt-8 lg:mt-16 w-5/6 lg:w-3/4 xl:w-1/2"} {onsubmit}>
            <label class={"label"}>
                <span class={"label-text text-lg lg:text-2xl"}>{locales_store.get(TK::Login)}</span>
            </label>
            <TextInput
                label={locales_store.get(TK::Username)} itype={InputType::Text} enabled={true}
                placeholder={locales_store.get(TK::TypeUsername)} autocomplete={"username"}
                onchange={onchange_username} error={(*error_state).clone().map(|_| String::new())}/>
            <TextInput
                label={locales_store.get(TK::Password)} itype={InputType::Password} enabled={true}
                placeholder={locales_store.get(TK::TypePassword)} autocomplete={"current-password"}
                onchange={onchange_password} error={(*error_state).clone()}/>
            <button class={"btn btn-primary shadow-md lg:text-xl mt-4"}>{locales_store.get(TK::Login)}</button>
        </form>
        </PageBase>
    }
}
