use super::command::{insert_after_selection, EditorCommand};
use crate::{
    api::{blob::BlobClient, client::ApiClient},
    components::atoms::modal::{
        show_modal_callback, Buttons, ModalButton, ModalData, ModalStore, MODAL_FIELD_PREFIX,
    },
};
use deref_derive::Deref;
use wasm_bindgen::JsCast;
use web_sys::HtmlElement;
use yew::prelude::*;
use yewdux::prelude::*;

#[derive(Deref)]
pub struct ImageCommand(String);

impl EditorCommand for ImageCommand {
    fn create(target: &str) -> Self {
        Self(target.to_string())
    }

    fn img(&self) -> &str {
        "/img/ui/image.svg"
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
        let onclick = Callback::from(move |_: MouseEvent| {
            let document = web_sys::window().unwrap().document().unwrap();
            let input = document
                .get_element_by_id(&format!("{}src", MODAL_FIELD_PREFIX))
                .unwrap()
                .first_element_child()
                .unwrap()
                .unchecked_into::<HtmlElement>();
            let url = input.inner_text();
            if url.is_empty() {
                return;
            }
            let full_url: String = web_sys::js_sys::encode_uri(
                &<ApiClient as BlobClient>::get_url("image-upload", &url),
            )
            .into();
            let image = format!("![{}]({})", &url, &full_url);
            cb.emit(insert_after_selection(&id, &image));
        });
        let modal_data = ModalData::ImageSelector(Buttons::ConfirmCancel(
            ModalButton::new("insert", Some(onclick)),
            ModalButton::new("cancel", None),
        ));
        show_modal_callback(modal_data, modal_dispatch)
    }
}
