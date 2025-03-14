use crate::layers::{Activation, Bias1d, Dense};
use crate::matmul::MatMulImpl;
use crate::{Dtype, Float};

/// A gated linear unit, containing dense layers and bias for both the gate and gated connections.
#[derive(Clone, Debug, Default)]
pub struct GLU<
    E: Dtype + Float + MatMulImpl,
    const I: usize,
    const O: usize,
    A: crate::Module<[E; O], Output = [E; O]> + Default = Activation<E>,
> {
    gate_connections: Dense<E, I, O>,
    gate_bias: Bias1d<E, O>,

    sig_connections: Dense<E, I, O>,
    sig_bias: Bias1d<E, O>,

    activation: A,
}

impl<E: Dtype + Float + MatMulImpl, const I: usize, const O: usize> GLU<E, I, O, Activation<E>> {
    pub fn sigmoid() -> Self {
        Self {
            activation: Activation::<E>::Sigmoid,
            ..Self::default()
        }
    }
    pub fn relu() -> Self {
        Self {
            activation: Activation::<E>::Relu,
            ..Self::default()
        }
    }
    pub fn leaky_relu(a: f32) -> Self {
        Self {
            activation: Activation::<E>::LeakyRelu(E::from_f32(a).unwrap()),
            ..Self::default()
        }
    }
}

impl<
        E: Dtype + Float + MatMulImpl,
        const I: usize,
        const O: usize,
        A: crate::Module<[E; O], Output = [E; O]> + Default,
    > GLU<E, I, O, A>
{

    #[doc(hidden)]
    pub fn connection_params(&self) -> (&[[E; I]; O], &[E; O], &[[E; I]; O], &[E; O]) {
        (
            &self.gate_connections.weights,
            &self.gate_bias.bias,
            &self.sig_connections.weights,
            &self.sig_bias.bias,
        )
    }
}

impl<E: Dtype + Float + MatMulImpl, const I: usize, const O: usize>
    GLU<E, I, O, super::Swish<E, O>>
{
    pub fn swish() -> Self {
        Self::default()
    }
}

impl<
        E: Dtype + Float + MatMulImpl,
        const I: usize,
        const O: usize,
        A: crate::Module<[E; O], Output = [E; O]> + TracedModule<[E; O]> + Default,
    > crate::Module<[E; I]> for GLU<E, I, O, A>
{
    type Output = [E; O];

    fn forward(&self, x: &[E; I]) -> Result<Self::Output, crate::Error> {
        let gates = self.gate_bias.forward(&self.gate_connections.forward(x)?)?;
        let mut gates = self.activation.forward(&gates)?;
        let sigs = self.sig_bias.forward(&self.sig_connections.forward(x)?)?;

        gates
            .iter_mut()
            .zip(sigs.into_iter())
            .for_each(|(o, s)| *o *= s);

        Ok(gates)
    }
}

use crate::TracedModule;

impl<
        E: Dtype + Float + MatMulImpl,
        const I: usize,
        const O: usize,
        A: crate::Module<[E; O], Output = [E; O]> + TracedModule<[E; O]> + Default,
    > TracedModule<[E; I]> for GLU<E, I, O, A>
{
    type Trace = (
        // gate
        (
            <Dense<E, I, O> as TracedModule<[E; I]>>::Trace,
            <Bias1d<E, O> as TracedModule<[E; O]>>::Trace,
            <A as TracedModule<[E; O]>>::Trace,
        ),
        // signal
        (
            <Dense<E, I, O> as TracedModule<[E; I]>>::Trace,
            <Bias1d<E, O> as TracedModule<[E; O]>>::Trace,
        ),
        // outputs for gate and signal
        ([E; O], [E; O]),
    );

    fn traced_forward(
        &self,
        x: [E; I],
    ) -> Result<(<Self as crate::Module<[E; I]>>::Output, Self::Trace), crate::Error> {
        let (gc_out, gc_trace) = self.gate_connections.traced_forward(x.clone())?;
        let (gb_out, gb_trace) = self.gate_bias.traced_forward(gc_out)?;
        let (ga_out, ga_trace) = self.activation.traced_forward(gb_out)?;

        let (sc_out, sc_trace) = self.sig_connections.traced_forward(x)?;
        let (sb_out, sb_trace) = self.sig_bias.traced_forward(sc_out)?;

        let mut out = ga_out.clone();
        out.iter_mut()
            .zip(sb_out.iter())
            .for_each(|(o, s)| *o *= *s);

        Ok((
            out,
            (
                (gc_trace, gb_trace, ga_trace),
                (sc_trace, sb_trace),
                (ga_out, sb_out),
            ),
        ))
    }
}

use crate::BackpropModule;

impl<
        E: Dtype + Float + MatMulImpl,
        const I: usize,
        const O: usize,
        A: crate::Module<[E; O], Output = [E; O]>
            + TracedModule<[E; O]>
            + BackpropModule<[E; O]>
            + Default,
    > crate::BackpropModule<[E; I]> for GLU<E, I, O, A>
{
    type SelfGrads = (
        // gate
        (
            <Dense<E, I, O> as BackpropModule<[E; I]>>::SelfGrads,
            <Bias1d<E, O> as BackpropModule<[E; O]>>::SelfGrads,
            <A as BackpropModule<[E; O]>>::SelfGrads,
        ),
        // signal
        (
            <Dense<E, I, O> as BackpropModule<[E; I]>>::SelfGrads,
            <Bias1d<E, O> as BackpropModule<[E; O]>>::SelfGrads,
        ),
    );

    fn backprop(
        &self,
        trace: &<Self as crate::TracedModule<[E; I]>>::Trace,
        grads_wrt_output: <Self as crate::Module<[E; I]>>::Output,
    ) -> ([E; I], Self::SelfGrads) {
        // grads for the signals side of the chain is our output grads * the output of the gate
        let mut sig_grads_wrt_output = grads_wrt_output.clone();
        sig_grads_wrt_output
            .iter_mut()
            .zip(trace.2 .0)
            .for_each(|(g, o)| *g *= o);
        // backprop for the signals chain
        let (sig_grads, sb_grads) = self.sig_bias.backprop(&trace.1 .1, sig_grads_wrt_output);
        let (sig_grads, sc_grads) = self.sig_connections.backprop(&trace.1 .0, sig_grads);

        // grads for the gate side of the chain is our output grads * the output of the signals
        let mut gate_grads_wrt_output = grads_wrt_output;
        gate_grads_wrt_output
            .iter_mut()
            .zip(trace.2 .1)
            .for_each(|(g, o)| *g *= o);

        // backprop for the signals chain
        let (gate_grads, ga_grads) = self.activation.backprop(&trace.0 .2, gate_grads_wrt_output);
        let (gate_grads, gb_grads) = self.gate_bias.backprop(&trace.0 .1, gate_grads);
        let (gate_grads, gc_grads) = self.gate_connections.backprop(&trace.0 .0, gate_grads);

        // output derivatives is just the sum of the partial derivatives from each chain
        // TODO: not sure about this
        let mut out = sig_grads;
        out.iter_mut()
            .zip(gate_grads.into_iter())
            .for_each(|(o, x)| *o += x);

        (out, ((gc_grads, gb_grads, ga_grads), (sc_grads, sb_grads)))
    }

    fn update(
        &mut self,
        applyer: &mut impl crate::optimizers::GradApplyer,
        updates: Self::SelfGrads,
    ) -> Result<(), crate::Error> {
        self.gate_connections.update(applyer, updates.0 .0)?;
        self.gate_bias.update(applyer, updates.0 .1)?;
        self.activation.update(applyer, updates.0 .2)?;

        self.sig_connections.update(applyer, updates.1 .0)?;
        self.sig_bias.update(applyer, updates.1 .1)?;

        Ok(())
    }
}

impl<
        E: Dtype + Float + MatMulImpl,
        const I: usize,
        const O: usize,
        A: crate::Module<[E; O], Output = [E; O]> + Default + crate::ResetParams,
    > crate::ResetParams for GLU<E, I, O, A>
{
    fn rand_params<RNG: rand::Rng>(
        &mut self,
        rng: &mut RNG,
        scale: f32,
    ) -> Result<(), crate::Error> {
        self.gate_connections.rand_params(rng, scale)?;
        self.gate_bias.rand_params(rng, scale)?;
        self.sig_connections.rand_params(rng, scale)?;
        self.sig_bias.rand_params(rng, scale)?;
        self.activation.rand_params(rng, scale)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Module;

    #[test]
    fn test_forward() {
        let mut g = GLU::<f32, 1, 1>::default();
        g.gate_connections.weights[0] = [1.0];
        g.sig_connections.weights[0] = [1.0];

        assert_eq!(g.forward(&[1.0]), Ok([1.0]));

        g.activation = Activation::Sigmoid;
        // o = 0.5 * sigmoid(0.5) - which itself is 0.62...
        let o = g.forward(&[0.5]).unwrap()[0];
        assert!(o > 0.3);
        assert!(o < 0.35);

        let mut g = GLU::<f32, 2, 1>::default();
        g.gate_connections.weights[0] = [1.0, 0.0];
        g.sig_connections.weights[0] = [0.0, 1.0];

        assert_eq!(g.forward(&[1.0, 1.0]), Ok([1.0]));
        assert_eq!(g.forward(&[1.0, 0.5]), Ok([0.5]));
        assert_eq!(g.forward(&[0.2, 1.0]), Ok([0.2]));
        assert_eq!(g.forward(&[-1.0, 1.0]), Ok([0.0])); // relu on gate

        assert_eq!(g.traced_forward([1.0, 0.5]).unwrap().0, [0.5]);
        assert_eq!(g.traced_forward([-1.0, 1.0]).unwrap().0, [0.0]); // relu on gate
    }

    #[test]
    fn test_backward_simple() {
        let mut g = GLU::<f32, 2, 1>::default();
        g.gate_connections.weights = [[2.0, 1.0]];
        g.gate_bias.bias[0] = -2.0;
        g.sig_connections.weights = [[1.0, -1.0]];
        g.sig_bias.bias[0] = 1.0;

        let (out, trace) = g.traced_forward([1.0, 2.0]).unwrap();
        assert_eq!(out, [0.0]);

        // trace.2 = (gate_out, sig_out)
        assert_eq!(trace.2, ([2.0], [0.0]));

        // assume MSE loss, want=1 so loss=0.5 and dL/dY = -1.
        let (_out_grads, grads) = g.backprop(&trace, [-1.0]);
        assert_eq!(out, [0.0]); // no gradient backwards as the gate was inactive
        assert_eq!(grads.0, ([[0.0, 0.0]], [0.0], ())); // no gradient for gate as sig was inactive
        assert_eq!(grads.1, ([[-2.0, -4.0]], [-2.0])); // activation was non-zero so gradients for gate
    }
}
