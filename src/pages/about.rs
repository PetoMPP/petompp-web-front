use crate::{data::resources::id::ResId, pages::page_base::EditablePage};
use yew::prelude::*;

#[function_component(About)]
pub fn about() -> Html {
    const RES_KEY: &str = "about-content";
    html! {
        <EditablePage resid={ResId::ResKey(RES_KEY.to_string())}/ >
    }
}
