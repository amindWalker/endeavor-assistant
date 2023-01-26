use dioxus::{events::FormData, prelude::*};

#[derive(Props)]
pub struct FormTextareaProps<'a> {
    oninput: EventHandler<'a, FormData>,

    #[props(optional)]
    rows: Option<u8>,

    #[props(optional)]
    cols: Option<u8>,

    #[props(optional)]
    placeholder: Option<String>,
}

pub fn FormTextarea<'a>(cx: Scope<'a, FormTextareaProps<'a>>) -> Element {
    let rows = cx.props.rows.unwrap_or(8);
    let cols = cx.props.cols.unwrap_or(8);
    let place_holder = cx.props.placeholder.clone().unwrap_or_default();
    cx.render(rsx! {
        fieldset {
            class: "form-group",
            textarea {
                class: "text-field",
                oninput: move |e| cx.props.oninput.call(e.data.as_ref().clone()),
                placeholder: "{place_holder}",
                rows: "{rows}",
                cols: "{cols}"
            }
        }
    })
}