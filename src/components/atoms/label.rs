use yew::prelude::*;

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct LabelProps {
    pub label: String,
    pub error: bool,
    pub children: Children,
}

#[function_component(Label)]
pub fn label(props: &LabelProps) -> Html {
    let span_class = match props.error {
        true => "label-text lg:text-lg text-error",
        false => "label-text lg:text-lg",
    };
    html! {
        <div class={"flex flex-col gap-2"}>
            <label class={"label"}>
                <span class={span_class}>{&props.label}</span>
            </label>
            {props.children.clone()}
        </div>
    }
}
