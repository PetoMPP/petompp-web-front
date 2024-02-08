use crate::{components::atoms::markdown::Editable, data::resources::id::ResId};
use yew::prelude::*;

#[derive(PartialEq, Properties)]
pub struct PageBaseProps {
    #[prop_or_default]
    pub children: Children,
    pub title: String,
    pub mockup: Option<()>,
}

#[function_component(PageBase)]
pub fn page_base(props: &PageBaseProps) -> Html {
    let mut class = classes!(
        "relative",
        "flex",
        "flex-col",
        "grow",
        "mt-20",
        "w-full",
        "px-6",
        "py-8",
        "lg:px-8",
        "rounded-t-xl",
        "bg-base-100"
    );
    if props.mockup.is_none() {
        class.push("animate-fade");
        class.push("animate-duration-500");
        class.push("animate-ease-in-out");
    }
    let mockup = props.mockup;

    use_effect_with_deps(
        move |title| {
            if mockup.is_some() {
                return;
            }
            let title = match title.is_empty() {
                true => "PetoMPP.NET".to_string(),
                false => format!("{} - PetoMPP.NET", title),
            };
            web_sys::window()
                .and_then(|w| w.document())
                .unwrap()
                .set_title(&title)
        },
        props.title.clone(),
    );
    html! {
        <div {class}>
            <div class={"lg:w-5/6 w-full mx-auto"}>
                {props.children.clone()}
            </div>
        </div>
    }
}

#[derive(PartialEq, Properties)]
pub struct EditablePageBaseProps {
    #[prop_or_default]
    pub children: Children,
    pub title: String,
    pub animatenone: Option<()>,
    pub resid: ResId,
}

#[function_component(EditablePage)]
pub fn editable_page_base(props: &EditablePageBaseProps) -> Html {
    html! {
        <PageBase title={props.title.clone()}>
            <Editable resid={props.resid.clone()}/ >
        </PageBase>
    }
}
