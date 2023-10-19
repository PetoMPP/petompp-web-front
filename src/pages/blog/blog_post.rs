use crate::{
    api::client::{ApiClient, BlobClient},
    components::atoms::markdown::Markdown,
    data::filename,
    use_effect_deps,
};
use yew::{platform::spawn_local, prelude::*};

#[derive(Clone, PartialEq, Properties)]
pub struct BlogPostProps {
    pub id: String,
}

#[function_component(BlogPost)]
pub fn blog_post(props: &BlogPostProps) -> Html {
    let md = use_state(|| String::new());
    use_effect_deps!(|md, props| {
        spawn_local(async move {
            let Ok(meta) = ApiClient::get_post_meta(props.id.as_str()).await else {
                gloo::console::log!("xD");
                return;
            };
            let filename_service = filename::FilenameService::default();

            let Ok(content) =
                BlobClient::get_post_content(meta.filename(&filename_service).as_str()).await
            else {
                gloo::console::log!("xDDD");
                return;
            };
            md.set(content);
        })
    });
    html! {
        <div class={"prose"}>
            <Markdown markdown={(*md).clone()} allowhtml={true}/>
        </div>
    }
}
