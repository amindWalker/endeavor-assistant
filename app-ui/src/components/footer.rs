use dioxus::prelude::*;

use crate::DarkMode;

pub fn Footer(cx: Scope) -> Element {
    let dark_mode = use_shared_state::<DarkMode>(cx).unwrap();
    let is_dark = dark_mode.read().0;

    cx.render(rsx! {
        footer { class: "@apply w-full text-center p8 m-auto",
            span { class: if is_dark { "footer text-white" } else { "footer" }, "Project made in Full Rust Stack🦀" }
            br {}
            span { class: "footer",
                a {
                    class: "text-red-600 hover:brightness-150",
                    href: "https://github.com/amindWalker",
                    target: "_blank",
                    "Breno Rocha"
                }
                a {
                    class: "i-line-md:buy-me-a-coffee-filled text-xl text-amber-700 hover:brightness-125",
                    href: "https://www.buymeacoffee.com/brenorochadev",
                    target: "_blank"
                }
                a {
                    class: "text-amber-700 hover:brightness-125",
                    href: "https://www.buymeacoffee.com/brenorochadev",
                    target: "_blank",
                    "buy me a coffee"
                }
            }
        }
    })
}
