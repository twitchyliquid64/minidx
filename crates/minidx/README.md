# minidx

Minidx helps you implement small to medium-sized neural networks.

#### Defining network architecture

In minidx, you define your network using tuples of layers, with the
dimensionality of inputs/outputs defined as generic constants.
For instance, the below example defines a network which takes 2 inputs
and produces 3 outputs, by first going through two hidden layers with
a hidden dimension of 3 and a relu activation, before a softmax layer.

```rust
use minidx::prelude::*;
use layers::*;

type network = (
  (Linear::<2, 3>, Relu), // Fully-connected + bias layer with relu activation
  (Linear::<3, 3>, Relu),
  Softmax,
);

// Instantiates our neural network.
let mut network = Buildable::<f32>::build(&network::default());
```

You can see the full set of implemented layers in the [layer_spec] module.

#### Random initialization of a network

Before training, you likely want to initialize the parameters of the network
to reasonable random values.

```rust
use rand::{SeedableRng, rngs::SmallRng};
let mut rng = SmallRng::seed_from_u64(42);
network.rand_params(&mut rng, 0.5).unwrap();
```

[`rand_params`](`core::ResetParams::rand_params`) performs sensible initialization of each layer using
the given RNG. The float argument represents the max magnitude of random parameters. `0.5` to `1.0` is a good starting parameter.

#### Training

Training a network in minidx requires two things:

 - An updater: some object that stores training state and implements
   the optimizer algorithm you want to use
 - A training loop: a loop where you call [train_step] or [train_batch]
   with network inputs and their correct outputs, and a closure that wires up
   the loss function you want to use.

```rust
// initialize training state
let mut updater = network.new_momentum(
    TrainParams::with_lr(1.0e-5).and_l2(1.0e-6), 0.4);

// train the network with 50 examples
for _i in 0..50 {
    // fake training data
    let input = [1.0, 2.0];
    let output = [1.0, 0.0, 0.0];
    // train on an individual input/output pair, using the
    // mean-square error (MSE) loss function.
    use loss::DiffLoss;
    train_step(
        &mut updater,
        &mut network,
        |got, want| (got.mse(want), got.mse_input_grads(want)),
        input,
        output,
    );
}
```

Everything is fairly self-explanatory except for the closure you need to pass for your loss function.
That function takes both the output of the network as well as the correct output of the network, and
needs to return the loss with respect to the output as well as the gradient of the loss with respect
to the loss function. The [minidx::prelude::loss] module contains implemented loss functions and
corresponding methods to compute their gradients.

Its also worth noting that there are batch and threaded-batch variants of [train_step], namely [train_batch]
and [train_batch_parallel]. Both batch training methods return the average loss over the samples.

#### Inference

You can run inference over a trained network using [`forward()`](`core::Module::forward`):

```rust
let output = network.forward(&[1.0, 2.0]).unwrap(); // outputs [f32; 3]
```

Networks can be loaded and stored using [`LoadableModule`](core::LoadableModule).

License: MIT OR Apache-2.0
