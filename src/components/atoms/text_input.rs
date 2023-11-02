use yew::prelude::*;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum InputType {
    Text,
    Password,
    Textarea,
}

impl InputType {
    fn as_str(&self) -> &'static str {
        match self {
            InputType::Text => "text",
            InputType::Password => "password",
            InputType::Textarea => panic!(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct TextInputProps {
    pub label: String,
    pub itype: InputType,
    pub enabled: bool,
    pub value: Option<String>,
    pub placeholder: Option<String>,
    pub autocomplete: Option<String>,
    pub onchange: Option<Callback<InputEvent>>,
    pub error: Option<String>,
}

#[function_component(TextInput)]
pub fn text_input(props: &TextInputProps) -> Html {
    let input = match props.itype {
        InputType::Textarea => {
            let class = match &props.error {
                Some(_) => "input input-bordered py-3 overflow-hidden resize-none shadow-md input-error text-error",
                None => "input input-bordered py-3 overflow-hidden resize-none shadow-md",
            };
            html! {
                <textarea {class} disabled={!props.enabled}
                    placeholder={props.placeholder.clone()} autocomplete={props.autocomplete.clone()}
                    oninput={props.onchange.clone()} value={props.value.clone()} />
            }
        }
        _ => {
            let class = match &props.error {
                Some(_) => "input input-bordered shadow-md input-error text-error",
                None => "input input-bordered shadow-md",
            };
            html! {
                <input {class} type={props.itype.as_str()} disabled={!props.enabled}
                    placeholder={props.placeholder.clone()} autocomplete={props.autocomplete.clone()}
                    oninput={props.onchange.clone()} value={props.value.clone()} />
            }
        }
    };
    let span_class = match &props.error {
        Some(_) => "label-text lg:text-lg text-error",
        None => "label-text lg:text-lg",
    };
    html! {
        <div class={"flex flex-col gap-2"}>
            <label class={"label"}>
                <span class={span_class}>{&props.label}</span>
            </label>
            {input}
            <span class={"text-error mt-1"}>{props.error.clone().unwrap_or_default()}</span>
        </div>
    }
}
