use crate::{
    components::atoms::label::Label,
    data::locales::{store::LocalesStore, tk::TK},
    utils::style::get_svg_bg_mask_style,
};
use petompp_web_models::models::tag::{Tag, Tags};
use web_sys::HtmlInputElement;
use yew::{prelude::*, virtual_dom::VNode};
use yewdux::prelude::*;

#[derive(Clone, Properties, PartialEq)]
pub struct BlogTagsInputProps {
    pub data: Option<Tags>,
    pub ondatachanged: Callback<Tags>,
}

#[function_component(BlobTagsInput)]
pub fn blob_tags_input(props: &BlogTagsInputProps) -> Html {
    let (locales_store, _) = use_store::<LocalesStore>();
    let tags = props.data.clone().unwrap_or_default().tags();
    let tag_nodes = {
        let ondatachanged = props.ondatachanged.clone();
        let tags = tags.clone();
        let ts = props.data.clone().unwrap_or_default().tags().clone();
        ts.iter()
            .map(move |t| {
                let onclick = {
                    let t = t.clone();
                    let tags = tags.clone();
                    let ondatachanged = ondatachanged.clone();
                    Callback::from(move |_| {
                        let new_tags = tags.iter().filter(|x| x != &&t).cloned().collect::<Vec<Tag>>();
                        ondatachanged.emit(new_tags.into());
                })};
                html! { <span {onclick} class={"flex btn btn-xs normal-case rounded-full"}>{&**t}<div class={"w-4 h-4 bg-base-content"} style={get_svg_bg_mask_style("/img/ui/x.svg")} /></span> }})
            .collect::<VNode>()
    };
    let onkeydown = {
        let ondatachanged = props.ondatachanged.clone();
        Callback::from(move |e: KeyboardEvent| {
            if e.key() != "Enter" {
                return;
            }
            let element: HtmlInputElement = e.target_unchecked_into();
            let tag = Tag {
                tag: element.value(),
            };
            if tag.is_empty() || tags.contains(&tag) {
                return;
            }
            element.set_value("");
            ondatachanged.emit(
                tags.iter()
                    .cloned()
                    .chain(std::iter::once(tag))
                    .collect::<Vec<Tag>>()
                    .into(),
            );
        })
    };
    html! {
        <Label label={locales_store.get(TK::Tags)} error={false}>
            <div class={"flex h-full gap-2 py-3 items-center input input-bordered shadow-md flex-wrap"}>
                {tag_nodes}
                <input class={"flex w-12 grow outline-none bg-transparent"} type={"text"} placeholder={locales_store.get(TK::EnterTag)} {onkeydown} />
            </div>
        </Label>
    }
}
