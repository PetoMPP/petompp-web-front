use crate::api::client::ApiClient;
use crate::components::atoms::loading::Loading;
use crate::components::state::State;
use crate::data::locales::store::LocalesStore;
use crate::data::locales::tk::TK;
use crate::data::resources::ResId;
use crate::data::session::SessionStore;
use crate::router::blog::BlogRoute;
use crate::{
    components::{atoms::markdown::Editable, organisms::blog::blog_summary::BlogSummary},
    pages::page_base::PageBase,
};
use petompp_web_models::models::tag::Tags;
use petompp_web_models::models::user::RoleData;
use yew::platform::spawn_local;
use yew::prelude::*;
use yew_router::prelude::*;
use yewdux::prelude::*;

#[function_component(Blog)]
pub fn blog() -> Html {
    let location = use_location().unwrap();
    let tags = location.query::<Tags>().unwrap_or_default();
    let (session_store, _) = use_store::<SessionStore>();
    let (locales_store, _) = use_store::<LocalesStore>();
    let navigator = use_navigator().unwrap();
    let data = use_state(|| State::Ok(None));
    use_effect_with_deps(
        move |data| {
            let data = data.clone();
            match &*data {
                State::Loading | State::Err(_) | State::Ok(Some(_)) => return,
                _ => data.set(State::Loading),
            }
            spawn_local(async move {
                match ApiClient::get_posts_meta().await {
                    Ok(posts) => data.set(State::Ok(Some(posts))),
                    Err(e) => data.set(State::Err(e)),
                };
            });
        },
        data.clone(),
    );
    let posts = match &*data {
        State::Ok(Some(posts)) => {
            let summaries = posts
                .clone()
                .into_iter()
                .filter(|meta| {
                    tags.is_empty() || tags.tags().iter().any(|t| meta.tags.tags().contains(&t))
                })
                .filter(|meta| meta.lang == locales_store.curr)
                .map(|meta| {
                    html! {
                        <BlogSummary {meta}/>
                    }
                });
            let new_post_button = match &session_store.user {
                Some(u) if u.role == RoleData::Admin => {
                    let onclick = Callback::from(move |_| navigator.push(&BlogRoute::New));
                    Some(html! {
                        <div class={"flex w-full justify-end py-2"}>
                            <button class={"flex btn btn-primary btn-outline"} {onclick}>
                                {locales_store.get(TK::CreateNewBlogPost)}
                            </button>
                        </div>
                    })
                }
                _ => None,
            };

            html! {
                <>
                {new_post_button}
                <div class={"flex flex-col gap-2"}>
                    {for summaries}
                </div>
                </>
            }
        }
        State::Loading | State::Ok(None) => html! {
            <Loading resource={"blog posts".to_string()} />
        },
        State::Err(e) => {
            html! {
                <>
                <h3 class={"mx-auto py-4 text-xl font-semibold"}>{"Failed to load posts!"}</h3>
                <p>{e.to_string()}</p>
                </>
            }
        }
    };
    html! {
        <PageBase>
        <Editable resid={ResId::ResKey("blog-intro".to_string())}/>
        {posts}
        </PageBase>
    }
}
