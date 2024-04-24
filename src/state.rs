
use odyssey_crdt::register::LWW;
use odyssey_crdt::time::LamportTimestamp;

// use im::OrdMap;
use std::collections::BTreeMap;

// TODO: Actually switch to the corresponding CRDTs.
pub type Sequence<A> = Vec<A>;
pub type OrderedMap<K,V> = BTreeMap<K,V>;
pub type RGA<A> = A;

pub type OdysseyRef<A> = A;
pub type Image = ();

type UserId = u32;
type Time = LamportTimestamp<UserId>; // TODO: Switch to hashes for logical time. XXX
pub type RecipeId = usize;
#[derive(Clone)]
pub struct Recipe {
    pub title: LWW<Time, String>,
    pub ingredients: Sequence<String>,
    pub instructions: RGA<String>,
    pub image: Sequence<OdysseyRef<Image>>,
}

pub type CookbookId = usize;
#[derive(Clone)]
pub struct Cookbook {
    pub title: String, // TODO: LWW<String>, XXX
    pub recipes: OrderedMap<RecipeId, Recipe>,
}

