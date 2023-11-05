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
    let title_onchange = {
        let props = props.clone();
        props.ondatachanged.clone().reform(move |title: String| {
            let mut data = props.data.clone().unwrap_or_default();
            data.title = title;
            data
        })
    };
    html! {
        <>
            <TextInput label={"Title"} itype={InputType::Text} enabled={true} value={props.data.as_ref().map(|d| d.title.clone()).unwrap_or_default()} onchange={title_onchange}/>
            <BlogTagsEditor data={props.data.as_ref().map(|d| d.tags.clone())} ondatachanged={Callback::noop()}/>
            <TextInput label={"Created"} itype={InputType::Text} enabled={false} value={props.data.as_ref().map(|d| d.created.format("%Y-%m-%d %H:%M:%S").to_string()).unwrap_or_default()}/>
            <TextInput label={"Updated"} itype={InputType::Text} enabled={false} value={props.data.as_ref().map(|d| d.updated.format("%Y-%m-%d %H:%M:%S").to_string()).unwrap_or_default()}/>
            <TextInput label={"Image"} itype={InputType::Text} enabled={true} value={props.data.as_ref().map(|d| d.image.clone()).unwrap_or_default()}/>
            <TextareaInput label={"Summary"} enabled={true} value={props.data.as_ref().map(|d| d.summary.clone()).unwrap_or_default()}/>
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
        (*tags).clone().into_iter()
            .map(move |t| {
                let onclick = {
                    let t = t.clone();
                    let tags = tags.clone();
                    Callback::from(move |_| {
                        tags.set(tags.iter().filter(|x| x != &&t).cloned().collect::<Vec<Tag>>());
                })};
                html! { <span {onclick} class={"flex btn btn-xs normal-case rounded-full"}>{&*t}<div class={"w-4 h-4 bg-base-content"} style={get_svg_bg_mask_style("/img/ui/x.svg")} /></span> }})
            .collect::<VNode>()
    };
    let onkeydown = Callback::from(move |e: KeyboardEvent| {
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
        tags.set(
            tags.iter()
                .cloned()
                .chain(std::iter::once(tag))
                .collect::<Vec<Tag>>(),
        );
    });
    html! {
        <Label label={"Tags"}>
            <div class={"flex h-full gap-2 py-3 items-center input input-bordered shadow-md flex-wrap"}>
                {tag_nodes}
                <input class={"flex w-12 grow outline-none"} type={"text"} placeholder={"Add tag.."} {onkeydown} />
            </div>
        </Label>
    }
}
