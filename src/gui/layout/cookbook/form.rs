use dioxus::prelude::*;

use crate::gui::form::{FieldLabel, TextField};

pub struct NewCookbookForm {
    pub name: Signal<String>,
}

pub fn valid_new_cookbook_form(form: &NewCookbookForm) -> bool {
    let name = form.name.peek();
    let is_err = validate_name(&name).is_err();
    !is_err
}

pub fn validate_name(name: &str) -> Result<(), &'static str> {
    if name.len() == 0 {
        Err("Please enter a name.")
    } else {
        Ok(())
    }
}

pub fn new_cookbook_form() -> (Element, NewCookbookForm) {
    let mut name = use_signal(|| "".to_string());

    let form_state = NewCookbookForm {
        name,
    };

    let view = rsx! (
            div {
                class: "w-full p-3",
                FieldLabel {
                    label: "Name",
                    id: "cookbookname",
                    field: rsx!( TextField {
                        placeholder: "Cookbook name",
                        id: "cookbookname",
                        value: name,
                        oninput: move |evt: Event<FormData>| name.set(evt.value()),
                        validation_fn: validate_name,
                    })
                }
            }
    );
    (view, form_state)
}
