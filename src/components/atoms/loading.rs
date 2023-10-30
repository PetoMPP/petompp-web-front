use yew::prelude::*;

#[derive(Clone, Properties, PartialEq)]
pub struct LoadingProps {
    pub resource: Option<String>,
}

#[function_component(Loading)]
pub fn loading(props: &LoadingProps) -> Html {
    let text = match &props.resource {
        Some(name) => format!("Loading {}...", name),
        None => "Loading...".to_string(),
    };
    html! {
        <div class={"w-full flex rounded-lg"}>
            <div class={"mx-auto flex flex-row gap-2 rounded-lg"}>
                <span class={"flex loading loading-ring loading-lg"}/>
                <p class={"flex text-base items-center"}>{text}</p>
            </div>
        </div>
    }
}