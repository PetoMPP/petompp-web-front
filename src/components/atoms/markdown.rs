use crate::api::client::ApiClient;
use crate::data::locales::store::LocalesStore;
use crate::data::locales::tk::TK;
use crate::data::resources::{ResId, ResourceId};
use crate::data::session::SessionStore;
use crate::{router::route::Route, use_effect_deps};
use petompp_web_models::models::user::RoleData;
use wasm_bindgen::JsCast;
use web_sys::Element;
use yew::platform::spawn_local;
use yew::prelude::*;
use yew_router::prelude::*;
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
        |()| web_sys::window().unwrap().crypto().unwrap().random_uuid()[..10].to_string(),
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
    let (reskey, lang) = (props.reskey.clone(), locales_store.curr.clone());
    let navigator = use_navigator().unwrap();
    let edit_onclick = {
        let (reskey, lang) = (reskey.clone(), lang.clone());
        Callback::from(move |_| {
            navigator
                .push_with_query(
                    &Route::Editor,
                    &ResourceId::from((ResId::ResKey(reskey.clone()), lang.clone())),
                )
                .unwrap()
        })
    };
    let edit_button = session_store.user.as_ref().and_then(|u| {
        if u.role != RoleData::Admin {
            return None;
        }
        Some(html! {
            <button class={"btn absolute top-5 right-5 btn-accent btn-xs btn-outline"} onclick={edit_onclick}>{locales_store.get(TK::Edit)}</button>
        })
    });
    let markdown = use_state_eq(|| None);
    {
        let markdown = markdown.clone();
        spawn_local(async move {
            if markdown.is_some() {
                return;
            }
            if let Ok(md) = ApiClient::get_resource(reskey.as_str(), &lang).await {
                markdown.set(Some(md));
            }
        })
    };
    let markdown = (*markdown).clone().unwrap_or_default();

    html! {
        <>
        {edit_button}
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
        if let Some(onclick) = Route::get_onclick_from_str(href.as_str(), navigator.clone()) {
            link.add_event_listener_with_callback("click", onclick.as_ref().unchecked_ref())
                .unwrap();
            onclick.forget();
        };
        if href.starts_with("http") {
            link.set_attribute("target", "_blank").unwrap();
        }
    }
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
