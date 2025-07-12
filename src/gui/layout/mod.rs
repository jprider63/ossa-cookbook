pub mod recipe;

use dioxus::events::MouseEvent;
use dioxus::prelude::*;
use dioxus_heroicons::{solid::Shape, Icon};
use dioxus_markdown::Markdown;
// TODO: Fix outline icons.

use odyssey_crdt::map::twopmap::TwoPMapOp;
use tracing::{debug, error, warn};

use crate::gui::layout::recipe::form::{recipe_form, valid_recipe_form};
use crate::state::{Cookbook, CookbookId, CookbookOp, Recipe, RecipeId, RecipeOp, State};

use crate::{new_store_in_scope, CookbookApplication, OdysseyProp, UseStore};

enum View {
    Login,
    NoSelection,
    Cookbook(CookbookId),
    CookbookRecipe(CookbookId, RecipeId),
    CookbookRecipeNew(CookbookId),
    CookbookRecipeEdit(CookbookId, RecipeId),
    ConnectToPeer,
}

fn is_cookbook_selected(view: &View, cid: CookbookId) -> bool {
    match view {
        View::Login => false,
        View::NoSelection => false,
        View::ConnectToPeer => false,
        View::Cookbook(vid) => *vid == cid,
        View::CookbookRecipe(vid, _rid) => *vid == cid,
        View::CookbookRecipeNew(vid) => *vid == cid,
        View::CookbookRecipeEdit(vid, _rid) => *vid == cid,
    }
}

#[component]
// pub fn layout(cx: Scope, state: Vec<UseStore<CookbookApplication, Cookbook>>) -> Element {
pub fn layout(
    state: Signal<Vec<UseStore<CookbookApplication, Cookbook>>>,
    root_scope: ScopeId,
) -> Element {
    let view = use_signal(|| View::NoSelection);

    let v = view.read();
    let r = match &*v {
        View::Login => todo!(),
        View::NoSelection => rsx!(NoSelectionView {
            view: view,
            state: state
        }),
        View::Cookbook(cookbookid) => rsx!(CookbookView {
            view: view,
            state: state,
            cookbook_id: *cookbookid
        }),
        View::CookbookRecipe(cookbookid, recipeid) => rsx!(CookbookRecipeView {
            view: view,
            state: state,
            cookbook_id: *cookbookid,
            recipe_id: recipeid.clone()
        }),
        View::CookbookRecipeNew(cookbookid) => rsx!(CookbookRecipeNewView {
            view: view,
            state: state,
            cookbook_id: *cookbookid
        }),
        View::CookbookRecipeEdit(cookbookid, recipeid) => rsx!(CookbookRecipeEditView {
            view: view,
            state: state,
            cookbook_id: *cookbookid,
            recipe_id: recipeid.clone()
        }),
        View::ConnectToPeer => rsx!(ConnectToPeerView {
            view: view,
            state: state,
            root_scope,
        }),
    };
    rsx!(
        div {
            class: "wrapper",
            { r }
        }
    )
}

#[component]
fn NoSelectionView(
    view: Signal<View>,
    state: Signal<State>,
) -> Element {
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

fn get_cookbook_store(
    mut view: Signal<View>,
    state: Signal<State>,
    cookbook_id: CookbookId,
) -> Option<UseStore<CookbookApplication, Cookbook>> {
    let cookbook = state.with(|state| state.get(cookbook_id).cloned()); // TODO: Can we avoid this clone? Return a ref?
    if cookbook.is_none() {
        // Cookbook not found, so set no selection.
        // TODO: Log this and display error.
        error!("Cookbook {} not found.", cookbook_id);

        view.set(View::NoSelection);
    }
    cookbook
}

fn get_recipe(mut view: Signal<View>, cookbook: &Cookbook, recipe_id: RecipeId) -> Option<&Recipe> {
    let recipe = cookbook.recipes.get(&recipe_id);
    if recipe.is_none() {
        // Recipe not found, so set no selection.
        // TODO: Log this and display error.
        error!("Recipe {:?} not found.", recipe_id);

        view.set(View::NoSelection);
    }
    recipe
}

#[component]
fn CookbookView(view: Signal<View>, state: Signal<State>, cookbook_id: CookbookId) -> Element {
    let cookbook_store = get_cookbook_store(view, state, cookbook_id).expect("TODO"); // ?;
    if let Some(cookbook) = cookbook_store.get_current_state() {

        let pills = cookbook.recipes.iter().map(|(recipe_id, recipe)| {
            rsx!(
                RecipePill {
                    view: view,
                    cookbook_id: cookbook_id,
                    recipe_id: *recipe_id,
                    recipe: recipe.clone()
                } // TODO: Can we avoid this clone?
            )
        });
        rsx! (
            Sidebar { view: view, state: state }
            div {
                class: "content",
                div {
                    class: "flex justify-center",
                    h1 {
                        class: "text-3xl font-bold mt-4 mb-6 text-center",
                        "{cookbook.title.value()}"
                    }
                }
                div {
                    class: "flex flex-row flex-wrap",
                    { pills },
                    div {
                        class: "basis-1/3",
                        div {
                            class: "recipe-card",
                            onclick: move |_e| {view.set(View::CookbookRecipeNew(cookbook_id))},
                            div {
                                class: "new-recipe",
                                Icon {
                                    class: "w-14 h-14",
                                    icon: Shape::Plus,
                                }
                            }
                            p {
                                class: "p-5 text-center",
                                "New Recipe"
                            }
                        }
                    }
                }
            }
        )
    } else {
        rsx! (
            Sidebar { view: view, state: state }
            div {
                class: "content",
                div {
                    class: "flex justify-center items-center h-screen",
                    "Downloading..."
                }
            }
        )
    }
}

#[component]
fn CookbookRecipeView(
    view: Signal<View>,
    state: Signal<State>,
    cookbook_id: CookbookId,
    recipe_id: RecipeId,
) -> Element {
    let cookbook_store = get_cookbook_store(view, state, cookbook_id).expect("TODO"); // ?;
    let cookbook = cookbook_store.get_current_state().expect("TODO"); // ?;
    let recipe = get_recipe(view, &cookbook, recipe_id).expect("TODO"); // ?;

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
                        onclick: move |_e| {view.set(View::Cookbook(cookbook_id))},
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
                        onclick: move |_e| {view.set(View::CookbookRecipeEdit(cookbook_id, recipe_id.clone()))},
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

#[component]
fn CookbookRecipeNewView(
    view: Signal<View>,
    state: Signal<State>,
    cookbook_id: CookbookId,
) -> Element {
    let cookbook_store = get_cookbook_store(view, state, cookbook_id).expect("TODO"); // ?;
    let cookbook = cookbook_store.get_current_state().expect("TODO"); // ?;
    rsx!(
        Sidebar { view: view, state: state }
        div {
            class: "content",
            p {
                "TODO: Create recipe function"
            }
        }
    )
}

#[component]
fn CookbookRecipeEditView(
    view: Signal<View>,
    state: Signal<State>,
    cookbook_id: CookbookId,
    recipe_id: RecipeId,
) -> Element {
    let mut cookbook_store = get_cookbook_store(view, state, cookbook_id).expect("TODO"); // ?;
    let cookbook_store_state = cookbook_store.get_current_store_state().expect("TODO"); // ?;
    let old_cookbook = cookbook_store_state.state;
    let parent_header_ids = cookbook_store_state.ecg.tips().clone();

    let old_recipe = get_recipe(view, &old_cookbook, recipe_id).expect("TODO"); // ?;

    let old_recipe = old_recipe.clone();
    let (form, form_state) = recipe_form(&old_recipe);

    let save_handler = move |mut _e| {
        // Validate all fields.
        if !valid_recipe_form(&form_state) {
            return;
        }

        let mut pending_ops = Vec::new(); // cookbook.create_batch_operations();

        // Diff all fields.
        let new_name = form_state.name.peek();
        let new_ingredients = form_state.ingredients.peek();
        let new_instructions = form_state.instructions.peek();
        if *old_recipe.title.value() != *new_name {
            pending_ops.push(RecipeOp::Title(new_name.clone()));
        }
        if *old_recipe.ingredients.value() != *new_ingredients {
            pending_ops.push(RecipeOp::Ingredients(new_ingredients.clone()));
        }
        if *old_recipe.instructions.value() != *new_instructions {
            pending_ops.push(RecipeOp::Instructions(new_instructions.clone()));
        }

        // Save updated fields by applying CRDT operations.
        // cookbook_store.apply_batch_operations(pending_ops);
        let ops = pending_ops
            .into_iter()
            .map(|op| {
                CookbookOp::Recipes(TwoPMapOp::Apply {
                    key: recipe_id,
                    operation: op,
                })
            })
            .collect();
        debug!("ops: {:?}", ops);
        cookbook_store.apply_batch(parent_header_ids.clone(), ops);

        // TODO: Send CRDT operations
        // let mut new_cookbook = old_cookbook.clone();
        // new_cookbook.recipes.insert(*recipe_id, new_recipe);
        // let mut new_cookbooks: Vec<Cookbook> = cookbooks.clone().to_vec();
        // new_cookbooks[*cookbook_id] = new_cookbook;
        // state.set(new_cookbooks);

        view.set(View::CookbookRecipe(cookbook_id, recipe_id.clone()));
    };

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
                        onclick: move |_e| {view.set(View::CookbookRecipe(cookbook_id, recipe_id))},
                        Icon {
                            class: "w-6 h-6",
                            icon: Shape::ChevronLeft,
                        },
                        span {
                            "Cancel" // "{recipe.title}"
                        }
                    }
                }
                div {
                    class: "whitespace-nowrap",
                    h1 {
                        class: "text-3xl font-bold text-center",
                            "Edit Recipe"
                    }
                }
                div {
                    class: "flex-1 flex justify-end ml-auto whitespace-nowrap",
                    div {
                        class: "text-blue-500 hover:text-blue-400 inline-flex items-center px-3",
                        onclick: save_handler,
                        span {
                            "Save"
                        }
                    }
                }
            }
            { form }
        }
    )
}

#[component]
fn RecipePill(
    view: Signal<View>,
    cookbook_id: CookbookId,
    recipe_id: RecipeId,
    recipe: Recipe,
) -> Element {
    rsx! (
        div {
            class: "basis-1/3",
            div {
                class: "recipe-card",
                onclick: move |_e| {view.set(View::CookbookRecipe(cookbook_id, recipe_id))},
                img {
                    src: "https://food.fnr.sndimg.com/content/dam/images/food/fullset/2008/8/14/0/GT0107_kalbi_s4x3.jpg.rend.hgtvcom.1280.720.suffix/1519669666497.jpeg"
                }
                div {
                    class: "border-t",
                    p {
                        class: "p-5 text-center",
                        "{recipe.title.value()}"
                    }
                }
            }
        }
    )
}

#[component]
fn Sidebar(
    view: Signal<View>,
    state: Signal<State>,
) -> Element {
    let cookbooks = state.read();
    let cookbooks = cookbooks
        .iter()
        .enumerate()
        .map(|(i, cookbook_store)| {
            if let Some(title) = cookbook_store.get_current_state().map(|cookbook| cookbook.title.value().clone()) {
                rsx!(SidebarItem {
                    title: title,
                    icon: Shape::BookOpen,
                    selected: is_cookbook_selected(&view.read(), i),
                    onclick: move |_e| { view.set(View::Cookbook(i)) }
                })
            } else {
                rsx!(SidebarItem {
                    title: "Downloading...",
                    icon: Shape::BookOpen,
                    selected: is_cookbook_selected(&view.read(), i),
                    onclick: move |_e| { view.set(View::Cookbook(i)) }
                })
            }
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
                    SidebarItem   { title: "Weekly meals", icon: Shape::PencilSquare, onclick: |_e| {warn!("TODO!")}, selected: false }
                    SidebarItem   { title: "Thanksgiving", icon: Shape::PencilSquare, selected: false, onclick: |_e| {warn!("TODO!")} }
                    SidebarHeader { title: "SETTINGS" }
                    SidebarItem   { title: "Account", icon: Shape::User, selected: false, onclick: |_e| {warn!("TODO!")} }
                    SidebarItem   { title: "Logout", icon: Shape::ArrowRightOnRectangle, selected: false, onclick: |_e| {warn!("TODO!")} }
                    SidebarItem   { title: "Connections (TMP)", icon: Shape::Users, selected: false, onclick: move |_e| { view.set(View::ConnectToPeer) } }
                }
            }
        }
    )
}

#[derive(Props, Clone, PartialEq)]
pub struct SidebarHeaderProps {
    // <'a> {
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
    let is_selected = if props.selected { "selected" } else { "" };
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

#[component]
fn ConnectToPeerView(
    view: Signal<View>,
    state: Signal<State>,
    root_scope: ScopeId,
) -> Element {
    let odyssey = use_context::<OdysseyProp<CookbookApplication>>().odyssey;
    let connect_handler = move |_| {
        odyssey.connect_to_peer_ipv4("127.0.0.1:8080".parse().unwrap());
    };

    rsx! (
        Sidebar { view: view, state: state }
        div {
            class: "content",
            div {
                class: "text-blue-500 hover:text-blue-400 inline-flex items-center px-3",
                onclick: connect_handler,
                span {
                    "Connect to peer 127.0.0.1:8080"
                }
            }
            ConnectToStoreView {
                view,
                state,
                root_scope,
            }
        }
    )
}

#[component]
fn ConnectToStoreView(
    view: Signal<View>,
    state: Signal<State>,
    root_scope: ScopeId,
) -> Element {
    use crate::gui::form::TextField;

    pub fn validate_store_id(store_id: &str) -> Result<(), &'static str> {
        Ok(())
    }

    let mut store_id = use_signal(|| "".to_string());

    // let odyssey = use_context::<OdysseyProp<CookbookApplication>>().odyssey;
    let connect_handler = move |_| {
        let store_id = store_id.peek().parse().expect("TODO");
        debug!("Connecting to store: {:?}", store_id);
        let recipe_store = new_store_in_scope(root_scope, |odyssey| {
        // let recipe_store = use_store(|odyssey| {
            odyssey.connect_to_store::<Cookbook>(store_id) // , MemoryStorage::new());
        }).expect("Failed to connect_to_store");
        let cookbook_id = state.len();
        state.push(recipe_store);
        view.set(View::Cookbook(cookbook_id));
    };

    rsx! (
        div {
            TextField {
                placeholder: "Store Id",
                id: "store_id",
                value: store_id,
                oninput: move |evt: Event<FormData>| store_id.set(evt.value()),
                validation_fn: validate_store_id,
            }
            div {
                class: "text-blue-500 hover:text-blue-400 inline-flex items-center px-3",
                onclick: connect_handler,
                span {
                    "Connect to store"
                }
            }
        }
    )
}
