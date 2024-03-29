use dioxus::{
    events::{FormData, MouseEvent},
    prelude::*,
};
use dioxus_router::Link;

use crate::{components::{FormButton, FormInput}, DarkMode};

pub fn SignIn(cx: Scope) -> Element {
    let email = use_state(cx, String::new);
    let password = use_state(cx, String::new);

    let dark_mode = use_shared_state::<DarkMode>(cx).unwrap();
    let is_dark = dark_mode.read().0;
    let dark = if is_dark {"dark"} else {""};

    cx.render(rsx! {
        div { class: "@apply signin md:w-screen-sm lg:w-screen-md md:p8 mx6 md:mx16 md:ml32 xl:ml40 rounded-xl drop-shadow-xl md:shadow-xl",
            h2 { class: "bg-white bg-opacity-50 leading-tight rounded-lg p4 mb4 md:mb8 mix-blend-exclusion",
                "Endeavor / Sign In"
            }
            section { class: "signin-section base-container{dark} grid p8",
                div { class: "form-data header-wrapper",
                    p { class: "btn-transparent",
                        Link { to: "/signup", "Don't have an account?" }
                    }

                    div { class: "error-message invisible",
                        // TODO: toast message or message over the input
                        p { "Invalid credentials" }
                    }

                    form { class: "grid gap4",
                        FormInput { oninput: move |s: FormData| email.set(s.value), placeholder: "Email".to_string() }
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
    })
}