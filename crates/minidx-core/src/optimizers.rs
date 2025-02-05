use crate::{Gradients, Unit};
use num_traits::FromPrimitive;

/// An object responsible for tweaking gradient updates, such as to
/// add the effects of momentum, perform gradient clipping, etc.
pub trait GradAdjuster<G: Gradients> {
    fn adjust(&mut self, gradient_updates: G, loss: f32) -> G;
}

/// An object responsible for applying gradient updates, such as to
/// add the gradient updates to the weights while performing regularization.
pub trait GradApplyer {
    fn apply<G: Gradients>(
        &mut self,
        gradient_updates: G,
        weights: &mut G,
    ) -> Result<(), crate::Error>;
}

/// Describes the basic set of parameters used in training. Implements
/// optimizer traits, so it can be passed into training methods.
pub struct TrainParams {
    pub lr: f32,
    pub l1_reg: f32,
    pub l2_reg: f32,
}

impl Default for TrainParams {
    fn default() -> Self {
        Self {
            lr: 1.0e-8,
            l1_reg: 0.0,
            l2_reg: 0.0,
        }
    }
}

impl TrainParams {
    /// Constructs an object with the given training rate.
    pub fn with_lr(lr: f32) -> Self {
        Self {
            lr,
            ..Default::default()
        }
    }

    /// Sets the l1 regularization weight, keeping all other parameters unaffected.
    ///
    /// This method can be chained in a builder-pattern kind of way.
    pub fn and_l1(mut self, l1: f32) -> Self {
        self.l1_reg = l1;
        self
    }

    /// Sets the l2 regularization weight, keeping all other parameters unaffected.
    ///
    /// This method can be chained in a builder-pattern kind of way.
    pub fn and_l2(mut self, l2: f32) -> Self {
        self.l2_reg = l2;
        self
    }
}

impl<G: Gradients> GradAdjuster<G> for TrainParams {
    fn adjust(&mut self, mut gradient_updates: G, loss: f32) -> G {
        let l = G::Concrete::from_f32(-loss * self.lr).unwrap();
        gradient_updates.grad_iter_mut().for_each(|g| *g *= l);
        gradient_updates
    }
}

impl GradApplyer for TrainParams {
    fn apply<G: Gradients>(
        &mut self,
        gradient_updates: G,
        weights: &mut G,
    ) -> Result<(), crate::Error> {
        weights
            .grad_iter_mut()
            .zip(gradient_updates.into_grads())
            .for_each(|(w, u)| {
                let reg_penalty = G::Concrete::from_f32(if self.l1_reg != 0.0 {
                    if *w > G::Concrete::default() {
                        self.l1_reg
                    } else if *w < G::Concrete::default() {
                        -self.l1_reg
                    } else {
                        0.0
                    }
                } else {
                    0.0
                })
                .unwrap()
                    + if self.l2_reg != 0.0 {
                        (*w) * (G::Concrete::ONE + G::Concrete::ONE)
                            * G::Concrete::from_f32(self.l2_reg).unwrap()
                    } else {
                        G::Concrete::default()
                    };

                *w += u - reg_penalty;
            });
        Ok(())
    }
}

/// Implements momentum computation in addition to the basics provided by [TrainParams].
///
/// Implements optimizer traits, so it can be passed into training methods.
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

    fn update(&mut self, gradient_updates: G, loss: f32) -> G {
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

impl<G: Gradients> GradApplyer for Momentum<G> {
    fn apply<G2: Gradients>(
        &mut self,
        gradient_updates: G2,
        weights: &mut G2,
    ) -> Result<(), crate::Error> {
        self.params.apply(gradient_updates, weights)
    }
}
