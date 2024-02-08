use crate::{
    api::{blob::BlobClient, client::ApiClient, resource::ResourceClient},
    async_event,
    components::atoms::modal::{
        show_modal_callback, Buttons, DialogData, ModalButton, ModalData, ModalStore,
    },
    data::{
        locales::{store::LocalesStore, tk::TK},
        resources::store::LocalStore,
        session::SessionStore,
    },
    pages::editor::{EditorData, EditorProps, EditorState},
};
use petompp_web_models::models::blob::blob_meta::{BlobMetaDto, BlobUpload};
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
                let upload = BlobUpload {
                    meta: BlobMetaDto::from((**meta).clone()),
                    content: value.into_bytes(),
                };
                gloo::console::log!(format!("{:?}", &upload.content));
                ApiClient::create_or_update(&token, "blog", &upload)
                    .await
                    .map(|_| ())
            }
            EditorData::Project((value, meta)) => {
                let upload = BlobUpload {
                    meta: BlobMetaDto::from((**meta).clone()),
                    content: value.into_bytes(),
                };
                ApiClient::create_or_update(&token, "project", &upload)
                    .await
                    .map(|_| ())
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
    });
    let (text, title, message) = match is_new {
        true => (TK::Create, TK::CreateResource, TK::CreateResourceQuestion),
        false => (TK::Save, TK::SaveChanges, TK::SaveChangesQuestion),
    };
    let onclick = show_modal_callback(
        ModalData::Dialog(DialogData {
            title,
            message,
            buttons: Buttons::ConfirmCancel(
                ModalButton::new(TK::Save, Some(onclick)),
                ModalButton::new(TK::Cancel, None),
            ),
        }),
        modal_dispatch.clone(),
    );

    html! {
        <button class={"btn btn-success grow"} {onclick}>
            {locales_store.get(text)}
        </button>
    }
}
