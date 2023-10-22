use crate::{
    api::client::{ApiClient, BlobClient},
    components::atoms::markdown::Markdown,
    data::session::SessionStore,
    handle_api_error,
    pages::page_base::PageBase,
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
    let error_state = use_state_eq(|| None);
    let blog_data = use_state(|| None);
    use_effect_with_deps(
        |(props, error_state, blog_data)| {
            let props = props.clone();
            let error_state = error_state.clone();
            let blog_data = blog_data.clone();
            if error_state.is_some() || blog_data.is_some() {
                return;
            }
            spawn_local(async move {
                let meta = match ApiClient::get_post_meta(props.id.as_str()).await {
                    Ok(meta) => meta,
                    Err(e) => {
                        error_state.set(Some(e));
                        return;
                    }
                };
                let filename = meta.filename(&FilenameService::default());
                let md = match BlobClient::get_post_content(filename.as_str()).await {
                    Ok(content) => content,
                    Err(e) => {
                        error_state.set(Some(e));
                        return;
                    }
                };
                blog_data.set(Some((meta, md)));
            });
        },
        (props.clone(), error_state.clone(), blog_data.clone()),
    );
    handle_api_error!(error_state, session_dispatch, false);
    let onclick = Callback::from(move |_| navigator.push(&Route::Blog));
    let markdown = blog_data
        .as_ref()
        .map(|(_, md)| md.clone())
        .unwrap_or_default();
    let meta = blog_data.as_ref().map(|(m, _)| {
        let img = m.image.as_ref().map(|i| BlobClient::get_url(format!("image-upload/{}", i).as_str())).unwrap_or("/img/placeholder.svg".to_string());
        html! {
            <div class={"hero mb-4 md:pt-36 pt-16 rounded-lg p-2"} style={format!("background-image: url({}); -webkit-mask-image: -webkit-linear-gradient(top, rgba(0,0,0,0),rgba(0,0,0,0.8));", img)}>
                <div class={"prose text-neutral text-center max-w-md"}>
                    <h1 class={"text-neutral"}>{&m.title}</h1>
                    <p>{&m.summary}</p>
                </div>
            </div>
        }}
    );

    html! {
        <PageBase>
            <a class={"lg:mb-6 mb-4"} href={"javascript:void(0);"} {onclick}>{"\u{021A9} back to posts.."}</a>
            {meta}
            <div class={"divider"}/>
            <div class={"mx-auto prose flex flex-col w-full"}>
                <Markdown {markdown} allowhtml={true} interactive={Some(())}/>
            </div>
        </PageBase>
    }
}
