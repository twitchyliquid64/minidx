use crate::gradients::{ClassBias, ClassWrapper, Gradients};
use crate::Dtype;

/// A learnable bias on each element.
#[derive(Clone, Debug)]
pub struct Bias1d<E: Dtype, const I: usize> {
    pub(crate) bias: ClassWrapper<[E; I], ClassBias>,
}

impl<E: Dtype, const I: usize> Default for Bias1d<E, I> {
    fn default() -> Self {
        Self {
            bias: ClassWrapper::<[E; I], ClassBias>::wrap([E::default(); I]),
        }
    }
}

impl<E: Dtype, const I: usize> Bias1d<E, I> {
    fn forward(&self, input: &[E; I]) -> [E; I] {
        let mut out: [E; I] = self.bias.raw_grads_ref().clone();
        for (o, i) in out.iter_mut().zip(input.iter()) {
            *o += *i;
        }
        out
    }
}

impl<E: Dtype, const I: usize> crate::BaseModule for Bias1d<E, I> {}

impl<E: Dtype, const I: usize> crate::Module<[E; I]> for Bias1d<E, I> {
    type Output = [E; I];

    fn forward(&self, x: &[E; I]) -> Result<Self::Output, crate::Error> {
        Ok(Bias1d::forward(self, x))
    }
}

impl<E: Dtype, const I: usize> crate::RevModule<[E; I]> for Bias1d<E, I> {
    type SelfGrads = ClassWrapper<[E; I], ClassBias>;

    fn reverse(&self, _inputs: &[E; I], grads_wrt_output: &[E; I]) -> ([E; I], Self::SelfGrads) {
        (
            grads_wrt_output.clone(),
            Self::SelfGrads::wrap(grads_wrt_output.clone()),
        )
    }

    fn apply(
        &mut self,
        applyer: &mut impl crate::optimizers::GradApplyer,
        updates: Self::SelfGrads,
    ) -> Result<(), crate::Error> {
        applyer.apply(updates, &mut self.bias)
    }
}

impl<E: Dtype, const I: usize> crate::ResetParams for Bias1d<E, I> {
    fn rand_params<RNG: rand::Rng>(
        &mut self,
        rng: &mut RNG,
        scale: f32,
    ) -> Result<(), crate::Error> {
        // Xavier/Glorot initialization vibes, but scaled down a ton for bias initialization.
        // (Unlike dense layers, biases are still learned from zero parameters)
        let stddev = 1.0 / ((I * I) as f32 * 64.0).sqrt();
        let normal = rand_distr::Normal::new(0.0, stddev).unwrap();

        self.bias.grad_iter_mut().for_each(|b| {
            let s: f32 = rng.sample::<f32, _>(normal) * scale;
            *b = E::from_f32(s).unwrap();
        });
        Ok(())
    }
}

impl<E: Dtype, const I: usize> crate::VisualizableUnit for Bias1d<E, I> {
    const KIND: &'static str = "bias1d";
    type Params = [[E; I]; 1];
    fn params(&self) -> &Self::Params {
        // SAFETY: An array of N is exactly the same as a unary array of the array of N
        unsafe { std::mem::transmute(self.bias.raw_grads_ref()) }
    }
}

impl<E: Dtype, const I: usize> crate::LoadableModule for Bias1d<E, I> {
    fn save(
        &self,
        path: String,
        dict: &mut std::collections::HashMap<String, Vec<f64>>,
    ) -> Result<(), crate::LoadSaveError> {
        dict.insert(
            path,
            self.bias.grad_iter().map(|f| f.to_f64().unwrap()).collect(),
        );
        Ok(())
    }

    fn load(
        &mut self,
        path: String,
        dict: &std::collections::HashMap<String, Vec<f64>>,
    ) -> Result<(), crate::LoadSaveError> {
        let params = dict.get(&path).ok_or(crate::LoadSaveError {
            path: path.clone(),
            err: "Parameters missing".into(),
        })?;
        if params.len() != I {
            return Err(crate::LoadSaveError {
                path,
                err: format!(
                    "Parameters have wrong size: got {}, want {}",
                    params.len(),
                    I
                )
                .into(),
            });
        }
        for (a, b) in self.bias.grad_iter_mut().zip(params.into_iter()) {
            *a = E::from_f64(*b).unwrap();
        }
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
        let layer = Bias1d::<f32, 2> {
            bias: ClassWrapper::<[f32; 2], ClassBias>::wrap([1.5, 3.2]),
        };
        assert_eq!(layer.forward(&[1.0, 3.0]), [2.5, 6.2],);
    }
}
