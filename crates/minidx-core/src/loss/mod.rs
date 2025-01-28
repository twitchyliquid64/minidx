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

/// Computes the gradients of each input with regards to the MSE.
pub fn mse_input_grads<E: Dtype, const I: usize>(test: &[E; I], truth: &[E; I]) -> [E; I] {
    let c = E::from_usize(I).unwrap();
    let mut out = [E::default(); I];

    out.iter_mut()
        .zip(test)
        .zip(truth)
        .for_each(|((out, test), truth)| *out = (E::ONE + E::ONE) * (*test - *truth) / c);

    out
}

pub fn logit_bce<E: Float, const I: usize>(test: &[E; I], truth: &[E; I]) -> E {
    if I == 0 {
        return E::default();
    }

    test.iter()
        .zip(truth)
        .fold(E::default(), |a, (test, truth)| {
            // (1 - target_probs) * logits + log(1 + exp(-logits))
            a + (E::ONE - *truth) * *test + (E::ONE + test.neg().exp()).ln()
        })
        / E::from_usize(I).unwrap()
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
        assert_eq!(logit_bce(&[1.0, 0.0], &[1.0, 0.0]), 0.0f32);

        let one_wrong_loss = logit_bce(&[0.0, 1.0], &[1.0, 0.0]);
        assert!(one_wrong_loss <= -0.49);
    }
}
