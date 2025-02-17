//! Toy functions that neural networks can learn.

use minidx_core::Dtype;
use std::ops::Range;

/// A supervised learning scenario that can be sampled and
/// a network trained to predict.
pub trait Problem {
    type Input: Sized + std::fmt::Debug;
    type Output: Sized + std::fmt::Debug;

    fn sample(&mut self) -> (Self::Input, Self::Output);
}

/// For the problem Ax + B. Randomly generates A, x, and B uniformly in the
/// domain and the correct answer.
pub struct AxPlusB<E: Dtype, RNG: rand::Rng> {
    domain: Range<E>,
    rng: RNG,
}

impl<E: Dtype, RNG: rand::Rng> AxPlusB<E, RNG> {
    fn new(domain: Range<E>, rng: RNG) -> Self {
        Self { domain, rng }
    }
}

impl<E: Dtype + std::ops::Neg<Output = E>, RNG: rand::Rng> AxPlusB<E, RNG> {
    fn default_with_rng(rng: RNG) -> Self {
        Self {
            domain: Range {
                start: E::ONE.neg(),
                end: E::ONE,
            },
            rng,
        }
    }
}

impl<E: Dtype + rand::distr::uniform::SampleUniform, RNG: rand::Rng> Problem for AxPlusB<E, RNG> {
    type Input = [E; 3];
    type Output = [E; 1];

    fn sample(&mut self) -> (Self::Input, Self::Output) {
        let input = [
            self.rng.random_range(self.domain.clone()), // a
            self.rng.random_range(self.domain.clone()), // x
            self.rng.random_range(self.domain.clone()), // b
        ];

        let output = input[0] * input[1] + input[2];
        (input, [output])
    }
}

/// For the problem of computing the parity bit given N input bits.
pub struct Parity<E: Dtype, const N: usize, RNG: rand::Rng> {
    marker: std::marker::PhantomData<[E; N]>,
    rng: RNG,
}

impl<E: Dtype + rand::distr::uniform::SampleUniform, RNG: rand::Rng, const N: usize>
    Parity<E, N, RNG>
{
    pub fn new(rng: RNG) -> Self {
        Self {
            marker: Default::default(),
            rng,
        }
    }
}

impl<E: Dtype + rand::distr::uniform::SampleUniform, RNG: rand::Rng, const N: usize> Problem
    for Parity<E, N, RNG>
{
    type Input = [E; N];
    type Output = [E; 2];

    fn sample(&mut self) -> (Self::Input, Self::Output) {
        let mut output: bool = false;
        let input = core::array::from_fn(|_i| {
            let b = self.rng.random_bool(0.5);
            output ^= b;

            if b {
                E::ONE
            } else {
                E::default()
            }
        });

        let o = if output { E::ONE } else { E::default() };
        (input, [o, E::ONE - o])
    }
}

/// For the problem of adding two numbers mod 10 together, with inputs and outputs
/// represented with one-hot encodings.
pub struct ModularAddition10<E: Dtype, RNG: rand::Rng> {
    marker: std::marker::PhantomData<E>,
    rng: RNG,
}

impl<E: Dtype + rand::distr::uniform::SampleUniform, RNG: rand::Rng> ModularAddition10<E, RNG> {
    pub fn new(rng: RNG) -> Self {
        Self {
            marker: Default::default(),
            rng,
        }
    }
}

impl<E: Dtype + rand::distr::uniform::SampleUniform, RNG: rand::Rng> Problem
    for ModularAddition10<E, RNG>
{
    type Input = [E; 20];
    type Output = [E; 10];

    fn sample(&mut self) -> (Self::Input, Self::Output) {
        use crate::OneHotEncoder;

        let (lhs, rhs) = (
            (self.rng.next_u32() % 10) as usize,
            (self.rng.next_u32() % 10) as usize,
        );
        let output = (lhs + rhs) % 10;

        let mut input = [E::default(); 20];
        for (out, s) in input.iter_mut().zip(
            OneHotEncoder::<10>::value(lhs)
                .into_iter()
                .chain(OneHotEncoder::<10>::value(rhs).into_iter()),
        ) {
            *out = s;
        }

        (input, OneHotEncoder::<10>::value(output))
    }
}

/// For the problem of adding two numbers mod 32 together, with inputs and outputs
/// represented with one-hot encodings.
pub struct ModularAddition32<E: Dtype, RNG: rand::Rng> {
    marker: std::marker::PhantomData<E>,
    rng: RNG,
}

impl<E: Dtype + rand::distr::uniform::SampleUniform, RNG: rand::Rng> ModularAddition32<E, RNG> {
    pub fn new(rng: RNG) -> Self {
        Self {
            marker: Default::default(),
            rng,
        }
    }
}

impl<E: Dtype + rand::distr::uniform::SampleUniform, RNG: rand::Rng> Problem
    for ModularAddition32<E, RNG>
{
    type Input = [E; 64];
    type Output = [E; 32];

    fn sample(&mut self) -> (Self::Input, Self::Output) {
        use crate::OneHotEncoder;

        let (lhs, rhs) = (
            (self.rng.next_u32() % 32) as usize,
            (self.rng.next_u32() % 32) as usize,
        );
        let output = (lhs + rhs) % 32;

        let mut input = [E::default(); 64];
        for (out, s) in input.iter_mut().zip(
            OneHotEncoder::<32>::value(lhs)
                .into_iter()
                .chain(OneHotEncoder::<32>::value(rhs).into_iter()),
        ) {
            *out = s;
        }

        (input, OneHotEncoder::<32>::value(output))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::prelude::*;
    use rand::rngs::SmallRng;
    use rand::SeedableRng;

    #[test]
    fn test_ax_plus_b() {
        let network = (
            (layers::Linear::<3, 15> {}, layers::Relu),
            (layers::Linear::<15, 5> {}, layers::Relu),
            (layers::Linear::<5, 1> {}),
        );

        use crate::Buildable;
        let mut nn = Buildable::<f32>::build(&network);

        let mut rng = SmallRng::seed_from_u64(45645);
        nn.rand_params(&mut rng, 0.5).unwrap();

        let mut problem = AxPlusB::default_with_rng(rng);

        use minidx_core::loss::DiffLoss;
        let mut updater = nn.new_rmsprop_with_momentum(TrainParams::with_lr(5.0e-3), 0.5, 0.8);
        for _i in 0..4500 {
            let (input, target) = problem.sample();
            train_step(
                &mut updater,
                &mut nn,
                |got, want| (got.mse(want), got.mse_input_grads(want)),
                input,
                target,
            );
        }

        for _ in 0..10 {
            let (input, target) = problem.sample();
            let out = nn.forward(&input).unwrap();
            let loss = out.mse(&target);
            println!(
                "input={:?}: got={:?}, want={:?}: loss={}",
                input, out, target, loss
            );
            assert!(loss < 0.1);
        }
    }

    #[test]
    fn test_parity() {
        let network = (
            (layers::Linear::<4, 12> {}, layers::Sigmoid),
            layers::Linear::<12, 2> {},
            layers::Softmax::default(),
        );

        use crate::Buildable;
        let mut nn = Buildable::<f32>::build(&network);

        let mut rng = SmallRng::seed_from_u64(546);
        nn.rand_params(&mut rng, 0.5).unwrap();

        let mut problem = Parity::new(rng);

        use minidx_core::loss::LogitLoss;
        let mut updater = nn.new_rmsprop_with_momentum(TrainParams::with_lr(2.0e-2), 0.85, 0.7);
        for _i in 0..2500 {
            train_batch(
                &mut updater,
                &mut nn,
                |got, want| (got.logit_bce(want), got.logit_bce_input_grads(want)),
                &mut || problem.sample(),
                5,
            );
        }

        for _ in 0..10 {
            let (input, target) = problem.sample();
            let out = nn.forward(&input).unwrap();
            let loss = out.logit_bce(&target);
            println!(
                "input={:?}: got={:?}, want={:?}: loss={}",
                input, out, target, loss
            );
            assert!(loss < 0.3);
        }
    }
}
