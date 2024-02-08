use super::bold::BoldCommand;
use crate::{
    components::{
        atoms::modal::ModalStore,
        organisms::markdown::editor_commands::{
            code::{CodeBlockCommand, CodeCommand},
            image::ImageCommand,
            italic::ItalicCommand,
            link::LinkCommand,
            quote::QuoteCommand,
            strikethrough::StrikethroughCommand,
            underline::UnderlineCommand,
        },
    },
    utils::js::{get_selection, get_textarea},
};
use yew::prelude::*;
use yewdux::dispatch::Dispatch;

pub trait EditorCommand {
    fn create(target: &str) -> Self
    where
        Self: Sized;
    fn img(&self) -> &str;
    fn can_do(&self) -> bool;
    fn command(
        &self,
        cb: Callback<String>,
        modal_dispatch: Dispatch<ModalStore>,
    ) -> Callback<Event>;
}

pub fn get_commands(target: &str) -> Vec<Box<dyn EditorCommand>> {
    vec![
        Box::new(BoldCommand::create(target)),
        Box::new(ItalicCommand::create(target)),
        Box::new(UnderlineCommand::create(target)),
        Box::new(LinkCommand::create(target)),
        Box::new(ImageCommand::create(target)),
        Box::new(CodeCommand::create(target)),
        Box::new(CodeBlockCommand::create(target)),
        Box::new(StrikethroughCommand::create(target)),
        Box::new(QuoteCommand::create(target)),
    ]
}

pub fn is_selection_empty(target: impl Into<String>) -> bool {
    let textarea = get_textarea(&target.into());
    let (s, e) = get_selection(&textarea);
    s == e
}

/// Valid selection is a selection that is not empty and does not start or end with whitespace
pub fn is_selection_valid(target: impl Into<String>) -> bool {
    let textarea = get_textarea(&target.into());
    let (s, e) = get_selection(&textarea);
    if s == e {
        return false;
    }
    let value = textarea
        .value()
        .encode_utf16()
        .skip(s)
        .take(e - s)
        .collect::<Vec<_>>();
    let value = String::from_utf16(&value).unwrap();
    !value.starts_with(char::is_whitespace) && !value.ends_with(char::is_whitespace)
}

pub enum Decorator<'a> {
    Simple(&'a str),
    Complex(&'a str, &'a str),
}

impl<'a> From<&'a str> for Decorator<'a> {
    fn from(s: &'a str) -> Self {
        Decorator::Simple(s)
    }
}

impl<'a> From<(&'a str, &'a str)> for Decorator<'a> {
    fn from((pre, post): (&'a str, &'a str)) -> Self {
        Decorator::Complex(pre, post)
    }
}

impl Decorator<'_> {
    fn decorate(&self, pre: &str, sel: &str, post: &str) -> String {
        match self {
            Decorator::Simple(decorator) => {
                format!("{}{}{}{}{}", pre, decorator, sel, decorator, post)
            }
            Decorator::Complex(pre_decor, post_decor) => {
                format!("{}{}{}{}{}", pre, pre_decor, sel, post_decor, post)
            }
        }
    }
}

pub fn decorate_selection<'a, D: Into<Decorator<'a>>>(
    target: impl Into<String>,
    decorator: D,
) -> String {
    let element = get_textarea(&target.into());
    let value = element.value();
    let (sel_start, sel_end) = get_selection(&element);
    let pre = value.encode_utf16().take(sel_start).collect::<Vec<_>>();
    let pre = String::from_utf16(&pre).unwrap();
    let selected = value
        .encode_utf16()
        .skip(sel_start)
        .take(sel_end - sel_start)
        .collect::<Vec<_>>();
    let selected = String::from_utf16(&selected).unwrap();
    let post = value.encode_utf16().skip(sel_end).collect::<Vec<_>>();
    let post = String::from_utf16(&post).unwrap();

    decorator.into().decorate(&pre, &selected, &post)
}

pub fn insert_before_selection(target: impl Into<String>, text: &str) -> String {
    let element = get_textarea(&target.into());
    let value = element.value();
    let (sel_start, _) = get_selection(&element);
    let pre = value.encode_utf16().take(sel_start).collect::<Vec<_>>();
    let pre = String::from_utf16(&pre).unwrap();
    let post = value.encode_utf16().skip(sel_start).collect::<Vec<_>>();
    let post = String::from_utf16(&post).unwrap();

    format!("{}{}{}", pre, text, post)
}

pub fn insert_after_selection(target: impl Into<String>, text: &str) -> String {
    let element = get_textarea(&target.into());
    let value = element.value();
    let (_, sel_end) = get_selection(&element);
    let pre = value.encode_utf16().take(sel_end).collect::<Vec<_>>();
    let pre = String::from_utf16(&pre).unwrap();
    let post = value.encode_utf16().skip(sel_end).collect::<Vec<_>>();
    let post = String::from_utf16(&post).unwrap();

    format!("{}{}{}", pre, text, post)
}
