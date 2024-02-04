use deref_derive::{Deref, DerefMut};
use std::time::Duration;
use wasm_bindgen::prelude::Closure;
use wasm_bindgen::JsCast;
use web_sys::HtmlDialogElement;
use yew::{platform::spawn_local, prelude::*};
use yew_router::prelude::*;
use yewdux::prelude::*;

use crate::{
    data::locales::{store::LocalesStore, tk::TK},
    router::route::Route,
    utils::ext::Mergable,
};

#[derive(PartialEq, Clone)]
pub struct ModalButton {
    pub text: String,
    pub onclick: Option<Callback<MouseEvent>>,
}

impl ModalButton {
    pub fn new(text: impl Into<String>, onclick: Option<Callback<MouseEvent>>) -> Self {
        Self {
            text: text.into(),
            onclick,
        }
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
        Self::Confirm(ModalButton::new("OK", None))
    }
}

#[derive(PartialEq, Clone, Store, Default, Deref, DerefMut)]
pub struct ModalStore(pub ModalData);

#[derive(PartialEq, Clone)]
pub enum ModalData {
    Dialog(DialogData),
    Image(ImageData),
}

impl Default for ModalData {
    fn default() -> Self {
        Self::Dialog(DialogData::default())
    }
}

#[derive(PartialEq, Clone, Default)]
pub struct DialogData {
    pub title: String,
    pub message: String,
    pub buttons: Buttons,
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
            html! {
                <DialogModal title={data.title} message={data.message} buttons={data.buttons} />
            }
        }
        ModalData::Image(data) => {
            html! {
                <ImageModal src={data.src} title={data.title} />
            }
        }
    }
}

#[derive(Clone, PartialEq, Properties)]
struct DialogModalProps {
    pub title: String,
    pub message: String,
    pub buttons: Buttons,
}

#[function_component(DialogModal)]
fn dialog_modal(props: &DialogModalProps) -> Html {
    html! {
        <>
        <dialog id={MODAL_ID} class={"modal z-80"}>
        <form method={"dialog"} class={"modal-box"}>
            <h3 class={"font-bold text-lg"}>{&props.title}</h3>
            <p class={"py-4"}>{&props.message}</p>
            <div class={"flex flex-row-reverse justify-between"}>
                {get_buttons(&props.buttons)}
            </div>
        </form>
        </dialog>
        </>
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

fn get_buttons(buttons: &Buttons) -> Html {
    match buttons.clone() {
        Buttons::Confirm(button) => {
            let onclick = into_modal_onclick(button.onclick);
            html! {
                <button class="btn btn-primary" {onclick}>{&button.text}</button>
            }
        }
        Buttons::ConfirmCancel(confirm_button, cancel_button) => {
            let confirm_onclick = into_modal_onclick(confirm_button.onclick);
            let cancel_onclick = into_modal_onclick(cancel_button.onclick);
            html! {
                <>
                <button class="btn btn-neutral" onclick={cancel_onclick}>{&cancel_button.text}</button>
                <button class="btn btn-primary" onclick={confirm_onclick}>{&confirm_button.text}</button>
                </>
            }
        }
        Buttons::RiskyCancel(risky_button, cancel_button) => {
            let risky_onclick = into_modal_onclick(risky_button.onclick);
            let cancel_onclick = into_modal_onclick(cancel_button.onclick);
            html! {
                <>
                <button class="btn btn-neutral" onclick={cancel_onclick}>{&cancel_button.text}</button>
                <button class="btn btn-error" onclick={risky_onclick}>{&risky_button.text}</button>
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
