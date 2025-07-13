use crate::{Dtype, Unit};
use num_traits::FromPrimitive;

/// What kind of parameter the gradient represents.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum GradClass {
    /// A gradient relating to the weight of a connection between two neurons.
    Connective,
    /// A gradient relating to some bias applied to a neuron.
    Bias,
    /// A gradient which parameterizes an activation function.
    Activation,
    /// Other kinds of parameter gradients.
    Other,
}

impl GradClass {
    pub fn should_regularize(&self) -> bool {
        use GradClass::*;
        match self {
            Connective => true,
            _ => false,
        }
    }
}

/// The set of gradients that describe some movement in parameters of a module.
pub trait Gradients: Clone + std::fmt::Debug {
    type Concrete: Dtype;

    /// Returns an iterator over each parameter gradient.
    fn grad_iter(&self) -> impl Iterator<Item = &Self::Concrete>;

    /// Returns a mutable iterator over each parameter gradient.
    fn grad_iter_mut(&mut self) -> impl Iterator<Item = &mut Self::Concrete>;

    /// Consumes the object, returning an iterator of each parameter gradient.
    fn into_grads(self) -> impl Iterator<Item = Self::Concrete>;

    /// Returns a mutable iterator over each parameter gradient, yielding
    /// both the parameter gradient and its [GradClass].
    fn grad_iter_mut_with_class(
        &mut self,
    ) -> impl Iterator<Item = (&mut Self::Concrete, GradClass)> {
        self.grad_iter_mut().map(|g| (g, GradClass::Other))
    }

    /// Merges the values from the given gradient into the current one, based on the given weight.
    ///
    /// A weight of 1.0 replaces the current gradient with the given one, where-as a weight of
    /// 0.0 means no change is made to the current gradients.
    fn merge(&mut self, other: Self, weight: f32) {
        assert!(weight >= 0.0);
        assert!(weight <= 1.0);
        let weight: Self::Concrete = Self::Concrete::from_f32(weight).unwrap();

        self.grad_iter_mut()
            .zip(other.into_grads())
            .for_each(|(g, o)| {
                use std::ops::{Mul, Sub};
                *g = Self::Concrete::ONE.sub(weight).mul(*g) + weight.mul(o);
            });
    }

    fn add(&mut self, other: Self) {
        self.grad_iter_mut()
            .zip(other.into_grads())
            .for_each(|(g, o)| {
                *g += o;
            });
    }

    /// Scales each gradient by the given constant.
    ///
    /// This method can be used to modulate gradient updates by some loss value and learning rate.
    fn scale(&mut self, s: f32) {
        self.grad_iter_mut()
            .for_each(|g| *g *= Self::Concrete::from_f32(s).unwrap());
    }

    /// Returns the cosine similarity between the two gradients.
    fn cosine_similarity(&self, other: &Self) -> Option<f32> {
        use num_traits::ToPrimitive;
        let dot_product: f32 = self
            .grad_iter()
            .zip(other.grad_iter())
            .map(|(x, y)| (*x * *y).to_f32().unwrap())
            .sum();
        let magnitude_a: f32 = self
            .grad_iter()
            .map(|x| (*x * *x).to_f32().unwrap())
            .sum::<f32>()
            .sqrt();
        let magnitude_b: f32 = other
            .grad_iter()
            .map(|x| (*x * *x).to_f32().unwrap())
            .sum::<f32>()
            .sqrt();

        if magnitude_a == 0.0 || magnitude_b == 0.0 {
            return None;
        }

        Some(dot_product / (magnitude_a * magnitude_b))
    }

    /// Returns an empty gradient object
    fn empty() -> Self;
}

impl Gradients for () {
    type Concrete = f32;

    fn grad_iter(&self) -> impl Iterator<Item = &Self::Concrete> {
        <&[Self::Concrete]>::into_iter(&[])
    }

    fn grad_iter_mut(&mut self) -> impl Iterator<Item = &mut Self::Concrete> {
        <&mut [Self::Concrete]>::into_iter(&mut [])
    }

    fn into_grads(self) -> impl Iterator<Item = Self::Concrete> {
        std::iter::IntoIterator::into_iter([0.0f32; 0])
    }

    fn empty() -> Self {
        ()
    }
}

impl<E: Dtype, const L: usize> Gradients for [E; L] {
    type Concrete = E;

    fn grad_iter(&self) -> impl Iterator<Item = &Self::Concrete> {
        <&[E; L]>::into_iter(self)
    }

    fn grad_iter_mut(&mut self) -> impl Iterator<Item = &mut Self::Concrete> {
        <&mut [E; L]>::into_iter(self)
    }

    fn into_grads(self) -> impl Iterator<Item = Self::Concrete> {
        std::iter::IntoIterator::into_iter(self)
    }

    fn empty() -> Self {
        [E::default(); L]
    }
}

impl<E: Dtype, const L1: usize, const L2: usize> Gradients for [[E; L2]; L1] {
    type Concrete = E;

    fn grad_iter(&self) -> impl Iterator<Item = &Self::Concrete> {
        <&[[E; L2]; L1]>::into_iter(self).flatten()
    }

    fn grad_iter_mut(&mut self) -> impl Iterator<Item = &mut Self::Concrete> {
        <&mut [[E; L2]; L1]>::into_iter(self).flatten()
    }

    fn grad_iter_mut_with_class(
        &mut self,
    ) -> impl Iterator<Item = (&mut Self::Concrete, GradClass)> {
        self.grad_iter_mut().map(|g| (g, GradClass::Connective))
    }

    fn into_grads(self) -> impl Iterator<Item = Self::Concrete> {
        std::iter::IntoIterator::into_iter(self)
            .map(|a| std::iter::IntoIterator::into_iter(a))
            .flatten()
    }

    fn empty() -> Self {
        [[E::default(); L2]; L1]
    }
}

macro_rules! tuple_impls {
    ([$($name:ident),+] [$($idx:tt),*], $last:ident, [$($rev_tail:ident),*]) => {
        impl<
            E: Dtype,
            $last:
            $(Gradients<Concrete = E>, $rev_tail: )*
            Gradients<Concrete = E>
        > Gradients for ($($name,)+) {
            type Concrete = E;

            fn grad_iter(&self) -> impl Iterator<Item = &Self::Concrete> {
            	let x = self.0.grad_iter();
                $(let x = x.chain(self.$idx.grad_iter());)*
                x
            }

            fn grad_iter_mut(&mut self) -> impl Iterator<Item = &mut Self::Concrete> {
            	let x = self.0.grad_iter_mut();
                $(let x = x.chain(self.$idx.grad_iter_mut());)*
                x
            }

            #[inline(always)]
            fn grad_iter_mut_with_class(&mut self) -> impl Iterator<Item = (&mut Self::Concrete, GradClass)> {
                let x = self.0.grad_iter_mut_with_class();
                $(let x = x.chain(self.$idx.grad_iter_mut_with_class());)*
                x
            }

		    fn into_grads(self) -> impl Iterator<Item = Self::Concrete> {
		        self.0.into_grads()
		        $(.chain(self.$idx.into_grads()))*
		    }

            fn empty() -> Self {
                (
                    $($name::empty(),)*
                )
            }
        }
    }
}

tuple_impls!([M1][], M1, []);
tuple_impls!([M1, M2][1], M2, [M1]);
tuple_impls!([M1, M2, M3] [1, 2], M3, [M2, M1]);
tuple_impls!([M1, M2, M3, M4] [1, 2, 3], M4, [M3, M2, M1]);
tuple_impls!([M1, M2, M3, M4, M5] [1, 2, 3, 4], M5, [M4, M3, M2, M1]);
tuple_impls!([M1, M2, M3, M4, M5, M6] [1, 2, 3, 4, 5], M6, [M5, M4, M3, M2, M1]);
tuple_impls!([M1, M2, M3, M4, M5, M6, M7] [1, 2, 3, 4, 5, 6], M7, [M6, M5, M4, M3, M2, M1]);

/// Marker for gradients which represent bias parameters.
#[derive(Clone, Debug)]
pub struct ClassBias;

impl ClassMarker for ClassBias {
    fn class() -> GradClass {
        GradClass::Bias
    }
    fn new() -> Self {
        ClassBias
    }
}

/// Marker for gradients which represent activation parameters.
#[derive(Clone, Debug)]
pub struct ClassActivation;

impl ClassMarker for ClassActivation {
    fn class() -> GradClass {
        GradClass::Activation
    }
    fn new() -> Self {
        ClassActivation
    }
}

pub(crate) trait ClassMarker: Clone + std::fmt::Debug {
    fn new() -> Self;
    fn class() -> GradClass;
}

/// A wrapper type for gradients which overrides the class reported via [Gradients::grad_iter_mut_with_class].
#[allow(private_bounds, dead_code)]
#[derive(Clone, Debug)]
pub struct ClassWrapper<G: Gradients, M: ClassMarker> {
    g: G,
    m: M,
}

#[allow(dead_code, private_bounds)]
impl<G: Gradients, M: ClassMarker> ClassWrapper<G, M> {
    #[inline(always)]
    pub(crate) fn wrap(g: G) -> Self {
        Self { g, m: M::new() }
    }
    #[inline(always)]
    pub(crate) fn raw_grads(self) -> G {
        self.g
    }
    #[inline(always)]
    pub(crate) fn raw_grads_ref(&self) -> &G {
        &self.g
    }
    #[inline(always)]
    pub(crate) fn raw_grads_mut(&mut self) -> &mut G {
        &mut self.g
    }
}

impl<G: Gradients, M: ClassMarker> Gradients for ClassWrapper<G, M> {
    type Concrete = G::Concrete;

    fn grad_iter(&self) -> impl Iterator<Item = &Self::Concrete> {
        self.g.grad_iter()
    }

    fn grad_iter_mut(&mut self) -> impl Iterator<Item = &mut Self::Concrete> {
        self.g.grad_iter_mut()
    }

    #[inline(always)]
    fn grad_iter_mut_with_class(
        &mut self,
    ) -> impl Iterator<Item = (&mut Self::Concrete, GradClass)> {
        self.grad_iter_mut().map(|g| (g, M::class()))
    }

    fn into_grads(self) -> impl Iterator<Item = Self::Concrete> {
        self.g.into_grads()
    }

    fn empty() -> Self {
        Self {
            g: G::empty(),
            m: M::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_impl_gradients() {
        const _: () = {
            const fn assert_is_gradient<T: Gradients>() {}

            assert_is_gradient::<[f32; 5]>();
            assert_is_gradient::<[[f32; 15]; 5]>();

            assert_is_gradient::<([f32; 5], [f32; 5])>();
        };
    }

    #[test]
    fn test_grad_iter() {
        let mut grads = [0.0f32; 10];
        grads.grad_iter().for_each(|x| assert!(*x == 0.0));
        grads.grad_iter_mut().for_each(|x| *x += 2.0);
        grads.grad_iter().for_each(|x| assert!(*x == 2.0));

        let mut grads = [[0.0f32; 10]; 10];
        grads.grad_iter().for_each(|x| assert!(*x == 0.0));
        grads.grad_iter_mut().for_each(|x| *x += 2.0);
        grads.grad_iter().for_each(|x| assert!(*x == 2.0));

        let mut grads = ([0.0f32; 10], [0.0f32; 15]);
        assert_eq!(grads.grad_iter().count(), 25);
        grads.grad_iter().for_each(|x| assert!(*x == 0.0));
        grads.grad_iter_mut().for_each(|x| *x += 2.0);
        grads.grad_iter().for_each(|x| assert!(*x == 2.0));
    }

    #[test]
    fn test_grad_into_iter() {
        let grads = [42.0f32; 2];
        assert_eq!(vec![42.0, 42.0], grads.into_grads().collect::<Vec<_>>());

        let grads = [[42.0f32, 0.0, 69.0]; 2];
        assert_eq!(
            vec![42.0, 0.0, 69.0, 42.0, 0.0, 69.0],
            grads.into_grads().collect::<Vec<_>>()
        );

        let grads = ([42.0f32; 2], [42.0f32; 3]);
        assert_eq!(
            vec![42.0, 42.0, 42.0, 42.0, 42.0],
            grads.into_grads().collect::<Vec<_>>()
        );
    }

    #[test]
    fn test_grad_merge() {
        let mut grads = [1.0f32; 2];
        grads.merge([2.0f32; 2], 0.5);
        grads.grad_iter().for_each(|x| assert!(*x == 1.5));

        let mut grads = [1.0f32; 2];
        grads.merge([2.0f32; 2], 0.0);
        grads.grad_iter().for_each(|x| assert!(*x == 1.0));

        let mut grads = [1.0f32; 2];
        grads.merge([2.0f32; 2], 1.0);
        grads.grad_iter().for_each(|x| assert!(*x == 2.0));
    }

    #[test]
    fn test_grad_iter_mut_with_class() {
        // Default for [E; N] is Other
        let mut grads = [0.0f32; 10];
        grads
            .grad_iter_mut_with_class()
            .for_each(|(_, c)| assert!(c == GradClass::Other));
        let mut grads = ([0.0f32; 10],);
        grads
            .grad_iter_mut_with_class()
            .for_each(|(_, c)| assert!(c == GradClass::Other));
        // Default for [[E; N1]; N2] is Connective
        let mut grads = [[0.0f32; 10]; 10];
        grads
            .grad_iter_mut_with_class()
            .for_each(|(_, c)| assert!(c == GradClass::Connective));

        // Test ClassWrapper
        let mut grads = (ClassWrapper::<[f32; 10], ClassBias> {
            g: [0.0f32; 10],
            m: ClassBias,
        },);
        grads
            .grad_iter_mut_with_class()
            .for_each(|(_, c)| assert!(c == GradClass::Bias));
    }
}
