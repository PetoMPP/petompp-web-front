use crate::{
    api::client::ApiClient,
    components::{
        atoms::{flag::FlagSelect, loading::Loading},
        state::State,
    },
    data::{
        resources::{
            id::{ResId, ResourceId},
            store::LocalStore,
        },
        session::SessionStore,
    },
    pages::editor::EditorState,
};
use petompp_web_models::models::{blog_data::BlogMetaData, country::Country};
use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;
use yew::{platform::spawn_local, prelude::*};
use yewdux::prelude::*;

#[derive(Clone, PartialEq, Properties)]
pub struct ResourceSelectProps {
    pub resid: Option<ResId>,
    pub lang: Option<Country>,
    pub state: Option<EditorState>,
    pub onselectedchanged: Callback<ResourceId>,
}

#[derive(Debug, Clone, PartialEq)]
enum Mode {
    Resources,
    Posts,
}

impl From<&ResId> for Mode {
    fn from(resid: &ResId) -> Self {
        match resid {
            ResId::ResKey(_) => Self::Resources,
            ResId::Blob(_) => Self::Posts,
        }
    }
}

impl Mode {
    fn next(&self) -> Self {
        match self {
            Self::Resources => Self::Posts,
            Self::Posts => Self::Resources,
        }
    }
}

#[function_component(ResourceSelect)]
pub fn resource_select(props: &ResourceSelectProps) -> Html {
    let (session_store, session_dispatch) = use_store::<SessionStore>();
    let (local_store, _) = use_store::<LocalStore>();
    let token = session_store.token.clone().unwrap_or_default();
    let data = use_state(|| State::Ok(None));
    let last_state = use_state(|| None);
    use_effect_with_deps(
        |(data, token, state, last_state, local_store)| {
            match (&*state, &**last_state) {
                (Some(State::Ok(Some(_))), Some(State::Ok(Some(_))))
                | (Some(State::Ok(None)), Some(State::Ok(None)))
                | (Some(State::Loading), Some(State::Loading)) => return,
                _ => last_state.set(state.clone()),
            }
            let data = data.clone();
            match &*data {
                State::Ok(Some(_)) => {}
                State::Loading | State::Err(_) => return,
                _ => data.set(State::Loading),
            };
            let mut cached_res = Vec::new();
            let mut cached_posts = Vec::new();
            for resid in local_store.get_all_resids() {
                match resid {
                    ResId::ResKey(_) => cached_res.push(resid),
                    ResId::Blob(_) => cached_posts.push(resid),
                }
            }
            let token = token.clone();
            spawn_local(async move {
                match ApiClient::get_res_ids(token.as_str()).await {
                    Ok((res, ps)) => {
                        let cached_res: Vec<_> = cached_res
                            .into_iter()
                            .filter(|r| !res.contains(r))
                            .collect();
                        let cached_posts: Vec<_> = cached_posts
                            .into_iter()
                            .filter(|r| !ps.contains(r))
                            .collect();
                        data.set(State::Ok(Some(((res, cached_res), (ps, cached_posts)))));
                    }
                    Err(e) => data.set(State::Err(e)),
                }
            });
        },
        (
            data.clone(),
            token,
            props.state.clone(),
            last_state.clone(),
            local_store.clone(),
        ),
    );
    let onselectedchanged_resid = {
        let props = props.clone();
        Callback::from(move |r| {
            props.onselectedchanged.emit(ResourceId::from((
                r,
                props.lang.clone().unwrap_or_default(),
            )))
        })
    };
    let onselectedchanged_lang = {
        let props = props.clone();
        Callback::from(move |c| {
            props
                .onselectedchanged
                .emit(ResourceId::from((props.resid.clone().unwrap(), c)))
        })
    };
    let list = match (*data).clone() {
        State::Ok(Some((resources, posts))) => {
            html! {
                <ResourceList currentresid={props.resid.clone()} currentlang={props.lang.clone()} {resources} {posts} onselectedchanged={onselectedchanged_resid}/>
            }
        }
        State::Ok(None) | State::Loading => html! {
            <Loading resource={"available resources".to_string()} />
        },
        State::Err(e) => {
            if let Err(redirect) = e.handle_failed_auth(session_dispatch) {
                return redirect;
            }
            html! {
                <div class={"btn btn-warning pointer-events-none"}>{"Unable to load data!"}</div>
            }
        }
    };
    let lang_select = props.lang.as_ref().map(|c| {
        html! {
            <FlagSelect country={c.clone()} onselectedchanged={onselectedchanged_lang} />
        }
    });

    html! {
        <div class={"flex flex-row gap-4 w-full lg:w-auto"}>
            <div class={"dropdown grow lg:min-w-[12rem]"}>
            <label class={"btn flex grow"} tabindex={"0"}>{props.resid.as_ref().map(|r| r.id().to_string()).unwrap_or("Select a resource!".to_string())}</label>
            <ul tabindex={"0"} class={"dropdown-content w-full flex flex-col mt-1 gap-1 z-10"}>
                {list}
            </ul>
            </div>
            {lang_select}
        </div>
    }
}

#[derive(Debug, Clone, PartialEq, Properties)]
struct ResourceListProps {
    pub currentresid: Option<ResId>,
    pub currentlang: Option<Country>,
    pub resources: (Vec<ResId>, Vec<ResId>),
    pub posts: (Vec<ResId>, Vec<ResId>),
    pub onselectedchanged: Callback<ResId>,
}

#[function_component(ResourceList)]
fn resource_list(props: &ResourceListProps) -> Html {
    let mode: UseStateHandle<Mode> = use_state_eq(|| {
        (&props.currentresid)
            .as_ref()
            .map(|r| r.into())
            .unwrap_or(Mode::Resources)
    });
    let res_page = use_state_eq(|| 0);
    let blog_page = use_state_eq(|| 0);
    let onclick = {
        let mode = mode.clone();
        Callback::from(move |_| mode.set(mode.next()))
    };
    let get_btn_onclick = |r: &ResId| {
        let props = props.clone();
        let r = r.clone();
        Callback::from(move |_| props.onselectedchanged.emit(r.clone()))
    };
    let res_into_button = |r: &ResId, cache_only: bool| {
        let id = r.id();
        let mut class = classes!("btn", "flex");
        if props.currentresid.as_ref() == Some(&r) {
            class.push("btn-primary");
        };
        if cache_only {
            class.push("btn-outline");
            class.push("italic");
        };
        html! {
            <li onclick={get_btn_onclick(r)} {class}>{id}</li>
        }
    };
    let vec_into_elements = |vec: Vec<(&ResId, bool)>| {
        vec.clone()
            .chunks(3)
            .into_iter()
            .map(|x| {
                x.iter()
                    .cloned()
                    .map(|(r, c)| res_into_button(r, c))
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<Vec<_>>>()
    };
    let (text, page, elements) = match *mode {
        Mode::Resources => (
            "Resources",
            res_page,
            vec_into_elements(
                props
                    .resources
                    .0
                    .iter()
                    .map(|r| (r, false))
                    .chain(props.resources.1.iter().map(|r| (r, true)))
                    .collect::<Vec<_>>(),
            ),
        ),
        Mode::Posts => (
            "Blog posts",
            blog_page,
            vec_into_elements(
                props
                    .posts
                    .0
                    .iter()
                    .map(|r| (r, false))
                    .chain(props.posts.1.iter().map(|r| (r, true)))
                    .collect::<Vec<_>>(),
            ),
        ),
    };
    let page_count = elements.len();
    let inc_page = {
        let page = page.clone();
        Callback::from(move |_| page.set((*page + 1).min(page_count - 1)))
    };
    let dec_page = {
        let page = page.clone();
        Callback::from(move |_| page.set((*page - 1).max(0)))
    };
    const NEW_INPUT_ID: &str = "new-input-00";
    let (_, local_dispatch) = use_store::<LocalStore>();
    let new_element_input = use_state(|| false);
    let new_element_content = match *new_element_input {
        true => {
            let onkeydown = {
                let new_element_input = new_element_input.clone();
                let onselectedchanged = props.onselectedchanged.clone();
                let mode = mode.clone();
                let currlang = props.currentlang.clone();
                Callback::from(move |e: KeyboardEvent| {
                    if e.key() != "Enter" {
                        return;
                    }
                    let element: HtmlInputElement = e.target_unchecked_into();
                    let id = element.value();
                    if id.is_empty() {
                        return;
                    }
                    let (resid, meta) = match *mode {
                        Mode::Resources => (ResId::ResKey(id), None),
                        Mode::Posts => (ResId::Blob(id), Some(BlogMetaData::default())),
                    };
                    local_dispatch.reduce_mut(|s| {
                        s.insert(
                            resid.clone(),
                            currlang.clone().unwrap_or_default().key(),
                            String::new(),
                            meta.clone(),
                        );
                    });
                    onselectedchanged.emit(resid);
                    new_element_input.set(false);
                })
            };
            let onclick = {
                let new_element_input = new_element_input.clone();
                Callback::from(move |e: MouseEvent| {
                    e.set_cancel_bubble(true);
                    new_element_input.set(false);
                })
            };
            html! {
                <div class={"flex flex-row gap-1"}>
                    <input {onkeydown} id={NEW_INPUT_ID} type={"text"} class={"input input-bordered input-xs text-base-content w-full"} placeholder={"Name.."} />
                    <a {onclick} type={"reset"} class={"btn btn-circle btn-xs btn-error"} >{"X"}</a>
                </div>
            }
        }
        false => html! {{format!("New {}", text[..text.len() - 1].to_string())}},
    };
    use_effect_with_deps(
        |new_element_input| {
            if !**new_element_input {
                return;
            }
            if let Some(input) = web_sys::window()
                .and_then(|w| w.document())
                .and_then(|d| d.get_element_by_id(NEW_INPUT_ID))
                .and_then(|e| e.dyn_into::<HtmlInputElement>().ok())
            {
                input.focus().unwrap();
            }
        },
        new_element_input.clone(),
    );
    let new_element_onclick = {
        let new_element_input = new_element_input.clone();
        Callback::from(move |_| new_element_input.set(true))
    };
    let mut new_element_class = classes!("btn", "btn-secondary", "flex");
    if *new_element_input {
        new_element_class.push("no-animation");
    }
    let elements = elements[*page].clone().into_iter();

    html! {
        <div class={"flex flex-col gap-1"}>
            <li onclick={new_element_onclick} class={new_element_class}>{new_element_content}</li>
            <li class={"join flex w-full"}>
                <a onclick={dec_page} class={"join-item flex btn"}>{"«"}</a>
                <a {onclick} class={"join-item px-0.5 flex grow shrink btn no-animation content-center"}>{format!("{} {}/{}",text, *page + 1, page_count)}</a>
                <a onclick={inc_page} class={"join-item flex btn"}>{"»"}</a>
            </li>
            {for elements}
        </div>
    }
}

#[derive(Debug, Clone, PartialEq, Properties)]
struct ResourceButtonProps {
    pub resid: ResId,
    pub active: bool,
    pub onclick: Option<Callback<MouseEvent>>,
}

#[function_component(ResourceButton)]
fn resource_button(props: &ResourceButtonProps) -> Html {
    let class = match props.active {
        true => "btn btn-primary",
        false => "btn",
    };

    html! {
        <li onclick={props.onclick.clone()} {class}>{props.resid.id()}</li>
    }
}
