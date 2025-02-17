pub use minidx_core as core;

pub mod layer_spec;
pub use minidx_core::{train_batch, train_step};
use minidx_core::{Dtype, Error};

pub mod prelude {
    pub use crate::layer_spec as layers;
    pub use crate::Buildable;
    pub use minidx_core::optimizers::TrainParams;
    pub use minidx_core::{BackpropModule, Error, Module, ResetParams, TracedModule};

    pub use crate::{train_batch, train_step};
}

pub mod problem;

/// OneHotEncoder describes the encoding of some integer value modulus N into
/// a vector where exactly one value is set.
#[derive(Clone, Debug, Default)]
pub struct OneHotEncoder<const N: usize> {
    modulus: std::marker::PhantomData<[bool; N]>,
}

impl<const N: usize> OneHotEncoder<N> {
    pub fn value<E: Dtype>(idx: usize) -> [E; N] {
        let mut out = [E::default(); N];
        out[idx] = E::ONE;
        out
    }
}

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_one_hot_encoding() {
        assert_eq!(OneHotEncoder::<3>::value::<f32>(0), [1.0f32, 0.0, 0.0]);
        assert_eq!(OneHotEncoder::<3>::value::<f32>(2), [0.0f32, 0.0, 1.0]);
        assert_eq!(OneHotEncoder::<4>::value::<f32>(1), [0.0f32, 1.0, 0.0, 0.0]);
    }
}
