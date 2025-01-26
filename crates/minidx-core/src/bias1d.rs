use crate::Dtype;

#[derive(Clone, Debug)]
struct Bias1d<E: Dtype, const I: usize> {
    bias: [E; I],
}

impl<E: Dtype, const I: usize> Default for Bias1d<E, I> {
    fn default() -> Self {
        Self {
            bias: [E::default(); I],
        }
    }
}

impl<E: Dtype, const I: usize> Bias1d<E, I> {
    pub fn forward(&self, input: &[E; I]) -> [E; I] {
        let mut out: [E; I] = self.bias.clone();
        for (o, i) in out.iter_mut().zip(input.iter()) {
            *o += *i;
        }
        out
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_zero() {
        let layer = Bias1d::<f32, 1>::default();
        assert_eq!(layer.forward(&[1.0]), [1.0],);
    }

    #[test]
    fn test_2() {
        let layer = Bias1d::<f32, 2> { bias: [1.5, 3.2] };
        assert_eq!(layer.forward(&[1.0, 3.0]), [2.5, 6.2],);
    }
}
