use crate::{Dtype, Gradients};

/// A wrapper which locally adjusts the learning rate.
#[derive(Clone, Debug, Default)]
pub struct LR<E: Dtype, const I: usize, M: Default + crate::Module<[E; I]>> {
    pub module: M,
    pub update_multiplier: f32,
    pub dt: std::marker::PhantomData<E>,
}

impl<E: Dtype, const I: usize, M: Default + crate::Module<[E; I]>> crate::Module<[E; I]>
    for LR<E, I, M>
{
    type Output = M::Output;

    fn forward(&self, x: &[E; I]) -> Result<Self::Output, crate::Error> {
        self.module.forward(x)
    }
}

impl<
        E: Dtype,
        const I: usize,
        M: Default + crate::Module<[E; I]> + crate::TracedModule<[E; I]>,
    > crate::TracedModule<[E; I]> for LR<E, I, M>
{
    type Trace = M::Trace;

    fn traced_forward(
        &self,
        x: [E; I],
    ) -> Result<(<Self as crate::Module<[E; I]>>::Output, Self::Trace), crate::Error> {
        self.module.traced_forward(x)
    }
}

impl<
        E: Dtype,
        const I: usize,
        M: Default
            + crate::Module<[E; I]>
            + crate::TracedModule<[E; I]>
            + crate::BackpropModule<[E; I]>,
    > crate::BackpropModule<[E; I]> for LR<E, I, M>
where
    M::SelfGrads: Gradients,
{
    type SelfGrads = M::SelfGrads;

    fn backprop(
        &self,
        trace: &<M as crate::TracedModule<[E; I]>>::Trace,
        grads_wrt_output: <M as crate::Module<[E; I]>>::Output,
    ) -> ([E; I], Self::SelfGrads) {
        self.module.backprop(trace, grads_wrt_output)
    }

    fn update(
        &mut self,
        applyer: &mut impl crate::optimizers::GradApplyer,
        mut updates: Self::SelfGrads,
    ) -> Result<(), crate::Error> {
        use num_traits::FromPrimitive;
        let m = <Self::SelfGrads as Gradients>::Concrete::from_f32(self.update_multiplier).unwrap();
        updates.grad_iter_mut().for_each(|u| *u *= m);

        self.module.update(applyer, updates)
    }
}

impl<E: Dtype, const I: usize, M: Default + crate::Module<[E; I]> + crate::ResetParams>
    crate::ResetParams for LR<E, I, M>
{
    fn rand_params<RNG: rand::Rng>(
        &mut self,
        rng: &mut RNG,
        scale: f32,
    ) -> Result<(), crate::Error> {
        self.module.rand_params(rng, scale)
    }
}

impl<E: Dtype, const I: usize, M: Default + crate::Module<[E; I]> + crate::VisualizableUnit>
    crate::VisualizableUnit for LR<E, I, M>
{
    const KIND: &'static str = M::KIND;
    type Params = M::Params;
    fn params(&self) -> &Self::Params {
        self.module.params()
    }
}
