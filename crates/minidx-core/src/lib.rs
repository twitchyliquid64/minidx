mod dtypes;
pub use dtypes::*;
pub mod shapes;
pub use shapes::*;

mod iterate;
pub(crate) use iterate::*;

mod tensor;
pub use tensor::{Backend, Error, Tensor};

pub mod ops;

/// An id used in to associate gradients with Tensors.
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash, PartialOrd, Ord)]
pub struct UniqueId(usize);

/// Generate a [UniqueId].
pub(crate) fn unique_id() -> UniqueId {
    static COUNTER: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0);
    UniqueId(COUNTER.fetch_add(1, std::sync::atomic::Ordering::Relaxed))
}

/// A minimal backend for testing purposes.
/// TODO: Should we just impl Backend for () instead?
#[derive(Debug, Clone, Default)]
pub struct MiniBackend<E: Dtype> {
    e: std::marker::PhantomData<E>,
}

impl<E: Dtype> Backend<E> for MiniBackend<E> {
    type Vec = Vec<E>;

    fn try_alloc_len(&self, len: usize) -> Result<Self::Vec, Error> {
        Ok(vec![Default::default(); len])
    }
    fn tensor_to_vec<S: Shape>(&self, tensor: &Tensor<S, E, Self>) -> Vec<E> {
        tensor.data.as_ref().clone()
    }
    fn len(&self, v: &Self::Vec) -> usize {
        v.len()
    }
}
