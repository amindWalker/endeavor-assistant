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
        div { class: "fixed mt16 md:m-auto z1 col-end-6 md:col-end-1 md:mt8",
            div { class: "@apply navbar text-lg md:ml8 drop-shadow-xl",
                menu { class: "base-container{dark} nav-menu",
                    Link { title: "Dashboard", class: "nav-item btn-transparent", to: "/", i { class: "i-mdi:monitor-dashboard" } }
                    Link {
                        title: "Task Editor",
                        id: "tasks",
                        class: "nav-item btn-transparent",
                        to: "/new_task",
                        i { class: "i-line-md:check-list-3-filled" }
                        span { class: "hidden hover:inline", "Tasks" }
                    }
                    Link { title: "Settings", class: "nav-item btn-transparent", to: "/settings", i { class: "i-line-md:switch-filled" } }
                    Link { title: "Sign In", class: "nav-item btn-transparent", to: "/signin", i { class: "i-line-md:account-small" } }
                    Link { title: "Sign Up", class: "nav-item btn-transparent", to: "/signup", i { class: "i-line-md:clipboard-check" } }

                    hr { class: "opacity-10 mix-blend-difference my4" }

                    button {
                        class: "cursor-pointer text-orange-600 hover:brightness-150",
                        title: "Switch Theme",
                        onclick: move |_| {
                            let dark_value = !dark_mode.read().0;
                            dark_mode.write().0 = dark_value;
                        },
                        i { class: moon_sun }
                    }
                }
            }
        }
    })
}
