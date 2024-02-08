use crate::{
    api::{client::ApiClient, user::UserClient},
    async_event,
    components::{
        atoms::{
            loading::Loading,
            modal::{show_modal_callback, Buttons, DialogData, ModalButton, ModalData, ModalStore},
        },
        state::State,
    },
    data::{
        locales::{store::LocalesStore, tk::TK},
        session::SessionStore,
    },
};
use petompp_web_models::models::user::UserData;
use yew::{platform::spawn_local, prelude::*, virtual_dom::VNode};
use yewdux::prelude::*;

#[function_component(UserManager)]
pub fn user_manager() -> Html {
    let (locales_store, _) = use_store::<LocalesStore>();
    let (session_store, session_dispatch) = use_store::<SessionStore>();
    let token = session_store.token.clone().unwrap_or_default();
    let data = use_state_eq(|| State::Ok(None));
    use_effect_with_deps(
        |data| {
            let data = data.clone();
            match &*data {
                State::Ok(Some(_)) | State::Loading | State::Err(_) => return,
                _ => data.set(State::Loading),
            };
            spawn_local(async move {
                match ApiClient::get_users(&token).await {
                    Ok(users) => data.set(State::Ok(Some(users))),
                    Err(error) => data.set(State::Err(error)),
                };
            })
        },
        data.clone(),
    );
    let reload = {
        let data = data.clone();
        Callback::from(move |_| data.set(State::Ok(None)))
    };
    let list = match &*data {
        State::Ok(Some(users)) => users
            .iter()
            .cloned()
            .map(|user| html! {<UserRow {user} reload={reload.clone()} />})
            .collect::<VNode>(),
        State::Loading | State::Ok(None) => html! {
            <Loading />
        },
        State::Err(e) => {
            if let Err(redirect) = e.handle_failed_auth(session_dispatch.clone()) {
                return redirect;
            }
            html! {
                <>
                <h3 class={"mx-auto py-4 text-xl font-semibold"}>{"Failed to load users!"}</h3>
                <p>{e.to_string()}</p>
                </>
            }
        }
    };
    if let State::Err(e) = &*data {
        if let Err(redirect) = e.handle_failed_auth(session_dispatch) {
            return redirect;
        }
    }
    html! {
        <table class={"table"}>
            <thead>
                <tr>
                    <th>{locales_store.get(TK::Id)}</th>
                    <th>{locales_store.get(TK::Name)}</th>
                    <th>{locales_store.get(TK::Actions)}</th>
                </tr>
            </thead>
            <tbody class={"items-center"}>
                {list}
            </tbody>
        </table>
    }
}

#[derive(Properties, Clone, PartialEq)]
pub struct UserRowProps {
    pub user: UserData,
    pub reload: Callback<()>,
}

#[function_component(UserRow)]
fn user_row(props: &UserRowProps) -> Html {
    html! {
        <tr>
            <td>{&props.user.id}</td>
            <td class={"break-all"}>{&props.user.name}</td>
            <td>
                <div class="flex flex-row">
                    <ActivateButton user={props.user.clone()} reload={props.reload.clone()} />
                    <DeleteButton user={props.user.clone()} reload={props.reload.clone()} />
                </div>
            </td>
        </tr>
    }
}

#[function_component(ActivateButton)]
fn activate_button(props: &UserRowProps) -> Html {
    let (locales_store, _) = use_store::<LocalesStore>();
    let (session_store, session_dispatch) = use_store::<SessionStore>();
    let (_, dispatch) = use_store::<ModalStore>();
    let error_state = use_state_eq(|| None);
    let token = session_store.token.clone().unwrap_or_default();
    let onclick = async_event!(|props, token, error_state| {
        match ApiClient::activate_user(&token, props.user.id).await {
            Ok(()) => props.reload.emit(()),
            Err(error) => error_state.set(Some(error)),
        }
    });
    if let Some(error) = &*error_state {
        if let Err(redirect) = error.handle_failed_auth(session_dispatch) {
            return redirect;
        }
    }
    let (onclick, class) = match props.user.deleted_at.is_some() || props.user.confirmed {
        true => (
            None,
            "btn btn-sm btn-success px-1 mr-1 btn-disabled aria-disabled",
        ),
        false => (
            Some(show_modal_callback(
                ModalData::Dialog(DialogData {
                    title: TK::Activate,
                    message: TK::ActivateUserQuestion(props.user.name.clone()),
                    buttons: Buttons::ConfirmCancel(
                        ModalButton::new(TK::Activate, Some(onclick)),
                        ModalButton::new(TK::Cancel, None),
                    ),
                }),
                dispatch.clone(),
            )),
            "btn btn-sm btn-success px-1 mr-1",
        ),
    };
    html! {
        <button {class} {onclick}>{locales_store.get(TK::Activate)}</button>
    }
}

#[function_component(DeleteButton)]
fn delete_button(props: &UserRowProps) -> Html {
    let (locales_store, _) = use_store::<LocalesStore>();
    let (session_store, session_dispatch) = use_store::<SessionStore>();
    let (_, dispatch) = use_store::<ModalStore>();
    let error_state = use_state_eq(|| None);
    let token = session_store.token.clone().unwrap_or_default();
    let onclick = async_event!(|props, token, error_state| {
        match ApiClient::delete_user(&token, props.user.id).await {
            Ok(()) => props.reload.emit(()),
            Err(error) => error_state.set(Some(error)),
        }
    });
    if let Some(error) = &*error_state {
        if let Err(redirect) = error.handle_failed_auth(session_dispatch) {
            return redirect;
        }
    }
    let (onclick, class) = match props.user.deleted_at.is_some() {
        true => (
            None,
            "btn btn-sm btn-warning px-1 mr-1 btn-disabled aria-disabled",
        ),
        false => (
            Some(show_modal_callback(
                ModalData::Dialog(DialogData {
                    title: TK::Delete,
                    message: TK::DeleteUserQuestion(props.user.name.clone()),
                    buttons: Buttons::RiskyCancel(
                        ModalButton::new(TK::Delete, Some(onclick)),
                        ModalButton::new(TK::Cancel, None),
                    ),
                }),
                dispatch.clone(),
            )),
            "btn btn-sm btn-warning px-1 mr-1",
        ),
    };
    html! {
        <button {class} {onclick}>{locales_store.get(TK::Delete)}</button>
    }
}
