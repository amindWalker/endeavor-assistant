#![allow(non_snake_case)]
mod components;
mod pages;

use crate::components::{Footer, NavBar};
use crate::pages::{NewTask, Home, Settings, SignIn, SignUp};
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
        main {
            class: "@apply all:transition-400 all:transition-all grid auto-cols-max",

            Router {
                NavBar {}
                Route { to: "/", Home {} }
                Route { to: "/settings", Settings {} }
                Route { to: "/signup", SignUp {} }
                Route { to: "/signin", SignIn {} }
                Route { to: "/new_task", NewTask {} }
                Footer {}
            }
        }
    ))
}
