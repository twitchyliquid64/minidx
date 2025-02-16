use crate::Float;

fn sigmoid<E: Float>(i: E) -> E {
    E::ONE / (E::ONE + i.neg().exp())
}

/// An element-wise activation function with no trainable parameters.
#[derive(Clone, Debug, Default)]
pub enum Activation<E: Float> {
    /// [Rectified Linear Unit (ReLU)](https://en.wikipedia.org/wiki/Rectifier_(neural_networks)). `max(0, t)`
    ///
    /// The derivative is the [Heaviside](https://en.wikipedia.org/wiki/Heaviside_step_function) function.
    #[default]
    Relu,
    /// [Sigmoid](https://en.wikipedia.org/wiki/Sigmoid_function). `1 / (1 + exp(-t))`.
    ///
    /// The derivative is `sigmoid(t) * (1.0 - sigmoid(t))`.
    Sigmoid,
    /// [Leaky ReLu](https://en.wikipedia.org/wiki/Rectifier_(neural_networks)#Piecewise-linear_variants). `if t > 0 { t } else { a * t }`
    LeakyRelu(E),
}

impl<E: Float> Activation<E> {
    #[inline]
    fn forward<const I: usize>(&self, input: &[E; I]) -> [E; I] {
        let mut out: [E; I] = [E::default(); I];
        for (o, i) in out.iter_mut().zip(input.iter()) {
            *o = match self {
                Activation::Sigmoid => sigmoid(*i),
                Activation::Relu => E::default().max(*i),
                Activation::LeakyRelu(a) => {
                    if i < &E::default() {
                        *a * *i
                    } else {
                        *i
                    }
                }
            };
        }
        out
    }

    #[inline]
    fn backward<const I: usize>(&self, input: &[E; I]) -> [E; I] {
        let mut out: [E; I] = [E::default(); I];
        for (o, i) in out.iter_mut().zip(input.iter()) {
            *o = match self {
                Activation::Sigmoid => {
                    let sig = sigmoid(*i);
                    sig * E::ONE.sub(sig)
                    // TODO: Do we need to compute sigmoid, can we just use i?
                    // Thats what dfdx does: https://github.com/coreylowman/dfdx/blob/main/dfdx-core/src/tensor_ops/sigmoid/cpu_kernel.rs#L12
                }
                Activation::Relu => {
                    if i > &E::default() {
                        E::ONE
                    } else {
                        E::default()
                    }
                }
                Activation::LeakyRelu(a) => {
                    if i < &E::default() {
                        *a
                    } else {
                        E::ONE
                    }
                }
            };
        }
        out
    }
}

impl<E: Float> crate::BaseModule for Activation<E> {}

impl<E: Float, const I: usize> crate::Module<[E; I]> for Activation<E> {
    type Output = [E; I];

    fn forward(&mut self, x: &[E; I]) -> Result<Self::Output, crate::Error> {
        Ok(Activation::forward(self, x))
    }
}

impl<E: Float, const I: usize> crate::RevModule<[E; I]> for Activation<E> {
    type SelfGrads = ();

    fn reverse(&mut self, inputs: &[E; I], grads_wrt_output: &[E; I]) -> ([E; I], Self::SelfGrads) {
        let mut grads = self.backward(inputs);
        grads
            .iter_mut()
            .zip(grads_wrt_output)
            .for_each(|(ga, go)| *ga *= *go);

        (grads, ())
    }

    fn apply(
        &mut self,
        _applyer: &mut impl crate::optimizers::GradApplyer,
        _updates: Self::SelfGrads,
    ) -> Result<(), crate::Error> {
        Ok(())
    }
}

impl<E: Float> crate::ResetParams for Activation<E> {
    fn rand_params<RNG: rand::Rng>(
        &mut self,
        _rng: &mut RNG,
        _scale: f32,
    ) -> Result<(), crate::Error> {
        Ok(())
    }
}

impl<E: Float> crate::VisualizableUnit for Activation<E> {
    const KIND: &'static str = "activation";
    type Params = ();
    fn params(&self) -> &Self::Params {
        &()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sigmoid() {
        let layer = Activation::Sigmoid;
        let out = layer.forward(&[10.0, -10.0]);
        assert!(out[0] > 0.99994);
        assert!(out[0] < 0.99996);
        assert!(out[1] > 1.0e-6);
        assert!(out[1] < 1.0e-4);
    }

    #[test]
    fn test_relu() {
        let layer = Activation::Relu;
        let out = layer.forward(&[10.0, 0.0, -0.0001]);
        assert_eq!(out[0], 10.0);
        assert_eq!(out[1], 0.0);
        assert_eq!(out[2], 0.0);
    }

    #[test]
    fn test_leaky_relu() {
        let layer = Activation::LeakyRelu(0.01);
        let out = layer.forward(&[10.0, 0.0, -0.1]);
        assert_eq!(out[0], 10.0);
        assert_eq!(out[1], 0.0);
        assert_eq!(out[2], -0.001);
    }
}
