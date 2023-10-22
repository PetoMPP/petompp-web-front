use super::atoms::blog_tag::BlogTag;
use crate::{
    api::client::BlobClient,
    components::atoms::date_display::{CreatedDateDisplay, UpdatedDateDisplay},
    router::Route,
};
use petompp_web_models::models::blog_data::BlogMetaData;
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Clone, Properties, PartialEq)]
pub struct BlogSummaryProps {
    pub meta: BlogMetaData,
}

#[function_component(BlogSummary)]
pub fn blog_summary(props: &BlogSummaryProps) -> Html {
    let navigator = use_navigator().unwrap();
    let tags = props
        .meta
        .tags
        .tags()
        .into_iter()
        .map(|tag| html! { <BlogTag {tag}/> });
    let style = "-webkit-mask-image: -webkit-linear-gradient(left, rgba(0,0,0,0),rgba(0,0,0,0.8));";
    let img = match &props.meta.image {
        Some(img) => BlobClient::get_url(format!("image-upload/{}", img).as_str()),
        None => "/img/placeholder.svg".to_string(),
    };
    let onclick = {
        let id = props.meta.id.clone();
        Callback::from(move |_| navigator.push(&Route::BlogPost { id: id.clone() }))
    };

    html! {
    <div class={"card card-side bg-base-200 z-10"} {onclick}>
        <div class={"card-body pt-4"}>
            <div class={"flex flex-col gap-4 lg:gap-2"}>
                <div class={"flex flex-row justify-start"}>
                    <div class={"flex flex-row gap-2"}>
                    {for tags}
                    </div>
                </div>
                <h2 class={"text-2xl my-2 lg:my-0 font-semibold"}>{&props.meta.title}</h2>
                <div class={"flex lg:flex-row flex-col gap-1"}>
                    <CreatedDateDisplay date={props.meta.created} />
                    <UpdatedDateDisplay date={props.meta.updated} />
                </div>
            </div>
                <div class={"flex flex-col"}>
                    <div class={"divider mt-0"}/>
                    <p>{&props.meta.summary}</p>
                </div>
            </div>
        <figure class={"absolute z-0 right-0 h-full w-2/3 object-fill pointer-events-none"} {style}>
            <img class={"rounded-xl min-h-full"} src={img}/>
        </figure>
      </div>
    }
}
