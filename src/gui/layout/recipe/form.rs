use dioxus::prelude::*;
use keyboard_types::Key;

use crate::gui::form::{FieldLabel, TextField};
use crate::state::Recipe;

pub struct RecipeForm<'a> {
    pub name: &'a UseState<String>,
    pub ingredients: &'a UseState<Vec<String>>,
    // TODO: new_ingredient?
    pub instructions: &'a UseState<String>,
    // TODO: new_instruction?
}

pub fn recipe_form<'a, P>(cx: &'a Scoped<'a, P>, initial_recipe: Recipe) -> (Element, RecipeForm<'a>) {
    let name = use_state(&cx, || initial_recipe.title.clone());
    fn validate_name(name: &str) -> Result<(), &'static str> {
        if name.len() == 0 {
            Err("Please enter a name.")
        } else {
            Ok(())
        }
    }

    let ingredients = use_state(&cx, || initial_recipe.ingredients.clone());
    let new_ingredient: &UseState<String> = use_state(&cx, || "".into());
    fn validate_ingredient(ingredient: &str) -> Result<(), &'static str> {
        // if ingredient.len() == 0 {
        //     Err("Please enter an ingredient.")
        // } else {
            Ok(())
        // }
    }
    // let new_ingredient_err = validate_ingredient(new_ingredient.get());

    let instructions = use_state(&cx, || initial_recipe.instructions.clone());
    fn validate_instructions(instructions: &str) -> Result<(), &'static str> {
        if instructions.len() == 0 {
            Err("Please enter instructions.")
        } else {
            Ok(())
        }
    }
    let instructions_err = validate_instructions(instructions.get());

    // let save_handler = move |mut _e| {
    //     // Validate all fields.
    //     let new_name = name.get();
    //     let new_ingredients = ingredients.get();
    //     let new_instructions = instructions.get();
    //     if validate_name(new_name).is_err()
    //     || new_ingredients.iter().any(|i| validate_ingredient(i).is_err())
    //     || validate_instructions(new_instructions).is_err() {
    //         return;
    //     }

    //     unimplemented!{};
    // };

    let form_state = RecipeForm {
        name: name,
        ingredients: ingredients,
        instructions: instructions,
    };

    let view = cx.render(rsx! (
            div {
                class: "w-full p-3",
                FieldLabel {
                    label: "Name",
                    id: "recipename",
                    field: cx.render(rsx!( TextField {
                        placeholder: "Recipe name",
                        id: "recipename",
                        value: name.get(),
                        oninput: move |evt: Event<FormData>| name.set(evt.value.clone()),
                        validation_fn: validate_name,
                    }))
                }
                // div {
                //     class: "flex flex-col mb-4",
                //     "TODO: Images",
                // }
                FieldLabel {
                    label: "Ingredients",
                    id: "recipeingredients-0",
                    field: cx.render(rsx!(
                        ingredients.iter().enumerate().map(|(idx,ingredient)| {
                            cx.render(rsx!(
                                div {
                                    class: "mb-1 w-full",
                                    TextField {
                                        placeholder: "Ingredient",
                                        class: "w-full",
                                        id: "recipeingredients-{idx}",
                                        value: ingredient,
                                        oninput: move |evt: Event<FormData>| ingredients.with_mut(|a| a[idx] = evt.value.clone()),
                                        validation_fn: validate_ingredient,
                                    }
                                }
                            ))
                        })
                        TextField {
                            placeholder: "Add ingredient...",
                            id: "recipeingredients-{ingredients.len()}",
                            value: new_ingredient.get(),
                            oninput: move |evt: Event<FormData>| new_ingredient.set(evt.value.clone()),
                            onkeyup: move |evt: Event<KeyboardData>| {
                                let i = new_ingredient.get();
                                if evt.key() == Key::Enter && validate_ingredient(i).is_ok() {
                                    ingredients.with_mut(|a| a.push(i.clone()));
                                    new_ingredient.set("".to_string());
                                }
                            }
                            validation_fn: validate_ingredient,
                        }
                    ))
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
                        oninput: move |evt| instructions.set(evt.value.clone()),
                        value: "{instructions}"
                    }
                    instructions_err.err().map(|err| rsx!(
                        p {
                            class: "text-red-500 text-sm",
                            "{err}"
                        }
                    ))
                }
            }
    ));
    (view, form_state)
}

