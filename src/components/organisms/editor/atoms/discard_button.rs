use crate::{
    async_event,
    components::atoms::modal::{
        show_modal_callback, Buttons, DialogData, ModalButton, ModalData, ModalStore,
    },
    data::{
        locales::{store::LocalesStore, tk::TK},
        resources::store::LocalStore,
    },
    pages::editor::{EditorProps, EditorState},
    router::route::Route,
};
use yew::prelude::*;
use yew_router::prelude::*;
use yewdux::prelude::*;

#[function_component(DiscardButton)]
pub fn discard_button(props: &EditorProps) -> Html {
    let (local_store, local_dispatch) = use_store::<LocalStore>();
    let (locales_store, _) = use_store::<LocalesStore>();
    let (_, modal_dispatch) = use_store::<ModalStore>();
    let navigator = use_navigator().unwrap();
    let (Some(resid), Some(lang)) = (&props.resid, &props.lang) else {
        return html! {};
    };
    let is_new = match &props.state {
        EditorState::Ok(Some(state)) => state.is_new,
        _ => {
            return html! {};
        }
    };
    if local_store.get(resid, lang.key()).is_none() {
        return html! {};
    }
    let onstatechanged = &props.onstatechanged;
    let onclick = async_event!(
        |onstatechanged, resid, lang, local_dispatch, is_new, navigator| {
            local_dispatch.reduce_mut(|store| {
                store.remove(&resid, lang.key());
            });
            if is_new.unwrap_or_default() {
                navigator.push(&Route::Editor);
            }
            onstatechanged.emit(EditorState::Ok(None));
        }
    );
    let onclick = show_modal_callback(
        ModalData::Dialog(DialogData {
            title: locales_store.get(TK::DiscardChanges),
            message: locales_store.get(TK::DiscardChangesQuestion),
            buttons: Buttons::RiskyCancel(
                ModalButton::new(locales_store.get(TK::Discard), Some(onclick)),
                ModalButton::new(locales_store.get(TK::Cancel), None),
            ),
        }),
        modal_dispatch.clone(),
    );

    html! {
        <button class={"btn btn-warning grow"} {onclick}>
            {locales_store.get(TK::Discard)}
        </button>
    }
}
