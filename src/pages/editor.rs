use crate::{
    api::{
        client::{ApiClient, RequestError},
        editor::EditorClient,
        resource::ResourceClient,
    },
    components::{
        atoms::{
            collapse::Collapse, loading::Loading, markdown::Editable,
            resource_select::ResourceSelect,
        },
        organisms::{
            blog::blog_meta_editor::BlogMetaEditor,
            editor::atoms::{
                delete_button::DeleteButton, discard_button::DiscardButton, save_button::SaveButton,
            },
            markdown::markdown_editor::MarkdownEditor,
            markdown::markdown_preview::MarkdownPreview,
            project::project_meta_editor::ProjectMetaEditor,
        },
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
    pages::page_base::PageBase,
    router::route::Route,
    utils::style::get_svg_bg_mask_style,
};
use petompp_web_models::models::blob::{blog::BlogMetaData, project::ProjectMetaData};
use petompp_web_models::models::country::Country;
use web_sys::HtmlInputElement;
use yew::platform::spawn_local;
use yew::prelude::*;
use yew_router::prelude::*;
use yewdux::prelude::*;

pub type EditorState = State<Option<EditorDataState>, RequestError>;

#[derive(Debug, Clone, PartialEq)]
pub enum EditorData {
    Resource(String),
    Blog((String, BlogMetaData)),
    Project((String, ProjectMetaData)),
}

impl std::fmt::Display for EditorData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EditorData::Resource(s) | EditorData::Blog((s, _)) | EditorData::Project((s, _)) => {
                f.write_str(s.as_str())
            }
        }
    }
}

impl EditorData {
    pub fn with_string(self, s: String) -> Self {
        match self {
            Self::Resource(_) => Self::Resource(s),
            Self::Blog((_, m)) => Self::Blog((s, m)),
            Self::Project((_, m)) => Self::Project((s, m)),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct EditorDataState {
    pub is_new: Option<bool>,
    pub id: (ResId, Country),
    pub data: EditorData,
}

#[derive(Clone, PartialEq, Properties)]
pub struct EditorProps {
    pub state: EditorState,
    pub onstatechanged: Callback<EditorState>,
    pub resid: Option<ResId>,
    pub lang: Option<Country>,
}

#[function_component(Editor)]
pub fn editor() -> Html {
    let location = use_location().unwrap();
    let (resid, lang) = location
        .query::<ResourceId>()
        .ok()
        .and_then(|r| {
            TryInto::<(ResId, Country)>::try_into(r)
                .map_err(|e| gloo::console::error!(e))
                .ok()
        })
        .map(|(r, l)| (Some(r), Some(l)))
        .unwrap_or_default();
    let (_, session_dispatch) = use_store::<SessionStore>();
    let (local_store, local_dispatch) = use_store::<LocalStore>();
    let (locales_store, _) = use_store::<LocalesStore>();
    let navigator = use_navigator().unwrap();
    let state = use_state_eq(|| EditorState::Ok(None));
    let is_preview = use_state_eq(|| false);
    use_effect_with_deps(
        move |(resid, lang, state, local_store)| {
            let Some(resid) = resid.clone() else {
                state.set(State::Ok(None));
                return;
            };
            let Some(lang) = *lang else {
                state.set(State::Ok(None));
                return;
            };
            let state = state.clone();
            let local_store = local_store.clone();
            let cached_state = local_store.get(&resid, lang.key());
            let any_cached_state = cached_state.is_some() || local_store.exists(&resid);
            match &*state {
                // we already know that the resource exists or not, no need to call api
                State::Ok(Some(data_state))
                    if data_state.is_new.is_some()
                        && &data_state.id.0 == &resid
                        && &data_state.id.1 == &lang =>
                {
                    if let Some(local_value) = &cached_state {
                        if &data_state.data != local_value {
                            state.set(State::Ok(Some(EditorDataState {
                                data: local_value.clone(),
                                ..data_state.clone()
                            })));
                        }
                    }
                    return;
                }
                State::Loading | State::Err(_) => return,
                _ => state.set(State::Loading),
            };
            spawn_local(async move {
                // does current resource exist?
                let (is_new, new_state) = match &resid {
                    ResId::Blob(blob_type) => match ApiClient::get_data(blob_type, lang).await {
                        Ok(ed) => (ed.is_none(), ed),
                        Err(e) => match cached_state.is_some() || any_cached_state {
                            true => (true, None),
                            false => {
                                state.set(State::Err(e));
                                return;
                            }
                        },
                    },
                    ResId::ResKey(k) => match ApiClient::get_resource(k.as_str(), &lang).await {
                        Ok((c, v)) => match c == lang {
                            true => (false, Some(EditorData::Resource(v))),
                            false => (false, None),
                        },
                        Err(e) => match cached_state.is_some() || any_cached_state {
                            true => (true, None),
                            false => {
                                state.set(State::Err(e));
                                return;
                            }
                        },
                    },
                };
                match (new_state, cached_state) {
                    (Some(_), Some(cs)) => {
                        state.set(State::Ok(Some(EditorDataState {
                            data: cs.clone(),
                            id: (resid.clone(), lang),
                            is_new: Some(is_new),
                        })));
                    }
                    (Some(ns), None) => {
                        state.set(State::Ok(Some(EditorDataState {
                            data: ns.clone(),
                            id: (resid.clone(), lang),
                            is_new: Some(is_new),
                        })));
                    }
                    (None, Some(cs)) => {
                        state.set(State::Ok(Some(EditorDataState {
                            data: cs.clone(),
                            id: (resid.clone(), lang),
                            is_new: Some(is_new),
                        })));
                    }
                    (None, None) => {
                        let data = match &resid {
                            ResId::Blob(blob_type) => match blob_type {
                                BlobType::Blog(id) => {
                                    EditorData::Blog((String::new(), BlogMetaData::empty(id, lang)))
                                }
                                BlobType::Project(id) => EditorData::Project((
                                    String::new(),
                                    ProjectMetaData::empty(id, lang),
                                )),
                            },
                            ResId::ResKey(_) => EditorData::Resource(String::new()),
                        };
                        state.set(State::Ok(Some(EditorDataState {
                            data,
                            id: (resid.clone(), lang),
                            is_new: Some(is_new),
                        })));
                    }
                }
            })
        },
        (resid.clone(), lang, state.clone(), local_store.clone()),
    );
    if let State::Err(e) = &*state {
        if let Err(redirect) = e.handle_failed_auth(session_dispatch.clone()) {
            return redirect;
        }
    }
    let (editor, title) = match &*state {
        State::Ok(Some(state)) => {
            let (resid, lang) = state.id.clone();
            let editor = match &*is_preview {
                true => {
                    html! {<MarkdownPreview data={state.clone()} />}
                }
                false => {
                    let onchanged = {
                        let local_dispatch = local_dispatch.clone();
                        let resid = resid.clone();
                        let lang = lang.clone();
                        Callback::from(move |data: EditorData| {
                            local_dispatch
                                .reduce_mut(|store| store.insert(resid.clone(), lang.key(), data));
                        })
                    };
                    html! {
                        <MarkdownEditor state={state.data.clone()} {onchanged}/>
                    }
                }
            };
            let action = match state.is_new.unwrap_or_default() {
                true => locales_store.get(TK::Creating),
                false => locales_store.get(TK::Editing),
            };
            (editor, format!("{}: {}", action, resid.id()))
        }
        State::Ok(None) => {
            let text = locales_store.get(TK::NothingSelected);
            let editor = html! {
                <div class={"w-full flex rounded-lg bg-base-100"}>
                    <p class={"mx-auto py-4 text-xl font-semibold"}>{&text}</p>
                </div>
            };
            (editor, text)
        }
        State::Loading => (
            html! {
                <Loading resource={locales_store.get(TK::PageContents)} />
            },
            locales_store.get(TK::Loading),
        ),
        State::Err(e) => {
            if let Err(redirect) = e.handle_failed_auth(session_dispatch) {
                return redirect;
            }
            (
                html! {
                    <div class={"w-full flex flex-col gap-4 rounded-lg bg-error"}>
                        <p class={"mx-auto py-4 text-error-content text-xl font-semibold"}>{locales_store.get(TK::ErrorOccured)}</p>
                        <p class={"mx-auto py-4 text-error-content"}>{e.to_string()}</p>
                    </div>
                },
                String::new(),
            )
        }
    };
    let reload = match &*state {
        State::Err(_) => {
            let state = state.clone();
            Some(html! {
                <button class={"btn btn-square"} onclick={Callback::from(move |_| {
                    state.set(State::Ok(None));
                })}>
                    <div class={"bg-base-content h-10 w-10"} style={get_svg_bg_mask_style("/img/ui/reload.svg")}/>
                </button>
            })
        }
        _ => None,
    };
    let go_back = match &*state {
        State::Ok(None) => None,
        _ => {
            let navigator = navigator.clone();
            Some(html! {
                <button class={"btn btn-square"} onclick={Callback::from(move |_| navigator.push(&Route::Editor))}>
                    <div class={"bg-base-content h-10 w-10"} style={get_svg_bg_mask_style("/img/ui/back.svg")}/>
                </button>
            })
        }
    };
    let onselectedchanged = {
        let navigator = navigator.clone();
        Callback::from(move |resource_id| {
            navigator
                .push_with_query(&Route::Editor, &resource_id)
                .unwrap()
        })
    };
    let onstatechanged = {
        let state = state.clone();
        Callback::from(move |new_state: EditorState| state.set(new_state))
    };
    let meta_editor = match &*state {
        State::Ok(Some(state)) => {
            let local_dispatch = local_dispatch.clone();
            let (resid, lang) = state.id.clone();
            match state.data.clone() {
                EditorData::Blog((value, meta)) => {
                    let ondatachanged = Callback::from(move |new_data: BlogMetaData| {
                        local_dispatch.reduce_mut(|store| {
                            store.insert(
                                resid.clone(),
                                lang.key(),
                                EditorData::Blog((value.clone(), new_data)),
                            )
                        })
                    });
                    Some(html! {
                        <Collapse label={locales_store.get(TK::BlogPostMetadata)}>
                            <BlogMetaEditor data={meta.clone()} {ondatachanged} />
                        </Collapse>
                    })
                }
                EditorData::Project((value, meta)) => {
                    let ondatachanged = Callback::from(move |new_data: ProjectMetaData| {
                        local_dispatch.reduce_mut(|store| {
                            store.insert(
                                resid.clone(),
                                lang.key(),
                                EditorData::Project((value.clone(), new_data)),
                            )
                        })
                    });
                    Some(html! {
                        <Collapse label={locales_store.get(TK::ProjectMetadata)}>
                            <ProjectMetaEditor data={meta.clone()} {ondatachanged} />
                        </Collapse>
                    })
                }
                EditorData::Resource(_) => None,
            }
        }
        State::Loading => {
            Some(html! { <Loading resource={locales_store.get(TK::BlogPostMetadata)} /> })
        }
        _ => None,
    };
    let is_new = match &*state {
        State::Ok(Some(s)) => s.is_new,
        _ => None,
    };
    let edit_text = {
        let edit_pref = match is_new {
            Some(true) => locales_store.get(TK::Creating),
            _ => locales_store.get(TK::Editing),
        };
        match match &resid {
            Some(ResId::Blob(_)) => Some(locales_store.get(TK::BlogPost)),
            Some(ResId::ResKey(_)) => Some(locales_store.get(TK::Resource)),
            None => None,
        } {
            Some(e) => format!("{}: {}:", edit_pref, e),
            None => format!("{}:", edit_pref),
        }
    };
    let onchange = Callback::from(move |e: Event| {
        let element: HtmlInputElement = e.target_unchecked_into();
        is_preview.set(element.checked());
        spawn_local(async move {
            let Some((window, body, ele)) = web_sys::window().and_then(|w| {
                w.document().and_then(|d| {
                    d.body()
                        .and_then(|b| d.document_element().map(|e| (b, e)).map(|(b, e)| (w, b, e)))
                })
            }) else {
                gloo::console::error!("failed to scroll to bottom");
                return;
            };
            ele.set_attribute("style", "scroll-behavior: smooth;")
                .unwrap();
            async_std::task::sleep(std::time::Duration::from_millis(100)).await;
            window.scroll_to_with_x_and_y(0f64, body.scroll_height() as f64);
            ele.set_attribute("style", "scroll-behavior: auto;")
                .unwrap();
        })
    });

    html! {
        <PageBase {title}>
            <Editable resid={ResId::ResKey("editor-intro".to_string())}/>
            <div class={"flex flex-col lg:flex-row gap-4 pb-6 items-center"}>
                <h2 class={"flex font-semibold text-2xl"}>{edit_text}</h2>
                <ResourceSelect resid={resid.clone()} lang={lang} {onselectedchanged} state={Some((*state).clone())}/>
                <div class={"flex flex-row flex-wrap gap-4 lg:w-auto w-full"}>
                    {go_back}
                    {reload}
                    <DiscardButton state={(*state).clone()} onstatechanged={onstatechanged.clone()} resid={resid.clone()} lang={lang}/>
                    <SaveButton state={(*state).clone()} onstatechanged={onstatechanged.clone()} resid={resid.clone()} lang={lang}/>
                    <DeleteButton state={(*state).clone()} {onstatechanged} resid={resid.clone()} lang={lang}/>
                </div>
            </div>
            <div class={"flex flex-col gap-6"}>
                {meta_editor}
                <div class={"flex flex-row gap-4"}>
                    <p>{locales_store.get(TK::Editor)}</p>
                    <input type={"checkbox"} class={"toggle bg-base-content hover:bg-base-content"} {onchange}/>
                    <p>{locales_store.get(TK::Preview)}</p>
                </div>
                {editor}
            </div>
        </PageBase>
    }
}
