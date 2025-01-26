use crate::Dtype;

#[derive(Clone, Debug)]
pub struct Bias1d<E: Dtype, const I: usize> {
    pub(crate) bias: [E; I],
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

impl<E: Dtype, const I: usize> crate::Module<[E; I]> for Bias1d<E, I> {
    type Output = [E; I];

    fn forward(&mut self, x: [E; I]) -> Result<Self::Output, super::Error> {
        Ok(Bias1d::forward(self, &x))
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
