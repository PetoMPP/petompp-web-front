use crate::{
    api::client::{ApiClient, BlobClient},
    components::{
        atoms::{
            loading::Loading,
            markdown::{EditButton, Markdown},
        },
        state::State,
    },
    data::{
        locales::{store::LocalesStore, tk::TK},
        resources::id::ResId,
        session::SessionStore,
    },
    pages::page_base::PageBase,
    router::route::Route,
};
use chrono::{DateTime, Local};
use petompp_web_models::models::blog_data::BlogMetaData;
use yew::{platform::spawn_local, prelude::*};
use yew_router::prelude::*;
use yewdux::prelude::*;

#[derive(Clone, PartialEq, Properties)]
pub struct BlogPostProps {
    pub id: String,
}

#[function_component(BlogPost)]
pub fn blog_post(props: &BlogPostProps) -> Html {
    let navigator = use_navigator().unwrap();
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
                let meta =
                    match ApiClient::get_post_meta(props.id.as_str(), locales_store.curr.key())
                        .await
                    {
                        Ok(meta) => meta,
                        Err(e) => {
                            data.set(State::Err(e));
                            return;
                        }
                    };
                let md = match BlobClient::get_post_content(
                    format!("{}/{}.md", meta.id, meta.lang.key()).as_str(),
                )
                .await
                {
                    Ok(content) => content,
                    Err(e) => {
                        data.set(State::Err(e));
                        return;
                    }
                };
                data.set(State::Ok(Some((meta, md, locales_store.curr))));
            });
        },
        (props.clone(), data.clone(), locales_store.clone()),
    );
    let (meta, markdown, title) = match &*data {
        State::Ok(Some((m, md, _))) => (
            html! {<BlogPostMeta meta={m.clone()} />},
            Some(html! {<Markdown markdown={md.clone()} allowhtml={true} interactive={Some(())}/>}),
            m.title.clone(),
        ),
        State::Loading | State::Ok(None) => {
            (html! { <Loading /> }, None, locales_store.get(TK::Loading))
        }
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
                None,
                locales_store.get(TK::ErrorOccured),
            )
        }
    };
    let onclick = Callback::from(move |_| navigator.push(&Route::Blog));
    html! {
        <PageBase {title}>
            <EditButton resid={ResId::Blob(props.id.clone())} />
            <a class={"lg:mb-6 mb-4"} href={"javascript:void(0);"} {onclick}>{locales_store.get(TK::BackToBlogPosts)}</a>
            {meta}
            <div class={"divider"}/>
            <div class={"mx-auto prose flex flex-col w-full"}>
                {markdown}
            </div>
        </PageBase>
    }
}

#[derive(Clone, PartialEq, Properties)]
pub struct BlogPostMetaProps {
    pub meta: BlogMetaData,
}

#[function_component(BlogPostMeta)]
pub fn blog_post_meta(props: &BlogPostMetaProps) -> Html {
    let img = match props.meta.image.as_str() {
        "" => "/img/placeholder.svg".to_string(),
        img => BlobClient::get_url(format!("image-upload/{}", img).as_str()),
    };
    let sign = format!(
        "{} by {}",
        DateTime::<Local>::from(props.meta.created).format("%Y-%m-%d %H:%M:%S"),
        "PetoMPP"
    );
    html! {
        <>
        <div class={"hero mb-4 py-12 lg:py-16 rounded-lg p-2"} style={format!("background-image: url({});", img)}>
            <div class={"flex p-4 lg:w-1/2 text-base-content font-semibold text-center text-xl lg:text-4xl aspect-1-1 bg-base-100 bg-opacity-60 rounded-full items-center"}>
                <div>
                    <div class={"divider divider-base-content w-5/6 mx-auto"} />
                    <p class={"px-6 lg:px-12"}>{&props.meta.title}</p>
                    <div class={"divider divider-base-content w-5/6 mx-auto"} />
                </div>
            </div>
        </div>
        <p class={"italic text-lg"}>{sign}</p>
        </>
    }
}
