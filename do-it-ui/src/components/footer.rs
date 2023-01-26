use dioxus::prelude::*;
use dioxus_router::Link;

pub fn Footer(cx: Scope) -> Element {
    cx.render(rsx! {
        footer {
            class: "@apply col-start-1 lg:col-end-6 p8 justify-center",
            div {
                class: "",
                span {
                    class: "footer flex items-center justify-center",
                    "Full Stack ported from TypeScript to pure Rust",
                }
                span {
                    class: "footer flex items-center justify-center",
                    a {
                        class: "text-red-600 hover:brightness-150",
                        href: "https://github.com/amindWalker", target: "_blank",
                        "Breno Rocha"
                    }
                    a {
                        class: "i-line-md:buy-me-a-coffee-filled text-xl text-amber-900 hover:brightness-125",
                        href: "https://www.buymeacoffee.com/brenorochadev", target: "_blank",
                    }
                    a {
                        class: "text-amber-900 hover:brightness-125",
                        href: "https://www.buymeacoffee.com/brenorochadev", target: "_blank",
                        "buy me a coffee"
                    }
                }
            }
        }
    })
}
