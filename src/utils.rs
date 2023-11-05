pub mod macros {
    /// This macro is used to create a callback that will spawn an async block on the local thread.
    #[macro_export]
    macro_rules! async_event {
        (|$($dep:ident),*| $block:block) => {
            {
                use yew::prelude::*;
                use yew::platform::spawn_local;
                $(
                    let $dep = $dep.clone();
                )*
                Callback::from(move |_| {
                    $(
                        let $dep = $dep.clone();
                    )*
                    spawn_local(async move {$block});
                })
            }
        };
        ([prevent $event_type:ty] | $($dep:ident),*| $block:block) => {
            {
                use yew::prelude::*;
                use yew::platform::spawn_local;
                $(
                    let $dep = $dep.clone();
                )*
                Callback::from(move |e: $event_type| {
                    e.prevent_default();
                    $(
                        let $dep = $dep.clone();
                    )*
                    spawn_local(async move {$block});
                })
            }
        };
    }
}

pub mod ext {
    use yew::Callback;

    pub trait Mergable {
        fn merge(self, other: Self) -> Self;
    }

    impl<T: Clone + 'static> Mergable for Callback<T> {
        fn merge(self, other: Self) -> Self {
            Callback::from(move |e: T| {
                self.emit(e.clone());
                other.emit(e);
            })
        }
    }
}

pub mod style {
    pub fn get_svg_bg_mask_style(path: &str) -> String {
        format!(
            "-webkit-mask: url({}) no-repeat center;mask: url({}) no-repeat center;",
            path, path
        )
    }
}

pub mod js {
    use wasm_bindgen::JsCast;
    use web_sys::{Element, HtmlInputElement};

    pub fn set_textarea_text(value: &str, id: &str) {
        let element: HtmlInputElement = web_sys::window()
            .unwrap()
            .document()
            .unwrap()
            .get_element_by_id(id)
            .unwrap()
            .unchecked_into();
        element.set_value(value);
        set_textarea_height(&element);
    }

    pub fn set_textarea_height(element: &Element) {
        let body = web_sys::window()
            .unwrap()
            .document()
            .unwrap()
            .body()
            .unwrap();
        body.set_attribute(
            "style",
            format!("height: {}px;", body.client_height()).as_str(),
        )
        .unwrap();
        element.set_attribute("style", "height: auto;").unwrap();
        let scroll_height = element.scroll_height();
        if scroll_height > element.client_height() {
            element
                .set_attribute("style", format!("height: {}px;", scroll_height).as_str())
                .unwrap();
        }
        body.set_attribute("style", "height: auto;").unwrap();
    }
}
