use dioxus::{events::MouseEvent, prelude::*};

#[derive(Props)]
pub struct FormButtonProps<'a> {
    onclick: EventHandler<'a, MouseEvent>,
    label: String,
}

pub fn FormButton<'a>(cx: Scope<'a, FormButtonProps<'a>>) -> Element {
    cx.render(rsx! {
        button {
            class: "btn-primary",
            r#type: "button",
            onclick: move |e| cx.props.onclick.call(e),
            "{cx.props.label}"
        }
    })
}