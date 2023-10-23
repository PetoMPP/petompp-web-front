use crate::pages::blog::{blog::Blog, blog_post::BlogPost, new_post::NewPost};
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Debug, Clone, Routable, PartialEq)]
pub enum BlogRoute {
    #[at("/blog")]
    Blog,
    #[at("/blog/post/:id")]
    Post { id: String },
    #[at("/blog/new")]
    New,
}

impl BlogRoute {
    pub fn switch(self) -> Html {
        match self {
            BlogRoute::Blog => html! {<Blog />},
            BlogRoute::Post { id } => html! {<BlogPost {id} />},
            BlogRoute::New => html! {<NewPost />},
        }
    }
}
