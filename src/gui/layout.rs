use dioxus::events::MouseEvent;
use dioxus::prelude::*;
use dioxus_heroicons::{Icon, solid::Shape};
// TODO: Fix outline icons.

use crate::state::{Cookbook, CookbookId};

enum View {
    Login,
    NoSelection,
    Cookbook(CookbookId),
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
    };
    cx.render(rsx!(
        div {
            class: "min-h-screen flex flex-col flex-auto flex-shrink-0 antialiased bg-gray-50 text-gray-800",
            r
        }
    ))
}

#[inline_props]
fn NoSelectionView<'a>(cx: Scope, view: &'a UseState<View>, state: &'a UseState<Vec<Cookbook>>) -> Element {
    cx.render(rsx! (
        Sidebar { view: view, state: state }
    ))
}

#[inline_props]
fn CookbookView<'a>(cx: Scope, view: &'a UseState<View>, state: &'a UseState<Vec<Cookbook>>, cookbook_id: CookbookId) -> Element {
    if let Some(cookbook) = state.current().get(*cookbook_id) {
        println!("rendering {}..", cookbook.title);
        cx.render(rsx! (
            Sidebar { view: view, state: state }
            div {
                class: "min-h-screen flex flex-col flex-auto flex-shrink-0 antialiased",
                "{cookbook.title}"
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
fn Sidebar<'a>(cx: Scope, view: &'a UseState<View>, state: &'a UseState<Vec<Cookbook>>) -> Element {
    let cookbooks = state.iter().enumerate().map(|(i,cookbook)| rsx!(
        SidebarItem { title: &cookbook.title, icon: Shape::BookOpen, onclick: move |_e| {view.set(View::Cookbook(i))} }
    ));

    cx.render(rsx! (
        div {
            class: "fixed flex flex-col top-0 left-0 w-64 bg-white h-full border-r",
            div {
                class: "overflow-y-auto overflow-x-hidden flex-grow",
                ul {
                    class: "flex flex-col py-4 space-y-1",
                    SidebarHeader { title: "COOKBOOKS" }
                    cookbooks
                    SidebarHeader { title: "MEAL PLANNER" }
                    SidebarItem   { title: "Weekly meals", icon: Shape::PencilSquare, onclick: |_e| {println!("TODO!")} }
                    SidebarItem   { title: "Thanksgiving", icon: Shape::PencilSquare, onclick: |_e| {println!("TODO!")} }
                    SidebarHeader { title: "SETTINGS" }
                    SidebarItem   { title: "Account", icon: Shape::User, onclick: |_e| {println!("TODO!")} }
                    SidebarItem   { title: "Logout", icon: Shape::ArrowRightOnRectangle, onclick: |_e| {println!("TODO!")} }
                }
            }
        }
        div {
            // class: "flex flex-col",
            "TEST!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!"
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
}

pub fn SidebarItem<'a>(cx: Scope<'a, SidebarItemProps<'a>>) -> Element {
    cx.render(rsx! (
        li {
            a {
                class: "relative flex flex-row items-center h-11 focus:outline-none hover:bg-gray-50 text-gray-600 hover:text-gray-800 border-l-4 border-transparent hover:border-indigo-500 pr-6",
                href: "#",
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
