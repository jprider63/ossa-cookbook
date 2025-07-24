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
use odyssey_core::network::protocol::ecg_sync;
use odyssey_core::storage::memory::MemoryStorage;
use odyssey_core::store::ecg::v0::{Body, Header, HeaderId, OperationId};
use odyssey_core::store::ecg::{self, ECGBody, ECGHeader};
use odyssey_core::time::{CausalTime, ConcretizeTime};
use odyssey_core::util::Sha256Hash;
use odyssey_core::{core::OdysseyType, Odyssey, OdysseyConfig};
use odyssey_crdt::{map::twopmap::TwoPMap, register::LWW, time::LamportTimestamp, CRDT};
use serde::Serialize;
use tracing::{debug, info, trace};
use std::borrow::BorrowMut;
use std::cell::RefCell;
use std::collections::{BTreeMap, BTreeSet};
use std::future::Future;
use std::net::{Ipv4Addr, SocketAddrV4};
use std::panic::Location;
use std::rc::Rc;

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

const app_name: &str = "Odyssey Cookbook";
// const CSS: &str = manganis::mg!(file("./dist/style.css"));

#[derive(Clone)]
struct MenuMap {
    id_to_op: BTreeMap<MenuId, MenuOperation>,
    // op_to_menu: BTreeMap<MenuOperation, MenuItem>, // Can't do this since MenuItem is not Sync..
}

impl MenuMap {
    fn new() -> Self {
        Self { id_to_op: BTreeMap::new()} // , op_to_menu: BTreeMap::new() }
    }

    fn insert(&mut self, menuitem: MenuItem, operation: MenuOperation) {
        self.id_to_op.try_insert(menuitem.id().clone(), operation).expect("Menu item already exists.");
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

    let mut menu_id_map = MenuMap::new();

    let mut about_menu = Submenu::new(app_name, true);
    about_menu.append(&PredefinedMenuItem::about(None, None)).expect("TODO");
    about_menu.append(&PredefinedMenuItem::separator()).expect("TODO");
    about_menu.append(&PredefinedMenuItem::hide(None)).expect("TODO");
    about_menu.append(&PredefinedMenuItem::hide_others(None)).expect("TODO");
    about_menu.append(&PredefinedMenuItem::show_all(None)).expect("TODO");
    about_menu.append(&PredefinedMenuItem::separator()).expect("TODO");
    about_menu.append(&PredefinedMenuItem::quit(None)).expect("TODO");

    let mut file_menu = Submenu::new("File", true);
    let new_cookbook_menu_item = MenuItem::new("New Cookbook", true, Some(Accelerator::new(Some(Modifiers::SUPER), Code::KeyN)));
    file_menu.append(&new_cookbook_menu_item).expect("TODO");
    menu_id_map.insert(new_cookbook_menu_item, MenuOperation::NewCookbook);
    let new_recipe_menu_item = MenuItem::new("New Recipe", false, None);
    file_menu.append(&new_recipe_menu_item).expect("TODO");
    menu_id_map.insert(new_recipe_menu_item, MenuOperation::NewCookbookRecipe);
    file_menu.append(&PredefinedMenuItem::separator()).expect("TODO");
    file_menu.append(&PredefinedMenuItem::close_window(None)).unwrap();

    let edit_menu = Submenu::new("Edit", true);
    edit_menu.append(&PredefinedMenuItem::undo(None)).expect("TODO");
    edit_menu.append(&PredefinedMenuItem::redo(None)).expect("TODO");
    edit_menu.append(&PredefinedMenuItem::separator()).expect("TODO");
    edit_menu.append(&PredefinedMenuItem::cut(None)).expect("TODO");
    edit_menu.append(&PredefinedMenuItem::copy(None)).expect("TODO");
    edit_menu.append(&PredefinedMenuItem::paste(None)).expect("TODO");
    edit_menu.append(&PredefinedMenuItem::separator()).expect("TODO");
    edit_menu.append(&PredefinedMenuItem::select_all(None)).expect("TODO");

    let mut view_menu = Submenu::new("View", true);
    // // TODO: Hide tab bar items.

    let mut window_menu = Submenu::new("Window", true);
    window_menu.append(&PredefinedMenuItem::minimize(None)).unwrap();
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
    // let odyssey_prop = OdysseyProp::new(odyssey);
    // dioxus_desktop::launch_with_props(app, odyssey_prop, c);
    dioxus_desktop::launch::launch(
        app,
        vec![
            Box::new(move || {
                let odyssey: Odyssey<CookbookApplication> = Odyssey::start(odyssey_config);
                let odyssey_prop = OdysseyProp::new(odyssey);
                Box::new(odyssey_prop)
            }),
            Box::new(move || {
                Box::new(menu_id_map.clone())
            }),
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
// fn app(cx: Scope<OdysseyProp<CookbookApplication>>) -> Element {
fn app() -> Element {
    // let state = use_state(&cx, || {
    //     initial_demo_state()
    // });

    // let odyssey = use_context::<OdysseyProp<CookbookApplication>>().odyssey;
    let recipe_store = use_store(|odyssey| {
        let init_st: Cookbook = initial_demo_state();
        (*odyssey).create_store::<Cookbook, _>(init_st, MemoryStorage::new())
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

// #[derive(Props, PartialEq)]
struct OdysseyProp<A: OdysseyType + 'static> {
    odyssey: Rc<Odyssey<A>>,
}

impl<A: OdysseyType + 'static> Clone for OdysseyProp<A> {
    fn clone(&self) -> Self {
        OdysseyProp {
            odyssey: self.odyssey.clone(),
        }
    }
}

impl<A: OdysseyType> OdysseyProp<A> {
    fn new(odyssey: Odyssey<A>) -> Self {
        OdysseyProp {
            odyssey: Rc::new(odyssey),
        }
    }
}

enum CookbookApplication {}

impl OdysseyType for CookbookApplication {
    type Hash = Sha256Hash;
    type StoreId = Sha256Hash; // TODO
                       // type ECGHeader<T: CRDT<Op: Serialize, Time = OperationId<HeaderId<Sha256Hash>>>> = Header<Sha256Hash, T>;
                       // type ECGHeader<T: CRDT<Time = OperationId<HeaderId<Sha256Hash>>>> = Header<Sha256Hash, T>;
    type ECGHeader = Header<Sha256Hash>;
    // type ECGHeader<T: CRDT<Op: Serialize>> = Header<Sha256Hash, T>;
    type ECGBody<T: CRDT<Op: ConcretizeTime<HeaderId<Sha256Hash>>>> = Body<Sha256Hash, <T::Op as ConcretizeTime<HeaderId<Sha256Hash>>>::Serialized>; // : CRDT<Time = Self::Time, Op: Serialize>> = Body<Sha256Hash, T>;

    type Time = OperationId<HeaderId<Sha256Hash>>;

    type CausalState<T: CRDT<Time = Self::Time>> = ecg::State<Self::ECGHeader, T>;
    // type ECGHeader<T> = Header<Sha256Hash, T>
    //     where T: CRDT<Time = OperationId<HeaderId<Sha256Hash>>>;
    // type ECGHeader<T> = Header<Sha256Hash, T>;

    fn to_causal_state<'a, T: CRDT<Time = Self::Time>>(
        st: &'a ecg::State<Self::ECGHeader, T>,
    ) -> &'a Self::CausalState<T> {
        st
    }
}

// TODO: Create `odyssey-dioxus` crate?
use odyssey_core::core::{StoreHandle};
use odyssey_core::store::StateUpdate;
pub struct UseStore<OT: OdysseyType + 'static, T: CRDT<Time = OT::Time, Op: ConcretizeTime<<OT::ECGHeader as ECGHeader>::HeaderId>> + 'static> {
    future: Task,
    handle: Rc<RefCell<StoreHandle<OT, T>>>,
    // handle: StoreHandle<OT, T>,
    state: Signal<Option<StoreState<OT, T>>>,
    // peers, connections, etc
}

// TODO: Provide way to gracefully drop UseStore
// impl<OT: OdysseyType + 'static, T: CRDT<Time = OT::Time, Op: Serialize> + 'static> Drop for UseStore<OT, T> {
//     fn drop(&mut self) {
//         // JP: Gracefully shutdown store receiver somehow?
//         self.future.cancel();
// 
//         self.state.manually_drop(); // JP: Is this needed?
//     }
// }

impl<OT: OdysseyType + 'static, T: CRDT<Time = OT::Time, Op: ConcretizeTime<<OT::ECGHeader as ECGHeader>::HeaderId>>> Clone for UseStore<OT, T> {
    fn clone(&self) -> Self {
        UseStore {
            future: self.future.clone(),
            handle: self.handle.clone(),
            state: self.state.clone(),
        }
    }
}

// #[derive(Clone)]
struct StoreState<OT: OdysseyType, T: CRDT<Time = OT::Time>> {
    state: T,
    ecg: ecg::State<OT::ECGHeader, T>,
}

impl<OT: OdysseyType, T: CRDT<Time = OT::Time> + Clone> Clone for StoreState<OT, T>
where
    <OT as OdysseyType>::ECGHeader: Clone,
{
    fn clone(&self) -> Self {
        StoreState {
            state: self.state.clone(),
            ecg: self.ecg.clone(),
        }
    }
}

pub fn use_store<OT: OdysseyType + 'static, T: CRDT<Time = OT::Time, Op: ConcretizeTime<<OT::ECGHeader as ECGHeader>::HeaderId>>, F>(
    build_store_handle: F,
) -> UseStore<OT, T>
where
    F: FnOnce(&Odyssey<CookbookApplication>) -> StoreHandle<OT, T>,
{
    let scope = current_scope_id().expect("Failed to get scope id");
    let odyssey = use_context::<OdysseyProp<CookbookApplication>>().odyssey;
    let caller = std::panic::Location::caller();
    use_hook(|| new_store_helper(&odyssey, scope, caller, build_store_handle).unwrap())
}

fn new_store_helper<OT: OdysseyType + 'static, T: CRDT<Time = OT::Time, Op: ConcretizeTime<<OT::ECGHeader as ECGHeader>::HeaderId>>, F>(
    odyssey: &Odyssey<CookbookApplication>,
    scope: ScopeId,
    caller: &'static Location,
    build_store_handle: F,
) -> Option<UseStore<OT, T>>
where
    F: FnOnce(&Odyssey<CookbookApplication>) -> StoreHandle<OT, T>,
{
    let mut handle = build_store_handle(odyssey);
    let mut recv_st = handle.subscribe_to_state();

    // use_hook(|| Signal::new_maybe_sync_in_scope_with_caller(f(), scope, caller))
    let mut state = Signal::new_maybe_sync_in_scope_with_caller(None, scope, caller);

    let future = spawn_in_scope(scope, async move {
        debug!("Creating future for store");
        //     let mut recv_state = handle2.subscribe_to_state();
        //     // let mut recv_state = Rc::try_unwrap(recv_state).unwrap();
        //     // let mut recv_state = recv_state.clone();
        //     // let recv_state = Rc::get_mut(&mut recv_state).unwrap();
        while let Some(msg) = recv_st.recv().await {
            match msg {
                StateUpdate::Snapshot {
                    snapshot,
                    ecg_state,
                } => {
                    debug!("Received state!");
                    let s = StoreState {
                        state: snapshot,
                        ecg: ecg_state,
                    };
                    state.set(Some(s));
                }
                StateUpdate::Downloading {percent} => {
                    debug!("Store is downloading ({percent}%)");
                    state.set(None);
                }
            }
        }
        debug!("Future for store is exiting");
    });

    let Some(future) = future else {
        state.manually_drop(); // JP: Is this needed?
        return None;
    };

    let handle = Rc::new(RefCell::new(handle));
    Some(UseStore { future, handle, state })
}

pub fn new_store_in_scope<OT: OdysseyType + 'static, T: CRDT<Time = OT::Time, Op: ConcretizeTime<<OT::ECGHeader as ECGHeader>::HeaderId>>, F>(
    scope: ScopeId,
    build_store_handle: F,
) -> Option<UseStore<OT, T>>
where
    F: FnOnce(&Odyssey<CookbookApplication>) -> StoreHandle<OT, T>,
{
    let odyssey = use_context::<OdysseyProp<CookbookApplication>>().odyssey;
    let caller = std::panic::Location::caller();
    new_store_helper(&odyssey, scope, caller, build_store_handle)
}

/*
fn use_signal_in_scope<T: 'static>(scope: ScopeId, f: impl FnOnce() -> T) -> Signal<T, UnsyncStorage> {
    let caller = std::panic::Location::caller();
    use_hook(|| Signal::new_maybe_sync_in_scope_with_caller(f(), scope, caller))
}

// JP: Is this usage of ScopeId correct? Will this be deallocated properly?
fn use_store_in_scope<OT: OdysseyType + 'static, T: CRDT<Time = OT::Time, Op: Serialize>, F>(
    scope: ScopeId,
    build_store_handle: F,
) -> UseStore<OT, T>
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
        let recv_st = CopyValue::new_in_scope(recv_st, scope); // JP: Annoyingly required since dioxus requires clone... XXX
        (h, recv_st)
    });
    // let mut state = use_signal(|| None);
    let mut state = use_signal_in_scope(scope, || None);
    // let mut state: Signal<T> = use_signal(|| {
    //     // let recv_state = Rc::get_mut(&mut recv_state).unwrap();
    //     let init_st = recv_state.write().blocking_recv().unwrap();
    //     init_st
    //     // Rc::into_inner(init_st).unwrap()
    // });
    // let handle2 = handle.clone();
    // TODO...
    let future = use_future(move || async move {
        debug!("Creating future for store");
        //     let mut recv_state = handle2.subscribe_to_state();
        //     // let mut recv_state = Rc::try_unwrap(recv_state).unwrap();
        //     // let mut recv_state = recv_state.clone();
        //     // let recv_state = Rc::get_mut(&mut recv_state).unwrap();
        while let Some(msg) = recv_state.write().recv().await {
            match msg {
                StateUpdate::Snapshot {
                    snapshot,
                    ecg_state,
                } => {
                    debug!("Received state!");
                    let s = StoreState {
                        state: snapshot,
                        ecg: ecg_state,
                    };
                    state.set(Some(s));
                }
                StateUpdate::Downloading {percent} => {
                    debug!("Store is downloading ({percent}%)");
                    state.set(None);
                }
            }
        }
        debug!("Future for store is exiting");
    });
    UseStore { future, handle, state }
}
*/

pub struct OperationBuilder<OT: OdysseyType, T: CRDT<Time = OT::Time, Op: ConcretizeTime<<OT::ECGHeader as ECGHeader>::HeaderId>>> {
    handle: Rc<RefCell<StoreHandle<OT, T>>>,
    parents: BTreeSet<<<OT as OdysseyType>::ECGHeader as ECGHeader>::HeaderId>,
    operations: Vec<<T::Op as ConcretizeTime<<OT::ECGHeader as ECGHeader>::HeaderId>>::Serialized>,
}

const MAX_OPS: usize = 256; // TODO: This is already defined somewhere else?
impl<OT: OdysseyType, T: CRDT<Time = OT::Time, Op: ConcretizeTime<<OT::ECGHeader as ECGHeader>::HeaderId>>> OperationBuilder<OT, T> {
    /// Queue up an operation to apply.
    /// Cannot queue up more than 256 operations inside a single queue.
    /// This function will return `None` if more than 256 operations are queued.
    /// Only refer to the returned time in other operations inside this queue, otherwise the time reference will be incorrect. 
    pub fn queue<F>(&mut self, f: F) -> Option<CausalTime<OT::Time>>
    where
        OT::Time: Clone,
        F: FnOnce(CausalTime<OT::Time>) -> <T::Op as ConcretizeTime<<OT::ECGHeader as ECGHeader>::HeaderId>>::Serialized,
    {
        if self.operations.len() >= MAX_OPS {
            return None;
        }
        let t = CausalTime::current_time((self.operations.len()) as u8);
        self.operations.push(f(t.clone()));
        
        Some(t)
    }

    pub fn apply(self) -> <OT::ECGHeader as ECGHeader>::HeaderId
    where
        // T::Op<CausalTime<OT::Time>>: Serialize,
        T::Op: ConcretizeTime<<OT::ECGHeader as ECGHeader>::HeaderId>,
        OT::ECGBody<T>: ECGBody<T::Op, <T::Op as ConcretizeTime<<OT::ECGHeader as ECGHeader>::HeaderId>>::Serialized, Header = OT::ECGHeader>,
    {
        (*self.handle).borrow_mut().apply_batch(self.parents, self.operations)

        // let mut ops = vec![];
        // let mut ret = vec![];
        // for op in self.operations {
        //     if ops.len() >= MAX_OPS {
        //         ret.push((*self.handle).borrow_mut().apply_batch(self.parents.clone(), ops));
        //         ops = vec![];
        //     }

        //     ops.push(op);
        // }

        // if !ops.is_empty() {
        //     ret.push((*self.handle).borrow_mut().apply_batch(self.parents, ops));
        // }

        // ret
    }
}

impl<OT: OdysseyType, T: CRDT<Time = OT::Time, Op: ConcretizeTime<<OT::ECGHeader as ECGHeader>::HeaderId>>> UseStore<OT, T>
// where
//     T::Op<CausalTime<OT::Time>>: Serialize,
{
    // TODO: Apply operations, get current state, etc
    pub fn get_current_state(&self) -> Option<T>
    where
        T: Clone,
        <OT as OdysseyType>::ECGHeader: Clone,
    {
        self.state.cloned().map(|s| s.state)
    }

    pub fn get_current_store_state(&self) -> Option<StoreState<OT, T>>
    where
        T: Clone,
        <OT as OdysseyType>::ECGHeader: Clone,
    {
        self.state.cloned()
    }

    // fn apply_helper(
    //     &self,
    //     op: T::Op<CausalTime<OT::Time>>,
    // ) -> <OT::ECGHeader as ECGHeader>::HeaderId
    // where
    //     OT::ECGBody<T>: ECGBody<T, Header = OT::ECGHeader>,
    // {
    //     let parent_header_ids = {
    //         let cookbook_store_state = self.state.peek();
    //         let cookbook_store_state = cookbook_store_state.as_ref().expect("TODO");
    //         cookbook_store_state.ecg.tips().clone()
    //     };
    //     (*self.handle).borrow_mut().apply(parent_header_ids, op)
    // }

    /// Applies an operation to the Store's CRDT with the closure the builds the operation. If  you want to apply multiple operations, use `operation_builder`.
    pub fn apply<F>(
        &self,
        op: F,
    ) -> OperationId<<OT::ECGHeader as ECGHeader>::HeaderId>
    where
        F: FnOnce(CausalTime<OT::Time>) -> <T::Op as ConcretizeTime<<OT::ECGHeader as ECGHeader>::HeaderId>>::Serialized,
        T::Op: ConcretizeTime<<OT::ECGHeader as ECGHeader>::HeaderId>,
        OT::ECGBody<T>: ECGBody<T::Op, <T::Op as ConcretizeTime<<OT::ECGHeader as ECGHeader>::HeaderId>>::Serialized, Header = OT::ECGHeader>,
    {
        let parent_header_ids = {
            let cookbook_store_state = self.state.peek();
            let cookbook_store_state = cookbook_store_state.as_ref().expect("TODO");
            cookbook_store_state.ecg.tips().clone()
        };
        self.apply_with_parents(parent_header_ids, op)
    }

    pub fn apply_with_parents<F>(
        &self,
        parents: BTreeSet<<<OT as OdysseyType>::ECGHeader as ECGHeader>::HeaderId>,
        op: F,
    ) -> OperationId<<OT::ECGHeader as ECGHeader>::HeaderId>
    where
        F: FnOnce(CausalTime<OT::Time>) -> <T::Op as ConcretizeTime<<OT::ECGHeader as ECGHeader>::HeaderId>>::Serialized,
        T::Op: ConcretizeTime<<OT::ECGHeader as ECGHeader>::HeaderId>,
        OT::ECGBody<T>: ECGBody<T::Op, <T::Op as ConcretizeTime<<OT::ECGHeader as ECGHeader>::HeaderId>>::Serialized, Header = OT::ECGHeader>,
    {
        let t = CausalTime::current_time(0);
        let header_id = (*self.handle).borrow_mut().apply(parents, op(t));

        OperationId {
            header_id: Some(header_id),
            operation_position: 0,
        }
    }

    // pub fn apply_batch(
    //     &mut self,
    //     parents: BTreeSet<<<OT as OdysseyType>::ECGHeader as ECGHeader>::HeaderId>,
    //     op: Vec<T::Op<CausalTime<OT::Time>>>,
    // ) -> <OT::ECGHeader as ECGHeader>::HeaderId
    // where
    //     OT::ECGBody<T>: ECGBody<T, Header = OT::ECGHeader>,
    // {
    //     (*self.handle).borrow_mut().apply_batch(parents, op)
    // }

    pub fn operations_builder(&self) -> OperationBuilder<OT, T>
    where 
        T::Op: ConcretizeTime<<OT::ECGHeader as ECGHeader>::HeaderId>,
    {
        let cookbook_store_state = self.state.peek();
        let cookbook_store_state = cookbook_store_state.as_ref().expect("TODO");
        let parent_header_ids = cookbook_store_state.ecg.tips().clone();

        OperationBuilder {
            handle: self.handle.clone(),
            parents: parent_header_ids,
            operations: vec![],
        }
    }
}
