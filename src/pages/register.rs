use crate::api::client::ApiClient;
use crate::components::atoms::text_input::{InputType, TextInput};
use crate::data::locales::localizable::Localizable;
use crate::data::locales::store::LocalesStore;
use crate::data::locales::tk::TK;
use crate::router::route::Route;
use crate::{
    api::client::RequestError, async_event, components::atoms::modal::show_error,
    pages::page_base::PageBase,
};
use petompp_web_models::error::{Error, UsernameValidationError, ValidationError};
use petompp_web_models::models::credentials::Credentials;
use std::fmt::Display;
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew_router::prelude::*;
use yewdux::prelude::*;

#[derive(Clone, Debug, PartialEq)]
enum RegisterError {
    Username(String),
    Password(String),
}

impl Display for RegisterError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RegisterError::Username(e) => write!(f, "{}", e),
            RegisterError::Password(e) => write!(f, "{}", e),
        }
    }
}

#[function_component(Register)]
pub fn register() -> Html {
    let form_data = use_mut_ref(Credentials::default);
    let error_state = use_state_eq(|| Option::None);
    let (locales_store, _) = use_store::<LocalesStore>();
    let history = use_navigator().unwrap();

    let onchange_username = {
        let error_state = error_state.clone();
        let form_data = form_data.clone();
        Callback::from(move |e: InputEvent| {
            let target_element = e.target_unchecked_into::<HtmlInputElement>();
            form_data.borrow_mut().name = target_element.value();
            error_state.set(Option::None);
        })
    };
    let onchange_password = {
        let error_state = error_state.clone();
        let form_data = form_data.clone();
        Callback::from(move |e: InputEvent| {
            let target_element = e.target_unchecked_into::<HtmlInputElement>();
            form_data.borrow_mut().password = target_element.value();
            error_state.set(Option::None);
        })
    };
    let onsubmit = async_event!(
    [prevent SubmitEvent] |form_data, history, error_state, locales_store| {
            let creds = form_data.borrow().clone();
            match ApiClient::register(creds).await {
            Ok(()) => {
                error_state.set(Option::None);
                history.push(&Route::Login);
            },
            Err(error) => {
                match error {
                    RequestError::Endpoint(_, error) => {
                        match &error {
                            Error::UserNameTaken(_) => error_state.set(Some(RegisterError::Username(error.localize(&*locales_store)))),
                            Error::ValidationError(ve) => match ve {
                                ValidationError::Username(ue) => match ue {
                                    UsernameValidationError::InvalidLength(_, _) |
                                    UsernameValidationError::InvalidCharacters(_) => error_state.set(Some(RegisterError::Username(error.localize(&*locales_store)))),
                                },
                                ValidationError::Password(_) => error_state.set(Some(RegisterError::Password(error.localize(&*locales_store)))),
                                _ => show_error(error.localize(&*locales_store), Some((&Route::Home, &history))),
                            },
                            _ => show_error(error.localize(&*locales_store), Some((&Route::Home, &history))),
                        }
                    }
                    RequestError::Parse(error) | RequestError::Network(error) => {
                        show_error(error, Some((&Route::Home, &history)))
                    }
                }
            }
        }
    });
    let username_error = match &*error_state {
        Some(RegisterError::Username(error)) => Some(error.clone()),
        _ => None,
    };
    let password_error = match &*error_state {
        Some(RegisterError::Password(error)) => Some(error.clone()),
        _ => None,
    };
    html! {
        <PageBase>
        <form class={"form-control mx-auto mt-8 lg:mt-16 w-5/6 lg:w-3/4 xl:w-1/2"} {onsubmit}>
            <label class={"label"}>
                <span class={"label-text text-lg lg:text-2xl"}>{locales_store.get(TK::Register)}</span>
            </label>
            <TextInput
                label={locales_store.get(TK::Username)} itype={InputType::Text} enabled={true}
                placeholder={locales_store.get(TK::TypeUsername)} autocomplete={"username"}
                onchange={onchange_username} error={username_error}/>
            <TextInput
                label={locales_store.get(TK::Password)} itype={InputType::Password} enabled={true}
                placeholder={locales_store.get(TK::TypePassword)} autocomplete={"new-password"}
                onchange={onchange_password} error={password_error}/>
            <button class={"btn btn-primary shadow-md lg:text-xl mt-4"}>{locales_store.get(TK::Register)}</button>
        </form>
        </PageBase>
    }
}
