#![allow(non_snake_case)]
use crate::components::{Footer, NavBar};
use crate::pages::{Home, NewTask, Settings, SignIn, SignUp};
use dioxus::prelude::*;
use dioxus_router::{Route, Router};

mod components;
mod pages;

fn main() {
    // Init debug tool for WebAssembly.
    // wasm_logger::init(wasm_logger::Config::default());
    // console_error_panic_hook::set_once();

    dioxus_web::launch(App);
}
pub struct DarkMode(pub bool);
pub struct ToastMessage(pub &'static str);

fn App(cx: Scope) -> Element {
    use_shared_state_provider(cx, || DarkMode(false));
    use_shared_state_provider(cx, || ToastMessage(""));

    let dark_mode = use_shared_state::<DarkMode>(cx).unwrap();
    let is_dark = dark_mode.read().0;

    let toast_message = use_shared_state::<ToastMessage>(cx).unwrap();
    let message = toast_message.read().0;
    let toast = if !toast_message.read().0.is_empty() {"top-20"} else {"-top-20"};

    cx.render(rsx!(
        div {
            class: if is_dark {"@apply top-notificationdark"} else {"top-notification"},
            "note: under heavy development and changes are due"
        }
        Router {
            main {
                class: if is_dark {"main-containerdark"} else {"main-container"},
                div {
                    class: "@apply toast-message fixed z10 flex {toast} w-full items-center justify-center",
                    div {
                        class: "bg-green bg-opacity-80 text-white rounded p4",
                        "✅ {message}"
                    }
                }
                NavBar {}
                    Route { to: "/", Home {} }
                    Route { to: "/new_task", NewTask {} }
                    Route { to: "/settings", Settings {} }
                    Route { to: "/signin", SignIn {} }
                    Route { to: "/signup", SignUp {} }
                    Footer {}
            }
        }
    ))
}
