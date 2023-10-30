use crate::{data::resources::ResId, pages::page_base::EditablePage};
use yew::prelude::*;

#[function_component(Contact)]
pub fn contact() -> Html {
    const RES_KEY: &str = "contact-content";
    html! {
        <EditablePage resid={ResId::ResKey(RES_KEY.to_string())}/ >
    }
}
