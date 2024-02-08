use crate::{
    components::{
        atoms::text_input::{InputType, TextInput, TextareaInput},
        organisms::{blob_tags_input::BlobTagsInput, image_link_input::ImageLinkInput},
    },
    data::locales::{store::LocalesStore, tk::TK},
};
use petompp_web_models::models::blob::blog::BlogMetaData;
use yew::prelude::*;
use yewdux::prelude::*;

#[derive(Clone, Properties, PartialEq)]
pub struct BlogMetaEditorProps {
    pub data: BlogMetaData,
    pub ondatachanged: Callback<BlogMetaData>,
}

#[function_component(BlogMetaEditor)]
pub fn blog_meta_editor(props: &BlogMetaEditorProps) -> Html {
    let (locales_store, _) = use_store::<LocalesStore>();
    let data = props.data.clone();
    let ondatachanged = props.ondatachanged.clone();
    let title_onchange = {
        let data = data.clone();
        let ondatachanged = ondatachanged.clone();
        ondatachanged.reform(move |value| {
            let mut data = data.clone();
            data.set_title(value);
            data
        })
    };
    let summary_onchange = {
        let data = data.clone();
        let ondatachanged = ondatachanged.clone();
        ondatachanged.reform(move |value| {
            let mut data = data.clone();
            data.set_summary(value);
            data
        })
    };
    let tags_onchange = {
        let data = data.clone();
        let ondatachanged = ondatachanged.clone();
        ondatachanged.reform(move |value| {
            let mut data = data.clone();
            data.tags = value;
            data
        })
    };
    let image_onchange = {
        let data = data.clone();
        let ondatachanged = ondatachanged.clone();
        ondatachanged.reform(move |value| {
            let mut data = data.clone();
            data.set_image(value);
            data
        })
    };

    html! {
        <>
            <TextInput
                label={locales_store.get(TK::Title)}
                itype={InputType::Text}
                enabled={true}
                value={props.data.title().clone()}
                onchange={title_onchange}/>
            <TextareaInput
                label={locales_store.get(TK::Summary)}
                enabled={true}
                value={props.data.summary().clone()}
                onchange={summary_onchange}
                error={false}/>
            <BlobTagsInput data={props.data.tags.clone()} ondatachanged={tags_onchange}/>
            <TextInput
                label={locales_store.get(TK::Created)}
                itype={InputType::Text}
                enabled={false}
                value={props.data.created.format("%Y-%m-%d %H:%M:%S").to_string()}/>
            <TextInput
                label={locales_store.get(TK::Updated)}
                itype={InputType::Text}
                enabled={false}
                value={props.data.updated.format("%Y-%m-%d %H:%M:%S").to_string()}/>
            <ImageLinkInput container={"image-upload".to_string()} data={props.data.image().clone()} ondatachanged={image_onchange}/>
        </>
    }
}
