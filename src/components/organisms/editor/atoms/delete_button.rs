use crate::{
    api::client::ApiClient,
    data::{
        locales::{store::LocalesStore, tk::TK},
        resources::id::ResId,
        session::SessionStore,
    },
    router::route::Route,
};
use petompp_web_models::models::country::Country;
use yew::{platform::spawn_local, prelude::*};
use yew_router::prelude::*;
use yewdux::prelude::*;

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct DeleteButtonProps {
    pub resid: ResId,
    pub lang: Country,
}

#[function_component(DeleteButton)]
pub fn delete_button(props: &DeleteButtonProps) -> Html {
    let (session_store, session_dispatch) = use_store::<SessionStore>();
    let (locales_store, _) = use_store::<LocalesStore>();
    let navigator = use_navigator().unwrap();
    let resid = props.resid.clone();
    let lang = props.lang;
    let err = use_state(|| None);
    let onclick = {
        let err = err.clone();
        Callback::from(move |_| {
            let navigator = navigator.clone();
            let resid = resid.clone();
            let lang = lang;
            let err = err.clone();
            let token = session_store.token.clone().unwrap_or_default();
            spawn_local(async move {
                match match resid {
                    ResId::Blob(id) => ApiClient::delete_post(&id, lang.key(), &token).await,
                    ResId::ResKey(id) => match lang {
                        Country::UnitedKingdom => ApiClient::delete_resource(&token, &id).await,
                        _ => ApiClient::delete_resource_lang(&token, &id, &lang).await,
                    },
                } {
                    Ok(_) => navigator.push(&Route::Editor),
                    Err(e) => err.set(Some(e)),
                }
            })
        })
    };
    if let Some(e) = &*err {
        if let Err(redirect) = e.handle_failed_auth(session_dispatch) {
            return redirect;
        }
        gloo::dialogs::alert(e.to_string().as_str());
    }
    html! {
        <button {onclick} class={"flex btn btn-error"}>{locales_store.get(TK::Delete)}</button>
    }
}
