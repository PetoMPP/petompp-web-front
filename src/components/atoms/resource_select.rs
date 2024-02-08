use crate::{
    api::{client::ApiClient, editor::EditorClient},
    components::{
        atoms::{flag::FlagSelect, loading::Loading},
        state::State,
    },
    data::{
        locales::{store::LocalesStore, tk::TK},
        resources::{
            id::{BlobType, ResId, ResourceId},
            store::LocalStore,
        },
        session::SessionStore,
    },
    pages::editor::{EditorData, EditorState},
};
use petompp_web_models::models::{
    blob::{blog::BlogMetaData, project::ProjectMetaData},
    country::Country,
};
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
    Blogs,
    Projects,
}

impl From<&ResId> for Mode {
    fn from(resid: &ResId) -> Self {
        match resid {
            ResId::ResKey(_) => Self::Resources,
            ResId::Blob(BlobType::Blog(_)) => Self::Blogs,
            ResId::Blob(BlobType::Project(_)) => Self::Projects,
        }
    }
}

impl Mode {
    fn next(&self) -> Self {
        match self {
            Self::Resources => Self::Blogs,
            Self::Blogs => Self::Projects,
            Self::Projects => Self::Resources,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
struct ResourceSelectState {
    resources: (Vec<ResId>, Vec<ResId>),
    blogs: (Vec<ResId>, Vec<ResId>),
    projects: (Vec<ResId>, Vec<ResId>),
}

#[function_component(ResourceSelect)]
pub fn resource_select(props: &ResourceSelectProps) -> Html {
    let (session_store, session_dispatch) = use_store::<SessionStore>();
    let (local_store, _) = use_store::<LocalStore>();
    let (locales_store, _) = use_store::<LocalesStore>();
    let token = session_store.token.clone().unwrap_or_default();
    let state = use_state(|| State::Ok(None));
    let last_state = use_state(|| None);
    use_effect_with_deps(
        |(state, token, editor_state, last_editor_state, local_store)| {
            match (editor_state, &**last_editor_state) {
                (Some(State::Ok(Some(_))), Some(State::Ok(Some(_))))
                | (Some(State::Ok(None)), Some(State::Ok(None)))
                | (Some(State::Loading), Some(State::Loading)) => return,
                _ => last_editor_state.set(editor_state.clone()),
            }
            let state = state.clone();
            match &*state {
                State::Ok(Some(_)) => {}
                State::Loading | State::Err(_) => return,
                _ => state.set(State::Loading),
            };
            let mut cached_res = Vec::new();
            let mut cached_blogs = Vec::new();
            let mut cached_projects = Vec::new();
            for resid in local_store.get_all_resids() {
                match &resid {
                    ResId::ResKey(_) => cached_res.push(resid),
                    ResId::Blob(bt) => match bt {
                        BlobType::Blog(_) => cached_blogs.push(resid),
                        BlobType::Project(_) => cached_projects.push(resid),
                    },
                }
            }
            let token = token.clone();
            spawn_local(async move {
                match ApiClient::get_res_ids(token.as_str()).await {
                    Ok((res, bl, prj)) => {
                        let cached_res: Vec<_> = cached_res
                            .into_iter()
                            .filter(|r| !res.contains(r))
                            .collect();
                        let cached_blogs: Vec<_> = cached_blogs
                            .into_iter()
                            .filter(|r| !bl.contains(r))
                            .collect();
                        let cached_projects: Vec<_> = cached_projects
                            .into_iter()
                            .filter(|r| !prj.contains(r))
                            .collect();
                        state.set(State::Ok(Some(ResourceSelectState {
                            resources: (res, cached_res),
                            blogs: (bl, cached_blogs),
                            projects: (prj, cached_projects),
                        })));
                    }
                    Err(e) => state.set(State::Err(e)),
                }
            });
        },
        (
            state.clone(),
            token,
            props.state.clone(),
            last_state.clone(),
            local_store.clone(),
        ),
    );
    let onselectedchanged_resid = {
        let props = props.clone();
        Callback::from(move |r| {
            props
                .onselectedchanged
                .emit(ResourceId::from((r, props.lang.unwrap_or_default())))
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
    let list = match (*state).clone() {
        State::Ok(Some(state)) => {
            html! {
                <ResourceList currentresid={props.resid.clone()} currentlang={props.lang}
                    resources={state.resources} blogs={state.blogs} projects={state.projects}
                    onselectedchanged={onselectedchanged_resid}/>
            }
        }
        State::Ok(None) | State::Loading => html! {
            <Loading resource={locales_store.get(TK::AvailableResources)} />
        },
        State::Err(e) => {
            if let Err(redirect) = e.handle_failed_auth(session_dispatch) {
                return redirect;
            }
            gloo::console::error!(e.to_string());
            html! {
                <div class={"btn btn-warning pointer-events-none"}>{locales_store.get(TK::ErrorOccured)}</div>
            }
        }
    };
    let lang_select = props.lang.as_ref().map(|c| {
        html! {
            <FlagSelect country={*c} onselectedchanged={onselectedchanged_lang} />
        }
    });

    html! {
        <div class={"flex flex-row gap-4 w-full lg:w-auto"}>
            <div class={"dropdown grow lg:min-w-[12rem]"}>
            <label class={"btn flex grow"} tabindex={"0"}>{props.resid.as_ref().map(|r| r.id().to_string()).unwrap_or_else(|| locales_store.get(TK::SelectResource))}</label>
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
    pub blogs: (Vec<ResId>, Vec<ResId>),
    pub projects: (Vec<ResId>, Vec<ResId>),
    pub onselectedchanged: Callback<ResId>,
}

#[function_component(ResourceList)]
fn resource_list(props: &ResourceListProps) -> Html {
    let (locales_store, _) = use_store::<LocalesStore>();
    let (session_store, session_dispatch) = use_store::<SessionStore>();
    let mode: UseStateHandle<Mode> = use_state_eq(|| {
        props
            .currentresid
            .as_ref()
            .map(|r| r.into())
            .unwrap_or(Mode::Resources)
    });
    let res_page = use_state_eq(|| 0);
    let blog_page = use_state_eq(|| 0);
    let proj_page = use_state_eq(|| 0);
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
        if props.currentresid.as_ref() == Some(r) {
            class.push("btn-primary");
        };
        if cache_only {
            class.push("btn-outline italic bg-base-100");
        };
        html! {
            <li onclick={get_btn_onclick(r)} {class}>{id}</li>
        }
    };
    let vec_into_elements = |vec: Vec<(&ResId, bool)>| {
        if vec.is_empty() {
            return vec![vec![]];
        }

        vec.clone()
            .chunks(3)
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
            locales_store.get(TK::Resources),
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
        Mode::Blogs => (
            locales_store.get(TK::BlogPosts),
            blog_page,
            vec_into_elements(
                props
                    .blogs
                    .0
                    .iter()
                    .map(|r| (r, false))
                    .chain(props.blogs.1.iter().map(|r| (r, true)))
                    .collect::<Vec<_>>(),
            ),
        ),
        Mode::Projects => (
            locales_store.get(TK::Projects),
            proj_page,
            vec_into_elements(
                props
                    .projects
                    .0
                    .iter()
                    .map(|r| (r, false))
                    .chain(props.projects.1.iter().map(|r| (r, true)))
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
        Callback::from(move |_| page.set(page.max(1) - 1))
    };
    const NEW_INPUT_ID: &str = "new-input-00";
    let (_, local_dispatch) = use_store::<LocalStore>();
    let new_element_input = use_state(|| State::Ok(false));
    let new_element_content = match &*new_element_input {
        State::Ok(true) => {
            let onkeydown = {
                let new_element_input = new_element_input.clone();
                let onselectedchanged = props.onselectedchanged.clone();
                let mode = mode.clone();
                let currlang = props.currentlang.unwrap_or_default();
                let token = session_store.token.clone();
                Callback::from(move |e: KeyboardEvent| {
                    if e.key() != "Enter" {
                        return;
                    }
                    let element: HtmlInputElement = e.target_unchecked_into();
                    let id = element.value();
                    if id.is_empty() {
                        return;
                    }
                    let new_element_input = new_element_input.clone();
                    let token = token.clone();
                    let mode = mode.clone();
                    let local_dispatch = local_dispatch.clone();
                    let onselectedchanged = onselectedchanged.clone();
                    new_element_input.set(State::Loading);
                    spawn_local(async move {
                        match ApiClient::get_res_ids(token.unwrap_or_default().as_str()).await {
                            Ok((res, bl, prj)) => {
                                let (resid, data, exists) = match *mode {
                                    Mode::Resources => {
                                        let resid = ResId::ResKey(id);
                                        let contains = res.contains(&resid);
                                        (resid, EditorData::Resource(Default::default()), contains)
                                    }
                                    Mode::Blogs => {
                                        let meta = BlogMetaData::empty(&id, currlang);
                                        let resid = ResId::Blob(BlobType::Blog(id));
                                        let contains = bl.contains(&resid);
                                        (
                                            resid,
                                            EditorData::Blog((Default::default(), meta)),
                                            contains,
                                        )
                                    }
                                    Mode::Projects => {
                                        let meta = ProjectMetaData::empty(&id, currlang);
                                        let resid = ResId::Blob(BlobType::Project(id));
                                        let contains = prj.contains(&resid);
                                        (
                                            resid,
                                            EditorData::Project((Default::default(), meta)),
                                            contains,
                                        )
                                    }
                                };
                                if !exists {
                                    local_dispatch.reduce_mut(|s| {
                                        if s.exists(&resid) {
                                            return;
                                        }
                                        s.insert(resid.clone(), currlang.key(), data);
                                    });
                                }
                                onselectedchanged.emit(resid);
                                new_element_input.set(State::Ok(false));
                            }
                            Err(e) => new_element_input.set(State::Err(e)),
                        }
                    });
                })
            };
            let onclick = {
                let new_element_input = new_element_input.clone();
                Callback::from(move |e: MouseEvent| {
                    e.set_cancel_bubble(true);
                    new_element_input.set(State::Ok(false));
                })
            };
            html! {
                <div class={"flex flex-row gap-1"}>
                    <input {onkeydown} id={NEW_INPUT_ID} type={"text"} class={"input input-bordered input-xs text-base-content w-full"} placeholder={"Name.."} />
                    <a {onclick} type={"reset"} class={"btn btn-circle btn-xs btn-error"} >{"X"}</a>
                </div>
            }
        }
        State::Ok(false) => html! {{
            match *mode {
                Mode::Resources => locales_store.get(TK::NewResource),
                Mode::Blogs => locales_store.get(TK::NewBlogPost),
                Mode::Projects => locales_store.get(TK::NewProject),
            }
        }},
        State::Loading => html! {
            <Loading/>
        },
        State::Err(e) => {
            if let Err(redirect) = e.handle_failed_auth(session_dispatch) {
                return redirect;
            }
            html! {
                <div class={"btn btn-warning pointer-events-none"}>{locales_store.get(TK::ErrorOccured)}</div>
            }
        }
    };
    use_effect_with_deps(
        |new_element_input| {
            if **new_element_input != State::Ok(true) {
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
        Callback::from(move |_| new_element_input.set(State::Ok(true)))
    };
    let mut new_element_class = classes!("btn", "btn-secondary", "flex");
    if *new_element_input == State::Ok(true) {
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
