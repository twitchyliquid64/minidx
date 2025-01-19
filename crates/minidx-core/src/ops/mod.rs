use crate::{unique_id, MiniBackend};
use crate::{Backend, Dtype, Error, LendingIterator, NdIndex, Shape, Tensor, ZerosTensor};
use std::borrow::Cow;

mod negate;
pub use negate::*;
mod add;
pub use add::*;

/// UnaryKernel implements the execution of a unary op on a tensor.
pub trait UnaryKernel<Op, E: Dtype>: Backend<E> {
    fn forward<S: Shape>(
        &self,
        op: Op,
        inp: Cow<Tensor<S, E, Self>>,
    ) -> Result<Tensor<S, E, Self>, Error>;
}

/// UnaryOp implements an elementwise unary operation.
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

/// BinaryKernel implements the execution of a binary op on a tensor.
pub trait BinaryKernel<Op, E: Dtype>: Backend<E> {
    fn forward<S: Shape>(
        &self,
        op: Op,
        lhs: Cow<Tensor<S, E, Self>>,
        rhs: Cow<Tensor<S, E, Self>>,
    ) -> Result<Tensor<S, E, Self>, Error>;
}

/// BinaryOp implements an elementwise binary operation.
pub trait BinaryOp<E: Dtype> {
    fn f(&self, lhs: &E, rhs: &E) -> E;
}

impl<E: Dtype, Op: BinaryOp<E>> BinaryKernel<Op, E> for MiniBackend<E> {
    fn forward<S: Shape>(
        &self,
        op: Op,
        lhs: Cow<Tensor<S, E, Self>>,
        rhs: Cow<Tensor<S, E, Self>>,
    ) -> Result<Tensor<S, E, Self>, Error> {
        match (lhs, rhs) {
            (Cow::Borrowed(lhs), Cow::Borrowed(rhs)) => {
                let mut out = self.try_zeros_like(&lhs.shape)?;
                let mut lhs_iter = lhs.iter();
                let mut rhs_iter = rhs.iter();
                for o in out.buf_iter_mut() {
                    let l = lhs_iter.next().unwrap();
                    let r = rhs_iter.next().unwrap();
                    *o = op.f(l, r);
                }
                Ok(out)
            }
            (Cow::Owned(mut lhs), Cow::Owned(mut rhs)) => {
                let lhs_valid = lhs.strides == lhs.shape.strides();
                let rhs_valid = rhs.strides == rhs.shape.strides();
                if lhs_valid || rhs_valid {
                    let lhs_count = std::sync::Arc::strong_count(&lhs.data);
                    let rhs_count = std::sync::Arc::strong_count(&rhs.data);
                    if rhs_valid && (rhs_count == 1 || !lhs_valid || lhs_count != 1) {
                        rhs.id = unique_id();
                        let mut lhs_idx = NdIndex::new(lhs.shape, lhs.strides);
                        for r in rhs.buf_iter_mut() {
                            *r = op.f(&lhs.data[lhs_idx.next().unwrap()], r);
                        }
                        Ok(rhs)
                    } else {
                        lhs.id = unique_id();
                        let mut rhs_idx = NdIndex::new(rhs.shape, rhs.strides);
                        for l in lhs.buf_iter_mut() {
                            *l = op.f(l, &rhs.data[rhs_idx.next().unwrap()]);
                        }
                        Ok(lhs)
                    }
                } else {
                    <Self as BinaryKernel<Op, E>>::forward(
                        self,
                        op,
                        Cow::Borrowed(&lhs),
                        Cow::Borrowed(&rhs),
                    )
                }
            }
            _ => unreachable!(),
        }
    }
}
