use petompp_web_models::models::tag::{Tag, Tags};
use yew::prelude::*;
use yew_router::prelude::*;

use crate::router::Route;

#[derive(Clone, PartialEq, Properties)]
pub struct BlogTagProps {
    pub tag: Tag,
}

#[function_component(BlogTag)]
pub fn blog_tag(props: &BlogTagProps) -> Html {
    let navigator = use_navigator().unwrap();
    let location = use_location().unwrap();
    let curr_tags = location.query::<Tags>();
    let tag = props.tag.clone();
    let (class, next_tags) = match curr_tags {
        Ok(curr_tags) if curr_tags.tags().contains(&tag) => (
            "badge badge-sm badge-primary font-bold",
            Tags::from(
                curr_tags
                    .tags()
                    .iter()
                    .filter(|t| t != &&tag)
                    .cloned()
                    .collect::<Vec<_>>(),
            ),
        ),
        Ok(curr_tags) => (
            "badge badge-sm badge-outline badge-primary",
            Tags::from(
                curr_tags
                    .tags()
                    .iter()
                    .chain(vec![tag].iter())
                    .cloned()
                    .collect::<Vec<_>>(),
            ),
        ),
        _ => (
            "badge badge-sm badge-outline badge-primary",
            Tags::from(vec![tag]),
        ),
    };
    let onclick = Callback::from(move |_| {
        if next_tags.tags().is_empty() {
            navigator.push(&Route::Blog);
            return;
        }
        navigator.push_with_query(&Route::Blog, &next_tags).unwrap();
    });
    html! {
        <button {class} {onclick}>{&props.tag.tag}</button>
    }
}
