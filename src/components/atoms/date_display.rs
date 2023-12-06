use crate::{
    data::locales::{store::LocalesStore, tk::TK},
    utils::style::get_svg_bg_mask_style,
};
use chrono::{DateTime, Local, Utc};
use yew::prelude::*;
use yewdux::prelude::*;

#[derive(Clone, PartialEq, Properties)]
pub struct DateDisplayProps {
    pub date: DateTime<Utc>,
    pub tooltip: Option<String>,
}

#[function_component(DateDisplay)]
pub fn date_display(props: &DateDisplayProps) -> Html {
    let (locales_store, _) = use_store::<LocalesStore>();
    let formatter = timeago::Formatter::with_language(locales_store.curr);
    let display = formatter.convert_chrono(props.date, Utc::now());
    html! {
        <time class={"tooltip lg:tooltip-bottom tooltip-right italic"} data-tip={props.tooltip.clone()} datetime={props.date.to_string()}>{display}</time>
    }
}

#[derive(Clone, PartialEq, Properties)]
pub struct DateProps {
    pub date: DateTime<Utc>,
}

#[function_component(CreatedDateDisplay)]
pub fn created_date_display(props: &DateProps) -> Html {
    let (locales_store, _) = use_store::<LocalesStore>();
    let date = DateTime::<Local>::from(props.date);
    let tooltip = format!(
        "{}: {}",
        locales_store.get(TK::Created),
        date.format("%Y-%m-%d %H:%M:%S")
    );
    let onclick = Callback::from(move |e: MouseEvent| {
        e.set_cancel_bubble(true);
        e.stop_propagation();
    });
    html! {
        <div {onclick} class={"flex flex-row items-center text-sm text-base-content text-opacity-60"}>
            <div class={"bg-secondary h-6 w-6"} style={get_svg_bg_mask_style("/img/ui/created.svg")}/>
            <DateDisplay date={props.date} {tooltip}/>
        </div>
    }
}

#[function_component(UpdatedDateDisplay)]
pub fn updated_date_display(props: &DateProps) -> Html {
    let (locales_store, _) = use_store::<LocalesStore>();
    let date = DateTime::<Local>::from(props.date);
    let tooltip = format!(
        "{}: {}",
        locales_store.get(TK::Updated),
        date.format("%Y-%m-%d %H:%M:%S")
    );
    let onclick = Callback::from(move |e: MouseEvent| {
        e.set_cancel_bubble(true);
        e.stop_propagation();
    });
    html! {
        <div {onclick} class={"flex flex-row items-center text-sm text-base-content text-opacity-60"}>
            <div class={"bg-accent h-6 w-6"} style={get_svg_bg_mask_style("/img/ui/updated.svg")}/>
            <DateDisplay date={props.date} {tooltip}/>
        </div>
    }
}
