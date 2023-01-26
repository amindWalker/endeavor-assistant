use dioxus::{
    events::{FormData, MouseEvent},
    prelude::*,
};

use crate::components::{FormButton, FormInput, FormTextarea};

pub fn NewTask(cx: Scope) -> Element {
    let title = use_state(&cx, String::new);
    let summary = use_state(&cx, String::new);
    let content = use_state(&cx, String::new);
    let tags = use_state(&cx, String::new);
    cx.render(rsx! {
        div {
            class: "@apply editor-mode",
            div {
                class: "",
                h1 {
                    class: "",
                    "New Task"
                }

                hr {}

                div {
                    class: "task-section",

                    div {
                        class: "form-data",
                        form {
                            class: "grid gap4",
                            FormInput {
                                oninput: move |s: FormData| title.set(s.value),
                                placeholder: "Task title".to_string()
                            }
                            FormInput {
                                oninput: move |s: FormData| summary.set(s.value),
                                placeholder: "What's this task about?".to_string()
                            }
                            FormTextarea {
                                oninput: move |s: FormData| content.set(s.value),
                                placeholder: "Task description".to_string(),
                                rows: 16,
                                cols: 32
                            }
                            FormInput {
                                oninput: move |s: FormData| tags.set(s.value),
                                placeholder: "Enter tags".to_string(),

                            }
                            div {
                                class: "tag-list"
                            }
                            FormButton {
                                onclick: move |_: MouseEvent| {
                                    log::info!("[NewTask] button clicked. title: {}", title);
                                },
                                label: "Publish Task".to_string()
                            }
                        }
                    }
                }
            }
        }
    })
}