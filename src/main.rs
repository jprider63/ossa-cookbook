use dioxus::prelude::*;

fn main() {
    dioxus::desktop::launch(app);
}

fn app(cx: Scope) -> Element {
    cx.render(rsx! (
        style { [include_str!("../dist/style.css")] }
        div { "Hello, world!" }
    ))
}

