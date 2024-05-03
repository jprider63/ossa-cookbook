
use odyssey_crdt::{
    CRDT,
    register::LWW,
    map::twopmap::TwoPMap,
};
use odyssey_crdt::time::LamportTimestamp;

// use im::OrdMap;
use std::collections::BTreeMap;

// TODO: Actually switch to the corresponding CRDTs.
pub type Sequence<A> = Vec<A>;
pub type OrderedMap<K,V> = BTreeMap<K,V>;
pub type RGA<A> = A;

pub type OdysseyRef<A> = A;
pub type Image = ();

pub type UserId = u32;
pub type Time = LamportTimestamp<UserId>; // TODO: Switch to hashes for logical time. XXX
// pub struct RecipeId(Time); // JP: How do we get this newtype wrapper to work? `Into` instance?
pub type RecipeId = Time;
#[derive(Clone)]
pub struct Recipe {
    pub title: LWW<Time, String>,
    pub ingredients: LWW<Time, Vec<String>>, // Sequence<String>,
    pub instructions: LWW<Time, String>, // RGA<String>,
    pub image: Sequence<OdysseyRef<Image>>,
}

pub enum RecipeOp {
    Title(<LWW<Time, String> as CRDT>::Op),
    Ingredients(<LWW<Time, Vec<String>> as CRDT>::Op),
    Instructions(<LWW<Time, String> as CRDT>::Op),
}

impl CRDT for Recipe {
    type Op = RecipeOp;
    type Time = Time;

    fn apply(self, op_time: Time, op: Self::Op) -> Self {
        todo!()
    }
}

pub type CookbookId = usize;
#[derive(Clone)]
pub struct Cookbook {
    pub title: LWW<Time, String>,
    pub recipes: TwoPMap<RecipeId, Recipe>,
}

pub enum CookbookOp {
    Title(<LWW<Time, String> as CRDT>::Op),
    Recipes(<TwoPMap<RecipeId, Recipe> as CRDT>::Op),
}

impl CRDT for Cookbook {
    type Op = CookbookOp;
    type Time = Time;

    fn apply(self, op_time: Time, op: Self::Op) -> Self {
        todo!()
    }
}
