use minidx::prelude::*;
use minidx::problem::Problem;
use rand::rngs::SmallRng;
use rand::SeedableRng;
use std::env;
use std::fs::File;

// RUST_MIN_STACK=104857600 cargo test --release -- --nocapture --include-ignored mnist_network

#[test]
#[ignore]
fn mnist_network() {
    rayon::ThreadPoolBuilder::new()
        .num_threads(4)
        .stack_size(196 * 1024 * 1024)
        .build_global()
        .unwrap();

    use minidx::problem::mnist;
    let img_file = File::open(
        env::var("MNIST_TRAIN_IMG_PATH").unwrap_or("/tmp/train-images-idx3-ubyte".into()),
    )
    .unwrap();
    let labels_file = File::open(
        env::var("MNIST_TRAIN_LABELS_PATH").unwrap_or("/tmp/train-labels-idx1-ubyte".into()),
    )
    .unwrap();
    const INPUT_DIMS: usize = 28 * 28;
    let mut p: mnist::ImgClassification<f32, SmallRng, INPUT_DIMS, 10> =
        mnist::ImgClassification::from_files(
            SmallRng::seed_from_u64(6542453345876),
            img_file,
            labels_file,
        )
        .unwrap();

    let network = (
        (
            layers::Linear::<INPUT_DIMS, 140>::default(),
            layers::Swish::<140> {},
            layers::DyT::<140> {},
        ),
        (
            layers::Linear::<140, 80>::default(),
            layers::Swish::<80> {},
            layers::DyT::<80> {},
        ),
        (layers::GLU::<80, 40>::default(), layers::Swish::<40> {}),
        layers::Linear::<40, 10>::default(),
        layers::Softmax::default(),
    );

    let mut nn = Buildable::<f32>::build(&network);

    let mut rng = SmallRng::seed_from_u64(4353);
    nn.rand_params(&mut rng, 0.1).unwrap();

    use minidx_core::loss::LogitLoss;
    let mut updater = nn.new_rmsprop_with_momentum(
        TrainParams::with_lr(4.0e-3)
            .and_l2(0.5e-6)
            .and_soft_start(100)
            .and_lr_cosine_decay(1.0e-3, 40000),
        0.8,
        0.97,
    );
    for i in 0..70000 {
        let avg_loss = train_batch_parallel(
            &mut updater,
            &mut nn,
            |got, want| (got.logit_bce(want), got.logit_bce_input_grads(want)),
            &mut || p.sample(),
            match i {
                0..2000 => 64,
                2000..6000 => 48,
                6000..15000 => 32,
                15000..30000 => 24,
                _ => 20,
            },
        );
        if i % 20 == 0 {
            println!(
                "{:05}: lr={:.5}, loss={:.4}",
                i,
                updater.train_params().current_lr(),
                avg_loss
            );
        }
    }

    for _ in 0..15 {
        let (input, target) = p.sample();
        let out = nn.forward(&input).unwrap();
        let loss = out.logit_bce(&target);
        println!("got={:?}, want={:?}: loss={}", out, target, loss);
        assert!(loss < 0.1);
    }
}
