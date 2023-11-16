use self::macros::onchange;
use crate::{
    components::atoms::{
        label::Label,
        text_input::{InputType, TextInput, TextareaInput},
    },
    utils::style::get_svg_bg_mask_style,
};
use petompp_web_models::models::{
    blog_data::BlogMetaData,
    tag::{Tag, Tags},
};
use web_sys::HtmlInputElement;
use yew::{prelude::*, virtual_dom::VNode};

#[derive(Clone, Properties, PartialEq)]
pub struct BlogMetaEditorProps {
    pub data: Option<BlogMetaData>,
    pub ondatachanged: Callback<BlogMetaData>,
}

#[function_component(BlogMetaEditor)]
pub fn blog_meta_editor(props: &BlogMetaEditorProps) -> Html {
    let title_onchange = onchange!(props, props.data.title);
    let tags_onchange = onchange!(props, props.data.tags);
    let image_onchange = onchange!(props, props.data.image);
    let summary_onchange = onchange!(props, props.data.summary);

    html! {
        <>
            <TextInput
                label={"Title"}
                itype={InputType::Text}
                enabled={true}
                value={props.data.as_ref().map(|d| d.title.clone()).unwrap_or_default()}
                onchange={title_onchange}/>
            <BlogTagsEditor data={props.data.as_ref().map(|d| d.tags.clone())} ondatachanged={tags_onchange}/>
            <TextInput
                label={"Created"}
                itype={InputType::Text}
                enabled={false}
                value={props.data.as_ref().map(|d| d.created.format("%Y-%m-%d %H:%M:%S").to_string()).unwrap_or_default()}/>
            <TextInput
                label={"Updated"}
                itype={InputType::Text}
                enabled={false}
                value={props.data.as_ref().map(|d| d.updated.format("%Y-%m-%d %H:%M:%S").to_string()).unwrap_or_default()}/>
            <TextInput
                label={"Image"}
                itype={InputType::Text}
                enabled={true}
                value={props.data.as_ref().map(|d| d.image.clone()).unwrap_or_default()}
                onchange={image_onchange}/>
            <TextareaInput
                label={"Summary"}
                enabled={true}
                value={props.data.as_ref().map(|d| d.summary.clone()).unwrap_or_default()}
                onchange={summary_onchange}/>
        </>
    }
}

#[derive(Clone, Properties, PartialEq)]
pub struct BlogTagsEditorProps {
    pub data: Option<Tags>,
    pub ondatachanged: Callback<Tags>,
}

#[function_component(BlogTagsEditor)]
pub fn blog_tags_editor(props: &BlogTagsEditorProps) -> Html {
    let tags = use_state_eq(|| props.data.clone().unwrap_or_default().tags());
    let tag_nodes = {
        let tags = tags.clone();
        let ondatachanged = props.ondatachanged.clone();
        (*tags).clone().into_iter()
            .map(move |t| {
                let onclick = {
                    let t = t.clone();
                    let tags = tags.clone();
                    let ondatachanged = ondatachanged.clone();
                    Callback::from(move |_| {
                        let new_tags = tags.iter().filter(|x| x != &&t).cloned().collect::<Vec<Tag>>();
                        tags.set(new_tags.clone());
                        ondatachanged.emit(new_tags.into());
                })};
                html! { <span {onclick} class={"flex btn btn-xs normal-case rounded-full"}>{&*t}<div class={"w-4 h-4 bg-base-content"} style={get_svg_bg_mask_style("/img/ui/x.svg")} /></span> }})
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
            let new_tags = tags
                .iter()
                .cloned()
                .chain(std::iter::once(tag))
                .collect::<Vec<Tag>>();
            tags.set(new_tags.clone());
            ondatachanged.emit(new_tags.into());
        })
    };
    html! {
        <Label label={"Tags"}>
            <div class={"flex h-full gap-2 py-3 items-center input input-bordered shadow-md flex-wrap"}>
                {tag_nodes}
                <input class={"flex w-12 grow outline-none bg-transparent"} type={"text"} placeholder={"Add tag.."} {onkeydown} />
            </div>
        </Label>
    }
}

mod macros {
    macro_rules! onchange {
        ($props:expr, $_0:ident.$_1:ident.$field:ident) => {{
            let props = $props.clone();
            props
                .ondatachanged
                .clone()
                .reform(move |data| BlogMetaData {
                    $field: data,
                    ..props.data.clone().unwrap_or_default()
                })
        }};
    }

    pub(crate) use onchange;
}
