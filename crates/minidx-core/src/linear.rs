use crate::{Dim, Dtype, Shape};

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

/// A fully-connected layer with a given number of inputs and outputs. No bias.
#[derive(Clone, Debug)]
pub struct Dense<E: Dtype, const I: usize, const O: usize> {
    pub(crate) weights: [[E; I]; O],
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

    #[inline]
    fn gradients_wrt_input(&self, output_gradients: &[E; O]) -> [E; I] {
        let mut out: [E; I] = [E::default(); I];

        // TODO: Use gemm package + parallelism
        let (m, k) = (1, I);
        let n = O;
        let strides = (m, n).strides();
        naive_gemm(
            (m, n, k),
            true,
            output_gradients.as_ptr(),
            strides,
            self.weights.as_ptr() as *const E,
            Shape::strides(&(O, I)),
            out.as_mut_ptr(),
            Shape::strides(&(1, I)),
        );
        out
    }

    #[inline]
    fn gradients_wrt_weights(&self, input: &[E; I], output_gradients: &[E; O]) -> [[E; I]; O] {
        let mut out: [[E; I]; O] = [[E::default(); I]; O];

        // TODO: Use gemm package + parallelism
        let (m, k) = (1, I);
        let n = O;
        let strides = (m, n).strides();
        naive_gemm(
            (k, m, n),
            true,
            input.as_ptr(),
            Shape::strides(&(I, 1)),
            output_gradients.as_ptr(),
            strides,
            out.as_mut_ptr() as *mut E,
            Shape::strides(&(O, I)),
        );
        out
    }
}

impl<E: Dtype, const I: usize, const O: usize> crate::Module<[E; I]> for Dense<E, I, O> {
    type Output = [E; O];

    fn forward(&mut self, x: [E; I]) -> Result<Self::Output, super::Error> {
        Ok(Dense::forward(self, &x))
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
        assert_eq!(layer.gradients_wrt_input(&[2.0, 0.0]), [1.0],);

        assert_eq!(
            layer.gradients_wrt_weights(&[1.0], &[2.0, 0.0]),
            [[2.0], [0.0]],
        );
        assert_eq!(
            layer.gradients_wrt_weights(&[1.0], &[0.0, 2.0]),
            [[0.0], [2.0]],
        );
        assert_eq!(
            layer.gradients_wrt_weights(&[1.0], &[1.0, 2.0]),
            [[1.0], [2.0]],
        );
    }

    #[test]
    fn test_2_1() {
        let layer = Dense::<f32, 2, 1> {
            weights: [[0.05, 0.5]],
        };
        assert_eq!(layer.forward(&[1.0, 2.0]), [1.05],);
        assert_eq!(layer.gradients_wrt_input(&[2.0]), [0.1, 1.0],);
    }

    #[test]
    fn test_2_2() {
        let layer = Dense::<f32, 2, 2> {
            weights: [[0.1, 0.4], [0.5, 0.2]],
        };
        assert_eq!(layer.forward(&[1.0, 2.0]), [1.1, 0.8],);
        assert_eq!(layer.gradients_wrt_input(&[0.0, 1.0]), [0.5, 0.2],);
    }
}
