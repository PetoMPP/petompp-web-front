use crate::api::client::{ApiClient, BlobClient};
use crate::components::atoms::flag::FlagSelect;
use crate::components::organisms::markdown_editor::MarkdownEditor;
use crate::data::locales::store::LocalesStore;
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
    let res_id: Result<ResId, _> = location
        .query::<ResourceId>()
        .map_err(|h| h.to_string())
        .and_then(|r| r.try_into());
    let (_, session_dispatch) = use_store::<SessionStore>();
    let (locales_store, _) = use_store::<LocalesStore>();
    let state = use_state_eq(|| None);
    let error_state = use_state_eq(|| None);
    use_effect_with_deps(
        |(res_id, state, error_state)| {
            let Ok(res_id) = res_id.clone() else {
                return;
            };
            let state = state.clone();
            let error_state = error_state.clone();
            if error_state.is_some() {
                return;
            }
            match res_id {
                ResId::ResKey((reskey, lang)) => spawn_local(async move {
                    match ApiClient::get_resource(reskey.as_str(), lang.key()).await {
                        Ok(new_state) => state.set(Some(new_state)),
                        Err(e) => {
                            error_state.set(Some(e));
                        }
                    }
                }),
                ResId::Blob((path, lang)) => spawn_local(async move {
                    match BlobClient::get_post_content(
                        format!("{}/{}.md", path, lang.key()).as_str(),
                    )
                    .await
                    {
                        Ok(new_state) => state.set(Some(new_state)),
                        Err(e) => {
                            error_state.set(Some(e));
                        }
                    }
                }),
            }
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
    html! {
        <PageBase>
            <div class={"prose"}>
                <h1>{"Editor"}</h1>
                <p>{"This is the editor page. Here you can edit the content of the page selected."}</p>
                <h3>{"Now editing:"}<a class={"btn btn-sm m-1 p-1"}>{res_id.map(|r| format!("{:?}", r)).unwrap_or("-".to_string())}</a></h3>
                <h3 class={"not-prose flex gap-2 align-center"}>{"In lang:"}<FlagSelect country={locales_store.curr} /></h3>
                <p/>
            </div>
            <div class={"flex bg-base-300 rounded-lg p-2"}>
                {editor}
            </div>
        </PageBase>
    }
}
