use crate::Dtype;

/// A layer which learns a scalar scaling factor.
#[derive(Clone, Debug)]
pub struct ScalarScale<E: Dtype> {
    pub(crate) scale: E,
}

impl<E: Dtype> Default for ScalarScale<E> {
    fn default() -> Self {
        ScalarScale {
            scale: E::ONE.div(E::ONE + E::ONE),
        }
    }
}

impl<E: Dtype> ScalarScale<E> {
    #[inline]
    fn forward<const I: usize>(&self, input: &[E; I]) -> [E; I] {
        let mut out: [E; I] = [E::default(); I];

        out.iter_mut()
            .zip(input.iter())
            .for_each(|(o, i)| *o = *i * self.scale);

        out
    }

    #[inline]
    fn gradients_wrt_input<const I: usize>(&self, output_gradients: &[E; I]) -> [E; I] {
        let mut out: [E; I] = [E::default(); I];

        out.iter_mut()
            .zip(output_gradients.iter())
            .for_each(|(o, w)| *o = self.scale * *w);

        out
    }

    #[inline]
    fn gradients_wrt_weights<const I: usize>(
        &self,
        input: &[E; I],
        output_gradients: &[E; I],
    ) -> E {
        input
            .iter()
            .zip(output_gradients.iter())
            .fold(E::default(), |a, (i, w)| a + *i * *w)
    }
}

impl<E: Dtype> crate::BaseModule for ScalarScale<E> {}

impl<E: Dtype, const I: usize> crate::Module<[E; I]> for ScalarScale<E> {
    type Output = [E; I];

    fn forward(&self, x: &[E; I]) -> Result<Self::Output, crate::Error> {
        Ok(ScalarScale::forward(self, &x))
    }
}

impl<E: Dtype, const I: usize> crate::RevModule<[E; I]> for ScalarScale<E> {
    type SelfGrads = [E; 1];

    fn reverse(&self, inputs: &[E; I], grads_wrt_output: &[E; I]) -> ([E; I], Self::SelfGrads) {
        (
            ScalarScale::gradients_wrt_input(self, grads_wrt_output),
            [ScalarScale::gradients_wrt_weights(
                self,
                inputs,
                grads_wrt_output,
            )],
        )
    }

    fn apply(
        &mut self,
        applyer: &mut impl crate::optimizers::GradApplyer,
        updates: Self::SelfGrads,
    ) -> Result<(), crate::Error> {
        applyer.apply(updates, unsafe {
            // SAFETY: A value of N has the same memory layout as stacked unary arrays of N
            std::mem::transmute(&mut self.scale)
        })
    }
}

impl<E: Dtype> crate::ResetParams for ScalarScale<E> {
    fn rand_params<RNG: rand::Rng>(
        &mut self,
        rng: &mut RNG,
        scale: f32,
    ) -> Result<(), crate::Error> {
        // 0.5 initial value
        self.scale = E::ONE.div(E::ONE + E::ONE);
        Ok(())
    }
}

impl<E: Dtype> crate::VisualizableUnit for ScalarScale<E> {
    const KIND: &'static str = "scalar_scale";
    type Params = [[E; 1]; 1];
    fn params(&self) -> &Self::Params {
        // SAFETY: A value of N has the same memory layout as stacked unary arrays of N
        unsafe { std::mem::transmute(&self.scale) }
    }
}

impl<E: Dtype> crate::LoadableModule for ScalarScale<E> {
    fn save(
        &self,
        path: String,
        dict: &mut std::collections::HashMap<String, Vec<f64>>,
    ) -> Result<(), crate::LoadSaveError> {
        dict.insert(path, vec![self.scale.to_f64().unwrap()]);
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
        if params.len() != 1 {
            return Err(crate::LoadSaveError {
                path,
                err: format!(
                    "Parameters have wrong size: got {}, want {}",
                    params.len(),
                    1
                )
                .into(),
            });
        }
        self.scale = E::from_f64(params[0]).unwrap();
        Ok(())
    }
}
