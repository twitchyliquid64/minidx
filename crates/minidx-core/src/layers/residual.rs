use crate::Dtype;

/// A residual connection around some module.
#[derive(Clone, Debug, Default)]
pub struct Residual<E: Dtype, const I: usize, M: Default + crate::Module<[E; I]>> {
    pub module: M,
    pub dt: std::marker::PhantomData<E>,
}

impl<E: Dtype, const I: usize, M: Default + crate::Module<[E; I], Output = [E; I]>>
    crate::Module<[E; I]> for Residual<E, I, M>
{
    type Output = M::Output;

    fn forward(&mut self, x: &[E; I]) -> Result<Self::Output, crate::Error> {
        let mut out = self.module.forward(x)?;
        out.iter_mut().zip(x).for_each(|(o, &x)| *o += x);
        Ok(out)
    }
}

impl<
        E: Dtype,
        const I: usize,
        M: Default + crate::Module<[E; I], Output = [E; I]> + crate::TracedModule<[E; I]>,
    > crate::TracedModule<[E; I]> for Residual<E, I, M>
{
    type Trace = M::Trace;

    fn traced_forward(
        &mut self,
        x: [E; I],
    ) -> Result<(<Self as crate::Module<[E; I]>>::Output, Self::Trace), crate::Error> {
        let (mut out, trace) = self.module.traced_forward(x)?;
        out.iter_mut().zip(x).for_each(|(o, x)| *o += x);
        Ok((out, trace))
    }
}

impl<
        E: Dtype,
        const I: usize,
        M: Default
            + crate::Module<[E; I], Output = [E; I]>
            + crate::TracedModule<[E; I]>
            + crate::BackpropModule<[E; I]>,
    > crate::BackpropModule<[E; I]> for Residual<E, I, M>
{
    type SelfGrads = M::SelfGrads;

    fn backprop(
        &mut self,
        trace: &<M as crate::TracedModule<[E; I]>>::Trace,
        grads_wrt_output: <M as crate::Module<[E; I]>>::Output,
    ) -> ([E; I], Self::SelfGrads) {
        let (mut out, mod_grads) = self.module.backprop(trace, grads_wrt_output.clone());
        out.iter_mut()
            .zip(grads_wrt_output.into_iter())
            .for_each(|(o, x)| *o += x);
        (out, mod_grads)
    }

    fn update(&mut self, updates: Self::SelfGrads) -> Result<(), crate::Error> {
        self.module.update(updates)
    }
}

impl<E: Dtype, const I: usize, M: Default + crate::Module<[E; I]> + crate::ResetParams>
    crate::ResetParams for Residual<E, I, M>
{
    fn rand_params<RNG: rand::Rng>(
        &mut self,
        rng: &mut RNG,
        scale: f32,
    ) -> Result<(), crate::Error> {
        self.module.rand_params(rng, scale)?;
        Ok(())
    }
}
