use crate::{
    components::atoms::{link::HrefLink, markdown::Markdown},
    data::resources::id::ResId,
    pages::{blog::blog_post::BlogPostMeta, page_base::PageBase},
    AppBase, router::route::Route,
};
use petompp_web_models::models::blog_data::BlogMetaData;
use yew::prelude::*;
use yew_router::Routable;

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct MarkdownPreviewProps {
    pub resid: ResId,
    pub markdown: String,
    pub meta: Option<BlogMetaData>,
}

#[function_component(MarkdownPreview)]
pub fn markdown_preview(props: &MarkdownPreviewProps) -> Html {
    let location = web_sys::window().unwrap().location();
    let url = location.protocol().unwrap()
        + "//"
        + location.host().unwrap().as_str()
        + match &props.resid {
            ResId::Blob(id) => (Route::BlogPost { id: id.clone() }).to_path(),
            ResId::ResKey(id) => "/".to_string() + id.trim_end_matches("-content"),
        }
        .as_str();
    let meta = match &props.resid {
        ResId::Blob(_) => Some(html! {
            <>
            <BlogPostMeta meta={props.meta.clone().unwrap_or_default()} />
            <div class={"divider"}/>
            </>
        }),
        _ => None,
    };
    html! {
        <div class={"mockup-browser border border-2 border-base-300 shadow-2xl"}>
            <div class={"mockup-browser-toolbar"}>
                <div class={"input border border-base-300 grow cursor-pointer"}><HrefLink href={url}/></div>
            </div>
            <div class={"border-t border-base-300"}>
                <AppBase preview={true}>
                <PageBase animatenone={Some(())}>
                    {meta}
                    <Markdown markdown={props.markdown.clone()} allowhtml={true}/>
                </PageBase>
                </AppBase>
            </div>
        </div>
    }
}
