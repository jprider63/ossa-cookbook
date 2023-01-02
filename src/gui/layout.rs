use dioxus::prelude::*;
use dioxus_heroicons::{Icon, solid::Shape};
// TODO: Fix outline icons.

// use crate::gui::layout::sidebar_header;

pub fn layout(cx: Scope) -> Element {
    cx.render(rsx! (
        div {
            class: "min-h-screen flex flex-col flex-auto flex-shrink-0 antialiased bg-gray-50 text-gray-800",
            div {
                class: "fixed flex flex-col top-0 left-0 w-64 bg-white h-full border-r",
                div {
                    class: "overflow-y-auto overflow-x-hidden flex-grow",
                    ul {
                        class: "flex flex-col py-4 space-y-1",
                        SidebarHeader { title: "COOKBOOKS" }
                        SidebarItem   { title: "Family Recipes", icon: Shape::BookOpen }
                        SidebarItem   { title: "My Recipes", icon: Shape::BookOpen }
                        SidebarHeader { title: "MEAL PLANNER" }
                        SidebarItem   { title: "Weekly meals", icon: Shape::PencilSquare }
                        SidebarItem   { title: "Thanksgiving", icon: Shape::PencilSquare }
                        SidebarHeader { title: "SETTINGS" }
                        SidebarItem   { title: "Account", icon: Shape::User }
                        SidebarItem   { title: "Logout", icon: Shape::ArrowRightOnRectangle }
                    }
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
}

pub fn SidebarItem<'a>(cx: Scope<'a, SidebarItemProps<'a>>) -> Element {
    cx.render(rsx! (
        li {
            a {
                class: "relative flex flex-row items-center h-11 focus:outline-none hover:bg-gray-50 text-gray-600 hover:text-gray-800 border-l-4 border-transparent hover:border-indigo-500 pr-6",
                href: "#",
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
