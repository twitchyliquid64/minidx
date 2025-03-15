use crate::optimizers::GradApplyer;
use crate::{Error, Gradients};
use std::collections::HashMap;

/// An error loading or saving parameters.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LoadSaveError {
    pub path: String,
    pub err: String,
}

/// A unit of computation that consumes `Input` and produces [Module::Output].
pub trait Module<X> {
    /// The type that this unit produces given `Input`.
    type Output;

    fn forward(&self, x: &X) -> Result<Self::Output, Error>;
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
        &self,
        inputs: &X,
        grads_wrt_output: &<Self as Module<X>>::Output,
    ) -> (X, Self::SelfGrads);

    /// Applies a gradient update step: adding product of the provided gradients and
    /// the scalar to the parameters.
    fn apply(
        &mut self,
        applyer: &mut impl GradApplyer,
        updates: Self::SelfGrads,
    ) -> Result<(), Error>;
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
    fn traced_forward(&self, x: X) -> Result<(<Self as Module<X>>::Output, Self::Trace), Error>;
}

impl<Input, M: Module<Input> + BaseModule> TracedModule<Input> for M {
    type Trace = Input;

    fn traced_forward(
        &self,
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
        &self,
        trace: &<Self as TracedModule<X>>::Trace,
        grads_wrt_output: <Self as Module<X>>::Output,
    ) -> (X, Self::SelfGrads);

    /// Applies a gradient update step: adding product of the provided gradients and
    /// the scalar to the parameters.
    fn update(
        &mut self,
        applyer: &mut impl GradApplyer,
        updates: Self::SelfGrads,
    ) -> Result<(), Error>;

    /// Initializes state which can be used to track momentum during training.
    fn new_momentum(
        &self,
        params: crate::optimizers::TrainParams,
        momentum_coefficient: f32,
    ) -> crate::optimizers::Momentum<Self::SelfGrads>
    where
        <Self as BackpropModule<X>>::SelfGrads: Gradients,
    {
        crate::optimizers::Momentum::new(params, momentum_coefficient)
    }

    /// Initializes state which can be used to track rmsprop during training.
    fn new_rmsprop(
        &self,
        params: crate::optimizers::TrainParams,
        beta: f32,
    ) -> crate::optimizers::RMSProp<Self::SelfGrads>
    where
        <Self as BackpropModule<X>>::SelfGrads: Gradients,
        <Self::SelfGrads as Gradients>::Concrete: crate::Float,
    {
        crate::optimizers::RMSProp::new(params, beta)
    }

    /// Initializes state which can be used to track rmsprop & momentum during training.
    fn new_rmsprop_with_momentum(
        &self,
        params: crate::optimizers::TrainParams,
        momentum_coefficient: f32,
        beta: f32,
    ) -> crate::optimizers::RMSProp<Self::SelfGrads>
    where
        <Self as BackpropModule<X>>::SelfGrads: Gradients,
        <Self::SelfGrads as Gradients>::Concrete: crate::Float,
    {
        crate::optimizers::RMSProp::new_with_momentum(params, momentum_coefficient, beta)
    }
}

impl<Input, M: TracedModule<Input, Trace = Input> + RevModule<Input> + BaseModule>
    BackpropModule<Input> for M
{
    type SelfGrads = <M as RevModule<Input>>::SelfGrads;

    fn backprop(
        &self,
        trace: &<M as TracedModule<Input>>::Trace,
        grads_wrt_output: <M as Module<Input>>::Output,
    ) -> (Input, Self::SelfGrads) {
        M::reverse(self, trace, &grads_wrt_output)
    }

    fn update(
        &mut self,
        applyer: &mut impl GradApplyer,
        updates: Self::SelfGrads,
    ) -> Result<(), Error> {
        M::apply(self, applyer, updates)
    }
}

/// A module who's parameters can be loaded or saved.
pub trait LoadableModule {
    /// Saves the parameters to the given dictionary.
    ///
    /// FIXME: We should be storing parameters as their base type, not f64.
    fn save(&self, path: String, dict: &mut HashMap<String, Vec<f64>>)
        -> Result<(), LoadSaveError>;

    /// Loads the parameters from the given dictionary.
    ///
    /// FIXME: We should be storing parameters as their base type, not f64.
    fn load(&mut self, path: String, dict: &HashMap<String, Vec<f64>>)
        -> Result<(), LoadSaveError>;
}

/// Marker trait for low-level layers which are composable modules.
///
/// Set on layers which are native to minidx: needed to get around
/// trait conflicts between impls on M and (M,)
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
            fn forward(&self, x: &Input) -> Result<Self::Output, Error> {
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
                &self,
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

        impl<
            $last:
            $(crate::LoadableModule, $rev_tail: )*
            crate::LoadableModule
        > crate::LoadableModule for ($($name,)+) {
            fn save(&self, path: String, dict: &mut HashMap<String, Vec<f64>>) -> Result<(), LoadSaveError> {
                self.0.save(path.clone() + ".0", dict)?;
                $(self.$idx.save(format!("{}.{}", path, $idx), dict)?;)*
                Ok(())
            }

            fn load(&mut self, path: String, dict: &HashMap<String, Vec<f64>>) -> Result<(), LoadSaveError> {
                self.0.load(path.clone() + ".0", dict)?;
                $(self.$idx.load(format!("{}.{}", path, $idx), dict)?;)*
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

macro_rules! backwd_tuple_impls {
    ([$($all:ident),+] [$($idx:tt),*] [$($rev_idx:tt),*], $first:ident, [$($rev_grads:ident),+], [$($fwd_grads:ident),+], [$(($mod_for:ident, $mod_from:ident)),*]) => {
        impl<Input,
             $first: BackpropModule<Input>,
             $($mod_for: BackpropModule::<$mod_from ::Output>,)*
            > BackpropModule<Input>
            for ($($all,)+)
        {
            type SelfGrads = (
                <$first as BackpropModule<Input>>::SelfGrads,
                $(<$mod_for as BackpropModule::<$mod_from ::Output>>::SelfGrads,)*
            );

            fn backprop(
                &self,
                trace: &<Self as TracedModule<Input>>::Trace,
                next_grads: <Self as Module<Input>>::Output,
            ) -> (Input, Self::SelfGrads) {
                $(let (next_grads, $rev_grads) = self.$rev_idx.backprop(&trace.$rev_idx, next_grads);)+
                (next_grads, ($($fwd_grads,)+))
            }

            fn update(&mut self, applyer: &mut impl GradApplyer, updates: Self::SelfGrads) -> Result<(), Error> {
                $(self.$idx.update(applyer, updates.$idx)?;)+
                Ok(())
            }

        }
    };
}

backwd_tuple_impls!([M1][0][0], M1, [u1], [u1], []);
backwd_tuple_impls!([M1, M2][0, 1][1, 0], M1, [u2, u1], [u1, u2], [(M2, M1)]);
backwd_tuple_impls!([M1, M2, M3][0, 1, 2][2, 1, 0], M1, [u3, u2, u1], [u1, u2, u3], [(M2, M1), (M3, M2)]);
backwd_tuple_impls!([M1, M2, M3, M4][0, 1, 2, 3][3, 2, 1, 0], M1, [u4, u3, u2, u1], [u1, u2, u3, u4], [(M2, M1), (M3, M2), (M4, M3)]);
backwd_tuple_impls!([M1, M2, M3, M4, M5][0, 1, 2, 3, 4][4, 3, 2, 1, 0], M1, [u5, u4, u3, u2, u1], [u1, u2, u3, u4, u5], [(M2, M1), (M3, M2), (M4, M3), (M5, M4)]);
backwd_tuple_impls!([M1, M2, M3, M4, M5, M6][0, 1, 2, 3, 4, 5][5, 4, 3, 2, 1, 0], M1, [u6, u5, u4, u3, u2, u1], [u1, u2, u3, u4, u5, u6], [(M2, M1), (M3, M2), (M4, M3), (M5, M4), (M6, M5)]);

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

        let mut network = (
            layers::Dense::<f32, 2, 3>::default(),
            layers::Activation::Relu,
            layers::Bias1d::<f32, 3>::default(),
            layers::Bias1d::<f32, 3>::default(),
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
        assert_eq!(gradient_updates, [[1.0, 0.0], [0.0, 2.0], [0.0, 0.0]]); // TODO: Not sure if this is right?
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
