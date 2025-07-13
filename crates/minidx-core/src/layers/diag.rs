use crate::Dtype;

/// A layer which learns a per-channel scaling factor.
///
/// Called Diag as it resembles a fully-connected weight matrix except
/// the off-diagonals of the matrix are 0.
#[derive(Clone, Debug)]
pub struct Diag<E: Dtype, const I: usize> {
    pub(crate) weights: [E; I],
}

impl<E: Dtype, const I: usize> Default for Diag<E, I> {
    fn default() -> Self {
        Diag {
            weights: [E::default(); I],
        }
    }
}

impl<E: Dtype, const I: usize> Diag<E, I> {
    #[inline]
    fn forward(&self, input: &[E; I]) -> [E; I] {
        let mut out: [E; I] = [E::default(); I];

        out.iter_mut()
            .zip(input.iter())
            .zip(self.weights.iter())
            .for_each(|((o, i), w)| *o = *i * *w);

        out
    }

    #[inline]
    fn gradients_wrt_input(&self, output_gradients: &[E; I]) -> [E; I] {
        let mut out: [E; I] = [E::default(); I];

        out.iter_mut()
            .zip(output_gradients.iter())
            .zip(self.weights.iter())
            .for_each(|((o, i), w)| *o = *i * *w);

        out
    }

    #[inline]
    fn gradients_wrt_weights(&self, input: &[E; I], output_gradients: &[E; I]) -> [E; I] {
        let mut out: [E; I] = [E::default(); I];

        out.iter_mut()
            .zip(input.iter())
            .zip(output_gradients.iter())
            .for_each(|((o, i), w)| *o = *i * *w);

        out
    }
}

impl<E: Dtype, const I: usize> crate::BaseModule for Diag<E, I> {}

impl<E: Dtype, const I: usize> crate::Module<[E; I]> for Diag<E, I> {
    type Output = [E; I];

    fn forward(&self, x: &[E; I]) -> Result<Self::Output, crate::Error> {
        Ok(Diag::forward(self, &x))
    }
}

impl<E: Dtype, const I: usize> crate::RevModule<[E; I]> for Diag<E, I> {
    type SelfGrads = [E; I];

    fn reverse(&self, inputs: &[E; I], grads_wrt_output: &[E; I]) -> ([E; I], Self::SelfGrads) {
        (
            Diag::gradients_wrt_input(self, grads_wrt_output),
            Diag::gradients_wrt_weights(self, inputs, grads_wrt_output),
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

impl<E: Dtype, const I: usize> crate::ResetParams for Diag<E, I> {
    fn rand_params<RNG: rand::Rng>(
        &mut self,
        _rng: &mut RNG,
        _scale: f32,
    ) -> Result<(), crate::Error> {
        self.weights = [E::ONE; I];
        Ok(())
    }
}

impl<E: Dtype, const I: usize> crate::VisualizableUnit for Diag<E, I> {
    const KIND: &'static str = "diag";
    type Params = [[E; I]; 1];
    fn params(&self) -> &Self::Params {
        // SAFETY: An array of N is exactly the same as a unary array of the array of N
        unsafe { std::mem::transmute(&self.weights) }
    }
}

impl<E: Dtype, const I: usize> crate::LoadableModule for Diag<E, I> {
    fn save(
        &self,
        path: String,
        dict: &mut std::collections::HashMap<String, Vec<f64>>,
    ) -> Result<(), crate::LoadSaveError> {
        dict.insert(
            path,
            self.weights.iter().map(|f| f.to_f64().unwrap()).collect(),
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
        for (a, w) in self.weights.iter_mut().zip(params.into_iter()) {
            *a = E::from_f64(*w).unwrap();
        }
        Ok(())
    }
}
