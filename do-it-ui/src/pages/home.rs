use dioxus::prelude::*;
use dioxus_router::Link;

pub fn Home(cx: Scope) -> Element {
    cx.render(rsx! {
        div {
            class: "@apply home p8 rounded-xl ring ring-orange-200 shadow-2xl",
            section {
                class: "dashboard max-w-full grid lg:grid-flow-col md:auto-cols-max space-x-10",
                div {
                    class: "task-section base lg:min-w-128",
                    div {
                        class: "task-meta",
                        p {
                            "Tasks"
                        },
                        ul {
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
                        button {
                            class: "btn-primary flex",
                            i { class: "i-line-md:edit-twotone mr1" }
                            "Edit tasks"
                        }
                    }
                    Link {
                        class: "preview-link", to: "#edit-task",
                        h2 {
                            class: "text-lg font-bold",
                            "Basic tasks"
                        }
                        p { "Describe your tasks here" }
                        span { "Read more ..." }
                    }

                }
                div {
                    class: "overview-section base w100",
                    Link { class: "nav-link active", to: "", "Overview" }
                    div {
                        class: "task-meta",
                        Link { to: "#task",
                            div {
                                class: "i-flat-color-icons:statistics text-[16rem]"
                            }
                        }
                    }
                    button {
                        class: "btn-primary flex",
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
        }
    })
}
