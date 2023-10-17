use super::atoms::blog_tag::BlogTag;
use crate::{
    components::atoms::date_display::{CreatedDateDisplay, UpdatedDateDisplay},
    models::blog_data::BlogSummaryData,
};
use gloo::dialogs::alert;
use yew::prelude::*;

#[derive(Clone, Properties, PartialEq)]
pub struct BlogSummaryProps {
    pub data: BlogSummaryData,
}

#[function_component(BlogSummary)]
pub fn blog_summary(props: &BlogSummaryProps) -> Html {
    let tags = props
        .data
        .meta
        .tags
        .tags()
        .into_iter()
        .map(|tag| html! { <BlogTag {tag}/> });
    let style = "-webkit-mask-image: -webkit-linear-gradient(left, rgba(0,0,0,0),rgba(0,0,0,0.8));";
    let img = props
        .data
        .meta
        .image
        .clone()
        .unwrap_or("/img/placeholder.svg".to_string());
    let onclick = Callback::from(|_| alert("Clicked!"));

    html! {
    <div class={"card card-side bg-base-200"}>
        <div class={"card-body pt-4"}>
            <div class={"flex flex-col gap-2 z-10"}>
                <div class={"flex flex-row justify-start"}>
                    <div class={"flex flex-row gap-2"}>
                    {for tags}
                    </div>
                </div>
                <h2 class={"hover:underline text-2xl font-semibold"} {onclick}>{&props.data.meta.title}</h2>
                <div class={"flex flex-row gap-1"}>
                    <CreatedDateDisplay date={props.data.meta.created} />
                    <UpdatedDateDisplay date={props.data.meta.updated} />
                </div>
            </div>
                <div class={"flex flex-col"}>
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
