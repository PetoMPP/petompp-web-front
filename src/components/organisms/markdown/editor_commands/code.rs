use super::command::{decorate_selection, is_selection_empty, is_selection_valid, EditorCommand};
use crate::components::atoms::modal::ModalStore;
use deref_derive::Deref;
use yew::prelude::*;
use yewdux::dispatch::Dispatch;

#[derive(Deref)]
pub struct CodeCommand(String);

impl EditorCommand for CodeCommand {
    fn create(target: &str) -> Self {
        Self(target.to_string())
    }
    fn img(&self) -> &str {
        "/img/ui/code.svg"
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
            cb.emit(decorate_selection(&id, "`"));
        })
    }
}

#[derive(Deref)]
pub struct CodeBlockCommand(String);

impl EditorCommand for CodeBlockCommand {
    fn create(target: &str) -> Self {
        Self(target.to_string())
    }
    fn img(&self) -> &str {
        "/img/ui/code-block.svg"
    }
    fn can_do(&self) -> bool {
        !is_selection_empty((*self).clone())
    }
    fn command(
        &self,
        cb: Callback<String>,
        _modal_dispatch: Dispatch<ModalStore>,
    ) -> Callback<Event> {
        let id = (*self).clone();
        Callback::from(move |_| {
            cb.emit(decorate_selection(&id, ("```\n", "\n```")));
        })
    }
}
