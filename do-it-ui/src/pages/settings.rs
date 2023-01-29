use dioxus::{
    events::{FormData, MouseEvent},
    prelude::*,
};

use crate::{components::{FormButton, FormInput, FormTextarea}, DarkMode};

pub fn Settings(cx: Scope) -> Element {
    let profilePictureURL = use_state(&cx, String::new);
    let name = use_state(&cx, String::new);
    let biography = use_state(&cx, String::new);
    let email = use_state(&cx, String::new);
    let password = use_state(&cx, String::new);

    let dark_mode = use_shared_state::<DarkMode>(cx).unwrap();
    let is_dark = dark_mode.read().0;
    let dark = if is_dark {"dark"} else {""};

    cx.render(rsx! {
        div {
            class: "@apply settings md:w-screen-sm lg:w-screen-md grid md:p8 mx6 md:mx16 md:ml32 xl:ml40 rounded-xl shadow-2xl",
            h2 {
                class: "text-4xl font-black text-true-gray-600 p2 mix-blend-exclusion text-center",
                "Settings"
            }
            div {
                class: "grid p8 base-container{dark}",
                aside {
                    class: "header-wrapper",
                    label {
                        class: "h-title-header", "Profile"
                    },
                    p {
                        class: "p-description",
                        "Customize your profile settings"
                    }
                }

                section {
                    class: "form-data p8 shadow-inner my4 rounded-xl",
                    form {
                        class: "grid gap4",
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
                            placeholder: "Share details about yourself".to_string(),
                            cols: 32
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