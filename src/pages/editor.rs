use crate::{
    api::client::{ApiClient, RequestError},
    components::{
        atoms::{collapse::Collapse, loading::Loading, resource_select::ResourceSelect},
        organisms::{
            blog::blog_meta_editor::BlogMetaEditor,
            editor::atoms::{delete_button::DeleteButton, save_button::SaveButton},
            markdown_editor::MarkdownEditor,
            markdown_preview::MarkdownPreview,
        },
        state::State,
    },
    data::{
        resources::{
            id::{ResId, ResourceId},
            store::LocalStore,
        },
        session::SessionStore,
    },
    pages::page_base::PageBase,
    router::route::Route,
    utils::style::get_svg_bg_mask_style,
};
use petompp_web_models::models::blog_data::BlogMetaData;
use petompp_web_models::models::country::Country;
use web_sys::HtmlInputElement;
use yew::platform::spawn_local;
use yew::prelude::*;
use yew_router::prelude::*;
use yewdux::prelude::*;

pub type EditorState = State<
    Option<(
        Option<bool>,
        (ResId, Country),
        (String, Option<BlogMetaData>),
    )>,
    RequestError,
>;

#[function_component(Editor)]
pub fn editor() -> Html {
    let location = use_location().unwrap();
    let (resid, lang) = location
        .query::<ResourceId>()
        .ok()
        .and_then(|r| TryInto::<(ResId, Country)>::try_into(r).ok())
        .and_then(|(r, l)| Some((Some(r), Some(l))))
        .unwrap_or_default();
    let (_, session_dispatch) = use_store::<SessionStore>();
    let (local_store, local_dispatch) = use_store::<LocalStore>();
    let navigator = use_navigator().unwrap();
    let state = use_state_eq(|| EditorState::Ok(None));
    let is_preview = use_state_eq(|| false);
    use_effect_with_deps(
        move |(resid, lang, state, local_store)| {
            let Some(resid) = resid.clone() else {
                state.set(State::Ok(None));
                return;
            };
            let Some(lang) = lang.clone() else {
                state.set(State::Ok(None));
                return;
            };
            let state = state.clone();
            let local_store = local_store.clone();
            let cached_state = local_store.get(&resid, lang.key()).cloned();
            let any_cached_state = cached_state.is_some() || local_store.exists(&resid);
            match &*state {
                State::Ok(Some((n, (r, l), value))) if r == &resid && l == &lang => {
                    if let Some(local_value) = &cached_state {
                        if value == local_value {
                            return;
                        }
                        // we already know that the resource exists or not, no need to call api
                        if n.is_some() {
                            state.set(State::Ok(Some((*n, (resid, lang), local_value.clone()))));
                            return;
                        }
                    } else {
                        return;
                    }
                }
                State::Loading | State::Err(_) => return,
                _ => state.set(State::Loading),
            };
            spawn_local(async move {
                // does current resource exist?
                let (is_new, new_state) = match &resid {
                    ResId::Blob(p) => match ApiClient::get_post_meta(&p, lang.key()).await {
                        Ok(m) => match resid.get_value(&lang).await {
                            Ok(v) => (false, Some((v, Some(m)))),
                            Err(e) => {
                                state.set(State::Err(e));
                                return;
                            }
                        },
                        Err(e) => match e {
                            RequestError::Endpoint(404, _) => {
                                // does it exist in another language?
                                match ApiClient::get_posts_meta(Some(p.to_string())).await {
                                    Ok(_) => (true, None),
                                    Err(e) => match e {
                                        RequestError::Endpoint(404, _) => {
                                            // does it exist locally?
                                            match cached_state.is_some() || any_cached_state {
                                                true => (true, None),
                                                false => {
                                                    state.set(State::Err(e));
                                                    return;
                                                }
                                            }
                                        }
                                        _ => {
                                            state.set(State::Err(e));
                                            return;
                                        }
                                    },
                                }
                            }
                            _ => {
                                state.set(State::Err(e));
                                return;
                            }
                        },
                    },
                    ResId::ResKey(k) => match ApiClient::get_resource(k.as_str(), &lang).await {
                        Ok((c, v)) => match c == lang {
                            true => (false, Some((v, None))),
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
                        state.set(State::Ok(Some((Some(is_new), (resid, lang), cs.clone()))));
                    }
                    (Some(ns), None) => {
                        state.set(State::Ok(Some((Some(is_new), (resid, lang), ns))));
                    }
                    (None, Some(cs)) => {
                        state.set(State::Ok(Some((Some(is_new), (resid, lang), cs.clone()))));
                    }
                    (None, None) => {
                        let meta = match &resid {
                            ResId::Blob(_) => Some(BlogMetaData::default()),
                            _ => None,
                        };
                        state.set(State::Ok(Some((
                            Some(is_new),
                            (resid, lang),
                            ("".to_string(), meta),
                        ))));
                    }
                }
            })
        },
        (
            resid.clone(),
            lang.clone(),
            state.clone(),
            local_store.clone(),
        ),
    );
    if let State::Err(e) = &*state {
        if let Err(redirect) = e.handle_failed_auth(session_dispatch.clone()) {
            return redirect;
        }
    }
    let editor = match &*state {
        State::Ok(Some((_, (resid, lang), (value, meta)))) => match &*is_preview {
            true => {
                html! {<MarkdownPreview resid={resid.clone()} markdown={value.clone()} meta={meta.clone()} />}
            }
            false => {
                let onchanged = {
                    let local_dispatch = local_dispatch.clone();
                    let resid = resid.clone();
                    let lang = lang.clone();
                    let meta = meta.clone();
                    Callback::from(move |data: String| {
                        local_dispatch.reduce_mut(|store| {
                            if let Some((value, _)) = store.get_mut(&resid, lang.key()) {
                                *value = data.clone();
                            } else {
                                store.insert(resid.clone(), lang.key(), data.clone(), meta.clone());
                            }
                        });
                    })
                };
                html! {
                <div class={"border border-2 border-base-300 rounded-2xl p-2 shadow-2xl"}>
                    <MarkdownEditor state={value.clone()} {onchanged}/>
                </div>
                }
            }
        },
        State::Ok(None) => html! {
            <div class={"w-full flex rounded-lg bg-base-100"}>
                <p class={"mx-auto py-4 text-xl font-semibold"}>{"Select something to edit!"}</p>
            </div>
        },
        State::Loading => html! {
            <div class={"w-full flex rounded-lg bg-base-100"}>
                <span class={"flex mx-auto py-4 loading loading-ring loading-lg"}/>
            </div>
        },
        State::Err(e) => {
            if let Err(redirect) = e.handle_failed_auth(session_dispatch) {
                return redirect;
            }
            html! {
                <div class={"w-full flex flex-col gap-4 rounded-lg bg-error"}>
                    <p class={"mx-auto py-4 text-error-content text-xl font-semibold"}>{"Something went wrong!"}</p>
                    <p class={"mx-auto py-4 text-error-content"}>{e.to_string()}</p>
                </div>
            }
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
    let clear_local = match (&resid, &lang) {
        (Some(resid), Some(lang)) => match local_store.get(&resid, lang.key()) {
            Some(_) => {
                let navigator = navigator.clone();
                let local_dispatch = local_dispatch.clone();
                let resid = resid.clone();
                let lang = lang.clone();
                Some(html! {
                    <button class={"btn btn-warning grow"} onclick={Callback::from(move |_| {
                        local_dispatch.reduce_mut(|store| {
                            store.remove(&resid, lang.key());
                        });
                        navigator.push(&Route::Editor);
                    })}>
                    {"Discard"}
                    </button>
                })
            }
            _ => None,
        },
        _ => None,
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
    let meta_editor = match &resid {
        Some(ResId::Blob(_)) => match &*state {
            State::Ok(Some((_, (resid, lang), (value, Some(meta))))) => Some({
                let local_dispatch = local_dispatch.clone();
                let resid = resid.clone();
                let lang = lang.clone();
                let value = value.clone();
                let ondatachanged = Callback::from(move |data: BlogMetaData| {
                    local_dispatch.reduce_mut(|store| {
                        if let Some((_, meta)) = store.get_mut(&resid.clone(), lang.key()) {
                            *meta = Some(data.clone());
                        } else {
                            store.insert(
                                resid.clone(),
                                lang.key(),
                                value.clone(),
                                Some(data.clone()),
                            );
                        }
                    });
                });
                html! {
                    <Collapse label={"Blog Post Metadata"}>
                        <BlogMetaEditor data={meta.clone()} {ondatachanged} />
                    </Collapse>
                }
            }),
            State::Loading => {
                Some(html! { <Loading resource={"blog post metadata".to_string()} /> })
            }
            _ => None,
        },
        _ => None,
    };
    let is_new = match &*state {
        State::Ok(Some((n, _, _))) => n.clone(),
        _ => None,
    };
    let edit_text = {
        let edit_pref = match is_new {
            Some(true) => "Creating:",
            _ => "Editing:",
        };
        let edit_text = match &resid {
            Some(ResId::Blob(_)) => "Blog post:",
            Some(ResId::ResKey(_)) => "Resource:",
            None => "Nothing selected:",
        };
        format!("{} {}", edit_pref, edit_text)
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
    let delete_button = match (is_new, &resid, &lang) {
        (Some(false), Some(resid), Some(lang)) => Some(html! {
            <DeleteButton resid={resid.clone()} lang={lang.clone()}/>
        }),
        _ => None,
    };

    html! {
        <PageBase>
            <div class={"prose"}>
                <h1>{"Editor"}</h1>
                <p>{"This is the editor page. Here you can edit the content of the page selected."}</p>
                <p/>
                </div>
            <div class={"flex flex-col lg:flex-row gap-4 pb-6 items-center"}>
                <h2 class={"flex font-semibold text-2xl"}>{edit_text}</h2>
                <ResourceSelect resid={resid.clone()} lang={lang.clone()} {onselectedchanged} state={Some((*state).clone())}/>
                {go_back}
                {reload}
                <div class={"flex flex-row gap-4 lg:w-auto w-full"}>
                    {clear_local}
                    <SaveButton state={(*state).clone()} {onstatechanged} resid={resid.clone()} lang={lang.clone()}/>
                    {delete_button}
                </div>
            </div>
            <div class={"flex flex-col gap-6"}>
                {meta_editor}
                <div id={"swap"} class={"flex flex-row gap-4"}>
                    <p>{"Editor"}</p>
                    <input type={"checkbox"} class={"toggle bg-opacity-100"} {onchange}/>
                    <p>{"Preview"}</p>
                </div>
                {editor}
            </div>
        </PageBase>
    }
}
