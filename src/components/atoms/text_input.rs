use crate::{components::atoms::label::Label, utils::js::set_textarea_height};
use web_sys::HtmlInputElement;
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
    pub onchange: Option<Callback<String>>,
    pub error: Option<String>,
}

#[function_component(TextInput)]
pub fn text_input(props: &TextInputProps) -> Html {
    match props.itype {
        InputType::Textarea => {
            let class = match &props.error {
                Some(_) => "input input-bordered py-3 overflow-hidden resize-none shadow-md input-error text-error",
                None => "input input-bordered py-3 overflow-hidden resize-none shadow-md",
            };
            let oninput = {
                let onchange = props.onchange.clone();
                Callback::from(move |e: InputEvent| {
                    let target_element = e.target_unchecked_into::<HtmlInputElement>();
                    set_textarea_height(&target_element);
                    if let Some(cb) = &onchange {
                        cb.emit(target_element.value());
                    }
                })
            };
            html! {
                <Label label={props.label.clone()} error={props.error.clone()}>
                <textarea {class} disabled={!props.enabled} placeholder={props.placeholder.clone()}
                        autocomplete={props.autocomplete.clone()} {oninput} value={props.value.clone()} />
                </Label>
            }
        }
        _ => {
            let class = match &props.error {
                Some(_) => "input input-bordered shadow-md input-error text-error",
                None => "input input-bordered shadow-md",
            };
            let oninput = props.onchange.clone().map(move |cb| {
                Callback::from(move |e: InputEvent| {
                    let target_element = e.target_unchecked_into::<HtmlInputElement>();
                    cb.emit(target_element.value());
                })
            });
            html! {
                <Label label={props.label.clone()} error={props.error.clone()}>
                    <input {class} type={props.itype.as_str()} disabled={!props.enabled}
                        placeholder={props.placeholder.clone()} autocomplete={props.autocomplete.clone()}
                        {oninput} value={props.value.clone()} />
                </Label>
            }
        }
    }
}
