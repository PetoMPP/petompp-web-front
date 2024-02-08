use crate::{
    api::client::LocalClient,
    components::{
        atoms::modal::{ErrorModal, Modal},
        organisms::header::Header,
    },
    data::locales::store::LocalesStore,
    router::route::Route,
};
use yew::{platform::spawn_local, prelude::*};
use yew_router::{BrowserRouter, Switch};
use yewdux::prelude::*;

mod api;
mod components;
mod data;
mod hooks;
mod pages;
mod router;
mod utils;

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct AppProps {
    pub children: Children,
    pub preview: bool,
}

#[function_component(AppBase)]
pub fn app_base(props: &AppProps) -> Html {
    let header = match props.preview {
        true => None,
        false => Some(html! { <Header /> }),
    };
    let mut class = classes!("flex", "flex-col", "bg-base-300");
    if !props.preview {
        class.push("min-h-screen");
    }
    html! {
        <body {class}>
            {header}
            <img src={"/img/coast.jpg"} class={"w-full w-max-full h-max-full absolute top-10 opacity-40 my-4 h-90"} />
            <div class={"m-auto w-11/12 lg:w-5/6 xl:w-2/3 flex flex-col grow items-center"}>
                {props.children.clone()}
            </div>
        </body>
    }
}

#[function_component(App)]
pub fn app() -> Html {
    let error_state = use_state(|| None);
    let (locale_store, locale_dispatch) = use_store::<LocalesStore>();
    if !locale_store.is_loaded(locale_store.curr) || error_state.is_some() {
        let locale_store = locale_store.clone();
        let locale_dispatch = locale_dispatch.clone();
        spawn_local(async move {
            match LocalClient::get_locale(locale_store.curr.key()).await {
                Ok(data) => {
                    locale_dispatch.reduce_mut(|l| l.load(locale_store.curr, data));
                }
                Err(e) => {
                    error_state.set(Some(e));
                }
            };
        })
    }
    _ = use_memo(
        |_| {
            LocalesStore::add_lang_change_event_listener(locale_dispatch);
        },
        (),
    );
    use_effect_with_deps(
        |curr| {
            web_sys::window()
                .unwrap()
                .document()
                .unwrap()
                .document_element()
                .unwrap()
                .set_attribute("lang", curr.key())
                .unwrap();
        },
        locale_store.curr,
    );

    html! {
        <BrowserRouter>
            <AppBase preview={false}>
                <Switch<Route> render={Route::switch}/>
            </AppBase>
            <Modal />
            <ErrorModal />
        </BrowserRouter>
    }
}
