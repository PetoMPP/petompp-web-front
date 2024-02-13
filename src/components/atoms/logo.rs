use crate::{
    hooks::color_scheme::{use_color_scheme, ColorScheme},
    router::route::Route,
};
use yew::prelude::*;
use yew_router::prelude::*;

#[function_component(Logo)]
pub fn logo() -> Html {
    let navigator = use_navigator().unwrap();
    let color_scheme = use_color_scheme();
    let onclick = Callback::from(move |_| navigator.push(&Route::Home));
    let src = match color_scheme {
        ColorScheme::Light => "img/logo_light.png",
        ColorScheme::Dark => "img/logo_dark.png",
    };
    html! {
        <img {src} class={"btn btn-ghost"} {onclick} />
    }
}
