use crate::matmul::MatMulImpl;
use crate::{Const, ConstDim, Dtype, Gradients};

/// A 1d convolution across input parameters, with the specified filter and output sizes.
#[derive(Clone, Debug, Default)]
pub struct Conv1d<
    E: Dtype + MatMulImpl,
    const I: usize,
    const O: usize,
    C: Conv1dKernel<E, Const<I>, Const<O>>,
> {
    pub(crate) weights: C::Weights,
}

impl<
        E: Dtype + MatMulImpl,
        const I: usize,
        const O: usize,
        C: Conv1dKernel<E, Const<I>, Const<O>>,
    > Conv1d<E, I, O, C>
{
    fn forward(&mut self, x: &[E; I]) -> [E; O] {
        let mut output = [E::default(); O];
        output.iter_mut().enumerate().for_each(|(i, o)| {
            *o = x[i..]
                .iter()
                .zip(self.weights.as_ref())
                .map(|(i, w)| *i * *w)
                .fold(E::default(), |a, x| a + x);
        });

        output
    }

    #[inline]
    fn gradients_wrt_input(&self, output_gradients: &[E; O]) -> [E; I] {
        let mut out = [E::default(); I];
        output_gradients.iter().enumerate().for_each(|(i, o)| {
            self.weights
                .as_ref()
                .iter()
                .zip(out[i..].iter_mut())
                .for_each(|(w, inp_grad)| *inp_grad += *w * *o);
        });

        out
    }

    #[inline]
    fn gradients_wrt_weights(&self, input: &[E; I], output_gradients: &[E; O]) -> C::Weights {
        let mut out = C::Weights::default();
        out.grad_iter_mut().enumerate().for_each(|(i, o)| {
            *o = input[i..]
                .iter()
                .zip(output_gradients.iter())
                .map(|(i, w)| *i * *w)
                .fold(E::default(), |a, x| a + x);
        });

        out
    }
}

/// Valid combinations of input dimension, filter size, and output dimension.
pub trait Conv1dKernel<E: Dtype + MatMulImpl, I: ConstDim, O: ConstDim> {
    type Weights: Gradients<Concrete = E> + AsRef<[E]> + Default;
}

impl<E: Dtype + MatMulImpl> Conv1dKernel<E, Const<4>, Const<2>> for Const<3> {
    type Weights = [E; 3];
}
impl<E: Dtype + MatMulImpl> Conv1dKernel<E, Const<5>, Const<3>> for Const<3> {
    type Weights = [E; 3];
}
impl<E: Dtype + MatMulImpl> Conv1dKernel<E, Const<8>, Const<6>> for Const<3> {
    type Weights = [E; 3];
}
impl<E: Dtype + MatMulImpl> Conv1dKernel<E, Const<12>, Const<10>> for Const<3> {
    type Weights = [E; 3];
}
impl<E: Dtype + MatMulImpl> Conv1dKernel<E, Const<16>, Const<14>> for Const<3> {
    type Weights = [E; 3];
}

impl<
        E: Dtype + MatMulImpl,
        const I: usize,
        const O: usize,
        C: Conv1dKernel<E, Const<I>, Const<O>>,
    > crate::BaseModule for Conv1d<E, I, O, C>
{
}

impl<
        E: Dtype + MatMulImpl,
        const I: usize,
        const O: usize,
        C: Conv1dKernel<E, Const<I>, Const<O>>,
    > crate::Module<[E; I]> for Conv1d<E, I, O, C>
{
    type Output = [E; O];

    fn forward(&mut self, x: &[E; I]) -> Result<Self::Output, crate::Error> {
        Ok(Conv1d::forward(self, x))
    }
}

impl<
        E: Dtype + MatMulImpl,
        const I: usize,
        const O: usize,
        C: Conv1dKernel<E, Const<I>, Const<O>>,
    > crate::RevModule<[E; I]> for Conv1d<E, I, O, C>
{
    type SelfGrads = C::Weights;

    fn reverse(&mut self, inputs: &[E; I], grads_wrt_output: &[E; O]) -> ([E; I], Self::SelfGrads) {
        (
            Conv1d::gradients_wrt_input(self, grads_wrt_output),
            Conv1d::gradients_wrt_weights(self, inputs, grads_wrt_output),
        )
    }

    fn apply(
        &mut self,
        applyer: &mut impl crate::optimizers::GradApplyer,
        updates: Self::SelfGrads,
    ) -> Result<(), crate::Error> {
        applyer.apply(updates, &mut self.weights)
    }
}

impl<
        E: Dtype + MatMulImpl,
        const I: usize,
        const O: usize,
        C: Conv1dKernel<E, Const<I>, Const<O>>,
    > crate::ResetParams for Conv1d<E, I, O, C>
{
    fn rand_params<RNG: rand::Rng>(
        &mut self,
        rng: &mut RNG,
        scale: f32,
    ) -> Result<(), crate::Error> {
        // Xavier/Glorot Initialization: initial values from a distribution with
        // zero mean and a variance of 2 / (inp + outp).
        // Can use either normal or uniform distribution, we use normal for now.
        let normal = rand_distr::Normal::new(0.0, 2.0 / (I as f32 + O as f32).sqrt()).unwrap();

        self.weights.grad_iter_mut().for_each(|w| {
            let s: f32 = rng.sample::<f32, _>(normal) * scale;
            *w = E::from_f32(s).unwrap();
        });
        Ok(())
    }
}

impl<
        E: Dtype + MatMulImpl,
        const I: usize,
        const O: usize,
        C: Conv1dKernel<E, Const<I>, Const<O>>,
    > crate::VisualizableUnit for Conv1d<E, I, O, C>
{
    const KIND: &'static str = "conv1d";
    type Params = C::Weights;
    fn params(&self) -> &Self::Params {
        &self.weights
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_forward() {
        let mut c1d = Conv1d::<f32, 4, 2, Const<3>>::default();
        c1d.weights = [1.0, 1.0, 1.0];
        assert_eq!(c1d.forward(&[1.0, 2.0, 4.0, 8.0]), [7.0, 14.0]);

        let mut c1d = Conv1d::<f32, 5, 3, Const<3>>::default();
        c1d.weights = [1.0, 2.0, 1.0];
        assert_eq!(c1d.forward(&[1.0, 2.0, 4.0, 8.0, 16.0]), [9.0, 18.0, 36.0]);
    }

    #[test]
    fn test_weight_grads() {
        let mut c1d = Conv1d::<f32, 4, 2, Const<3>>::default();
        c1d.weights = [1.0, 1.0, 1.0];
        assert_eq!(
            c1d.gradients_wrt_weights(&[1.0, 2.0, 4.0, 8.0], &[1.0, 1.0]),
            [3.0, 6.0, 12.0]
        );
    }

    #[test]
    fn test_weight_inputs() {
        let mut c1d = Conv1d::<f32, 4, 2, Const<3>>::default();
        c1d.weights = [1.0, 1.0, 1.0];
        assert_eq!(c1d.gradients_wrt_input(&[1.0, 1.0]), [1.0, 2.0, 2.0, 1.0]);

        let mut c1d = Conv1d::<f32, 5, 3, Const<3>>::default();
        c1d.weights = [1.0, 1.0, 1.0];
        assert_eq!(
            c1d.gradients_wrt_input(&[1.0, 1.0, 1.0]),
            [1.0, 2.0, 3.0, 2.0, 1.0]
        );
    }
}
