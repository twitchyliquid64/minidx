use super::sigmoid;
use crate::Float;

/// The swish activation function with learnable beta.
#[derive(Clone, Debug)]
pub struct Swish<E: Float, const I: usize> {
    pub(crate) beta: [E; I],
}

impl<E: Float, const I: usize> Default for Swish<E, I> {
    fn default() -> Self {
        Self { beta: [E::ONE; I] }
    }
}

impl<E: Float, const I: usize> Swish<E, I> {
    #[inline]
    fn forward(&self, input: &[E; I]) -> [E; I] {
        let mut out: [E; I] = [E::default(); I];
        for ((o, x), b) in out.iter_mut().zip(input.iter()).zip(self.beta.iter()) {
            *o = *x * sigmoid(*b * *x);
        }
        out
    }

    #[inline]
    fn gradients_wrt_input(&self, input: &[E; I]) -> [E; I] {
        let mut out: [E; I] = [E::default(); I];
        for ((o, x), b) in out.iter_mut().zip(input.iter()).zip(self.beta.iter()) {
            let act = sigmoid(*b * *x);
            *o = act * (E::ONE + (*b * *x) * E::ONE.sub(act));
        }
        out
    }

    #[inline]
    fn gradients_wrt_beta(&self, input: &[E; I], output_gradients: &[E; I]) -> [E; I] {
        let mut out: [E; I] = [E::default(); I];
        for (((o, x), b), g) in out
            .iter_mut()
            .zip(input.iter())
            .zip(self.beta.iter())
            .zip(output_gradients.iter())
        {
            let act = sigmoid(*b * *x);
            *o = *g * (*x * *x) * act * E::ONE.sub(act);
        }
        out
    }
}

impl<E: Float, const I: usize> crate::BaseModule for Swish<E, I> {}

impl<E: Float, const I: usize> crate::Module<[E; I]> for Swish<E, I> {
    type Output = [E; I];

    fn forward(&self, x: &[E; I]) -> Result<Self::Output, crate::Error> {
        Ok(Swish::forward(self, x))
    }
}

impl<E: Float, const I: usize> crate::RevModule<[E; I]> for Swish<E, I> {
    type SelfGrads = [E; I];

    fn reverse(&self, inputs: &[E; I], grads_wrt_output: &[E; I]) -> ([E; I], Self::SelfGrads) {
        let mut output_grads = self.gradients_wrt_input(inputs);
        output_grads
            .iter_mut()
            .zip(grads_wrt_output)
            .for_each(|(ga, go)| *ga *= *go);

        (
            output_grads,
            self.gradients_wrt_beta(inputs, grads_wrt_output),
        )
    }

    fn apply(
        &mut self,
        applyer: &mut impl crate::optimizers::GradApplyer,
        updates: Self::SelfGrads,
    ) -> Result<(), crate::Error> {
        applyer.apply(updates, &mut self.beta)
    }
}

impl<E: Float, const I: usize> crate::ResetParams for Swish<E, I> {
    fn rand_params<RNG: rand::Rng>(
        &mut self,
        rng: &mut RNG,
        scale: f32,
    ) -> Result<(), crate::Error> {
        // Xavier/Glorot initialization vibes, but scaled down a bit
        // and centered about 1.
        let stddev = 1.0 / ((I * I) as f32 * 8.0).sqrt();
        let normal = rand_distr::Normal::new(1.0, stddev).unwrap();

        self.beta.iter_mut().for_each(|b| {
            let s: f32 = rng.sample::<f32, _>(normal) * scale;
            *b = E::from_f32(s).unwrap();
        });
        Ok(())
    }
}

impl<E: Float, const I: usize> crate::VisualizableUnit for Swish<E, I> {
    const KIND: &'static str = "swish";
    type Params = [[E; I]; 1];
    fn params(&self) -> &Self::Params {
        // SAFETY: An array of N is exactly the same as a unary array of the array of N
        unsafe { std::mem::transmute(&self.beta) }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default() {
        let layer = Swish::<f32, 2>::default();
        assert_eq!(layer.beta, [1.0, 1.0],);
    }

    #[test]
    fn test_forward() {
        let mut layer = Swish::<f32, 4>::default();
        layer.beta[2] = 355.0;
        let out = layer.forward(&[10.0, 0.0, 1.0, 1.0]);
        assert!(out[0] > 9.99 && out[0] < 10.0);
        assert_eq!(out[1], 0.0);
        assert_eq!(out[2], 1.0);
        assert!(out[3] > 0.72 && out[3] < 0.75);
    }

    #[test]
    fn test_gradients_wrt_input() {
        let mut layer = Swish::<f32, 1>::default();
        let out = layer.gradients_wrt_input(&[2.0]);
        assert!(out[0] > 1.08 && out[0] < 1.091);

        layer.beta[0] = 10.0;
        let out = layer.gradients_wrt_input(&[2.0]);
        assert!(out[0] > 0.9999 && out[0] < 1.0001);
    }

    #[test]
    fn test_gradients_wrt_beta() {
        let mut layer = Swish::<f32, 1>::default();
        let out = layer.gradients_wrt_beta(&[2.0], &[1.0]);
        assert!(out[0] > 0.409 && out[0] < 0.421);

        layer.beta[0] = 10.0;
        let out = layer.gradients_wrt_beta(&[-2.0], &[1.0]);
        assert!(out[0] > 8.0e-9 && out[0] < 8.26e-9);
    }
}
