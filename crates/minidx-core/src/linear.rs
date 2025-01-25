use crate::ops::MatMulKernel;
use crate::{Backend, Dim, Dtype, Shape, Tensor};

#[allow(unused)]
#[allow(clippy::too_many_arguments)]
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

                    // println!(
                    //     "({},{},{}) {:?} * {:?} = {:?} ({}, {}, {})",
                    //     i_m,
                    //     i_k,
                    //     i_n,
                    //     a,
                    //     b,
                    //     a * b,
                    //     a_strides[0] * i_m + a_strides[1] * i_k,
                    //     b_strides[0] * i_k + b_strides[1] * i_n,
                    //     c_strides[0] * i_m + c_strides[1] * i_n
                    // );
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

#[derive(Clone, Debug)]
struct Dense<E: Dtype, const I: usize, const O: usize> {
    weights: [[E; I]; O],
}

impl<E: Dtype, const I: usize, const O: usize> Default for Dense<E, I, O> {
    fn default() -> Self {
        Dense {
            weights: [[E::default(); I]; O],
        }
    }
}

impl<E: Dtype, const I: usize, const O: usize> Dense<E, I, O> {
    pub fn forward(&self, input: &[E; I]) -> [E; O] {
        let mut out: [E; O] = [E::default(); O];

        // TODO: Use gemm package + parallelism
        let (m, k) = (1, I);
        let n = O;
        naive_gemm(
            (m, k, n),
            true,
            input.as_ptr(),
            Shape::strides(&(1, I)),
            self.weights.as_ptr() as *const E,
            Shape::strides(&(I, O)),
            out.as_mut_ptr(),
            Shape::strides(&(1, O)),
        );

        out
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_zero_weights() {
        let layer = Dense::<f32, 1, 2>::default();
        assert_eq!(layer.forward(&[1.0]), [0.0, 0.0],);
    }

    #[test]
    fn test_1_2() {
        let layer = Dense::<f32, 1, 2> {
            weights: [[0.5], [1.0]],
        };
        assert_eq!(layer.forward(&[1.0]), [0.5, 1.0],);
    }

    #[test]
    fn test_2_1() {
        let layer = Dense::<f32, 2, 1> {
            weights: [[0.05, 0.5]],
        };
        assert_eq!(layer.forward(&[1.0, 2.0]), [1.05],);
    }

    #[test]
    fn test_2_2() {
        let layer = Dense::<f32, 2, 2> {
            weights: [[0.1, 0.4], [0.5, 0.2]],
        };
        assert_eq!(layer.forward(&[1.0, 2.0]), [1.1, 0.8],);
    }
}
