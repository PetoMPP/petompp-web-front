use crate::components::atoms::modal::ModalStore;
use crate::components::organisms::markdown::editor_commands::command::get_commands;
use crate::hooks::event::use_event;
use crate::pages::editor::EditorData;
use crate::utils::js::{get_textarea, set_textarea_height, set_textarea_text};
use crate::utils::style::get_svg_bg_mask_style;
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yewdux::prelude::*;

const TEXTAREA_ID: &str = "editor-textarea";

#[derive(Clone, PartialEq, Properties)]
pub struct MarkdownEditorProps {
    pub state: EditorData,
    pub onchanged: Callback<EditorData>,
}

#[function_component(MarkdownEditor)]
pub fn markdown_editor(props: &MarkdownEditorProps) -> Html {
    use_effect_with_deps(
        move |initial_state| {
            set_textarea_text(initial_state.to_string().as_str(), TEXTAREA_ID);
        },
        props.state.clone(),
    );
    let oninput = {
        let onchanged = props.onchanged.clone();
        let state = props.state.clone();
        Callback::from(move |e: InputEvent| {
            let element: HtmlInputElement = e.target_unchecked_into();
            let value = element.value();
            let state = state.clone();
            onchanged.emit(state.with_string(value));
            set_textarea_height(&element);
        })
    };
    let onchanged = {
        let onchanged = props.onchanged.clone();
        let state = props.state.clone();
        Callback::from(move |new_value: String| {
            onchanged.emit(state.clone().with_string(new_value));
        })
    };
    html! {
        <div class={"border border rounded-2xl shadow-2xl"}>
            <EditorCommands {onchanged} />
            <textarea id={TEXTAREA_ID} {oninput} class={"w-full font-mono bg-base-100 outline-none p-4 rounded-lg overflow-hidden resize-none leading-normal"}></textarea>
        </div>
    }
}

#[derive(Clone, PartialEq, Properties)]
struct EditorCommandsProps {
    pub onchanged: Callback<String>,
}

#[function_component(EditorCommands)]
fn editor_commands(props: &EditorCommandsProps) -> Html {
    let (_, modal_dispatch) = use_store::<ModalStore>();
    let commands = use_state(Vec::new);
    let selection = use_state(|| 0);
    {
        let selection = selection.clone();
        use_event(&get_textarea(TEXTAREA_ID), "select", move |_| {
            gloo::console::log!("Selection changed");
            selection.set(*selection + 1);
        });
    }
    {
        let selection = selection.clone();
        use_event(
            &web_sys::window().unwrap().document().unwrap(),
            "click",
            move |_| {
                gloo::console::log!("Selection changed");
                selection.set(*selection + 1);
            },
        );
    }
    {
        let commands = commands.clone();
        use_effect_with_deps(
            move |(onchanged, _)| {
                gloo::console::log!("recomputing commands");
                commands.set(
                    get_commands(TEXTAREA_ID)
                        .into_iter()
                        .map(|c| {
                            (
                                c.img().to_string(),
                                c.can_do(),
                                c.command(onchanged.clone(), modal_dispatch.clone()),
                            )
                        })
                        .collect::<Vec<_>>(),
                );
            },
            (props.onchanged.clone(), selection.clone()),
        );
    }
    let commands = commands.iter().cloned().map(|(img, cd, onclick)| {
        let mut class = classes!("btn", "btn-sm", "btn-primary", "btn-square");
        let mut inner_class = classes!("h-8", "w-8");
        match cd {
            true => inner_class.push("bg-primary-content"),
            false => {
                class.push("btn-disabled");
                inner_class.push("bg-base-content");
            }
        }
        let onclick = Callback::from(move |e: MouseEvent| {
            onclick.emit(e.into());
            let textarea = get_textarea(TEXTAREA_ID);
            textarea.focus().unwrap();
        });
        html! {
            <button {class} {onclick}>
                <div class={inner_class} style={get_svg_bg_mask_style(&img)}/>
            </button>
        }
    });
    html! {
        <div class={"bg-base-200 rounded-t-2xl flex flex-row flex-wrap gap-4 lg:justify-evenly p-2 w-full border-b"}>
            {for commands}
        </div>
    }
}
