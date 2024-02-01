use crate::{
    components::{
        atoms::label::Label,
        organisms::blob_image_select::{BlobBrowserDialog, BlobBrowserDialogConfig},
    },
    data::locales::{store::LocalesStore, tk::TK},
};
use yew::prelude::*;
use yewdux::prelude::*;

#[derive(Clone, Properties, PartialEq)]
pub struct ImageDirectoryBrowserProps {
    pub container: String,
    pub folder: Option<String>,
}

#[function_component(ImageDirectoryBrowser)]
pub fn image_directory_browser(props: &ImageDirectoryBrowserProps) -> Html {
    let (locales_store, _) = use_store::<LocalesStore>();
    html! {
        <Label label={locales_store.get(TK::Images)} error={false}>
            <BlobImageDirectoryView container={props.container.clone()} folder={props.folder.clone()} />
        </Label>
    }
}

#[derive(Clone, Properties, PartialEq)]
pub struct BlobImageDirectoryViewProps {
    pub container: String,
    pub folder: Option<String>,
}

#[function_component(BlobImageDirectoryView)]
pub fn blob_image_dir_view(props: &BlobImageDirectoryViewProps) -> Html {
    let (locales_store, _) = use_store::<LocalesStore>();
    let id = use_memo(
        |_| web_sys::window().unwrap().crypto().unwrap().random_uuid()[..10].to_string(),
        (),
    );
    let force_open = use_state(|| false);
    let mut dropdown_class = classes!(
        "dropdown",
        "dropdown-top",
        "w-full",
        "input",
        "input-bordered",
        "shadow-md",
        "grid",
        "grid-cols-auto-2",
        "items-center",
        "justify-between",
        "px-0"
    );
    if *force_open {
        dropdown_class.push("dropdown-open");
    }
    let onforceopenchanged = {
        let force_open = force_open.clone();
        Callback::from(move |fo| force_open.set(fo))
    };
    let config = BlobBrowserDialogConfig {
        readonly: true,
        container: props.container.clone(),
        root: props.folder.clone(),
    };
    html! {
        <div class={"flex flex-col gap-2"}>
            <div class={"w-full"}>
                <div id={(*id).clone()} tabindex={"0"} class={dropdown_class}>
                    <div class={"pl-2 truncate"}>{"Click here to select images"}</div>
                    <label class={"rounded-l-none btn btn-primary no-animation"} tabindex={"0"}>{locales_store.get(TK::Edit)}</label>
                    <div tabindex={"0"} class={"dropdown-content w-full flex flex-col mb-4 gap-1 z-10"}>
                        <BlobBrowserDialog parentid={(*id).clone()} {config} ondatachanged={Callback::noop()} {onforceopenchanged} />
                    </div>
                </div>
            </div>
        </div>
    }
}
