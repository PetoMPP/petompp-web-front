use crate::{data::locales::{LocalesStore, TK}, utils::style::get_svg_bg_mask_style};
use chrono::{DateTime, Local, Utc};
use yew::prelude::*;
use yewdux::prelude::*;

#[derive(Clone, PartialEq, Properties)]
pub struct DateProps {
    pub date: DateTime<Utc>,
}

#[function_component(DateDisplay)]
pub fn date_display(props: &DateProps) -> Html {
    let (locales_store, _) = use_store::<LocalesStore>();
    let formatter = timeago::Formatter::with_language(locales_store.curr);
    let display = formatter.convert_chrono(props.date, Utc::now());
    html! {
        <time class={"italic"} datetime={props.date.to_string()}>{display}</time>
    }
}

#[function_component(CreatedDateDisplay)]
pub fn created_date_display(props: &DateProps) -> Html {
    let (locales_store, _) = use_store::<LocalesStore>();
    let date = DateTime::<Local>::from(props.date.clone());
    let tooltip = format!("{}: {}", locales_store.get(TK::Created), date.format("%Y-%m-%d %H:%M:%S"));
    html! {
        <div class={"tooltip"} data-tip={tooltip}>
            <div class={"flex flex-row items-center text-sm opacity-60"}>
                <div class={"bg-secondary h-6 w-6"} style={get_svg_bg_mask_style("/img/ui/created.svg")}/>
                <DateDisplay date={props.date} />
            </div>
        </div>
    }
}

#[function_component(UpdatedDateDisplay)]
pub fn updated_date_display(props: &DateProps) -> Html {
    let (locales_store, _) = use_store::<LocalesStore>();
    let date = DateTime::<Local>::from(props.date.clone());
    let tooltip = format!("{}: {}", locales_store.get(TK::Updated), date.format("%Y-%m-%d %H:%M:%S"));
    html! {
        <div class={"tooltip"} data-tip={tooltip}>
            <div class={"flex flex-row items-center text-sm opacity-60"}>
                <div class={"bg-accent h-6 w-6"} style={get_svg_bg_mask_style("/img/ui/updated.svg")}/>
                <DateDisplay date={props.date} />
            </div>
        </div>
    }
}
