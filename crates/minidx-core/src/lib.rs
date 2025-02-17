mod dtypes;
pub use dtypes::{Dtype, Float, Unit};
pub mod shapes;
pub use shapes::*;

#[doc(hidden)]
pub mod matmul;

mod iterate;
// pub(crate) use iterate::*;

mod gradients;
pub use gradients::Gradients;
mod modules;
pub use modules::*;

pub mod layers;
pub mod loss;
pub mod optimizers;
use optimizers::{GradAdjuster, GradApplyer};

pub type Error = ();

/// Does a training step, updating a network using a pair of inputs and outputs.
pub fn train_step<
    Input,
    LV: Float,
    Network: BackpropModule<Input>,
    GA: GradAdjuster<Network::SelfGrads> + GradApplyer,
>(
    ga: &mut GA,
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

    let (_, gradient_updates) = network.backprop(&trace, loss_grads);
    let gradient_updates = ga.adjust(gradient_updates, lv.to_f32().unwrap());
    // println!("updates: {:?}\n", gradient_updates);

    network.update(ga, gradient_updates).expect("update failed");
}

/// Does a training minibatch, updating a network based on averaged gradients from
/// computing N input-output pairs.
pub fn train_batch<
    Input,
    LV: Float,
    Network: BackpropModule<Input>,
    GA: GradAdjuster<Network::SelfGrads> + GradApplyer,
    S: FnMut() -> (Input, Network::Output),
>(
    ga: &mut GA,
    network: &mut Network,
    loss: impl Fn(&Network::Output, &Network::Output) -> (LV, Network::Output),
    source: &mut S,
    batch_size: usize,
) where
    // Network::Output: std::fmt::Debug,
    LV: std::ops::Mul<f32, Output = f32>,
    <Network as modules::BackpropModule<Input>>::SelfGrads: Gradients,
{
    let mut grads: Option<(Network::SelfGrads, LV)> = None;

    let (mut grads, lv) = (0..batch_size).into_iter().fold(
        (Network::SelfGrads::empty(), LV::default()),
        |(mut accumulated_grads, mut accumulated_lv), _i| {
            let (input, output) = source();

            let (out, trace) = network.traced_forward(input).unwrap();
            let (lv, loss_grads) = loss(&out, &output);

            let (_, gradient_updates) = network.backprop(&trace, loss_grads);
            accumulated_grads.add(gradient_updates);
            accumulated_lv += lv;

            (accumulated_grads, accumulated_lv)
        },
    );

    grads.scale((batch_size as f32).recip());
    let gradient_updates = ga.adjust(grads, lv.to_f32().unwrap() * (batch_size as f32).recip());
    network.update(ga, gradient_updates).expect("update failed");
}

/// Something which can have its parameters visualized.
pub trait VisualizableUnit {
    const KIND: &'static str;
    type Params: std::fmt::Debug + Sized;

    fn params(&self) -> &Self::Params;
}

#[cfg(test)]
mod tests {
    use super::*;
    use loss::{DiffLoss, LogitLoss};
    use optimizers::TrainParams;
    use rand::SeedableRng;
    use rand::{rngs::SmallRng, Rng};

    #[test]
    fn test_manual_train() {
        let mut network = (
            layers::Dense::<f32, 1, 2>::default(),
            layers::Dense::<f32, 2, 2>::default(),
        );
        let mut updater = network.new_momentum(TrainParams::with_lr(3.0e-8).and_l1(1.0e-5), 0.2);
        let mut rng = SmallRng::seed_from_u64(42);
        network.rand_params(&mut rng, 0.5).unwrap();

        for _i in 0..1900 {
            let input = rng.random_range(-20.0..20.0);
            let target = [-input, input];
            let (out, trace) = network.traced_forward([input]).unwrap();
            let loss = out.mse(&target);

            // NOTE: we should use the gradients WRT the loss as the input to
            // backprop, not the target.
            let (_, mut gradient_updates) = network.backprop(&trace, target.clone());

            let gradient_updates = updater.adjust(gradient_updates, -loss);
            network
                .update(&mut updater, gradient_updates)
                .expect("update failed");
        }

        let out = network.forward(&[1.0]).unwrap();
        let loss = out.mse(&[-1.0, 1.0]);
        println!("got={:?}, want={:?}: loss={}", out, [-1.0, 1.0], loss);
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

        let mut params = TrainParams::with_lr(1.0e-5);
        for _i in 0..200 {
            let input = rng.random_range(-20.0..20.0);
            let target = [-input, input];
            train_step(
                &mut params,
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
    fn test_train_step_momentum() {
        let mut network = (
            layers::Dense::<f32, 1, 2>::default(),
            layers::Dense::<f32, 2, 2>::default(),
        );
        let mut rng = SmallRng::seed_from_u64(765);
        network.rand_params(&mut rng, 0.1).unwrap();

        let mut updater = network.new_momentum(TrainParams::with_lr(1.0e-5), 0.4);
        for _i in 0..50 {
            let input = rng.random_range(-20.0..20.0);
            let target = [-input, input];
            train_step(
                &mut updater,
                &mut network,
                |got, want| (got.mse(want), got.mse_input_grads(want)),
                [input],
                target,
            );
        }

        let out = network.forward(&[1.0]).unwrap();
        let loss = out.mse(&[-1.0, 1.0]);
        println!("got={:?}, want={:?}: loss={}", out, [-1.0, 1.0], loss);
        assert!(loss < 0.1);
    }

    #[test]
    fn test_train_step_rmsprop() {
        let mut network = (
            layers::Dense::<f32, 1, 2>::default(),
            layers::Dense::<f32, 2, 2>::default(),
        );
        let mut rng = SmallRng::seed_from_u64(765);
        network.rand_params(&mut rng, 0.1).unwrap();

        let mut updater = network.new_rmsprop_with_momentum(TrainParams::with_lr(1.0e-4), 0.6, 0.5);
        for _i in 0..50 {
            let input = rng.random_range(-20.0..20.0);
            let target = [-input, input];
            train_step(
                &mut updater,
                &mut network,
                |got, want| (got.mse(want), got.mse_input_grads(want)),
                [input],
                target,
            );
        }

        let out = network.forward(&[1.0]).unwrap();
        let loss = out.mse(&[-1.0, 1.0]);
        println!("got={:?}, want={:?}: loss={}", out, [-1.0, 1.0], loss);
        assert!(loss < 0.1);
    }

    #[test]
    fn test_train_step_softmax() {
        let mut network = (
            (
                layers::Dense::<f32, 1, 4>::default(),
                layers::Bias1d::<f32, 4>::default(),
                layers::Activation::Relu,
            ),
            (
                layers::Dense::<f32, 4, 2>::default(),
                layers::Bias1d::<f32, 2>::default(),
                layers::Activation::Relu,
            ),
            layers::Softmax::default(),
        );
        let mut rng = SmallRng::seed_from_u64(23423);
        network.rand_params(&mut rng, 0.5).unwrap();

        let func = |inp| {
            let r = if inp > 0.5 { 1.0 } else { 0.0 };
            [1.0 - r, r]
        };

        let mut params = TrainParams::with_lr(8.0e-2).and_l2(1.0e-6);
        for _i in 0..5000 {
            let input = rng.random_range(-2.0..2.0);
            let target = func(input);
            train_step(
                &mut params,
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

        let mut params = TrainParams::with_lr(8.0e-2);
        for _i in 0..3000 {
            let input = rng.random_range(-2.0..2.0);
            let target = func(input);
            train_step(
                &mut params,
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

    #[test]
    fn test_residual() {
        let mut network = (
            layers::Dense::<f32, 1, 2>::default(),
            layers::Dense::<f32, 2, 1>::default(),
            layers::Residual {
                module: (
                    layers::Dense::<f32, 1, 3>::default(),
                    layers::Dense::<f32, 3, 1>::default(),
                    layers::Bias1d::<f32, 1>::default(),
                ),
                ..layers::Residual::default()
            },
        );
        let mut rng = SmallRng::seed_from_u64(23432);
        network.rand_params(&mut rng, 1.0).unwrap();

        let func = |inp| inp - 2.2;

        let mut updater = network.new_momentum(TrainParams::with_lr(1.0e-4), 0.3);
        for _i in 0..20000 {
            let input = rng.random_range(-5.0..5.0);
            let target = [func(input)];
            train_step(
                &mut updater,
                &mut network,
                |got, want| (got.mse(want), got.mse_input_grads(want)),
                [input],
                target,
            );
        }

        let inp = 3.2;
        let out = network.forward(&[inp]).unwrap();
        let loss = out.mse(&[func(inp)]);
        println!("got={:?}, want={:?}: loss={}", out, func(inp), loss);
        assert!(loss < 0.1);
    }

    // NOTE: Seems to have issues with smashing the stack.
    #[test]
    fn test_glu() {
        let mut network = (
            layers::Dense::<f32, 1, 3>::default(),
            layers::Bias1d::<f32, 3>::default(),
            layers::Activation::LeakyRelu(0.5),
            layers::GLU::<f32, 3, 4>::default(),
            layers::Dense::<f32, 4, 1>::default(),
            layers::Bias1d::<f32, 1>::default(),
        );
        let mut rng = SmallRng::seed_from_u64(43);
        network.rand_params(&mut rng, 0.2).unwrap();

        let func = |inp| inp - 2.2;

        let mut updater = network.new_momentum(TrainParams::with_lr(5.0e-3), 0.1);
        for _i in 0..1000 {
            let input = rng.random_range(-4.0..4.0);
            let target = [func(input)];
            train_step(
                &mut updater,
                &mut network,
                |got, want| (got.mse(want), got.mse_input_grads(want)),
                [input],
                target,
            );
        }

        let inp = 3.2;
        let out = network.forward(&[inp]).unwrap();
        let loss = out.mse(&[func(inp)]);
        println!("got={:?}, want={:?}: loss={}", out, func(inp), loss);
        assert!(loss < 0.15);
    }
}
