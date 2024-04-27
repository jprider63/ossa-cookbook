
use odyssey_crdt::{
    CRDT,
    register::LWW
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
pub type RecipeId = usize;
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

pub type CookbookId = usize;
#[derive(Clone)]
pub struct Cookbook {
    pub title: String, // TODO: LWW<String>, XXX
    pub recipes: OrderedMap<RecipeId, Recipe>,
}

