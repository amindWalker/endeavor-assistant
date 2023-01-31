#![allow(non_snake_case)]
use crate::components::{Footer, NavBar, Toast};
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

    cx.render(rsx!(
        Router {
            main {
                class: if is_dark {"main-containerdark"} else {"main-container"},
                // NavBar {}
                // Toast {}
                div {
                    class: "grid",
                    Route { to: "/", Home {} }
                    Route { to: "/new_task", NewTask {} }
                    Route { to: "/settings", Settings {} }
                    Route { to: "/signin", SignIn {} }
                    Route { to: "/signup", SignUp {} }
                    Footer {}
                }
            }
        }
    ))
}
