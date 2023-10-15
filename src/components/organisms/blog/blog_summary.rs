use super::atoms::blog_tag::BlogTag;
use yew::prelude::*;

#[derive(Clone, Properties, PartialEq)]
pub struct BlogSummaryProps {
    pub data: crate::models::blog_data::BlogSummaryData,
}

#[function_component(BlogSummary)]
pub fn blog_summary(props: &BlogSummaryProps) -> Html {
    let tags = props
        .data
        .meta
        .tags
        .tags()
        .into_iter()
        .map(|tag| html! { <BlogTag tag={tag.clone()}/> })
        .collect::<Html>();
    let style = "-webkit-mask-image: -webkit-linear-gradient(left, rgba(0,0,0,0),rgba(0,0,0,0.8));";
    let img = props.data.meta.image.clone().unwrap_or("/img/placeholder.svg".to_string());

    html! {
    <div class={"card card-side bg-base-200"}>
        <div class={"card-body"}>
            <div class={"flex flex-row justify-between z-10"}>
                <h2 class={"card-title"}>{&props.data.meta.title}</h2>
                <div class={"flex flex-col gap-1"}>
                    <span>{&props.data.meta.created}</span>
                    <span>{&props.data.meta.updated}</span>
                    <div class={"flex flex-row-reverse gap-2"}>
                        {tags}
                    </div>
                </div>
            </div>
            <div class={"card-actions flex flex-col"}>
                <div class={"divider mt-0"}/>
                <p>{&props.data.summary}</p>
            </div>
            </div>
        <figure class={"absolute right-0 h-full w-2/3 object-fill pointer-events-none"} {style}>
            <img class={"rounded-xl min-h-full"} src={img}/>
        </figure>
      </div>
    }
}
