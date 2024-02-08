use crate::{
    pages::{
        about::About, blog::Blog, blog_post::BlogPost, contact::Contact, editor::Editor,
        home::Home, login::Login, not_found::NotFound, project::Project, projects::Projects,
        register::Register,
    },
    router::admin::AdminRoute,
};
use serde::Serialize;
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
    #[at("/blog")]
    Blog,
    #[at("/blog/:id")]
    BlogPost { id: String },
    #[at("/projects")]
    Projects,
    #[at("/project/:id")]
    Project { id: String },
    // Admin routes
    #[at("/admin")]
    AdminRoot,
    #[at("/admin/*")]
    Admin,
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
            Route::Blog => html! {<Blog />},
            Route::BlogPost { id } => html! {<BlogPost {id} />},
            Route::Projects => html! {<Projects />},
            Route::Project { id } => html! {<Project {id} />},
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
                _ => Some(Self::get_onclick(route, navigator)),
            },
            None => None,
        }
    }

    pub fn navigate_from_str(
        path: &str,
        query: Option<&impl Serialize>,
        navigator: Navigator,
    ) -> Option<()> {
        match Self::recognize(path) {
            Some(route) => match route {
                Route::Admin | Route::AdminRoot => {
                    let route = AdminRoute::recognize(path)?;
                    match query {
                        Some(query) => match navigator.push_with_query(&route, query) {
                            Ok(_) => Some(()),
                            Err(_) => None,
                        },
                        None => {
                            navigator.push(&route);
                            Some(())
                        }
                    }
                }
                _ => match query {
                    Some(query) => match navigator.push_with_query(&route, query) {
                        Ok(_) => Some(()),
                        Err(_) => None,
                    },
                    None => {
                        navigator.push(&route);
                        Some(())
                    }
                },
            },
            None => None,
        }
    }
}
