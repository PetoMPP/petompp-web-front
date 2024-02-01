use crate::{
    components::{
        atoms::text_input::{InputType, TextInput, TextareaInput},
        organisms::{
            blob_tags_input::BlobTagsInput, image_directory_browser::ImageDirectoryBrowser,
        },
    },
    data::locales::{store::LocalesStore, tk::TK},
};
use petompp_web_models::models::blob::project::ProjectMetaData;
use yew::prelude::*;
use yewdux::prelude::*;

#[derive(Clone, Properties, PartialEq)]
pub struct ProjectMetaEditorProps {
    pub data: ProjectMetaData,
    pub ondatachanged: Callback<ProjectMetaData>,
}

#[function_component(ProjectMetaEditor)]
pub fn project_meta_editor(props: &ProjectMetaEditorProps) -> Html {
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
            <ImageDirectoryBrowser container={"project".to_string()} folder={Some((props.data.id().to_string() + "/images/").to_string())}/>
        </>
    }
}
