use yew::prelude::*;

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct CollapseProps {
    pub label: String,
    pub children: Children,
}

#[function_component(Collapse)]
pub fn collapse(props: &CollapseProps) -> Html {
    html! {
        <div class={"collapse collapse-arrow border shadow-lg"}>
            <input type={"checkbox"} />
            <div class={"collapse-title text-xl"}>
                {&props.label}
            </div>
            <div class={"collapse-content"}>
                {props.children.clone()}
            </div>
        </div>
    }
}
