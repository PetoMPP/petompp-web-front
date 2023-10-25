use crate::components::atoms::flag::FlagSelect;
use crate::components::organisms::markdown_editor::MarkdownEditor;
use crate::data::resources::{ResId, ResourceId};
use crate::data::session::SessionStore;
use crate::handle_api_error;
use crate::pages::page_base::PageBase;
use yew::platform::spawn_local;
use yew::prelude::*;
use yewdux::prelude::*;

#[function_component(Editor)]
pub fn editor() -> Html {
    let location = use_location().unwrap();
    let res_id: ResId = location
        .query::<ResourceId>()
        .map_err(|h| h.to_string())
        .and_then(|r| r.try_into())
        .unwrap();
    let (_, session_dispatch) = use_store::<SessionStore>();
    let navigator = use_navigator().unwrap();
    let state = use_state_eq(|| None);
    let error_state = use_state_eq(|| None);
    use_effect_with_deps(
        |(res_id, state, error_state)| {
            let res_id = res_id.clone();
            let state = state.clone();
            let error_state = error_state.clone();
            if error_state.is_some() {
                return;
            }
            spawn_local(async move {
                match res_id.get_value().await {
                    Ok(new_state) => state.set(Some(new_state)),
                    Err(e) => {
                        error_state.set(Some(e));
                    }
                }
            });
        },
        (res_id.clone(), state.clone(), error_state.clone()),
    );
    handle_api_error!(error_state, session_dispatch, None);
    let editor = state
        .as_ref()
        .map(|s| {
            html! {
                <MarkdownEditor state={s.clone()} onmodifiedchanged={Callback::noop()}/>
            }
        })
        .unwrap_or(html! {
            <span class={"flex mx-auto loading loading-ring loading-lg"}/>
        });
    let onselectedchanged = {
        let res_id = res_id.clone();
        let navigator = navigator.clone();
        Callback::from(move |c| {
            let navigator = navigator.clone();
            navigator
                .push_with_query(
                    &Route::Editor,
                    &ResourceId::from(res_id.clone().with_lang(c)),
                )
                .unwrap()
        })
    };
    html! {
        <PageBase>
            <div class={"prose"}>
                <h1>{"Editor"}</h1>
                <p>{"This is the editor page. Here you can edit the content of the page selected."}</p>
                <h2>{"Now editing:"}<a class={"btn btn-md m-1 p-1"}>{format!("{:?}", res_id)}</a></h2>
                <h2 class={"not-prose flex gap-2 items-center"}>{"In lang:"}<FlagSelect country={res_id.lang().clone()} {onselectedchanged} /></h2>
                <p/>
            </div>
            <div class={"flex bg-secondary rounded-lg p-2"}>
                {editor}
            </div>
        </PageBase>
    }
}
