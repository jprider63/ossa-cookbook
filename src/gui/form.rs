use dioxus::prelude::*;

#[derive(Props)]
pub struct FieldLabelProps<'a> {
    label: &'a str,
    id: &'a str,
    field: Element<'a>,
}

pub fn FieldLabel<'a>(cx: Scope<'a, FieldLabelProps<'a>>) -> Element {
    cx.render(rsx! (
        div {
            class: "flex flex-col mb-4",
            label {
                class: "font-bold mb-2",
                r#for: cx.props.id,
                cx.props.label
            }
            &cx.props.field
        }
    ))
}

#[derive(Props)]
pub struct TextFieldProps<'a> {
    id: &'a str,
    class: Option<&'a str>,
    placeholder: &'a str,
    value: &'a str,
    oninput: EventHandler<'a,Event<FormData>>,
    validation_fn: for<'b> fn(&'b str) -> Result<(), &'static str>,
}

pub fn TextField<'a>(cx: Scope<'a, TextFieldProps<'a>>) -> Element {
    let err: Result<(), &'static str> = (cx.props.validation_fn)(cx.props.value);

    cx.render(rsx! (
        input {
            class: format_args!("{} appearance-none border rounded py-1 px-2 {}", cx.props.class.unwrap_or(""), if err.is_err() {"border-red-500"} else {""}),
            r#id: cx.props.id,
            r#type: "text",
            placeholder: cx.props.placeholder,
            oninput: move |evt| cx.props.oninput.call(evt),
            value: "{cx.props.value}"
        }
        err.err().map(|e| rsx!(
            p {
                class: format_args!("text-red-500 text-sm"),
                "{e}"
            }
        ))
    ))
}
