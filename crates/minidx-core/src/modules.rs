use crate::{Error, Gradients};

/// A unit of computation that consumes `Input` and produces [Module::Output].
pub trait Module<X> {
    /// The type that this unit produces given `Input`.
    type Output;

    fn forward(&mut self, x: &X) -> Result<Self::Output, Error>;
}

/// A unit of computation which can do backpropagation without knowledge of any
/// additional state.
///
/// This trait is to be implemented by individual network layers, but not compositions of them.
pub trait RevModule<X>: Module<X> {
    /// The type describing gradients with respect to the modules' own parameters.
    type SelfGrads: Gradients;

    /// Returns the gradients with respect to the input, and the
    /// gradients with respect to any internal parameters.
    fn reverse(
        &mut self,
        inputs: &X,
        grads_wrt_output: &<Self as Module<X>>::Output,
    ) -> (X, Self::SelfGrads);

    /// Applies a gradient update step: adding product of the provided gradients and
    /// the scalar to the parameters.
    fn apply(&mut self, updates: Self::SelfGrads, scalar: f32) -> Result<(), Error>;
}

/// Some sequential computation that consumes `Input` and produces [Module::Output],
/// but also produces artifacts describing the execution that can later be used
/// during backprop.
///
/// A supertrait of [Module].
pub trait TracedModule<X>: Module<X> {
    /// The type that this unit produces to describe intermediate state.
    ///
    /// This is typically the input to the module.
    type Trace;

    /// Same as [Module::forward], except intermediate computations that are needed
    /// for backprop are returned.
    fn traced_forward(&mut self, x: X)
        -> Result<(<Self as Module<X>>::Output, Self::Trace), Error>;
}

impl<Input, M: Module<Input> + BaseModule> TracedModule<Input> for M {
    type Trace = Input;

    fn traced_forward(
        &mut self,
        x: Input,
    ) -> Result<(<Self as crate::Module<Input>>::Output, Self::Trace), Error> {
        Ok((Module::forward(self, &x)?, (x)))
    }
}

/// Some sequential computation which can perform backprop on itself given trace state
/// and the gradients of its outputs: computing parameter updates for itself.
///
/// Relies on behavior from the [TracedModule] trait.
pub trait BackpropModule<X>: TracedModule<X> {
    /// Type describing movement in the modules' own parameters in response to backpropagation.
    type SelfGrads;

    fn backprop(
        &mut self,
        trace: &<Self as TracedModule<X>>::Trace,
        grads_wrt_output: <Self as Module<X>>::Output,
    ) -> (X, Self::SelfGrads);

    /// Applies a gradient update step: adding product of the provided gradients and
    /// the scalar to the parameters.
    fn update(&mut self, updates: Self::SelfGrads, scalar: f32) -> Result<(), Error>;
}

impl<Input, M: TracedModule<Input, Trace = Input> + RevModule<Input> + BaseModule>
    BackpropModule<Input> for M
{
    type SelfGrads = <M as RevModule<Input>>::SelfGrads;

    fn backprop(
        &mut self,
        trace: &<M as TracedModule<Input>>::Trace,
        grads_wrt_output: <M as Module<Input>>::Output,
    ) -> (Input, Self::SelfGrads) {
        M::reverse(self, trace, &grads_wrt_output)
    }

    fn update(&mut self, updates: Self::SelfGrads, scalar: f32) -> Result<(), Error> {
        M::apply(self, updates, scalar)
    }
}

/// Marker trait for low-level layers which are composable modules.
pub(crate) trait BaseModule {}

/// Something that can have its learnable parameters reset.
pub trait ResetParams {
    fn rand_params<RNG: rand::Rng>(&mut self, rng: &mut RNG, scale: f32) -> Result<(), Error>;
}

macro_rules! fwd_tuple_impls {
    ([$($name:ident),+] [$($idx:tt),*], $last:ident, [$($rev_tail:ident),*], [$($trace_name:ident),*]) => {
        impl<
            Input,
            $last:
            $(crate::Module::<$rev_tail ::Output>, $rev_tail: )*
            crate::Module<Input>
        > crate::Module<Input> for ($($name,)+) {
            type Output = $last ::Output;

            /// Calls forward sequentially on each module in the tuple.
            fn forward(&mut self, x: &Input) -> Result<Self::Output, Error> {
                let x = self.0.forward(x)?;
                $(let x = self.$idx.forward(&x)?;)*
                Ok(x)
            }
        }

        impl<Input,
             $last:
             $(TracedModule::<$rev_tail ::Output>, $rev_tail: )*
             TracedModule<Input>
            > TracedModule<Input> for ($($name,)+)
        {
            type Trace = ($($name::Trace,)+);

            fn traced_forward(
                &mut self,
                x: Input,
            ) -> Result<(<Self as Module<Input>>::Output, Self::Trace), Error> {
                let (x, m1t) = self.0.traced_forward(x)?;
                $(let (x, $trace_name) = self.$idx.traced_forward(x)?;)*
                Ok((x, (m1t, $($trace_name,)*)))
            }
        }

        impl<
            $last:
            $(crate::ResetParams, $rev_tail: )*
            crate::ResetParams
        > crate::ResetParams for ($($name,)+) {
            fn rand_params<RNG: rand::Rng>(&mut self, rng: &mut RNG, scale: f32) -> Result<(), Error> {
                self.0.rand_params(rng, scale)?;
                $(self.$idx.rand_params(rng, scale)?;)*
                Ok(())
            }
        }
    };
}

fwd_tuple_impls!([M1][], M1, [], []);
fwd_tuple_impls!([M1, M2][1], M2, [M1], [m2t]);
fwd_tuple_impls!([M1, M2, M3] [1, 2], M3, [M2, M1], [m2t, m3t]);
fwd_tuple_impls!([M1, M2, M3, M4] [1, 2, 3], M4, [M3, M2, M1], [m2t, m3t, m4t]);
fwd_tuple_impls!([M1, M2, M3, M4, M5] [1, 2, 3, 4], M5, [M4, M3, M2, M1], [m2t, m3t, m4t, m5t]);
fwd_tuple_impls!([M1, M2, M3, M4, M5, M6] [1, 2, 3, 4, 5], M6, [M5, M4, M3, M2, M1], [m2t, m3t, m4t, m5t, m6t]);
fwd_tuple_impls!([M1, M2, M3, M4, M5, M6, M7] [1, 2, 3, 4, 5, 6], M7, [M6, M5, M4, M3, M2, M1], [m2t, m3t, m4t, m5t, m6t, m7t]);

// TODO: Turn BackpropModule impls into a macro

impl<Input, M: BackpropModule<Input, SelfGrads = <M as TracedModule<Input>>::Trace>>
    BackpropModule<Input> for (M,)
{
    type SelfGrads = (<M as BackpropModule<Input>>::SelfGrads,);

    fn backprop(
        &mut self,
        trace: &<Self as TracedModule<Input>>::Trace,
        grads_wrt_output: <Self as Module<Input>>::Output,
    ) -> (Input, Self::SelfGrads) {
        let (next_grads, updates) = self.0.backprop(&trace.0, grads_wrt_output);
        (next_grads, (updates,))
    }

    fn update(&mut self, updates: Self::SelfGrads, scalar: f32) -> Result<(), Error> {
        self.0.update(updates.0, scalar)?;
        Ok(())
    }
}

impl<Input, M1: BackpropModule<Input>, M2: BackpropModule<M1::Output>> BackpropModule<Input>
    for (M1, M2)
{
    type SelfGrads = (
        <M1 as BackpropModule<Input>>::SelfGrads,
        <M2 as BackpropModule<M1::Output>>::SelfGrads,
    );

    fn backprop(
        &mut self,
        trace: &<Self as TracedModule<Input>>::Trace,
        grads_wrt_output: <Self as Module<Input>>::Output,
    ) -> (Input, Self::SelfGrads) {
        let (next_grads, u2) = self.1.backprop(&trace.1, grads_wrt_output);
        let (next_grads, u1) = self.0.backprop(&trace.0, next_grads);
        (next_grads, (u1, u2))
    }

    fn update(&mut self, updates: Self::SelfGrads, scalar: f32) -> Result<(), Error> {
        self.0.update(updates.0, scalar)?;
        self.1.update(updates.1, scalar)?;
        Ok(())
    }
}

impl<
        Input,
        M1: BackpropModule<Input>,
        M2: BackpropModule<M1::Output>,
        M3: BackpropModule<M2::Output>,
    > BackpropModule<Input> for (M1, M2, M3)
{
    type SelfGrads = (
        <M1 as BackpropModule<Input>>::SelfGrads,
        <M2 as BackpropModule<M1::Output>>::SelfGrads,
        <M3 as BackpropModule<M2::Output>>::SelfGrads,
    );

    fn backprop(
        &mut self,
        trace: &<Self as TracedModule<Input>>::Trace,
        grads_wrt_output: <Self as Module<Input>>::Output,
    ) -> (Input, Self::SelfGrads) {
        let (next_grads, u3) = self.2.backprop(&trace.2, grads_wrt_output);
        let (next_grads, u2) = self.1.backprop(&trace.1, next_grads);
        let (next_grads, u1) = self.0.backprop(&trace.0, next_grads);
        (next_grads, (u1, u2, u3))
    }

    fn update(&mut self, updates: Self::SelfGrads, scalar: f32) -> Result<(), Error> {
        self.0.update(updates.0, scalar)?;
        self.1.update(updates.1, scalar)?;
        self.2.update(updates.2, scalar)?;
        Ok(())
    }
}

impl<
        Input,
        M1: BackpropModule<Input>,
        M2: BackpropModule<M1::Output>,
        M3: BackpropModule<M2::Output>,
        M4: BackpropModule<M3::Output>,
    > BackpropModule<Input> for (M1, M2, M3, M4)
{
    type SelfGrads = (
        <M1 as BackpropModule<Input>>::SelfGrads,
        <M2 as BackpropModule<M1::Output>>::SelfGrads,
        <M3 as BackpropModule<M2::Output>>::SelfGrads,
        <M4 as BackpropModule<M3::Output>>::SelfGrads,
    );

    fn backprop(
        &mut self,
        trace: &<Self as TracedModule<Input>>::Trace,
        grads_wrt_output: <Self as Module<Input>>::Output,
    ) -> (Input, Self::SelfGrads) {
        let (next_grads, u4) = self.3.backprop(&trace.3, grads_wrt_output);
        let (next_grads, u3) = self.2.backprop(&trace.2, next_grads);
        let (next_grads, u2) = self.1.backprop(&trace.1, next_grads);
        let (next_grads, u1) = self.0.backprop(&trace.0, next_grads);
        (next_grads, (u1, u2, u3, u4))
    }

    fn update(&mut self, updates: Self::SelfGrads, scalar: f32) -> Result<(), Error> {
        self.0.update(updates.0, scalar)?;
        self.1.update(updates.1, scalar)?;
        self.2.update(updates.2, scalar)?;
        self.3.update(updates.3, scalar)?;
        Ok(())
    }
}

impl<
        Input,
        M1: BackpropModule<Input>,
        M2: BackpropModule<M1::Output>,
        M3: BackpropModule<M2::Output>,
        M4: BackpropModule<M3::Output>,
        M5: BackpropModule<M4::Output>,
    > BackpropModule<Input> for (M1, M2, M3, M4, M5)
{
    type SelfGrads = (
        <M1 as BackpropModule<Input>>::SelfGrads,
        <M2 as BackpropModule<M1::Output>>::SelfGrads,
        <M3 as BackpropModule<M2::Output>>::SelfGrads,
        <M4 as BackpropModule<M3::Output>>::SelfGrads,
        <M5 as BackpropModule<M4::Output>>::SelfGrads,
    );

    fn backprop(
        &mut self,
        trace: &<Self as TracedModule<Input>>::Trace,
        grads_wrt_output: <Self as Module<Input>>::Output,
    ) -> (Input, Self::SelfGrads) {
        let (next_grads, u5) = self.4.backprop(&trace.4, grads_wrt_output);
        let (next_grads, u4) = self.3.backprop(&trace.3, next_grads);
        let (next_grads, u3) = self.2.backprop(&trace.2, next_grads);
        let (next_grads, u2) = self.1.backprop(&trace.1, next_grads);
        let (next_grads, u1) = self.0.backprop(&trace.0, next_grads);
        (next_grads, (u1, u2, u3, u4, u5))
    }

    fn update(&mut self, updates: Self::SelfGrads, scalar: f32) -> Result<(), Error> {
        self.0.update(updates.0, scalar)?;
        self.1.update(updates.1, scalar)?;
        self.2.update(updates.2, scalar)?;
        self.3.update(updates.3, scalar)?;
        self.4.update(updates.4, scalar)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::layers;
    use rand::rngs::SmallRng;
    use rand::SeedableRng;

    #[test]
    fn test_module_forward() {
        let mut network = (
            layers::Dense::<f32, 2, 3>::default(),
            layers::Activation::Relu,
            layers::Bias1d::<f32, 3>::default(),
        );
        network.0.weights[0][0] = 2.5;
        network.0.weights[0][1] = 0.5;
        network.2.bias.iter_mut().for_each(|x| *x += 1.0);

        let output = network.forward(&[1.0, 2.0]);
        assert_eq!(output, Ok([3.5, 1.5, 1.0]));

        // Also try the TracedModule path
        let (output2, _) = network.traced_forward([1.0, 2.0]).unwrap();
        assert_eq!(output.unwrap(), output2);
    }

    #[test]
    fn test_module_backward() {
        let mut network = (
            layers::Dense::<f32, 2, 3>::default(),
            layers::Activation::Relu,
            layers::Bias1d::<f32, 3>::default(),
        );

        let (_, trace) = network.traced_forward([1.0, 2.0]).unwrap();
        let (grad_wrt_input, _gradient_updates) = network.backprop(&trace, [0.0, 0.0, 0.0]);
        assert_eq!(grad_wrt_input, [0.0, 0.0]);
    }

    #[test]
    fn test_nested_module() {
        let mut network = (
            (
                layers::Dense::<f32, 2, 3>::default(),
                layers::Bias1d::<f32, 3>::default(),
            ),
            layers::Activation::Relu,
        );

        let (_, trace) = network.traced_forward([1.0, 2.0]).unwrap();
        let (grad_wrt_input, _gradient_updates) = network.backprop(&trace, [5.0, 1.0, 1.0]);
        assert_eq!(grad_wrt_input, [0.0, 0.0]);
    }

    #[test]
    fn test_reset_params() {
        let mut network = (
            layers::Dense::<f32, 2, 3>::default(),
            layers::Bias1d::<f32, 3>::default(),
            layers::Activation::<f32>::Relu,
        );
        let mut rng = SmallRng::seed_from_u64(1);
        network.rand_params(&mut rng, 1.0).unwrap();

        for x in network.0.weights.iter().flatten() {
            assert!(*x != 0.0);
        }
        for x in network.1.bias.iter() {
            assert!(*x != 0.0);
        }
    }

    #[test]
    fn test_dense_backprop() {
        let mut network = layers::Dense::<f32, 2, 3>::default();

        let (_, trace) = network.traced_forward([1.0, 2.0]).unwrap();
        let (grad_wrt_input, gradient_updates) = network.backprop(&trace, [0.0, 0.0, 0.0]);
        assert_eq!(grad_wrt_input, [0.0, 0.0]);
        assert_eq!(gradient_updates, [[0.0, 0.0], [0.0, 0.0], [0.0, 0.0]]);

        let (_, gradient_updates) = network.backprop(&trace, [1.0, 0.0, 0.0]);
        assert_eq!(gradient_updates, [[1.0, 0.0], [2.0, 0.0], [0.0, 0.0]]); // TODO: Not sure if this is right?
    }

    #[test]
    fn test_bias1d_backprop() {
        let mut network = layers::Bias1d::<f32, 2>::default();

        let (_, trace) = network.traced_forward([1.0, 2.0]).unwrap();
        let (grad_wrt_input, gradient_updates) = network.backprop(&trace, [0.0, 0.0]);
        assert_eq!(grad_wrt_input, [0.0, 0.0]);
        assert_eq!(gradient_updates, [0.0, 0.0]);

        let (_, gradient_updates) = network.backprop(&trace, [2.0, 7.0]);
        assert_eq!(gradient_updates, [2.0, 7.0]);
    }

    #[test]
    fn test_activation_backprop() {
        let mut network = layers::Activation::Relu;

        let (_, trace) = network.traced_forward([1.0, 2.0]).unwrap();
        let (grad_wrt_input, gradient_updates) = network.backprop(&trace, [0.0, 0.0]);
        assert_eq!(grad_wrt_input, [0.0, 0.0]);
        assert_eq!(gradient_updates, ());
    }
}
