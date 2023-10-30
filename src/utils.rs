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
        format!("-webkit-mask: url({}) no-repeat center;mask: url({}) no-repeat center;", path, path)
    }
}
