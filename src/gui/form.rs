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

#[derive(Props, Clone)]
pub struct TextFieldProps {
    id: String,
    class: Option<String>,
    placeholder: String,
    value: String,
    oninput: EventHandler<Event<FormData>>,
    onkeyup: Option<EventHandler<Event<KeyboardData>>>,
    validation_fn: for<'b> fn(&'b str) -> Result<(), &'static str>,
}

impl PartialEq for TextFieldProps {
    fn eq(&self, other: &Self) -> bool {
        todo!()
    }
}

pub fn TextField<'a>(props: TextFieldProps) -> Element {
    let err: Result<(), &'static str> = (props.validation_fn)(&props.value);

    rsx! (
        input {
            class: format_args!("{} appearance-none border rounded py-1 px-2 {}", props.class.unwrap_or("".to_string()), if err.is_err() {"border-red-500"} else {""}),
            r#id: props.id,
            r#type: "text",
            placeholder: props.placeholder,
            oninput: move |evt| props.oninput.call(evt),
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
