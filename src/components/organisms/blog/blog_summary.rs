use super::atoms::blog_tag::BlogTag;
use crate::{
    api::{blob::BlobClient, client::ApiClient},
    components::atoms::date_display::{CreatedDateDisplay, UpdatedDateDisplay},
    router::route::Route,
};
use petompp_web_models::models::blob::blog::BlogMetaData;
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Clone, Properties, PartialEq)]
pub struct BlogSummaryProps {
    pub meta: BlogMetaData,
}

#[function_component(BlogSummary)]
pub fn blog_summary(props: &BlogSummaryProps) -> Html {
    let tags = props
        .meta
        .tags
        .tags()
        .into_iter()
        .map(|tag| html! { <BlogTag {tag}/> });
    let style = "-webkit-mask-image: -webkit-linear-gradient(left, rgba(0,0,0,0),rgba(0,0,0,0.8));";
    let img = match props.meta.image().as_str() {
        "" => "/img/placeholder.svg".to_string(),
        img => <ApiClient as BlobClient>::get_url("image-upload", img),
    };
    let id = props.meta.id().to_string();

    html! {
        <Link<Route> classes={"card card-side bg-base-200 cursor-pointer"} to={Route::BlogPost { id: id.to_string() }}>
            <div class={"card-body pt-4 z-10"}>
                <div class={"flex flex-col gap-4 lg:gap-2"}>
                    <div class={"flex flex-row justify-start"}>
                        <div class={"flex flex-row gap-2 flex-wrap"}>
                        {for tags}
                        </div>
                    </div>
                    <h2 class={"text-2xl my-2 lg:my-0 font-semibold"}>{&props.meta.title()}</h2>
                    <div class={"flex lg:flex-row flex-col gap-1 cursor-default"}>
                        <CreatedDateDisplay date={*props.meta.created} />
                        <UpdatedDateDisplay date={*props.meta.updated} />
                    </div>
                </div>
                    <div class={"flex flex-col"}>
                        <div class={"divider mt-0"}/>
                        <p>{&props.meta.summary()}</p>
                    </div>
                </div>
            <figure class={"absolute z-0 right-0 h-full w-2/3 object-fill pointer-events-none"} {style}>
                <img class={"rounded-xl min-h-full"} src={img}/>
            </figure>
        </Link<Route>>
    }
}
