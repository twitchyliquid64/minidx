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
        (layers::Linear::<20, 30> {}, layers::Sigmoid),
        (layers::Linear::<30, 15> {}, layers::Swish),
        (layers::Linear::<15, 10> {}, layers::Swish),
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
        TrainParams::with_lr(2.0e-3)
            .and_l2(1.0e-6)
            .and_soft_start(500),
        0.8,
        0.95,
    );
    for i in 0..178000 {
        let batch_loss = train_batch(
            &mut updater,
            &mut nn,
            |got, want| (got.mse(want), got.mse_input_grads(want)),
            &mut || problem.sample(),
            20,
        );
        if i % 923 == 0 {
            let other_loss = problem.avg_loss(&mut nn, |got, want| got.mse(want), 10);
            recorder.push(i as f32, (other_loss + batch_loss) / 2.0, nn.clone());
        }
    }

    assert!(matches!(recorder.wait(), Some(anim::RecorderErr::Recv(_))));
}
