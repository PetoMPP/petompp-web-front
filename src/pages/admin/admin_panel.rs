use crate::{
    data::session::SessionStore,
    pages::{not_found::NotFound, page_base::EditablePage},
};
use petompp_web_models::models::user::Role;
use yew::prelude::*;
use yewdux::prelude::*;

#[function_component(AdminPanel)]
pub fn admin_panel() -> Html {
    const RES_KEY: &str = "admin-panel-content";
    let (session_store, _) = use_store::<SessionStore>();
    let Some(Role::Admin) = session_store.as_ref().user.as_ref().map(|u| &u.role) else {
        return html! {<NotFound />};
    };
    html! {
        <EditablePage reskey={RES_KEY.to_string()}/ >
    }
}
