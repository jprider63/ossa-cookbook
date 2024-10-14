use clap::Parser;
use dioxus::prelude::*;
// use dioxus_desktop::tao::menu::{AboutMetadata, MenuBar, MenuItem, MenuItemAttributes};
use futures::StreamExt;
use odyssey_core::{Odyssey, OdysseyConfig, core::OdysseyType};
use odyssey_core::network::p2p::{P2PManager, P2PSettings};
use odyssey_core::store::ecg::{self, ECGBody, ECGHeader};
use odyssey_core::storage::memory::MemoryStorage;
use odyssey_core::store::ecg::v0::{OperationId, Header, HeaderId};
use odyssey_core::util::Sha256Hash;
use odyssey_crdt::{
    map::twopmap::TwoPMap,
    register::LWW,
    time::LamportTimestamp,
    CRDT,
};
use serde::Serialize;
use std::borrow::BorrowMut;
use std::cell::RefCell;
use std::collections::BTreeSet;
use std::net::{Ipv4Addr, SocketAddrV4};
use std::rc::Rc;

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

const app_name: &str = "Odyssey Cookbook";
// const CSS: &str = manganis::mg!(file("./dist/style.css"));

fn main() {
    let args = cli::Arguments::parse();

    // use typeable::Typeable;
    // println!("typeid: {}", Header::<Sha256Hash, ()>::type_ident());

    let port = args.port.unwrap_or(8080);
    let odyssey_config = OdysseyConfig {
        // address: SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), port),
        // TODO: IPV4 and/or IPV6
        port,
    };
    // // TODO: switch to this API XXX
    // let odyssey: Odyssey<Sha256Hash> = Odyssey::new(odyssey_config);
    // odyssey.go_online(); // start_network(); // Starts server, connects to DHT, connect to known peers, etc
    // // odyssey.go_offline(); // stop_network();
    // let odyssey: Odyssey<CookbookApplication> = Odyssey::start(odyssey_config);

    // if args.port.is_some() {
    //     // TODO: join_store()
    // } else {
    //     let init_st = initial_demo_state();
    //     let recipe_store = odyssey.create_store(init_st, MemoryStorage::new()); // TODO: Owner, initial state
    //     // JP: We also want a load_store()?
    // }

    // let init_st = initial_demo_state();
    // let recipe_store = odyssey.create_store(init_st, MemoryStorage::new());


    // if let Some(port) = args.port {
    //     let odyssey_manager = P2PManager::initialize::<Sha256Hash,Sha256Hash>(P2PSettings {address: SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), port)});
    // } else {
    //     cli::run_client();
    // }

    // let mut about_menu = MenuBar::new();
    // about_menu.add_native_item(MenuItem::About(app_name.into(), AboutMetadata::default()));
    // about_menu.add_native_item(MenuItem::Separator);
    // about_menu.add_native_item(MenuItem::Hide);
    // about_menu.add_native_item(MenuItem::HideOthers);
    // about_menu.add_native_item(MenuItem::ShowAll);
    // about_menu.add_native_item(MenuItem::Separator);
    // about_menu.add_native_item(MenuItem::Quit);

    // let mut file_menu = MenuBar::new();
    // file_menu.add_native_item(MenuItem::CloseWindow);

    // let mut edit_menu = MenuBar::new();
    // edit_menu.add_native_item(MenuItem::Undo);
    // edit_menu.add_native_item(MenuItem::Redo);
    // edit_menu.add_native_item(MenuItem::Separator);
    // edit_menu.add_native_item(MenuItem::Cut);
    // edit_menu.add_native_item(MenuItem::Copy);
    // edit_menu.add_native_item(MenuItem::Paste);
    // edit_menu.add_native_item(MenuItem::Separator);
    // edit_menu.add_native_item(MenuItem::SelectAll);

    // let view_menu = MenuBar::new();
    // // TODO: Hide tab bar items.

    // let mut window_menu = MenuBar::new();
    // window_menu.add_native_item(MenuItem::Minimize);
    // window_menu.add_native_item(MenuItem::Zoom);
    // // window_menu.add_native_item(MenuItem::Separator);
    // // window_menu.add_native_item(MenuItem::BringAllToFront);
    // // window_menu.add_native_item(MenuItem::Window);
    // // window_menu.add_native_item(MenuItem::CloseWindow);

    // let mut help_menu = MenuBar::new();
    // help_menu.add_item(MenuItemAttributes::new(&format!("{} Help", app_name)));

    // let mut menu = MenuBar::new();
    // menu.add_submenu( app_name, true, about_menu);
    // menu.add_submenu( "File", true, file_menu);
    // menu.add_submenu( "Edit", true, edit_menu);
    // menu.add_submenu( "View", true, view_menu);
    // menu.add_submenu( "Window", true, window_menu);
    // menu.add_submenu( "Help", true, help_menu);

    let w = dioxus_desktop::WindowBuilder::new().with_title(app_name);
                                                // .with_menu(menu); // TODO XXX
    let c = dioxus_desktop::Config::new().with_window(w);
    // let odyssey_prop = OdysseyProp::new(odyssey);
    // dioxus_desktop::launch_with_props(app, odyssey_prop, c);
    dioxus_desktop::launch::launch(app, vec![Box::new(move || {
        let odyssey: Odyssey<CookbookApplication> = Odyssey::start(odyssey_config);
        let odyssey_prop = OdysseyProp::new(odyssey);
        Box::new(odyssey_prop)
    })], c);
}

fn initial_demo_state() -> Cookbook {
    struct T (u8);
    impl T {
        fn new() -> Self {
            T(0)
        }

        fn t(&mut self) -> Time {
            let x = self.0;
            self.0 += 1;

            OperationId {
                header_id: None,
                operation_position: x,
            }
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
    let recipes = TwoPMap::new();
    let recipes = recipes.apply(l.t(), TwoPMap::insert(recipe.clone()));
    let recipes = recipes.apply(l.t(), TwoPMap::insert(recipe.clone()));
    let recipes = recipes.apply(l.t(), TwoPMap::insert(recipe.clone()));
    let recipes = recipes.apply(l.t(), TwoPMap::insert(recipe.clone()));
    let recipes = recipes.apply(l.t(), TwoPMap::insert(recipe.clone()));
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
    Cookbook {title: lww(&mut l, "My Recipes".into()), recipes: recipes}
}

// #[inline_props]
// fn app(cx: Scope<OdysseyProp<CookbookApplication>>) -> Element {
fn app() -> Element {
    // let state = use_state(&cx, || {
    //     initial_demo_state()
    // });

    // let odyssey = use_context::<OdysseyProp<CookbookApplication>>().odyssey;
    let recipe_store = use_store(|odyssey| {
        let init_st = initial_demo_state();
        (*odyssey).create_store(init_st, MemoryStorage::new())
    });
    let state = use_signal(|| {
        // let odyssey: Odyssey<CookbookApplication> = todo!();
        // let init_st = initial_demo_state();
        // let recipe_store = (*odyssey).create_store(init_st, MemoryStorage::new());
        // let recipe_store = use_store(recipe_store);
        vec![recipe_store]
    });
    // let state = State {
    //     cookbooks: vec![recipe_store],
    // }; // JP: Include map from cookbook StoreIds to Cookbooks?

    rsx! (
        head {
            style {{ include_str!("../dist/style.css") }}
            // link {
            //     rel: "stylesheet",
            //     href: { manganis::mg!(file("/dist/style.css")) }
            // }
        }

        gui::layout::layout { state: state }
    )
}

// #[derive(Props, PartialEq)]
struct OdysseyProp<A: 'static> {
    odyssey: Rc<Odyssey<A>>,
}

impl<A: 'static> Clone for OdysseyProp<A> {
    fn clone(&self) -> Self {
        OdysseyProp {
            odyssey: self.odyssey.clone(),
        }
    }
}

impl<A> OdysseyProp<A> {
    fn new(odyssey: Odyssey<A>) -> Self {
        OdysseyProp {
            odyssey: Rc::new(odyssey),
        }
    }
}

enum CookbookApplication {}

impl OdysseyType for CookbookApplication {
    type StoreId = (); // TODO
    // type ECGHeader<T: CRDT<Time = OperationId<HeaderId<Sha256Hash>>>> = Header<Sha256Hash, T>; // TODO
    type ECGHeader<T: CRDT<Time = Self::Time, Op: Serialize>> = Header<Sha256Hash, T>;

    type Time = OperationId<HeaderId<Sha256Hash>>;
    // type ECGHeader<T> = Header<Sha256Hash, T>
    //     where T: CRDT<Time = OperationId<HeaderId<Sha256Hash>>>;
    // type ECGHeader<T> = Header<Sha256Hash, T>;
}


// TODO: Create `odyssey-dioxus` crate?
use odyssey_core::core::{StateUpdate, StoreHandle};
struct UseStore<OT: OdysseyType + 'static, T: CRDT<Time = OT::Time, Op: Serialize> + 'static> {
    handle: Rc<RefCell<StoreHandle<OT, T>>>,
    state: Signal<Option<StoreState<OT, T>>>,
    // peers, connections, etc
}

impl<OT: OdysseyType + 'static, T: CRDT<Time = OT::Time, Op: Serialize>> Clone for UseStore<OT, T> {
    fn clone(&self) -> Self {
        UseStore {
            handle: self.handle.clone(),
            state: self.state.clone(),
        }
    }
}

// #[derive(Clone)]
struct StoreState<OT: OdysseyType, T: CRDT<Time = OT::Time, Op: Serialize>> {
    state: T,
    ecg: ecg::State<OT::ECGHeader<T>, T>,
}

impl<OT: OdysseyType, T: CRDT<Time = OT::Time, Op: Serialize> + Clone> Clone for StoreState<OT, T>
where
    <OT as OdysseyType>::ECGHeader<T>: Clone,
{
    fn clone(&self) -> Self {
        StoreState {
            state: self.state.clone(),
            ecg: self.ecg.clone(),
        }
    }
}

// fn use_store<OT: OdysseyType, T>(handle: StoreHandle<OT, T>) -> UseStore<OT, T> {
fn use_store<OT: OdysseyType + 'static, T: CRDT<Time = OT::Time, Op: Serialize>, F>(build_store_handle: F) -> UseStore<OT, T>
where
    F: FnOnce(&Odyssey<CookbookApplication>) -> StoreHandle<OT, T>,
{
    // JP: How do we put this inside the `use_hook`?
    let odyssey = use_context::<OdysseyProp<CookbookApplication>>().odyssey;
    let (handle, mut recv_state) = use_hook(|| {
        let mut h = build_store_handle(&odyssey);
        let recv_st = h.subscribe_to_state();

        // // Get current state.
        // let st = recv_st.blocking_recv().unwrap();


        let h = Rc::new(RefCell::new(h)); // JP: Annoyingly required since dioxus requires clone... XXX
        // let st = Rc::new(st); // JP: Annoyingly required since dioxus requires clone... XXX
        // let recv_st = Rc::new(recv_st); // JP: Annoyingly required since dioxus requires clone... XXX
        let recv_st = CopyValue::new(recv_st); // JP: Annoyingly required since dioxus requires clone... XXX
        (h, recv_st)
    });
    let mut state = use_signal(|| {
        None
    });
    // let mut state: Signal<T> = use_signal(|| {
    //     // let recv_state = Rc::get_mut(&mut recv_state).unwrap();
    //     let init_st = recv_state.write().blocking_recv().unwrap();
    //     init_st
    //     // Rc::into_inner(init_st).unwrap()
    // });
    // let handle2 = handle.clone();
    // TODO...
    use_future(move || async move {
    //     let mut recv_state = handle2.subscribe_to_state();
    //     // let mut recv_state = Rc::try_unwrap(recv_state).unwrap();
    //     // let mut recv_state = recv_state.clone();
    //     // let recv_state = Rc::get_mut(&mut recv_state).unwrap();
        while let Some(msg) = recv_state.write().recv().await {
            match msg {
                StateUpdate::Snapshot { snapshot, ecg_state } => {
                    println!("Received state!");
                    let s = StoreState {
                        state: snapshot,
                        ecg: ecg_state,
                    };
                    state.set(Some(s));
                }
            }
        }
    });
    UseStore {
        handle,
        state,
    }
}

impl<OT: OdysseyType, T: CRDT<Time = OT::Time>> UseStore<OT, T>
where
    T::Op: Serialize,
{
    // TODO: Apply operations, get current state, etc
    pub fn get_current_state(&self) -> Option<T>
    where
        T: Clone,
        <OT as OdysseyType>::ECGHeader<T>: Clone,
    {
        self.state.cloned().map(|s| s.state)
    }

    pub fn get_current_store_state(&self) -> Option<StoreState<OT, T>>
    where
        T: Clone,
        <OT as OdysseyType>::ECGHeader<T>: Clone,
    {
        self.state.cloned()
    }

    pub fn apply(&mut self, parents: BTreeSet<<<OT as OdysseyType>::ECGHeader<T> as ECGHeader<T>>::HeaderId>, op: T::Op) -> T::Time
    where <<OT as OdysseyType>::ECGHeader<T> as ECGHeader<T>>::Body: ECGBody<T>,
    {
        (*self.handle).borrow_mut().apply(parents, op)
    }

    pub fn apply_batch(&mut self, parents: BTreeSet<<<OT as OdysseyType>::ECGHeader<T> as ECGHeader<T>>::HeaderId>, op: Vec<T::Op>) -> Vec<T::Time>
    where <<OT as OdysseyType>::ECGHeader<T> as ECGHeader<T>>::Body: ECGBody<T>,
    {
        (*self.handle).borrow_mut().apply_batch(parents, op)
    }
}
