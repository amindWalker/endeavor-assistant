use dioxus::prelude::*;
use dioxus_router::Link;

pub fn Home(cx: Scope) -> Element {
    cx.render(rsx! {
        div {
            class: "@apply home w-full p8 rounded-xl shadow-2xl",
            div {
                class: "grid gap8 place-items-center grid-cols-1 xl:grid-cols-2 w-full",
                section {
                    class: "task-section base-container w-full h-full",
                    div {
                        class: "task-meta p4 lg:p8",
                        aside {
                            class: "border border-dotted rounded p4",
                            label {
                                class: "text-xl text-orange-600 font-bold",
                                "Tasks"
                            },
                            h2 {
                                class: "h-title",
                                "Today's latest goals"
                            }
                            p {
                                class: "p-description",
                                "plan your business strategy and set timely goals"
                            }
                        }
                        ul {
                            class: "task-block mt4",
                            li {
                                class: "task-list-item",
                                input {
                                    class: "mr4",
                                    r#type: "checkbox",
                                    id: "tasks",
                                    name: "tasks",
                                    checked: true
                                }
                                label { "Daily work" }
                            },
                            li {
                                class: "task-list-item",
                                input {
                                    class: "mr4",
                                    r#type: "checkbox",
                                    id: "tasks",
                                    name: "tasks",
                                    checked: true
                                }
                                label { "Running" }
                            },
                            li {
                                class: "task-list-item",
                                input {
                                    class: "mr4",
                                    r#type: "checkbox",
                                    id: "tasks",
                                    name: "tasks",
                                    checked: true
                                }
                                label { "Fill weekly appointments" }
                            },
                            li {
                                class: "task-list-item",
                                input {
                                    class: "mr4",
                                    r#type: "checkbox",
                                    id: "tasks",
                                    name: "tasks",
                                    checked: true
                                }
                                label { "Get more clients" }
                            },
                            li {
                                class: "task-list-item",
                                input {
                                    class: "mr4",
                                    r#type: "checkbox",
                                    id: "tasks",
                                    name: "tasks",
                                    checked: true
                                }
                                label { "Rinse and repeat" }
                            }
                            Link {
                                class: "btn-primary flex items-center justify-center", to: "/new_task",
                                i { class: "i-line-md:edit-twotone mr1" }
                                "New task"
                            }
                        }
                    }

                }

                section {
                    class: "overview-section base-container w-full h-full",
                    div {
                        class: "overview-meta  p4 lg:p8",
                        aside {
                            class: "border border-dotted rounded p4",
                            label {
                                class: "text-xl text-orange-600 font-bold",
                                "Overview"
                            },
                            h2 {
                                class: "h-title",
                                "See your progress with AI insights"
                            }
                            p {
                                class: "p-description",
                                "get suggestions based on your daily tasks"
                            }
                        }
                        Link {
                            class: "btn-primary my4 flex items-center justify-center", to: "#overview",
                            i { class: "i-teenyicons:pie-chart-solid mr1" }
                            "Overview"
                        }
                        div {
                            class: "task-block overview flex items-center justify-center drop-shadow-2xl",
                            Link { to: "#overview",
                                i {
                                    class: "i-flat-color-icons:statistics text-9xl md:text-[16rem] drop-shadow-2xl"
                                }
                            }
                        }
                    }

                }
            }
         }
    })
}
