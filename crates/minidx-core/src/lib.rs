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

#[cfg(test)]
mod tests {
    use super::*;
    use rand::SeedableRng;
    use rand::{rngs::SmallRng, Rng};

    #[test]
    fn test_basic_train() {
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
            // println!(
            //     "\tupdates={:?}",
            //     Gradients::grad_iter(&gradient_updates).collect::<Vec<_>>(),
            // );
            gradient_updates.scale(loss * 1.0e-8);
            network.update(gradient_updates).expect("updated failed");
        }

        let out = network.forward(&[1.0]).unwrap();
        let loss = loss::mse(&out, &[-1.0, 1.0]);
        assert!(loss < 0.1);
    }
}
