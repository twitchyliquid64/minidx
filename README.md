# minidx

A sized, zero allocation neural-network library with a focus on stable rust and portability.

Minidx started life as a loving, minimalist fork/rewrite of the incredible [dfdx](https://github.com/coreylowman/dfdx) by coreylowman et al,
but over time has diverged significantly.

```rust
let network = (
    (layers::Linear::<20, 35> {}, layers::Sigmoid),
    layers::Linear::<35, 10> {},
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
```


Equivalently licensed, AKA MIT OR Apache.

## Goals

 - Predominately run on the CPU
 - Only use Stable rust features
 - Correctness through testing
 - (stretch goal) Support inference in no-std environments
 - (stretch goal) Generate pytorch code for training
