# minidx

[![Crates.io](https://img.shields.io/crates/v/minidx.svg)](https://crates.io/crates/minidx) [![minidx](https://docs.rs/minidx/badge.svg)](https://docs.rs/minidx)

A sized, zero allocation neural-network library with a focus on stable rust and portability.

Minidx started life as a loving, minimalist fork/rewrite of the incredible [dfdx](https://github.com/coreylowman/dfdx) by coreylowman et al,
but over time has diverged significantly.

Documentation: [here](https://docs.rs/minidx)

```rust
use minidx::prelude::*;
use layers::*;

// A neural network with 20 inputs and 10 outputs, with
// - A sigmoid-activated first layer with a hidden dimension of 35
// - A Softmax layer with a final dimension of 10
type network = (
    (Linear::<20, 35>, Sigmoid),
    Linear::<35, 10>,
    Softmax,
);

// instantiate the neural network
let mut nn = Buildable::<f32>::build(&network::default());

// randomly initialize with a fixed seed
let mut rng = SmallRng::seed_from_u64(456645);
nn.rand_params(&mut rng, 1.0).unwrap();

// sample input/output pairs from the ModularAddition10 type (which is a toy supervised-learning source)
let mut problem = ModularAddition10::new(rng);

use loss::LogitLoss;
// setup an rmsprop-with-momentum optimizer, with specified learning rate, momentum, and beta
let mut updater = nn.new_rmsprop_with_momentum(TrainParams::with_lr(2.0e-2), 0.85, 0.8);
// train the network over 5000 update steps, using binary cross-entropy loss and a minibatch size of 10.
for _i in 0..5000 {
    train_batch(
        &mut updater,
        &mut nn,
        |got, want| (got.logit_bce(want), got.logit_bce_input_grads(want)), // returns the loss for a sample, and its gradients WRT loss
        &mut || problem.sample(), // returns input-output pairs
        10,
    );
}
```


Equivalently licensed, AKA MIT OR Apache.

## Goals

 - Predominately run on the CPU
 - Only use Stable rust features
 - Correctness through testing