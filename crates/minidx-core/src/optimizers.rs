use crate::Gradients;
use num_traits::FromPrimitive;

/// An object responsible for tweaking gradient updates, such as to
/// add the effects of momentum, perform gradient clipping, etc.
pub trait GradAdjuster<G: Gradients> {
    fn adjust(&mut self, gradient_updates: G, loss: f32) -> G;
}

/// Describes the basic set of parameters used in training.
pub struct TrainParams {
    pub lr: f32,
}

impl Default for TrainParams {
    fn default() -> Self {
        Self { lr: 1.0e-8 }
    }
}

impl<G: Gradients> GradAdjuster<G> for TrainParams {
    fn adjust(&mut self, mut gradient_updates: G, loss: f32) -> G {
        let l = G::Concrete::from_f32(-loss * self.lr).unwrap();
        gradient_updates.grad_iter_mut().for_each(|g| *g *= l);
        gradient_updates
    }
}

/// Tracks momentum state across epochs during training.
pub struct Momentum<G: Gradients> {
    params: TrainParams,
    velocity: G,
    momentum_coeff: f32,
}

impl<G: Gradients> Momentum<G> {
    /// Constructs a new object with the given coefficient for momentum.
    ///
    /// Typical values are 0 to 1, with 0.5 being a good starting value.
    /// 0.9 is used by torch, but leads to oscillations with high learning
    /// rates or small batch sizes.
    pub fn new(params: TrainParams, momentum_coeff: f32) -> Momentum<G> {
        Self {
            params,
            momentum_coeff,
            velocity: G::empty(),
        }
    }

    pub(crate) fn update(&mut self, gradient_updates: G, loss: f32) -> G {
        let mc = G::Concrete::from_f32(self.momentum_coeff).unwrap();
        let loss = G::Concrete::from_f32(-loss * self.params.lr).unwrap();

        // v = coeff * last_v + gradient_updates
        self.velocity
            .grad_iter_mut()
            .zip(gradient_updates.into_grads())
            .for_each(|(v, g)| {
                *v = (*v * mc) + (g * loss);
            });

        self.velocity.clone()
    }
}

impl<G: Gradients> GradAdjuster<G> for Momentum<G> {
    fn adjust(&mut self, gradient_updates: G, loss: f32) -> G {
        self.update(gradient_updates, loss)
    }
}
