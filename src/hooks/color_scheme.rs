use super::event::use_event;
use yew::prelude::*;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ColorScheme {
    Light,
    Dark,
}

#[hook]
pub fn use_color_scheme() -> ColorScheme {
    let curr = use_state(|| get_color_scheme_from_media(get_color_scheme_media()));
    {
        let curr = curr.clone();
        use_event(&get_color_scheme_media(), "change", move |e| {
            let media: web_sys::MediaQueryList = e.target_unchecked_into();
            curr.set(get_color_scheme_from_media(media));
        });
    }
    *curr
}

fn get_color_scheme_media() -> web_sys::MediaQueryList {
    web_sys::window()
        .unwrap()
        .match_media("(prefers-color-scheme: dark)")
        .unwrap()
        .unwrap()
}

fn get_color_scheme_from_media(media: web_sys::MediaQueryList) -> ColorScheme {
    match media.matches() {
        true => ColorScheme::Dark,
        false => ColorScheme::Light,
    }
}
