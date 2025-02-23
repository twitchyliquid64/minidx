//! Descriptors of different neural layers which can be composed into a network.

use minidx_core::layers::{
    Activation, Bias1d, Conv1d as Conv1dL, Dense as DenseL, Softmax as SoftmaxL, GLU as GLUL,
};
use minidx_core::matmul::MatMulImpl;
use minidx_core::{Const, Dtype};

/// A fully-connected layer with a fixed number of inputs and outputs. No bias.
#[derive(Clone, Copy, Debug, Default)]
pub struct Dense<const I: usize, const O: usize> {}

impl<const I: usize, const O: usize, E: Dtype + MatMulImpl> crate::Buildable<E> for Dense<I, O> {
    type Built = DenseL<E, I, O>;
    fn try_build(&self) -> Result<Self::Built, crate::Error> {
        Ok(DenseL::default())
    }
}

/// A fully-connected layer with a fixed number of inputs and outputs, and
/// learnable bias on each output. A standard pre-activation MLP layer.
///
/// This is the same as putting a [Bias1d] after a [Dense].
#[derive(Clone, Copy, Debug, Default)]
pub struct Linear<const I: usize, const O: usize> {}

impl<const I: usize, const O: usize, E: Dtype + MatMulImpl> crate::Buildable<E> for Linear<I, O> {
    type Built = (DenseL<E, I, O>, Bias1d<E, O>);
    fn try_build(&self) -> Result<Self::Built, crate::Error> {
        Ok((DenseL::default(), Bias1d::default()))
    }
}

/// The ReLu activation function.
#[derive(Clone, Copy, Debug, Default)]
pub struct Relu;

impl<E: Dtype + minidx_core::Float> crate::Buildable<E> for Relu {
    type Built = Activation<E>;
    fn try_build(&self) -> Result<Self::Built, crate::Error> {
        Ok(Activation::<E>::Relu)
    }
}

/// The Leaky-ReLu activation function.
#[derive(Clone, Copy, Debug)]
pub struct LeakyRelu(pub f32);

impl Default for LeakyRelu {
    fn default() -> Self {
        Self(0.5)
    }
}

impl<E: Dtype + minidx_core::Float> crate::Buildable<E> for LeakyRelu {
    type Built = Activation<E>;
    fn try_build(&self) -> Result<Self::Built, crate::Error> {
        Ok(Activation::<E>::LeakyRelu(E::from_f32(self.0).unwrap()))
    }
}

/// The Sigmoid activation function.
#[derive(Clone, Copy, Debug, Default)]
pub struct Sigmoid;

impl<E: Dtype + minidx_core::Float> crate::Buildable<E> for Sigmoid {
    type Built = Activation<E>;
    fn try_build(&self) -> Result<Self::Built, crate::Error> {
        Ok(Activation::<E>::Sigmoid)
    }
}

/// The Swish (SiLU) activation function.
#[derive(Clone, Copy, Debug, Default)]
pub struct Swish;

impl<E: Dtype + minidx_core::Float> crate::Buildable<E> for Swish {
    type Built = Activation<E>;
    fn try_build(&self) -> Result<Self::Built, crate::Error> {
        Ok(Activation::<E>::Swish)
    }
}

/// A softmax layer with a given temperature.
#[derive(Clone, Copy, Debug)]
pub struct Softmax(pub f32);

impl Default for Softmax {
    fn default() -> Self {
        Self(1.0)
    }
}

impl<E: Dtype + minidx_core::Float> crate::Buildable<E> for Softmax {
    type Built = SoftmaxL;
    fn try_build(&self) -> Result<Self::Built, crate::Error> {
        Ok(SoftmaxL(self.0))
    }
}

/// A GLU layer with a sigmoid gate.
#[derive(Clone, Copy, Debug, Default)]
pub struct GLU<const I: usize, const O: usize> {}

impl<const I: usize, const O: usize, E: Dtype + minidx_core::Float + MatMulImpl> crate::Buildable<E>
    for GLU<I, O>
{
    type Built = GLUL<E, I, O, Activation<E>>;
    fn try_build(&self) -> Result<Self::Built, crate::Error> {
        Ok(GLUL::sigmoid())
    }
}

/// A GLU layer with a leaky-relu gate.
#[derive(Clone, Copy, Debug)]
pub struct GLULeakyRelu<const I: usize, const O: usize>(pub f32);

impl<const I: usize, const O: usize> Default for GLULeakyRelu<I, O> {
    fn default() -> Self {
        Self(0.5)
    }
}

impl<const I: usize, const O: usize, E: Dtype + minidx_core::Float + MatMulImpl> crate::Buildable<E>
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

impl<
        const I: usize,
        const O: usize,
        const F: usize,
        E: Dtype + minidx_core::Float + MatMulImpl,
    > crate::Buildable<E> for Conv1d<I, O, F>
where
    Const<F>: minidx_core::layers::Conv1dKernel<E, Const<I>, Const<O>>,
{
    type Built = Conv1dL<E, I, O, Const<F>>;
    fn try_build(&self) -> Result<Self::Built, crate::Error> {
        Ok(Conv1dL::default())
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
