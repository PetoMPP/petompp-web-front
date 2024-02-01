use crate::api::blob::BlobClient;
use crate::api::client::ApiClient;
use crate::components::atoms::modal::show_error;
use crate::pages::editor::EditorData;
use crate::utils::js::{set_textarea_height, set_textarea_text};
use crate::{api::client::RequestError, data::session::SessionStore};
use petompp_web_models::models::blob::blob_meta::{BlobMetaDto, BlobUpload};
use std::rc::Rc;
use wasm_bindgen::JsCast;
use web_sys::{ClipboardEvent, HtmlInputElement};
use yew::{platform::spawn_local, prelude::*};
use yewdux::prelude::*;

const TEXTAREA_ID: &str = "editor-textarea";
const UPLOAD_FOLDER: &str = "editor";

#[derive(Clone, PartialEq, Properties)]
pub struct MarkdownEditorProps {
    pub state: EditorData,
    pub onchanged: Callback<EditorData>,
}

#[function_component(MarkdownEditor)]
pub fn markdown_editor(props: &MarkdownEditorProps) -> Html {
    let error_state = use_state_eq(|| None);
    let (session_store, session_dispatch) = use_store::<SessionStore>();
    use_effect_with_deps(
        move |initial_state| {
            set_textarea_text(initial_state.to_string().as_str(), TEXTAREA_ID);
        },
        props.state.clone(),
    );

    let onpaste = {
        let props = props.clone();
        let session_store = session_store.clone();
        let error_state = error_state.clone();
        let onchanged = Callback::from(move |new_value: String| {
            props
                .onchanged
                .emit(props.state.clone().with_string(new_value))
        });
        Callback::from(move |e: Event| {
            let Ok(e) = e.dyn_into::<ClipboardEvent>() else {
                return;
            };
            let Some(data) = e.clipboard_data() else {
                return;
            };
            let Some(files) = data.files() else {
                return;
            };
            let Some(file) = files.get(0) else {
                return;
            };
            let onchanged = onchanged.clone();
            send_file(
                session_store.clone(),
                file.clone(),
                error_state.clone(),
                onchanged,
            );
        })
    };

    let oninput = {
        let onchanged = props.onchanged.clone();
        let state = props.state.clone();
        Callback::from(move |e: InputEvent| {
            let element: HtmlInputElement = e.target_unchecked_into();
            let value = element.value();
            let state = state.clone();
            onchanged.emit(state.with_string(value));
            set_textarea_height(&element);
        })
    };
    let ondrop = {
        let props = props.clone();
        let session_store = session_store.clone();
        let error_state = error_state.clone();
        let onchanged = Callback::from(move |new_value: String| {
            props
                .onchanged
                .emit(props.state.clone().with_string(new_value))
        });
        Callback::from(move |e: DragEvent| {
            let onchanged = onchanged.clone();
            e.prevent_default();
            let Some(dt) = e.data_transfer() else {
                return;
            };
            let Some(files) = dt.files() else {
                return;
            };
            if let Some(file) = files.get(0) {
                send_file(session_store.clone(), file, error_state.clone(), onchanged);
            }
        })
    };
    if let Some(error) = &*error_state {
        if let Err(redirect) = error.handle_failed_auth(session_dispatch) {
            return redirect;
        }
    }

    html! {
        <textarea id={TEXTAREA_ID} {oninput} {onpaste} {ondrop} class={"w-full font-mono bg-base-100 outline-none p-2 rounded-lg overflow-hidden resize-none leading-normal"}></textarea>
    }
}

fn send_file(
    session_store: Rc<SessionStore>,
    file: web_sys::File,
    error_state: UseStateHandle<Option<RequestError>>,
    onchanged: Callback<String>,
) {
    spawn_local(async move {
        let upload = match BlobUpload::from_file(&file).await {
            Ok(upload) => BlobUpload {
                meta: BlobMetaDto {
                    filename: format!("{}/", UPLOAD_FOLDER),
                    ..upload.meta
                },
                content: upload.content,
            },
            Err(e) => {
                error_state.set(Some(RequestError::Network(e.to_string())));
                return;
            }
        };
        match ApiClient::create_or_update(
            session_store.token.as_deref().unwrap_or_default(),
            "image-upload",
            &upload,
        )
        .await
        {
            Ok(filename) => {
                let Some(new_value) = insert_img_into_textarea(
                    &<ApiClient as BlobClient>::get_url("image-upload", &filename),
                ) else {
                    return;
                };
                onchanged.emit(new_value);
            }
            Err(e) => {
                gloo::console::error!(e.to_string());
                if let RequestError::Endpoint(413, e) = e {
                    show_error(e.to_string(), None);
                } else {
                    error_state.set(Some(e))
                }
            }
        }
    })
}

fn insert_img_into_textarea(img_url: &str) -> Option<String> {
    if let Some(element) = web_sys::window()
        .unwrap()
        .document()
        .unwrap()
        .get_element_by_id(TEXTAREA_ID)
    {
        let element: HtmlInputElement = element.unchecked_into();
        let value = element.value();
        let sel_start = element
            .selection_start()
            .unwrap_or_default()
            .unwrap_or_default() as usize;
        let sel_end = element
            .selection_end()
            .unwrap_or_default()
            .unwrap_or_default() as usize;
        let new_value = format!(
            "{}![{}]({}){}",
            &value.chars().take(sel_start).collect::<String>(),
            &value
                .chars()
                .skip(sel_start)
                .take(sel_end - sel_start)
                .collect::<String>(),
            img_url,
            &value.chars().skip(sel_end).collect::<String>()
        );
        set_textarea_text(new_value.as_str(), TEXTAREA_ID);
        return Some(new_value);
    }
    None
}
