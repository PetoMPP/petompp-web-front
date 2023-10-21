use crate::api::client::ApiClient;
use crate::data::locales::store::LocalesStore;
use crate::{
    components::{atoms::markdown::Editable, organisms::blog::blog_summary::BlogSummary},
    pages::page_base::PageBase,
};
use petompp_web_models::models::tag::Tags;
use yew::platform::spawn_local;
use yew::prelude::*;
use yew_router::prelude::*;
use yewdux::prelude::*;

#[function_component(Blog)]
pub fn blog() -> Html {
    let location = use_location().unwrap();
    let tags = location.query::<Tags>().unwrap_or_default();
    let (locales_store, _) = use_store::<LocalesStore>();
    let posts_int = use_mut_ref(|| None);
    let posts = use_state(|| None);
    use_effect_with_deps(
        move |(posts_int, tags, curr, posts)| {
            let posts_int = posts_int.clone();
            let tags = tags.clone();
            let curr = curr.clone();
            let posts = posts.clone();
            spawn_local(async move {
                let posts_int = posts_int.clone();
                if *posts_int.borrow() == None {
                    *posts_int.borrow_mut() = Some(
                        ApiClient::get_posts_meta()
                            .await
                            .unwrap()
                            .into_iter()
                            .collect::<Vec<_>>(),
                    );
                }
                posts.set(Some(
                    posts_int
                        .borrow()
                        .clone()
                        .unwrap_or_default()
                        .into_iter()
                        .filter(|meta| {
                            tags.is_empty()
                                || tags.tags().iter().any(|t| meta.tags.tags().contains(&t))
                        })
                        .filter(|meta| meta.lang == curr)
                        .collect::<Vec<_>>(),
                ));
            });
        },
        (
            posts_int.clone(),
            tags.clone(),
            locales_store.curr.clone(),
            posts.clone(),
        ),
    );
    let posts = (*posts)
        .clone()
        .unwrap_or_default()
        .into_iter()
        .map(|meta| {
            html! {
                <BlogSummary {meta}/>
            }
        });
    gloo::console::log!("rendering blog", posts.len());
    html! {
        <PageBase>
        <Editable reskey={"blog-intro".to_string()}/>
        <div class={"flex flex-col gap-2"}>
            {for posts}
        </div>
        </PageBase>
    }
}
