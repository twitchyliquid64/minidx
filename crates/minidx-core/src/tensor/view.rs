use super::Tensor;
use crate::{Backend, HasShape, Shape, UniqueId};

/// Describes a view into some dimensioned array of data, like a tensor.
///
/// *If it looks like a tensor and barks like a tensor, then pet it like a tensor.*
#[allow(clippy::len_without_is_empty)]
pub trait View<S: Shape, E, B: Backend<E>>: HasShape<Shape = S> {
    fn id(&self) -> UniqueId;
    fn len(&self) -> usize;
    fn strides(&self) -> S::Concrete;
    fn backend(&self) -> &B;
    fn data(&self) -> Option<&B::Vec>;
}

impl<S: Shape, E, B: Backend<E>> View<S, E, B> for Tensor<S, E, B> {
    fn id(&self) -> UniqueId {
        self.id
    }

    fn len(&self) -> usize {
        self.backend.len(&self.data)
    }

    fn strides(&self) -> S::Concrete {
        self.strides
    }

    fn backend(&self) -> &B {
        &self.backend
    }

    fn data(&self) -> Option<&B::Vec> {
        Some(self.data.as_ref())
    }
}
