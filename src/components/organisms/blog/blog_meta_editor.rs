use self::macros::onchange;
use crate::{
    components::{
        atoms::{
            label::Label,
            text_input::{InputType, TextInput, TextareaInput},
        },
        organisms::blog::blog_image_select::BlogImageSelect,
    },
    data::locales::{store::LocalesStore, tk::TK},
    utils::{ext::Mergable, style::get_svg_bg_mask_style},
};
use petompp_web_models::models::{
    blog_data::BlogMetaData,
    tag::{Tag, Tags},
};
use wasm_bindgen::JsCast;
use web_sys::{HtmlElement, HtmlInputElement};
use yew::{prelude::*, virtual_dom::VNode};
use yewdux::prelude::*;

#[derive(Clone, Properties, PartialEq)]
pub struct BlogMetaEditorProps {
    pub data: Option<BlogMetaData>,
    pub ondatachanged: Callback<BlogMetaData>,
}

#[function_component(BlogMetaEditor)]
pub fn blog_meta_editor(props: &BlogMetaEditorProps) -> Html {
    let (locales_store, _) = use_store::<LocalesStore>();
    let title_onchange = onchange!(props, props.data.title);
    let tags_onchange = onchange!(props, props.data.tags);
    let image_onchange = onchange!(props, props.data.image);
    let summary_onchange = onchange!(props, props.data.summary);

    html! {
        <>
            <TextInput
                label={locales_store.get(TK::Title)}
                itype={InputType::Text}
                enabled={true}
                value={props.data.as_ref().map(|d| d.title.clone()).unwrap_or_default()}
                onchange={title_onchange}/>
            <BlogTagsEditor data={props.data.as_ref().map(|d| d.tags.clone())} ondatachanged={tags_onchange}/>
            <TextInput
                label={locales_store.get(TK::Created)}
                itype={InputType::Text}
                enabled={false}
                value={props.data.as_ref().map(|d| d.created.format("%Y-%m-%d %H:%M:%S").to_string()).unwrap_or_default()}/>
            <TextInput
                label={locales_store.get(TK::Updated)}
                itype={InputType::Text}
                enabled={false}
                value={props.data.as_ref().map(|d| d.updated.format("%Y-%m-%d %H:%M:%S").to_string()).unwrap_or_default()}/>
            <ImageLinkEditor data={props.data.as_ref().map(|d| d.image.clone())} ondatachanged={image_onchange}/>
            <TextareaInput
                label={locales_store.get(TK::Summary)}
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
        <Label label={locales_store.get(TK::Tags)}>
            <div class={"flex h-full gap-2 py-3 items-center input input-bordered shadow-md flex-wrap"}>
                {tag_nodes}
                <input class={"flex w-12 grow outline-none bg-transparent"} type={"text"} placeholder={locales_store.get(TK::EnterTag)} {onkeydown} />
            </div>
        </Label>
    }
}

#[derive(Clone, Properties, PartialEq)]
pub struct ImageLinkEditorProps {
    pub data: Option<String>,
    pub ondatachanged: Callback<String>,
}

#[function_component(ImageLinkEditor)]
pub fn image_link_editor(props: &ImageLinkEditorProps) -> Html {
    let (locales_store, _) = use_store::<LocalesStore>();
    let focus_out = Callback::from(|_| {
        let element = web_sys::window()
            .unwrap()
            .document()
            .unwrap()
            .get_element_by_id("modal")
            .unwrap()
            .unchecked_into::<HtmlElement>();
        element.focus().unwrap();
    });
    let ondatachanged = {
        let ondatachanged = props.ondatachanged.clone();
        Callback::from(move |d: Option<_>| d.map(|d| ondatachanged.emit(d)).unwrap_or_default())
            .merge(focus_out)
    };
    html! {
        <Label label={locales_store.get(TK::Image)} >
            <BlogImageSelect data={props.data.clone()} {ondatachanged} />
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
