use crate::api::client::ApiClient;
use crate::components::atoms::collapse::Collapse;
use crate::components::atoms::loading::Loading;
use crate::components::atoms::resource_select::ResourceSelect;
use crate::components::organisms::blog::blog_meta_editor::BlogMetaEditor;
use crate::components::organisms::markdown_editor::MarkdownEditor;
use crate::components::organisms::markdown_preview::MarkdownPreview;
use crate::components::state::State;
use crate::data::resources::id::{ResId, ResourceId};
use crate::data::session::SessionStore;
use crate::pages::page_base::PageBase;
use crate::router::route::Route;
use crate::utils::style::get_svg_bg_mask_style;
use petompp_web_models::models::country::Country;
use web_sys::HtmlInputElement;
use yew::platform::spawn_local;
use yew::prelude::*;
use yew_router::prelude::*;
use yewdux::prelude::*;

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
    let navigator = use_navigator().unwrap();
    let state = use_state_eq(|| State::Ok(None));
    let is_preview = use_state_eq(|| false);
    use_effect_with_deps(
        |(resid, lang, state)| {
            let Some(resid) = resid.clone() else {
                state.set(State::Ok(None));
                return;
            };
            let Some(lang) = lang.clone() else {
                state.set(State::Ok(None));
                return;
            };
            let state = state.clone();
            match &*state {
                State::Ok(Some(((r, l), _))) if r == &resid && l == &lang => return,
                State::Loading | State::Err(_) => return,
                _ => state.set(State::Loading),
            };
            spawn_local(async move {
                let value = match resid.get_value(&lang).await {
                    Ok(state) => state,
                    Err(e) => {
                        state.set(State::Err(e));
                        return;
                    }
                };
                let meta = match &resid {
                    ResId::Blob(p) => match ApiClient::get_post_meta(&p, lang.key()).await {
                        Ok(meta) => Some(meta),
                        Err(e) => {
                            state.set(State::Err(e));
                            return;
                        }
                    },
                    _ => None,
                };
                state.set(State::Ok(Some(((resid, lang), (value, meta)))));
            })
        },
        (resid.clone(), lang.clone(), state.clone()),
    );
    let editor = match &*state {
        State::Ok(Some(((id, _), (s, m)))) => match &*is_preview {
            true => {
                html! {<MarkdownPreview resid={id.clone()} markdown={s.clone()} meta={m.clone()} />}
            }
            false => {
                html! { <MarkdownEditor state={s.clone()} onmodifiedchanged={Callback::noop()}/> }
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
    let onselectedchanged = {
        let navigator = navigator.clone();
        Callback::from(move |resource_id| {
            navigator
                .push_with_query(&Route::Editor, &resource_id)
                .unwrap()
        })
    };
    let meta_editor = match &resid {
        Some(ResId::Blob(_)) => match &*state {
            State::Ok(Some(((_, _), (_, Some(meta))))) => Some(html! {
                <Collapse label={"Blog Post Metadata"}>
                    <BlogMetaEditor data={meta.clone()} ondatachanged={Callback::noop()} />
                </Collapse>
            }),
            State::Loading => {
                Some(html! { <Loading resource={"blog post metadata".to_string()} /> })
            }
            _ => None,
        },
        _ => None,
    };
    let edit_text = match &resid {
        Some(ResId::Blob(_)) => "Blog post:",
        Some(ResId::ResKey(_)) => "Resource:",
        None => "Nothing selected:",
    };
    let onchange = Callback::from(move |e: Event| {
        let element: HtmlInputElement = e.target_unchecked_into();
        is_preview.set(element.checked());
    });

    html! {
        <PageBase>
            <div class={"prose"}>
                <h1>{"Editor"}</h1>
                <p>{"This is the editor page. Here you can edit the content of the page selected."}</p>
                <h2 class={"not-prose flex flex-col lg:flex-row gap-2 items-center"}>{edit_text}<ResourceSelect {resid} {lang} {onselectedchanged}/>{reload}</h2>
                <p/>
            </div>
            <div class={"flex flex-col gap-6"}>
                {meta_editor}
                <div id={"swap"} class={"flex flex-row gap-2"}>
                    <p>{"Editor"}</p>
                    <input type={"checkbox"} class={"toggle bg-opacity-100"} {onchange}/>
                    <p>{"Preview"}</p>
                </div>
                <div class={"border rounded-lg p-2 shadow-lg"}>
                    {editor}
                </div>
            </div>
        </PageBase>
    }
}
