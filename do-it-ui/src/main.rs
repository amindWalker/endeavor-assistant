#![allow(non_snake_case)]
mod components;
mod pages;

use crate::components::{Footer, NavBar};
use crate::pages::{Home, NewTask, Settings, SignIn, SignUp};
use dioxus::prelude::*;
use dioxus_router::{Route, Router};

fn main() {
    // Init debug tool for WebAssembly.
    wasm_logger::init(wasm_logger::Config::default());
    console_error_panic_hook::set_once();

    dioxus_web::launch(App);
}

fn App(cx: Scope) -> Element {
    cx.render(rsx!(
        Router {
            div {
                class: "@apply top-notification",
                "NOTE: APP UNDER HEAVY DEVELOPMENT AND CHANGES ARE DUE"
            }
            main {
                class: "main-std",
                div {
                    class: "max-w-max col-end-1",
                    NavBar {}
                }
                div {
                    class: "max-w-full col-start-1 col-end-6",
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
