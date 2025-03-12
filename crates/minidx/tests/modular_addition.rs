use minidx::prelude::*;
use minidx::problem::Problem;
use rand::rngs::SmallRng;
use rand::SeedableRng;

#[test]
#[ignore]
fn integration_modular_addition10() {
    use minidx::problem::ModularAddition10;

    let network = (
        (
            layers::LRDiv::<f32, 20, 2, layers::Linear<20, 35>>::default(),
            layers::Sigmoid,
        ),
        layers::Linear::<35, 10>::default(),
        layers::Softmax::default(),
    );

    let mut nn = Buildable::<f32>::build(&network);

    let mut rng = SmallRng::seed_from_u64(456645);
    nn.rand_params(&mut rng, 1.0).unwrap();

    let mut problem = ModularAddition10::new(rng);

    use minidx_core::loss::LogitLoss;
    let mut updater = nn.new_rmsprop_with_momentum(TrainParams::with_lr(2.0e-2), 0.85, 0.8);
    for _i in 0..5000 {
        train_batch(
            &mut updater,
            &mut nn,
            |got, want| (got.logit_bce(want), got.logit_bce_input_grads(want)),
            &mut || problem.sample(),
            5,
        );
    }

    for _ in 0..30 {
        let (input, target) = problem.sample();
        let out = nn.forward(&input).unwrap();
        let loss = out.logit_bce(&target);
        println!(
            "input={:?}: got={:?}, want={:?}: loss={}",
            input, out, target, loss
        );
        assert!(loss < 0.1);
    }
}

#[test]
#[ignore]
fn integration_modular_addition32() {
    use minidx::problem::ModularAddition32;

    let network = (
        (layers::Linear::<64, 64> {}, layers::Sigmoid),
        layers::Linear::<64, 32> {},
        layers::Softmax::default(),
    );

    let mut nn = Buildable::<f32>::build(&network);

    let mut rng = SmallRng::seed_from_u64(345643);
    nn.rand_params(&mut rng, 1.0).unwrap();

    let mut problem = ModularAddition32::new(rng);

    use minidx_core::loss::LogitLoss;
    let mut updater = nn.new_rmsprop_with_momentum(TrainParams::with_lr(2.0e-2), 0.9, 0.9);
    for _i in 0..42000 {
        train_batch(
            &mut updater,
            &mut nn,
            |got, want| (got.logit_bce(want), got.logit_bce_input_grads(want)),
            &mut || problem.sample(),
            5,
        );
    }

    for _ in 0..30 {
        let (input, target) = problem.sample();
        let out = nn.forward(&input).unwrap();
        let loss = out.logit_bce(&target);
        println!(
            "input={:?}: got={:?}, want={:?}: loss={}",
            input, out, target, loss
        );
        assert!(loss < 0.1);
    }
}
