use crate::pages::{
    admin::{admin_panel::AdminPanel, user_management::UserManagement},
    not_found::NotFound,
};
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Clone, Routable, PartialEq, Debug)]
pub enum AdminRoute {
    #[at("/admin")]
    AdminPanel,
    #[at("/admin/user_management")]
    UserManagement,
    #[not_found]
    #[at("/admin/404")]
    NotFound,
}

impl AdminRoute {
    pub fn switch(route: AdminRoute) -> Html {
        match route {
            AdminRoute::AdminPanel => html! { <AdminPanel />},
            AdminRoute::UserManagement => html! { <UserManagement />},
            AdminRoute::NotFound => html! {<NotFound />},
        }
    }
}
