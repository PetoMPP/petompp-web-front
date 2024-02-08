use crate::api::client::ApiClient;
use crate::api::settings::SettingsClient;
use crate::api::user::UserClient;
use crate::components::atoms::text_input::{InputType, TextInput};
use crate::components::state::State;
use crate::data::locales::localizable::Localizable;
use crate::data::locales::store::LocalesStore;
use crate::data::locales::tk::TK;
use crate::router::route::Route;
use crate::{
    api::client::RequestError, async_event, components::atoms::modal::show_error,
    pages::page_base::PageBase,
};
use petompp_web_models::error::Error;
use petompp_web_models::models::credentials::Credentials;
use yew::platform::spawn_local;
use yew::prelude::*;
use yew_router::prelude::*;
use yewdux::prelude::*;

#[function_component(Register)]
pub fn register() -> Html {
    let form_data = use_state(Credentials::default);
    let requirements = use_state(|| State::Ok(None));
    let (locales_store, _) = use_store::<LocalesStore>();
    let history = use_navigator().unwrap();
    use_effect_with_deps(
        move |requirements| {
            let requirements = requirements.clone();
            match &*requirements {
                State::Ok(Some(_)) | State::Loading | State::Err(_) => return,
                _ => {}
            }
            requirements.set(State::Loading);
            spawn_local(async move {
                match ApiClient::get_user_settings().await {
                    Ok(req) => requirements.set(State::Ok(Some(req))),
                    Err(error) => requirements.set(State::Err(error)),
                }
            });
        },
        requirements.clone(),
    );

    let onchange_username = {
        let form_data = form_data.clone();
        Callback::from(move |e| {
            form_data.set(Credentials {
                name: e,
                ..(*form_data).clone()
            });
        })
    };
    let onchange_password = {
        let form_data = form_data.clone();
        Callback::from(move |e| {
            form_data.set(Credentials {
                password: e,
                ..(*form_data).clone()
            });
        })
    };
    let onsubmit = async_event!(
    [prevent SubmitEvent] |form_data, history, locales_store, requirements| {
            let creds = (*form_data).clone();
            match ApiClient::register(creds).await {
            Ok(()) => {
                history.push(&Route::Login);
            },
            Err(error) => {
                match error {
                    RequestError::Endpoint(_, error) => {
                        match &error {
                            Error::Register(reg_err) => {
                                let message = match &*requirements {
                                    State::Ok(Some((ur, pr))) => {
                                        let name_error = (ur, &reg_err.username_errors.iter().map(|s|s.as_str()).collect::<Vec<&str>>()).localize(&locales_store);
                                        let pass_error = (pr, &reg_err.password_errors.iter().map(|s|s.as_str()).collect::<Vec<&str>>()).localize(&locales_store);
                                        name_error + "\n" + pass_error.as_str()
                                    },
                                    _ => reg_err.username_errors.join("\n") + "\n" + reg_err.password_errors.join("\n").as_str(),
                                };
                                show_error(message, None)
                            },
                            _ => show_error(error.localize(&locales_store), None),
                        }
                    }
                    RequestError::Parse(error) | RequestError::Network(error) => {
                        show_error(error, Some((&Route::Home, &history)))
                    }
                }
            }
        }
    });
    let (name_err, pass_err) = match &*requirements {
        State::Ok(Some((ur, pr))) => {
            let name_error = match (ur, &form_data.name).localize(&locales_store) {
                s if s.is_empty() => None,
                s => Some(s),
            };
            let pass_error = match (pr, &form_data.password).localize(&locales_store) {
                s if s.is_empty() => None,
                s => Some(s),
            };
            (name_error, pass_error)
        }
        _ => (None, None),
    };
    html! {
        <PageBase title={locales_store.get(TK::Register)}>
        <form class={"form-control mx-auto mt-8 lg:mt-16 w-5/6 lg:w-3/4 xl:w-1/2"} {onsubmit}>
            <label class={"label"}>
                <span class={"label-text text-lg lg:text-2xl"}>{locales_store.get(TK::Register)}</span>
            </label>
            <TextInput
                label={locales_store.get(TK::Username)} itype={InputType::Text} enabled={true}
                placeholder={locales_store.get(TK::TypeUsername)} autocomplete={"username"}
                onchange={onchange_username} error={name_err}/>
            <TextInput
                label={locales_store.get(TK::Password)} itype={InputType::Password} enabled={true}
                placeholder={locales_store.get(TK::TypePassword)} autocomplete={"new-password"}
                onchange={onchange_password} error={pass_err}/>
            <button class={"btn btn-primary shadow-md lg:text-xl mt-4"}>{locales_store.get(TK::Register)}</button>
        </form>
        </PageBase>
    }
}
