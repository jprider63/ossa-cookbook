use odyssey_crdt::{map::twopmap::TwoPMap, register::LWW, time::CausalState, CRDT};
// use odyssey_crdt::time::LamportTimestamp;
use odyssey_core::store::ecg::v0::{HeaderId, OperationId};
use odyssey_core::util::Sha256Hash;

// use im::OrdMap;
use serde::{Deserialize, Serialize};
use typeable::Typeable;
use std::collections::BTreeMap;

use crate::{CookbookApplication, UseStore};
use dioxus::prelude::Props;

// TODO: Actually switch to the corresponding CRDTs.
pub type Sequence<A> = Vec<A>;
pub type OrderedMap<K, V> = BTreeMap<K, V>;
pub type RGA<A> = A;

pub type OdysseyRef<A> = A;
pub type Image = ();

pub type UserId = u32;
// pub type Time = OperationId<Header<Sha256Hash, impl CRDT>, impl CRDT>;
pub type Time = OperationId<HeaderId<Sha256Hash>>;
// pub struct RecipeId(Time); // TODO: Newtype wrap this. JP: How do we get this newtype wrapper to work? `Into` instance?
pub type RecipeId = Time;
#[derive(Clone, Debug, PartialEq, Typeable, Serialize, Deserialize)]
pub struct Recipe {
    pub title: LWW<Time, String>,
    pub ingredients: LWW<Time, Vec<String>>, // Sequence<String>,
    pub instructions: LWW<Time, String>,     // RGA<String>,
                                             // pub image: Sequence<OdysseyRef<Image>>, // Sequence?
}

// TODO: Define the CBOR for this properly
// TODO: Derive this automatically. (use `heck` for case conversion) XXX
#[derive(Debug, Serialize, Deserialize)]
pub enum RecipeOp {
    Title(<LWW<Time, String> as CRDT>::Op),
    Ingredients(<LWW<Time, Vec<String>> as CRDT>::Op),
    Instructions(<LWW<Time, String> as CRDT>::Op),
}

// TODO: Derive this automatically. (use `heck` for case conversion) XXX
impl CRDT for Recipe {
    type Op = RecipeOp;
    type Time = Time;

    fn apply<CS: CausalState<Time = Self::Time>>(
        self,
        st: &CS,
        op_time: Time,
        op: Self::Op,
    ) -> Self {
        match op {
            RecipeOp::Title(t) => Recipe {
                title: self.title.apply(st, op_time, t),
                ..self
            },
            RecipeOp::Ingredients(i) => Recipe {
                ingredients: self.ingredients.apply(st, op_time, i),
                ..self
            },
            RecipeOp::Instructions(i) => Recipe {
                instructions: self.instructions.apply(st, op_time, i),
                ..self
            },
        }
    }
}

pub type CookbookId = usize; // TODO: Newtype wrap this.
#[derive(Clone, Debug, Serialize, Typeable)]
pub struct Cookbook {
    pub title: LWW<Time, String>,
    pub recipes: TwoPMap<RecipeId, Recipe>,
}

// TODO: Define the CBOR for this properly
// TODO: Derive this automatically. (use `heck` for case conversion) XXX
#[derive(Debug, Serialize, Deserialize)]
pub enum CookbookOp {
    Title(<LWW<Time, String> as CRDT>::Op),
    Recipes(<TwoPMap<RecipeId, Recipe> as CRDT>::Op),
}

// TODO: Derive this automatically. (use `heck` for case conversion) XXX
impl CRDT for Cookbook {
    type Op = CookbookOp;
    type Time = Time;

    fn apply<CS: CausalState<Time = Self::Time>>(
        self,
        st: &CS,
        op_time: Time,
        op: Self::Op,
    ) -> Self {
        match op {
            CookbookOp::Title(t) => Cookbook {
                title: self.title.apply(st, op_time, t),
                ..self
            },
            CookbookOp::Recipes(rs) => Cookbook {
                recipes: self.recipes.apply(st, op_time, rs),
                ..self
            },
        }
    }
}

pub type State = Vec<UseStore<CookbookApplication, Cookbook>>;

// use std::marker::PhantomData;
// #[derive(Props)]
// pub struct State<'a> {
//     pub cookbooks: Vec<UseStore<CookbookApplication, Cookbook>>,
//     _phantom: PhantomData<'a, ()>,
// }
