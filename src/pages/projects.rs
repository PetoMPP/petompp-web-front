use crate::{
    api::{blob::BlobClient, client::ApiClient},
    components::{
        atoms::{
            carousel::{Carousel, Slide}, link::RouteLink, loading::Loading, markdown::Editable
        },
        state::State,
    },
    data::{
        locales::{store::LocalesStore, tk::TK},
        resources::id::ResId,
    },
    pages::page_base::PageBase,
    router::route::Route,
};
use petompp_web_models::models::blob::project::ProjectMetaData;
use yew::{platform::spawn_local, prelude::*};
use yew_router::prelude::*;
use yewdux::prelude::*;

#[function_component(Projects)]
pub fn projects() -> Html {
    let navigator = use_navigator().unwrap();
    let (locales_store, _) = use_store::<LocalesStore>();
    let state = use_state(|| State::Ok(None));
    use_effect_with_deps(
        |state| {
            match &**state {
                State::Ok(Some(_)) | State::Loading | State::Err(_) => return,
                _ => state.set(State::Loading),
            }
            let state = state.clone();
            spawn_local(async move {
                match ApiClient::get_meta_all::<ProjectMetaData>("project", None).await {
                    Ok(m) => state.set(State::Ok(Some(m))),
                    Err(e) => state.set(State::Err(e)),
                }
            })
        },
        state.clone(),
    );

    // TODO: Top 3 most viewed + most recent
    let (slides, projects) = match &*state {
        State::Ok(Some(m)) => (
            m.iter()
                .map(|p| {
                    let id = p.id().to_string();
                    let src = <ApiClient as BlobClient>::get_url(
                        "project",
                        format!("{}/images/{}", &id, p.splash().cloned().unwrap_or_default())
                            .as_str(),
                    );
                    let onclick = {
                        let id = id.clone();
                        let navigator = navigator.clone();
                        Some(Callback::from(move |_| {
                            let id = id.clone();
                            let navigator = navigator.clone();
                            navigator.push(&Route::Project { id })
                        }))
                    };
                    Slide {
                        src,
                        title: p.title().clone(),
                        summary: Some(p.summary().clone()),
                        onclick,
                    }
                })
                .collect::<Vec<_>>(),
            m.clone(),
        ),
        State::Err(e) => {
            return html! {<p class={"text-xl text-error font-semibold"}>{e.to_string()}</p>}
        }
        _ => return html! {<Loading />},
    };
    let projects = projects.into_iter().map(|p| {
        let id = p.id().to_string();
        html! {
            <li>
                <RouteLink route={Route::Project { id }} text={p.title().clone()}/>
            </li>
        }
    });

    html! {
        <PageBase title={locales_store.get(TK::Projects)}>
            <Editable resid={ResId::ResKey("projects-content".to_string())}/>
            <Carousel {slides} />
            // All projects
            <div class={"prose pt-6"}>
                <h2>{locales_store.get(TK::AllProjects)}</h2>
                <ul>
                    {for projects}
                </ul>
            </div>
        </PageBase>
    }
}
