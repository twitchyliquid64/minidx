//! A [Problem](crate::problem::Problem) implementation for MNIST classification.
use super::idx;
use crate::problem::Problem;
use minidx_core::Dtype;
use std::io::{Error, Read};

/// Samples from an MNIST image classification dataset.
pub struct ImgClassification<E: Dtype, RNG: rand::Rng, const I: usize, const O: usize> {
    data: Vec<([E; I], usize)>,
    rng: RNG,
}

impl<E: Dtype, RNG: rand::Rng, const I: usize, const O: usize> ImgClassification<E, RNG, I, O> {
    pub fn from_files<F: Read>(rng: RNG, img_idx: F, labels_idx: F) -> Result<Self, Error> {
        let imgs = idx::Reader::from_file(img_idx)?;
        assert_eq!(imgs.row_bytes_len(), I);
        let labels = idx::Reader::from_file(labels_idx)?;
        assert_eq!(labels.row_bytes_len(), 1);

        assert_eq!(imgs.row_count(), labels.row_count());
        let mut data: Vec<([E; I], usize)> = vec![([E::default(); I], 0); imgs.row_count()];

        for ((i, l), e) in imgs
            .into_row_iter::<u8>()
            .zip(labels.into_row_iter::<u8>())
            .zip(data.iter_mut())
        {
            assert_eq!(i.len(), I);
            for (ii, i) in e.0.iter_mut().zip(i.into_iter()) {
                *ii = E::from_f32(i as f32 / 255.0).unwrap();
            }
            e.1 = l[0] as usize;
        }

        Ok(Self { rng, data })
    }
}

impl<E: Dtype, RNG: rand::Rng, const I: usize, const O: usize> Problem
    for ImgClassification<E, RNG, I, O>
{
    type Input = [E; I];
    type Output = [E; O];

    fn sample(&mut self) -> (Self::Input, Self::Output) {
        use crate::OneHotEncoder;

        let (input, output) = self.data[self.rng.random_range(0..self.data.len())];

        (input, OneHotEncoder::<O>::value(output))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::rngs::SmallRng;
    use rand::SeedableRng;
    use std::env;
    use std::fs::File;

    #[test]
    #[ignore]
    fn from_paths() {
        let img_file = File::open(
            env::var("MNIST_TRAIN_IMG_PATH").unwrap_or("/tmp/train-images-idx3-ubyte".into()),
        )
        .unwrap();
        let labels_file = File::open(
            env::var("MNIST_TRAIN_LABELS_PATH").unwrap_or("/tmp/train-labels-idx1-ubyte".into()),
        )
        .unwrap();

        const INPUT_DIMS: usize = 28 * 28;
        let _p: ImgClassification<f32, SmallRng, INPUT_DIMS, 10> =
            ImgClassification::from_files(SmallRng::seed_from_u64(45645), img_file, labels_file)
                .unwrap();

        // panic!("{:?}", p.sample());
    }
}
