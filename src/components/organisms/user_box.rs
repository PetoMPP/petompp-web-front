use crate::{
    components::{
        atoms::modal::{
            show_modal_callback, Buttons, DialogData, ModalButton, ModalData, ModalStore,
        },
        organisms::menu::close_menu,
    },
    data::{locales::tk::TK, session::SessionStore},
    pages::login::LoginRedirect,
    router::{admin::AdminRoute, route::Route},
    utils::style::get_svg_bg_mask_style,
};
use petompp_web_models::models::user::{RoleData, UserData};
use yew::prelude::*;
use yew_router::prelude::*;
use yewdux::prelude::*;

/// A component to display a user's information.
/// Or login/register if not logged in.
#[function_component(UserBox)]
pub fn user_box() -> Html {
    let (session_store, _) = use_store::<SessionStore>();
    if let Some(user) = &session_store.as_ref().user {
        return html! {
            <>
                <UserButton user={user.clone()}/>
                <LogoutButton/>
            </>
        };
    }
    html! {
        <>
            <LoginButton/>
            <RegisterButton/>
        </>
    }
}

#[function_component(LoginButton)]
fn login_button() -> Html {
    let clicked = use_state(|| false);
    let onclick = {
        let clicked = clicked.clone();
        Callback::from(move |_| {
            clicked.set(true);
            close_menu();
        })
    };
    if *clicked {
        clicked.set(false);
        return html! { <LoginRedirect /> };
    }
    html! {
        <div class={"btn btn-secondary p-1"} {onclick}>
            <a class={"aspect-square h-full bg-secondary-content"} style={get_svg_bg_mask_style("/img/ui/login.svg")}/>
        </div>
    }
}

#[function_component(LogoutButton)]
fn logout_button() -> Html {
    let (_, session_dispatch) = use_store::<SessionStore>();
    let (_, dispatch) = use_store::<ModalStore>();
    let onclick = Callback::from(move |_| {
        close_menu();
        session_dispatch.reduce(|_| SessionStore::default().into());
    });
    let onclick = show_modal_callback(
        ModalData::Dialog(DialogData {
            title: TK::Logout,
            message: TK::LogoutQuestion,
            buttons: Buttons::RiskyCancel(
                ModalButton::new(TK::Logout, Some(onclick)),
                ModalButton::new(TK::Cancel, None),
            ),
        }),
        dispatch,
    );
    html! {
        <div class={"btn btn-warning p-1"} {onclick}>
            <a class={"aspect-square h-full bg-warning-content"} style={get_svg_bg_mask_style("/img/ui/logout.svg")}/>
        </div>
    }
}

#[function_component(RegisterButton)]
fn register_button() -> Html {
    let navigator = use_navigator().unwrap();
    let onclick = Callback::from(move |_| {
        close_menu();
        navigator.push(&Route::Register);
    });
    html! {
        <div class={"btn btn-accent p-1"} {onclick}>
            <a class={"aspect-square h-full bg-accent-content"} style={get_svg_bg_mask_style("/img/ui/register.svg")}/>
        </div>
    }
}

#[derive(PartialEq, Properties, Clone)]
struct UserButtonProps {
    user: UserData,
}

#[function_component(UserButton)]
fn user_button(props: &UserButtonProps) -> Html {
    let navigator = use_navigator().unwrap();
    let onclick = match props.user.role {
        RoleData::Admin => Some(Callback::from(move |_| {
            close_menu();
            navigator.push(&AdminRoute::AdminPanel);
        })),
        _ => None,
    };
    html! {
        <a {onclick} class={"btn btn-primary font-mono text-xl normal-case"}>{&props.user.name}</a>
    }
}
