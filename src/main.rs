use dioxus::prelude::*;
use crate::state::*;

mod gui;
mod state;

fn main() {
    dioxus::desktop::launch(app);
}

fn app(cx: Scope) -> Element {
    let state = use_state(&cx, || {
        let book1 = Cookbook {title: "Family Recipes".into(), recipes: vec![]};
        let book2 = Cookbook {title: "My Recipes".into(), recipes: vec![]};
        vec![book1, book2]
        // TODO: Should be a Map CRDT. Include other store metadata like sharing/permissions, peers, etc
    });

    cx.render(rsx! (
        style { [include_str!("../dist/style.css")] }

        rsx! (
            gui::layout::layout { state: state }
        )
    ))
}

