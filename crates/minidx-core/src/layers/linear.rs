use crate::matmul::MatMulImpl;
use crate::{Dim, Dtype, Shape};

/// A fully-connected layer with a fixed number of inputs and outputs. No bias.
#[derive(Clone, Debug)]
pub struct Dense<E: Dtype + MatMulImpl, const I: usize, const O: usize> {
    pub(crate) weights: [[E; I]; O],
}

impl<E: Dtype + MatMulImpl, const I: usize, const O: usize> Default for Dense<E, I, O> {
    fn default() -> Self {
        Dense {
            weights: [[E::default(); I]; O],
        }
    }
}

impl<E: Dtype + MatMulImpl, const I: usize, const O: usize> Dense<E, I, O> {
    #[inline]
    fn forward(&self, input: &[E; I]) -> [E; O] {
        let mut out: [E; O] = [E::default(); O];

        let (m, k) = (1, I);
        let n = O;
        E::matmul(
            (m, k, n),
            true, // I think this needs to be false?
            input.as_ptr(),
            Shape::strides(&(1, I)),
            self.weights.as_ptr() as *const E,
            Shape::strides(&(I, O)),
            out.as_mut_ptr(),
            Shape::strides(&(1, O)),
        );

        out
    }

    #[inline]
    fn gradients_wrt_input(&self, output_gradients: &[E; O]) -> [E; I] {
        let mut out: [E; I] = [E::default(); I];

        let (m, k) = (1, I);
        let n = O;
        let strides = (m, n).strides();
        E::matmul(
            (m, n, k),
            true,
            output_gradients.as_ptr(),
            strides,
            self.weights.as_ptr() as *const E,
            Shape::strides(&(O, I)),
            out.as_mut_ptr(),
            Shape::strides(&(1, I)),
        );
        out
    }

    #[inline]
    fn gradients_wrt_weights(&self, input: &[E; I], output_gradients: &[E; O]) -> [[E; I]; O] {
        let mut out: [[E; I]; O] = [[E::default(); I]; O];

        let (m, k) = (1, I);
        let n = O;
        let strides = (m, n).strides();
        E::matmul(
            (k, m, n),
            false, // Just flipped this to false and it still works?
            input.as_ptr(),
            Shape::strides(&(I, 1)),
            output_gradients.as_ptr(),
            strides,
            out.as_mut_ptr() as *mut E,
            Shape::strides(&(I, O)),
        );
        out
    }
}

impl<E: Dtype + MatMulImpl, const I: usize, const O: usize> crate::BaseModule for Dense<E, I, O> {}

impl<E: Dtype + MatMulImpl, const I: usize, const O: usize> crate::Module<[E; I]>
    for Dense<E, I, O>
{
    type Output = [E; O];

    fn forward(&mut self, x: &[E; I]) -> Result<Self::Output, crate::Error> {
        Ok(Dense::forward(self, &x))
    }
}

impl<E: Dtype + MatMulImpl, const I: usize, const O: usize> crate::RevModule<[E; I]>
    for Dense<E, I, O>
{
    type SelfGrads = [[E; I]; O];

    fn reverse(&mut self, inputs: &[E; I], grads_wrt_output: &[E; O]) -> ([E; I], Self::SelfGrads) {
        (
            Dense::gradients_wrt_input(self, grads_wrt_output),
            Dense::gradients_wrt_weights(self, inputs, grads_wrt_output),
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

impl<E: Dtype + MatMulImpl, const I: usize, const O: usize> crate::ResetParams for Dense<E, I, O> {
    fn rand_params<RNG: rand::Rng>(
        &mut self,
        rng: &mut RNG,
        scale: f32,
    ) -> Result<(), crate::Error> {
        // Xavier/Glorot Initialization: initial values from a distribution with
        // zero mean and a variance of 2 / (inp + outp).
        // Can use either normal or uniform distribution, we use normal for now.
        let normal = rand_distr::Normal::new(0.0, 2.0 / (I as f32 + O as f32).sqrt()).unwrap();

        self.weights.iter_mut().for_each(|r| {
            r.iter_mut().for_each(|w| {
                let s: f32 = rng.sample::<f32, _>(normal) * scale;
                *w = E::from_f32(s).unwrap();
            })
        });
        Ok(())
    }
}

impl<E: Dtype + MatMulImpl, const I: usize, const O: usize> crate::VisualizableUnit
    for Dense<E, I, O>
{
    const KIND: &'static str = "dense";
    type Params = [[E; I]; O];
    fn params(&self) -> &Self::Params {
        &self.weights
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_zero_weights() {
        let layer = Dense::<f32, 1, 2>::default();
        assert_eq!(layer.forward(&[1.0]), [0.0, 0.0],);
    }

    #[test]
    fn test_1_2() {
        let layer = Dense::<f32, 1, 2> {
            weights: [[0.5], [1.0]],
        };
        assert_eq!(layer.forward(&[1.0]), [0.5, 1.0],);
        assert_eq!(layer.gradients_wrt_input(&[2.0, 0.0]), [1.0],);

        assert_eq!(
            layer.gradients_wrt_weights(&[1.0], &[2.0, 0.0]),
            [[2.0], [0.0]],
        );
        assert_eq!(
            layer.gradients_wrt_weights(&[1.0], &[0.0, 2.0]),
            [[0.0], [2.0]],
        );
        assert_eq!(
            layer.gradients_wrt_weights(&[1.0], &[1.0, 2.0]),
            [[1.0], [2.0]],
        );
    }

    #[test]
    fn test_2_1() {
        let layer = Dense::<f32, 2, 1> {
            weights: [[0.05, 0.5]],
        };
        assert_eq!(layer.forward(&[1.0, 2.0]), [1.05],);
        assert_eq!(layer.gradients_wrt_input(&[2.0]), [0.1, 1.0],);
    }

    #[test]
    fn test_2_2() {
        let layer = Dense::<f32, 2, 2> {
            weights: [[0.1, 0.4], [0.5, 0.2]],
        };
        assert_eq!(layer.forward(&[1.0, 2.0]), [1.1, 0.8],);
        assert_eq!(layer.gradients_wrt_input(&[0.0, 1.0]), [0.5, 0.2],);
    }

    #[test]
    fn grad_wrt_input() {
        let layer = Dense::<f32, 2, 2> {
            weights: [[0.0, 1.0], [1.0, 0.0]],
        };
        assert_eq!(layer.forward(&[1.0, 0.0]), [0.0, 1.0],);
        assert_eq!(layer.gradients_wrt_input(&[1.0, -1.0]), [-1.0, 1.0],);
    }
}
