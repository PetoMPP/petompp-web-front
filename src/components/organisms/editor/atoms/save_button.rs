use crate::{
    api::client::ApiClient,
    async_event,
    components::atoms::modal::{show_modal_callback, Buttons, ModalButton, ModalData, ModalStore},
    data::{
        locales::{store::LocalesStore, tk::TK},
        resources::{id::ResId, store::LocalStore},
        session::SessionStore,
    },
    pages::editor::{EditorProps, EditorState},
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
    let (Some(resid), Some(lang)) = (&props.resid, &props.lang) else {
        return html! {};
    };
    let Some((value, meta)) = local_store.get(resid, lang.key()) else {
        return html! {};
    };
    if let EditorState::Loading = &props.state {
        return html! {};
    }
    let onstatechange = &props.onstatechanged;
    let token = session_store.token.clone().unwrap_or_default();
    let isnew = matches!(&props.state, EditorState::Ok(Some((Some(true), _, _))));
    let onclick = async_event!(
        |onstatechange, local_dispatch, resid, lang, value, meta, token| {
            onstatechange.emit(EditorState::Loading);
            match match &resid {
                ResId::ResKey(key) => match isnew {
                    true => ApiClient::create_resource(&token, key, &lang, &value).await,
                    false => ApiClient::update_resource(&token, key, &lang, &value).await,
                },
                ResId::Blob(_) => {
                    ApiClient::create_or_update_post(
                        resid.id(),
                        lang.key(),
                        &token,
                        &BlogData {
                            meta: meta.clone().unwrap_or_default(),
                            content: value,
                        },
                    )
                    .await
                }
            } {
                Ok(_) => {
                    local_dispatch.reduce_mut(|store| store.remove(&resid, lang.key()));
                    onstatechange.emit(EditorState::Ok(None));
                }
                Err(e) => {
                    onstatechange.emit(EditorState::Err(e));
                }
            }
        }
    );
    let (text, title, message) = match isnew {
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
