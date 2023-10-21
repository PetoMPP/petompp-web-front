use crate::{
    api::client::{ApiClient, BlobClient},
    components::atoms::markdown::Markdown,
    data::session::SessionStore,
    handle_api_error,
    pages::page_base::PageBase,
    use_effect_deps,
};
use petompp_web_models::services::filename::FilenameService;
use yew::{platform::spawn_local, prelude::*};
use yewdux::prelude::*;

#[derive(Clone, PartialEq, Properties)]
pub struct BlogPostProps {
    pub id: String,
}

#[function_component(BlogPost)]
pub fn blog_post(props: &BlogPostProps) -> Html {
    let navigator = use_navigator().unwrap();
    let (_, session_dispatch) = use_store::<SessionStore>();
    let loaded = use_state_eq(|| false);
    let error_state = use_state_eq(|| None);
    let prelude = use_mut_ref(|| String::new());
    let md = use_mut_ref(|| String::new());
    use_effect_deps!(|md, props, prelude, error_state| {
        if *loaded {
            return;
        }
        spawn_local(async move {
            let meta = match ApiClient::get_post_meta(props.id.as_str()).await {
                Ok(meta) => {
                    *prelude.borrow_mut() = format!("# {}\n\n*{}*", meta.title, meta.summary);
                    meta
                }
                Err(e) => {
                    error_state.set(Some(e));
                    return;
                }
            };
            let filename_service = FilenameService::default();

            match BlobClient::get_post_content(meta.filename(&filename_service).as_str()).await {
                Ok(content) => {
                    *md.borrow_mut() = content;
                    loaded.set(true);
                }
                Err(e) => error_state.set(Some(e)),
            }
        })
    });
    handle_api_error!(error_state, session_dispatch, false);
    let onclick = Callback::from(move |_| navigator.push(&Route::Blog));

    html! {
        <PageBase>
            <div class={"prose"}>
                <a href={"javascript:void(0);"} {onclick}>{"<-- back to posts"}</a>
                <Markdown markdown={prelude.borrow().clone()} allowhtml={true} interactive={Some(())}/>
                <Markdown markdown={md.borrow().clone()} allowhtml={true} interactive={Some(())}/>
            </div>
        </PageBase>
    }
}
