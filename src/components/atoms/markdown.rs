use crate::components::atoms::loading::Loading;
use crate::components::state::State;
use crate::data::locales::store::LocalesStore;
use crate::data::locales::tk::TK;
use crate::data::resources::{ResId, ResourceId};
use crate::data::session::SessionStore;
use crate::router::route::Route;
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
    use_effect_with_deps(
        |(interactive, id, navigator)| {
            if interactive.is_some() {
                make_links_clickable(navigator.clone(), id.as_str());
            }
        },
        (interactive, id.clone(), navigator.clone()),
    );

    html! {
        <div {class} id={id.to_string()}>
            {Html::VRef(div.into())}
        </div>
    }
}

#[derive(PartialEq, Properties)]
pub struct EditableProps {
    pub resid: ResId,
}

#[function_component(Editable)]
pub fn editable(props: &EditableProps) -> Html {
    let (locales_store, _) = use_store::<LocalesStore>();
    let (_, session_dispatch) = use_store::<SessionStore>();
    let (resid, lang) = (props.resid.clone(), locales_store.curr.clone());
    let markdown = use_state_eq(|| State::Ok(None));
    {
        let markdown = markdown.clone();
        spawn_local(async move {
            match &*markdown {
                State::Ok(Some((r, l, _))) if r == &resid && l == &lang => return,
                State::Loading | State::Err(_) => return,
                _ => markdown.set(State::Loading),
            };

            match resid.get_value(&lang).await {
                Ok(md) => markdown.set(State::Ok(Some((resid, lang, md)))),
                Err(e) => markdown.set(State::Err(e)),
            }
        })
    };
    let markdown = match &*markdown {
        State::Ok(Some((_, _, md))) => html! {
            <div class={"animate-fade"}>
            <Markdown markdown={md.clone()} interactive={Some(())} allowhtml={true}/>
            </div>
        },
        State::Ok(None) | State::Loading => html! {
            <Loading resource={"page content".to_string()} />
        },
        State::Err(e) => {
            if let Err(redirect) = e.handle_failed_auth(session_dispatch) {
                return redirect;
            }
            html! {
                <p class={"text-xl text-error font-semibold"}>{e.to_string()}</p>
            }
        }
    };

    html! {
        <>
        <EditButton resid={props.resid.clone()} />
        {markdown}
        </>
    }
}

#[derive(Clone, PartialEq, Properties)]
struct EditButtonProps {
    resid: ResId,
}

#[function_component(EditButton)]
fn edit_button(props: &EditButtonProps) -> Html {
    let (locales_store, _) = use_store::<LocalesStore>();
    let (session_store, _) = use_store::<SessionStore>();
    let navigator = use_navigator().unwrap();
    let (resid, lang) = (props.resid.clone(), locales_store.curr.clone());
    let edit_onclick = Callback::from(move |_| {
        navigator
            .push_with_query(
                &Route::Editor,
                &ResourceId::from((resid.clone(), lang.clone())),
            )
            .unwrap()
    });

    if session_store.user.as_ref().map(|u| &u.role) != Some(&RoleData::Admin) {
        return html! {};
    }

    html! {
        <button class={"btn absolute top-5 right-5 btn-accent btn-xs btn-outline"} onclick={edit_onclick}>{locales_store.get(TK::Edit)}</button>
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
