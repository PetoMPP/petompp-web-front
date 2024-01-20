use crate::{
    api::client::ApiClient,
    async_event,
    components::atoms::modal::{show_modal_callback, Buttons, ModalButton, ModalData, ModalStore},
    data::{
        locales::{store::LocalesStore, tk::TK},
        resources::{id::ResId, store::LocalStore},
        session::SessionStore,
    },
    pages::editor::{EditorData, EditorProps, EditorState},
};
use petompp_web_models::models::blog_data::BlogData;
use yew::prelude::*;
use yewdux::prelude::*;

#[function_component(SaveButton)]
pub fn save_button(props: &EditorProps) -> Html {
    let (session_store, _) = use_store::<SessionStore>();
    let (local_store, local_dispatch) = use_store::<LocalStore>();
    let (locales_store, _) = use_store::<LocalesStore>();
    let (_, modal_dispatch) = use_store::<ModalStore>();
    let state = match &props.state {
        EditorState::Ok(Some(state)) => state,
        _ => {
            return html! {};
        }
    };
    let (resid, lang) = &state.id;
    let Some(data) = local_store.get(resid, lang.key()) else {
        return html! {};
    };
    let onstatechange = props.onstatechanged.clone();
    let token = session_store.token.clone().unwrap_or_default();
    let is_new = state.is_new.unwrap_or_default();
    let onclick = async_event!(|onstatechange, resid, lang, local_dispatch, data, token| {
        onstatechange.emit(EditorState::Loading);
        match match data {
            EditorData::Resource(value) => match is_new {
                true => ApiClient::create_resource(&token, resid.id(), &lang, &value).await,
                false => ApiClient::update_resource(&token, resid.id(), &lang, &value).await,
            },
            EditorData::Blog((value, meta)) => {
                ApiClient::create_or_update_post(
                    resid.id(),
                    lang.key(),
                    &token,
                    &BlogData {
                        meta: meta.clone(),
                        content: value,
                    },
                )
                .await
            }
            EditorData::Project((value, meta)) => todo!(),
        } {
            Ok(_) => {
                local_dispatch.reduce_mut(|store| store.remove(&resid, lang.key()));
                onstatechange.emit(EditorState::Ok(None));
            }
            Err(e) => {
                onstatechange.emit(EditorState::Err(e));
            }
        }
    });
    let (text, title, message) = match is_new {
        true => (
            locales_store.get(TK::Create),
            locales_store.get(TK::CreateResource),
            locales_store.get(TK::CreateResourceQuestion),
        ),
        false => (
            locales_store.get(TK::Save),
            locales_store.get(TK::SaveChanges),
            locales_store.get(TK::SaveChangesQuestion),
        ),
    };
    let onclick = show_modal_callback(
        ModalData {
            title,
            message,
            buttons: Buttons::ConfirmCancel(
                ModalButton::new(locales_store.get(TK::Save), Some(onclick)),
                ModalButton::new(locales_store.get(TK::Cancel), None),
            ),
        },
        modal_dispatch.clone(),
    );

    html! {
        <button class={"btn btn-success grow"} {onclick}>
            {text}
        </button>
    }
}
