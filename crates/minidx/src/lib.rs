pub use minidx_core as core;

pub mod layer_spec;
pub use minidx_core::train_step;
use minidx_core::{Dtype, Error};

pub mod prelude {
    pub use crate::layer_spec as layers;
    pub use crate::Buildable;
    pub use minidx_core::optimizers::TrainParams;
    pub use minidx_core::{BackpropModule, Error, Module, ResetParams, TracedModule};

    pub use crate::train_step;
}

pub mod problem;

/// A layer or composition of layers that can be constructed, using some Dtype as the element type.
pub trait Buildable<E: Dtype>: Clone {
    type Built: Clone + std::fmt::Debug;
    fn build(&self) -> Self::Built {
        self.try_build().unwrap()
    }
    fn try_build(&self) -> Result<Self::Built, Error>;
}

macro_rules! tuple_impls {
    ([$($name:ident),+], [$($idx:tt),+]) => {

        impl<Elem: Dtype, $($name: Buildable<Elem>),+> Buildable<Elem> for ($($name,)+) {
            type Built = ($($name::Built, )+);
            fn try_build(&self) -> Result<Self::Built, Error> {
                Ok(($(
                    self.$idx.try_build()?,
                )+))
            }
        }

    }
}

tuple_impls!([M1], [0]);
tuple_impls!([M1, M2], [0, 1]);
tuple_impls!([M1, M2, M3], [0, 1, 2]);
tuple_impls!([M1, M2, M3, M4], [0, 1, 2, 3]);
tuple_impls!([M1, M2, M3, M4, M5], [0, 1, 2, 3, 4]);
tuple_impls!([M1, M2, M3, M4, M5, M6], [0, 1, 2, 3, 4, 5]);
