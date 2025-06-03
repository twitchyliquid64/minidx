//! Descriptors of different neural layers which can be composed into a network.
//!
//! You can compose these layers into a neural network using tuples, e.g.:
//!
//! ```
//! use minidx::prelude::layers::*;
//! type network = (
//!   (Dense::<1, 3>, Relu),
//!   (Dense::<3, 3>, Relu),
//!   Softmax,
//! );
//! ```
//! Which you can then instantiate to create a network you can train or run inference on:
//! ```
//! # use minidx::prelude::*;
//! # use minidx::prelude::layers::*;
//! # type network = (
//! #   (Dense::<1, 3>, Relu),
//! #   (Dense::<3, 3>, Relu),
//! #   Softmax,
//! # );
//! use minidx::Buildable;
//! let my_net = Buildable::<f32>::build(&network::default());
//! ```
//!
use crate::Buildable;
use minidx_core::layers::{
    Activation, Bias1d, Conv1d as Conv1dL, Dense as DenseL, Diag, ScalarScale, Softmax as SoftmaxL,
    Swish as SwishL, GLU as GLUL, LR,
};
use minidx_core::matmul::MatMulImpl;
use minidx_core::{Const, Dtype, Float};

/// A fully-connected layer with a fixed number of inputs and outputs. No bias.
///
///  - **I**: The number of inputs this layer takes.
///  - **O**: The number of outputs this layer produces.
///
/// This results in `I*O` number of learnable parameters.
#[derive(Clone, Copy, Debug, Default)]
pub struct Dense<const I: usize, const O: usize> {}

impl<const I: usize, const O: usize, E: Dtype + MatMulImpl> Buildable<E> for Dense<I, O> {
    type Built = DenseL<E, I, O>;
    fn try_build(&self) -> Result<Self::Built, crate::Error> {
        Ok(DenseL::default())
    }
}

/// A fully-connected layer with a fixed number of inputs and outputs, and
/// learnable bias on each output. A standard pre-activation MLP layer.
///
/// This is the same as putting a [Bias1d] after a [Dense].
///
///  - **I**: The number of inputs this layer takes.
///  - **O**: The number of outputs this layer produces.
///
/// This results in `I*O + O` number of learnable parameters.
#[derive(Clone, Copy, Debug, Default)]
pub struct Linear<const I: usize, const O: usize> {}

impl<const I: usize, const O: usize, E: Dtype + MatMulImpl> Buildable<E> for Linear<I, O> {
    type Built = (DenseL<E, I, O>, Bias1d<E, O>);
    fn try_build(&self) -> Result<Self::Built, crate::Error> {
        Ok((DenseL::default(), Bias1d::default()))
    }
}

/// The ReLu activation function.
///
/// `Output = max(0, Input)`
///
/// This layer produces the same number of outputs as given inputs, and there
/// are no learnable parameters.
#[derive(Clone, Copy, Debug, Default)]
pub struct Relu;

impl<E: Dtype + Float> Buildable<E> for Relu {
    type Built = Activation<E>;
    fn try_build(&self) -> Result<Self::Built, crate::Error> {
        Ok(Activation::<E>::Relu)
    }
}

/// The Leaky-ReLu activation function.
///
/// The given parameter `.0` describes the multiplier to apply to each input when
/// input is less than zero.
///
/// `Output = max(Input * self.0, Input)`
///
/// This layer produces the same number of outputs as given inputs, and there
/// are no learnable parameters.
#[derive(Clone, Copy, Debug)]
pub struct LeakyRelu(pub f32);

impl Default for LeakyRelu {
    fn default() -> Self {
        Self(0.5)
    }
}

impl<E: Dtype + Float> Buildable<E> for LeakyRelu {
    type Built = Activation<E>;
    fn try_build(&self) -> Result<Self::Built, crate::Error> {
        Ok(Activation::<E>::LeakyRelu(E::from_f32(self.0).unwrap()))
    }
}

/// The Sigmoid activation function.
///
/// `Output = sigmoid(Input)`
///
/// This layer produces the same number of outputs as given inputs, and there
/// are no learnable parameters.
#[derive(Clone, Copy, Debug, Default)]
pub struct Sigmoid;

impl<E: Dtype + Float> Buildable<E> for Sigmoid {
    type Built = Activation<E>;
    fn try_build(&self) -> Result<Self::Built, crate::Error> {
        Ok(Activation::<E>::Sigmoid)
    }
}

/// The tanh activation function.
///
/// `Output = tanh(Input)`
///
/// This layer produces the same number of outputs as given inputs, and there
/// are no learnable parameters.
#[derive(Clone, Copy, Debug, Default)]
pub struct Tanh;

impl<E: Dtype + Float> Buildable<E> for Tanh {
    type Built = Activation<E>;
    fn try_build(&self) -> Result<Self::Built, crate::Error> {
        Ok(Activation::<E>::Tanh)
    }
}

/// The SiLU (Swish /w Beta=1) activation function.
///
/// `Output = Input * sigmoid(Input)`
///
/// This layer produces the same number of outputs as given inputs, and there
/// are no learnable parameters.
#[derive(Clone, Copy, Debug, Default)]
pub struct SiLU;

impl<E: Dtype + Float> Buildable<E> for SiLU {
    type Built = Activation<E>;
    fn try_build(&self) -> Result<Self::Built, crate::Error> {
        Ok(Activation::<E>::SiLU)
    }
}

/// A softmax layer with a given temperature.
///
/// This layer produces the same number of outputs as given inputs, and there
/// are no learnable parameters.
#[derive(Clone, Copy, Debug)]
pub struct Softmax(pub f32);

impl Default for Softmax {
    fn default() -> Self {
        Self(1.0)
    }
}

impl<E: Dtype + Float> Buildable<E> for Softmax {
    type Built = SoftmaxL;
    fn try_build(&self) -> Result<Self::Built, crate::Error> {
        Ok(SoftmaxL(self.0))
    }
}

/// A GLU layer with a sigmoid gate.
///
/// Gated Linear Units learn a linear transform + bias of the input
/// values to the output values, while also learning when to activate
/// each output (the 'gate', a linear transform + bias + sigmoid).
///
///  - **I**: The number of inputs this layer takes.
///  - **O**: The number of outputs this layer produces.
///  - **E**: The datatype of the parameters (i.e. [f32]).
///
/// This results in `2*I*O + 2O` number of learnable parameters.
#[derive(Clone, Copy, Debug, Default)]
pub struct GLU<const I: usize, const O: usize> {}

impl<const I: usize, const O: usize, E: Dtype + Float + MatMulImpl> Buildable<E> for GLU<I, O> {
    type Built = GLUL<E, I, O, Activation<E>>;
    fn try_build(&self) -> Result<Self::Built, crate::Error> {
        Ok(GLUL::sigmoid())
    }
}

/// A GLU layer with a Swish gate.
///
/// Gated Linear Units learn a linear transform + bias of the input
/// values to the output values, while also learning when to activate
/// each output (the 'gate', a linear transform + bias + swish).
///
///  - **I**: The number of inputs this layer takes.
///  - **O**: The number of outputs this layer produces.
///  - **E**: The datatype of the parameters (i.e. [f32]).
///
/// This results in `2*I*O + 3O` number of learnable parameters.
#[derive(Clone, Copy, Debug, Default)]
pub struct SwiGLU<const I: usize, const O: usize> {}

impl<const I: usize, const O: usize, E: Dtype + Float + MatMulImpl> Buildable<E> for SwiGLU<I, O> {
    type Built = GLUL<E, I, O, SwishL<E, O>>;
    fn try_build(&self) -> Result<Self::Built, crate::Error> {
        Ok(GLUL::swish())
    }
}

/// A GLU layer with a leaky-relu gate.
///
/// Gated Linear Units learn a linear transform + bias of the input
/// values to the output values, while also learning when to activate
/// each output (the 'gate', a linear transform + bias + leaky-relu).
///
///  - **I**: The number of inputs this layer takes.
///  - **O**: The number of outputs this layer produces.
///  - **E**: The datatype of the parameters (i.e. [f32]).
///
/// This results in `2*I*O + 2O` number of learnable parameters.
#[derive(Clone, Copy, Debug)]
pub struct GLULeakyRelu<const I: usize, const O: usize>(pub f32);

impl<const I: usize, const O: usize> Default for GLULeakyRelu<I, O> {
    fn default() -> Self {
        Self(0.5)
    }
}

impl<const I: usize, const O: usize, E: Dtype + Float + MatMulImpl> Buildable<E>
    for GLULeakyRelu<I, O>
{
    type Built = GLUL<E, I, O, Activation<E>>;
    fn try_build(&self) -> Result<Self::Built, crate::Error> {
        Ok(GLUL::leaky_relu(self.0))
    }
}

/// A 1-dimensional convolution with specified input size, output size, and filter width.
#[derive(Clone, Copy, Debug, Default)]
pub struct Conv1d<const I: usize, const O: usize, const F: usize> {}

impl<const I: usize, const O: usize, const F: usize, E: Dtype + Float + MatMulImpl> Buildable<E>
    for Conv1d<I, O, F>
where
    Const<F>: minidx_core::layers::Conv1dKernel<E, Const<I>, Const<O>>,
{
    type Built = Conv1dL<E, I, O, Const<F>>;
    fn try_build(&self) -> Result<Self::Built, crate::Error> {
        Ok(Conv1dL::default())
    }
}

/// The Swish activation function with learnable beta.
///
///  - **I**: The number of inputs this layer takes.
///
/// This results in `2*I*O + 2O` number of learnable parameters.
#[derive(Clone, Copy, Debug, Default)]
pub struct Swish<const I: usize> {}

impl<const I: usize, E: Dtype + Float> Buildable<E> for Swish<I> {
    type Built = SwishL<E, I>;
    fn try_build(&self) -> Result<Self::Built, crate::Error> {
        Ok(SwishL::default())
    }
}

/// Learning Rate Divisor - wrapper to reduce local learning rate.
///
///  - **E**: The datatype of the parameters (i.e. [f32]).
///  - **I**: The number of inputs this layer takes.
///  - **D**: The divisor of the learning rate.
///  - **B**: The layer(s) this layer wraps.
#[derive(Clone, Copy, Debug, Default)]
pub struct LRDiv<E: Dtype, const I: usize, const D: usize, B: Buildable<E>> {
    module: B,
    pd: std::marker::PhantomData<E>,
}

impl<
        E: Dtype,
        const I: usize,
        const D: usize,
        B: crate::Buildable<E, Built = M>,
        M: Clone + Default + std::fmt::Debug + minidx_core::Module<[E; I]>,
    > Buildable<E> for LRDiv<E, I, D, B>
{
    type Built = LR<E, I, M>;
    fn try_build(&self) -> Result<Self::Built, crate::Error> {
        Ok(LR {
            module: self.module.try_build()?,
            update_multiplier: (D as f32).recip(),
            ..Default::default()
        })
    }
}

/// The 'Dynamic Tanh' normalization layer.
/// See: <https://arxiv.org/abs/2503.10622>
///
///  - **I**: The number of inputs/outputs this layer takes.
///
/// This results in `I*2 + 1` number of learnable parameters.
#[derive(Clone, Copy, Debug, Default)]
pub struct DyT<const I: usize> {}

impl<const I: usize, E: Float> Buildable<E> for DyT<I> {
    type Built = (ScalarScale<E>, Activation<E>, Diag<E, I>, Bias1d<E, I>);
    fn try_build(&self) -> Result<Self::Built, crate::Error> {
        Ok((
            ScalarScale::<E>::default(),
            Activation::<E>::Tanh,
            Diag::<E, I>::default(),
            Bias1d::default(),
        ))
    }
}

/// The Softplus activation function.
///
/// `Output = Ln(1 + e^Input)`
///
/// This layer produces the same number of outputs as given inputs, and there
/// are no learnable parameters.
#[derive(Clone, Copy, Debug, Default)]
pub struct Softplus;

impl<E: Dtype + Float> Buildable<E> for Softplus {
    type Built = Activation<E>;
    fn try_build(&self) -> Result<Self::Built, crate::Error> {
        Ok(Activation::<E>::Softplus)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_composition() {
        let network = (
            (Linear::<1, 3> {}, Relu),
            (Linear::<1, 3> {}, Relu),
            Dense::<3, 3> {},
            LeakyRelu(0.5),
            Softmax::default(),
        );

        use crate::Buildable;
        let _realized = Buildable::<f32>::build(&network);
        let _realized = Buildable::<f32>::build(&(GLU::<3, 2>::default(),));
        let _realized = Buildable::<f32>::build(&(Conv1d::<4, 2, 3>::default(),));
    }

    #[test]
    fn test_basic_typed_composition() {
        type NetType = ((Linear<1, 3>, Relu), LeakyRelu);
        let network = NetType::default();

        use crate::Buildable;
        let _realized = Buildable::<f32>::build(&network);
    }
}
