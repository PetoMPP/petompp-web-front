use crate::components::atoms::resource_select::ResourceSelect;
use crate::components::organisms::markdown_editor::MarkdownEditor;
use crate::components::state::State;
use crate::data::resources::{ResId, ResourceId};
use crate::data::session::SessionStore;
use crate::pages::page_base::PageBase;
use crate::router::route::Route;
use crate::utils::style::get_svg_bg_mask_style;
use petompp_web_models::models::country::Country;
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
                state.set(match resid.get_value(&lang).await {
                    Ok(state) => State::Ok(Some(((resid, lang), state))),
                    Err(e) => State::Err(e),
                })
            });
        },
        (resid.clone(), lang.clone(), state.clone()),
    );
    let editor = match &*state {
        State::Ok(Some(((_, _), s))) => {
            html! { <MarkdownEditor state={s.clone()} onmodifiedchanged={Callback::noop()}/> }
        }
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
        State::Err(_) => Some(html! {
            <button class={"btn btn-square"} onclick={Callback::from(move |_| {
                state.set(State::Ok(None));
            })}>
                <div class={"bg-base-content h-10 w-10"} style={get_svg_bg_mask_style("/img/ui/reload.svg")}/>
            </button>
        }),
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

    html! {
        <PageBase>
            <div class={"prose"}>
                <h1>{"Editor"}</h1>
                <p>{"This is the editor page. Here you can edit the content of the page selected."}</p>
                <h2 class={"not-prose flex gap-2 items-center"}>{"Now editing:"}<ResourceSelect {resid} {lang} {onselectedchanged}/>{reload}</h2>
                <p/>
            </div>
            <div class={"flex bg-base-300 rounded-lg p-2"}>
                {editor}
            </div>
        </PageBase>
    }
}
