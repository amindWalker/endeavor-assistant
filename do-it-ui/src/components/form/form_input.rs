use dioxus::{events::FormData, prelude::*};

#[derive(Props)]
pub struct FormInputProps<'a> {
    oninput: EventHandler<'a, FormData>,

    #[props(optional)]
    placeholder: Option<String>,
}

pub fn FormInput<'a>(cx: Scope<'a, FormInputProps<'a>>) -> Element {
    let place_holder = cx.props.placeholder.clone().unwrap_or_default();
    cx.render(rsx! {
        fieldset {
            class: "form-group grid gap4",
            input {
                class: "text-field",
                r#type: "text",
                oninput: move |e| cx.props.oninput.call(e.data.as_ref().clone()),
                placeholder: "{place_holder}",
            }
        }
    })
}