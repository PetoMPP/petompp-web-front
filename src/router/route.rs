use super::blog::BlogRoute;
use crate::{
    pages::{
        about::About, contact::Contact, editor::Editor, home::Home, login::Login,
        not_found::NotFound, projects::Projects, register::Register,
    },
    router::admin::AdminRoute,
};
use std::fmt::Display;
use wasm_bindgen::prelude::*;
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Clone, Routable, PartialEq, Debug)]
pub enum Route {
    #[at("/")]
    Root,
    #[at("/home")]
    Home,
    #[at("/projects")]
    Projects,
    #[at("/about")]
    About,
    #[at("/contact")]
    Contact,
    #[at("/login")]
    Login,
    #[at("/register")]
    Register,
    #[at("/editor")]
    Editor,
    // Admin routes
    #[at("/admin")]
    AdminRoot,
    #[at("/admin/*")]
    Admin,
    // Blog routes
    #[at("/blog/")]
    BlogRoot,
    #[at("/blog/*")]
    Blog,
    #[not_found]
    #[at("/404")]
    NotFound,
}

impl Display for Route {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{:?}", self))
    }
}

impl Route {
    pub fn switch(self) -> Html {
        match self {
            Route::Root | Route::Home => html! {<Home />},
            Route::Projects => html! {<Projects />},
            Route::Blog | Route::BlogRoot => {
                html! {<Switch<BlogRoute> render={BlogRoute::switch} />}
            }
            Route::About => html! {<About />},
            Route::Contact => html! {<Contact />},
            Route::Login => html! {<Login />},
            Route::Register => html! {<Register />},
            Route::Editor => html! { <Editor />},
            Route::AdminRoot | Route::Admin => {
                html! {<Switch<AdminRoute> render={AdminRoute::switch} />}
            }
            Route::NotFound => html! {  <NotFound />},
        }
    }

    fn get_onclick<T: Routable + std::fmt::Debug + 'static>(
        route: T,
        navigator: Navigator,
    ) -> Closure<dyn Fn(Event)> {
        let navigator = navigator.clone();
        Closure::new(Box::new(move |e: Event| {
            e.prevent_default();
            navigator.push(&route);
        }))
    }

    pub fn get_onclick_from_str(
        path: &str,
        navigator: Navigator,
    ) -> Option<Closure<dyn Fn(Event)>> {
        match Self::recognize(path) {
            Some(route) => match route {
                Route::Admin | Route::AdminRoot => {
                    let route = AdminRoute::recognize(path)?;
                    Some(Self::get_onclick(route, navigator))
                }
                Route::Blog | Route::BlogRoot => {
                    let route = BlogRoute::recognize(path)?;
                    Some(Self::get_onclick(route, navigator))
                }
                _ => Some(Self::get_onclick(route, navigator)),
            },
            None => None,
        }
    }
}
