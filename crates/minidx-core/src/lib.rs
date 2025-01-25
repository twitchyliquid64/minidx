mod dtypes;
pub use dtypes::*;
pub mod shapes;
pub use shapes::*;

mod iterate;
pub(crate) use iterate::*;

mod tensor;
pub use tensor::{Backend, Error, Tensor, ZerosTensor};

pub mod ops;

pub mod linear; // TODO: Move to own crate.

/// An id used in to associate gradients with Tensors.
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash, PartialOrd, Ord)]
pub struct UniqueId(usize);

/// Generate a [UniqueId].
pub(crate) fn unique_id() -> UniqueId {
    static COUNTER: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0);
    UniqueId(COUNTER.fetch_add(1, std::sync::atomic::Ordering::Relaxed))
}

/// A minimal backend for testing purposes.
///
/// TODO: Should we just impl Backend for () instead?
/// TODO: Backend should probably not be parameterized by E.
#[derive(Debug, Clone, Default)]
pub struct MiniBackend<E: Dtype> {
    e: std::marker::PhantomData<E>,
}

impl<E: Dtype> MiniBackend<E> {
    #[inline]
    pub(crate) fn try_alloc_zeros(&self, numel: usize) -> Result<Vec<E>, Error> {
        self.try_alloc_elem(numel, Default::default())
    }

    #[inline]
    pub(crate) fn try_alloc_elem(&self, numel: usize, elem: E) -> Result<Vec<E>, Error> {
        Ok(std::vec![elem; numel])
    }
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
