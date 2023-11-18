use crate::{
    api::client::ApiClient,
    data::{
        resources::{id::ResId, store::LocalStore},
        session::SessionStore,
    },
    pages::editor::EditorState,
};
use petompp_web_models::models::{blog_data::BlogData, country::Country};
use yew::{platform::spawn_local, prelude::*};
use yewdux::prelude::*;

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub state: EditorState,
    pub onstatechanged: Callback<EditorState>,
    pub resid: Option<ResId>,
    pub lang: Option<Country>,
}

#[function_component(SaveButton)]
pub fn save_button(props: &Props) -> Html {
    let (session_store, _) = use_store::<SessionStore>();
    let (local_store, local_dispatch) = use_store::<LocalStore>();
    let (Some(resid), Some(lang)) = (&props.resid, &props.lang) else {
        return html! {};
    };
    let Some((value, meta)) = local_store.get(&resid, lang.key()) else {
        return html! {};
    };
    if let EditorState::Loading = &props.state {
        return html! {
            <div class={"w-full flex rounded-lg bg-base-100"}>
                <span class={"flex mx-auto py-4 loading loading-ring loading-lg"}/>
            </div>
        };
    }
    let onstatechange = props.onstatechanged.clone();
    let local_dispatch = local_dispatch.clone();
    let resid = resid.clone();
    let lang = lang.clone();
    let value = value.clone();
    let meta = meta.clone();
    let token = session_store.token.clone().unwrap_or_default();
    let onclick = Callback::from(move |_| {
        let onstatechange = onstatechange.clone();
        let local_dispatch = local_dispatch.clone();
        let resid = resid.clone();
        let lang = lang.clone();
        let value = value.clone();
        let meta = meta.clone();
        let token = token.clone();
        spawn_local(async move {
            onstatechange.emit(EditorState::Loading);
            match &resid {
                ResId::ResKey(key) => {
                    match ApiClient::update_resource(&token, key, &lang, &value).await {
                        Ok(_) => {
                            local_dispatch.reduce_mut(|store| store.remove(&resid, lang.key()));
                            onstatechange.emit(EditorState::Ok(None));
                        }
                        Err(e) => {
                            onstatechange.emit(EditorState::Err(e));
                        }
                    }
                }
                ResId::Blob(_) => {
                    match ApiClient::create_or_update_post(
                        resid.id(),
                        lang.key(),
                        &token,
                        &BlogData {
                            meta: meta.clone().unwrap_or_default(),
                            content: value,
                        },
                    )
                    .await
                    {
                        Ok(_) => {
                            local_dispatch.reduce_mut(|store| store.remove(&resid, lang.key()));
                            onstatechange.emit(EditorState::Ok(None));
                        }
                        Err(e) => {
                            onstatechange.emit(EditorState::Err(e));
                        }
                    }
                }
            }
        });
    });
    html! {
        <button class={"btn btn-success grow"} {onclick}>
        {"Save"}
        </button>
    }
}
