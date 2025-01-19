use super::{BinaryOp, UnaryOp};
use crate::Dtype;

/// Binary element-wise addition of corresponding values in two tensors.
pub struct AddOp;

impl<E: Dtype + std::ops::Add<Output = E>> BinaryOp<E> for AddOp {
    fn f(&self, lhs: &E, rhs: &E) -> E {
        *lhs + *rhs
    }
}

/// Element-wise addition of the values in a tensor and some fixed scalar.
pub struct ScalarAddOp<E: Dtype + std::ops::Add<Output = E>> {
    pub scalar: E,
}

impl<E: Dtype + std::ops::Add<Output = E>> UnaryOp<E> for ScalarAddOp<E> {
    fn f(&self, inp: &E) -> E {
        self.scalar + *inp
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ops::BinaryKernel;
    use crate::tensor::{AsArray, TensorFrom};
    use std::borrow::Cow;

    #[test]
    fn test_scalar_add_1d() {
        let b: crate::MiniBackend<f32> = Default::default();
        let a = b.tensor([1.0, -0.1]);

        assert_eq!([1.5, 0.4], {
            use crate::ops::UnaryKernel;
            UnaryKernel::forward(&b, ScalarAddOp { scalar: 0.5f32 }, Cow::Borrowed(&a))
                .unwrap()
                .array()
        });
    }

    #[test]
    fn test_add_0d() {
        let b: crate::MiniBackend<f32> = Default::default();
        let lhs = b.tensor(1.0);
        let rhs = b.tensor(3.5);

        assert_eq!(
            vec![4.5f32],
            b.forward(AddOp, Cow::Borrowed(&lhs), Cow::Borrowed(&rhs))
                .unwrap()
                .as_vec()
        );
    }

    #[test]
    fn test_add_1d() {
        let b: crate::MiniBackend<f32> = Default::default();
        let lhs = b.tensor([1.0, -0.4]);
        let rhs = b.tensor([3.5, 0.2]);

        assert_eq!(
            [4.5, -0.2],
            b.forward(AddOp, Cow::Borrowed(&lhs), Cow::Borrowed(&rhs))
                .unwrap()
                .array()
        );
    }

    #[test]
    fn test_add_2d() {
        let b: crate::MiniBackend<f32> = Default::default();
        let lhs = b.tensor([[1.0, -0.4], [3.0, 2.0]]);
        let rhs = b.tensor([[3.5, 0.2], [1.5, 0.5]]);

        assert_eq!(
            [[4.5, -0.2], [4.5, 2.5]],
            b.forward(AddOp, Cow::Borrowed(&lhs), Cow::Borrowed(&rhs))
                .unwrap()
                .array()
        );
    }
}
