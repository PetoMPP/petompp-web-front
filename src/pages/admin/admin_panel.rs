use crate::{
    data::{resources::id::ResId, session::SessionStore},
    pages::{not_found::NotFound, page_base::EditablePage},
};
use petompp_web_models::models::user::RoleData;
use yew::prelude::*;
use yewdux::prelude::*;

#[function_component(AdminPanel)]
pub fn admin_panel() -> Html {
    const RES_KEY: &str = "admin-panel-content";
    let (session_store, _) = use_store::<SessionStore>();
    let Some(RoleData::Admin) = session_store.as_ref().user.as_ref().map(|u| &u.role) else {
        return html! {<NotFound />};
    };
    html! {
        <EditablePage title={"Admin panel".to_string()} resid={ResId::ResKey(RES_KEY.to_string())}/ >
    }
}
