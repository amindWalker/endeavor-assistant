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
        nav {
            class: "@apply navbar col-end-1 max-w-max mx4 mt8",
            div {
                ul {
                    class: "base mx4 nav-list place-items-center",
                    li {
                        class: "nav-item btn",
                        // todo: Add "active" class when you're on that page.
                        Link { class:"nav-link active flex", to: "/",
                            i { class: "i-line-md:circle-to-confirm-circle-twotone-transition mr1" }
                            "DO IT" }
                    }
                    li {
                        class: "nav-item btn-2",
                        Link {
                            class: "nav-link flex", to: "/new_task",
                            i { class: "i-line-md:check-list-3-filled mr1" },
                            "New Task",
                        }
                    }
                    li {
                        class: "nav-item btn-2",
                        Link {
                            class: "nav-link flex", to: "/settings",
                            i { class: "i-line-md:switch-filled mr1" },
                            "Settings"
                        }
                    }
                    li {
                        class: "nav-item btn-2",
                        Link { class:"nav-link flex", to: "/signin",
                        i { class: "i-line-md:account-small mr1" },
                        "Sign in" }
                    }
                    li {
                        class: "nav-item btn-2",
                        Link { class:"nav-link flex", to: "/signup",
                        i { class: "i-line-md:clipboard-check mr1" },
                        "Sign up" }
                    }
                    span {
                        class: "btn-2 cursor-pointer flex",
                        i { class: theme },
                        button {
                            onclick: move |_| {
                                state.set(!state);
                            },
                            if *state.get() { "dark" } else { "light" }
                        }
                    }
                }
            }
        }
    })
}
