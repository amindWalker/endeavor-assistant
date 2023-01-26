use dioxus::prelude::*;
use dioxus_router::Link;

pub fn Footer(cx: Scope) -> Element {
    cx.render(rsx! {
        footer {
            class: "@apply col-end-2 flex justify-center mt8",
            div {
                class: "",
                span {
                    class: "attribution flex items-center justify-center",
                    "Full Stack application ported from TypeScript to pure Rust |",
                    a {
                        class: "text-red-600 hover:brightness-150",
                        href: "https://github.com/amindWalker", target: "_blank",
                        "\tBreno Rocha"
                    }
                }
                span {
                    class: "attribution flex items-center justify-center",
                    a {
                        class: "i-line-md:buy-me-a-coffee-filled text-lg text-amber-900 hover:brightness-125",
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
