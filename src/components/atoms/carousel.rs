use crate::components::atoms::modal::{show_modal_callback, ImageData, ModalData, ModalStore};
use yew::prelude::*;
use yew_router::prelude::*;
use yewdux::prelude::*;

#[derive(Clone, Debug, PartialEq)]
pub struct Slide {
    pub src: String,
    pub title: String,
    pub summary: Option<String>,
    pub onclick: Option<Callback<MouseEvent>>,
}

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct CarouselProps {
    pub slides: Vec<Slide>,
}

impl Default for Slide {
    fn default() -> Self {
        Self {
            src: "".to_string(),
            title: "".to_string(),
            summary: None,
            onclick: None,
        }
    }
}

#[function_component(Carousel)]
pub fn carousel(props: &CarouselProps) -> Html {
    let (_, modal_dispatch) = use_store::<ModalStore>();
    let id_base = use_memo(
        |_| {
            format!(
                "#{}",
                &web_sys::window().unwrap().crypto().unwrap().random_uuid()[..8]
            )
        },
        (),
    );
    let location = use_location().unwrap();
    let curr = location.hash().to_string();
    let curr: usize = curr
        .trim_start_matches(id_base.as_str())
        .parse()
        .unwrap_or_default();
    let slide_cnt = props.slides.len();
    if slide_cnt == 0 {
        return html! {};
    }
    let slides = props.slides.iter().enumerate().map(|(i, s)| {
        let id_base = (*id_base)[1..].to_string();
        let onclick = s.onclick.clone();
        let id = format!("{}{}", id_base, i);
        let onclick = onclick.unwrap_or_else(|| {
                // toggle fullscreen callback
                let data = ModalData::Image(ImageData {
                    src: s.src.clone(),
                    title: s.title.clone(),
                });
                show_modal_callback(data, modal_dispatch.clone())
        });
        html! {
            <div {id} class={"carousel-item w-full cursor-pointer"} {onclick}>
                <img class={"w-full object-cover"} src={s.src.clone()}/>
                <div class={"absolute left-[1rem] bottom-[0.5rem] p-2 pr-4 rounded-lg bg-base-100 bg-opacity-60 text-base-content font-semibold"}>
                    <h1 class={"text-3xl"}>{s.title.clone()}</h1>
                    <p class={"text-lg"}>{s.summary.clone().unwrap_or_default()}</p>
                </div>
            </div>
        }
    });
    let id_base = (*id_base).clone();
    let prev_id = format!(
        "{}{}",
        &id_base,
        match curr == 0 {
            true => slide_cnt - 1,
            false => curr - 1,
        }
    );
    let next_id = format!(
        "{}{}",
        &id_base,
        match slide_cnt - 1 == curr {
            true => 0,
            false => curr + 1,
        }
    );
    let no_bubble = Callback::from(move |e: MouseEvent| {
        e.stop_propagation();
    });

    html! {
        <div class={"relative"}>
            <div class={"carousel relative rounded-lg shadow-lg w-full h-96"}>
                {for slides}
            </div>
            <div class={"absolute flex justify-between transform -translate-y-1/2 left-5 right-5 top-1/2"}>
                <a onclick={no_bubble.clone()} href={prev_id} class={"btn btn-circle"}>{"❮"}</a>
                <a onclick={no_bubble.clone()} href={next_id} class={"btn btn-circle"}>{"❯"}</a>
            </div>
        </div>
    }
}
