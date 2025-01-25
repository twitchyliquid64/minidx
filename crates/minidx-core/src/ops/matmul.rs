use crate::{Backend, Dim, Dtype, Error, MiniBackend, Shape, Tensor, ZerosTensor};
use std::sync::Arc;

#[allow(unused)]
#[allow(clippy::too_many_arguments)]
#[inline(always)]
fn naive_gemm<F: Dtype, M: Dim, K: Dim, N: Dim>(
    (m, k, n): (M, K, N),
    accum: bool,
    ap: *const F,
    a_strides: [usize; 2],
    bp: *const F,
    b_strides: [usize; 2],
    cp: *mut F,
    c_strides: [usize; 2],
) {
    for i_m in 0..m.size() {
        for i_k in 0..k.size() {
            for i_n in 0..n.size() {
                unsafe {
                    let a = *ap.add(a_strides[0] * i_m + a_strides[1] * i_k);
                    let b = *bp.add(b_strides[0] * i_k + b_strides[1] * i_n);
                    let c = cp.add(c_strides[0] * i_m + c_strides[1] * i_n);
                    if accum {
                        *c += a * b;
                    } else {
                        *c = a * b;
                    }
                }
            }
        }
    }
}

/// Binary matrix multiplication of two tensors.
pub(crate) struct MatMulOp;

pub trait MatMulKernel<E: Dtype>: Backend<E> {
    fn matmul<M: Dim, K: Dim, N: Dim>(
        &self,
        lhs: &Tensor<(M, K), E, Self>,
        rhs: &Tensor<(K, N), E, Self>,
    ) -> Result<Tensor<(M, N), E, Self>, Error>;
}

impl<E: Dtype> MatMulKernel<E> for MiniBackend<E> {
    fn matmul<M: Dim, K: Dim, N: Dim>(
        &self,
        lhs: &Tensor<(M, K), E, Self>,
        rhs: &Tensor<(K, N), E, Self>,
    ) -> Result<Tensor<(M, N), E, Self>, Error> {
        let (m, k) = lhs.shape;
        let n = rhs.shape.1;
        let mut out = self.try_zeros_like(&(m, n))?;
        // TODO: Use gemm package + parallelism
        naive_gemm(
            (m, k, n),
            false,
            lhs.data.as_ptr(),
            lhs.strides,
            rhs.data.as_ptr(),
            rhs.strides,
            Arc::get_mut(&mut out.data).unwrap().as_mut_ptr(),
            out.strides,
        );
        Ok(out)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tensor::{AsArray, TensorFrom};

    #[test]
    fn test_matmul_2d() {
        let b: crate::MiniBackend<f32> = Default::default();
        let lhs = b.tensor([[3.0f32], [4.0f32]]);
        let rhs = b.tensor([[2.0f32, 5.0f32]]);

        assert_eq!(
            [[6.0f32, 15.0f32], [8.0f32, 20.0f32]],
            b.matmul(&lhs, &rhs).unwrap().array()
        );
    }
}
