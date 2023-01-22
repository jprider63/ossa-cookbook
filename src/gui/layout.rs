use dioxus::events::MouseEvent;
use dioxus::prelude::*;
use dioxus_heroicons::{Icon, solid::Shape};
// TODO: Fix outline icons.

use crate::state::{Cookbook, CookbookId, RecipeId, Recipe};

enum View {
    Login,
    NoSelection,
    Cookbook(CookbookId),
    CookbookRecipe(CookbookId, RecipeId),
}

fn is_cookbook_selected(view: &View, cid: CookbookId) -> bool {
    match view {
        View::Login => false,
        View::NoSelection => false,
        View::Cookbook(vid) => *vid == cid,
        View::CookbookRecipe(vid, _rid) => *vid == cid,
    }
}

#[inline_props]
pub fn layout<'a>(cx: Scope, state: &'a UseState<Vec<Cookbook>>) -> Element {
    let view = use_state(&cx, || {
        View::NoSelection
    });

    let r = match *view.current() {
        View::Login => todo!(),
        View::NoSelection => rsx! ( NoSelectionView { view: view, state: state } ),
        View::Cookbook(cookbookid) => rsx! ( CookbookView { view: view, state: state, cookbook_id: cookbookid } ),
        View::CookbookRecipe(cookbookid, recipeid) => rsx! ( CookbookRecipeView { view: view, state: state, cookbook_id: cookbookid, recipe_id: recipeid } ),
    };
    cx.render(rsx!(
        div {
            class: "wrapper",
            r
        }
    ))
}

#[inline_props]
fn NoSelectionView<'a>(cx: Scope, view: &'a UseState<View>, state: &'a UseState<Vec<Cookbook>>) -> Element {
    cx.render(rsx! (
        Sidebar { view: view, state: state }
        div {
            class: "content",
            div {
                class: "flex justify-center items-center h-screen font-bold",
                "No selection" // No selection. | New cookbook | | New meal planner |
            }
        }
    ))
}

#[inline_props]
fn CookbookView<'a>(cx: Scope, view: &'a UseState<View>, state: &'a UseState<Vec<Cookbook>>, cookbook_id: CookbookId) -> Element {
    if let Some(cookbook) = state.current().get(*cookbook_id) {
        let pills = cookbook.recipes.iter().map(|(recipe_id, recipe)| { rsx! (
            RecipePill { view: view, cookbook_id: *cookbook_id, recipe_id: *recipe_id, recipe: recipe.clone() } // TODO: Can we avoid this clone?
        )});
        cx.render(rsx! (
            Sidebar { view: view, state: state }
            div {
                class: "content",
                div {
                    class: "flex justify-center",
                    h1 {
                        class: "text-3xl font-bold mt-4 mb-6 text-center",
                        "{cookbook.title}"
                    }
                }
                div {
                    class: "flex flex-row flex-wrap",
                    pills
                    div {
                        class: "basis-1/3",
                        div {
                            class: "recipe-card",
                            onclick: |_e| {println!("TODO!")},
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
        ))
    } else {
        // Cookbook not found, so set no selection.
        // TODO: Log this.
        println!("Cookbook {} not found.", cookbook_id);

        view.set(View::NoSelection);
        cx.render(rsx! (
            ""
        ))
    }
}

#[inline_props]
fn CookbookRecipeView<'a>(cx: Scope, view: &'a UseState<View>, state: &'a UseState<Vec<Cookbook>>, cookbook_id: CookbookId, recipe_id: RecipeId) -> Element {
    cx.render(rsx! (
        Sidebar { view: view, state: state }
        div {
            class: "content",
            nav {
                class: "flex w-full",
                div {
                    class: "flex-1 flex justify-start mr-auto",
                    div {
                        // class: "flex flex-row",
                        onclick: |_e| {view.set(View::Cookbook(*cookbook_id))},
                        Icon {
                            // class: "w-14 h-14",
                            icon: Shape::ChevronLeft,
                        },
                        "TODO: Cookbook"
                    }
                }
                div {
                    "TODO: Recipe"
                }
                div {
                    class: "flex-1 flex justify-end ml-auto",
                    "TODO: Edit"
                }
            }
            div {
            "TODO: Image carousel"
            }
            div {
            "TODO: Ingredients"
            }
            div {
            "TODO: Instructions"
            }
        }
    ))
}

#[inline_props]
fn RecipePill<'a>(cx: Scope, view: &'a UseState<View>, cookbook_id: CookbookId, recipe_id: RecipeId, recipe: Recipe) -> Element {
    cx.render(rsx! (
        div {
            class: "basis-1/3",
            div {
                class: "recipe-card",
                onclick: |_e| {view.set(View::CookbookRecipe(*cookbook_id, *recipe_id))},
                img {
                    src: "https://food.fnr.sndimg.com/content/dam/images/food/fullset/2008/8/14/0/GT0107_kalbi_s4x3.jpg.rend.hgtvcom.1280.720.suffix/1519669666497.jpeg"
                }
                div {
                    class: "border-t",
                    p {
                        class: "p-5 text-center",
                        "{recipe.title}"
                    }
                }
            }
        }
    ))
}

#[inline_props]
fn Sidebar<'a>(cx: Scope, view: &'a UseState<View>, state: &'a UseState<Vec<Cookbook>>) -> Element {
    let cookbooks = state.iter().enumerate().map(|(i,cookbook)| rsx!(
        SidebarItem { title: &cookbook.title, icon: Shape::BookOpen, selected: is_cookbook_selected(&view, i), onclick: move |_e| {view.set(View::Cookbook(i))} }
    ));

    cx.render(rsx! (
        div {
            class: "sidebar",
            div {
                class: "overflow-y-auto overflow-x-hidden flex-grow",
                ul {
                    class: "flex flex-col py-4 space-y-1",
                    SidebarHeader { title: "COOKBOOKS" }
                    cookbooks
                    SidebarHeader { title: "MEAL PLANNER" }
                    SidebarItem   { title: "Weekly meals", icon: Shape::PencilSquare, onclick: |_e| {println!("TODO!")}, selected: false }
                    SidebarItem   { title: "Thanksgiving", icon: Shape::PencilSquare, selected: false, onclick: |_e| {println!("TODO!")} }
                    SidebarHeader { title: "SETTINGS" }
                    SidebarItem   { title: "Account", icon: Shape::User, selected: false, onclick: |_e| {println!("TODO!")} }
                    SidebarItem   { title: "Logout", icon: Shape::ArrowRightOnRectangle, selected: false, onclick: |_e| {println!("TODO!")} }
                }
            }
        }
    ))
}

#[derive(Props)]
pub struct SidebarHeaderProps<'a> {
    title: &'a str
}

pub fn SidebarHeader<'a>(cx: Scope<'a, SidebarHeaderProps<'a>>) -> Element {
    cx.render(rsx! (
        li {
            class: "px-5",
            div {
                class: "flex flex-row items-center h-8",
                div {
                    class: "text-sm font-light tracking-wide text-gray-500",
                    "{cx.props.title}"
                }
            }
        }
    ))
}

#[derive(Props)]
pub struct SidebarItemProps<'a> {
    title: &'a str,
    icon: Shape,
    onclick: EventHandler<'a, MouseEvent>,
    selected: bool,
}

pub fn SidebarItem<'a>(cx: Scope<'a, SidebarItemProps<'a>>) -> Element {
    let is_selected = if cx.props.selected {"selected"} else {""};
    cx.render(rsx! (
        li {
            div {
                class: format_args!("sidebar-item {}", is_selected),
                onclick: |e| cx.props.onclick.call(e),
                span {
                    class: "inline-flex justify-center items-center ml-4",
                    Icon {
                        class: "w-6 h-6",
                        icon: cx.props.icon,
                    }
                }
                span {
                    class: "ml-2 text-sm tracking-wide truncate",
                    "{cx.props.title}"
                }
            }
        }
    ))
}
