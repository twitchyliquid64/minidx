use super::*;
use std::sync::Arc;

mod destruct_traits;
pub use destruct_traits::*;
mod construct_traits;
pub use construct_traits::*;

mod view;
pub use view::View;

#[non_exhaustive]
#[derive(Debug)]
pub enum Error {
    /// Not enough elements were provided when creating a tensor
    WrongNumElements,
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{self:?}")
    }
}

impl std::error::Error for Error {}

/// The owner of all tensors created to service computations.
pub trait Backend<E>: 'static + std::fmt::Debug + Default + Clone {
    /// Generic Storage type
    type Vec: 'static + std::fmt::Debug + Clone + Send + Sync;

    fn try_alloc_len(&self, len: usize) -> Result<Self::Vec, Error>;
    fn tensor_to_vec<S: Shape>(&self, tensor: &Tensor<S, E, Self>) -> Vec<E>;
    fn len(&self, v: &Self::Vec) -> usize;
}

/// The single tensor struct that stores data arrays and metadata.
///
/// Generics:
/// 1. [Shape] - the shape of the underlying nd array
/// 2. [Dtype] - the type of the datas stored in the array
/// 3. [Backend] - the storage implementation
#[derive(Debug, Clone)]
pub struct Tensor<S: Shape, E, B: Backend<E>> {
    pub(crate) id: UniqueId,
    pub(crate) data: Arc<B::Vec>,
    pub(crate) shape: S,
    pub(crate) strides: S::Concrete,
    pub(crate) backend: B,
}

impl<S: Shape, E, B: Backend<E>> HasShape for Tensor<S, E, B> {
    type WithShape<New: Shape> = Tensor<New, E, B>;
    type Shape = S;
    fn shape(&self) -> &Self::Shape {
        &self.shape
    }
}

impl<S: Shape, E: Unit, B: Backend<E>> HasUnitType for Tensor<S, E, B> {
    type Unit = E;
}

impl<S: Shape, E: Dtype, B: Backend<E>> HasDtype for Tensor<S, E, B> {
    type Dtype = E;
}

impl<S: Shape, E: Dtype> Tensor<S, E, super::MiniBackend<E>> {
    #[inline]
    pub(crate) fn buf_iter(&self) -> std::slice::Iter<'_, E> {
        self.data.iter()
    }

    #[inline]
    pub(crate) fn buf_iter_mut(&mut self) -> std::slice::IterMut<'_, E> {
        std::sync::Arc::make_mut(&mut self.data).iter_mut()
    }

    #[inline]
    pub(crate) fn iter(&self) -> StridedRefIter<S, E> {
        StridedRefIter {
            data: self.data.as_ref(),
            index: NdIndex::new(self.shape, self.strides),
        }
    }

    #[inline]
    pub(crate) fn iter_mut(&mut self) -> StridedMutIter<S, E> {
        StridedMutIter {
            data: Arc::get_mut(&mut self.data).unwrap(),
            index: NdIndex::new(self.shape, self.strides),
        }
    }

    #[inline]
    pub(crate) fn iter_with_index(&self) -> StridedRefIndexIter<S, E> {
        StridedRefIndexIter {
            data: self.data.as_ref(),
            index: NdIndex::new(self.shape, self.strides),
        }
    }

    #[inline]
    pub(crate) fn iter_mut_with_index(&mut self) -> StridedMutIndexIter<S, E> {
        StridedMutIndexIter {
            data: Arc::get_mut(&mut self.data).unwrap(),
            index: NdIndex::new(self.shape, self.strides),
        }
    }
}

impl<E: Unit + Dtype> TensorFromVec<E> for super::MiniBackend<E> {
    fn try_tensor_from_vec<S: Shape>(
        &self,
        src: Vec<E>,
        shape: S,
    ) -> Result<Tensor<S, E, Self>, Error> {
        let num_elements = shape.num_elements();

        if src.len() != num_elements {
            Err(Error::WrongNumElements)
        } else {
            Ok(Tensor {
                id: unique_id(),
                data: Arc::new(src),
                shape,
                strides: shape.strides(),
                backend: self.clone(),
            })
        }
    }
}
