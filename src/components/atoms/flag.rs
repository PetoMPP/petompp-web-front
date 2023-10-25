use petompp_web_models::models::country::{into_iter, Country};
use wasm_bindgen::JsCast;
use web_sys::HtmlElement;
use yew::prelude::*;

#[derive(Clone, PartialEq, Properties)]
pub struct FlagProps {
    pub country: Country,
}

#[function_component(Flag)]
pub fn flag(props: &FlagProps) -> Html {
    html! {
        <img src={format!("/img/flags/{}.svg", props.country.key())} class={"w-12 h-8 rounded-xl"} />
    }
}

#[derive(Clone, PartialEq, Properties)]
pub struct FlagSelectProps {
    pub country: Country,
    pub onselectedchanged: Option<Callback<Country>>,
}

#[function_component(FlagSelect)]
pub fn flag_select(props: &FlagSelectProps) -> Html {
    let get_onclick = |c: &Country| {
        let changed = props.onselectedchanged.clone();
        let c = *c;
        Callback::from(move |_| {
            if let Some(element) = web_sys::window()
                .unwrap()
                .document()
                .unwrap()
                .active_element()
            {
                element.unchecked_into::<HtmlElement>().blur().unwrap();
            }
            if let Some(cb) = changed.as_ref() {
                cb.emit(c)
            }
        })
    };
    html! {
        <div class={"dropdown block"}>
            <label class={"flex btn btn-md"} tabindex={"0"}>
            <Flag country={props.country} />
            </label>
            <ul tabindex={"0"} class={"dropdown-content bg-base-200 hover:bg-base-300 mt-1 rounded-md flex z-[1]"}>
            { for Country::iter()
                .filter(|c| c != &props.country)
                .map(|country|
                    html! {
                        <li class={"flex btn btn-md"} onclick={get_onclick(&country)}>
                            <Flag {country} />
                        </li>
                    }
                )
            }
            </ul>
        </div>
    }
}
