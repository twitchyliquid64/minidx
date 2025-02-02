# minidx

A sized, zero allocation neural-network library with a focus on stable rust and portability.

Minidx started life as a loving, minimalist fork/rewrite of the incredible [dfdx](https://github.com/coreylowman/dfdx) by coreylowman et al,
but over time has diverged significantly.

Equivalently licensed, AKA MIT OR Apache.

## Goals

 - Predominately run on the CPU
 - Only use Stable rust features
 - Correctness through testing
 - (stretch goal) Support inference in no-std environments
 - (stretch goal) Generate pytorch code for training

## Biggggg TODO

 - Updater trait (for how gradient updates are applied)
 - Viz crate (for visualizing training runs)
 - Batch support
 - Momentum and other optimizers