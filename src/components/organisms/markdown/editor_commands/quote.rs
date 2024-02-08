use super::command::{insert_before_selection, EditorCommand};
use crate::components::atoms::modal::ModalStore;
use deref_derive::Deref;
use yew::prelude::*;
use yewdux::dispatch::Dispatch;

#[derive(Deref)]
pub struct QuoteCommand(String);

impl EditorCommand for QuoteCommand {
    fn create(target: &str) -> Self {
        Self(target.to_string())
    }
    fn img(&self) -> &str {
        "/img/ui/quote.svg"
    }
    fn can_do(&self) -> bool {
        true
    }
    fn command(
        &self,
        cb: Callback<String>,
        _modal_dispatch: Dispatch<ModalStore>,
    ) -> Callback<Event> {
        let id = (*self).clone();
        Callback::from(move |_| {
            cb.emit(insert_before_selection(&id, "> "));
        })
    }
}
