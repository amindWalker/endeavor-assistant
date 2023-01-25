use dioxus::{
    events::{FormData, MouseEvent},
    prelude::*,
};

use crate::components::{FormButton, FormInput, FormTextarea};

pub fn Settings(cx: Scope) -> Element {
    let profilePictureURL = use_state(&cx, String::new);
    let name = use_state(&cx, String::new);
    let biography = use_state(&cx, String::new);
    let email = use_state(&cx, String::new);
    let password = use_state(&cx, String::new);

    cx.render(rsx! {
        div {
            class: "settings",
            div {
                class: "container page",
                div {
                    class: "col-md-6 offset-md-3 col-xs-12",
                    h1 {
                        class: "text-xs-center", "Your Settings"
                    }

                    hr {}

                    form {
                        FormInput{
                            oninput: move |s: FormData| profilePictureURL.set(s.value),
                            placeholder: "Profile picture".to_string()
                        }
                        FormInput {
                            oninput: move |s: FormData| name.set(s.value),
                            placeholder: "Your Name".to_string()
                        }
                        FormTextarea{
                            oninput: move |s: FormData| biography.set(s.value),
                            placeholder: "Share details about yourself (optional)".to_string()
                        }
                        FormInput {
                            oninput: move |s: FormData| email.set(s.value),
                            placeholder: "Email".to_string()
                        }
                        FormInput {
                            oninput: move |s: FormData| password.set(s.value),
                            placeholder: "Password".to_string()
                        }
                        FormButton {
                            onclick: move |_: MouseEvent| {
                                log::info!("[Settings] button clicked. name: {} | email: {}", name, email);
                            },
                            label: "Update Settings".to_string()
                        }
                    }
                }
            }
        }
    })
}