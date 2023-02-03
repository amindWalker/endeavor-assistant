use dioxus::prelude::*;
use dioxus_router::Link;

use crate::{DarkMode, components::Chart};

// fn(&Scoped<'_, _>) -> _
pub fn Home(cx: Scope) -> Element {
    let dark_mode = use_shared_state::<DarkMode>(cx).unwrap();
    let is_dark = dark_mode.read().0;

    let dark = if is_dark {"dark"} else {""};

    let task_item_theme = if is_dark {"list-itemdark"} else {"list-item"};

    cx.render(rsx! {
        div { class: "home col-span-full md:p8 mx6 md:mx16 md:ml32 xl:ml40 rounded-xl drop-shadow-xl md:shadow-xl",
            h2 { class: "breadcrumb", "Endeavor / Dashboard" }
            div { class: "@apply grid gap4 md:gap8 place-items-center xl:grid-cols-2 w-full",
                section { class: "task-section base-container{dark} w-full h-full",
                    div { class: "task-meta p8 lg:p8",
                        aside { class: "header-wrapper",
                            h2 { class: "h-title-header", "Tasks" }
                            h3 { class: "h-title", "Today's latest goals" }
                            p { class: "p-description", "plan your business strategy and set timely goals" }
                        }
                        ul { class: if is_dark { "block-wrapperdark mt4" } else { "block-wrapper mt4" },
                            li { class: "{task_item_theme}",
                                input {
                                    class: "mr4",
                                    r#type: "checkbox",
                                    id: "tasks",
                                    name: "tasks",
                                    checked: true
                                }
                                label { "Daily work" }
                            }
                            li { class: "{task_item_theme}",
                                input {
                                    class: "mr4",
                                    r#type: "checkbox",
                                    id: "tasks",
                                    name: "tasks",
                                    checked: true
                                }
                                label { "Running" }
                            }
                            li { class: "{task_item_theme}",
                                input {
                                    class: "mr4",
                                    r#type: "checkbox",
                                    id: "tasks",
                                    name: "tasks",
                                    checked: true
                                }
                                label { "Fill weekly appointments" }
                            }
                            li { class: "{task_item_theme}",
                                input {
                                    class: "mr4",
                                    r#type: "checkbox",
                                    id: "tasks",
                                    name: "tasks",
                                    checked: true
                                }
                                label { "Get more clients" }
                            }
                            li { class: "{task_item_theme}",
                                input {
                                    class: "mr4",
                                    r#type: "checkbox",
                                    id: "tasks",
                                    name: "tasks",
                                    checked: true
                                }
                                label { "Rinse and repeat" }
                            }
                            Link { class: "btn-primary flex items-center justify-center", to: "/new_task",
                                i { class: "i-line-md:edit-twotone mr1" }
                                "New task"
                            }
                        }
                    }
                }

                section { class: if is_dark {
    "overview-section base-containerdark w-full h-full"
} else {
    "overview-section base-container  w-full h-full"
},
                    div { class: "overview-meta  p4 lg:p8",
                        aside { class: "header-wrapper",
                            h2 { class: "h-title-header", "Overview" }
                            h3 { class: "h-title", "See your progress with AI insights" }
                            p { class: "p-description", "get suggestions based on your daily tasks" }
                        }
                        Link {
                            class: "btn-primary my4 flex items-center justify-center",
                            to: "#overview",
                            i { class: "i-teenyicons:pie-chart-solid mr1" }
                            "Overview"
                        }
                        div { class: if is_dark {
    "block-wrapperdark mt4 overview flex items-center justify-center"
} else {
    "block-wrapper mt4 overview flex items-center justify-center"
},
                            Link { to: "#overview", i { class: "i-flat-color-icons:statistics text-9xl md:text-[16rem] saturate-50" } }
                        }
                    }
                }
            }
        }
    })
}
