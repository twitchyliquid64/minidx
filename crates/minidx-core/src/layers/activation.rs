use crate::Float;

pub(crate) fn sigmoid<E: Float>(i: E) -> E {
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
    /// [SiLU / Swish1](https://en.wikipedia.org/wiki/Swish_function). `t / (1 + exp(-t))`.
    ///
    /// The derivative is `sigmoid(t) * (1.0 + t * (1.0 - sigmoid(t)))`.
    SiLU,
    /// [Tanh](https://en.wikipedia.org/wiki/Hyperbolic_functions). `(e^x - e^-x) / (e^x + e^-x)`.
    ///
    /// The derivative is `1 - (tanh(t)^2)`.
    Tanh,
    /// [Leaky ReLu](https://en.wikipedia.org/wiki/Rectifier_(neural_networks)#Piecewise-linear_variants). `if t > 0 { t } else { a * t }`
    LeakyRelu(E),
    /// [Softplus](https://en.wikipedia.org/wiki/Softplus). `ln(1 + e^x)`
    Softplus,
    /// Sine function.
    ///
    /// The derivative is `cos(t)`.
    Sine,
    /// Cosine function.
    ///
    /// The derivative is `-sin(t)`.
    Cosine,
}

impl<E: Float> Activation<E> {
    #[inline]
    fn forward<const I: usize>(&self, input: &[E; I]) -> [E; I] {
        let mut out: [E; I] = [E::default(); I];
        for (o, i) in out.iter_mut().zip(input.iter()) {
            *o = match self {
                Activation::Sigmoid => sigmoid(*i),
                Activation::SiLU => *i * sigmoid(*i),
                Activation::Tanh => i.tanh(),
                Activation::Relu => E::default().max(*i),
                Activation::LeakyRelu(a) => {
                    if i < &E::default() {
                        *a * *i
                    } else {
                        *i
                    }
                }
                Activation::Softplus => (i.exp() + E::ONE).ln(),
                Activation::Sine => i.sin(),
                Activation::Cosine => i.cos(),
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
                Activation::SiLU => {
                    let sig = sigmoid(*i);
                    sig * (E::ONE + *i * E::ONE.sub(sig))
                }
                Activation::Tanh => {
                    let tanh = i.tanh();
                    E::ONE.sub(tanh * tanh)
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
                Activation::Softplus => sigmoid(*i),
                Activation::Sine => (*i).cos(),
                Activation::Cosine => -(*i).sin(),
            };
        }
        out
    }
}

impl<E: Float> crate::BaseModule for Activation<E> {}

impl<E: Float, const I: usize> crate::Module<[E; I]> for Activation<E> {
    type Output = [E; I];

    fn forward(&self, x: &[E; I]) -> Result<Self::Output, crate::Error> {
        Ok(Activation::forward(self, x))
    }
}

impl<E: Float, const I: usize> crate::RevModule<[E; I]> for Activation<E> {
    type SelfGrads = ();

    fn reverse(&self, inputs: &[E; I], grads_wrt_output: &[E; I]) -> ([E; I], Self::SelfGrads) {
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

impl<E: Float> crate::LoadableModule for Activation<E> {
    fn save(
        &self,
        _path: String,
        _dict: &mut std::collections::HashMap<String, Vec<f64>>,
    ) -> Result<(), crate::LoadSaveError> {
        Ok(())
    }

    fn load(
        &mut self,
        _path: String,
        _dict: &std::collections::HashMap<String, Vec<f64>>,
    ) -> Result<(), crate::LoadSaveError> {
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
    fn test_silu() {
        let layer = Activation::SiLU;
        let out = layer.forward(&[10.0, -10.0, 2.0]);
        assert!(out[0] > 9.9, "val is {}", out[0]);
        assert!(out[0] < 10.0, "val is {}", out[0]);
        assert!(out[1] > -1.0e-2, "val is {}", out[1]);
        assert!(out[1] < -1.0e-5, "val is {}", out[1]);
        assert!(out[2] > 1.75, "val is {}", out[2]);
        assert!(out[2] < 1.77, "val is {}", out[2]);

        let back = layer.backward(&[2.0]);
        assert!(back[0] > 1.08, "val is {}", back[0]);
        assert!(back[0] < 1.10, "val is {}", back[0]);
    }

    #[test]
    fn test_tanh() {
        let layer = Activation::Tanh;
        let out = layer.forward(&[10.0, -10.0, 0.0]);
        assert!(out[0] > 0.9, "val is {}", out[0]);
        assert!(out[0] < 1.01, "val is {}", out[0]);
        assert!(out[1] > -1.1, "val is {}", out[1]);
        assert!(out[1] < -0.9, "val is {}", out[1]);
        assert!(out[2] == 0.0, "val is {}", out[2]);

        let back = layer.backward(&[0.0, -3.0]);
        assert!(back[0] > 0.99, "val is {}", back[0]);
        assert!(back[0] < 1.01, "val is {}", back[0]);
        assert!(back[1] < 0.01, "val is {}", back[0]);
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

    #[test]
    fn test_softplus() {
        let layer = Activation::Softplus;
        let out = layer.forward(&[1.0]);
        assert!(out[0] > 1.31);
        assert!(out[0] < 1.33);

        let back = layer.backward(&[1.0]);
        assert!(back[0] > 0.72, "val is {}", back[0]);
        assert!(back[0] < 0.74, "val is {}", back[0]);
    }
}
