use crate::{
    components::atoms::label::Label,
    utils::js::{set_textarea_height, set_textarea_text},
};
use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;
use yew::prelude::*;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum InputType {
    Text,
    Password,
}

impl InputType {
    fn as_str(&self) -> &'static str {
        match self {
            InputType::Text => "text",
            InputType::Password => "password",
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
    let id = use_memo(
        |_| web_sys::window().unwrap().crypto().unwrap().random_uuid()[..10].to_string(),
        (),
    );
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
    {
        let id = id.clone();
        let initial_state = props.value.clone();
        use_effect_with_deps(
            move |_| {
                if let Some(input) = web_sys::window()
                    .and_then(|w| w.document())
                    .and_then(|d| d.get_element_by_id(id.as_str()))
                    .and_then(|e| e.dyn_into::<HtmlInputElement>().ok())
                {
                    input.set_value(
                        initial_state
                            .as_ref()
                            .map(|s| s.as_str())
                            .unwrap_or_default(),
                    );
                }
            },
            (),
        );
    }
    html! {
        <Label label={props.label.clone()} error={props.error.clone()}>
            <input id={(*id).clone()} {class} type={props.itype.as_str()} disabled={!props.enabled}
                placeholder={props.placeholder.clone()} autocomplete={props.autocomplete.clone()}
                {oninput}/>
        </Label>
    }
}

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct TextareaInputProps {
    pub label: String,
    pub enabled: bool,
    pub value: Option<String>,
    pub autocomplete: Option<String>,
    pub onchange: Option<Callback<String>>,
    pub error: Option<String>,
}

#[function_component(TextareaInput)]
pub fn textarea_input(props: &TextareaInputProps) -> Html {
    let id = use_memo(
        |_| web_sys::window().unwrap().crypto().unwrap().random_uuid()[..10].to_string(),
        (),
    );
    use_effect_with_deps(
        |id| {
            let element = web_sys::window()
                .unwrap()
                .document()
                .unwrap()
                .get_element_by_id(id.as_str())
                .unwrap();
            set_textarea_height(&element);
        },
        id.clone(),
    );
    let class = match &props.error {
        Some(_) => {
            "input input-bordered py-3 overflow-hidden resize-none shadow-md input-error text-error"
        }
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
    {
        let id = id.clone();
        let initial_state = props.value.clone();
        use_effect_with_deps(
            move |_| {
                set_textarea_text(
                    initial_state
                        .as_ref()
                        .map(|s| s.as_str())
                        .unwrap_or_default(),
                    &id,
                )
            },
            (),
        );
    }
    html! {
        <Label label={props.label.clone()} error={props.error.clone()}>
            <textarea id={(*id).clone()} {class} disabled={!props.enabled}
                autocomplete={props.autocomplete.clone()} {oninput}/>
        </Label>
    }
}
