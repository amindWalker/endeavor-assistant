use dioxus::prelude::*;
use dioxus_router::Link;

use crate::DarkMode;

pub fn Footer(cx: Scope) -> Element {
    let dark_mode = use_shared_state::<DarkMode>(cx).unwrap();
    let is_dark = dark_mode.read().0;

    let dark = if is_dark {"white"} else {""};
    cx.render(rsx! {
        footer {
            class: "@apply col-start-1 col-span-full p8 justify-center",
            div {
                class: "",
                span {
                    class: if is_dark {"footer flex items-center justify-center text-white"} else {"footer flex items-center justify-center"},
                    "Project made in Full Rust Stack🦀",
                }
                span {
                    class: "footer flex items-center justify-center text-{dark}",
                    a {
                        class: "text-red-600 hover:brightness-150",
                        href: "https://github.com/amindWalker", target: "_blank",
                        "Breno Rocha"
                    }
                    a {
                        class: "i-line-md:buy-me-a-coffee-filled text-xl text-amber-700 hover:brightness-125",
                        href: "https://www.buymeacoffee.com/brenorochadev", target: "_blank",
                    }
                    a {
                        class: "text-amber-700 hover:brightness-125",
                        href: "https://www.buymeacoffee.com/brenorochadev", target: "_blank",
                        "buy me a coffee"
                    }
                }
            }
        }
    })
}
