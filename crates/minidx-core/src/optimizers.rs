use crate::misc::Decay;
use crate::{Dtype, Float, Gradients, Unit};
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
    pub lr: Decay,
    pub l1_reg: f32,
    pub l2_reg: f32,

    pub soft_start_epochs: Option<usize>,

    pub grad_clip: Option<f32>,

    step: usize,
}

impl Default for TrainParams {
    fn default() -> Self {
        Self {
            lr: Decay::None(1.0e-6),
            l1_reg: 0.0,
            l2_reg: 0.0,
            soft_start_epochs: None,
            grad_clip: None,
            step: 0,
        }
    }
}

impl TrainParams {
    /// Constructs an object with the given training rate.
    pub fn with_lr(lr: f32) -> Self {
        Self {
            lr: Decay::None(lr),
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

    /// Sets the learning rate (linear) decay, keeping all other parameters unaffected.
    ///
    /// This method can be chained in a builder-pattern kind of way.
    pub fn and_lr_decay(mut self, lr_decay: f32) -> Self {
        self.lr = Decay::Linear {
            start: self.lr.start_value(),
            decay: lr_decay,
        };
        self
    }

    /// Sets the learning rate (cosine) decay, keeping all other parameters unaffected.
    ///
    /// This method can be chained in a builder-pattern kind of way.
    pub fn and_lr_cosine_decay(mut self, final_lr: f32, over_steps: usize) -> Self {
        self.lr = Decay::Cosine {
            start: self.lr.start_value(),
            end: final_lr,
            num_steps: over_steps,
        };
        self
    }

    /// Sets the number of epochs the lr should linearly ramp up to the LR at the start of training.
    ///
    /// This method can be chained in a builder-pattern kind of way.
    pub fn and_soft_start(mut self, epochs: usize) -> Self {
        self.soft_start_epochs = Some(epochs);
        self
    }

    /// Sets the maximum magnitude a single gradient can be. Any gradient larger in magnitude is
    /// limited to this magnitude.
    pub fn and_gradient_clip(mut self, clip: f32) -> Self {
        self.grad_clip = Some(clip);
        self
    }

    fn current_lr(&self) -> f32 {
        let lr = self.lr.at_timestep(self.step);
        match self.soft_start_epochs {
            Some(soft_start_epochs) => {
                if self.step < soft_start_epochs {
                    lr / soft_start_epochs as f32 * (1 + self.step) as f32
                } else {
                    lr
                }
            }
            None => lr,
        }
    }

    fn clip_grad<G: Dtype>(&self, grad: G) -> G {
        if let Some(clip) = self.grad_clip {
            let clip = G::from_f32(clip).unwrap();
            let neg_clip = G::default() - clip;
            if grad > clip {
                clip
            } else if grad < neg_clip {
                neg_clip
            } else {
                grad
            }
        } else {
            grad
        }
    }
}

impl<G: Gradients> GradAdjuster<G> for TrainParams {
    fn adjust(&mut self, mut gradient_updates: G, loss: f32) -> G {
        let l = G::Concrete::from_f32(-loss * self.current_lr()).unwrap();
        gradient_updates
            .grad_iter_mut()
            .for_each(|g| *g = self.clip_grad(*g) * l);
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

        self.step += 1;
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

    similarity_term: Option<f32>,
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
            similarity_term: None, // Typical value 0.5=>1.5,
        }
    }

    /// Sets the penalty term for updates that deviate (in cosine distance) from
    /// the velocity. Typical values are 0.5 to 1.5.
    pub fn and_similarity_penalty(mut self, similarity_term: f32) -> Self {
        self.similarity_term = Some(similarity_term);
        self
    }

    fn update(&mut self, gradient_updates: G, loss: f32) -> G {
        let mc = G::Concrete::from_f32(self.momentum_coeff).unwrap();
        let loss = G::Concrete::from_f32(-loss * self.params.current_lr()).unwrap();

        let sim_mul = G::Concrete::from_f32(if let Some(term) = self.similarity_term {
            self.velocity
                .cosine_similarity(&gradient_updates)
                .map(|x| (2.0 * (x + 1.0) + term).log2() + 0.25)
                .unwrap_or(1.0)
        } else {
            1.0
        })
        .unwrap();

        // v = coeff * last_v + gradient_updates
        self.velocity
            .grad_iter_mut()
            .zip(gradient_updates.into_grads())
            .for_each(|(v, g)| {
                *v = (*v * mc) + (self.params.clip_grad(g) * loss * sim_mul);
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

enum RMSPropBase<G: Gradients> {
    NoMomentum(TrainParams),
    Momentum(Momentum<G>),
}

impl<G: Gradients> GradAdjuster<G> for RMSPropBase<G> {
    fn adjust(&mut self, gradient_updates: G, loss: f32) -> G {
        use RMSPropBase::*;
        match self {
            NoMomentum(params) => params.adjust(gradient_updates, loss),
            Momentum(m) => m.adjust(gradient_updates, loss),
        }
    }
}

impl<G: Gradients> GradApplyer for RMSPropBase<G> {
    fn apply<G2: Gradients>(
        &mut self,
        gradient_updates: G2,
        weights: &mut G2,
    ) -> Result<(), crate::Error> {
        use RMSPropBase::*;
        match self {
            NoMomentum(params) => params.apply(gradient_updates, weights),
            Momentum(m) => m.apply(gradient_updates, weights),
        }
    }
}

/// Implements rmsprop on top of basic training parameters or [Momentum].
pub struct RMSProp<G: Gradients>
where
    G::Concrete: Float,
{
    base: RMSPropBase<G>,
    accumulator: G,
    beta: f32,
}

impl<G: Gradients> RMSProp<G>
where
    G::Concrete: Float,
{
    /// Constructs a new rmsprop optimizer.
    pub fn new(params: TrainParams, beta: f32) -> RMSProp<G> {
        Self {
            beta,
            base: RMSPropBase::NoMomentum(params),
            accumulator: G::empty(),
        }
    }

    /// Constructs a new rmsprop optimizer with momentum.
    pub fn new_with_momentum(params: TrainParams, momentum_coeff: f32, beta: f32) -> RMSProp<G> {
        Self {
            beta,
            base: RMSPropBase::Momentum(Momentum::new(params, momentum_coeff)),
            accumulator: G::empty(),
        }
    }

    /// Sets the penalty term for updates that deviate (in cosine distance) from
    /// the velocity. Typical values are 0.5 to 1.5.
    pub fn and_similarity_penalty(mut self, similarity_term: f32) -> Self {
        if let RMSPropBase::Momentum(m) = &mut self.base {
            m.similarity_term = Some(similarity_term);
        }
        self
    }
}

impl<G: Gradients> GradAdjuster<G> for RMSProp<G>
where
    G::Concrete: Float,
{
    fn adjust(&mut self, mut gradient_updates: G, loss: f32) -> G {
        let b = G::Concrete::from_f32(self.beta).unwrap();
        self.accumulator
            .grad_iter_mut()
            .zip(gradient_updates.grad_iter_mut())
            .for_each(|(a, u)| {
                let new_a = (*a * b) + (G::Concrete::ONE - b) * (*u) * (*u);
                *a = new_a;

                // rmsprop divides the learning rate by sqrt(accumulator + epsilon).
                // the learning rate will be multiplied in next, so we just apply it to
                // the whole gradient.
                *u /= (new_a + G::Concrete::SMOL).sqrt();
            });

        self.base.adjust(gradient_updates, loss)
    }
}

impl<G: Gradients> GradApplyer for RMSProp<G>
where
    G::Concrete: Float,
{
    fn apply<G2: Gradients>(
        &mut self,
        gradient_updates: G2,
        weights: &mut G2,
    ) -> Result<(), crate::Error> {
        self.base.apply(gradient_updates, weights)
    }
}
