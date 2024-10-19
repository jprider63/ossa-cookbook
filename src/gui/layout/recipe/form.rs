use dioxus::prelude::*;
use keyboard_types::Key;

use crate::gui::form::{FieldLabel, TextField};
use crate::state::Recipe;

pub struct RecipeForm {
    pub name: Signal<String>,
    pub ingredients: Signal<Vec<String>>,
    // TODO: new_ingredient?
    pub instructions: Signal<String>,
}

pub fn valid_recipe_form(recipe_form: &RecipeForm) -> bool {
    let new_name = recipe_form.name.peek();
    let new_ingredients = recipe_form.ingredients.peek();
    // TODO: new_ingredient?
    let new_instructions = recipe_form.instructions.peek();
    let is_err = validate_name(&new_name).is_err()
        || new_ingredients
            .iter()
            .any(|i| validate_ingredient(i).is_err())
        || validate_instructions(&new_instructions).is_err();
    !is_err
}

pub fn validate_name(name: &str) -> Result<(), &'static str> {
    if name.len() == 0 {
        Err("Please enter a name.")
    } else {
        Ok(())
    }
}

pub fn validate_ingredient(ingredient: &str) -> Result<(), &'static str> {
    // if ingredient.len() == 0 {
    //     Err("Please enter an ingredient.")
    // } else {
    Ok(())
    // }
}

pub fn validate_instructions(instructions: &str) -> Result<(), &'static str> {
    if instructions.len() == 0 {
        Err("Please enter instructions.")
    } else {
        Ok(())
    }
}

pub fn recipe_form(initial_recipe: &Recipe) -> (Element, RecipeForm) {
    let mut name = use_signal(|| initial_recipe.title.value().clone());

    let mut ingredients = use_signal(|| initial_recipe.ingredients.value().clone());
    let mut new_ingredient: Signal<String> = use_signal(|| "".into());

    let mut instructions = use_signal(|| initial_recipe.instructions.value().clone());
    let instructions_err = validate_instructions(&instructions.read());

    let form_state = RecipeForm {
        name,
        ingredients,
        instructions,
    };

    let view = rsx! (
            div {
                class: "w-full p-3",
                FieldLabel {
                    label: "Name",
                    id: "recipename",
                    field: rsx!( TextField {
                        placeholder: "Recipe name",
                        id: "recipename",
                        value: name,
                        oninput: move |evt: Event<FormData>| name.set(evt.value()),
                        validation_fn: validate_name,
                    })
                }
                // div {
                //     class: "flex flex-col mb-4",
                //     "TODO: Images",
                // }
                FieldLabel {
                    label: "Ingredients",
                    id: "recipeingredients-0",
                    field: rsx!(
                        { ingredients.iter().enumerate().map(|(idx,ingredient)| {
                            rsx!(
                                div {
                                    class: "mb-1 w-full",
                                    TextField {
                                        placeholder: "Ingredient",
                                        class: "w-full",
                                        id: "recipeingredients-{idx}",
                                        value: ingredient,
                                        oninput: move |evt: Event<FormData>| ingredients.with_mut(|a| a[idx] = evt.value()),
                                        validation_fn: validate_ingredient,
                                    }
                                }
                            )
                        }) }
                        TextField {
                            placeholder: "Add ingredient...",
                            id: "recipeingredients-{ingredients.len()}",
                            value: new_ingredient,
                            oninput: move |evt: Event<FormData>| new_ingredient.set(evt.value()),
                            onkeyup: move |evt: Event<KeyboardData>| {
                                let i = new_ingredient.read().clone();
                                if evt.key() == Key::Enter && validate_ingredient(&i).is_ok() {
                                    ingredients.with_mut(|a| a.push(i));
                                    new_ingredient.set("".to_string());
                                }
                            },
                            validation_fn: validate_ingredient,
                        }
                    )
                }
                div {
                    class: "flex flex-col mb-4",
                    label {
                        class: "font-bold mb-2",
                        r#for: "recipeinstructions",
                        "Instructions"
                    }
                    textarea {
                        class: format_args!("appearance-none border rounded py-1 px-2 {}", if instructions_err.is_err() {"border-red-500"} else {""}),
                        r#id: "recipeinstructions",
                        r#rows: 10,
                        autocomplete: "false",
                        "autocorrect": "false",
                        autocapitalize: "false",
                        spellcheck: "false",
                        placeholder: "Instructions",
                        oninput: move |evt| instructions.set(evt.value()),
                        value: "{instructions}"
                    }
                    { instructions_err.err().map(|err| rsx!(
                        p {
                            class: "text-red-500 text-sm",
                            "{err}"
                        }
                    )) }
                }
            }
    );
    (view, form_state)
}
