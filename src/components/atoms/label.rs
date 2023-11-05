use yew::prelude::*;

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct LabelProps {
    pub label: String,
    pub error: Option<String>,
    pub children: Children,
}

#[function_component(Label)]
pub fn label(props: &LabelProps) -> Html {
    let span_class = match &props.error {
        Some(_) => "label-text lg:text-lg text-error",
        None => "label-text lg:text-lg",
    };
    html! {
        <div class={"flex flex-col gap-2"}>
            <label class={"label"}>
                <span class={span_class}>{&props.label}</span>
            </label>
            {props.children.clone()}
            <span class={"text-error mt-1"}>{props.error.clone().unwrap_or_default()}</span>
        </div>
    }
}
