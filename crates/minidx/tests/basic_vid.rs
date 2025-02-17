use minidx::prelude::*;
use minidx::problem::Problem;
use minidx_vis::prelude::*;
use rand::rngs::SmallRng;
use rand::SeedableRng;

// This test MUST be run in release mode if you want it to take 2 minutes instead of 5 hours lol.

#[test]
#[ignore]
fn vis_as_video() {
    use minidx::problem::ModularAddition10;

    let network = (
        (layers::Linear::<20, 23> {}, layers::Sigmoid),
        (layers::Linear::<23, 15> {}, layers::LeakyRelu(0.05)),
        (layers::Linear::<15, 10> {}, layers::Relu),
        layers::Linear::<10, 10> {},
        layers::Softmax::default(),
    );

    let mut nn = Buildable::<f32>::build(&network);

    let mut rng = SmallRng::seed_from_u64(94356213);
    nn.rand_params(&mut rng, 0.8).unwrap();

    let mut problem = ModularAddition10::new(rng);
    let mut recorder = anim::Recorder::mp4(
        "/tmp/test_vis_as_video.mp4",
        (1080, 720),
        ParamVisOpts::small(),
    );

    use minidx_core::loss::DiffLoss;
    let mut updater = nn.new_rmsprop_with_momentum(
        TrainParams::with_lr(4.0e-3)
            .and_l2(1.0e-6)
            .and_lr_decay(1.5e-10),
        0.6,
        0.9,
    );
    for i in 0..578000 {
        train_batch(
            &mut updater,
            &mut nn,
            |got, want| (got.mse(want), got.mse_input_grads(want)),
            &mut || problem.sample(),
            20,
        );
        if i % 450 == 0 {
            let (input, target) = problem.sample();
            let out = nn.forward(&input).unwrap();
            let loss = out.mse(&target);
            recorder.push(i as f32, loss, nn.clone());
        }
    }

    assert!(matches!(recorder.wait(), Some(anim::RecorderErr::Recv(_))));
}
