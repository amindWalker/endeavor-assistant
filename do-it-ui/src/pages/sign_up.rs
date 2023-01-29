use dioxus::{
    events::{FormData, MouseEvent},
    prelude::*,
};
use dioxus_router::Link;

use crate::{components::{FormButton, FormInput}, DarkMode};

pub fn SignUp(cx: Scope) -> Element {
    let name = use_state(&cx, String::new);
    let email = use_state(&cx, String::new);
    let password = use_state(&cx, String::new);

    let dark_mode = use_shared_state::<DarkMode>(cx).unwrap();
    let is_dark = dark_mode.read().0;
    let dark = if is_dark {"dark"} else {""};

    cx.render(rsx! {
        div {
            class: "@apply signup md:w-screen-sm lg:w-screen-md grid md:p8 mx6 md:mx16 md:ml32 xl:ml40 rounded-xl shadow-2xl",
            h2 {
                class: "text-4xl font-black text-true-gray-600 p2 mix-blend-exclusion text-center",
                "Sign Up"
            }
            div {
                class: "signup-section",
                div {
                    class: "form-data grid p8 base-container{dark}",
                    div {
                        class: "form-data header-wrapper",
                        p {
                            class: "btn-transparent",
                            Link { to: "/signin", "Have an account?" }
                        }

                        div {
                            class: "error-messages invisible",
                            p { "That email is already taken" }
                        }

                        form {
                            class: "grid gap4",
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