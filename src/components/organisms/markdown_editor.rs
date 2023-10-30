use crate::api::client::ApiClient;
use crate::components::atoms::modal::show_error;
use crate::{api::client::RequestError, data::session::SessionStore};
use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::JsCast;
use web_sys::{ClipboardEvent, Element, HtmlInputElement};
use yew::{platform::spawn_local, prelude::*};
use yewdux::prelude::*;

const TEXTAREA_ID: &str = "editor-textarea";
const UPLOAD_FOLDER: &str = "editor";

#[derive(Clone, PartialEq, Properties)]
pub struct MarkdownEditorProps {
    pub state: String,
    pub onmodifiedchanged: Callback<bool>,
}

#[function_component(MarkdownEditor)]
pub fn markdown_editor(props: &MarkdownEditorProps) -> Html {
    let props = props.clone();
    let error_state = use_state_eq(|| None);
    let current_val = use_mut_ref(String::new);
    let last_state = use_mut_ref(|| props.state.clone());
    let last_mod_state = use_mut_ref(|| false);
    let markdown = use_state(String::new);
    let (session_store, session_dispatch) = use_store::<SessionStore>();
    use_effect_with_deps(
        move |(props, current_val, last_state, markdown, last_mod_state)| {
            let val = match !current_val.borrow().is_empty() && last_state.borrow().eq(&props.state)
            {
                true => current_val.borrow().to_string(),
                false => {
                    current_val.borrow_mut().clear();
                    let mut last_mod = last_mod_state.borrow_mut();
                    if *last_mod {
                        props.onmodifiedchanged.emit(false);
                        *last_mod = false;
                    }
                    props.state.clone()
                }
            };
            set_textarea_text(val.as_str());
            markdown.set(val);
            last_state.replace(props.state.clone());
            move || {}
        },
        (
            props.clone(),
            current_val.clone(),
            last_state.clone(),
            markdown.clone(),
            last_mod_state.clone(),
        ),
    );

    let onpaste = {
        let last_state = last_state.clone();
        let last_mod_state = last_mod_state.clone();
        let props = props.clone();
        let current_val = current_val.clone();
        let session_store = session_store.clone();
        let error_state = error_state.clone();
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
            send_file(
                session_store.clone(),
                file.clone(),
                error_state.clone(),
                props.clone(),
                current_val.clone(),
                last_state.clone(),
                last_mod_state.clone(),
            );
        })
    };

    let oninput = {
        let last_state = last_state.clone();
        let last_mod_state = last_mod_state.clone();
        let props = props.clone();
        let current_val = current_val.clone();
        Callback::from(move |e: InputEvent| {
            let element: HtmlInputElement = e.target_unchecked_into();
            let value = element.value();
            let mut last_mod = last_mod_state.borrow_mut();
            let changed = last_state.borrow().ne(&value);
            if changed != *last_mod {
                props.onmodifiedchanged.emit(changed);
                *last_mod = changed;
            }
            *current_val.borrow_mut() = value;
            set_textarea_height(&element);
        })
    };
    let ondrop = {
        let last_state = last_state.clone();
        let last_mod_state = last_mod_state.clone();
        let props = props.clone();
        let current_val = current_val.clone();
        let session_store = session_store.clone();
        let error_state = error_state.clone();
        Callback::from(move |e: DragEvent| {
            e.prevent_default();
            let Some(dt) = e.data_transfer() else {
                return;
            };
            let Some(files) = dt.files() else {
                return;
            };
            if let Some(file) = files.get(0) {
                send_file(
                    session_store.clone(),
                    file,
                    error_state.clone(),
                    props.clone(),
                    current_val.clone(),
                    last_state.clone(),
                    last_mod_state.clone(),
                );
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
    props: MarkdownEditorProps,
    current_val: Rc<RefCell<String>>,
    last_state: Rc<RefCell<String>>,
    last_mod_state: Rc<RefCell<bool>>,
) {
    spawn_local(async move {
        match ApiClient::upload_img(
            session_store.token.as_deref().unwrap_or_default(),
            file,
            UPLOAD_FOLDER,
        )
        .await
        {
            Ok(url) => {
                let Some(new_value) = insert_img_into_textarea(url.as_str()) else {
                    return;
                };
                let mut last_mod = last_mod_state.borrow_mut();
                let changed = last_state.borrow().ne(&new_value);
                if changed != *last_mod {
                    props.onmodifiedchanged.emit(changed);
                    *last_mod = changed;
                }
                *current_val.borrow_mut() = new_value;
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
        set_textarea_text(new_value.as_str());
        return Some(new_value);
    }
    None
}

fn set_textarea_text(value: &str) {
    let element: HtmlInputElement = web_sys::window()
        .unwrap()
        .document()
        .unwrap()
        .get_element_by_id(TEXTAREA_ID)
        .unwrap()
        .unchecked_into();
    element.set_value(value);
    set_textarea_height(&element);
}

fn set_textarea_height(element: &Element) {
    let body = web_sys::window()
        .unwrap()
        .document()
        .unwrap()
        .body()
        .unwrap();
    body.set_attribute(
        "style",
        format!("height: {}px;", body.client_height()).as_str(),
    )
    .unwrap();
    element.set_attribute("style", "height: auto;").unwrap();
    let scroll_height = element.scroll_height();
    if scroll_height > element.client_height() {
        element
            .set_attribute("style", format!("height: {}px;", scroll_height).as_str())
            .unwrap();
    }
    body.set_attribute("style", "height: auto;").unwrap();
}
