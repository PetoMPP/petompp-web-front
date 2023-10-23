use crate::api::client::ApiClient;
use crate::components::atoms::flag::FlagSelect;
use crate::components::organisms::markdown_editor::MarkdownEditor;
use crate::data::locales::store::LocalesStore;
use crate::data::resources::Key;
use crate::pages::page_base::PageBase;
use yew::platform::spawn_local;
use yew::prelude::*;
use yewdux::prelude::*;

#[derive(Clone, PartialEq, Properties)]
pub struct EditorProps {
    pub reskey: String,
    pub lang: String,
}

impl From<EditorProps> for Key {
    fn from(val: EditorProps) -> Self {
        Key {
            reskey: val.reskey,
            lang: val.lang,
        }
    }
}

#[function_component(Editor)]
pub fn editor(props: &EditorProps) -> Html {
    let (locales_store, _) = use_store::<LocalesStore>();
    let state = use_state_eq(|| None);
    let error_state = use_state_eq(|| None);
    let reskey: Key = props.clone().into();
    use_effect_with_deps(
        |(reskey, state, error_state)| {
            let reskey = reskey.clone();
            let state = state.clone();
            let error_state = error_state.clone();
            spawn_local(async move {
                match ApiClient::get_resource(reskey.reskey.as_str(), reskey.lang.as_str()).await {
                    Ok(new_state) => state.set(Some(new_state)),
                    Err(e) => {
                        error_state.set(Some(e));
                    }
                }
            })
        },
        (reskey.clone(), state.clone(), error_state.clone()),
    );
    let editor = state.as_ref().map(|s| html! {
        <div class={"flex bg-base-300 rounded-lg p-2"}>
            <MarkdownEditor reskey={reskey.clone()} state={s.clone()} onmodifiedchanged={Callback::noop()}/>
        </div>
    });
    html! {
        <PageBase>
            <div class={"prose"}>
                <h1>{"Editor"}</h1>
                <p>{"This is the editor page. Here you can edit the content of the page selected."}</p>
                <h3>{"Now editing:"}<a class={"btn btn-sm m-1 p-1"}>{&props.reskey}</a></h3>
                <h3 class={"not-prose flex gap-2 align-center"}>{"In lang:"}<FlagSelect country={locales_store.curr} /></h3>
                <p/>
            </div>
            {editor}
        </PageBase>
    }
}
