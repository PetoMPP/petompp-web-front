use crate::{
    api::client::ApiClient,
    components::atoms::flag::FlagSelect,
    data::{
        resources::{ResId, ResourceId},
        session::SessionStore,
    },
    handle_api_error,
};
use petompp_web_models::models::country::Country;
use yew::{platform::spawn_local, prelude::*};
use yewdux::prelude::*;

#[derive(Clone, PartialEq, Properties)]
pub struct ResourceSelectProps {
    pub resid: Option<ResId>,
    pub lang: Option<Country>,
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
    let token = session_store.token.clone().unwrap_or_default();
    let error_state = use_state_eq(|| None);
    let data = use_state(|| None);
    use_effect_with_deps(
        |(data, token, error_state)| {
            if data.is_some() {
                return;
            }
            let data = data.clone();
            let token = token.clone();
            let error_state = error_state.clone();
            spawn_local(async move {
                match ApiClient::get_res_ids(token.as_str()).await {
                    Ok((res, ps)) => data.set(Some((res, ps))),
                    Err(e) => {
                        error_state.set(Some(e));
                    }
                }
            });
        },
        (data.clone(), token, error_state.clone()),
    );
    handle_api_error!(error_state, session_dispatch, None);
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
        Some((resources, posts)) => {
            html! {
                <ResourceList currentresid={props.resid.clone()} resources={resources} posts={posts} onselectedchanged={onselectedchanged_resid}/>
            }
        }
        None => html! {
            <span class={"flex mx-auto loading loading-ring loading-lg"}/>
        },
    };
    let lang_select = props.lang.as_ref().map(|c| {
        html! {
            <FlagSelect country={c.clone()} onselectedchanged={onselectedchanged_lang} />
        }
    });

    html! {
        <div class={"flex flex-row gap-4"}>
            <div class={"dropdown min-w-[16rem]"}>
            <label class={"btn w-full"} tabindex={"0"}>{props.resid.as_ref().map(|r| format!("{}: {}", r.type_str(), r.id())).unwrap_or("Select a resource!".to_string())}</label>
            <ul tabindex={"0"} class={"dropdown-content w-full flex flex-col mt-1 gap-1 z-[1]"}>
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
    pub resources: Vec<ResId>,
    pub posts: Vec<ResId>,
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
    let get_btn_onclick = |r: ResId| {
        let props = props.clone();
        Callback::from(move |_| props.onselectedchanged.emit(r.clone()))
    };
    let res_into_button = |r: ResId| {
        let id = r.id();
        let class = match props.currentresid.as_ref() == Some(&r) {
            true => "btn btn-primary",
            false => "btn",
        };
        html! {
            <li onclick={get_btn_onclick(r.clone())} {class}>{id}</li>
        }
    };
    let vec_into_elements = |vec: &Vec<ResId>| {
        vec.clone()
            .chunks(3)
            .into_iter()
            .map(|x| x.iter().cloned().map(res_into_button).collect::<Vec<_>>())
            .collect::<Vec<Vec<_>>>()
    };
    let (text, page, elements) = match *mode {
        Mode::Resources => ("Resources", res_page, vec_into_elements(&props.resources)),
        Mode::Posts => ("Blog posts", blog_page, vec_into_elements(&props.posts)),
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
    let elements = elements[*page].clone();

    html! {
        <>
        <li class={"join flex w-full"}>
            <a onclick={dec_page} class={"join-item flex btn"}>{"«"}</a>
            <a {onclick} class={"join-item flex grow shrink btn no-animation content-center"}><div>{text}</div><div>{format!("{}/{}",*page + 1, page_count)}</div></a>
            <a onclick={inc_page} class={"join-item flex btn"}>{"»"}</a>
        </li>
        {for elements}
        </>
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