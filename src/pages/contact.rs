use crate::{
    data::{
        locales::{store::LocalesStore, tk::TK},
        resources::id::ResId,
    },
    pages::page_base::EditablePage,
};
use yew::prelude::*;
use yewdux::prelude::*;

#[function_component(Contact)]
pub fn contact() -> Html {
    const RES_KEY: &str = "contact-content";
    let (locales_store, _) = use_store::<LocalesStore>();
    html! {
        <EditablePage title={locales_store.get(TK::Contact)} resid={ResId::ResKey(RES_KEY.to_string())}/ >
    }
}
