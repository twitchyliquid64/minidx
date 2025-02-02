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
pub fn train_step<Input, LV: Float, Network: BackpropModule<Input>>(
    params: &TrainParams,
    network: &mut Network,
    loss: impl Fn(&Network::Output, &Network::Output) -> (LV, Network::Output),
    input: Input,
    output: Network::Output,
) where
    // Network::Output: std::fmt::Debug,
    LV: std::ops::Mul<f32, Output = f32>,
    <Network as modules::BackpropModule<Input>>::SelfGrads: Gradients,
{
    let (out, trace) = network.traced_forward(input).unwrap();

    // println!("got {:?}, want {:?}", &out, &output);
    let (lv, loss_grads) = loss(&out, &output);

    // println!("{:?}: loss_grads={:?}", lv, loss_grads);

    let (_, mut gradient_updates) = network.backprop(&trace, loss_grads);
    gradient_updates.scale(-lv * params.lr);
    // println!("updates: {:?}\n", gradient_updates);

    network.update(gradient_updates).expect("update failed");
}

#[cfg(test)]
mod tests {
    use super::*;
    use loss::{DiffLoss, LogitLoss};
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

            // NOTE: we should use the gradients WRT the loss as the input to
            // backprop, not the target.
            let (_, mut gradient_updates) = network.backprop(&trace, target.clone());
            gradient_updates.scale(loss * LR);

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
        let mut rng = SmallRng::seed_from_u64(765);
        network.rand_params(&mut rng, 0.1).unwrap();

        let params = TrainParams { lr: 1.0e-6 };
        for _i in 0..300 {
            let input = rng.random_range(-20.0..20.0);
            let target = [-input, input];
            train_step(
                &params,
                &mut network,
                |got, want| (got.mse(want), got.mse_input_grads(want)),
                [input],
                target,
            );
        }

        let out = network.forward(&[1.0]).unwrap();
        let loss = out.mse(&[-1.0, 1.0]);
        assert!(loss < 0.1);
    }

    #[test]
    fn test_train_step_softmax() {
        let mut network = (
            (
                layers::Dense::<f32, 1, 2>::default(),
                layers::Bias1d::<f32, 2>::default(),
                layers::Activation::Relu,
            ),
            layers::Softmax {},
        );
        let mut rng = SmallRng::seed_from_u64(23423);
        network.rand_params(&mut rng, 0.5).unwrap();

        let func = |inp| {
            let r = if inp > 0.5 { 1.0 } else { 0.0 };
            [1.0 - r, r]
        };

        let params = TrainParams { lr: 5.0e-1 };
        for _i in 0..9000 {
            let input = rng.random_range(-2.0..2.0);
            let target = func(input);
            train_step(
                &params,
                &mut network,
                |got, want| (got.logit_bce(want), got.logit_bce_input_grads(want)),
                [input],
                target,
            );
        }

        let out = network.forward(&[0.4]).unwrap();
        let loss = out.logit_bce(&func(0.4));
        println!("got={:?}, want={:?}: loss={}", out, func(2.0), loss);
        assert!(loss < 0.3);
    }

    #[test]
    fn test_train_step_sigmoid() {
        let mut network = (
            (
                layers::Dense::<f32, 1, 5>::default(),
                layers::Bias1d::<f32, 5>::default(),
                layers::Activation::LeakyRelu(0.5),
            ),
            (
                layers::Dense::<f32, 5, 2>::default(),
                layers::Bias1d::<f32, 2>::default(),
            ),
            layers::Activation::Sigmoid,
        );
        let mut rng = SmallRng::seed_from_u64(343);
        network.rand_params(&mut rng, 1.8).unwrap();

        let func = |inp| {
            let r = if inp > 0.5 { 1.0 } else { 0.0 };
            [1.0 - r, r]
        };

        let params = TrainParams { lr: 5.0e-2 };
        for _i in 0..3000 {
            let input = rng.random_range(-2.0..2.0);
            let target = func(input);
            train_step(
                &params,
                &mut network,
                |got, want| (got.logit_bce(want), got.logit_bce_input_grads(want)),
                [input],
                target,
            );
        }

        let inp = 0.7;
        let out = network.forward(&[inp]).unwrap();
        let loss = out.logit_bce(&func(inp));
        println!("got={:?}, want={:?}: loss={}", out, func(inp), loss);
        assert!(loss < 0.2);
    }
}
