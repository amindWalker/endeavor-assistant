use dioxus::prelude::*;
use dioxus_router::Link;

pub fn NavBar(cx: Scope) -> Element {
    let mut state = use_state(&cx, || false);
    let theme = if *state.get() {
        "i-line-md:moon-twotone mr1"
    } else {
        "i-line-md:moon-to-sunny-outline-transition mr1"
    };

    cx.render(rsx! {
        header {
            class: "@apply col-start-1 text-center p4 text-red-600 animate-pulse",
            "Note: app under heavy development and changes are due."
        }
        nav {
            class: "navbar col-end-1 mt8 ml4 drop-shadow-xl",
            Link {
                class: "btn-primary text-center p4", to: "/",
                "DO IT MANAGER"
            },
            menu {
                class: "base mx4 nav-list grid",
                    Link {
                        class: "nav-item btn-transparent flex items-center justify-start",
                        to: "/",
                        i { class: "i-mdi:monitor-dashboard mr1" }
                        "Dashboard"
                    }
                    Link {
                        class: "nav-item btn-transparent flex items-center justify-start", to: "/new_task",
                        i { class: "i-line-md:check-list-3-filled mr1" },
                        "Tasks",
                    }
                    Link {
                        class: "nav-item btn-transparent flex items-center justify-start", to: "/settings",
                        i { class: "i-line-md:switch-filled mr1" },
                        "Settings"
                    }
                    Link {
                        class: "nav-item btn-transparent flex items-center justify-start", to: "/signin",
                        i { class: "i-line-md:account-small mr1" },
                        "Sign in"
                    }
                    Link {
                        class: "nav-item btn-transparent flex items-center justify-start text-left", to: "/signup",
                        i { class: "i-line-md:clipboard-check mr1" },
                        "Sign up"
                    }

                hr { class: "drop-shadow-md mt4" }

                button {
                    class: "cursor-pointer flex items-center justify-center text-lg p4 text-orange-600 hover:brightness-150",
                    onclick: move |_| {
                        state.set(!state);
                    },
                    i { class: theme },
                }
            }
        }
    })
}
