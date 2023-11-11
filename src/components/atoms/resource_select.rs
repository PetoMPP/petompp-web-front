use crate::{
    api::client::ApiClient,
    components::{
        atoms::{flag::FlagSelect, loading::Loading},
        state::State,
    },
    data::{
        resources::id::{ResId, ResourceId},
        session::SessionStore,
    },
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
    let data = use_state(|| State::Ok(None));
    use_effect_with_deps(
        |(data, token)| {
            let data = data.clone();
            match &*data {
                State::Ok(Some(_)) | State::Loading | State::Err(_) => return,
                _ => data.set(State::Loading),
            };
            let token = token.clone();
            spawn_local(async move {
                match ApiClient::get_res_ids(token.as_str()).await {
                    Ok((res, ps)) => data.set(State::Ok(Some((res, ps)))),
                    Err(e) => data.set(State::Err(e)),
                }
            });
        },
        (data.clone(), token),
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
                <ResourceList currentresid={props.resid.clone()} resources={resources} posts={posts} onselectedchanged={onselectedchanged_resid}/>
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
        <div class={"flex flex-row gap-2 w-full lg:w-auto"}>
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
            true => "btn btn-primary flex",
            false => "btn flex",
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
        <div class={"flex flex-col gap-1"}>
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
