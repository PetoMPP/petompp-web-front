use crate::{
    api::{
        blob::BlobClient,
        client::{ApiClient, RequestError},
    },
    async_event,
    components::{
        atoms::{
            loading::Loading,
            modal::{show_modal_callback, Buttons, DialogData, ModalButton, ModalData, ModalStore},
        },
        state::State,
    },
    data::{
        blob::BlobStore,
        locales::{store::LocalesStore, tk::TK},
        session::SessionStore,
    },
    hooks::event::use_event,
    utils::{ext::Mergable, style::get_svg_bg_mask_style},
};
use petompp_web_models::models::blob::blob_meta::BlobUpload;
use std::path::Path;
use wasm_bindgen::JsCast;
use web_sys::{HtmlElement, HtmlImageElement, HtmlInputElement, Node};
use yew::{platform::spawn_local, prelude::*};
use yewdux::prelude::*;

#[derive(Clone, Properties, PartialEq)]
pub struct BlobImageSelectProps {
    pub container: String,
    pub id: Option<String>,
    pub folder: Option<String>,
    pub data: Option<String>,
    pub ondatachanged: Callback<Option<String>>,
}

#[function_component(BlobImageSelect)]
pub fn blob_image_select(props: &BlobImageSelectProps) -> Html {
    let (locales_store, _) = use_store::<LocalesStore>();
    let id = use_memo(
        |id| {
            id.clone().unwrap_or(
                web_sys::window().unwrap().crypto().unwrap().random_uuid()[..10].to_string(),
            )
        },
        props.id.clone(),
    );
    let force_open = use_state(|| false);
    let src = <ApiClient as BlobClient>::get_url(
        &props.container,
        format!(
            "{}{}",
            props.folder.clone().unwrap_or_default(),
            props.data.clone().unwrap_or_default()
        )
        .as_str(),
    );
    let mut dropdown_class = classes!(
        "dropdown",
        "dropdown-top",
        "w-full",
        "input",
        "input-bordered",
        "shadow-md",
        "grid",
        "grid-cols-auto-2",
        "items-center",
        "justify-between",
        "px-0"
    );
    if *force_open {
        dropdown_class.push("dropdown-open");
    }
    let onforceopenchanged = {
        let force_open = force_open.clone();
        Callback::from(move |fo| force_open.set(fo))
    };
    let config = BlobBrowserDialogConfig {
        readonly: false,
        container: props.container.clone(),
        root: props.folder.clone(),
    };
    html! {
        <div class={"flex flex-col gap-2"}>
            <div class={"border p-2 rounded-lg shadow-md w-full lg:max-h-[35%]"}>
                <img {src} class={"h-auto mx-auto"}/>
            </div>
            <div class={"w-full"}>
                <div id={(*id).clone()} tabindex={"0"} class={dropdown_class}>
                    <div class={"pl-2 truncate"}>{props.data.clone().unwrap_or_default()}</div>
                    <label class={"rounded-l-none btn btn-primary no-animation"} tabindex={"0"}>{locales_store.get(TK::Edit)}</label>
                    <div tabindex={"0"} class={"dropdown-content w-full flex flex-col mb-4 gap-1 z-10"}>
                        <BlobBrowserDialog parentid={(*id).clone()} {config} ondatachanged={props.ondatachanged.clone()} {onforceopenchanged} />
                    </div>
                </div>
            </div>
        </div>
    }
}

#[derive(Clone, PartialEq, Default)]
pub struct BlobBrowserDialogConfig {
    pub readonly: bool,
    pub container: String,
    pub root: Option<String>,
}

#[derive(Clone, Properties, PartialEq)]
pub struct BlobBrowserDialogProps {
    pub parentid: String,
    pub config: BlobBrowserDialogConfig,
    pub ondatachanged: Callback<Option<String>>,
    pub onforceopenchanged: Callback<bool>,
}

#[function_component(BlobBrowserDialog)]
pub fn blob_browser_dialog(props: &BlobBrowserDialogProps) -> Html {
    let (session_store, session_dispatch) = use_store::<SessionStore>();
    let (blob_store, blob_dispatch) = use_store::<BlobStore>();
    let (locales_store, _) = use_store::<LocalesStore>();
    let (_, modal_dispatch) = use_store::<ModalStore>();
    let container = props.config.container.clone();
    let prefix = props.config.root.clone();
    let curr = use_state(|| "/".to_string());
    let selected = use_state(|| None);
    let props = props.clone();
    let state = use_state(|| State::Ok(None));
    let dir_input_active = use_state(|| false);
    let onforceopenchanged = props.onforceopenchanged.clone();
    {
        let parentid = props.parentid.clone();
        use_event(
            &web_sys::window().and_then(|w| w.document()).unwrap(),
            "click",
            move |e| {
                if let Some(element) = web_sys::window()
                    .and_then(|w| w.document())
                    .and_then(|d| d.get_element_by_id(parentid.as_str()))
                {
                    let node = e.target_dyn_into::<Node>();
                    if element.contains(node.as_ref()) {
                        return;
                    }
                }
                onforceopenchanged.emit(false);
            },
        );
    }
    {
        let state = state.clone();
        use_effect_with_deps(move |_| state.set(State::Ok(None)), blob_store)
    }
    use_effect_with_deps(
        |(state, container, prefix)| {
            let state = state.clone();
            let container = container.clone();
            let prefix = prefix.clone();
            match &*state {
                State::Ok(Some(_)) | State::Loading | State::Err(_) => (),
                _ => {
                    spawn_local(async move {
                        let pf = prefix.clone().unwrap_or_default();
                        match ApiClient::get_names(&container, prefix.as_deref()).await {
                            Ok(p) => state.set(State::Ok(Some(
                                p.into_iter()
                                    .map(|p| {
                                        format!(
                                            "/{}",
                                            p.strip_prefix(pf.as_str())
                                                .map(|s| s.to_string())
                                                .unwrap_or(p)
                                        )
                                    })
                                    .collect::<Vec<_>>(),
                            ))),
                            Err(RequestError::Endpoint(404, _)) => {
                                state.set(State::Ok(Some(Vec::new())))
                            }
                            Err(e) => state.set(State::Err(e)),
                        }
                    });
                }
            }
        },
        (state.clone(), container.clone(), prefix.clone()),
    );
    {
        let curr = curr.clone();
        let parentid = props.parentid.clone();
        use_effect_with_deps(
            |(_, _, parentid)| {
                if let Some(browser) = web_sys::window()
                    .and_then(|w| w.document())
                    .and_then(|d| d.get_element_by_id(parentid.as_str()))
                    .and_then(|e| e.dyn_into::<HtmlElement>().ok())
                {
                    browser.focus().unwrap();
                }
            },
            (curr.clone(), selected.clone(), parentid.clone()),
        )
    }
    let onselectedchanged = {
        let selected = selected.clone();
        Callback::from(move |name: Option<_>| selected.set(name))
    };
    let onpathchanged = {
        let curr = curr.clone();
        Callback::from(move |path: String| match path.as_str() {
            ".." => {
                let path = curr
                    .trim_end_matches('/')
                    .rsplit_once('/')
                    .map(|(r, _)| r.to_string())
                    .unwrap_or(curr.to_string());
                curr.set(format!("{}/", path));
            }
            _ => curr.set(format!("{}{}/", &*curr, path)),
        })
    };
    let paths = match (*state).clone() {
        State::Ok(Some(paths)) => {
            let mut used = Vec::new();
            let curr = curr.clone();
            let selected = selected.clone();
            let onselectedchanged = onselectedchanged.clone();
            let onpathchanged = onpathchanged.clone();
            let mut items = {
                let curr = curr.clone();
                paths
                    .iter()
                    .filter_map(move |p| {
                        let curr = curr.clone();
                        let pt = p.strip_prefix(&*curr)?;
                        let Some((root, _rest)) = pt.split_once('/') else {
                            return Some(BrowseItem::File(pt.to_string()));
                        };
                        if !used.contains(&root) {
                            used.push(root);
                            return Some(BrowseItem::Dir(root.to_string()));
                        }
                        None
                    })
                    .collect::<Vec<_>>()
            };
            items.sort();
            if curr.as_str() != "/" {
                let mut first = vec![BrowseItem::Dir("..".to_string())];
                first.extend(items);
                items = first;
            }
            items
                .into_iter()
                .map(move |item| {
                    html! {
                        <BrowseListItem
                            selected={(*selected).clone()}
                            {item}
                            onselectedchanged={onselectedchanged.clone()}
                            onpathchanged={onpathchanged.clone()} />
                    }
                })
                .collect()
        }
        State::Ok(None) | State::Loading => html! {<Loading />},
        State::Err(_) => html! {},
    };
    let src = match selected.as_ref() {
        Some(BrowseItem::File(name)) => <ApiClient as BlobClient>::get_url(
            &container,
            format!(
                "{}{}{}",
                prefix.clone().unwrap_or_default(),
                &curr[1..],
                name
            )
            .as_str(),
        ),
        _ => "img/placeholder.svg".to_string(),
    };
    let close_onclick = {
        let props = props.clone();
        Callback::from(move |_| props.ondatachanged.emit(None))
    };
    let select_onclick = {
        let selected = selected.clone();
        let props = props.clone();
        let curr = curr.clone();
        Callback::from(move |_| {
            let val = selected.as_ref().and_then(|s| match s {
                BrowseItem::Dir(_) => None,
                BrowseItem::File(name) => Some(format!("{}{}", &curr.as_str()[1..], name)),
            });
            props.ondatachanged.emit(val)
        })
    };
    let mut select_class = classes!("btn", "btn-sm");
    match selected.as_ref() {
        Some(BrowseItem::File(_)) => select_class.push("btn-success"),
        _ => select_class.push("btn-disabled"),
    };
    let enable_force_open = {
        let onforceopenchanged = props.onforceopenchanged.clone();
        Callback::from(move |_| onforceopenchanged.emit(true))
    };
    let disable_force_open = {
        let onforceopenchanged = props.onforceopenchanged.clone();
        Callback::from(move |_| onforceopenchanged.emit(false))
    };
    let add_img_onclick = enable_force_open.clone();
    let input_class = classes!("flex", "w-12", "grow", "outline-none", "bg-transparent");
    let mut dir_input_class = input_class.clone();
    if !*dir_input_active {
        dir_input_class.push("hidden");
    }
    let add_dir_onclick = {
        let dir_input_active = dir_input_active.clone();
        Callback::from(move |_| {
            dir_input_active.set(!*dir_input_active);
        })
    };
    let dir_id = use_memo(
        |_| web_sys::window().unwrap().crypto().unwrap().random_uuid()[..10].to_string(),
        (),
    );
    use_effect_with_deps(
        |(dir_input_active, dir_id)| {
            if **dir_input_active {
                if let Some(dir_input) = web_sys::window()
                    .and_then(|w| w.document())
                    .and_then(|d| d.get_element_by_id(dir_id.as_str()))
                    .and_then(|e| e.dyn_into::<HtmlInputElement>().ok())
                {
                    dir_input.focus().unwrap();
                    dir_input.select();
                }
            }
        },
        (dir_input_active.clone(), dir_id.clone()),
    );
    let add_dir_onkeydown = {
        let curr = curr.clone();
        Callback::from(move |e: KeyboardEvent| match e.key().as_str() {
            "Enter" => {
                e.prevent_default();
                e.stop_propagation();
                let element = e.target_unchecked_into::<HtmlInputElement>();
                let dir = Path::new(&element.value()).display().to_string();
                element.set_value("");
                if dir.is_empty() {
                    return;
                }
                dir_input_active.set(false);
                curr.set(format!("{}{}/", &*curr, dir));
            }
            "/" | "\\" | "." | " " => {
                e.prevent_default();
            }
            _ => {}
        })
    };
    let oninput = {
        let state = state.clone();
        let curr = curr.clone();
        let prefix = prefix.clone();
        let container = container.clone();
        let blob_dispatch = blob_dispatch.clone();
        let session_store = session_store.clone();
        Callback::from(move |e: InputEvent| {
            let state = state.clone();
            let curr = curr.clone();
            let prefix = prefix.clone();
            let container = container.clone();
            let session_store = session_store.clone();
            let blob_dispatch = blob_dispatch.clone();
            let element = e.target_unchecked_into::<HtmlInputElement>();
            let img = element.files().unwrap().get(0).unwrap();
            state.set(State::Loading);
            spawn_local(async move {
                let token = session_store.token.clone().unwrap_or_default();
                let mut upload = match BlobUpload::from_file(&img).await {
                    Ok(u) => u,
                    Err(e) => {
                        state.set(State::Err(RequestError::Network(e.to_string())));
                        return;
                    }
                };
                upload.meta.filename = format!(
                    "{}{}{}",
                    prefix.unwrap_or_default(),
                    &curr.as_str()[1..],
                    upload.meta.filename
                );
                match ApiClient::create_or_update(token.as_str(), &container, &upload).await {
                    Ok(_) => {
                        state.set(State::Ok(None));
                        blob_dispatch.reduce_mut(|bs| bs.invalidate());
                    }
                    Err(e) => {
                        state.set(State::Err(e));
                    }
                }
            });
        })
    };
    let (delete_class, delete_icon) = match &*selected {
        Some(b) => (
            "btn btn-xs btn-error",
            match b {
                BrowseItem::Dir(_) => Some(
                    html! {<div class={"bg-error-content h-5 w-5"} style={get_svg_bg_mask_style("/img/ui/folder-del.svg")}/>},
                ),
                BrowseItem::File(_) => Some(
                    html! {<div class={"bg-error-content h-5 w-5"} style={get_svg_bg_mask_style("/img/ui/file-del.svg")}/>},
                ),
            },
        ),
        None => ("hidden", None),
    };
    let onforceopenchanged = &props.onforceopenchanged;
    let go_up = match &*state {
        State::Ok(Some(paths)) => paths.len() == 1,
        _ => false,
    };
    let delete_onclick = async_event!(|onforceopenchanged,
                                       selected,
                                       session_store,
                                       curr,
                                       container,
                                       prefix,
                                       state,
                                       go_up,
                                       blob_dispatch| {
        let path = match &*selected {
            Some(BrowseItem::Dir(path) | BrowseItem::File(path)) => path,
            _ => return,
        };
        let token = session_store.token.clone().unwrap_or_default();
        match ApiClient::delete(
            &token,
            &container,
            format!(
                "{}{}{}",
                prefix.unwrap_or_default(),
                &curr.as_str()[1..],
                path
            )
            .as_str(),
        )
        .await
        {
            Ok(_) => {
                if go_up {
                    curr.set("/".to_string());
                }
                state.set(State::Ok(None));
                blob_dispatch.reduce_mut(|bs| bs.invalidate());
            }
            Err(e) => state.set(State::Err(e)),
        }
        selected.set(None);
        onforceopenchanged.emit(false);
    });
    let title_message = match &*selected {
        Some(BrowseItem::Dir(_)) => Some((TK::DeleteDir, TK::DeleteDirQuestion)),
        Some(BrowseItem::File(_)) => Some((TK::DeleteFile, TK::DeleteFileQuestion)),
        None => None,
    };
    let delete_onclick = match title_message {
        Some((title, message)) => enable_force_open.clone().merge(show_modal_callback(
            ModalData::Dialog(DialogData {
                title,
                message,
                buttons: Buttons::RiskyCancel(
                    ModalButton::new(TK::Delete, Some(delete_onclick)),
                    ModalButton::new(TK::Cancel, Some(disable_force_open)),
                ),
            }),
            modal_dispatch,
        )),
        None => Callback::noop(),
    };
    let buttons = match &*state {
        State::Ok(_) => {
            let add_dir = match props.config.readonly {
                true => None,
                false => Some(html! {
                    <button class={"btn btn-xs btn-square btn-primary"} onclick={add_dir_onclick}>
                        <div class={"bg-primary-content h-5 w-5"} style={get_svg_bg_mask_style("/img/ui/folder-add.svg")}/>
                    </button>
                }),
            };
            html! {
                <>
                <button class={delete_class} onclick={delete_onclick}>{delete_icon}</button>
                {add_dir}
                <label class={"btn btn-xs btn-square btn-primary"} onclick={add_img_onclick}>
                    <div class={"bg-primary-content h-5 w-5"} style={get_svg_bg_mask_style("/img/ui/file-add.svg")}/>
                    <input {oninput} accept={"image/*"} type={"file"} class={"hidden"} />
                </label>
                </>
            }
        }
        State::Loading => html! {<Loading />},
        State::Err(e) => {
            if let Err(redirect) = e.handle_failed_auth(session_dispatch) {
                return redirect;
            }
            let state = state.clone();
            let onclick = Callback::from(move |_| {
                state.set(State::Ok(None));
            });
            html! {
                <>
                <p>{e.to_string()}</p>
                <div class={"btn btn-xs btn-secondary"} {onclick}>
                    <div class={"bg-primary-content h-5 w-5"} style={get_svg_bg_mask_style("/img/ui/reload.svg")}/>
                </div>
                </>
            }
        }
    };
    let img_onclick = Callback::from(|e: MouseEvent| {
        let element = e.target_unchecked_into::<HtmlImageElement>();
        web_sys::window()
            .unwrap()
            .open_with_url(&element.src())
            .unwrap();
    });
    let ok_cancel_buttons = match props.config.readonly {
        true => None,
        false => Some(html! {
            <div class={"flex flex-row gap-2 w-full justify-end"}>
                <button class={select_class} onclick={select_onclick}>{locales_store.get(TK::Ok)}</button>
                <button class={"btn btn-sm btn-warning"} onclick={close_onclick}>{locales_store.get(TK::Cancel)}</button>
            </div>
        }),
    };
    html! {
        <div class={"bg-base-200 border border-2 rounded-md p-2 shadow-lg"}>
            <div class={"lg:max-h-[18rem] flex flex-col"}>
                <div class={"flex flex-wrap gap-2 p-2 w-full lg:items-center"}>
                    <div class={"flex border rounded-md px-2 italic bg-base-100 grow"}>
                        <p class={"word-break"}>{&*curr}</p>
                        <input id={(*dir_id).clone()} enterkeyhint={"done"} placeholder={locales_store.get(TK::EnterDirname)} class={dir_input_class} onkeydown={add_dir_onkeydown}/>
                    </div>
                    <div class={"flex flex-row gap-2 justify-between"}>
                        {buttons}
                    </div>
                </div>
                <div class={"divider my-1 lg:divider-horizontal lg:mx-1 lg:my-auto h-auto"}/>
                <div class={"flex flex-col lg:flex-row grow p-2 min-h-full"}>
                    <div class={"flex grow flex-col gap-1 overflow-y-scroll overflow-x-hidden lg:min-h-[auto] min-h-[6rem]"}>
                        {paths}
                    </div>
                    <div class={"divider my-1 lg:divider-horizontal lg:mx-1 lg:my-auto h-auto"}/>
                    <div class={"border p-2 rounded-lg shadow-md w-full lg:max-w-[50%] max-h-full"}>
                        <img class={"m-auto lg:h-full h-24 cursor-pointer"} {src} onclick={img_onclick}/>
                    </div>
                </div>
                {ok_cancel_buttons}
            </div>
        </div>
    }
}

#[derive(Clone, PartialEq, PartialOrd, Ord, Eq)]
enum BrowseItem {
    Dir(String),
    File(String),
}

#[derive(Clone, Properties, PartialEq)]
struct BrowseListItemProps {
    pub item: BrowseItem,
    pub selected: Option<BrowseItem>,
    pub onselectedchanged: Callback<Option<BrowseItem>>,
    pub onpathchanged: Callback<String>,
}

#[function_component(BrowseListItem)]
fn browse_item(props: &BrowseListItemProps) -> Html {
    let mut class = classes!("btn", "btn-xs", "btn-outline");
    let selected = props.selected.as_ref() == Some(&props.item);
    if selected {
        class.push("btn-active");
    }
    let onselectedchanged = props.onselectedchanged.clone();
    match &props.item {
        BrowseItem::Dir(item) => {
            let onclick = {
                let item = item.clone();
                let props = props.clone();
                let onpathchanged = props.onpathchanged.clone();
                Callback::from(move |_| match item == ".." || selected {
                    true => {
                        onpathchanged.emit(item.clone());
                        onselectedchanged.emit(None);
                    }
                    false => onselectedchanged.emit(Some(props.item.clone())),
                })
            };
            class.push("btn-accent");
            html! {<a {onclick} {class}>{item}</a>}
        }
        BrowseItem::File(item) => {
            let onclick = {
                let item = props.item.clone();
                match selected {
                    false => Callback::from(move |_| onselectedchanged.emit(Some(item.clone()))),
                    true => Callback::from(move |_| onselectedchanged.emit(None)),
                }
            };
            class.push("btn-secondary");
            html! {<a {onclick} {class}>{item}</a>}
        }
    }
}
