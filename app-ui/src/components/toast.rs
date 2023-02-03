use dioxus::prelude::*;

use crate::ToastMessage;

pub fn Toast(cx: Scope) -> Element {
    let toast_message = use_shared_state::<ToastMessage>(cx).unwrap();
    let message = toast_message.read().0;
    let toast = if !toast_message.read().0.is_empty() {"top-20"} else {""};

    cx.render(rsx! {
        div { class: "@apply toast-message fixed z10 flex {toast} w-full items-center justify-center ease-in-out",
            div { class: "bg-green bg-opacity-50 filter backdrop-blur text-white rounded p4",
                "✅ {message}"
            }
        }
    })
}