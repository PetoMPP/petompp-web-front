use crate::data::locales::{store::LocalesStore, tk::TK};
use yew::prelude::*;
use yewdux::prelude::*;

#[derive(Clone, Properties, PartialEq)]
pub struct LoadingProps {
    pub resource: Option<String>,
}

#[function_component(Loading)]
pub fn loading(props: &LoadingProps) -> Html {
    let (locales_store, _) = use_store::<LocalesStore>();
    let text = locales_store.get(TK::Loading)
        + props
            .resource
            .as_ref()
            .map(|s| format!(": {}", s))
            .unwrap_or_default()
            .as_str()
        + "...";
    html! {
        <div class={"w-full flex rounded-lg"}>
            <div class={"mx-auto flex flex-row gap-2 rounded-lg"}>
                <span class={"flex loading loading-ring loading-lg"}/>
                <p class={"flex text-base items-center"}>{text}</p>
            </div>
        </div>
    }
}
