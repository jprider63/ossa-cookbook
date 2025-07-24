
use odyssey_core::store::ecg::v0::{HeaderId, OperationId};
use odyssey_core::time::{CausalTime, ConcretizeTime};
use odyssey_core::util::Sha256Hash;
use odyssey_crdt::map::twopmap::TwoPMapOp;
use odyssey_crdt::{map::twopmap::TwoPMap, register::LWW, time::CausalState, CRDT};

use serde::{Deserialize, Serialize};
use typeable::Typeable;

use crate::{CookbookApplication, UseStore};

pub type Time = OperationId<HeaderId<Sha256Hash>>;

// pub struct RecipeId(Time); // TODO: Newtype wrap this. JP: How do we get this newtype wrapper to work? `Into` instance?
pub type RecipeId<Time> = Time;
#[derive(Clone, Debug, PartialEq, Typeable, Serialize, Deserialize)]
pub struct Recipe<Time> {
    pub title: LWW<Time, String>,
    pub ingredients: LWW<Time, Vec<String>>, // Sequence<String>,
    pub instructions: LWW<Time, String>,     // RGA<String>,
                                             // pub image: Sequence<OdysseyRef<Image>>, // Sequence?
}

// TODO: Define the CBOR for this properly
// TODO: Derive this automatically. (use `heck` for case conversion) XXX
#[derive(Debug, Serialize, Deserialize)]
pub enum RecipeOp<Time> {
    Title(LWW<Time, String>),
    Ingredients(LWW<Time, Vec<String>>),
    Instructions(LWW<Time, String>),
    // Title(<LWW<Time, String> as CRDT>::Op<Time>),
    // Ingredients(<LWW<Time, Vec<String>> as CRDT>::Op<Time>),
    // Instructions(<LWW<Time, String> as CRDT>::Op<Time>),
}

// TODO: Derive this automatically. (use `heck` for case conversion) XXX
impl<Time: Ord> CRDT for Recipe<Time> {
    type Op = RecipeOp<Time>;
    type Time = Time;

    fn apply<CS: CausalState<Time = Self::Time>>(
        self,
        st: &CS,
        op: Self::Op,
    ) -> Self {
        match op {
            RecipeOp::Title(t) => Recipe {
                title: self.title.apply(st, t),
                ..self
            },
            RecipeOp::Ingredients(i) => Recipe {
                ingredients: self.ingredients.apply(st, i),
                ..self
            },
            RecipeOp::Instructions(i) => Recipe {
                instructions: self.instructions.apply(st, i),
                ..self
            },
        }
    }
}

pub type CookbookId = usize; // TODO: Newtype wrap this.
#[derive(Clone, Debug, Deserialize, Serialize, Typeable)]
pub struct Cookbook<Time: Clone + Ord> {
    pub title: LWW<Time, String>,
    pub recipes: TwoPMap<RecipeId<Time>, Recipe<Time>>,
}

// TODO: Define the CBOR for this properly
// TODO: Derive this automatically. (use `heck` for case conversion) XXX
#[derive(Debug, Serialize, Deserialize)]
pub enum CookbookOp<Time> {
    Title(LWW<Time, String>),
    Recipes(TwoPMapOp<RecipeId<Time>, Recipe<Time>, RecipeOp<Time>>),
    // Title(<LWW<Time, String> as CRDT>::Op<Time>),
    // Recipes(<TwoPMap<RecipeId, Recipe> as CRDT>::Op<Time>),
}

impl<HeaderId: Clone, Time: ConcretizeTime<HeaderId, Serialized = CausalTime<Time>>> ConcretizeTime<HeaderId> for Recipe<Time> {
    type Serialized = Recipe<CausalTime<Time>>;

    fn concretize_time(src: Self::Serialized, current_header: HeaderId) -> Self {
        Recipe {
            title: LWW::concretize_time(src.title, current_header.clone()),
            ingredients: LWW::concretize_time(src.ingredients, current_header.clone()),
            instructions: LWW::concretize_time(src.instructions, current_header.clone()),
        }
    }
}

impl<HeaderId, Time: ConcretizeTime<HeaderId, Serialized = CausalTime<Time>>> ConcretizeTime<HeaderId> for RecipeOp<Time> {
    type Serialized = RecipeOp<CausalTime<Time>>;

    fn concretize_time(src: Self::Serialized, current_header: HeaderId) -> Self {
        match src {
            RecipeOp::Title(lww) => RecipeOp::Title(LWW::concretize_time(lww, current_header)),
            RecipeOp::Ingredients(lww) => RecipeOp::Ingredients(LWW::concretize_time(lww, current_header)),
            RecipeOp::Instructions(lww) => RecipeOp::Instructions(LWW::concretize_time(lww, current_header)),
        }
    }
}

impl<HeaderId: Clone, Time: ConcretizeTime<HeaderId, Serialized = CausalTime<Time>>> ConcretizeTime<HeaderId> for CookbookOp<Time> {
    type Serialized = CookbookOp<CausalTime<Time>>;

    fn concretize_time(src: Self::Serialized, current_time: HeaderId) -> Self {
        match src {
            CookbookOp::Title(lww) => CookbookOp::Title(<LWW<Time, String> as ConcretizeTime<HeaderId>>::concretize_time(lww, current_time)),
            CookbookOp::Recipes(two_pmap_op) => CookbookOp::Recipes(TwoPMapOp::concretize_time(two_pmap_op, current_time)),
        }
    }
}

// TODO: Derive this automatically. (use `heck` for case conversion) XXX
impl<Time: Ord + Clone> CRDT for Cookbook<Time> {
    type Op = CookbookOp<Time>;
    type Time = Time;

    fn apply<CS: CausalState<Time = Self::Time>>(
        self,
        st: &CS,
        op: Self::Op,
    ) -> Self {
        match op {
            CookbookOp::Title(t) => Cookbook {
                title: self.title.apply(st, t),
                ..self
            },
            CookbookOp::Recipes(rs) => Cookbook {
                recipes: self.recipes.apply(st, rs),
                ..self
            },
        }
    }
}

pub type State = Vec<UseStore<CookbookApplication, Cookbook<Time>>>;

// impl<T, U> OperationFunctor<T, U> for RecipeOp<T> {
//     type Target<Time> = RecipeOp<Time>;
// 
//     fn fmap(self, f: impl Fn(T) -> U) -> Self::Target<U> {
//         match self {
//             RecipeOp::Title(op) => RecipeOp::Title(op.fmap(f)),
//             RecipeOp::Ingredients (op) => RecipeOp::Ingredients(op.fmap(f)),
//             RecipeOp::Instructions(op) => RecipeOp::Instructions(op.fmap(f)),
//         }
//     }
// }
// 
// impl<T, U> OperationFunctor<T, U> for CookbookOp<T> {
//     type Target<Time> = CookbookOp<Time>;
// 
//     fn fmap(self, f: impl Fn(T) -> U) -> Self::Target<U> {
//         match self {
//             CookbookOp::Title(op) => CookbookOp::Title(op.fmap(f)),
//             CookbookOp::Recipes(op) => CookbookOp::Recipes(op.fmap(f)), // <TwoPMapOp<RecipeId<T>, Recipe<T>, RecipeOp<T>> as OperationFunctor<T, U>>::fmap(op, f)),
//         }
//     }
// }

// use std::marker::PhantomData;
// #[derive(Props)]
// pub struct State<'a> {
//     pub cookbooks: Vec<UseStore<CookbookApplication, Cookbook>>,
//     _phantom: PhantomData<'a, ()>,
// }
