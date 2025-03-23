//! A reader for the idx format, used by MNIST.

use byteorder::{BigEndian, ReadBytesExt};
use minidx_core::Dtype;
use std::io::{BufReader, Error, ErrorKind, Read};

/// The datatype of value indexed by the dimensions.
#[derive(Debug, Clone, PartialEq)]
pub enum Element {
    UnsignedByte,
    SignedByte,
    Int,
    F32,
    F64,
}

impl Element {
    const fn size_bytes(&self) -> usize {
        use Element::*;
        match self {
            UnsignedByte | SignedByte => 1,
            Int | F32 => 4,
            F64 => 8,
        }
    }
}

impl TryFrom<u8> for Element {
    type Error = ();

    fn try_from(v: u8) -> Result<Self, Self::Error> {
        match v {
            0x08 => Ok(Self::UnsignedByte),
            0x09 => Ok(Self::SignedByte),
            0x0C => Ok(Self::Int),
            0x0D => Ok(Self::F32),
            0x0E => Ok(Self::F64),
            _ => Err(()),
        }
    }
}

/// A reader for data in the idx format.
pub struct Reader<F: Read> {
    element: Element,
    shape: Vec<usize>,
    file: BufReader<F>,
}

impl<F: Read> Reader<F> {
    pub fn from_file(f: F) -> Result<Self, Error> {
        let mut f = BufReader::new(f);

        let magic = f.read_u16::<BigEndian>()?;
        if magic != 0x0 {
            return Err(Error::new(ErrorKind::Other, "incorrect magic value"));
        }
        let element: Element = f
            .read_u8()?
            .try_into()
            .map_err(|_| Error::new(ErrorKind::Other, "unrecognized element kind"))?;

        let num_dims = f.read_u8()?;

        let mut shape = Vec::with_capacity(num_dims as usize);
        for _i in 0..num_dims {
            shape.push(f.read_u32::<BigEndian>()? as usize);
        }

        Ok(Self {
            element,
            shape,
            file: f,
        })
    }

    pub fn row_count(&self) -> usize {
        self.shape[0]
    }
    pub fn elements_per_row(&self) -> usize {
        self.shape.iter().skip(1).fold(1, |a, x| a * x)
    }
    pub fn row_bytes_len(&self) -> usize {
        self.elements_per_row() * self.element.size_bytes()
    }
    pub fn shape(&self) -> &Vec<usize> {
        &self.shape
    }

    pub fn next<E: Dtype>(&mut self) -> Result<Vec<E>, Error> {
        let mut out = vec![E::default(); self.elements_per_row()];
        for e in out.iter_mut() {
            use num_traits::cast::FromPrimitive;
            use Element::*;
            *e = match self.element {
                UnsignedByte => E::from_u8(self.file.read_u8()?).unwrap(),
                SignedByte => E::from_i8(self.file.read_i8()?).unwrap(),
                Int => E::from_u32(self.file.read_u32::<BigEndian>()?).unwrap(),
                F32 => E::from_f32(self.file.read_f32::<BigEndian>()?).unwrap(),
                F64 => E::from_f64(self.file.read_f64::<BigEndian>()?).unwrap(),
            }
        }

        Ok(out)
    }

    pub fn into_row_iter<E: Dtype>(self) -> RowIterator<E, F> {
        RowIterator {
            reader: self,
            marker: Default::default(),
        }
    }
}

pub struct RowIterator<E: Dtype, F: Read> {
    reader: Reader<F>,
    marker: std::marker::PhantomData<E>,
}

impl<E: Dtype, F: Read> Iterator for RowIterator<E, F> {
    type Item = Vec<E>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.reader.next() {
            Ok(v) => Some(v),
            Err(e) if e.kind() == ErrorKind::UnexpectedEof => None,
            Err(e) => panic!("err: {:?}", e),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;
    use std::fs::File;

    #[test]
    #[ignore]
    fn reader_from_file() {
        let file = File::open(
            env::var("MNIST_TRAIN_IMG_PATH").unwrap_or("/tmp/train-images-idx3-ubyte".into()),
        )
        .unwrap();

        let file = Reader::from_file(file).unwrap();
        assert_eq!(file.element, Element::UnsignedByte);
        assert_eq!(file.shape(), &vec![60000, 28, 28]);
        assert_eq!(file.row_bytes_len(), 28 * 28);

        assert_eq!(
            file.row_count(),
            file.into_row_iter::<u8>()
                .map(|row| assert_eq!(row.len(), 28 * 28))
                .count()
        );
    }
}
