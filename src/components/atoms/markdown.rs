use crate::api::client::ApiClient;
use crate::data::locales::store::LocalesStore;
use crate::data::locales::tk::TK;
use crate::data::resources::{Key, ResourceStore};
use crate::data::session::SessionStore;
use crate::{
    router::{AdminRoute, Route},
    use_effect_deps,
};
use petompp_web_models::models::user::RoleData;
use wasm_bindgen::{prelude::Closure, JsCast};
use web_sys::Element;
use yew::platform::spawn_local;
use yew::prelude::*;
use yew_router::{prelude::*, Routable};
use yewdux::prelude::use_store;

#[derive(Properties, PartialEq)]
pub struct MarkdownDisplayProps {
    pub markdown: String,
    pub allowhtml: bool,         // Allows HTML tags
    pub interactive: Option<()>, // Makes links clickable
}

#[function_component(Markdown)]
pub fn markdown_display(props: &MarkdownDisplayProps) -> Html {
    let id = use_memo(
        |()| {
            "md".to_string()
                + rand::random::<[char; 8]>()
                    .into_iter()
                    .collect::<String>()
                    .as_str()
        },
        (),
    );
    let navigator = use_navigator().unwrap();
    let html = markdown::to_html_with_options(
        props.markdown.as_str(),
        &markdown::Options {
            compile: markdown::CompileOptions {
                allow_dangerous_html: props.allowhtml,
                ..markdown::CompileOptions::default()
            },
            parse: markdown::ParseOptions::gfm(),
        },
    )
    .unwrap();
    let div = web_sys::window()
        .unwrap()
        .document()
        .unwrap()
        .create_element("div")
        .unwrap();
    div.set_inner_html(&html);
    let interactive = props.interactive;
    let class = match interactive {
        Some(()) => "prose w-full max-w-full",
        None => "prose w-full max-w-full pointer-events-none",
    };
    use_effect_deps!(|interactive, id| {
        if interactive.is_some() {
            make_links_clickable(navigator.clone(), id.as_str());
        }
        || {}
    });

    html! {
        <div {class} id={id.to_string()}>
            {Html::VRef(div.into())}
        </div>
    }
}

#[derive(PartialEq, Properties)]
pub struct EditableProps {
    pub reskey: String,
}

#[function_component(Editable)]
pub fn editable(props: &EditableProps) -> Html {
    let (locales_store, _) = use_store::<LocalesStore>();
    let (session_store, _) = use_store::<SessionStore>();
    let (res_store, res_dispatch) = use_store::<ResourceStore>();
    let reskey = Key {
        reskey: props.reskey.clone(),
        lang: locales_store.curr.key().to_string(),
    };
    let navigator = use_navigator().unwrap();
    let edit_onclick = {
        let reskey = reskey.clone();
        Callback::from(move |_| {
            navigator.push(&Route::Editor {
                key: reskey.reskey.clone(),
                lang: reskey.lang.clone(),
            });
        })
    };
    let edit_class = match &session_store.user {
        Some(u) if u.role == RoleData::Admin => {
            "btn absolute top-5 right-5 btn-accent btn-xs btn-outline"
        }
        _ => "hidden",
    };
    let markdown = res_store.get_state(&reskey).cloned().unwrap_or_default();
    spawn_local(async move {
        if let Ok(md) = ApiClient::get_resource(reskey.reskey.as_str(), reskey.lang.as_str()).await
        {
            if res_store.get_state(&reskey) != Some(&md) {
                res_dispatch.reduce_mut(|store| {
                    store.add_or_update_state(&reskey, md);
                });
            }
        }
    });

    html! {
        <>
        <button class={edit_class} onclick={edit_onclick}>{locales_store.get(TK::Edit)}</button>
        <Markdown {markdown} interactive={Some(())} allowhtml={true}/>
        </>
    }
}

fn make_links_clickable(navigator: Navigator, id: &str) {
    let element = get_display_element(id);
    let links = element.query_selector_all("a").unwrap();
    for i in 0..links.length() {
        let link: Element = links.get(i).unwrap().unchecked_into();
        let Some(href) = link.get_attribute("href") else {
            continue;
        };
        match href.as_str() {
            p if p.starts_with('/') => match Route::recognize(p) {
                Some(route) => {
                    let onclick = match route {
                        Route::AdminPanel => match AdminRoute::recognize(href.as_str()) {
                            Some(admin_route) => get_route_onclick(admin_route, navigator.clone()),
                            _ => get_route_onclick(route, navigator.clone()),
                        },
                        _ => get_route_onclick(route, navigator.clone()),
                    };
                    link.add_event_listener_with_callback(
                        "click",
                        onclick.as_ref().unchecked_ref(),
                    )
                    .unwrap();
                    onclick.forget();
                }
                None => link.set_attribute("target", "_blank").unwrap(),
            },
            p if p.starts_with("http") => link.set_attribute("target", "_blank").unwrap(),
            _ => continue,
        };
    }
}

fn get_route_onclick<T: Routable + std::fmt::Debug + 'static>(
    route: T,
    navigator: Navigator,
) -> Closure<dyn Fn(Event)> {
    let navigator = navigator.clone();
    Closure::new(Box::new(move |e: Event| {
        e.prevent_default();
        navigator.push(&route);
    }))
}

fn get_display_element(id: &str) -> Element {
    let element: Element = web_sys::window()
        .unwrap()
        .document()
        .unwrap()
        .get_element_by_id(id)
        .unwrap()
        .unchecked_into();
    element
}
