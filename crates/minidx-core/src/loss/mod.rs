use crate::{Dtype, Float};

/// Computes the Mean-squared error.
pub fn mse<E: Dtype, const I: usize>(test: &[E; I], truth: &[E; I]) -> E {
    if I == 0 {
        return E::default();
    }

    test.iter()
        .zip(truth)
        .fold(E::default(), |a, (test, truth)| {
            let err = *test - *truth;
            a + (err * err)
        })
        / E::from_usize(I).unwrap()
}

/// Computes the gradients of each input with regards to the [mse].
pub fn mse_input_grads<E: Dtype, const I: usize>(test: &[E; I], truth: &[E; I]) -> [E; I] {
    let c = E::from_usize(I).unwrap();
    let mut out = [E::default(); I];

    out.iter_mut()
        .zip(test)
        .zip(truth)
        .for_each(|((out, test), truth)| *out = (E::ONE + E::ONE) * (*test - *truth) / c);

    out
}

/// Computes the binary cross-entropy loss: assumes inputs are the outputs from
/// sigmoid activation.
pub fn logit_bce<E: Float, const I: usize>(test: &[E; I], truth: &[E; I]) -> E {
    if I == 0 {
        return E::default();
    }

    (test
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

/// Computes the gradients with respect to the inputs for [logit_bce].
pub fn logit_bce_input_grads<E: Float, const I: usize>(test: &[E; I], truth: &[E; I]) -> [E; I] {
    let mut out = [E::default(); I];

    out.iter_mut()
        .zip(test)
        .zip(truth)
        .for_each(|((out, test), truth)| {
            let y = *truth;
            let y_hat = *test;

            *out = (y_hat - y) / (E::SMOL + y_hat * (E::ONE - y_hat));
        });

    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mse() {
        assert_eq!(mse::<f32, 0>(&[], &[]), 0.0f32);
        assert_eq!(mse(&[3.5], &[3.5]), 0.0f32);
        assert_eq!(mse(&[2.0, -1.0], &[5.0, 1.0]), 6.5);

        assert_eq!(mse_input_grads(&[2.0, -1.0], &[5.0, 1.0]), [-3.0, -2.0]);
    }

    #[test]
    fn test_logit_bce() {
        assert_eq!(logit_bce::<f32, 0>(&[], &[]), 0.0f32);
        assert_eq!(logit_bce_input_grads::<f32, 0>(&[], &[]), []);

        // Small error for correct answer
        assert_eq!(logit_bce(&[1.0, 0.0], &[1.0, 0.0]), f32::SMOL);
        // Zero derivatives for correct answer
        assert_eq!(logit_bce_input_grads(&[1.0, 0.0], &[1.0, 0.0]), [0.0, 0.0]);

        // Derivatives in correct direction, and equal error ~= equal dfdx
        let [d_under, d_over]: [f32; 2] = logit_bce_input_grads(&[0.99, 0.01], &[1.0, 0.0]);
        assert!(d_under < 0.0);
        assert!(d_over > 0.0);
        assert!(d_under.abs() / d_over > 0.99 && d_under.abs() / d_over < 1.01);

        // Bigger errors have a bigger loss.
        let one_wrong_loss = logit_bce(&[0.8, 0.2], &[1.0, 0.0]);
        assert!(one_wrong_loss >= 0.1);
        let two_wrong_loss = logit_bce(&[0.0, 1.0], &[1.0, 0.0]);
        assert!(two_wrong_loss >= 1.0);
        assert!(two_wrong_loss >= 1.999 * one_wrong_loss);
    }
}
