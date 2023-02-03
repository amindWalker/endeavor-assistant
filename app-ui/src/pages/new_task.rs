use dioxus::{
    events::{FormData, MouseEvent},
    prelude::*,
};
use crate::{components::{FormButton, FormInput, FormTextarea}, DarkMode, ToastMessage};

pub fn NewTask(cx: Scope) -> Element {
    let title = use_state(cx, String::new);
    let summary = use_state(cx, String::new);
    let content = use_state(cx, String::new);
    let tags = use_state(cx, String::new);

    let dark_mode = use_shared_state::<DarkMode>(cx).unwrap();
    let is_dark = dark_mode.read().0;
    let dark = if is_dark {"dark"} else {""};

    let toast_message = use_shared_state::<ToastMessage>(cx).unwrap();

    cx.render(rsx! {
        div { class: "@apply tasks md:w-screen-sm lg:w-screen-md md:p8 mx6 md:mx16 md:ml32 xl:ml40 rounded-xl drop-shadow-xl md:shadow-xl",
            h2 { class: "breadcrumb", "Endeavor / Task Editor" }
            div { class: "p8 base-container{dark}",
                aside { class: "header-wrapper",
                    h2 { class: "h-title-header", "Create Task" }
                    p { class: "p-description", "Fill the forms with your desired goals for the week" }
                }

                section { class: "form-data p4 md:p8 my4 rounded-xl",
                    form { class: "grid gap4",
                        FormInput {
                            oninput: move |s: FormData| title.set(s.value),
                            placeholder: "Task title".to_string()
                        }
                        FormInput {
                            oninput: move |s: FormData| summary.set(s.value),
                            placeholder: "What is your goal about?".to_string()
                        }
                        FormTextarea {
                            oninput: move |s: FormData| content.set(s.value),
                            placeholder: "Task description".to_string(),
                            cols: 24
                        }
                        FormInput {
                            oninput: move |s: FormData| tags.set(s.value),
                            placeholder: "Enter tags related to this task".to_string()
                        }
                        div { class: "tag-list" }
                        FormButton {
                            onclick: move |_: MouseEvent| {
                                log::info!("[TOAST] wild toast appears.");
                                toast_message.write().0 = r"SUCESS! Check your new task: {dashboard}";
                            },
                            label: "Add".to_string()
                        }
                    }
                }
            }
        }
    })
}
