use dioxus::prelude::*;

#[derive(Props)]
pub struct TextFieldProps<'a> {
    title: &'a str,
    id: &'a str,
    placeholder: Option<&'a str>,
    state: &'a UseState<String>,
    validation_fn: for<'b> fn(&'b str) -> Result<(), &'static str>,
}

pub fn TextField<'a>(cx: Scope<'a, TextFieldProps<'a>>) -> Element {
    let err: Result<(), &'static str> = (cx.props.validation_fn)(cx.props.state.get());

    cx.render(rsx! (
        div {
            class: "flex flex-col mb-4",
            label {
                class: "font-bold mb-2",
                r#for: cx.props.id,
                cx.props.title
            }
            input {
                class: format_args!("appearance-none border rounded py-1 px-2 {}", if err.is_err() {"border-red-500"} else {""}),
                r#id: cx.props.id,
                r#type: "text",
                placeholder: cx.props.placeholder.unwrap_or(cx.props.title),
                oninput: move |evt| cx.props.state.set(evt.value.clone()),
                value: "{cx.props.state}"
            }
            err.err().map(|e| rsx!(
                p {
                    class: format_args!("text-red-500 text-sm"),
                    "{e}"
                }
            ))
        }
    ))
}
