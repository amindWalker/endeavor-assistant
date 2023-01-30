use dioxus::prelude::*;
use dioxus_router::Link;

use crate::DarkMode;

pub fn NavBar(cx: Scope) -> Element {
    // let state = use_state(&cx, || false);
    let dark_mode = use_shared_state::<DarkMode>(cx).unwrap();
    let moon_sun = if dark_mode.read().0 {
        "i-line-md:moon-twotone"
    } else {
        "i-line-md:moon-to-sunny-outline-transition"
    };

    let is_dark = dark_mode.read().0;

    let dark = if is_dark {"dark"} else {""};

    cx.render(rsx! {
        div {
            class: "fixed z1 col-end-6 md:col-end-1",
            div {
                class: "@apply navbar text-lg md:ml8 drop-shadow-xl mt4",
                menu {
                    class: "base-container{dark} nav-menu",
                        Link {
                            class: "nav-item btn-transparent", to: "/",
                            i { class: "i-mdi:monitor-dashboard" },
                        }
                        Link {
                            id: "tasks", class: "nav-item btn-transparent", to: "/new_task",
                            i { class: "i-line-md:check-list-3-filled" },
                            span { class: "hidden hover:inline", "Tasks"},
                        }
                        Link {
                            class: "nav-item btn-transparent", to: "/settings",
                            i { class: "i-line-md:switch-filled" },
                        }
                        Link {
                            class: "nav-item btn-transparent", to: "/signin",
                            i { class: "i-line-md:account-small" },
                        }
                        Link {
                            class: "nav-item btn-transparent", to: "/signup",
                            i { class: "i-line-md:clipboard-check" },
                        }

                    hr { class: "drop-shadow opacity-40 mix-blend-multiply my2 md:my8" }

                    button {
                        class: "cursor-pointer text-orange-600 hover:brightness-150",
                        title: "Theme",
                        onclick: move |_| {
                            let dark_value = !dark_mode.read().0;
                            dark_mode.write().0 = dark_value;
                        },
                        i { class: moon_sun },
                    }
                }
            }
        }
    })
}
