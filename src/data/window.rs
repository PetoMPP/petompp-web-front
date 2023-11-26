use serde::{Deserialize, Serialize};
use wasm_bindgen::{closure::Closure, JsCast};
use yewdux::prelude::*;

#[derive(PartialEq, Clone, Debug, Store, Serialize, Deserialize, Default)]
pub struct WindowStore {
    pub has_focus: bool,
}

impl WindowStore {
    pub fn add_focus_change_event_listener(dispatch: Dispatch<Self>) {
        let closure = Closure::wrap(Box::new(move || {
            dispatch.reduce_mut(|s| {
                s.has_focus = web_sys::window()
                    .unwrap()
                    .document()
                    .unwrap()
                    .has_focus()
                    .unwrap_or_default();
            });
        }) as Box<dyn FnMut()>);
        web_sys::window()
            .unwrap()
            .document()
            .unwrap()
            .add_event_listener_with_callback("focus", closure.as_ref().unchecked_ref())
            .unwrap();
        web_sys::window()
            .unwrap()
            .document()
            .unwrap()
            .add_event_listener_with_callback("focusin", closure.as_ref().unchecked_ref())
            .unwrap();
        web_sys::window()
            .unwrap()
            .document()
            .unwrap()
            .add_event_listener_with_callback("focusout", closure.as_ref().unchecked_ref())
            .unwrap();
        closure.forget();
    }
}
