use crate::{
    api::{
        blob::BlobClient,
        client::{ApiClient, RequestError},
        resource::ResourceClient,
    },
    async_event,
    components::atoms::modal::{show_modal_callback, Buttons, ModalButton, ModalData, ModalStore},
    data::{
        locales::{store::LocalesStore, tk::TK},
        resources::id::{BlobType, ResId},
        session::SessionStore,
    },
    pages::editor::{EditorProps, EditorState},
    router::route::Route,
};
use petompp_web_models::models::country::Country;
use yew::prelude::*;
use yew_router::prelude::*;
use yewdux::prelude::*;

#[function_component(DeleteButton)]
pub fn delete_button(props: &EditorProps) -> Html {
    let (session_store, session_dispatch) = use_store::<SessionStore>();
    let (locales_store, _) = use_store::<LocalesStore>();
    let (_, modal_dispatch) = use_store::<ModalStore>();
    let navigator = use_navigator().unwrap();
    let err = use_state(|| Option::<RequestError>::None);
    if let Some(e) = &*err {
        if let Err(redirect) = e.handle_failed_auth(session_dispatch) {
            return redirect;
        }
        gloo::dialogs::alert(e.to_string().as_str());
    }
    let (Some(resid), Some(lang)) = (&props.resid, &props.lang) else {
        return html! {};
    };
    match &props.state {
        EditorState::Loading | EditorState::Ok(None) => return Html::default(),
        EditorState::Ok(Some(state)) if state.is_new.unwrap_or(true) => return Html::default(),
        _ => {}
    }
    let onstatechange = &props.onstatechanged;
    let token = session_store.token.clone().unwrap_or_default();
    let onclick = async_event!(|onstatechange, navigator, resid, lang, err, token| {
        onstatechange.emit(EditorState::Loading);
        match match resid {
            ResId::Blob(blob) => match blob {
                BlobType::Blog(id) => ApiClient::delete(&token, "blog", &id).await,
                BlobType::Project(id) => ApiClient::delete(&token, "project", &id).await,
            },
            ResId::ResKey(id) => match lang {
                Country::UnitedKingdom => ApiClient::delete_resource(&token, &id).await,
                _ => ApiClient::delete_resource_lang(&token, &id, &lang).await,
            },
        } {
            Ok(_) => navigator.push(&Route::Editor),
            Err(e) => err.set(Some(e)),
        }
    });
    let onclick = show_modal_callback(
        ModalData {
            title: locales_store.get(TK::DeleteResource),
            message: locales_store.get(TK::DeleteResourceQuestion),
            buttons: Buttons::RiskyCancel(
                ModalButton::new(locales_store.get(TK::Delete), Some(onclick)),
                ModalButton::new(locales_store.get(TK::Cancel), None),
            ),
        },
        modal_dispatch.clone(),
    );
    html! {
        <button {onclick} class={"flex btn btn-error"}>{locales_store.get(TK::Delete)}</button>
    }
}
