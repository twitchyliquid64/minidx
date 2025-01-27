use crate::Float;

#[derive(Clone, Debug, Default)]
pub enum Activation<E: Float> {
    #[default]
    Relu,
    Sigmoid,
    LeakyRelu(E),
}

impl<E: Float> Activation<E> {
    #[inline]
    fn forward<const I: usize>(&self, input: &[E; I]) -> [E; I] {
        let mut out: [E; I] = [E::default(); I];
        for (o, i) in out.iter_mut().zip(input.iter()) {
            *o = match self {
                Activation::Sigmoid => E::ONE / (E::ONE + i.neg().exp()),
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
                Activation::Sigmoid => *i * E::ONE.sub(*i),
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

    fn forward(&mut self, x: &[E; I]) -> Result<Self::Output, super::Error> {
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
