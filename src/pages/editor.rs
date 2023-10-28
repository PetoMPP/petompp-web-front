use crate::components::atoms::resource_select::ResourceSelect;
use crate::components::organisms::markdown_editor::MarkdownEditor;
use crate::data::resources::ResourceId;
use crate::data::session::SessionStore;
use crate::handle_api_error;
use crate::pages::page_base::PageBase;
use yew::platform::spawn_local;
use yew::prelude::*;
use yewdux::prelude::*;

#[function_component(Editor)]
pub fn editor() -> Html {
    let location = use_location().unwrap();
    let (resid, lang) = match location
        .query::<ResourceId>()
        .map_err(|h| h.to_string())
        .and_then(|r| r.try_into())
    {
        Ok((resid, lang)) => (Some(resid), Some(lang)),
        Err(_) => (None, None),
    };
    let (_, session_dispatch) = use_store::<SessionStore>();
    let navigator = use_navigator().unwrap();
    let state = use_state_eq(|| None);
    let error_state = use_state_eq(|| None);
    use_effect_with_deps(
        |(resid, lang, state, error_state)| {
            let lang = lang.clone();
            let state = state.clone();
            let error_state = error_state.clone();
            let Some(resid) = resid.clone() else {
                state.set(None);
                return;
            };
            let Some(lang) = lang.clone() else {
                state.set(None);
                return;
            };
            spawn_local(async move {
                match resid.get_value(&lang).await {
                    Ok(new_state) => state.set(Some(((resid, lang), new_state))),
                    Err(e) => {
                        error_state.set(Some(e));
                    }
                }
            });
        },
        (
            resid.clone(),
            lang.clone(),
            state.clone(),
            error_state.clone(),
        ),
    );
    handle_api_error!(error_state, session_dispatch, None);
    let res_lang = resid.as_ref().and_then(|r| lang.as_ref().map(|l| (r, l)));
    let editor = match (state.as_ref(), res_lang) {
        (Some(((data_resid, data_lang), s)), Some((resid, lang)))
            if data_resid == resid && data_lang == lang =>
        {
            html! { <MarkdownEditor state={s.clone()} onmodifiedchanged={Callback::noop()}/> }
        }
        (_, Some(_)) => html! {
            <div class={"w-full flex bordered-lg bg-base-100"}>
                <span class={"flex mx-auto loading loading-ring loading-lg"}/>
            </div>
        },
        (_, None) => html! {
            <div class={"w-full flex bordered-lg bg-base-100"}>
                <p class={"mx-auto py-4 text-xl font-semibold"}>{"Select something to edit!"}</p>
            </div>
        },
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
                <h2 class={"not-prose flex gap-2 items-center"}>{"Now editing:"}<ResourceSelect {resid} {lang} {onselectedchanged}/></h2>
                <p/>
            </div>
            <div class={"flex bg-base-300 rounded-lg p-2"}>
                {editor}
            </div>
        </PageBase>
    }
}
