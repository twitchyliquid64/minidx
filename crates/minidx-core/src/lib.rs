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
use loss::DiffLoss;

pub mod layers;

pub type Error = ();

/// Describes the basic set of parameters used in training.
pub struct TrainParams {
    pub lr: f32,
}

impl Default for TrainParams {
    fn default() -> Self {
        Self { lr: 1.0e-8 }
    }
}

/// Does a training step, updating a network using a pair of inputs and outputs.
pub fn train_step<Input, Network: BackpropModule<Input>>(
    params: &TrainParams,
    network: &mut Network,
    input: Input,
    output: Network::Output,
) where
    Network::Output: DiffLoss,
    <Network as modules::BackpropModule<Input>>::SelfGrads: Gradients,
    <<Network as modules::Module<Input>>::Output as loss::DiffLoss>::Output:
        std::ops::Mul<f32, Output = f32>,
{
    let (out, trace) = network.traced_forward(input).unwrap();
    let loss = out.mse(&output);
    // println!(
    //     "{}: loss={:?}, input={:}, got={:?}\n\tweights={:?} {:?}",
    //     _i, loss, input, out, &network.0.weights, &network.1.weights
    // );
    let (_, mut gradient_updates) = network.backprop(&trace, output.clone());
    gradient_updates.scale(loss * params.lr);
    // println!(
    //     "\tupdates={:?}",
    //     Gradients::grad_iter(&gradient_updates).collect::<Vec<_>>(),
    // );
    network.update(gradient_updates).expect("update failed");
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::SeedableRng;
    use rand::{rngs::SmallRng, Rng};

    #[test]
    fn test_manual_train() {
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
            let loss = out.mse(&target);
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
        let loss = out.mse(&[-1.0, 1.0]);
        assert!(loss < 0.1);
    }

    #[test]
    fn test_train_step() {
        let mut network = (
            layers::Dense::<f32, 1, 2>::default(),
            layers::Dense::<f32, 2, 2>::default(),
        );
        let mut rng = SmallRng::seed_from_u64(675);
        network.rand_params(&mut rng, 0.5).unwrap();

        for _i in 0..9001 {
            let input = rng.random_range(-10.0..10.0);
            let target = [-input, input];
            let params = TrainParams::default();
            train_step(&params, &mut network, [input], target);
        }

        let out = network.forward(&[1.0]).unwrap();
        let loss = out.mse(&[-1.0, 1.0]);
        assert!(loss < 0.1);
    }
}
