use crate::{
    api::client::{ApiClient, BlobClient},
    components::atoms::markdown::Markdown,
    data::{locales::{store::LocalesStore, tk::TK}, session::SessionStore},
    handle_api_error,
    pages::page_base::PageBase,
};
use yew::{platform::spawn_local, prelude::*};
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
    let error_state = use_state_eq(|| None);
    let blog_data = use_state(|| None);
    let loading = use_state(|| false);
    use_effect_with_deps(
        |(props, error_state, blog_data, locales_store)| {
            let props = props.clone();
            let error_state = error_state.clone();
            let blog_data = blog_data.clone();
            let locales_store = locales_store.clone();
            if *loading {
                return;
            }
            match *blog_data {
                Some((_, _, last)) if last == locales_store.curr => return,
                _ => {}
            }
            loading.set(true);
            spawn_local(async move {
                let meta =
                    match ApiClient::get_post_meta(props.id.as_str(), locales_store.curr.key())
                        .await
                    {
                        Ok(meta) => meta,
                        Err(e) => {
                            error_state.set(Some(e));
                            loading.set(false);
                            return;
                        }
                    };
                let md = match BlobClient::get_post_content(meta.filename().as_str()).await {
                    Ok(content) => content,
                    Err(e) => {
                        error_state.set(Some(e));
                        loading.set(false);
                        return;
                    }
                };
                blog_data.set(Some((meta, md, locales_store.curr.clone())));
                loading.set(false);
            });
        },
        (
            props.clone(),
            error_state.clone(),
            blog_data.clone(),
            locales_store.clone(),
        ),
    );
    handle_api_error!(
        error_state,
        session_dispatch,
        Some((&Route::Blog, &navigator))
    );
    let onclick = Callback::from(move |_| navigator.push(&Route::Blog));
    let markdown = blog_data
        .as_ref()
        .map(|(_, md, _)| md.clone())
        .unwrap_or_default();
    let meta = blog_data.as_ref().map(|(m, _, _)| {
        let img = m.image.as_ref().map(|i| BlobClient::get_url(format!("image-upload/{}", i).as_str())).unwrap_or("/img/placeholder.svg".to_string());
        html! {
            <div class={"hero mb-4 md:pt-36 pt-16 rounded-lg p-2"} style={format!("background-image: url({}); -webkit-mask-image: -webkit-linear-gradient(top, rgba(0,0,0,0),rgba(0,0,0,0.8));", img)}>
                <div class={"prose text-neutral text-center max-w-md"}>
                    <h1 class={"text-neutral"}>{&m.title}</h1>
                    <p>{&m.summary}</p>
                </div>
            </div>
        }}
    );

    html! {
        <PageBase>
            <a class={"lg:mb-6 mb-4"} href={"javascript:void(0);"} {onclick}>{locales_store.get(TK::BackToBlogPosts)}</a>
            {meta}
            <div class={"divider"}/>
            <div class={"mx-auto prose flex flex-col w-full"}>
                <Markdown {markdown} allowhtml={true} interactive={Some(())}/>
            </div>
        </PageBase>
    }
}
