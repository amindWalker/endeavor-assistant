use dioxus::{
    events::{FormData, MouseEvent},
    prelude::*,
};
use dioxus_router::Link;

use crate::components::{FormButton, FormInput};

pub fn SignUp(cx: Scope) -> Element {
    let name = use_state(&cx, String::new);
    let email = use_state(&cx, String::new);
    let password = use_state(&cx, String::new);

    cx.render(rsx! {
        div {
            class: "auth",
            div {
                class: "container ",
                div {
                    class: "signup-section",
                    div {
                        class: "form-data",
                        h1 {
                            class: "text-title",
                            "Sign up"
                        }
                        p {
                            class: "text-para",
                            Link { to: "/signin", "Have an account?" }
                        }
                        br {}
                        br {}

                        ul {
                            class: "error-messages set-visible",
                            li { "That email is already taken" }
                        }

                        form {
                            FormInput {
                                oninput: move |s: FormData| name.set(s.value),
                                placeholder: "Your Name".to_string()
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
                                    log::info!("[SignUp] button clicked. name: {} | email: {}", name, email);
                                },
                                label: "Sign up".to_string()
                            }
                        }
                    }
                }
            }
        }
    })
}