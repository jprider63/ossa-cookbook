
// use im::OrdMap;
use std::collections::BTreeMap;

// TODO: Actually switch to the corresponding CRDTs.
pub type LWW<A> = A;
pub type Sequence<A> = Vec<A>;
pub type OrderedMap<K,V> = BTreeMap<K,V>;
pub type RGA<A> = A;

pub type OdysseyRef<A> = A;
pub type Image = ();

pub type RecipeId = usize;
#[derive(Clone)]
pub struct Recipe {
    pub title: LWW<String>,
    pub ingredients: Sequence<String>,
    pub instructions: RGA<String>,
    pub image: Sequence<OdysseyRef<Image>>,
}

pub type CookbookId = usize;
#[derive(Clone)]
pub struct Cookbook {
    pub title: LWW<String>,
    pub recipes: OrderedMap<RecipeId, Recipe>,
}

