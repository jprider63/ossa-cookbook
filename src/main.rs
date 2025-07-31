// #![feature(impl_trait_in_bindings)]
// #![feature(unboxed_closures)]
#![feature(map_try_insert)]

use clap::Parser;
use dioxus::prelude::*;
use dioxus_desktop::muda::accelerator::Accelerator;
use dioxus_desktop::muda::{Menu, MenuId, MenuItem, PredefinedMenuItem, Submenu};
use dioxus_desktop::use_muda_event_handler;
// use dioxus_desktop::tao::menu::{AboutMetadata, MenuBar, MenuItem, MenuItemAttributes};
use futures::StreamExt;
use ossa_core::network::protocol::ecg_sync;
use ossa_core::storage::memory::MemoryStorage;
use ossa_core::store::ecg::v0::{Body, Header, HeaderId, OperationId};
use ossa_core::store::ecg::{self, ECGBody, ECGHeader};
use ossa_core::time::{CausalTime, ConcretizeTime};
use ossa_core::util::Sha256Hash;
use ossa_core::{core::OssaType, Ossa, OssaConfig};
use ossa_crdt::{map::twopmap::TwoPMap, register::LWW, time::LamportTimestamp, CRDT};
use ossa_dioxus::{use_store, DefaultSetup, OssaProp};
use serde::Serialize;
use std::borrow::BorrowMut;
use std::cell::RefCell;
use std::collections::{BTreeMap, BTreeSet};
use std::future::Future;
use std::net::{Ipv4Addr, SocketAddrV4};
use std::panic::Location;
use std::rc::Rc;
use tracing::{debug, info, trace};

use crate::gui::layout::SignalView;
use crate::state::*;

mod cli;
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

const app_name: &str = "Ossa Cookbook";
// const CSS: &str = manganis::mg!(file("./dist/style.css"));

#[derive(Clone)]
struct MenuMap {
    id_to_op: BTreeMap<MenuId, MenuOperation>,
    // op_to_menu: BTreeMap<MenuOperation, MenuItem>, // Can't do this since MenuItem is not Sync..
}

impl MenuMap {
    fn new() -> Self {
        Self {
            id_to_op: BTreeMap::new(),
        } // , op_to_menu: BTreeMap::new() }
    }

    fn insert(&mut self, menuitem: MenuItem, operation: MenuOperation) {
        self.id_to_op
            .try_insert(menuitem.id().clone(), operation)
            .expect("Menu item already exists.");
        // self.op_to_menu.try_insert(operation, menuitem);
    }

    // fn get_menu(&self, op: &MenuOperation) -> Option<&MenuItem> {
    //     self.op_to_menu.get(op)
    // }

    fn get_op(&self, menu_id: &MenuId) -> Option<&MenuOperation> {
        self.id_to_op.get(menu_id)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, PartialOrd, Ord)]
enum MenuOperation {
    NewCookbook,
    NewCookbookRecipe,
}

fn main() {
    // Turn on logging.
    tracing_subscriber::fmt::init();

    let args = cli::Arguments::parse();

    // use typeable::Typeable;
    // println!("typeid: {}", Header::<Sha256Hash, ()>::type_ident());

    let port = args.port.unwrap_or(8080);
    let ossa_config = OssaConfig {
        // address: SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), port),
        // TODO: IPV4 and/or IPV6
        port,
    };
    // // TODO: switch to this API XXX
    // let ossa: Ossa<Sha256Hash> = Ossa::new(ossa_config);
    // ossa.go_online(); // start_network(); // Starts server, connects to DHT, connect to known peers, etc
    // // ossa.go_offline(); // stop_network();
    // let ossa: Ossa<CookbookApplication> = Ossa::start(ossa_config);

    // if args.port.is_some() {
    //     // TODO: join_store()
    // } else {
    //     let init_st = initial_demo_state();
    //     let recipe_store = ossa.create_store(init_st, MemoryStorage::new()); // TODO: Owner, initial state
    //     // JP: We also want a load_store()?
    // }

    // let init_st = initial_demo_state();
    // let recipe_store = ossa.create_store(init_st, MemoryStorage::new());

    // if let Some(port) = args.port {
    //     let ossa_manager = P2PManager::initialize::<Sha256Hash,Sha256Hash>(P2PSettings {address: SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), port)});
    // } else {
    //     cli::run_client();
    // }

    let mut menu_id_map = MenuMap::new();

    let mut about_menu = Submenu::new(app_name, true);
    about_menu
        .append(&PredefinedMenuItem::about(None, None))
        .expect("TODO");
    about_menu
        .append(&PredefinedMenuItem::separator())
        .expect("TODO");
    about_menu
        .append(&PredefinedMenuItem::hide(None))
        .expect("TODO");
    about_menu
        .append(&PredefinedMenuItem::hide_others(None))
        .expect("TODO");
    about_menu
        .append(&PredefinedMenuItem::show_all(None))
        .expect("TODO");
    about_menu
        .append(&PredefinedMenuItem::separator())
        .expect("TODO");
    about_menu
        .append(&PredefinedMenuItem::quit(None))
        .expect("TODO");

    let mut file_menu = Submenu::new("File", true);
    let new_cookbook_menu_item = MenuItem::new(
        "New Cookbook",
        true,
        Some(Accelerator::new(Some(Modifiers::SUPER), Code::KeyN)),
    );
    file_menu.append(&new_cookbook_menu_item).expect("TODO");
    menu_id_map.insert(new_cookbook_menu_item, MenuOperation::NewCookbook);
    let new_recipe_menu_item = MenuItem::new("New Recipe", false, None);
    file_menu.append(&new_recipe_menu_item).expect("TODO");
    menu_id_map.insert(new_recipe_menu_item, MenuOperation::NewCookbookRecipe);
    file_menu
        .append(&PredefinedMenuItem::separator())
        .expect("TODO");
    file_menu
        .append(&PredefinedMenuItem::close_window(None))
        .unwrap();

    let edit_menu = Submenu::new("Edit", true);
    edit_menu
        .append(&PredefinedMenuItem::undo(None))
        .expect("TODO");
    edit_menu
        .append(&PredefinedMenuItem::redo(None))
        .expect("TODO");
    edit_menu
        .append(&PredefinedMenuItem::separator())
        .expect("TODO");
    edit_menu
        .append(&PredefinedMenuItem::cut(None))
        .expect("TODO");
    edit_menu
        .append(&PredefinedMenuItem::copy(None))
        .expect("TODO");
    edit_menu
        .append(&PredefinedMenuItem::paste(None))
        .expect("TODO");
    edit_menu
        .append(&PredefinedMenuItem::separator())
        .expect("TODO");
    edit_menu
        .append(&PredefinedMenuItem::select_all(None))
        .expect("TODO");

    let mut view_menu = Submenu::new("View", true);
    // // TODO: Hide tab bar items.

    let mut window_menu = Submenu::new("Window", true);
    window_menu
        .append(&PredefinedMenuItem::minimize(None))
        .unwrap();
    // window_menu.append(&PredefinedMenuItem::zoom(None));
    // // window_menu.add_native_item(MenuItem::Separator);
    // // window_menu.add_native_item(MenuItem::BringAllToFront);
    // // window_menu.add_native_item(MenuItem::Window);
    // // window_menu.add_native_item(MenuItem::CloseWindow);

    let mut help_menu = Submenu::new("Help", true);
    // help_menu.append(MenuItemAttributes::new(&format!("{} Help", app_name)));

    let mut menu = Menu::new();
    menu.append(&about_menu).expect("TODO");
    menu.append(&file_menu).expect("TODO");
    menu.append(&edit_menu).expect("TODO");
    menu.append(&view_menu).expect("TODO");
    menu.append(&window_menu).expect("TODO");
    menu.append(&help_menu).expect("TODO");

    let w = dioxus_desktop::WindowBuilder::new().with_title(app_name);
    // .with_menu(menu); // TODO XXX
    let c = dioxus_desktop::Config::new().with_window(w).with_menu(menu);
    // let ossa_prop = OssaProp::new(ossa);
    // dioxus_desktop::launch_with_props(app, ossa_prop, c);
    dioxus_desktop::launch::launch(
        app,
        vec![
            Box::new(move || {
                let ossa: Ossa<DefaultSetup> = Ossa::start(ossa_config);
                let ossa_prop = OssaProp::new(ossa);
                Box::new(ossa_prop)
            }),
            Box::new(move || Box::new(menu_id_map.clone())),
        ],
        vec![Box::new(c)],
    );
}

fn initial_demo_state() -> crate::state::internal::Cookbook<Time> {
    struct T(u8);
    impl T {
        fn new() -> Self {
            T(0)
        }

        fn t(&mut self) -> Time {
            let x = self.0;
            self.0 += 1;

            OperationId::new(None, x)
        }
    }
    let mut l = T::new();

    fn lww<A>(l: &mut T, x: A) -> LWW<Time, A> {
        LWW::new(l.t(), x)
    }

    let recipe = Recipe {
        title: lww(&mut l, "Kalbi".into()),
        ingredients: lww(&mut l, vec!["1oz Soy sauce".into(), "1lb Beef Ribs".into()]),
        instructions: lww(&mut l, "1. Grill meat\n2. Eat\n3. ...".into()),
        // image: vec![],
    };
    // TODO: We should receive this from create_store?
    let ecg_state: ecg::State<Header<Sha256Hash>, LWW<Time, ()>> = ecg::State::new();

    let recipes = TwoPMap::new();
    let recipes = recipes.apply(&ecg_state, TwoPMap::insert(l.t(), recipe.clone()));
    let recipes = recipes.apply(&ecg_state, TwoPMap::insert(l.t(), recipe.clone()));
    let recipes = recipes.apply(&ecg_state, TwoPMap::insert(l.t(), recipe.clone()));
    let recipes = recipes.apply(&ecg_state, TwoPMap::insert(l.t(), recipe.clone()));
    let recipes = recipes.apply(&ecg_state, TwoPMap::insert(l.t(), recipe.clone()));

    // let recipes = BTreeMap::from([
    //                               (0, recipe.clone()),
    //                               (1, recipe.clone()),
    //                               (2, recipe.clone()),
    //                               (3, recipe.clone()),
    //                               (4, recipe.clone()),
    //                               (5, recipe.clone()),
    //                               (6, recipe.clone()),
    // ]);
    // let book1 = Cookbook {title: lww("Family Recipes".into()), recipes: recipes.clone()};
    // let book2 = Cookbook {title: lww("My Recipes".into()), recipes: recipes};
    // vec![book1, book2]
    // TODO: Should be a Map CRDT. Include other store metadata like sharing/permissions, peers, etc
    Cookbook {
        title: lww(&mut l, "My Recipes".into()),
        recipes,
    }
}

// #[inline_props]
// fn app(cx: Scope<OssaProp<CookbookApplication>>) -> Element {
fn app() -> Element {
    // let state = use_state(&cx, || {
    //     initial_demo_state()
    // });

    // let ossa = use_context::<OssaProp<CookbookApplication>>().ossa;
    let recipe_store = use_store(|ossa| {
        let init_st: Cookbook = initial_demo_state();
        (*ossa).create_store::<Cookbook, _>(init_st, MemoryStorage::new())
    });
    let state = use_signal(|| {
        // let ossa: Ossa<CookbookApplication> = todo!();
        // let init_st = initial_demo_state();
        // let recipe_store = (*ossa).create_store(init_st, MemoryStorage::new());
        // let recipe_store = use_store(recipe_store);
        vec![recipe_store]
    });
    // let state = State {
    //     cookbooks: vec![recipe_store],
    // }; // JP: Include map from cookbook StoreIds to Cookbooks?

    let mut view = SignalView::use_view(gui::layout::View::NoSelection);

    use_muda_event_handler(move |event| {
        let menu_map = use_context::<MenuMap>();
        let menu_id = event.id();
        match menu_map.get_op(menu_id) {
            Some(MenuOperation::NewCookbook) => view.set(gui::layout::View::CookbookNew),
            Some(MenuOperation::NewCookbookRecipe) => {
                let cookbook_id = *view.peek().selected_cookbook().expect("unreachable");
                view.set(gui::layout::View::CookbookRecipeNew(cookbook_id));
            }
            None => debug!("Unhandled menu event: {menu_id:?}"),
        }
    });

    let root_scope = current_scope_id().expect("Failed to retrieve root scope");
    rsx! (
        head {
            style {{ include_str!("../dist/style.css") }}
            // link {
            //     rel: "stylesheet",
            //     href: { manganis::mg!(file("/dist/style.css")) }
            // }
        }

        gui::layout::layout { view, state, root_scope }
    )
}
