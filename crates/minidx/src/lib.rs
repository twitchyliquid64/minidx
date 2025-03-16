//! Minidx helps you implement small to medium-sized neural networks.
//!
//! ### Defining network architecture
//!
//! In minidx, you define your network using tuples of layers, with the
//! dimensionality of inputs/outputs defined as generic constants.
//! For instance, the below example defines a network which takes 2 inputs
//! and produces 3 outputs, by first going through two hidden layers with
//! a hidden dimension of 3 and a relu activation, before a softmax layer.
//!
//! ```
//! use minidx::prelude::*;
//! use layers::*;
//!
//! type network = (
//!   (Linear::<2, 3>, Relu), // Fully-connected + bias layer with relu activation
//!   (Linear::<3, 3>, Relu),
//!   Softmax,
//! );
//!
//! // Instantiates our neural network.
//! let mut network = Buildable::<f32>::build(&network::default());
//! ```
//!
//! You can see the full set of implemented layers in the [layer_spec] module.
//!
//! ### Random initialization of a network
//!
//! Before training, you likely want to initialize the parameters of the network
//! to reasonable random values.
//!
//! ```
//! # use minidx::prelude::*;
//! # use layers::*;
//! # type network = (
//! #   (Linear::<2, 3>, Relu),
//! #   Softmax,
//! # );
//! # let mut network = Buildable::<f32>::build(&network::default());
//! use rand::{SeedableRng, rngs::SmallRng};
//! let mut rng = SmallRng::seed_from_u64(42);
//! network.rand_params(&mut rng, 0.5).unwrap();
//! ```
//!
//! [`rand_params`](`core::ResetParams::rand_params`) performs sensible initialization of each layer using
//! the given RNG. The float argument represents the max magnitude of random parameters. `0.5` to `1.0` is a good starting parameter.
//!
//! ### Training
//!
//! Training a network in minidx requires two things:
//!
//!  - An updater: some object that stores training state and implements
//!    the optimizer algorithm you want to use
//!  - A training loop: a loop where you call [train_step] or [train_batch]
//!    with network inputs and their correct outputs, and a closure that wires up
//!    the loss function you want to use.
//!
//! ```
//! # use minidx::prelude::*;
//! # use layers::*;
//! # type network = (
//! #   (Linear::<2, 3>, Relu),
//! #   Softmax,
//! # );
//! # let mut network = Buildable::<f32>::build(&network::default());
//! # use rand::{SeedableRng, rngs::SmallRng};
//! # let mut rng = SmallRng::seed_from_u64(42);
//! # network.rand_params(&mut rng, 0.5).unwrap();
//! // initialize training state
//! let mut updater = network.new_momentum(
//!     TrainParams::with_lr(1.0e-5).and_l2(1.0e-6), 0.4);
//!
//! // train the network with 50 examples
//! for _i in 0..50 {
//!     // fake training data
//!     let input = [1.0, 2.0];
//!     let output = [1.0, 0.0, 0.0];
//!     // train on an individual input/output pair, using the
//!     // mean-square error (MSE) loss function.
//!     use loss::DiffLoss;
//!     train_step(
//!         &mut updater,
//!         &mut network,
//!         |got, want| (got.mse(want), got.mse_input_grads(want)),
//!         input,
//!         output,
//!     );
//! }
//! ```
//!
//! Everything is fairly self-explanatory except for the closure you need to pass for your loss function.
//! That function takes both the output of the network as well as the correct output of the network, and
//! needs to return the loss with respect to the output as well as the gradient of the loss with respect
//! to the loss function. The [minidx::prelude::loss] module contains implemented loss functions and
//! corresponding methods to compute their gradients.
//!
//! Its also worth noting that there are batch and threaded-batch variants of [train_step], namely [train_batch]
//! and [train_batch_parallel]. Both batch training methods return the average loss over the samples.
//!
//! ### Inference
//!
//! You can run inference over a trained network using [`forward()`](`core::Module::forward`):
//!
//! ```
//! # use minidx::prelude::*;
//! # use layers::*;
//! # type network = (
//! #   (Linear::<2, 3>, Relu),
//! #   Softmax,
//! # );
//! # let mut network = Buildable::<f32>::build(&network::default());
//! let output = network.forward(&[1.0, 2.0]).unwrap(); // outputs [f32; 3]
//! ```
//!
//! Networks can be loaded and stored using [`LoadableModule`](core::LoadableModule).
pub use minidx_core as core;

pub mod layer_spec;
pub use minidx_core::{train_batch, train_batch_parallel, train_step};
use minidx_core::{Dtype, Error};

/// Common types and traits needed when using minidx.
pub mod prelude {
    pub use crate::layer_spec as layers;
    pub use crate::Buildable;
    pub use minidx_core::loss;
    pub use minidx_core::optimizers::TrainParams;
    pub use minidx_core::{
        BackpropModule, Error, LoadableModule, Module, ResetParams, TracedModule,
    };

    pub use crate::{train_batch, train_batch_parallel, train_step};
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
