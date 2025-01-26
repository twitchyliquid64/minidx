mod dtypes;
pub use dtypes::*;
pub mod shapes;
pub use shapes::*;

mod iterate;
pub(crate) use iterate::*;

pub mod activation; // TODO: move to layers module.
pub mod bias1d; // TODO: move to layers module.
pub mod linear; // TODO: move to layers module.
