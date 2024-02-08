use super::command::{insert_after_selection, EditorCommand};
use crate::components::atoms::modal::{
    show_modal_callback, Buttons, FormData, FormField, ModalButton, ModalData, ModalStore,
    MODAL_FIELD_PREFIX,
};
use deref_derive::Deref;
use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yewdux::dispatch::Dispatch;

#[derive(Deref)]
pub struct LinkCommand(String);

impl EditorCommand for LinkCommand {
    fn create(target: &str) -> Self {
        Self(target.to_string())
    }

    fn img(&self) -> &str {
        "/img/ui/link.svg"
    }

    fn can_do(&self) -> bool {
        true
    }

    fn command(
        &self,
        cb: Callback<String>,
        modal_dispatch: Dispatch<ModalStore>,
    ) -> Callback<Event> {
        let id = (*self).clone();
        let onclick = Callback::from(move |_| {
            let document = web_sys::window().unwrap().document().unwrap();
            let url_input = document
                .get_element_by_id(format!("{}{}", MODAL_FIELD_PREFIX, "URL").as_str())
                .unwrap()
                .dyn_into::<HtmlInputElement>()
                .unwrap();
            let text_input = document
                .get_element_by_id(format!("{}{}", MODAL_FIELD_PREFIX, "Text").as_str())
                .unwrap()
                .dyn_into::<HtmlInputElement>()
                .unwrap();
            let url = url_input.value();
            let text = text_input.value();
            if url.is_empty() {
                return;
            }
            let encoded_url: String = web_sys::js_sys::encode_uri(&url).into();
            let link = if text.is_empty() {
                format!("[{}]({})", url, encoded_url)
            } else {
                format!("[{}]({})", text, encoded_url)
            };
            cb.emit(insert_after_selection(id.as_str(), &link));
        });
        let modal_data = ModalData::Form(FormData {
            title: "Insert Link".to_string(),
            fields: vec![
                FormField {
                    label: "URL".to_string(),
                    required: true,
                },
                FormField {
                    label: "Text".to_string(),
                    required: false,
                },
            ],
            buttons: Buttons::ConfirmCancel(
                ModalButton::new("insert".to_string(), Some(onclick)),
                ModalButton::new("cancel".to_string(), None),
            ),
        });
        show_modal_callback(modal_data, modal_dispatch)
    }
}
