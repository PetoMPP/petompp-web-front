use crate::{
    components::atoms::markdown::Markdown,
    data::resources::id::ResId,
    pages::{blog::blog_post::BlogPostMeta, page_base::PageBase},
    router::{blog::BlogRoute, route::Route},
    AppBase,
};
use petompp_web_models::models::blog_data::BlogMetaData;
use yew::prelude::*;
use yew_router::{prelude::use_location, Routable};

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct MarkdownPreviewProps {
    pub resid: ResId,
    pub markdown: String,
    pub meta: Option<BlogMetaData>,
}

#[function_component(MarkdownPreview)]
pub fn markdown_preview(props: &MarkdownPreviewProps) -> Html {
    let location = web_sys::window().unwrap().location();
    let url = location.protocol().unwrap() + "//" + location.host().unwrap().as_str();
    let url = url
        + match &props.resid {
            ResId::Blob(id) => (BlogRoute::Post { id: id.clone() }).to_path(),
            ResId::ResKey(id) => id.clone(),
        }
        .as_str();
    let preview = match &props.resid {
        ResId::Blob(_) => html! {
            <>
            <BlogPostMeta meta={props.meta.clone().unwrap_or_default()} />
            <div class={"divider"}/>
            <div class={"mx-auto prose flex flex-col w-full"}>
                <Markdown markdown={props.markdown.clone()} allowhtml={true}/>
            </div>
            </>
        },
        _ => html! {
                <Markdown markdown={props.markdown.clone()} allowhtml={true}/>
        },
    };
    html! {
        <div class={"mockup-browser border border-2 border-base-300"}>
        <div class={"mockup-browser-toolbar"}>
          <div class={"input border border-base-300"}>{url}</div>
        </div>
        <div class={"border-t border-base-300"}>
        <AppBase preview={true}>
        <PageBase animatenone={Some(())}>
            {preview}
        </PageBase>
        </AppBase>
        </div>
      </div>
    }
}
