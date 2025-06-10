//! Composable neural-network layers

mod activation;
pub(crate) use activation::sigmoid;
pub use activation::Activation;
mod bias1d;
pub use bias1d::Bias1d;
mod linear;
pub use linear::Dense;
mod softmax;
pub use softmax::Softmax;
mod swish;
pub use swish::Swish;
mod residual;
pub use residual::Residual;
mod gate;
pub use gate::GLU;

mod conv1d;
pub use conv1d::{Conv1d, Conv1dKernel};

mod lr_modifier;
pub use lr_modifier::LR;

mod diag;
pub use diag::Diag;
mod scalar_scale;
pub use scalar_scale::ScalarScale;

mod rmsdiv;
pub use rmsdiv::RMSDiv;
