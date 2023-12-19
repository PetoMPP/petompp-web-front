use crate::{
    data::{
        locales::{store::LocalesStore, tk::TK},
        resources::id::ResId,
    },
    pages::page_base::EditablePage,
};
use yew::prelude::*;
use yewdux::prelude::*;

#[function_component(About)]
pub fn about() -> Html {
    const RES_KEY: &str = "about-content";
    let (locales_store, _) = use_store::<LocalesStore>();
    html! {
        <EditablePage title={locales_store.get(TK::About)} resid={ResId::ResKey(RES_KEY.to_string())}/ >
    }
}
