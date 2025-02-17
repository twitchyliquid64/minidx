use minidx::prelude::*;
use minidx::problem::Problem;
use minidx_vis::prelude::*;
use rand::rngs::SmallRng;
use rand::SeedableRng;

#[test]
#[ignore]
fn vis_as_video() {
    use minidx::problem::AxPlusB;

    let network = (
        (layers::Linear::<3, 5> {}, layers::Relu),
        (layers::Linear::<5, 3> {}, layers::Relu),
        layers::Linear::<3, 1> {},
    );

    let mut nn = Buildable::<f32>::build(&network);

    let mut rng = SmallRng::seed_from_u64(94356213);
    nn.rand_params(&mut rng, 0.8).unwrap();

    let mut problem = AxPlusB::new(-3f32..3f32, rng);
    let mut recorder = anim::Recorder::mp4(
        "/tmp/test_vis_as_video.mp4",
        (1080, 720),
        ParamVisOpts::small(),
    );

    use minidx_core::loss::DiffLoss;
    let mut updater = nn.new_rmsprop_with_momentum(TrainParams::with_lr(1.0e-3), 0.5, 0.8);
    for i in 0..18000 {
        train_batch(
            &mut updater,
            &mut nn,
            |got, want| (got.mse(want), got.mse_input_grads(want)),
            &mut || problem.sample(),
            5,
        );
        if i % 80 == 0 {
            let (input, target) = problem.sample();
            let out = nn.forward(&input).unwrap();
            let loss = out.mse(&target);
            recorder.push(loss, nn.clone());
        }
    }

    assert!(matches!(recorder.wait(), Some(anim::RecorderErr::Recv(_))));
}
