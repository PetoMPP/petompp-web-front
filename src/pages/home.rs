use crate::{data::resources::id::ResId, pages::page_base::EditablePage};
use yew::prelude::*;

#[function_component(Home)]
pub fn home() -> Html {
    const RES_KEY: &str = "home-content";
    html! {
        <EditablePage resid={ResId::ResKey(RES_KEY.to_string())}/ >
    }
}
