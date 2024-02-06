use crate::{
    api::{
        blob::BlobClient,
        client::{ApiClient, RequestError},
    },
    components::{
        atoms::{
            carousel::{Carousel, Slide},
            loading::Loading,
            markdown::{EditButton, Markdown},
        },
        state::State,
    },
    data::{
        locales::{store::LocalesStore, tk::TK},
        resources::id::{BlobType, ResId},
        session::SessionStore,
    },
    pages::page_base::PageBase,
};
use petompp_web_models::models::blob::{markdown::MarkdownMeta, project::ProjectMetaData};
use yew::{platform::spawn_local, prelude::*};
use yewdux::prelude::*;

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct ProjectProps {
    pub id: String,
}

#[function_component(Project)]
pub fn project(props: &ProjectProps) -> Html {
    let (_, session_dispatch) = use_store::<SessionStore>();
    let (locales_store, _) = use_store::<LocalesStore>();
    let data = use_state(|| State::Ok(None));
    use_effect_with_deps(
        |(props, data, locales_store)| {
            let props = props.clone();
            let data = data.clone();
            let locales_store = locales_store.clone();
            match *data {
                State::Ok(Some((_, _, last))) if last == locales_store.curr => return,
                State::Loading | State::Err(_) => return,
                _ => data.set(State::Loading),
            }
            spawn_local(async move {
                let filename = MarkdownMeta::create_filename(props.id.as_str(), locales_store.curr);
                match futures::join!(
                    ApiClient::get_meta::<ProjectMetaData>("project", &filename),
                    ApiClient::get_content_str("project", &filename),
                ) {
                    (Ok(meta), Ok(md)) => data.set(State::Ok(Some((meta, md, locales_store.curr)))),
                    (Err(e), _) | (_, Err(e)) => {
                        data.set(State::Err(e));
                    }
                }
            });
        },
        (props.clone(), data.clone(), locales_store.clone()),
    );
    let (markdown, title) = match &*data {
        State::Ok(Some((m, md, _))) => (
            html! {<Markdown markdown={md.clone()} allowhtml={true} interactive={Some(())}/>},
            m.title().clone(),
        ),
        State::Loading | State::Ok(None) => (html! { <Loading /> }, locales_store.get(TK::Loading)),
        State::Err(e) => {
            if let Err(redirect) = e.handle_failed_auth(session_dispatch) {
                return redirect;
            }
            (
                html! {
                    <>
                    <h3 class={"mx-auto py-4 text-xl font-semibold"}>{"Failed to load blog post!"}</h3>
                    <p>{e.to_string()}</p>
                    </>
                },
                locales_store.get(TK::ErrorOccured),
            )
        }
    };
    html! {
        <PageBase {title}>
            <EditButton resid={ResId::Blob(BlobType::Project(props.id.clone()))} />
            <div class={"mx-auto flex flex-col w-full"}>
                {markdown}
                <ProjectGallery id={props.id.clone()} />
            </div>
        </PageBase>
    }
}

#[function_component(ProjectGallery)]
pub fn project_gallery(props: &ProjectProps) -> Html {
    let (_, session_dispatch) = use_store::<SessionStore>();
    let (locales_store, _) = use_store::<LocalesStore>();
    let data = use_state(|| State::Ok(None));
    use_effect_with_deps(
        |(props, data)| {
            let props = props.clone();
            let data = data.clone();
            match *data {
                State::Ok(Some(_)) | State::Loading | State::Err(_) => return,
                _ => data.set(State::Loading),
            }
            spawn_local(async move {
                let img_dir = format!("{}/images/", &props.id);
                match ApiClient::get_names("project", Some(img_dir.as_str())).await {
                    Ok(images) => data.set(State::Ok(Some(images))),
                    Err(RequestError::Endpoint(404, _)) => data.set(State::Ok(Some(vec![]))),
                    Err(e) => data.set(State::Err(e)),
                }
            })
        },
        (props.clone(), data.clone()),
    );
    let inner = match &*data {
        State::Loading | State::Ok(None) => {
            html! {<Loading resource={locales_store.get(TK::Images)}/>}
        }
        State::Err(e) => {
            if let Err(redirect) = e.handle_failed_auth(session_dispatch) {
                return redirect;
            }
            html! {
                <>
                <h3 class={"mx-auto py-4 text-xl font-semibold"}>{"Failed to load gallery!"}</h3>
                <p>{e.to_string()}</p>
                </>
            }
        }
        State::Ok(Some(images)) => {
            let slides = images
                .iter()
                .map(|i| Slide {
                    src: <ApiClient as BlobClient>::get_url("project", i),
                    ..Default::default()
                })
                .collect::<Vec<_>>();
            html! {
                <Carousel {slides} />
            }
        }
    };
    html! {
        <>
        <div class={"prose pb-8"}>
            <h2>{"Gallery"}</h2>
        </div>
        {inner}
        </>
    }
}
