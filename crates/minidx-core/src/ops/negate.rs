use super::UnaryOp;
use crate::Dtype;
use std::borrow::Cow;

/// Element-wise negation of all values in the tensor.
pub struct NegateOp;

impl<E: Dtype + std::ops::Neg<Output = E>> UnaryOp<E> for NegateOp {
    fn f(&self, inp: &E) -> E {
        inp.neg()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ops::UnaryKernel;
    use crate::tensor::{AsArray, TensorFrom};

    #[test]
    fn test_negate_0d() {
        let b: crate::MiniBackend<f32> = Default::default();
        let a = b.tensor(1.0);

        assert_eq!(
            vec![-1.0f32],
            b.forward(NegateOp, Cow::Borrowed(&a)).unwrap().as_vec()
        );
    }

    #[test]
    fn test_negate_1d() {
        let b: crate::MiniBackend<f32> = Default::default();
        let a = b.tensor([1.0, -2.0, 1.45]);

        assert_eq!(
            vec![-1.0, 2.0, -1.45],
            b.forward(NegateOp, Cow::Borrowed(&a)).unwrap().as_vec()
        );
    }

    #[test]
    fn test_negate_2d() {
        let b: crate::MiniBackend<f32> = Default::default();
        let a = b.tensor([[1.0, -2.0, 1.45], [3.0, -2.5, 6.45]]);

        assert_eq!(
            [[-1.0, 2.0, -1.45], [-3.0, 2.5, -6.45]],
            b.forward(NegateOp, Cow::Borrowed(&a)).unwrap().array()
        );
    }
}
