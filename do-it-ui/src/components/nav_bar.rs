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
        div {
            class: "@apply navbar md:ml8 col-end-1 drop-shadow-xl mt4 max-w-max",
            Link {
                class: "flex btn-primary items-center justify-center", to: "/",
                "Do It Manager"
            },
            menu {
                class: "base-container nav-list max-w-max",
                    Link {
                        class: "nav-item btn-transparent flex items-center", to: "/",
                        i { class: "i-mdi:monitor-dashboard mr1" }
                        span {
                            "Dashboard"
                        }
                    }
                    Link {
                        class: "nav-item btn-transparent flex items-center", to: "/new_task",
                        i { class: "i-line-md:check-list-3-filled mr1" },
                        span {
                            "Tasks"
                        }
                    }
                    Link {
                        class: "nav-item btn-transparent flex items-center", to: "/settings",
                        i { class: "i-line-md:switch-filled mr1" },
                        span {
                            "Settings"
                        }
                    }
                    Link {
                        class: "nav-item btn-transparent flex items-center", to: "/signin",
                        i { class: "i-line-md:account-small mr1" },
                        span {
                            "Sign In"
                        }
                    }
                    Link {
                        class: "nav-item btn-transparent flex items-center", to: "/signup",
                        i { class: "i-line-md:clipboard-check mr1" },
                        span {
                            "Sign Up"
                        }
                    }

                hr { class: "drop-shadow-md mt4" }

                button {
                    class: "cursor-pointer w-full p8 -mb2 text-orange-600 hover:brightness-150",
                    onclick: move |_| {
                        state.set(!state);
                    },
                    i { class: theme },
                }
            }
        }
    })
}
