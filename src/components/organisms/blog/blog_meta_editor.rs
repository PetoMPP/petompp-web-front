use petompp_web_models::models::blog_data::BlogMetaData;
use yew::prelude::*;

use crate::components::atoms::text_input::{InputType, TextInput};

#[derive(Clone, Properties, PartialEq)]
pub struct BlogMetaEditorProps {
    pub data: Option<BlogMetaData>,
    pub ondatachanged: Callback<BlogMetaData>,
}

#[function_component(BlogMetaEditor)]
pub fn blog_meta_editor(props: &BlogMetaEditorProps) -> Html {
    html! {
        <div class={""}>
            <TextInput label={"Title"} itype={InputType::Text} enabled={true} value={props.data.as_ref().map(|d| d.title.clone()).unwrap_or_default()}/>
            <TextInput label={"Tags"} itype={InputType::Text} enabled={true} value={props.data.as_ref().map(|d| (*d.tags).clone()).unwrap_or_default()}/>
            <TextInput label={"Created"} itype={InputType::Text} enabled={false} value={props.data.as_ref().map(|d| d.created.to_string()).unwrap_or_default()}/>
            <TextInput label={"Updated"} itype={InputType::Text} enabled={false} value={props.data.as_ref().map(|d| d.updated.to_string()).unwrap_or_default()}/>
            <TextInput label={"Image"} itype={InputType::Text} enabled={true} value={props.data.as_ref().map(|d| d.image.clone()).unwrap_or_default()}/>
            <TextInput label={"Summary"} itype={InputType::Textarea} enabled={true} value={props.data.as_ref().map(|d| d.summary.clone()).unwrap_or_default()}/>
        </div>
    }
}
