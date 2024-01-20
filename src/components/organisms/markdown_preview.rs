use crate::{
    components::atoms::{link::HrefLink, markdown::Markdown},
    data::resources::id::{BlobType, ResId},
    pages::{
        blog_post::BlogPostMeta,
        editor::{EditorData, EditorDataState},
        page_base::PageBase,
    },
    router::route::Route,
    AppBase,
};
use yew::prelude::*;
use yew_router::Routable;

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct MarkdownPreviewProps {
    pub data: EditorDataState,
}

#[function_component(MarkdownPreview)]
pub fn markdown_preview(props: &MarkdownPreviewProps) -> Html {
    let location = web_sys::window().unwrap().location();
    let url = location.protocol().unwrap()
        + "//"
        + location.host().unwrap().as_str()
        + match &props.data.id.0 {
            ResId::Blob(id) => match &id {
                BlobType::Blog(s) => (Route::BlogPost { id: id.to_string() }).to_path(),
                BlobType::Project(s) => todo!(),
            },
            ResId::ResKey(id) => "/".to_string() + id.trim_end_matches("-content"),
        }
        .as_str();
    let (meta, markdown) = match &props.data.data {
        EditorData::Blog((markdown, meta)) => (
            Some(html! {
                <>
                <BlogPostMeta meta={meta.clone()} />
                <div class={"divider"}/>
                </>
            }),
            markdown.clone(),
        ),
        EditorData::Project((markdown, _)) | EditorData::Resource(markdown) => {
            (None, markdown.clone())
        }
    };
    html! {
        <div class={"mockup-browser border border-2 border-base-300 shadow-2xl"}>
            <div class={"mockup-browser-toolbar"}>
                <div class={"input border border-base-300 grow cursor-pointer"}><HrefLink href={url}/></div>
            </div>
            <div class={"border-t border-base-300"}>
                <AppBase preview={true}>
                <PageBase mockup={Some(())} title={String::new()}>
                    {meta}
                    <Markdown {markdown} allowhtml={true}/>
                </PageBase>
                </AppBase>
            </div>
        </div>
    }
}
