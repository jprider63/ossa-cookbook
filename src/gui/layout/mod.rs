pub mod recipe;

use std::ops::Deref;

use dioxus::events::MouseEvent;
use dioxus_markdown::Markdown;
use dioxus::prelude::*;
use dioxus_heroicons::{Icon, solid::Shape};
// TODO: Fix outline icons.

use crate::gui::layout::recipe::form::{recipe_form, valid_recipe_form};
use crate::state::{Cookbook, CookbookId, RecipeId, Recipe, RecipeOp};

use crate::{CookbookApplication, UseStore};

enum View {
    Login,
    NoSelection,
    Cookbook(CookbookId),
    CookbookRecipe(CookbookId, RecipeId),
    CookbookRecipeNew(CookbookId),
    CookbookRecipeEdit(CookbookId, RecipeId),
}

fn is_cookbook_selected(view: &View, cid: CookbookId) -> bool {
    match view {
        View::Login => false,
        View::NoSelection => false,
        View::Cookbook(vid) => *vid == cid,
        View::CookbookRecipe(vid, _rid) => *vid == cid,
        View::CookbookRecipeNew(vid) => *vid == cid,
        View::CookbookRecipeEdit(vid, _rid) => *vid == cid,
    }
}

#[component]
// pub fn layout(cx: Scope, state: Vec<UseStore<CookbookApplication, Cookbook>>) -> Element {
pub fn layout(state: Signal<Vec<UseStore<CookbookApplication, Cookbook>>>) -> Element {
    let view = use_signal(|| {
        View::NoSelection
    });

    let v = view.read();
    let r = match &*v {
        // View::Login => todo!(),
        View::NoSelection => rsx! ( NoSelectionView { view: view, state: state } ),
        // View::Cookbook(cookbookid) => rsx! ( CookbookView { view: view, state: state, cookbook_id: *cookbookid } ),
        View::CookbookRecipe(cookbookid, recipeid) => rsx! ( CookbookRecipeView { view: view, state: state, cookbook_id: *cookbookid, recipe_id: recipeid.clone() } ),
        // View::CookbookRecipeNew(cookbookid) => rsx! ( CookbookRecipeNewView { view: view, state: state, cookbook_id: *cookbookid } ),
        // View::CookbookRecipeEdit(cookbookid, recipeid) => rsx! ( CookbookRecipeEditView { view: view, state: state, cookbook_id: *cookbookid, recipe_id: recipeid.clone() } ),
        _ => todo!(),
    };
    rsx!(
        div {
            class: "wrapper",
            { r }
        }
    )
}

#[component]
fn NoSelectionView(view: Signal<View>, state: Signal<Vec<UseStore<CookbookApplication,Cookbook>>>) -> Element {
    rsx! (
        Sidebar { view: view, state: state }
        div {
            class: "content",
            div {
                class: "flex justify-center items-center h-screen",
                "No selection" // No selection. | New cookbook | | New meal planner |
            }
        }
    )
}

fn get_cookbook_store(mut view: Signal<View>, state: &std::rc::Rc<Vec<UseStore<CookbookApplication, Cookbook>>>, cookbook_id: CookbookId) -> Option<&UseStore<CookbookApplication, Cookbook>> {
    let cookbook = state.get(cookbook_id);
    if cookbook.is_none() {
        // Cookbook not found, so set no selection.
        // TODO: Log this and display error.
        println!("Cookbook {} not found.", cookbook_id);

        view.set(View::NoSelection);
    }
    cookbook
}

fn get_cookbook<'a>(view: Signal<View>, cookbook_store: &UseStore<CookbookApplication, Cookbook>) -> &Cookbook {
    todo!()
// fn get_cookbook<'a,'b>(view: &'a UseState<View>, state: &'a UseState<Vec<Cookbook>>, cookbook_id: CookbookId) -> Option<&'b Cookbook> {
    // let cookbook = state.get(cookbook_id);
    // if cookbook.is_none() {
    //     // Cookbook not found, so set no selection.
    //     // TODO: Log this and display error.
    //     println!("Cookbook {} not found.", cookbook_id);

    //     view.set(View::NoSelection);
    // }
    // cookbook
}


fn get_recipe(mut view: Signal<View>, cookbook: &Cookbook, recipe_id: RecipeId) -> Option<&Recipe> {
    let recipe = cookbook.recipes.get(&recipe_id);
    if recipe.is_none() {
        // Recipe not found, so set no selection.
        // TODO: Log this and display error.
        println!("Recipe {:?} not found.", recipe_id);

        view.set(View::NoSelection);
    }
    recipe
}

// #[inline_props]
// fn CookbookView<'a>(cx: Scope, view: &'a UseState<View>, state: &'a UseState<Vec<Cookbook>>, cookbook_id: CookbookId) -> Element {
//     let current_state = state.current();
//     let cookbook = get_cookbook(view, &current_state, *cookbook_id)?;
// 
//     let pills = cookbook.recipes.iter().map(|(recipe_id, recipe)| { rsx! (
//         RecipePill { view: view, cookbook_id: *cookbook_id, recipe_id: recipe_id.clone(), recipe: recipe.clone() } // TODO: Can we avoid this clone?
//     )});
//     cx.render(rsx! (
//         Sidebar { view: view, state: state }
//         div {
//             class: "content",
//             div {
//                 class: "flex justify-center",
//                 h1 {
//                     class: "text-3xl font-bold mt-4 mb-6 text-center",
//                     "{cookbook.title.value()}"
//                 }
//             }
//             div {
//                 class: "flex flex-row flex-wrap",
//                 pills
//                 div {
//                     class: "basis-1/3",
//                     div {
//                         class: "recipe-card",
//                         onclick: |_e| {view.set(View::CookbookRecipeNew(*cookbook_id))},
//                         div {
//                             class: "new-recipe",
//                             Icon {
//                                 class: "w-14 h-14",
//                                 icon: Shape::Plus,
//                             }
//                         }
//                         p {
//                             class: "p-5 text-center",
//                             "New Recipe"
//                         }
//                     }
//                 }
//             }
//         }
//     ))
// }

#[component]
fn CookbookRecipeView(view: Signal<View>, state: Signal<Vec<UseStore<CookbookApplication, Cookbook>>>, cookbook_id: CookbookId, recipe_id: RecipeId) -> Element {
    let current_state = state.read();
    let cookbook_store = todo!(); // get_cookbook_store(view, &current_state, *cookbook_id)?;
    let cookbook = get_cookbook(view, cookbook_store);
    let recipe = get_recipe(view, cookbook, recipe_id)?;

    rsx! (
        Sidebar { view: view, state: state }
        div {
            class: "content",
            nav {
                class: "flex w-full mt-4 mb-6",
                div {
                    class: "flex-1 flex justify-start mr-auto whitespace-nowrap",
                    div {
                        class: "text-blue-500 hover:text-blue-400 inline-flex items-center px-3",
                        onclick: |_e| {view.set(View::Cookbook(cookbook_id))},
                        Icon {
                            class: "w-6 h-6",
                            icon: Shape::ChevronLeft,
                        },
                        span {
                            "{cookbook.title.value()}"
                        }
                    }
                }
                div {
                    class: "whitespace-nowrap",
                    h1 {
                        class: "text-3xl font-bold text-center",
                            "{recipe.title.value()}"
                    }
                }
                div {
                    class: "flex-1 flex justify-end ml-auto whitespace-nowrap",
                    div {
                        class: "text-blue-500 hover:text-blue-400 inline-flex items-center px-3",
                        onclick: |_e| {view.set(View::CookbookRecipeEdit(cookbook_id, recipe_id.clone()))},
                        span {
                            "Edit"
                        }
                    }
                }
            }
            // div {
            //     class: "p-3",
            //     "TODO: Image carousel"
            // }
            div {
                class: "p-3",
                h2 {
                    class: "text-xl font-bold",
                    "Ingredients"
                }
                ul {
                    class: "selectable",
                    { recipe.ingredients.value().iter().map(|ingredient| rsx! (
                        li {
                            "{ingredient}"
                        }
                    )) }
                }
            }
            div {
                class: "p-3",
                h2 {
                    class: "text-xl font-bold",
                    "Instructions"
                }
                Markdown {
                    class: use_signal(|| "instructions selectable".to_string()),
                    content: "{recipe.instructions.value()}",
                }
            }
        }
    )
}

// #[inline_props]
// fn CookbookRecipeNewView<'a>(cx: Scope, view: &'a UseState<View>, state: &'a UseState<Vec<Cookbook>>, cookbook_id: CookbookId) -> Element {
//     let current_state = state.current();
//     let cookbook = get_cookbook(view, &current_state, *cookbook_id)?;
//     unimplemented!()
// }

// #[inline_props]
// fn CookbookRecipeEditView<'a>(cx: Scope, view: &'a UseState<View>, state: &'a UseState<Vec<Cookbook>>, cookbook_id: CookbookId, recipe_id: RecipeId) -> Element {
//     let cookbooks = state.current();
//     let cookbook = get_cookbook(view, &cookbooks, *cookbook_id)?;
//     let recipe = get_recipe(view, &cookbook, recipe_id.clone())?;
// 
//     let old_recipe: Recipe = recipe.clone();
//     let old_cookbook: Cookbook = cookbook.clone();
// 
//     let (form, form_state) = recipe_form(cx, &old_recipe);
// 
//     let save_handler = move |mut _e| {
//         // Validate all fields.
//         if !valid_recipe_form(&form_state) {
//             return;
//         }
// 
//         let mut ops = Vec::new(); // cookbook.create_batch_operations();
// 
//         // Diff all fields.
//         let mut new_recipe = old_recipe.clone();
//         let new_name = form_state.name.get();
//         let new_ingredients = form_state.ingredients.get();
//         let new_instructions = form_state.instructions.get();
//         if old_recipe.title.value() != new_name {
//             // new_recipe.title = new_name.clone();
// 
//             ops.push(RecipeOp::Title(new_name.clone()));
//             // let op = TwoPMapOp::Apply {
//             //     key: recipe_id,
//             //     operation: RecipeOp::TitleOp {
//             //         time: pending_ops.time(),
//             //         value: new_name,
//             //     },
//             // };
//             // let _id = pending_ops.append(op); // Take a closure that gives you the current "time"?
//         }
//         if old_recipe.ingredients.value() != new_ingredients {
//             // new_recipe.ingredients = new_ingredients.clone();
//             ops.push(RecipeOp::Ingredients(new_ingredients.clone()));
//         }
//         if old_recipe.instructions.value() != new_instructions {
//             // new_recipe.instructions = new_instructions.clone();
//             ops.push(RecipeOp::Instructions(new_instructions.clone()));
//         }
// 
//         // // Save updated fields by applying CRDT operations.
//         // cookbook.apply_batch_operations(pending_ops);
// 
//         // TODO: Send CRDT operations
//         // let mut new_cookbook = old_cookbook.clone();
//         // new_cookbook.recipes.insert(*recipe_id, new_recipe);
//         // let mut new_cookbooks: Vec<Cookbook> = cookbooks.clone().to_vec();
//         // new_cookbooks[*cookbook_id] = new_cookbook;
//         // state.set(new_cookbooks);
// 
//         view.set(View::CookbookRecipe(*cookbook_id, recipe_id.clone()));
//     };
// 
//     cx.render(rsx! (
//         Sidebar { view: view, state: state }
//         div {
//             class: "content",
//             nav {
//                 class: "flex w-full mt-4 mb-6",
//                 div {
//                     class: "flex-1 flex justify-start mr-auto whitespace-nowrap",
//                     div {
//                         class: "text-blue-500 hover:text-blue-400 inline-flex items-center px-3",
//                         onclick: |_e| {view.set(View::CookbookRecipe(*cookbook_id, recipe_id.clone()))},
//                         Icon {
//                             class: "w-6 h-6",
//                             icon: Shape::ChevronLeft,
//                         },
//                         span {
//                             "Cancel" // "{recipe.title}"
//                         }
//                     }
//                 }
//                 div {
//                     class: "whitespace-nowrap",
//                     h1 {
//                         class: "text-3xl font-bold text-center",
//                             "Edit Recipe"
//                     }
//                 }
//                 div {
//                     class: "flex-1 flex justify-end ml-auto whitespace-nowrap",
//                     div {
//                         class: "text-blue-500 hover:text-blue-400 inline-flex items-center px-3",
//                         onclick: save_handler,
//                         span {
//                             "Save"
//                         }
//                     }
//                 }
//             }
//             form
//         }
//     ))
// }

// #[component]
// fn RecipePill(view: Signal<View>, cookbook_id: CookbookId, recipe_id: RecipeId, recipe: Recipe) -> Element {
//     rsx! (
//         div {
//             class: "basis-1/3",
//             div {
//                 class: "recipe-card",
//                 onclick: |_e| {view.set(View::CookbookRecipe(cookbook_id, recipe_id))},
//                 img {
//                     src: "https://food.fnr.sndimg.com/content/dam/images/food/fullset/2008/8/14/0/GT0107_kalbi_s4x3.jpg.rend.hgtvcom.1280.720.suffix/1519669666497.jpeg"
//                 }
//                 div {
//                     class: "border-t",
//                     p {
//                         class: "p-5 text-center",
//                         "{recipe.title.value()}"
//                     }
//                 }
//             }
//         }
//     )
// }

#[component]
fn Sidebar(view: Signal<View>, state: Signal<Vec<UseStore<CookbookApplication, Cookbook>>>) -> Element {
    let cookbooks = state.read();
    let cookbooks = cookbooks.iter().filter_map(|cookbook_store|

            cookbook_store.get_current_state()() // WTF is this syntax!!!
        ).enumerate().map(|(i,cookbook)| {
            rsx!(
                SidebarItem { title: cookbook.title.value(), icon: Shape::BookOpen, selected: is_cookbook_selected(&view.read(), i), onclick: move |_e| {view.set(View::Cookbook(i))} }
            )
    });

    rsx! (
        div {
            class: "sidebar",
            div {
                class: "overflow-y-auto overflow-x-hidden flex-grow",
                ul {
                    class: "flex flex-col py-4 space-y-1",
                    SidebarHeader { title: "COOKBOOKS" }
                    { cookbooks }
                    SidebarHeader { title: "MEAL PLANNER" }
                    SidebarItem   { title: "Weekly meals", icon: Shape::PencilSquare, onclick: |_e| {println!("TODO!")}, selected: false }
                    SidebarItem   { title: "Thanksgiving", icon: Shape::PencilSquare, selected: false, onclick: |_e| {println!("TODO!")} }
                    SidebarHeader { title: "SETTINGS" }
                    SidebarItem   { title: "Account", icon: Shape::User, selected: false, onclick: |_e| {println!("TODO!")} }
                    SidebarItem   { title: "Logout", icon: Shape::ArrowRightOnRectangle, selected: false, onclick: |_e| {println!("TODO!")} }
                }
            }
        }
    )
}

#[derive(Props, Clone, PartialEq)]
pub struct SidebarHeaderProps { // <'a> {
    title: String, // &'a str
}

pub fn SidebarHeader(props: SidebarHeaderProps) -> Element {
    rsx! (
        li {
            class: "px-5",
            div {
                class: "flex flex-row items-center h-8",
                div {
                    class: "text-sm font-light tracking-wide text-gray-500",
                    "{props.title}"
                }
            }
        }
    )
}

#[derive(Props, Clone, PartialEq)]
pub struct SidebarItemProps {
    title: String, // &'a str,
    icon: Shape,
    onclick: EventHandler<MouseEvent>,
    selected: bool,
}

pub fn SidebarItem<'a>(props: SidebarItemProps) -> Element {
    let is_selected = if props.selected {"selected"} else {""};
    rsx! (
        li {
            div {
                class: format_args!("sidebar-item {}", is_selected),
                onclick: move |e| props.onclick.call(e),
                span {
                    class: "inline-flex justify-center items-center ml-4",
                    Icon {
                        class: "w-6 h-6",
                        icon: props.icon,
                    }
                }
                span {
                    class: "ml-2 text-sm tracking-wide truncate",
                    "{props.title}"
                }
            }
        }
    )
}
