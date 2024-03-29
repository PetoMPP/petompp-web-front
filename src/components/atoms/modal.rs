use deref_derive::{Deref, DerefMut};
use std::{rc::Rc, time::Duration};
use wasm_bindgen::prelude::Closure;
use wasm_bindgen::JsCast;
use web_sys::{HtmlDialogElement, HtmlElement};
use yew::{platform::spawn_local, prelude::*};
use yew_router::prelude::*;
use yewdux::prelude::*;

use crate::{
    components::{
        atoms::text_input::{InputType, TextInput},
        organisms::blob_image_select::BlobImageSelect,
    },
    data::locales::{store::LocalesStore, tk::TK},
    router::route::Route,
    utils::ext::Mergable,
};

#[derive(PartialEq, Clone)]
pub struct ModalButton {
    pub text_key: TK,
    pub onclick: Option<Callback<MouseEvent>>,
}

impl ModalButton {
    pub fn new(text_key: TK, onclick: Option<Callback<MouseEvent>>) -> Self {
        Self { text_key, onclick }
    }
}

#[derive(PartialEq, Clone)]
pub enum Buttons {
    Confirm(ModalButton),
    ConfirmCancel(ModalButton, ModalButton),
    RiskyCancel(ModalButton, ModalButton),
}

impl Default for Buttons {
    fn default() -> Self {
        Self::Confirm(ModalButton::new(TK::Ok, None))
    }
}

#[derive(PartialEq, Clone, Store, Default, Deref, DerefMut)]
pub struct ModalStore(pub ModalData);

#[derive(PartialEq, Clone)]
pub enum ModalData {
    Dialog(DialogData),
    Form(FormData),
    Image(ImageData),
    ImageSelector(Buttons),
}

impl Default for ModalData {
    fn default() -> Self {
        Self::Dialog(DialogData::default())
    }
}

#[derive(PartialEq, Clone, Default)]
pub struct DialogData {
    pub title: TK,
    pub message: TK,
    pub buttons: Buttons,
}

#[derive(PartialEq, Clone, Default)]
pub struct FormData {
    pub title: TK,
    pub fields: Vec<FormField>,
    pub buttons: Buttons,
}

#[derive(PartialEq, Clone, Default)]
pub struct FormField {
    pub id: String,
    pub label: TK,
    pub required: bool,
}

#[derive(PartialEq, Clone, Default)]
pub struct ImageData {
    pub src: String,
    pub title: String,
}

const MODAL_ID: &str = "modal";

pub fn show_modal(data: ModalData, dispatch: Dispatch<ModalStore>) {
    dispatch.reduce(|_| ModalStore(data).into());
    spawn_local(async move {
        async_std::task::sleep(Duration::from_millis(50)).await;
        let modal: HtmlDialogElement = web_sys::window()
            .unwrap()
            .document()
            .unwrap()
            .get_element_by_id(MODAL_ID)
            .unwrap()
            .unchecked_into();
        modal.show_modal().unwrap();
    });
}

pub fn show_modal_callback<T>(data: ModalData, dispatch: Dispatch<ModalStore>) -> Callback<T> {
    Callback::from(move |_| show_modal(data.clone(), dispatch.clone()))
}

#[derive(PartialEq, Properties)]
pub struct ModalProps {
    pub id: String,
    pub title: String,
    pub message: String,
    pub buttons: Buttons,
}

#[function_component(Modal)]
pub fn modal() -> Html {
    let (store, _) = use_store::<ModalStore>();
    match store.0.clone() {
        ModalData::Dialog(data) => {
            html! {<DialogModal title={data.title} message={data.message} buttons={data.buttons}/>}
        }
        ModalData::Form(data) => {
            html! {<FormModal title={data.title} fields={data.fields} buttons={data.buttons}/>}
        }
        ModalData::Image(data) => {
            html! {<ImageModal src={data.src} title={data.title}/>}
        }
        ModalData::ImageSelector(buttons) => html! {<ImageSelectorModal {buttons}/>},
    }
}

#[derive(Clone, PartialEq, Properties)]
struct DialogModalProps {
    pub title: TK,
    pub message: TK,
    pub buttons: Buttons,
}

#[function_component(DialogModal)]
fn dialog_modal(props: &DialogModalProps) -> Html {
    let (locales_store, _) = use_store::<LocalesStore>();
    html! {
        <dialog id={MODAL_ID} class={"modal z-80"}>
        <form method={"dialog"} class={"modal-box"}>
            <h3 class={"font-bold text-lg"}>{locales_store.get(props.title.clone())}</h3>
            <p class={"py-4"}>{locales_store.get(props.message.clone())}</p>
            <div class={"flex flex-row-reverse justify-between"}>
                {get_buttons(&props.buttons, locales_store)}
            </div>
        </form>
        </dialog>
    }
}

pub const MODAL_FIELD_PREFIX: &str = "modal-form-field-";

#[derive(Clone, PartialEq, Properties)]
struct FormModalProps {
    pub title: TK,
    pub fields: Vec<FormField>,
    pub buttons: Buttons,
}

#[function_component(FormModal)]
fn form_modal(props: &FormModalProps) -> Html {
    let (locales_store, _) = use_store::<LocalesStore>();
    let fields = props.fields.clone().into_iter().map(|f| {
        html! {
            <TextInput itype={InputType::Text} enabled={true} id={Some(format!("{}{}", MODAL_FIELD_PREFIX, &f.id))} label={locales_store.get(f.label)} />
        }
    });
    html! {
        <dialog id={MODAL_ID} class={"modal z-80"}>
        <form method={"dialog"} class={"modal-box overflow-visible"}>
            <h3 class={"font-bold text-lg"}>{locales_store.get(props.title.clone())}</h3>
            <div class={"py-4"}>
                {for fields}
            </div>
            <div class={"flex flex-row-reverse justify-between"}>
                {get_buttons(&props.buttons, locales_store)}
            </div>
        </form>
        </dialog>
    }
}

#[derive(Clone, PartialEq, Properties)]
struct ImageSelectorModalProps {
    pub buttons: Buttons,
}

#[function_component(ImageSelectorModal)]
fn image_selector_modal(props: &ImageSelectorModalProps) -> Html {
    let (locales_store, _) = use_store::<LocalesStore>();
    let data = use_state(|| None);
    let ondatachanged = {
        let data = data.clone();
        Callback::from(move |src: Option<String>| {
            data.set(src);
            web_sys::window()
                .unwrap()
                .document()
                .unwrap()
                .get_element_by_id(MODAL_ID)
                .unwrap()
                .unchecked_into::<HtmlElement>()
                .focus()
                .unwrap();
        })
    };
    html! {
        <dialog id={MODAL_ID} class={"modal z-80 items-end pb-6 lg:items-center"}>
            <div class={"modal-box overflow-visible flex flex-col gap-2"}>
                <BlobImageSelect id={format!("{}src", MODAL_FIELD_PREFIX)} container={"image-upload".to_string()} {ondatachanged} data={(*data).clone()}/>
                <div class={"flex flex-row-reverse justify-between"}>
                    {get_buttons(&props.buttons, locales_store)}
                </div>
            </div>
        </dialog>
    }
}

#[derive(Clone, PartialEq, Properties)]
struct ImageModalProps {
    pub src: String,
    pub title: String,
}

#[function_component(ImageModal)]
fn image_modal(props: &ImageModalProps) -> Html {
    let onclick = {
        let src = props.src.clone();
        Callback::from(move |_| {
            web_sys::window()
                .unwrap()
                .open_with_url(&src)
                .unwrap()
                .unwrap();
        })
    };
    html! {
        <>
        <dialog id={MODAL_ID} class={"modal z-80 outline-none"}>
            <div class={"p-4 bg-base-100 rounded-xl relative"}>
                <img {onclick} class={"w-full cursor-pointer max-w-[90vw] max-h-[90vh]"} src={props.src.clone()}/>
                    <div class={"font-bold bg-opacity-50 p-2 bg-base-100 rounded-lg text-lg absolute w-[fit-content] mx-auto bottom-6 left-0 right-0"}>
                    <h3>{&props.title}</h3>
                </div>
            </div>
            <form method={"dialog"} class={"absolute modal-backdrop h-[100lvh] w-[100vw] outline-none"}>
                <button class={"outline-none"}>{"close"}</button>
            </form>
        </dialog>
        </>
    }
}

#[derive(PartialEq, Properties)]
pub struct ErrorModalProps {
    pub id: String,
    pub title: String,
    pub message: String,
    pub children: Children,
}

pub const ERROR_MODAL_ID: &str = "error_modal";
pub const ERROR_MODAL_MSG_ID: &str = "error_modal_msg";
pub const ERROR_MODAL_BTN_ID: &str = "error_modal_btn";

#[function_component(ErrorModal)]
pub fn error_modal() -> Html {
    let (locales_store, _) = use_store::<LocalesStore>();
    html! {
        <>
        <dialog id={ERROR_MODAL_ID} class={"modal z-100"}>
        <form method={"dialog"} class={"modal-box bg-warning text-warning-content"}>
            <h3 class={"font-bold text-lg"}>{locales_store.get(TK::ErrorOccured)}</h3>
            <p id={ERROR_MODAL_MSG_ID} class={"py-4"}></p>
            <div class={"flex flex-row-reverse justify-between"}>
                <button id={ERROR_MODAL_BTN_ID} class="btn btn-error">{locales_store.get(TK::Ok)}</button>
            </div>
        </form>
        </dialog>
        </>
    }
}

pub fn show_error(msg: impl Into<String>, redirect: Option<(&Route, &Navigator)>) {
    let msg: String = msg.into();
    let modal: HtmlDialogElement = web_sys::window()
        .unwrap()
        .document()
        .unwrap()
        .get_element_by_id(ERROR_MODAL_ID)
        .unwrap()
        .unchecked_into();
    let msg_el: web_sys::HtmlElement = web_sys::window()
        .unwrap()
        .document()
        .unwrap()
        .get_element_by_id(ERROR_MODAL_MSG_ID)
        .unwrap()
        .unchecked_into();
    msg_el.set_inner_text(msg.as_str());
    let btn = web_sys::window()
        .unwrap()
        .document()
        .unwrap()
        .get_element_by_id(ERROR_MODAL_BTN_ID)
        .unwrap();
    let redirect_cb = redirect.map(|(route, navigator)| {
        let navigator = navigator.clone();
        let route = route.clone();
        Box::new(move || navigator.push(&route))
    });
    let cb = {
        let modal = modal.clone();
        Closure::wrap(Box::new(move || {
            let redirect_cb = redirect_cb.clone();
            if let Some(cb) = redirect_cb {
                cb();
            }
            modal.close();
        }) as Box<dyn FnMut()>)
    };
    btn.add_event_listener_with_callback("click", cb.as_ref().unchecked_ref())
        .unwrap();
    cb.forget();
    modal.show_modal().unwrap();
}

pub fn get_modal_close_callback() -> Callback<MouseEvent> {
    get_close_callback(MODAL_ID)
}

fn get_close_callback(id: &str) -> Callback<MouseEvent> {
    let id = id.to_string();
    Callback::from(move |_: MouseEvent| {
        let modal: HtmlDialogElement = web_sys::window()
            .unwrap()
            .document()
            .unwrap()
            .get_element_by_id(id.as_str())
            .unwrap()
            .unchecked_into();
        modal.close();
    })
}

fn get_buttons(buttons: &Buttons, locales_store: Rc<LocalesStore>) -> Html {
    match buttons.clone() {
        Buttons::Confirm(button) => {
            let onclick = into_modal_onclick(button.onclick);
            html! {
                <button class="btn btn-primary" {onclick}>{locales_store.get(button.text_key)}</button>
            }
        }
        Buttons::ConfirmCancel(confirm_button, cancel_button) => {
            let confirm_onclick = into_modal_onclick(confirm_button.onclick);
            let cancel_onclick = into_modal_onclick(cancel_button.onclick);
            html! {
                <>
                <button class="btn btn-neutral" onclick={cancel_onclick}>{locales_store.get(cancel_button.text_key)}</button>
                <button class="btn btn-primary" onclick={confirm_onclick}>{locales_store.get(confirm_button.text_key)}</button>
                </>
            }
        }
        Buttons::RiskyCancel(risky_button, cancel_button) => {
            let risky_onclick = into_modal_onclick(risky_button.onclick);
            let cancel_onclick = into_modal_onclick(cancel_button.onclick);
            html! {
                <>
                <button class="btn btn-neutral" onclick={cancel_onclick}>{locales_store.get(cancel_button.text_key)}</button>
                <button class="btn btn-error" onclick={risky_onclick}>{locales_store.get(risky_button.text_key)}</button>
                </>
            }
        }
    }
}

fn into_modal_onclick(onclick: Option<Callback<MouseEvent>>) -> Option<Callback<MouseEvent>> {
    match onclick {
        Some(onclick) => Some(onclick.merge(get_modal_close_callback())),
        None => Some(get_modal_close_callback()),
    }
}
