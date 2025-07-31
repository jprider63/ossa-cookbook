pub mod internal;

use ossa_crdt::map::twopmap::TwoPMapOp;
use ossa_crdt::{map::twopmap::TwoPMap, register::LWW, time::CausalState, CRDT};
use ossa_core::store::ecg::v0::{HeaderId, OperationId};
use ossa_core::util::Sha256Hash;

use serde::{Deserialize, Serialize};
use typeable::Typeable;
use std::collections::BTreeMap;

pub use internal::{CookbookOp, RecipeOp, State, Time};


// TODO: Actually switch to the corresponding CRDTs.
// pub type Sequence<A> = Vec<A>;
// pub type OrderedMap<K, V> = BTreeMap<K, V>;
// pub type RGA<A> = A;
// 
// pub type OssaRef<A> = A;
// pub type Image = ();
// 
// pub type UserId = u32;
// pub type Time = OperationId<Header<Sha256Hash, impl CRDT>, impl CRDT>;

pub type CookbookId = internal::CookbookId;
pub type Cookbook = internal::Cookbook<Time>;
pub type RecipeId = internal::RecipeId<Time>;
pub type Recipe = internal::Recipe<Time>;
