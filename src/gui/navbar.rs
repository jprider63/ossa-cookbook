use dioxus::events::MouseEvent;
use dioxus::prelude::*;
use dioxus_heroicons::{Icon, solid::Shape};

// #[derive(Props)]
// pub struct NavbarButtonProps<'a> {
//     pub text: &'a str,
//     pub icon: Option<Shape>,
//     pub onclick: Box<dyn 'a + FnMut(MouseEvent)>,
// }
// 
// pub fn NavbarButton<'a>(cx: Scope<'a, NavbarButtonProps<'a>>) -> Element {
//     None
// }

pub struct Button<'a> {
    pub text: &'a str,
    pub icon: Option<Shape>,
    pub onclick: Box<dyn 'a + Fn(MouseEvent)>, // FnMut(MouseEvent)>,
}

// #[derive(Props)]
// pub struct NavbarProps<'a> {
//     title: &'a str,
//     left_button: Option<Button<'a>>,
//     right_button: Option<Button<'a>>,
// }

// pub fn Navbar<'a>(cx: Scope<'a, NavbarProps<'a>>) -> Element {
// pub fn Navbar<'a>(cx: Scope<'a>, title: &'a str, left_button: Option<Button<'a>>, right_button:Option<Button<'a>>) -> Element<'a> {
//     None
// }
// 
#[inline_props]
pub fn Navbar<'a>(cx: Scope<'a>, title: &'a str, left_button: Option<Button<'a>>, right_button: Option<Button<'a>>) -> Element {
// // pub fn view<'a>(title: &'a str, left_button: Option<&'a Button<'a>>, right_button:Option<&'a Button<'a>>) -> LazyNodes<'a,'a> {
// pub fn view2<'a>(title: &'a str, left_button: Option<Button<'a>>, right_button:Option<Button<'a>>) -> LazyNodes<'a,'a> {
    fn btn<'a>(button: &'a Option<Button<'a>>) -> Option<LazyNodes<'a, 'a>> {
        button.as_ref().map(|button| {
            rsx! (
                div {
                    class: "text-blue-500 hover:text-blue-400 inline-flex items-center px-3",
                    onclick: move |e| {(button.onclick)(e)},
                    button.icon.map(|i| rsx!(
                        Icon {
                            class: "w-6 h-6",
                            icon: i,
                        }
                    )),
                    span {
                        "{button.text}"
                    }
                }
            )
        })
    }
    let left = btn(left_button);
    let right = btn(right_button);

    // let left =
    //         left_button.as_ref().map(|left_button| {
    //             rsx! (
    //             div {
    //                 class: "text-blue-500 hover:text-blue-400 inline-flex items-center px-3",
    //                 onclick: move |e| {(left_button.onclick)(e)},
    //                 left_button.icon.map(|i| rsx!(
    //                     Icon {
    //                         class: "w-6 h-6",
    //                         icon: i,
    //                     }
    //                 )),
    //                 span {
    //                     "{left_button.text}"
    //                 }
    //             }
    //         )});
    // let right = rsx! ( div {} );


    cx.render(rsx! (
        nav {
            class: "flex w-full mt-4 mb-6",
            div {
                class: "flex-1 flex justify-start mr-auto whitespace-nowrap",
                left
            }
            div {
                class: "whitespace-nowrap",
                h1 {
                    class: "text-3xl font-bold text-center",
                        "{title}"
                }
            }
            div {
                class: "flex-1 flex justify-end ml-auto whitespace-nowrap",
                right
            }
        }
    ))
}
