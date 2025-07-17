use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct FieldLabelProps {
    label: String,
    id: String,
    field: Element,
}

pub fn FieldLabel(props: FieldLabelProps) -> Element {
    rsx! (
        div {
            class: "flex flex-col mb-4",
            label {
                class: "font-bold mb-2",
                r#for: props.id,
                { props.label }
            }
            { props.field }
        }
    )
}

#[derive(PartialEq, Props, Clone)]
pub struct TextFieldProps {
    id: String,
    class: Option<String>,
    placeholder: String,
    value: String,
    oninput: EventHandler<Event<FormData>>,
    onkeyup: Option<EventHandler<Event<KeyboardData>>>,
    validation_fn: for<'b> fn(&'b str) -> Result<(), &'static str>,
}

pub fn TextField(props: TextFieldProps) -> Element {
    let mut is_modified = use_signal(|| false);
    let err: Result<(), &'static str> = if *is_modified.peek() {
        (props.validation_fn)(&props.value)
    } else {
        Ok(())
    };

    rsx! (
        input {
            class: format_args!("{} appearance-none border rounded py-1 px-2 {}", props.class.unwrap_or("".to_string()), if err.is_err() {"border-red-500"} else {""}),
            r#id: props.id,
            r#type: "text",
            placeholder: props.placeholder,
            oninput: move |evt| {
                is_modified.set(true);
                props.oninput.call(evt);
            },
            onkeyup: move |evt| props.onkeyup.as_ref().map_or((), |f| f.call(evt)),
            value: "{props.value}"
        }
        { err.err().map(|e| rsx!(
            p {
                class: format_args!("text-red-500 text-sm"),
                "{e}"
            }
        )) }
    )
}
