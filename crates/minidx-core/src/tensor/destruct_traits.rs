use super::*;

pub trait TensorToArray<S: Shape, E>: Backend<E> {
    type Array: std::fmt::Debug + PartialEq;
    fn tensor_to_array(&self, tensor: &Tensor<S, E, Self>) -> Self::Array;
}

pub trait AsArray {
    type Array: std::fmt::Debug + PartialEq;
    fn array(&self) -> Self::Array;
}

impl<S: Shape, E, B: TensorToArray<S, E>> AsArray for Tensor<S, E, B> {
    type Array = B::Array;
    /// Convert tensors to rust arrays
    fn array(&self) -> Self::Array {
        self.backend.tensor_to_array(self)
    }
}

impl<S: Shape, E, B: Backend<E>> Tensor<S, E, B> {
    pub fn as_vec(&self) -> std::vec::Vec<E> {
        self.backend.tensor_to_vec(self)
    }
}

impl<E: Unit + Dtype> TensorToArray<Rank0, E> for MiniBackend<E> {
    type Array = E;
    fn tensor_to_array(&self, tensor: &Tensor<Rank0, E, Self>) -> Self::Array {
        let mut out: Self::Array = Default::default();
        out.clone_from(&tensor.data[0]);
        out
    }
}

impl<E: Unit + Dtype, const M: usize> TensorToArray<Rank1<M>, E> for MiniBackend<E> {
    type Array = [E; M];
    fn tensor_to_array(&self, tensor: &Tensor<Rank1<M>, E, Self>) -> Self::Array {
        let mut out: Self::Array = [Default::default(); M];
        let mut iter = tensor.iter();
        for m in 0..M {
            out[m].clone_from(iter.next().unwrap());
        }
        out
    }
}

impl<E: Unit + Dtype, const M: usize, const N: usize> TensorToArray<Rank2<M, N>, E>
    for MiniBackend<E>
{
    type Array = [[E; N]; M];
    fn tensor_to_array(&self, tensor: &Tensor<Rank2<M, N>, E, Self>) -> Self::Array {
        let mut out: Self::Array = [[Default::default(); N]; M];
        let mut iter = tensor.iter();
        for m in 0..M {
            for n in 0..N {
                out[m][n].clone_from(iter.next().unwrap());
            }
        }
        out
    }
}

impl<E: Unit + Dtype, const M: usize, const N: usize, const O: usize>
    TensorToArray<Rank3<M, N, O>, E> for MiniBackend<E>
{
    type Array = [[[E; O]; N]; M];
    fn tensor_to_array(&self, tensor: &Tensor<Rank3<M, N, O>, E, Self>) -> Self::Array {
        let mut out: Self::Array = [[[Default::default(); O]; N]; M];
        let mut iter = tensor.iter_with_index();
        while let Some((v, [m, n, o])) = iter.next() {
            out[m][n][o].clone_from(v);
        }
        out
    }
}
