use crate::{unique_id, MiniBackend};
use crate::{Backend, Dtype, Error, Shape, Tensor};
use std::borrow::Cow;

mod negate;
pub use negate::*;

/// UnaryKernel implements the execution of a unary op on a tensor.
pub trait UnaryKernel<Op, E: Dtype>: Backend<E> {
    fn forward<S: Shape>(
        &self,
        op: Op,
        inp: Cow<Tensor<S, E, Self>>,
    ) -> Result<Tensor<S, E, Self>, Error>;
}

/// UnaryKernel implements an elementwise unary operation.
pub trait UnaryOp<E: Dtype> {
    fn f(&self, x: &E) -> E;
}

impl<E: Dtype, Op: UnaryOp<E>> UnaryKernel<Op, E> for MiniBackend<E> {
    fn forward<S: Shape>(
        &self,
        op: Op,
        inp: Cow<Tensor<S, E, Self>>,
    ) -> Result<Tensor<S, E, Self>, Error> {
        let mut out = match inp {
            Cow::Borrowed(inp) => {
                // allocate a new data buffer
                Tensor {
                    id: unique_id(),
                    data: inp.data.clone(),
                    shape: inp.shape,
                    strides: inp.strides,
                    backend: self.clone(),
                }
            }
            Cow::Owned(mut inp) => {
                // re-use the data buffer
                inp.id = unique_id();
                inp
            }
        };
        // NOTE: we can iterate over buf here because we know inp & out
        // have exact same strides due to clone.
        for x in out.buf_iter_mut() {
            *x = op.f(x);
        }
        Ok(out)
    }
}
