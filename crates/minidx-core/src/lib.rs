mod dtypes;
pub use dtypes::{Dtype, Float, Unit};
pub mod shapes;
pub use shapes::*;

mod iterate;
// pub(crate) use iterate::*;

mod gradients;
pub use gradients::Gradients;
mod modules;
pub use modules::*;

pub mod loss;

pub mod layers;

pub type Error = ();

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

#[cfg(test)]
mod tests {
    use super::*;
    use rand::SeedableRng;
    use rand::{rngs::SmallRng, Rng};

    #[test]
    fn test_basic_train() {
        const LR: f32 = 3.0e-8;
        let mut network = (
            layers::Dense::<f32, 1, 2>::default(),
            layers::Dense::<f32, 2, 2>::default(),
        );
        let mut rng = SmallRng::seed_from_u64(42);
        network.rand_params(&mut rng, 0.5).unwrap();

        for _i in 0..9001 {
            let input = rng.random_range(-20.0..20.0);
            let target = [-input, input];
            let (out, trace) = network.traced_forward([input]).unwrap();
            let loss = loss::mse(&out, &target);
            // println!(
            //     "{}: loss={:?}, input={:}, got={:?}\n\tweights={:?} {:?}",
            //     _i, loss, input, out, &network.0.weights, &network.1.weights
            // );
            let (_, mut gradient_updates) = network.backprop(&trace, target.clone());
            gradient_updates.scale(loss * LR);
            // println!(
            //     "\tupdates={:?}",
            //     Gradients::grad_iter(&gradient_updates).collect::<Vec<_>>(),
            // );
            network.update(gradient_updates).expect("update failed");
        }

        let out = network.forward(&[1.0]).unwrap();
        let loss = loss::mse(&out, &[-1.0, 1.0]);
        assert!(loss < 0.1);
    }

    #[test]
    fn test_cosine_similarity() {
        assert_eq!(cosine_similarity::<f64, _, 1>(&[0.0f32], &[1.0f32]), None);

        assert_eq!(cosine_similarity(&[0.1f32], &[99999.0f32]), Some(1.0f32));
        assert_eq!(
            cosine_similarity(&[1.0f32, 3.0f32], &[1.0f32, 3.0f32]),
            Some(1.0f32)
        );
    }
}
