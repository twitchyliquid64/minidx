//! Composable neural-network layers

mod activation;
pub use activation::Activation;
mod bias1d;
pub use bias1d::Bias1d;
mod linear;
pub use linear::Dense;
mod softmax;
pub use softmax::Softmax;
mod residual;
pub use residual::Residual;
