use crate::{
    data::{
        locales::{store::LocalesStore, tk::TK},
        resources::id::ResId,
    },
    pages::page_base::EditablePage,
};
use yew::prelude::*;
use yewdux::prelude::*;

#[function_component(Home)]
pub fn home() -> Html {
    const RES_KEY: &str = "home-content";
    let (locales_store, _) = use_store::<LocalesStore>();
    html! {
        <EditablePage title={locales_store.get(TK::Home)} resid={ResId::ResKey(RES_KEY.to_string())}/ >
    }
}
