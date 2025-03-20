use minidx::prelude::*;
use minidx::problem::Problem;
use minidx::recorder::{BatchInfo, Every, Recorder};
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
    let mut recorder = Recorder::new()
        .save_to("/tmp/training.json")
        .snapshot_freq(Every::Seconds(15))
        .build()
        .unwrap();

    let network = (
        (layers::Linear::<20, 30> {}, layers::Swish::<30> {}),
        layers::DyT::<30> {},
        (layers::Linear::<30, 15> {}, layers::SiLU),
        (layers::Linear::<15, 10> {}, layers::SiLU),
        (layers::Linear::<10, 10> {}, layers::Softmax::default()),
    );

    let mut nn = Buildable::<f32>::build(&network);

    let mut rng = SmallRng::seed_from_u64(94356213);
    nn.rand_params(&mut rng, 0.8).unwrap();

    use minidx::problem::ModularAddition10;
    let mut problem = ModularAddition10::new(rng);
    let mut anim = anim::Recorder::mp4(
        "/tmp/test_vis_as_video.mp4",
        (1080, 720),
        ParamVisOpts::small(),
    );

    use minidx_core::loss::DiffLoss;
    let mut updater = nn.new_rmsprop_with_momentum(
        TrainParams::with_lr(3.0e-3)
            .and_l2(2.0e-6)
            .and_soft_start(500)
            .and_lr_cosine_decay(1.5e-3, 420000),
        0.8,
        0.95,
    );
    for i in 0..450000 {
        let start = std::time::Instant::now();
        let batch_loss = train_batch_parallel(
            &mut updater,
            &mut nn,
            |got, want| (got.mse(want), got.mse_input_grads(want)),
            &mut || problem.sample(),
            20,
        );
        if i % 523 == 0 {
            let other_loss = problem.avg_loss(&mut nn, |got, want| got.mse(want), 5);
            anim.push(i as f32, (other_loss + batch_loss) / 2.0, nn.clone());
        }

        recorder
            .record_batch(
                BatchInfo {
                    step: i,
                    loss: batch_loss as f64,
                    size: 20,
                    time_us: std::time::Instant::now().duration_since(start).as_micros() as u64,
                },
                updater.train_params(),
                &mut nn,
            )
            .unwrap();
    }

    assert!(matches!(anim.wait(), Some(anim::RecorderErr::Recv(_))));
}
