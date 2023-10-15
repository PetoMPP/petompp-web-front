use crate::models::blog_data::{BlogMetaData, BlogSummaryData};
use crate::models::tag::{Tag, Tags};
use crate::{
    components::{atoms::markdown::Editable, organisms::blog::blog_summary::BlogSummary},
    pages::page_base::PageBase,
};
use yew::prelude::*;
use yew_router::prelude::*;

#[function_component(Blog)]
pub fn blog() -> Html {
    let location = use_location().unwrap();
    let tags = location.query::<Tags>();
    let posts = vec![
        BlogSummaryData::from_meta(
            BlogMetaData::new(
                "Post 1",
                Tags::from(vec![
                    Tag {
                        tag: "tag1".to_string(),
                    },
                    Tag {
                        tag: "tag2".to_string(),
                    },
                ]),
                chrono::Utc::now().to_rfc3339(),
            ),
            "This is the first post in the blog. It is about tag1 and tag2.",
        ),
        BlogSummaryData::from_meta(
            BlogMetaData::new(
                "Post 2",
                Tags::from(vec![
                    Tag {
                        tag: "tag2".to_string(),
                    },
                    Tag {
                        tag: "tag3".to_string(),
                    },
                ]),
                chrono::Utc::now().to_rfc3339(),
            ),
            "This is the second post in the blog. It is about tag2 and tag3.",
        ),
        BlogSummaryData::from_meta(
            BlogMetaData::new(
                "Post 3",
                Tags::from(vec![
                    Tag {
                        tag: "tag3".to_string(),
                    },
                    Tag {
                        tag: "tag4".to_string(),
                    },
                ]),
                chrono::Utc::now().to_rfc3339(),
            ),
            "This is the third post in the blog. It is about tag3 and tag4.",
        ),
    ];
    let posts = match tags {
        Ok(tags) => posts
            .iter()
            .filter(|summary| tags.tags().iter().any(|t| summary.meta.tags.tags().contains(&t)))
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
