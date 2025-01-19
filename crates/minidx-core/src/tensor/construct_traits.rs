use super::*;

/// Construct tensors from rust vectors. This trait is only used to implement TensorFrom.
pub trait TensorFromVec<E>: Backend<E> {
    fn tensor_from_vec<S: Shape>(&self, src: Vec<E>, shape: S) -> Tensor<S, E, Self> {
        self.try_tensor_from_vec::<S>(src, shape).unwrap()
    }

    fn try_tensor_from_vec<S: Shape>(
        &self,
        src: Vec<E>,
        shape: S,
    ) -> Result<Tensor<S, E, Self>, Error>;
}

/// Construct tensors from rust data
pub trait TensorFrom<Src, S: Shape, E>: Backend<E> {
    fn tensor(&self, src: Src) -> Tensor<S, E, Self> {
        self.try_tensor(src).unwrap()
    }
    /// Fallible version of [TensorFrom::tensor]
    fn try_tensor(&self, src: Src) -> Result<Tensor<S, E, Self>, Error>;
}

impl<E, D: TensorFromVec<E>> TensorFrom<E, Rank0, E> for D {
    fn try_tensor(&self, src: E) -> Result<Tensor<Rank0, E, Self>, Error> {
        self.try_tensor_from_vec(vec![src], ())
    }
}

impl<E: Copy, const M: usize, D: TensorFromVec<E>> TensorFrom<[E; M], Rank1<M>, E> for D {
    fn try_tensor(&self, src: [E; M]) -> Result<Tensor<Rank1<M>, E, Self>, Error> {
        self.try_tensor(&src)
    }
}

impl<E: Copy, const M: usize, D: TensorFromVec<E>> TensorFrom<&[E; M], Rank1<M>, E> for D {
    fn try_tensor(&self, src: &[E; M]) -> Result<Tensor<Rank1<M>, E, Self>, Error> {
        self.try_tensor_from_vec(src.to_vec(), (Const::<M>,))
    }
}

impl<E: Copy, const M: usize, const N: usize, D: TensorFromVec<E>>
    TensorFrom<[[E; N]; M], Rank2<M, N>, E> for D
{
    fn try_tensor(&self, src: [[E; N]; M]) -> Result<Tensor<Rank2<M, N>, E, Self>, Error> {
        let vec: Vec<E> = src.iter().flat_map(|v| v.iter().copied()).collect();

        self.try_tensor_from_vec(vec, (Const::<M>, Const::<N>))
    }
}

impl<E: Copy, const M: usize, const N: usize, const O: usize, D: TensorFromVec<E>>
    TensorFrom<[[[E; O]; N]; M], Rank3<M, N, O>, E> for D
{
    fn try_tensor(&self, src: [[[E; O]; N]; M]) -> Result<Tensor<Rank3<M, N, O>, E, Self>, Error> {
        let vec: Vec<E> = src
            .iter()
            .flat_map(|v| v.iter())
            .flat_map(|v| v.iter().copied())
            .collect();

        self.try_tensor_from_vec(vec, (Const::<M>, Const::<N>, Const::<O>))
    }
}

impl<
        E: Copy,
        const M: usize,
        const N: usize,
        const O: usize,
        const P: usize,
        D: TensorFromVec<E>,
    > TensorFrom<[[[[E; P]; O]; N]; M], Rank4<M, N, O, P>, E> for D
{
    fn try_tensor(
        &self,
        src: [[[[E; P]; O]; N]; M],
    ) -> Result<Tensor<Rank4<M, N, O, P>, E, Self>, Error> {
        let vec: Vec<E> = src
            .iter()
            .flat_map(|v| v.iter())
            .flat_map(|v| v.iter())
            .flat_map(|v| v.iter().copied())
            .collect();

        self.try_tensor_from_vec(vec, (Const::<M>, Const::<N>, Const::<O>, Const::<P>))
    }
}

impl<E, S: ConstShape, D: TensorFromVec<E>> TensorFrom<Vec<E>, S, E> for D {
    fn try_tensor(&self, src: Vec<E>) -> Result<Tensor<S, E, Self>, Error> {
        self.try_tensor_from_vec(src, S::default())
    }
}

impl<E, S: Shape, D: TensorFromVec<E>> TensorFrom<(Vec<E>, S), S, E> for D {
    fn try_tensor(&self, (src, shape): (Vec<E>, S)) -> Result<Tensor<S, E, Self>, Error> {
        self.try_tensor_from_vec(src, shape)
    }
}

/// Construct tensors filled with zeros.
pub trait ZerosTensor<E>: Backend<E> {
    /// Creates a tensor filled with zeros.
    fn zeros<S: ConstShape>(&self) -> Tensor<S, E, Self> {
        self.try_zeros_like::<S>(&Default::default()).unwrap()
    }

    /// Fallible version of [ZerosTensor::zeros]
    fn try_zeros<S: ConstShape>(&self) -> Result<Tensor<S, E, Self>, Error> {
        self.try_zeros_like::<S>(&Default::default())
    }

    /// Build the tensor with a shape given by something else.
    fn zeros_like<S: HasShape>(&self, src: &S) -> Tensor<S::Shape, E, Self> {
        self.try_zeros_like(src).unwrap()
    }

    /// Fallible version of [ZerosTensor::zeros_like]
    fn try_zeros_like<S: HasShape>(&self, src: &S) -> Result<Tensor<S::Shape, E, Self>, Error>;
}

pub trait ZeroFillStorage<E>: Backend<E> {
    fn try_fill_with_zeros(&self, storage: &mut Self::Vec) -> Result<(), Error>;
}
