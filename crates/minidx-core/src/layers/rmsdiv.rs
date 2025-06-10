use crate::matmul::MatMulImpl;
use crate::{Float, Shape};

/// A layer which divides each feature by the RMS of all features.
#[derive(Clone, Debug, Default)]
pub struct RMSDiv<E: Float + MatMulImpl, const I: usize> {
    pub(crate) marker: std::marker::PhantomData<[E; I]>,
}

impl<E: Float + MatMulImpl, const I: usize> RMSDiv<E, I> {
    #[inline]
    fn forward(&self, input: &[E; I]) -> [E; I] {
        let i = E::from_usize(I).unwrap();
        let rms: E = (input.iter().fold(E::default(), |acc, x| acc + (*x * *x)) / i).sqrt();

        let mut out: [E; I] = input.clone();
        for o in out.iter_mut() {
            *o /= rms;
        }
        out
    }

    #[inline]
    fn jacobian(&self, input: &[E; I]) -> [[E; I]; I] {
        let n = E::from_usize(I).unwrap();
        let rms: E = (input.iter().fold(E::default(), |acc, x| acc + (*x * *x))
            / E::from_usize(I).unwrap())
        .sqrt();

        let mut jacobian: [[E; I]; I] = [[E::default(); I]; I];
        for (i, iv) in input.iter().enumerate() {
            for (j, jv) in input.iter().enumerate() {
                jacobian[i][j] = if i == j {
                    (n * (rms * rms) - (*iv * *iv)) / (n * rms * rms * rms)
                } else {
                    -*jv * *iv / (n * rms * rms * rms)
                };
            }
        }
        jacobian
    }

    fn backward(&self, output_gradients: &[E; I], jacobian: &[[E; I]; I]) -> [E; I] {
        let mut out: [E; I] = [E::default(); I];

        let (m, k) = (1, I);
        let n = I;
        E::matmul(
            (m, k, n),
            true,
            output_gradients.as_ptr(),
            Shape::strides(&(1, I)),
            jacobian.as_ptr() as *const E,
            Shape::strides(&(I, I)),
            out.as_mut_ptr(),
            Shape::strides(&(1, I)),
        );

        out
    }
}

impl<E: Float + MatMulImpl, const I: usize> crate::BaseModule for RMSDiv<E, I> {}

impl<E: Float + MatMulImpl, const I: usize> crate::Module<[E; I]> for RMSDiv<E, I> {
    type Output = [E; I];

    fn forward(&self, x: &[E; I]) -> Result<Self::Output, crate::Error> {
        Ok(RMSDiv::forward(self, &x))
    }
}

impl<E: Float + MatMulImpl, const I: usize> crate::RevModule<[E; I]> for RMSDiv<E, I> {
    type SelfGrads = ();

    fn reverse(&self, inputs: &[E; I], grads_wrt_output: &[E; I]) -> ([E; I], Self::SelfGrads) {
        (
            RMSDiv::backward(self, grads_wrt_output, &RMSDiv::jacobian(self, inputs)),
            (),
        )
    }

    fn apply(
        &mut self,
        applyer: &mut impl crate::optimizers::GradApplyer,
        updates: Self::SelfGrads,
    ) -> Result<(), crate::Error> {
        Ok(())
    }
}

impl<E: Float + MatMulImpl, const I: usize> crate::ResetParams for RMSDiv<E, I> {
    fn rand_params<RNG: rand::Rng>(
        &mut self,
        rng: &mut RNG,
        scale: f32,
    ) -> Result<(), crate::Error> {
        Ok(())
    }
}

impl<E: Float + MatMulImpl, const I: usize> crate::VisualizableUnit for RMSDiv<E, I> {
    const KIND: &'static str = "rmsdiv";
    type Params = ();
    fn params(&self) -> &Self::Params {
        &()
    }
}

impl<E: Float + MatMulImpl, const I: usize> crate::LoadableModule for RMSDiv<E, I> {
    fn save(
        &self,
        path: String,
        dict: &mut std::collections::HashMap<String, Vec<f64>>,
    ) -> Result<(), crate::LoadSaveError> {
        Ok(())
    }

    fn load(
        &mut self,
        path: String,
        dict: &std::collections::HashMap<String, Vec<f64>>,
    ) -> Result<(), crate::LoadSaveError> {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_forward() {
        let layer = RMSDiv::<f32, 2>::default();
        assert_eq!(layer.forward(&[2.0, 2.0]), [1.0, 1.0],);
    }

    #[test]
    fn test_backward() {
        let layer = RMSDiv::<f32, 2>::default();
        assert_eq!(
            layer.backward(&[1.0, 1.0], &layer.jacobian(&[2.0, 2.0])),
            [0.0, 0.0],
        );
        let grads = layer.backward(&[1.0, 1.0], &layer.jacobian(&[4.0, 8.0]));
        assert!(grads[0] > 0.062, "[0]: {:?}", grads[0]);
        assert!(grads[0] < 0.0634, "[0]: {:?}", grads[0]);
        assert!(grads[1] > -0.0317, "[1]: {:?}", grads[1]);
        assert!(grads[1] < -0.0315, "[1]: {:?}", grads[1]);
    }
}
