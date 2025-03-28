//! The core types and logic implementing the `minidx` crate.
mod dtypes;
pub use dtypes::{Dtype, Float, Unit};
pub mod shapes;
pub use shapes::*;

#[doc(hidden)]
pub mod matmul;

pub mod misc;

mod iterate;
// pub(crate) use iterate::*;

pub mod gradients;
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
    ga.advance_step();
}

/// Does a training minibatch, updating a network based on averaged gradients from
/// computing N input-output pairs.
///
/// The average loss over all samples in the batch is returned.
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
) -> f32
where
    // Network::Output: std::fmt::Debug,
    LV: std::ops::Mul<f32, Output = f32>,
    <Network as modules::BackpropModule<Input>>::SelfGrads: Gradients,
{
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
    let lv = lv.to_f32().unwrap() * (batch_size as f32).recip();

    let gradient_updates = ga.adjust(grads, lv);
    network.update(ga, gradient_updates).expect("update failed");
    ga.advance_step();
    lv
}

/// Parallel version of [train_batch]. More threads is not necessarily faster.
///
/// The average loss over all samples in the batch is returned.
pub fn train_batch_parallel<
    Input,
    LV: Float,
    Network: BackpropModule<Input> + Sized + Sync + Send,
    GA: GradAdjuster<Network::SelfGrads> + GradApplyer,
    S: FnMut() -> (Input, Network::Output),
>(
    ga: &mut GA,
    network: &mut Network,
    loss: impl Fn(&Network::Output, &Network::Output) -> (LV, Network::Output) + Sync,
    source: &mut S,
    batch_size: usize,
) -> f32
where
    // Network::Output: std::fmt::Debug,
    LV: std::ops::Mul<f32, Output = f32>,
    (Input, Network::Output): Sized + Sync + Send,
    <Network as modules::BackpropModule<Input>>::SelfGrads: Gradients + Sync + Send,
{
    use rayon::prelude::*;

    let batch: Vec<_> = (0..batch_size).map(|_| source()).collect();
    let (mut grads, lv) = batch
        .into_par_iter()
        .map(|sample| {
            let (input, output) = sample;
            let (out, trace) = network.traced_forward(input).unwrap();
            let (lv, loss_grads) = loss(&out, &output);

            let (_, gradient_updates) = network.backprop(&trace, loss_grads);

            (gradient_updates, lv)
        })
        .reduce_with(|(mut l_grads, mut l_lv), (r_grads, r_lv)| {
            l_grads.add(r_grads);
            l_lv += r_lv;
            (l_grads, l_lv)
        })
        .unwrap();

    grads.scale((batch_size as f32).recip());
    let lv = lv.to_f32().unwrap() * (batch_size as f32).recip();

    let gradient_updates = ga.adjust(grads, lv);
    network.update(ga, gradient_updates).expect("update failed");
    ga.advance_step();
    lv
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
            let (_, gradient_updates) = network.backprop(&trace, target.clone());

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
        for _i in 0..250 {
            let input = rng.random_range(-20.0..20.0);
            let target = [-input, input];
            train_step(
                &mut updater,
                &mut network,
                |got, want| (got.huber(1.6, want), got.huber_input_grads(1.6, want)),
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
                layers::Swish::<f32, 4>::default(),
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
                // Also double the learning rate of this layer, not for
                // any real reason
                layers::LR::<f32, 5, layers::Dense<f32, 5, 2>> {
                    update_multiplier: 2.0,
                    ..Default::default()
                },
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

    #[test]
    fn test_swiglu_and_train_batch_parallel() {
        let mut network = (
            layers::GLU::<f32, 1, 4, layers::Swish<f32, 4>>::default(),
            layers::Dense::<f32, 4, 1>::default(),
            layers::Bias1d::<f32, 1>::default(),
        );
        let mut rng = SmallRng::seed_from_u64(43);
        network.rand_params(&mut rng, 0.5).unwrap();

        let func = |inp| inp - 2.2;

        let mut updater = network.new_momentum(TrainParams::with_lr(5.0e-3), 0.1);
        for _i in 0..1000 {
            train_batch_parallel(
                &mut updater,
                &mut network,
                |got, want| (got.mse(want), got.mse_input_grads(want)),
                &mut || {
                    let input = rng.random_range(-4.0..4.0);
                    let target = [func(input)];
                    ([input], target)
                },
                50,
            );
        }

        let inp = 3.2;
        let out = network.forward(&[inp]).unwrap();
        let loss = out.mse(&[func(inp)]);
        println!("got={:?}, want={:?}: loss={}", out, func(inp), loss);
        assert!(loss < 0.15);
    }

    #[test]
    fn test_train_step_conv1d() {
        let mut network = (layers::Conv1d::<f32, 4, 2, Const<3>>::default(),);
        let mut rng = SmallRng::seed_from_u64(95334578);
        network.rand_params(&mut rng, 0.5).unwrap();

        // Function that sums each set of 3 elements, i.e. filter kernel = 1.0
        let func = |inp: [f32; 4]| [inp[0] + inp[1] + inp[2], inp[1] + inp[2] + inp[3]];

        let mut params = TrainParams::with_lr(4.0e-3);
        for _i in 0..3000 {
            let input = [
                rng.random_range(-2.0..2.0),
                rng.random_range(-2.0..2.0),
                rng.random_range(-2.0..2.0),
                rng.random_range(-2.0..2.0),
            ];
            let target = func(input);
            train_step(
                &mut params,
                &mut network,
                |got: &[f32; 2], want| (got.mse(want), got.mse_input_grads(want)),
                input,
                target,
            );
        }

        println!("network={:?}", network.0);
        let w = &network.0.weights;
        assert!((w[0] - 1.0).abs() < 0.1);
        assert!((w[1] - 1.0).abs() < 0.1);
        assert!((w[2] - 1.0).abs() < 0.1);
    }

    #[test]
    fn test_save() {
        let network = (
            layers::Dense::<f32, 2, 1>::default(),
            layers::Bias1d::<f32, 1>::default(),
            layers::GLU::<f32, 1, 1, layers::Swish<f32, 1>>::default(),
            layers::Softmax::default(),
        );

        let mut store = std::collections::HashMap::new();
        assert!(network.save("".into(), &mut store).is_ok());
        assert_eq!(store.get(".0").unwrap().len(), 2);
        assert_eq!(store.get(".1").unwrap().len(), 1);
        assert_eq!(store.get(".2.sig_connections").unwrap().len(), 1);
        assert_eq!(store.get(".2.sig_bias").unwrap().len(), 1);
        assert_eq!(store.get(".2.gate_connections").unwrap().len(), 1);
        assert_eq!(store.get(".2.gate_bias").unwrap().len(), 1);
        assert_eq!(store.get(".2.activation").unwrap().len(), 1);
    }

    #[test]
    fn test_load() {
        let mut network = (
            layers::Dense::<f32, 2, 1>::default(),
            layers::Bias1d::<f32, 1>::default(),
            layers::Swish::<f32, 1>::default(),
        );
        let mut rng = SmallRng::seed_from_u64(54645);
        network.rand_params(&mut rng, 999.9).unwrap();

        // OR gate
        let store = std::collections::HashMap::from_iter([
            (".0".into(), vec![1.0, 1.0]),
            (".1".into(), vec![0.0]),
            (".2".into(), vec![1.0e15]),
        ]);
        assert_eq!(network.load("".into(), &store), Ok(()));

        assert_eq!(network.forward(&[1.0, 0.0]).unwrap(), [1.0]);
        assert_eq!(network.forward(&[0.0, 1.0]).unwrap(), [1.0]);
        assert_eq!(network.forward(&[0.0, 0.0]).unwrap(), [0.0]);
    }

    #[test]
    fn test_scalar_scale_layer() {
        let mut network = (layers::ScalarScale::<f32>::default(),);
        let mut rng = SmallRng::seed_from_u64(95334578);
        network.rand_params(&mut rng, 0.5).unwrap();

        let func = |inp: [f32; 2]| [inp[0] / 5.0, inp[1] / 5.0];

        let mut params = TrainParams::with_lr(0.25);
        for _i in 0..100 {
            let input = [rng.random_range(-2.0..2.0), rng.random_range(-2.0..2.0)];
            let target = func(input);
            train_step(
                &mut params,
                &mut network,
                |got: &[f32; 2], want| (got.mse(want), got.mse_input_grads(want)),
                input,
                target,
            );
        }

        let w = &network.0.scale;
        assert!((w - 0.2).abs() < 0.1, "got {}, want 0.2", w);
    }

    #[test]
    fn test_diag_layer() {
        let mut network = (layers::Diag::<f32, 2>::default(),);
        let mut rng = SmallRng::seed_from_u64(95334578);
        network.rand_params(&mut rng, 0.5).unwrap();

        let func = |inp: [f32; 2]| [inp[0] / 3.6, inp[1] * -1.8];

        let mut params = TrainParams::with_lr(5.0e-2);
        for _i in 0..500 {
            let input = [rng.random_range(-2.0..2.0), rng.random_range(-2.0..2.0)];
            let target = func(input);
            train_step(
                &mut params,
                &mut network,
                |got: &[f32; 2], want| (got.mse(want), got.mse_input_grads(want)),
                input,
                target,
            );
        }

        let w = &network.0.weights;
        assert!((w[0] - 0.277).abs() < 0.1, "got {}, want 0.277", w[0]);
        assert!((w[1] - -1.8).abs() < 0.1, "got {}, want 1.8", w[1]);
    }
}
