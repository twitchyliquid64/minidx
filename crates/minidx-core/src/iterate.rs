use super::Shape;

#[allow(dead_code)]
#[derive(Debug, Eq, PartialEq)]
pub(crate) struct NdIndex<S: Shape> {
    pub(crate) indices: S::Concrete,
    pub(crate) shape: S::Concrete,
    pub(crate) strides: S::Concrete,
    pub(crate) next: Option<usize>,
    pub(crate) contiguous: Option<usize>,
}

#[allow(dead_code)]
impl<S: Shape> NdIndex<S> {
    #[inline]
    pub(crate) fn new(shape: S, strides: S::Concrete) -> Self {
        Self {
            indices: Default::default(),
            shape: shape.concrete(),
            strides,
            next: if shape.num_elements() > 0 {
                Some(0)
            } else {
                None
            },
            contiguous: (strides == shape.strides()).then(|| shape.num_elements()),
        }
    }
}

#[allow(dead_code)]
impl<S: Shape> NdIndex<S> {
    pub(crate) fn get_strided_index(&self, mut idx: usize) -> usize {
        let mut out = 0;

        let shape = self.shape.as_ref();
        let strides = self.strides.as_ref();

        for (dim, stride) in shape.iter().zip(strides.iter()).rev() {
            out += (idx % dim) * stride;
            idx /= dim;
        }

        out
    }

    #[inline(always)]
    pub(crate) fn next(&mut self) -> Option<usize> {
        match self.contiguous {
            Some(numel) => match self.next.as_mut() {
                Some(i) => {
                    let idx = *i;
                    let next = idx + 1;
                    if next >= numel {
                        self.next = None;
                    } else {
                        *i = next;
                    }
                    Some(idx)
                }
                None => None,
            },
            None => self.next_with_idx().map(|(i, _)| i),
        }
    }

    #[inline(always)]
    pub(crate) fn next_with_idx(&mut self) -> Option<(usize, S::Concrete)> {
        match (S::NUM_DIMS, self.next.as_mut()) {
            (_, None) => None,
            (0, Some(i)) => {
                let idx = (*i, self.indices);
                self.next = None;
                Some(idx)
            }
            (_, Some(i)) => {
                let idx = (*i, self.indices);
                let mut dim = S::NUM_DIMS - 1;
                loop {
                    self.indices[dim] += 1;
                    *i += self.strides[dim];

                    if self.indices[dim] < self.shape[dim] {
                        break;
                    }

                    *i -= self.shape[dim] * self.strides[dim];
                    self.indices[dim] = 0;

                    if dim == 0 {
                        self.next = None;
                        break;
                    }

                    dim -= 1;
                }
                Some(idx)
            }
        }
    }
}

#[allow(dead_code)]
pub(crate) struct StridedRefIter<'a, S: Shape, E> {
    pub(crate) data: &'a Vec<E>,
    pub(crate) index: NdIndex<S>,
}

#[allow(dead_code)]
pub(crate) struct StridedMutIter<'a, S: Shape, E> {
    pub(crate) data: &'a mut Vec<E>,
    pub(crate) index: NdIndex<S>,
}

#[allow(dead_code)]
pub(crate) struct StridedRefIndexIter<'a, S: Shape, E> {
    pub(crate) data: &'a Vec<E>,
    pub(crate) index: NdIndex<S>,
}

#[allow(dead_code)]
pub(crate) struct StridedMutIndexIter<'a, S: Shape, E> {
    pub(crate) data: &'a mut Vec<E>,
    pub(crate) index: NdIndex<S>,
}

#[allow(dead_code)]
pub(crate) trait LendingIterator {
    type Item<'a>
    where
        Self: 'a;
    fn next(&'_ mut self) -> Option<Self::Item<'_>>;
}

impl<'q, S: Shape, E> LendingIterator for StridedRefIter<'q, S, E> {
    type Item<'a>
        = &'a E
    where
        Self: 'a;
    #[inline(always)]
    fn next(&'_ mut self) -> Option<Self::Item<'_>> {
        self.index.next().map(|i| &self.data[i])
    }
}

impl<'q, S: Shape, E> LendingIterator for StridedMutIter<'q, S, E> {
    type Item<'a>
        = &'a mut E
    where
        Self: 'a;
    #[inline(always)]
    fn next(&'_ mut self) -> Option<Self::Item<'_>> {
        self.index.next().map(|i| &mut self.data[i])
    }
}

impl<'q, S: Shape, E> LendingIterator for StridedRefIndexIter<'q, S, E> {
    type Item<'a>
        = (&'a E, S::Concrete)
    where
        Self: 'a;
    #[inline(always)]
    fn next(&'_ mut self) -> Option<Self::Item<'_>> {
        self.index
            .next_with_idx()
            .map(|(i, idx)| (&self.data[i], idx))
    }
}

impl<'q, S: Shape, E> LendingIterator for StridedMutIndexIter<'q, S, E> {
    type Item<'a>
        = (&'a mut E, S::Concrete)
    where
        Self: 'a;
    #[inline(always)]
    fn next(&'_ mut self) -> Option<Self::Item<'_>> {
        self.index
            .next_with_idx()
            .map(|(i, idx)| (&mut self.data[i], idx))
    }
}

#[cfg(test)]
mod tests {
    use crate::shapes::{Rank1, Rank2, Rank3};

    use super::*;

    #[test]
    fn test_0d_contiguous_iter() {
        let mut i = NdIndex::new((), ().strides());
        assert_eq!(i.next(), Some(0));
        assert!(i.next().is_none());
    }

    #[test]
    fn test_1d_contiguous_iter() {
        let shape: Rank1<3> = Default::default();
        let mut i = NdIndex::new(shape, shape.strides());
        assert_eq!(i.next(), Some(0));
        assert_eq!(i.next(), Some(1));
        assert_eq!(i.next(), Some(2));
        assert!(i.next().is_none());
    }

    #[test]
    fn test_2d_contiguous_iter() {
        let shape: Rank2<2, 3> = Default::default();
        let mut i = NdIndex::new(shape, shape.strides());
        assert_eq!(i.next(), Some(0));
        assert_eq!(i.next(), Some(1));
        assert_eq!(i.next(), Some(2));
        assert_eq!(i.next(), Some(3));
        assert_eq!(i.next(), Some(4));
        assert_eq!(i.next(), Some(5));
        assert!(i.next().is_none());
    }

    #[test]
    fn test_2d_broadcasted_0_iter() {
        let shape: Rank2<2, 3> = Default::default();
        let mut i = NdIndex::new(shape, [0, 1]);
        assert_eq!(i.next(), Some(0));
        assert_eq!(i.next(), Some(1));
        assert_eq!(i.next(), Some(2));
        assert_eq!(i.next(), Some(0));
        assert_eq!(i.next(), Some(1));
        assert_eq!(i.next(), Some(2));
        assert!(i.next().is_none());
    }

    #[test]
    fn test_2d_broadcasted_1_iter() {
        let shape: Rank2<2, 3> = Default::default();
        let mut i = NdIndex::new(shape, [1, 0]);
        assert_eq!(i.next(), Some(0));
        assert_eq!(i.next(), Some(0));
        assert_eq!(i.next(), Some(0));
        assert_eq!(i.next(), Some(1));
        assert_eq!(i.next(), Some(1));
        assert_eq!(i.next(), Some(1));
        assert!(i.next().is_none());
    }

    #[test]
    fn test_2d_permuted_iter() {
        let shape: Rank2<3, 2> = Default::default();
        let mut i = NdIndex::new(shape, [1, 3]);
        assert_eq!(i.next(), Some(0));
        assert_eq!(i.next(), Some(3));
        assert_eq!(i.next(), Some(1));
        assert_eq!(i.next(), Some(4));
        assert_eq!(i.next(), Some(2));
        assert_eq!(i.next(), Some(5));
        assert!(i.next().is_none());
    }

    #[test]
    fn test_3d_broadcasted_iter() {
        let shape: Rank3<3, 1, 2> = Default::default();
        let mut i = NdIndex::new(shape, [2, 0, 1]);
        assert_eq!(i.next(), Some(0));
        assert_eq!(i.next(), Some(1));
        assert_eq!(i.next(), Some(2));
        assert_eq!(i.next(), Some(3));
        assert_eq!(i.next(), Some(4));
        assert_eq!(i.next(), Some(5));
        assert!(i.next().is_none());
    }
}
