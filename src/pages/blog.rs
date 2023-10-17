use crate::models::blog_data::{BlogMetaData, BlogSummaryData};
use crate::models::tag::{Tag, Tags};
use crate::{
    components::{atoms::markdown::Editable, organisms::blog::blog_summary::BlogSummary},
    pages::page_base::PageBase,
};
use chrono::Utc;
use rand::Rng;
use yew::prelude::*;
use yew_router::prelude::*;

#[function_component(Blog)]
pub fn blog() -> Html {
    let location = use_location().unwrap();
    let tags = location.query::<Tags>();
    let posts = get_posts(4);
    let posts = match tags {
        Ok(tags) => posts
            .iter()
            .filter(|summary| {
                tags.tags()
                    .iter()
                    .any(|t| summary.meta.tags.tags().contains(&t))
            })
            .collect::<Vec<_>>(),
        Err(_) => posts.iter().collect::<Vec<_>>(),
    };
    let posts = posts
        .iter()
        .map(|summary| {
            html! {
                <BlogSummary data={(**summary).clone()}/>
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

fn get_posts(count: usize) -> Vec<BlogSummaryData> {
    let mut rand = rand::thread_rng();
    (1..count + 1)
        .into_iter()
        .map(|n| {
            BlogSummaryData::from_meta(
                BlogMetaData::new(
                    format!("Post {}", n),
                    Tags::from(vec![
                        Tag {
                            tag: format!("tag{}", n),
                        },
                        Tag {
                            tag: format!("tag{}", n + 1),
                        },
                    ]),
                    Utc::now()
                        .checked_sub_signed(
                            chrono::Duration::days(rand.gen_range(0..n as i64))
                                + chrono::Duration::hours(rand.gen_range(0..23 as i64))
                                + chrono::Duration::minutes(rand.gen_range(0..59 as i64))
                                + chrono::Duration::seconds(rand.gen_range(0..59 as i64)),
                        )
                        .unwrap(),
                ),
                format!(
                    "This is a post in the blog. It is about tag{} and tag{}.",
                    n,
                    n + 1
                ),
            )
        })
        .collect::<Vec<_>>()
}
