use crate::components::atoms::markdown::{Editable, EditableProps};
use yew::prelude::*;

#[derive(PartialEq, Properties)]
pub struct PageBaseProps {
    #[prop_or_default]
    pub children: Children,
    pub animatenone: Option<()>,
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
    if props.animatenone.is_none() {
        class.push("animate-fade");
        class.push("animate-duration-500");
        class.push("animate-ease-in-out");
    }
    html! {
        <div {class}>
            <div class={"lg:w-5/6 w-full mx-auto"}>
                {props.children.clone()}
            </div>
        </div>
    }
}

#[function_component(EditablePage)]
pub fn editable_page_base(props: &EditableProps) -> Html {
    html! {
        <PageBase>
            <Editable resid={props.resid.clone()}/ >
        </PageBase>
    }
}
