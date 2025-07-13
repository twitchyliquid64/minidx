use crate::Float;

/// A softmax activation layer with no trainable parameters.
#[derive(Clone, Debug)]
pub struct Softmax(pub f32);

impl Default for Softmax {
    fn default() -> Self {
        Self(1.0)
    }
}

impl Softmax {
    #[inline]
    fn forward<E: Float, const I: usize>(&self, input: &[E; I]) -> [E; I] {
        let t = E::from_f32(self.0).unwrap();
        let mut out: [E; I] = [E::default(); I];

        let max_val = input.iter().fold(E::NEG_INFINITY, |l, r| E::max(l, *r));

        // Compute exponential of difference between x and max value.
        out.iter_mut().zip(input.iter()).for_each(|(o, &x)| {
            *o = ((x - max_val) / t).exp();
        });

        // Normalize
        let sum_exp = out.iter().fold(E::default(), |a, x| a + *x);
        out.iter_mut().for_each(|o| *o /= sum_exp);

        out
    }

    #[inline]
    fn backprop<E: Float, const I: usize>(
        &self,
        input: &[E; I],
        grads_wrt_output: &[E; I],
    ) -> [E; I] {
        let t = E::from_f32(self.0).unwrap();
        let output = self.forward(input);

        let mut out: [E; I] = [E::default(); I];
        out.iter_mut().enumerate().for_each(|(i, o)| {
            let mut sum = E::default();
            for j in 0..I {
                let kronecker = if i == j { E::ONE } else { E::default() };
                sum += (kronecker - output[j]) * grads_wrt_output[j];
            }
            *o = output[i] * sum / t;
        });

        out
    }
}

impl crate::BaseModule for Softmax {}

impl<E: Float, const I: usize> crate::Module<[E; I]> for Softmax {
    type Output = [E; I];

    fn forward(&self, x: &[E; I]) -> Result<Self::Output, crate::Error> {
        Ok(Softmax::forward(self, x))
    }
}

impl<E: Float, const I: usize> crate::RevModule<[E; I]> for Softmax {
    type SelfGrads = ();

    fn reverse(&self, inputs: &[E; I], grads_wrt_output: &[E; I]) -> ([E; I], Self::SelfGrads) {
        (self.backprop(inputs, grads_wrt_output), ())
    }

    fn apply(
        &mut self,
        _applyer: &mut impl crate::optimizers::GradApplyer,
        _updates: Self::SelfGrads,
    ) -> Result<(), crate::Error> {
        Ok(())
    }
}

impl crate::LoadableModule for Softmax {
    fn save(
        &self,
        _path: String,
        _dict: &mut std::collections::HashMap<String, Vec<f64>>,
    ) -> Result<(), crate::LoadSaveError> {
        Ok(())
    }

    fn load(
        &mut self,
        _path: String,
        _dict: &std::collections::HashMap<String, Vec<f64>>,
    ) -> Result<(), crate::LoadSaveError> {
        Ok(())
    }
}

impl crate::ResetParams for Softmax {
    fn rand_params<RNG: rand::Rng>(
        &mut self,
        _rng: &mut RNG,
        _scale: f32,
    ) -> Result<(), crate::Error> {
        Ok(())
    }
}

impl crate::VisualizableUnit for Softmax {
    const KIND: &'static str = "softmax";
    type Params = ();
    fn params(&self) -> &Self::Params {
        &()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_softmax_zeros() {
        assert_eq!([0.5, 0.5f32], (Softmax::default()).forward(&[0.0, 0.0f32]));
        assert_eq!(
            [0.25, 0.25, 0.25, 0.25f32],
            (Softmax::default()).forward(&[0.0, 0.0, 0.0, 0.0f32])
        );
    }

    #[test]
    fn test_softmax() {
        let [l, r] = (Softmax::default()).forward(&[0.01, 1.0f32]);
        assert!(l < r);
        assert!(l < r / 2.0);
        assert!(l > r / 100.0);
    }

    #[test]
    fn test_softmax_backprop() {
        let [lg, rg] = (Softmax::default()).backprop(&[0.01, 1.0f32], &[1.0, -1.0f32]);
        assert!(lg > rg);
        // TODO: Needs moar checking
    }
}
