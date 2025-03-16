use crate::Float;

/// Some output which can have its loss computed.
pub trait DiffLoss: Clone {
    type Output: Clone;

    /// Computes the Mean-squared error.
    fn mse(&self, truth: &Self) -> Self::Output;
    /// Computes the gradients of each input with regards to the [DiffLoss::mse].
    fn mse_input_grads(&self, truth: &Self) -> Self;

    /// Computes the Huber error.
    fn huber(&self, beta: f32, truth: &Self) -> Self::Output;
    /// Computes the gradients of each input with regards to the [DiffLoss::huber].
    fn huber_input_grads(&self, beta: f32, truth: &Self) -> Self;
}

impl<E: Float, const I: usize> DiffLoss for [E; I] {
    type Output = E;

    /// Computes the Mean-squared error.
    fn mse(&self, truth: &Self) -> E {
        if I == 0 {
            return E::default();
        }

        self.iter()
            .zip(truth)
            .fold(E::default(), |a, (test, truth)| {
                let err = *test - *truth;
                a + (err * err)
            })
            / E::from_usize(I).unwrap()
    }

    /// Computes the gradients of each input with regards to the [DiffLoss::mse].
    fn mse_input_grads(&self, truth: &Self) -> [E; I] {
        let c = E::from_usize(I).unwrap();
        let mut out = [E::default(); I];

        out.iter_mut()
            .zip(self)
            .zip(truth)
            .for_each(|((out, test), truth)| *out = (E::ONE + E::ONE) * (*test - *truth) / c);

        out
    }

    /// Computes the Huber error.
    fn huber(&self, beta: f32, truth: &Self) -> E {
        if I == 0 {
            return E::default();
        }
        use num_traits::FromPrimitive;
        let half = E::from_f32(0.5).unwrap();
        let beta = E::from_f32(beta).unwrap();

        self.iter()
            .zip(truth)
            .fold(E::default(), |a, (test, truth)| {
                let err = *test - *truth;
                let err_abs = err.abs();

                let huber_err = if err_abs < beta {
                    half * err * err
                } else {
                    beta * (err_abs - half * beta * beta)
                };

                a + huber_err
            })
            / E::from_usize(I).unwrap()
    }

    /// Computes the gradients of each input with regards to the [DiffLoss::huber].
    fn huber_input_grads(&self, beta: f32, truth: &Self) -> [E; I] {
        use num_traits::FromPrimitive;
        let beta = E::from_f32(beta).unwrap();
        let c = E::from_usize(I).unwrap();
        let mut out = [E::default(); I];

        out.iter_mut()
            .zip(self)
            .zip(truth)
            .for_each(|((out, test), truth)| {
                let err = *test - *truth;
                let err_abs = err.abs();

                *out = if err_abs < beta {
                    err / c
                } else {
                    let signum = if err > E::default() { E::ONE } else { -E::ONE };

                    signum * beta / c
                };
            });

        out
    }
}

/// Some output which can have cross-entropy loss computed.
pub trait LogitLoss: Clone {
    type Output: Clone;

    /// Computes the binary cross-entropy loss: assumes inputs are the outputs from
    /// sigmoid activation.
    fn logit_bce(&self, truth: &Self) -> Self::Output;
    /// Computes the gradients with respect to the inputs for [LogitLoss::logit_bce].
    fn logit_bce_input_grads(&self, truth: &Self) -> Self;
}

impl<E: Float, const I: usize> LogitLoss for [E; I] {
    type Output = E;
    /// Computes the binary cross-entropy loss: assumes inputs are the outputs from
    /// sigmoid activation.
    fn logit_bce(&self, truth: &Self) -> E {
        if I == 0 {
            return E::default();
        }

        (self
            .iter()
            .zip(truth)
            .fold(E::default(), |a, (test, truth)| {
                let y = *truth;
                let y_hat = test.max(E::SMOL).min(E::ONE - E::SMOL); // Avoid 0

                a + (y * y_hat.ln() + (E::ONE - y) * (E::ONE - y_hat).ln())
            })
            / E::from_usize(I).unwrap())
        .neg()
        .max(E::SMOL)
    }

    /// Computes the gradients with respect to the inputs for [LogitLoss::logit_bce].
    fn logit_bce_input_grads(&self, truth: &Self) -> Self {
        let mut out = [E::default(); I];

        out.iter_mut()
            .zip(self)
            .zip(truth)
            .for_each(|((out, test), truth)| {
                let y = *truth;
                let y_hat = *test;

                *out = (y_hat - y) / (E::SMOL + y_hat * (E::ONE - y_hat));
            });

        out
    }
}

/// Some output which can have cosine similarity/difference computed.
pub trait CosineLoss<P>: Clone
where
    P: Float + std::iter::Sum + std::fmt::Display,
{
    /// Computes the cosine similarity.
    fn cosine_similarity(&self, other: &Self) -> Option<P>;

    /// Computes the cosine distance.
    fn cosine_distance(&self, other: &Self) -> Option<P> {
        self.cosine_similarity(other).map(|p| P::ONE - p)
    }
}

/// Computes the cosine similarity between two arrays of the same shape.
///
/// NB: Cosine distance is 1 - similarity.
fn cosine_similarity<
    P: Float + std::iter::Sum + std::fmt::Display,
    E: Float + num_traits::AsPrimitive<P>,
    const N: usize,
>(
    a: &[E; N],
    b: &[E; N],
) -> Option<P> {
    let dot_product: P = a.iter().zip(b.iter()).map(|(x, y)| x.as_() * y.as_()).sum();
    let magnitude_a: P = a.iter().map(|x| x.as_() * x.as_()).sum::<P>().sqrt();
    let magnitude_b: P = b.iter().map(|x| x.as_() * x.as_()).sum::<P>().sqrt();

    if magnitude_a == P::default() || magnitude_b == P::default() {
        return None;
    }

    Some(dot_product / (magnitude_a * magnitude_b))
}

impl<P, E: Float + num_traits::AsPrimitive<P>, const I: usize> CosineLoss<P> for [E; I]
where
    P: Float + std::iter::Sum + std::fmt::Display,
{
    fn cosine_similarity(&self, other: &Self) -> Option<P> {
        cosine_similarity(self, other)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mse() {
        assert_eq!([0.0f32; 0].mse(&[]), 0.0f32);
        assert_eq!([3.5].mse(&[3.5]), 0.0f32);
        assert_eq!([2.0, -1.0].mse(&[5.0, 1.0]), 6.5);

        assert_eq!([2.0, -1.0].mse_input_grads(&[5.0, 1.0]), [-3.0, -2.0]);
    }

    #[test]
    fn test_logit_bce() {
        assert_eq!([0.0f32; 0].logit_bce(&[]), 0.0f32);
        assert_eq!([0.0f32; 0].logit_bce_input_grads(&[]), []);

        // Small error for correct answer
        assert_eq!([1.0, 0.0].logit_bce(&[1.0, 0.0]), f32::SMOL);
        // Zero derivatives for correct answer
        assert_eq!([1.0, 0.0].logit_bce_input_grads(&[1.0, 0.0]), [0.0, 0.0]);

        // Derivatives in correct direction, and equal error ~= equal dfdx
        let [d_under, d_over]: [f32; 2] = [0.99, 0.01].logit_bce_input_grads(&[1.0, 0.0]);
        assert!(d_under < 0.0);
        assert!(d_over > 0.0);
        assert!(d_under.abs() / d_over > 0.99 && d_under.abs() / d_over < 1.01);

        // Bigger errors have a bigger loss.
        let one_wrong_loss = [0.8, 0.2].logit_bce(&[1.0, 0.0]);
        assert!(one_wrong_loss >= 0.1);
        let two_wrong_loss = [0.0, 1.0].logit_bce(&[1.0, 0.0]);
        assert!(two_wrong_loss >= 1.0);
        assert!(two_wrong_loss >= 1.999 * one_wrong_loss);
    }

    #[test]
    fn test_cosine_similarity() {
        assert_eq!(cosine_similarity::<f64, _, 1>(&[0.0f32], &[1.0f32]), None);

        assert_eq!([0.1f32].cosine_similarity(&[99999.0f32]), Some(1.0f32));
        assert_eq!(
            cosine_similarity(&[1.0f32, 3.0f32], &[1.0f32, 3.0f32]),
            Some(1.0f32)
        );
    }

    #[test]
    fn test_huber() {
        assert_eq!([0.0f32; 0].huber(1.0, &[]), 0.0f32);
        assert_eq!([3.5].huber(1.0, &[3.5]), 0.0f32);

        assert_eq!([2.0].huber(1.0, &[5.0]), 2.5);
        assert_eq!([2.0].huber_input_grads(1.0, &[5.0]), [-1.0]);

        assert_eq!([3.0].huber(1.0, &[3.5]), 0.125);
        assert_eq!([3.0].huber_input_grads(1.0, &[3.5]), [-0.5]);
        assert_eq!([3.5].huber_input_grads(1.0, &[3.0]), [0.5]);
    }
}
