use super::command::{decorate_selection, is_selection_valid, EditorCommand};
use crate::components::atoms::modal::ModalStore;
use deref_derive::Deref;
use yew::prelude::*;
use yewdux::dispatch::Dispatch;

#[derive(Deref)]
pub struct ItalicCommand(String);

impl EditorCommand for ItalicCommand {
    fn create(target: &str) -> Self {
        Self(target.to_string())
    }

    fn img(&self) -> &str {
        "/img/ui/italic.svg"
    }

    fn can_do(&self) -> bool {
        is_selection_valid((*self).clone())
    }

    fn command(
        &self,
        cb: Callback<String>,
        _modal_dispatch: Dispatch<ModalStore>,
    ) -> Callback<Event> {
        let id = (*self).clone();
        Callback::from(move |_| {
            cb.emit(decorate_selection(&id, "*"));
        })
    }
}
