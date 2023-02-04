use dioxus::prelude::*;
use dioxus_desktop::tao::menu::{MenuBar, MenuItem, MenuItemAttributes};
use std::collections::BTreeMap;

use crate::state::*;

mod gui;
mod state;

/*
use cocoa::appkit::{NSWindow, NSWindowStyleMask};
// use tauri::{Runtime, Window};
use dioxus_desktop::tao::window::Window;

pub trait WindowExt {
  #[cfg(target_os = "macos")]
  fn set_transparent_titlebar(&self, transparent: bool);
}

// impl<R: Runtime> WindowExt for Window<R> {
impl WindowExt for Window {
  #[cfg(target_os = "macos")]
  fn set_transparent_titlebar(&self, transparent: bool) {
    use cocoa::appkit::NSWindowTitleVisibility;

    unsafe {
      let id = self.ns_window().unwrap() as cocoa::base::id;

      let mut style_mask = id.styleMask();
      style_mask.set(
        NSWindowStyleMask::NSFullSizeContentViewWindowMask,
        transparent,
      );
      id.setStyleMask_(style_mask);

      id.setTitleVisibility_(if transparent {
        NSWindowTitleVisibility::NSWindowTitleHidden
      } else {
        NSWindowTitleVisibility::NSWindowTitleVisible
      });
      id.setTitlebarAppearsTransparent_(if transparent {
        cocoa::base::YES
      } else {
        cocoa::base::NO
      });
    }
  }
}
*/

const app_name: &str = "Odyssey Cookbook";

fn main() {
    dioxus::desktop::launch_cfg(app, |c|
        c.with_window(|w| {
            let mut about_menu = MenuBar::new();
            about_menu.add_native_item(MenuItem::About(app_name.into()));
            about_menu.add_native_item(MenuItem::Separator);
            about_menu.add_native_item(MenuItem::Hide);
            about_menu.add_native_item(MenuItem::HideOthers);
            about_menu.add_native_item(MenuItem::ShowAll);
            about_menu.add_native_item(MenuItem::Separator);
            about_menu.add_native_item(MenuItem::Quit);

            let mut file_menu = MenuBar::new();
            file_menu.add_native_item(MenuItem::CloseWindow);

            let mut edit_menu = MenuBar::new();
            edit_menu.add_native_item(MenuItem::Undo);
            edit_menu.add_native_item(MenuItem::Redo);
            edit_menu.add_native_item(MenuItem::Separator);
            edit_menu.add_native_item(MenuItem::Cut);
            edit_menu.add_native_item(MenuItem::Copy);
            edit_menu.add_native_item(MenuItem::Paste);
            edit_menu.add_native_item(MenuItem::Separator);
            edit_menu.add_native_item(MenuItem::SelectAll);

            let mut view_menu = MenuBar::new();
            // TODO: Hide tab bar items.

            let mut window_menu = MenuBar::new();
            window_menu.add_native_item(MenuItem::Minimize);
            window_menu.add_native_item(MenuItem::Zoom);
            // window_menu.add_native_item(MenuItem::Separator);
            // window_menu.add_native_item(MenuItem::BringAllToFront);
            // window_menu.add_native_item(MenuItem::Window);
            // window_menu.add_native_item(MenuItem::CloseWindow);

            let mut help_menu = MenuBar::new();
            help_menu.add_item(MenuItemAttributes::new(&format!("{} Help", app_name)));

            let mut menu = MenuBar::new();
            menu.add_submenu( app_name, true, about_menu);
            menu.add_submenu( "File", true, file_menu);
            menu.add_submenu( "Edit", true, edit_menu);
            menu.add_submenu( "View", true, view_menu);
            menu.add_submenu( "Window", true, window_menu);
            menu.add_submenu( "Help", true, help_menu);

            w.with_title(app_name)
             .with_menu(menu)
        })
    );
}


fn app(cx: Scope) -> Element {
    let state = use_state(&cx, || {
        let recipe = Recipe {
            title: "Kalbi".into(),
            ingredients: vec!["1oz Soy sauce".into(), "1lb Beef Ribs".into()],
            instructions: "TODO".into(),
            image: vec![],
        };
        let recipes = BTreeMap::from([
                                      (0, recipe.clone()),
                                      (1, recipe.clone()),
                                      (2, recipe.clone()),
                                      (3, recipe.clone()),
                                      (4, recipe.clone()),
                                      (5, recipe.clone()),
                                      (6, recipe.clone()),
        ]);
        let book1 = Cookbook {title: "Family Recipes".into(), recipes: recipes.clone()};
        let book2 = Cookbook {title: "My Recipes".into(), recipes: recipes};
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

