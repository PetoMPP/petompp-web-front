use crate::components::atoms::markdown::{Editable, EditableProps};
use yew::prelude::*;

#[derive(PartialEq, Properties)]
pub struct PageBaseProps {
    #[prop_or_default]
    pub children: Children,
}

#[function_component(PageBase)]
pub fn page_base(props: &PageBaseProps) -> Html {
    html! {
        <div class={"relative animate-fade animate-duration-500 animate-ease-in-out flex flex-col grow mt-10 lg:mt-20 w-full p-8 rounded-t-xl bg-base-100"}>
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
            <Editable reskey={props.reskey.clone()}/>
        </PageBase>
    }
}
