use crate::{
    components::admin::user_manager::UserManager,
    data::{
        locales::{LocalesStore, TK},
        session::SessionStore,
    },
    models::user::Role,
    pages::{not_found::NotFound, page_base::PageBase},
};
use yew::prelude::*;
use yewdux::prelude::*;

#[function_component(UserManagement)]
pub fn user_management() -> Html {
    let (session_store, _) = use_store::<SessionStore>();
    let Some(Role::Admin) = session_store.as_ref().user.as_ref().map(|u| &u.role) else {
        return html!{<NotFound />};
    };
    let (locales_store, _) = use_store::<LocalesStore>();
    html! {
        <PageBase>
        <div class="flex flex-col lg:w-3/4 w-full m-auto">
            <p class={"text-2xl font-bold font-mono mb-2"}>{locales_store.get(TK::UserManagement)}</p>
            <UserManager />
        </div>
        </PageBase>
    }
}
