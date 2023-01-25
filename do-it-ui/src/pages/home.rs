use dioxus::prelude::*;
use dioxus_router::Link;

pub fn Home(cx: Scope) -> Element {
    cx.render(rsx! {
        div {
            class: "@apply home col-start-1 pl8 border-l",
            div {
                class: "z-10 banner p4 rounded-xl bg-red fixed",
                div {
                    class: "container",
                    h1 {
                        p { class: "text-white text-center text-shadow font-bold animate-pulse",
                            "Note: app under heavy development" }
                    }
                }
            }
            div {
                class: "container page mt16",
                div {
                    class: "section",

                    div {
                        class: "task-section",
                        div {
                            class: "task",
                            ul {
                                class: "task-details",
                                li {
                                    class: "nav-item",
                                    Link { class: "nav-link active", to: "", "Overview" }
                                }
                                li {
                                    class: "nav-item",
                                    Link { class: "nav-link disabled", to: "", "Your tasks" }
                                }
                            }
                        }

                        div {
                            class: "task",
                            div {
                                class: "task-meta",
                                Link { to: "#task",
                                    ul {
                                        class: "base",
                                        li {
                                            class: "base m2",
                                            input {
                                                "type": "checkbox",
                                                id: "tasks",
                                                name: "tasks",
                                                checked: true
                                            }
                                            "\tDaily work"
                                        },
                                        li {
                                            class: "base m2",
                                            input {
                                                "type": "checkbox",
                                                id: "tasks",
                                                name: "tasks",
                                                checked: true
                                            }
                                            "\tRunning"
                                        },
                                        li {
                                            class: "base m2",
                                            input {
                                                "type": "checkbox",
                                                id: "tasks",
                                                name: "tasks",
                                                checked: true
                                            }
                                            "\tFill weekly appointments"
                                        },
                                        li {
                                            class: "base m2",
                                            input {
                                                "type": "checkbox",
                                                id: "tasks",
                                                name: "tasks",
                                                checked: true
                                            }
                                            "\tGet more clients"
                                        }
                                    }
                                }
                                button {
                                    class: "btn flex",
                                    i { class: "i-line-md:edit-twotone mr1" }
                                    "Edit tasks"
                                }
                            }
                            Link {
                                class: "preview-link", to: "#task",
                                h2 {
                                    class: "text-lg font-bold",
                                    "Basic tasks"
                                }
                                p { "Describe your tasks here" }
                                span { "Read more ..." }
                            }
                        }

                        div {
                            class: "task-preview",
                            div {
                                class: "task-meta",
                                Link { to: "#task",
                                    div {
                                        class: "i-flat-color-icons:statistics text-[16rem]"
                                    }
                                }
                            }
                            button {
                                class: "btn flex",
                                i { class: "i-teenyicons:pie-chart-solid mr1" }
                                "Overview"
                            }
                            Link {
                                class: "preview-link", to: "#task",
                                h2 {
                                    class: "text-lg font-bold",
                                    "Financial progress"
                                }
                                span { "Read more ..." }
                            }
                        }
                    }

                    div {
                        class: "side-container",
                        div {
                            class: "sidebar",
                            p { "Popular Tags" }
                            div {
                                class: "tag-list",
                                Link { to: "#tags", class: "tags", "programming" }
                                Link { to: "#tags", class: "tags", "javascript" }
                                Link { to: "#tags", class: "tags", "emberjs" }
                                Link { to: "#tags", class: "tags", "angularjs" }
                                Link { to: "#tags", class: "tags", "react" }
                                Link { to: "#tags", class: "tags", "mean" }
                                Link { to: "#tags", class: "tags", "node" }
                                Link { to: "#tags", class: "tags", "rust" }
                            }
                        }
                    }
                }
            }
        }
    })
}
