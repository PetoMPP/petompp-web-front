use crate::api::client::ApiClient;
use crate::models::tag::Tags;
use crate::use_effect_deps;
use crate::{
    components::{atoms::markdown::Editable, organisms::blog::blog_summary::BlogSummary},
    pages::page_base::PageBase,
};
use yew::platform::spawn_local;
use yew::prelude::*;
use yew_router::prelude::*;

#[function_component(Blog)]
pub fn blog() -> Html {
    let location = use_location().unwrap();
    let tags = location.query::<Tags>();
    let posts = use_state(|| vec![]);
    use_effect_deps!(|posts| {
        spawn_local(async move {
            let ps = ApiClient::get_posts_meta().await.unwrap();
            posts.set(ps);
        })
    });
    let posts = (*posts).clone();
    let posts = match tags {
        Ok(tags) => posts
            .into_iter()
            .filter(|meta| tags.tags().iter().any(|t| meta.tags.tags().contains(&t)))
            .collect::<Vec<_>>(),
        Err(_) => posts.into_iter().collect::<Vec<_>>(),
    };
    let posts = posts
        .into_iter()
        .map(|meta| {
            html! {
                <BlogSummary {meta}/>
            }
        })
        .collect::<Html>();
    html! {
        <PageBase>
        <Editable reskey={"blog-intro".to_string()}/>
        <div class={"flex flex-col gap-2"}>
            {posts}
        </div>
        </PageBase>
    }
}
