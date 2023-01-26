use dioxus::{
    events::{FormData, MouseEvent},
    prelude::*,
};
use dioxus_router::Link;

use crate::components::{FormButton, FormInput};

pub fn SignIn(cx: Scope) -> Element {
    let email = use_state(&cx, String::new);
    let password = use_state(&cx, String::new);

    cx.render(rsx! {
        div {
            class: "@apply auth",
            div {
                class: " ",
                div {
                    class: "signin-section",
                    div {
                        class: "form-data",
                        h1 {
                            class: "text-title",
                            "Sign in"
                        }
                        p {
                            class: "text-para",
                            Link { to: "/signup", "Don't have an account?" }
                        }

                        hr {}

                        ul {
                            class: "error-messages set-visible",
                            li { "Invalid credentials" }
                        }

                        form {
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
                                    log::info!("[SignIn] button clicked. email: {}", email);
                                },
                                label: "Sign in".to_string()
                            }
                        }
                    }
                }
            }
        }
    })
}