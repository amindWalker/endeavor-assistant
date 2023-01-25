use dioxus::{events::MouseEvent, prelude::*};

#[derive(Props)]
pub struct FormButtonProps<'a> {
    onclick: EventHandler<'a, MouseEvent>,
    label: String,
}

pub fn FormButton<'a>(cx: Scope<'a, FormButtonProps<'a>>) -> Element {
    cx.render(rsx! {
        button {
            // TODO: default button component
            class: "btn",
            r#type: "button",
            onclick: move |evt| cx.props.onclick.call(evt),
            "{cx.props.label}"
        }
    })
}