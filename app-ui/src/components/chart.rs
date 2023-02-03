use dioxus::prelude::*;

pub fn Chart(cx: Scope) -> Element {
    cx.render(rsx! {

        div { class: "w-full h-64 bg-gray-200",
            div {
                div { class: "absolute bottom-0 left-0 flex justify-between w-full px-4 py-2",
                    div { class: "text-xs text-gray-700", "0" }
                    div { class: "text-xs text-gray-700", "100" }
                }
                div { class: "absolute bottom-0 left-0 flex justify-between w-full px-4 py-2",
                    div { class: "text-xs text-gray-700", "Monday" }
                    div { class: "text-xs text-gray-700", "Sunday" }
                }
            }
        }
    })
}
