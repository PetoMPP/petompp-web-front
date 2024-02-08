use crate::{
    components::{atoms::label::Label, organisms::blob_image_select::BlobImageSelect},
    data::locales::{store::LocalesStore, tk::TK},
    utils::ext::Mergable,
};
use wasm_bindgen::JsCast;
use web_sys::HtmlElement;
use yew::prelude::*;
use yewdux::prelude::*;

#[derive(Clone, Properties, PartialEq)]
pub struct ImageLinkEditorProps {
    pub container: String,
    pub id: Option<String>,
    pub folder: Option<String>,
    pub data: Option<String>,
    pub ondatachanged: Callback<String>,
}

#[function_component(ImageLinkInput)]
pub fn image_link_input(props: &ImageLinkEditorProps) -> Html {
    let (locales_store, _) = use_store::<LocalesStore>();
    let focus_out = Callback::from(|_| {
        let element = web_sys::window()
            .unwrap()
            .document()
            .unwrap()
            .get_element_by_id("modal")
            .unwrap()
            .unchecked_into::<HtmlElement>();
        element.focus().unwrap();
    });
    let ondatachanged = {
        let ondatachanged = props.ondatachanged.clone();
        Callback::from(move |d: Option<_>| d.map(|d| ondatachanged.emit(d)).unwrap_or_default())
            .merge(focus_out)
    };
    html! {
        <Label label={locales_store.get(TK::Image)} error={false}>
            <BlobImageSelect id={props.id.clone()} container={props.container.clone()} folder={props.folder.clone()} data={props.data.clone()} {ondatachanged} />
        </Label>
    }
}
