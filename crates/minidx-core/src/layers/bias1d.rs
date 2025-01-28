use crate::Dtype;

/// A learnable bias on each element.
#[derive(Clone, Debug)]
pub struct Bias1d<E: Dtype, const I: usize> {
    pub(crate) bias: [E; I],
}

impl<E: Dtype, const I: usize> Default for Bias1d<E, I> {
    fn default() -> Self {
        Self {
            bias: [E::default(); I],
        }
    }
}

impl<E: Dtype, const I: usize> Bias1d<E, I> {
    fn forward(&self, input: &[E; I]) -> [E; I] {
        let mut out: [E; I] = self.bias.clone();
        for (o, i) in out.iter_mut().zip(input.iter()) {
            *o += *i;
        }
        out
    }
}

impl<E: Dtype, const I: usize> crate::BaseModule for Bias1d<E, I> {}

impl<E: Dtype, const I: usize> crate::Module<[E; I]> for Bias1d<E, I> {
    type Output = [E; I];

    fn forward(&mut self, x: &[E; I]) -> Result<Self::Output, crate::Error> {
        Ok(Bias1d::forward(self, x))
    }
}

impl<E: Dtype, const I: usize> crate::RevModule<[E; I]> for Bias1d<E, I> {
    type SelfGrads = [E; I];

    fn reverse(
        &mut self,
        _inputs: &[E; I],
        grads_wrt_output: &[E; I],
    ) -> ([E; I], Self::SelfGrads) {
        (grads_wrt_output.clone(), grads_wrt_output.clone())
    }
}

impl<E: Dtype, const I: usize> crate::ResetParams for Bias1d<E, I> {
    fn rand_params<RNG: rand::Rng>(
        &mut self,
        rng: &mut RNG,
        scale: f32,
    ) -> Result<(), crate::Error> {
        self.bias.iter_mut().for_each(|b| {
            let s: f32 = rng.sample::<f32, _>(rand_distr::StandardNormal) * scale;
            *b = E::from_f32(s).unwrap();
        });
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_zero() {
        let layer = Bias1d::<f32, 1>::default();
        assert_eq!(layer.forward(&[1.0]), [1.0],);
    }

    #[test]
    fn test_2() {
        let layer = Bias1d::<f32, 2> { bias: [1.5, 3.2] };
        assert_eq!(layer.forward(&[1.0, 3.0]), [2.5, 6.2],);
    }
}
