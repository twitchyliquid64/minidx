use minidx::prelude::*;
use minidx::problem::Problem;
use minidx_vis::prelude::*;
use rand::rngs::SmallRng;
use rand::SeedableRng;

// This test MUST be run in release mode if you want it to take 2 minutes instead of 5 hours lol.

#[test]
#[ignore]
fn vis_as_video() {
    rayon::ThreadPoolBuilder::new()
        .num_threads(4)
        .stack_size(12 * 1024 * 1024)
        .build_global()
        .unwrap();

    let network = (
        (layers::Linear::<20, 30> {}, layers::Swish::<30> {}),
        (layers::Linear::<30, 15> {}, layers::SiLU),
        (layers::Linear::<15, 10> {}, layers::SiLU),
        layers::Linear::<10, 10> {},
        layers::Softmax::default(),
    );

    let mut nn = Buildable::<f32>::build(&network);

    let mut rng = SmallRng::seed_from_u64(94356213);
    nn.rand_params(&mut rng, 0.8).unwrap();

    use minidx::problem::ModularAddition10;
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
    for i in 0..450000 {
        let batch_loss = train_batch_parallel(
            &mut updater,
            &mut nn,
            |got, want| (got.mse(want), got.mse_input_grads(want)),
            &mut || problem.sample(),
            20,
        );
        if i % 523 == 0 {
            let other_loss = problem.avg_loss(&mut nn, |got, want| got.mse(want), 5);
            recorder.push(i as f32, (other_loss + batch_loss) / 2.0, nn.clone());
        }
    }

    assert!(matches!(recorder.wait(), Some(anim::RecorderErr::Recv(_))));
}
