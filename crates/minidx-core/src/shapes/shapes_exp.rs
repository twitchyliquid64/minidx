#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
pub mod dtypes {
    //! Module for data type related traits and structs.
    //!
    /// Represents a type where all 0 bits is a valid pattern.
    pub trait SafeZeros {}
    /// Represents a unit type, but no arithmetic.
    pub trait Unit: 'static + Copy + Clone + Default + std::fmt::Debug + PartialEq + PartialOrd + Send + Sync + std::marker::Unpin + SafeZeros {
        const ONE: Self;
    }
    impl SafeZeros for f32 {}
    impl Unit for f32 {
        const ONE: Self = 1.0;
    }
    impl SafeZeros for f64 {}
    impl Unit for f64 {
        const ONE: Self = 1.0;
    }
    impl SafeZeros for usize {}
    impl Unit for usize {
        const ONE: Self = 1;
    }
    impl SafeZeros for isize {}
    impl Unit for isize {
        const ONE: Self = 1;
    }
    impl SafeZeros for u8 {}
    impl Unit for u8 {
        const ONE: Self = 1;
    }
    impl SafeZeros for i8 {}
    impl Unit for i8 {
        const ONE: Self = 1;
    }
    impl SafeZeros for u16 {}
    impl Unit for u16 {
        const ONE: Self = 1;
    }
    impl SafeZeros for i16 {}
    impl Unit for i16 {
        const ONE: Self = 1;
    }
    impl SafeZeros for u32 {}
    impl Unit for u32 {
        const ONE: Self = 1;
    }
    impl SafeZeros for i32 {}
    impl Unit for i32 {
        const ONE: Self = 1;
    }
    impl SafeZeros for u64 {}
    impl Unit for u64 {
        const ONE: Self = 1;
    }
    impl SafeZeros for i64 {}
    impl Unit for i64 {
        const ONE: Self = 1;
    }
    impl SafeZeros for u128 {}
    impl Unit for u128 {
        const ONE: Self = 1;
    }
    impl SafeZeros for i128 {}
    impl Unit for i128 {
        const ONE: Self = 1;
    }
    impl SafeZeros for bool {}
    impl Unit for bool {
        const ONE: Self = true;
    }
    /// Represents something that has a [Unit].
    pub trait HasUnitType {
        type Unit: Unit;
    }
    /// Represents a data type or element of an array that can have
    /// arithmatic operations applied to it. The main difference
    /// between [Dtype] and [Unit] is that [`bool`] is [Unit], but
    /// not [Dtype].
    pub trait Dtype: Unit + std::ops::Add<
            Self,
            Output = Self,
        > + std::ops::Sub<
            Self,
            Output = Self,
        > + std::ops::Mul<
            Self,
            Output = Self,
        > + std::ops::Div<
            Self,
            Output = Self,
        > + std::ops::AddAssign + std::ops::SubAssign + std::ops::MulAssign + std::ops::DivAssign + num_traits::FromPrimitive + num_traits::ToPrimitive {}
    impl Dtype for f32 {}
    impl Dtype for f64 {}
    impl Dtype for i8 {}
    impl Dtype for i16 {}
    impl Dtype for i32 {}
    impl Dtype for i64 {}
    impl Dtype for i128 {}
    impl Dtype for isize {}
    impl Dtype for u8 {}
    impl Dtype for u16 {}
    impl Dtype for u32 {}
    impl Dtype for u64 {}
    impl Dtype for u128 {}
    impl Dtype for usize {}
    /// Represents something that has a [Dtype].
    pub trait HasDtype {
        type Dtype: Dtype;
    }
    /// Marker trait for types that are **not** [AMP].
    pub trait NotMixedPrecision {}
    impl NotMixedPrecision for f32 {}
    impl NotMixedPrecision for f64 {}
    impl NotMixedPrecision for i8 {}
    impl NotMixedPrecision for i16 {}
    impl NotMixedPrecision for i32 {}
    impl NotMixedPrecision for i64 {}
    impl NotMixedPrecision for i128 {}
    impl NotMixedPrecision for isize {}
    impl NotMixedPrecision for u8 {}
    impl NotMixedPrecision for u16 {}
    impl NotMixedPrecision for u32 {}
    impl NotMixedPrecision for u64 {}
    impl NotMixedPrecision for u128 {}
    impl NotMixedPrecision for usize {}
}
pub mod shapes {
    mod dim {
        /// Represents a single dimension of a multi dimensional [Shape]
        pub trait Dim: 'static + Copy + Clone + std::fmt::Debug + Send + Sync + Eq + PartialEq {
            fn size(&self) -> usize;
            fn from_size(size: usize) -> Option<Self>;
        }
        /// Represents a single dimension where all
        /// instances are guaranteed to be the same size at compile time.
        pub trait ConstDim: Default + Dim {
            const SIZE: usize;
        }
        impl Dim for usize {
            #[inline(always)]
            fn size(&self) -> usize {
                *self
            }
            #[inline(always)]
            fn from_size(size: usize) -> Option<Self> {
                Some(size)
            }
        }
        /// Represents a [Dim] with size known at compile time
        pub struct Const<const M: usize>;
        #[automatically_derived]
        impl<const M: usize> ::core::clone::Clone for Const<M> {
            #[inline]
            fn clone(&self) -> Const<M> {
                *self
            }
        }
        #[automatically_derived]
        impl<const M: usize> ::core::marker::Copy for Const<M> {}
        #[automatically_derived]
        impl<const M: usize> ::core::fmt::Debug for Const<M> {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::write_str(f, "Const")
            }
        }
        #[automatically_derived]
        impl<const M: usize> ::core::default::Default for Const<M> {
            #[inline]
            fn default() -> Const<M> {
                Const {}
            }
        }
        #[automatically_derived]
        impl<const M: usize> ::core::cmp::Eq for Const<M> {
            #[inline]
            #[doc(hidden)]
            #[coverage(off)]
            fn assert_receiver_is_total_eq(&self) -> () {}
        }
        #[automatically_derived]
        impl<const M: usize> ::core::marker::StructuralPartialEq for Const<M> {}
        #[automatically_derived]
        impl<const M: usize> ::core::cmp::PartialEq for Const<M> {
            #[inline]
            fn eq(&self, other: &Const<M>) -> bool {
                true
            }
        }
        impl<const M: usize> Dim for Const<M> {
            #[inline(always)]
            fn size(&self) -> usize {
                M
            }
            #[inline(always)]
            fn from_size(size: usize) -> Option<Self> {
                if size == M { Some(Const) } else { None }
            }
        }
        impl<const M: usize> ConstDim for Const<M> {
            const SIZE: usize = M;
        }
        impl<const N: usize> core::ops::Add<Const<N>> for usize {
            type Output = usize;
            fn add(self, _: Const<N>) -> Self::Output {
                self.size() + N
            }
        }
        impl<const N: usize> core::ops::Add<usize> for Const<N> {
            type Output = usize;
            fn add(self, rhs: usize) -> Self::Output {
                N + rhs.size()
            }
        }
        impl<const N: usize> core::ops::Mul<Const<N>> for usize {
            type Output = usize;
            fn mul(self, _: Const<N>) -> Self::Output {
                self.size() * N
            }
        }
        impl<const N: usize> core::ops::Mul<usize> for Const<N> {
            type Output = usize;
            fn mul(self, rhs: usize) -> Self::Output {
                N * rhs.size()
            }
        }
        impl<const N: usize> core::ops::Div<Const<N>> for usize {
            type Output = usize;
            fn div(self, _: Const<N>) -> Self::Output {
                self.size() / N
            }
        }
        impl<const N: usize> core::ops::Div<usize> for Const<N> {
            type Output = usize;
            fn div(self, rhs: usize) -> Self::Output {
                N / rhs.size()
            }
        }
    }
    pub use dim::{Const, ConstDim, Dim};
    mod axes {
        use super::Dim;
        /// Represents indices into the dimensions of shapes
        pub trait Axes: 'static + Default + Copy + Clone {
            type Array: IntoIterator<Item = isize>;
            fn as_array() -> Self::Array;
        }
        /// A singular axis, e.g. `Axis<0>` or `Axis<1>`
        pub struct Axis<const I: isize>;
        #[automatically_derived]
        impl<const I: isize> ::core::clone::Clone for Axis<I> {
            #[inline]
            fn clone(&self) -> Axis<I> {
                *self
            }
        }
        #[automatically_derived]
        impl<const I: isize> ::core::marker::Copy for Axis<I> {}
        #[automatically_derived]
        impl<const I: isize> ::core::fmt::Debug for Axis<I> {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::write_str(f, "Axis")
            }
        }
        #[automatically_derived]
        impl<const I: isize> ::core::default::Default for Axis<I> {
            #[inline]
            fn default() -> Axis<I> {
                Axis {}
            }
        }
        impl<const I: isize> Axes for Axis<I> {
            type Array = [isize; 1];
            #[inline(always)]
            fn as_array() -> Self::Array {
                [I]
            }
        }
        /// A set of 2 axes, e.g. `Axes2<0, 1>`, or `Axes2<1, 3>`.
        pub struct Axes2<const I: isize, const J: isize>;
        #[automatically_derived]
        impl<const I: isize, const J: isize> ::core::clone::Clone for Axes2<I, J> {
            #[inline]
            fn clone(&self) -> Axes2<I, J> {
                *self
            }
        }
        #[automatically_derived]
        impl<const I: isize, const J: isize> ::core::marker::Copy for Axes2<I, J> {}
        #[automatically_derived]
        impl<const I: isize, const J: isize> ::core::fmt::Debug for Axes2<I, J> {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::write_str(f, "Axes2")
            }
        }
        #[automatically_derived]
        impl<const I: isize, const J: isize> ::core::default::Default for Axes2<I, J> {
            #[inline]
            fn default() -> Axes2<I, J> {
                Axes2 {}
            }
        }
        impl<const I: isize, const J: isize> Axes for Axes2<I, J> {
            type Array = [isize; 2];
            #[inline(always)]
            fn as_array() -> Self::Array {
                [I, J]
            }
        }
        /// A set of 3 axes, e.g. `Axes3<1, 3, 4>`
        pub struct Axes3<const I: isize, const J: isize, const K: isize>;
        #[automatically_derived]
        impl<const I: isize, const J: isize, const K: isize> ::core::clone::Clone
        for Axes3<I, J, K> {
            #[inline]
            fn clone(&self) -> Axes3<I, J, K> {
                *self
            }
        }
        #[automatically_derived]
        impl<const I: isize, const J: isize, const K: isize> ::core::marker::Copy
        for Axes3<I, J, K> {}
        #[automatically_derived]
        impl<const I: isize, const J: isize, const K: isize> ::core::fmt::Debug
        for Axes3<I, J, K> {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::write_str(f, "Axes3")
            }
        }
        #[automatically_derived]
        impl<const I: isize, const J: isize, const K: isize> ::core::default::Default
        for Axes3<I, J, K> {
            #[inline]
            fn default() -> Axes3<I, J, K> {
                Axes3 {}
            }
        }
        impl<const I: isize, const J: isize, const K: isize> Axes for Axes3<I, J, K> {
            type Array = [isize; 3];
            #[inline(always)]
            fn as_array() -> Self::Array {
                [I, J, K]
            }
        }
        /// A set of 4 axes
        pub struct Axes4<const I: isize, const J: isize, const K: isize, const L: isize>;
        #[automatically_derived]
        impl<
            const I: isize,
            const J: isize,
            const K: isize,
            const L: isize,
        > ::core::clone::Clone for Axes4<I, J, K, L> {
            #[inline]
            fn clone(&self) -> Axes4<I, J, K, L> {
                *self
            }
        }
        #[automatically_derived]
        impl<
            const I: isize,
            const J: isize,
            const K: isize,
            const L: isize,
        > ::core::marker::Copy for Axes4<I, J, K, L> {}
        #[automatically_derived]
        impl<
            const I: isize,
            const J: isize,
            const K: isize,
            const L: isize,
        > ::core::fmt::Debug for Axes4<I, J, K, L> {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::write_str(f, "Axes4")
            }
        }
        #[automatically_derived]
        impl<
            const I: isize,
            const J: isize,
            const K: isize,
            const L: isize,
        > ::core::default::Default for Axes4<I, J, K, L> {
            #[inline]
            fn default() -> Axes4<I, J, K, L> {
                Axes4 {}
            }
        }
        impl<const I: isize, const J: isize, const K: isize, const L: isize> Axes
        for Axes4<I, J, K, L> {
            type Array = [isize; 4];
            #[inline(always)]
            fn as_array() -> Self::Array {
                [I, J, K, L]
            }
        }
        /// A set of 5 axes
        pub struct Axes5<
            const I: isize,
            const J: isize,
            const K: isize,
            const L: isize,
            const M: isize,
        >;
        #[automatically_derived]
        impl<
            const I: isize,
            const J: isize,
            const K: isize,
            const L: isize,
            const M: isize,
        > ::core::clone::Clone for Axes5<I, J, K, L, M> {
            #[inline]
            fn clone(&self) -> Axes5<I, J, K, L, M> {
                *self
            }
        }
        #[automatically_derived]
        impl<
            const I: isize,
            const J: isize,
            const K: isize,
            const L: isize,
            const M: isize,
        > ::core::marker::Copy for Axes5<I, J, K, L, M> {}
        #[automatically_derived]
        impl<
            const I: isize,
            const J: isize,
            const K: isize,
            const L: isize,
            const M: isize,
        > ::core::fmt::Debug for Axes5<I, J, K, L, M> {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::write_str(f, "Axes5")
            }
        }
        #[automatically_derived]
        impl<
            const I: isize,
            const J: isize,
            const K: isize,
            const L: isize,
            const M: isize,
        > ::core::default::Default for Axes5<I, J, K, L, M> {
            #[inline]
            fn default() -> Axes5<I, J, K, L, M> {
                Axes5 {}
            }
        }
        impl<
            const I: isize,
            const J: isize,
            const K: isize,
            const L: isize,
            const M: isize,
        > Axes for Axes5<I, J, K, L, M> {
            type Array = [isize; 5];
            #[inline(always)]
            fn as_array() -> Self::Array {
                [I, J, K, L, M]
            }
        }
        /// A set of 6 axes
        #[rustfmt::skip]
        pub struct Axes6<
            const I: isize,
            const J: isize,
            const K: isize,
            const L: isize,
            const M: isize,
            const N: isize,
        >;
        #[automatically_derived]
        impl<
            const I: isize,
            const J: isize,
            const K: isize,
            const L: isize,
            const M: isize,
            const N: isize,
        > ::core::clone::Clone for Axes6<I, J, K, L, M, N> {
            #[inline]
            fn clone(&self) -> Axes6<I, J, K, L, M, N> {
                *self
            }
        }
        #[automatically_derived]
        impl<
            const I: isize,
            const J: isize,
            const K: isize,
            const L: isize,
            const M: isize,
            const N: isize,
        > ::core::marker::Copy for Axes6<I, J, K, L, M, N> {}
        #[automatically_derived]
        impl<
            const I: isize,
            const J: isize,
            const K: isize,
            const L: isize,
            const M: isize,
            const N: isize,
        > ::core::fmt::Debug for Axes6<I, J, K, L, M, N> {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::write_str(f, "Axes6")
            }
        }
        #[automatically_derived]
        impl<
            const I: isize,
            const J: isize,
            const K: isize,
            const L: isize,
            const M: isize,
            const N: isize,
        > ::core::default::Default for Axes6<I, J, K, L, M, N> {
            #[inline]
            fn default() -> Axes6<I, J, K, L, M, N> {
                Axes6 {}
            }
        }
        #[rustfmt::skip]
        impl<
            const I: isize,
            const J: isize,
            const K: isize,
            const L: isize,
            const M: isize,
            const N: isize,
        > Axes for Axes6<I, J, K, L, M, N> {
            type Array = [isize; 6];
            #[inline(always)]
            fn as_array() -> Self::Array {
                [I, J, K, L, M, N]
            }
        }
        /// Represents something that has the axes `Ax`
        pub trait HasAxes<Ax> {
            /// Returns the number of elements in dimensions along `Ax`
            fn size(&self) -> usize;
        }
        impl HasAxes<Axis<0>> for () {
            #[inline(always)]
            fn size(&self) -> usize {
                1
            }
        }
        impl<D1: Dim> HasAxes<Axis<0>> for (D1,) {
            #[inline(always)]
            fn size(&self) -> usize {
                self.0.size()
            }
        }
        impl HasAxes<Axis<0>> for [usize; 1] {
            #[inline(always)]
            fn size(&self) -> usize {
                self[0]
            }
        }
        impl<D1: Dim, D2: Dim> HasAxes<Axis<0>> for (D1, D2) {
            #[inline(always)]
            fn size(&self) -> usize {
                self.0.size()
            }
        }
        impl HasAxes<Axis<0>> for [usize; 2] {
            #[inline(always)]
            fn size(&self) -> usize {
                self[0]
            }
        }
        impl<D1: Dim, D2: Dim> HasAxes<Axis<1>> for (D1, D2) {
            #[inline(always)]
            fn size(&self) -> usize {
                self.1.size()
            }
        }
        impl HasAxes<Axis<1>> for [usize; 2] {
            #[inline(always)]
            fn size(&self) -> usize {
                self[1]
            }
        }
        impl<D1: Dim, D2: Dim, D3: Dim> HasAxes<Axis<0>> for (D1, D2, D3) {
            #[inline(always)]
            fn size(&self) -> usize {
                self.0.size()
            }
        }
        impl HasAxes<Axis<0>> for [usize; 3] {
            #[inline(always)]
            fn size(&self) -> usize {
                self[0]
            }
        }
        impl<D1: Dim, D2: Dim, D3: Dim> HasAxes<Axis<1>> for (D1, D2, D3) {
            #[inline(always)]
            fn size(&self) -> usize {
                self.1.size()
            }
        }
        impl HasAxes<Axis<1>> for [usize; 3] {
            #[inline(always)]
            fn size(&self) -> usize {
                self[1]
            }
        }
        impl<D1: Dim, D2: Dim, D3: Dim> HasAxes<Axis<2>> for (D1, D2, D3) {
            #[inline(always)]
            fn size(&self) -> usize {
                self.2.size()
            }
        }
        impl HasAxes<Axis<2>> for [usize; 3] {
            #[inline(always)]
            fn size(&self) -> usize {
                self[2]
            }
        }
        impl<D1: Dim, D2: Dim, D3: Dim, D4: Dim> HasAxes<Axis<0>> for (D1, D2, D3, D4) {
            #[inline(always)]
            fn size(&self) -> usize {
                self.0.size()
            }
        }
        impl HasAxes<Axis<0>> for [usize; 4] {
            #[inline(always)]
            fn size(&self) -> usize {
                self[0]
            }
        }
        impl<D1: Dim, D2: Dim, D3: Dim, D4: Dim> HasAxes<Axis<1>> for (D1, D2, D3, D4) {
            #[inline(always)]
            fn size(&self) -> usize {
                self.1.size()
            }
        }
        impl HasAxes<Axis<1>> for [usize; 4] {
            #[inline(always)]
            fn size(&self) -> usize {
                self[1]
            }
        }
        impl<D1: Dim, D2: Dim, D3: Dim, D4: Dim> HasAxes<Axis<2>> for (D1, D2, D3, D4) {
            #[inline(always)]
            fn size(&self) -> usize {
                self.2.size()
            }
        }
        impl HasAxes<Axis<2>> for [usize; 4] {
            #[inline(always)]
            fn size(&self) -> usize {
                self[2]
            }
        }
        impl<D1: Dim, D2: Dim, D3: Dim, D4: Dim> HasAxes<Axis<3>> for (D1, D2, D3, D4) {
            #[inline(always)]
            fn size(&self) -> usize {
                self.3.size()
            }
        }
        impl HasAxes<Axis<3>> for [usize; 4] {
            #[inline(always)]
            fn size(&self) -> usize {
                self[3]
            }
        }
        impl<D1: Dim, D2: Dim, D3: Dim, D4: Dim, D5: Dim> HasAxes<Axis<0>>
        for (D1, D2, D3, D4, D5) {
            #[inline(always)]
            fn size(&self) -> usize {
                self.0.size()
            }
        }
        impl HasAxes<Axis<0>> for [usize; 5] {
            #[inline(always)]
            fn size(&self) -> usize {
                self[0]
            }
        }
        impl<D1: Dim, D2: Dim, D3: Dim, D4: Dim, D5: Dim> HasAxes<Axis<1>>
        for (D1, D2, D3, D4, D5) {
            #[inline(always)]
            fn size(&self) -> usize {
                self.1.size()
            }
        }
        impl HasAxes<Axis<1>> for [usize; 5] {
            #[inline(always)]
            fn size(&self) -> usize {
                self[1]
            }
        }
        impl<D1: Dim, D2: Dim, D3: Dim, D4: Dim, D5: Dim> HasAxes<Axis<2>>
        for (D1, D2, D3, D4, D5) {
            #[inline(always)]
            fn size(&self) -> usize {
                self.2.size()
            }
        }
        impl HasAxes<Axis<2>> for [usize; 5] {
            #[inline(always)]
            fn size(&self) -> usize {
                self[2]
            }
        }
        impl<D1: Dim, D2: Dim, D3: Dim, D4: Dim, D5: Dim> HasAxes<Axis<3>>
        for (D1, D2, D3, D4, D5) {
            #[inline(always)]
            fn size(&self) -> usize {
                self.3.size()
            }
        }
        impl HasAxes<Axis<3>> for [usize; 5] {
            #[inline(always)]
            fn size(&self) -> usize {
                self[3]
            }
        }
        impl<D1: Dim, D2: Dim, D3: Dim, D4: Dim, D5: Dim> HasAxes<Axis<4>>
        for (D1, D2, D3, D4, D5) {
            #[inline(always)]
            fn size(&self) -> usize {
                self.4.size()
            }
        }
        impl HasAxes<Axis<4>> for [usize; 5] {
            #[inline(always)]
            fn size(&self) -> usize {
                self[4]
            }
        }
        impl<D1: Dim, D2: Dim, D3: Dim, D4: Dim, D5: Dim, D6: Dim> HasAxes<Axis<0>>
        for (D1, D2, D3, D4, D5, D6) {
            #[inline(always)]
            fn size(&self) -> usize {
                self.0.size()
            }
        }
        impl HasAxes<Axis<0>> for [usize; 6] {
            #[inline(always)]
            fn size(&self) -> usize {
                self[0]
            }
        }
        impl<D1: Dim, D2: Dim, D3: Dim, D4: Dim, D5: Dim, D6: Dim> HasAxes<Axis<1>>
        for (D1, D2, D3, D4, D5, D6) {
            #[inline(always)]
            fn size(&self) -> usize {
                self.1.size()
            }
        }
        impl HasAxes<Axis<1>> for [usize; 6] {
            #[inline(always)]
            fn size(&self) -> usize {
                self[1]
            }
        }
        impl<D1: Dim, D2: Dim, D3: Dim, D4: Dim, D5: Dim, D6: Dim> HasAxes<Axis<2>>
        for (D1, D2, D3, D4, D5, D6) {
            #[inline(always)]
            fn size(&self) -> usize {
                self.2.size()
            }
        }
        impl HasAxes<Axis<2>> for [usize; 6] {
            #[inline(always)]
            fn size(&self) -> usize {
                self[2]
            }
        }
        impl<D1: Dim, D2: Dim, D3: Dim, D4: Dim, D5: Dim, D6: Dim> HasAxes<Axis<3>>
        for (D1, D2, D3, D4, D5, D6) {
            #[inline(always)]
            fn size(&self) -> usize {
                self.3.size()
            }
        }
        impl HasAxes<Axis<3>> for [usize; 6] {
            #[inline(always)]
            fn size(&self) -> usize {
                self[3]
            }
        }
        impl<D1: Dim, D2: Dim, D3: Dim, D4: Dim, D5: Dim, D6: Dim> HasAxes<Axis<4>>
        for (D1, D2, D3, D4, D5, D6) {
            #[inline(always)]
            fn size(&self) -> usize {
                self.4.size()
            }
        }
        impl HasAxes<Axis<4>> for [usize; 6] {
            #[inline(always)]
            fn size(&self) -> usize {
                self[4]
            }
        }
        impl<D1: Dim, D2: Dim, D3: Dim, D4: Dim, D5: Dim, D6: Dim> HasAxes<Axis<5>>
        for (D1, D2, D3, D4, D5, D6) {
            #[inline(always)]
            fn size(&self) -> usize {
                self.5.size()
            }
        }
        impl HasAxes<Axis<5>> for [usize; 6] {
            #[inline(always)]
            fn size(&self) -> usize {
                self[5]
            }
        }
        impl<const I: isize, const J: isize, S> HasAxes<Axes2<I, J>> for S
        where
            Self: HasAxes<Axis<I>> + HasAxes<Axis<J>>,
        {
            #[inline(always)]
            fn size(&self) -> usize {
                <Self as HasAxes<Axis<I>>>::size(self)
                    * <Self as HasAxes<Axis<J>>>::size(self)
            }
        }
        impl<const I: isize, const J: isize, const K: isize, S> HasAxes<Axes3<I, J, K>>
        for S
        where
            Self: HasAxes<Axis<I>> + HasAxes<Axis<J>> + HasAxes<Axis<K>>,
        {
            #[inline(always)]
            fn size(&self) -> usize {
                <Self as HasAxes<Axis<I>>>::size(self)
                    * <Self as HasAxes<Axis<J>>>::size(self)
                    * <Self as HasAxes<Axis<K>>>::size(self)
            }
        }
        impl<
            const I: isize,
            const J: isize,
            const K: isize,
            const L: isize,
            S,
        > HasAxes<Axes4<I, J, K, L>> for S
        where
            Self: HasAxes<Axis<I>> + HasAxes<Axis<J>> + HasAxes<Axis<K>>
                + HasAxes<Axis<L>>,
        {
            #[inline(always)]
            fn size(&self) -> usize {
                <Self as HasAxes<Axis<I>>>::size(self)
                    * <Self as HasAxes<Axis<J>>>::size(self)
                    * <Self as HasAxes<Axis<K>>>::size(self)
                    * <Self as HasAxes<Axis<L>>>::size(self)
            }
        }
        impl<
            const I: isize,
            const J: isize,
            const K: isize,
            const L: isize,
            const M: isize,
            S,
        > HasAxes<Axes5<I, J, K, L, M>> for S
        where
            Self: HasAxes<Axis<I>> + HasAxes<Axis<J>> + HasAxes<Axis<K>>
                + HasAxes<Axis<L>> + HasAxes<Axis<M>>,
        {
            #[inline(always)]
            fn size(&self) -> usize {
                <Self as HasAxes<Axis<I>>>::size(self)
                    * <Self as HasAxes<Axis<J>>>::size(self)
                    * <Self as HasAxes<Axis<K>>>::size(self)
                    * <Self as HasAxes<Axis<L>>>::size(self)
                    * <Self as HasAxes<Axis<M>>>::size(self)
            }
        }
        impl<
            const I: isize,
            const J: isize,
            const K: isize,
            const L: isize,
            const M: isize,
            const N: isize,
            S,
        > HasAxes<Axes6<I, J, K, L, M, N>> for S
        where
            Self: HasAxes<Axis<I>> + HasAxes<Axis<J>> + HasAxes<Axis<K>>
                + HasAxes<Axis<L>> + HasAxes<Axis<M>> + HasAxes<Axis<N>>,
        {
            #[inline(always)]
            fn size(&self) -> usize {
                <Self as HasAxes<Axis<I>>>::size(self)
                    * <Self as HasAxes<Axis<J>>>::size(self)
                    * <Self as HasAxes<Axis<K>>>::size(self)
                    * <Self as HasAxes<Axis<L>>>::size(self)
                    * <Self as HasAxes<Axis<M>>>::size(self)
                    * <Self as HasAxes<Axis<N>>>::size(self)
            }
        }
    }
    pub use axes::{Axes, Axes2, Axes3, Axes4, Axes5, Axes6, Axis, HasAxes};
    mod broadcasts {
        use super::*;
        /// Marker for shapes that can be reduced to [Shape] `S` along [Axes] `Ax`.
        pub trait ReduceShapeTo<S, Ax>: HasAxes<Ax> + Sized {}
        /// Marker for shapes that can be broadcasted to [Shape] `S` along [Axes] `Ax`.
        pub trait BroadcastShapeTo<S, Ax>: Sized {}
        /// Marker for shapes that can have their [Axes] `Ax` reduced. See Self::Reduced
        /// for the resulting type.
        pub trait ReduceShape<
            Ax,
        >: Sized + HasAxes<Ax> + ReduceShapeTo<Self::Reduced, Ax> {
            type Reduced: Shape + BroadcastShapeTo<Self, Ax>;
        }
        impl ReduceShapeTo<(), Axis<0>> for () {}
        impl ReduceShape<Axis<0>> for () {
            type Reduced = ();
        }
        impl<
            Src: Shape,
            Dst: Shape + ReduceShapeTo<Src, Ax>,
            Ax,
        > BroadcastShapeTo<Dst, Ax> for Src {}
        pub(crate) use length;
        impl<A: Dim> ReduceShapeTo<(), Axis<{ { 0 } }>> for (A,) {}
        impl<A: Dim> ReduceShape<Axis<{ { 0 } }>> for (A,) {
            type Reduced = ();
        }
        impl ReduceShapeTo<(), Axis<{ { 0 } }>> for [usize; { 1 + 0 }] {}
        impl ReduceShape<Axis<{ { 0 } }>> for [usize; { 1 + 0 }] {
            type Reduced = ();
        }
        impl<A: Dim, B: Dim> ReduceShapeTo<(), Axes2<{ { 0 } }, { { 1 + 0 } }>>
        for (A, B) {}
        impl<A: Dim, B: Dim> ReduceShape<Axes2<{ { 0 } }, { { 1 + 0 } }>> for (A, B) {
            type Reduced = ();
        }
        impl ReduceShapeTo<(), Axes2<{ { 0 } }, { { 1 + 0 } }>>
        for [usize; { 1 + (1 + 0) }] {}
        impl ReduceShape<Axes2<{ { 0 } }, { { 1 + 0 } }>> for [usize; { 1 + (1 + 0) }] {
            type Reduced = ();
        }
        impl<
            A: Dim,
            B: Dim,
            C: Dim,
        > ReduceShapeTo<(), Axes3<{ { 0 } }, { { 1 + 0 } }, { { 1 + (1 + 0) } }>>
        for (A, B, C) {}
        impl<
            A: Dim,
            B: Dim,
            C: Dim,
        > ReduceShape<Axes3<{ { 0 } }, { { 1 + 0 } }, { { 1 + (1 + 0) } }>>
        for (A, B, C) {
            type Reduced = ();
        }
        impl ReduceShapeTo<(), Axes3<{ { 0 } }, { { 1 + 0 } }, { { 1 + (1 + 0) } }>>
        for [usize; { 1 + (1 + (1 + 0)) }] {}
        impl ReduceShape<Axes3<{ { 0 } }, { { 1 + 0 } }, { { 1 + (1 + 0) } }>>
        for [usize; { 1 + (1 + (1 + 0)) }] {
            type Reduced = ();
        }
        impl<
            A: Dim,
            B: Dim,
            C: Dim,
            D: Dim,
        > ReduceShapeTo<
            (),
            Axes4<
                { { 0 } },
                { { 1 + 0 } },
                { { 1 + (1 + 0) } },
                { { 1 + (1 + (1 + 0)) } },
            >,
        > for (A, B, C, D) {}
        impl<
            A: Dim,
            B: Dim,
            C: Dim,
            D: Dim,
        > ReduceShape<
            Axes4<
                { { 0 } },
                { { 1 + 0 } },
                { { 1 + (1 + 0) } },
                { { 1 + (1 + (1 + 0)) } },
            >,
        > for (A, B, C, D) {
            type Reduced = ();
        }
        impl ReduceShapeTo<
            (),
            Axes4<
                { { 0 } },
                { { 1 + 0 } },
                { { 1 + (1 + 0) } },
                { { 1 + (1 + (1 + 0)) } },
            >,
        > for [usize; { 1 + (1 + (1 + (1 + 0))) }] {}
        impl ReduceShape<
            Axes4<
                { { 0 } },
                { { 1 + 0 } },
                { { 1 + (1 + 0) } },
                { { 1 + (1 + (1 + 0)) } },
            >,
        > for [usize; { 1 + (1 + (1 + (1 + 0))) }] {
            type Reduced = ();
        }
        impl<
            A: Dim,
            B: Dim,
            C: Dim,
            D: Dim,
            E: Dim,
        > ReduceShapeTo<
            (),
            Axes5<
                { { 0 } },
                { { 1 + 0 } },
                { { 1 + (1 + 0) } },
                { { 1 + (1 + (1 + 0)) } },
                { { 1 + (1 + (1 + (1 + 0))) } },
            >,
        > for (A, B, C, D, E) {}
        impl<
            A: Dim,
            B: Dim,
            C: Dim,
            D: Dim,
            E: Dim,
        > ReduceShape<
            Axes5<
                { { 0 } },
                { { 1 + 0 } },
                { { 1 + (1 + 0) } },
                { { 1 + (1 + (1 + 0)) } },
                { { 1 + (1 + (1 + (1 + 0))) } },
            >,
        > for (A, B, C, D, E) {
            type Reduced = ();
        }
        impl ReduceShapeTo<
            (),
            Axes5<
                { { 0 } },
                { { 1 + 0 } },
                { { 1 + (1 + 0) } },
                { { 1 + (1 + (1 + 0)) } },
                { { 1 + (1 + (1 + (1 + 0))) } },
            >,
        > for [usize; { 1 + (1 + (1 + (1 + (1 + 0)))) }] {}
        impl ReduceShape<
            Axes5<
                { { 0 } },
                { { 1 + 0 } },
                { { 1 + (1 + 0) } },
                { { 1 + (1 + (1 + 0)) } },
                { { 1 + (1 + (1 + (1 + 0))) } },
            >,
        > for [usize; { 1 + (1 + (1 + (1 + (1 + 0)))) }] {
            type Reduced = ();
        }
        impl<
            A: Dim,
            B: Dim,
            C: Dim,
            D: Dim,
            E: Dim,
            F: Dim,
        > ReduceShapeTo<
            (),
            Axes6<
                { { 0 } },
                { { 1 + 0 } },
                { { 1 + (1 + 0) } },
                { { 1 + (1 + (1 + 0)) } },
                { { 1 + (1 + (1 + (1 + 0))) } },
                { { 1 + (1 + (1 + (1 + (1 + 0)))) } },
            >,
        > for (A, B, C, D, E, F) {}
        impl<
            A: Dim,
            B: Dim,
            C: Dim,
            D: Dim,
            E: Dim,
            F: Dim,
        > ReduceShape<
            Axes6<
                { { 0 } },
                { { 1 + 0 } },
                { { 1 + (1 + 0) } },
                { { 1 + (1 + (1 + 0)) } },
                { { 1 + (1 + (1 + (1 + 0))) } },
                { { 1 + (1 + (1 + (1 + (1 + 0)))) } },
            >,
        > for (A, B, C, D, E, F) {
            type Reduced = ();
        }
        impl ReduceShapeTo<
            (),
            Axes6<
                { { 0 } },
                { { 1 + 0 } },
                { { 1 + (1 + 0) } },
                { { 1 + (1 + (1 + 0)) } },
                { { 1 + (1 + (1 + (1 + 0))) } },
                { { 1 + (1 + (1 + (1 + (1 + 0)))) } },
            >,
        > for [usize; { 1 + (1 + (1 + (1 + (1 + (1 + 0))))) }] {}
        impl ReduceShape<
            Axes6<
                { { 0 } },
                { { 1 + 0 } },
                { { 1 + (1 + 0) } },
                { { 1 + (1 + (1 + 0)) } },
                { { 1 + (1 + (1 + (1 + 0))) } },
                { { 1 + (1 + (1 + (1 + (1 + 0)))) } },
            >,
        > for [usize; { 1 + (1 + (1 + (1 + (1 + (1 + 0))))) }] {
            type Reduced = ();
        }
        impl<
            A: Dim,
            B: Dim,
            C: Dim,
            D: Dim,
            E: Dim,
            F: Dim,
        > ReduceShapeTo<
            (F,),
            Axes5<
                { { 0 } },
                { { 1 + 0 } },
                { { 1 + (1 + 0) } },
                { { 1 + (1 + (1 + 0)) } },
                { { 1 + (1 + (1 + (1 + 0))) } },
            >,
        > for (A, B, C, D, E, F) {}
        impl<
            A: Dim,
            B: Dim,
            C: Dim,
            D: Dim,
            E: Dim,
            F: Dim,
        > ReduceShape<
            Axes5<
                { { 0 } },
                { { 1 + 0 } },
                { { 1 + (1 + 0) } },
                { { 1 + (1 + (1 + 0)) } },
                { { 1 + (1 + (1 + (1 + 0))) } },
            >,
        > for (A, B, C, D, E, F) {
            type Reduced = (F,);
        }
        impl ReduceShapeTo<
            [usize; { 1 + 0 }],
            Axes5<
                { { 0 } },
                { { 1 + 0 } },
                { { 1 + (1 + 0) } },
                { { 1 + (1 + (1 + 0)) } },
                { { 1 + (1 + (1 + (1 + 0))) } },
            >,
        > for [usize; { 1 + (1 + (1 + (1 + (1 + (1 + 0))))) }] {}
        impl ReduceShape<
            Axes5<
                { { 0 } },
                { { 1 + 0 } },
                { { 1 + (1 + 0) } },
                { { 1 + (1 + (1 + 0)) } },
                { { 1 + (1 + (1 + (1 + 0))) } },
            >,
        > for [usize; { 1 + (1 + (1 + (1 + (1 + (1 + 0))))) }] {
            type Reduced = [usize; { 1 + 0 }];
        }
        impl<
            A: Dim,
            B: Dim,
            C: Dim,
            D: Dim,
            E: Dim,
        > ReduceShapeTo<
            (E,),
            Axes4<
                { { 0 } },
                { { 1 + 0 } },
                { { 1 + (1 + 0) } },
                { { 1 + (1 + (1 + 0)) } },
            >,
        > for (A, B, C, D, E) {}
        impl<
            A: Dim,
            B: Dim,
            C: Dim,
            D: Dim,
            E: Dim,
        > ReduceShape<
            Axes4<
                { { 0 } },
                { { 1 + 0 } },
                { { 1 + (1 + 0) } },
                { { 1 + (1 + (1 + 0)) } },
            >,
        > for (A, B, C, D, E) {
            type Reduced = (E,);
        }
        impl ReduceShapeTo<
            [usize; { 1 + 0 }],
            Axes4<
                { { 0 } },
                { { 1 + 0 } },
                { { 1 + (1 + 0) } },
                { { 1 + (1 + (1 + 0)) } },
            >,
        > for [usize; { 1 + (1 + (1 + (1 + (1 + 0)))) }] {}
        impl ReduceShape<
            Axes4<
                { { 0 } },
                { { 1 + 0 } },
                { { 1 + (1 + 0) } },
                { { 1 + (1 + (1 + 0)) } },
            >,
        > for [usize; { 1 + (1 + (1 + (1 + (1 + 0)))) }] {
            type Reduced = [usize; { 1 + 0 }];
        }
        impl<
            A: Dim,
            B: Dim,
            C: Dim,
            D: Dim,
            E: Dim,
            F: Dim,
        > ReduceShapeTo<
            (E,),
            Axes5<
                { { 0 } },
                { { 1 + 0 } },
                { { 1 + (1 + 0) } },
                { { 1 + (1 + (1 + 0)) } },
                { { 1 + (1 + (1 + (1 + (1 + 0)))) } },
            >,
        > for (A, B, C, D, E, F) {}
        impl<
            A: Dim,
            B: Dim,
            C: Dim,
            D: Dim,
            E: Dim,
            F: Dim,
        > ReduceShape<
            Axes5<
                { { 0 } },
                { { 1 + 0 } },
                { { 1 + (1 + 0) } },
                { { 1 + (1 + (1 + 0)) } },
                { { 1 + (1 + (1 + (1 + (1 + 0)))) } },
            >,
        > for (A, B, C, D, E, F) {
            type Reduced = (E,);
        }
        impl ReduceShapeTo<
            [usize; { 1 + 0 }],
            Axes5<
                { { 0 } },
                { { 1 + 0 } },
                { { 1 + (1 + 0) } },
                { { 1 + (1 + (1 + 0)) } },
                { { 1 + (1 + (1 + (1 + (1 + 0)))) } },
            >,
        > for [usize; { 1 + (1 + (1 + (1 + (1 + (1 + 0))))) }] {}
        impl ReduceShape<
            Axes5<
                { { 0 } },
                { { 1 + 0 } },
                { { 1 + (1 + 0) } },
                { { 1 + (1 + (1 + 0)) } },
                { { 1 + (1 + (1 + (1 + (1 + 0)))) } },
            >,
        > for [usize; { 1 + (1 + (1 + (1 + (1 + (1 + 0))))) }] {
            type Reduced = [usize; { 1 + 0 }];
        }
        impl<
            A: Dim,
            B: Dim,
            C: Dim,
            D: Dim,
            E: Dim,
            F: Dim,
        > ReduceShapeTo<
            (E, F),
            Axes4<
                { { 0 } },
                { { 1 + 0 } },
                { { 1 + (1 + 0) } },
                { { 1 + (1 + (1 + 0)) } },
            >,
        > for (A, B, C, D, E, F) {}
        impl<
            A: Dim,
            B: Dim,
            C: Dim,
            D: Dim,
            E: Dim,
            F: Dim,
        > ReduceShape<
            Axes4<
                { { 0 } },
                { { 1 + 0 } },
                { { 1 + (1 + 0) } },
                { { 1 + (1 + (1 + 0)) } },
            >,
        > for (A, B, C, D, E, F) {
            type Reduced = (E, F);
        }
        impl ReduceShapeTo<
            [usize; { 1 + (1 + 0) }],
            Axes4<
                { { 0 } },
                { { 1 + 0 } },
                { { 1 + (1 + 0) } },
                { { 1 + (1 + (1 + 0)) } },
            >,
        > for [usize; { 1 + (1 + (1 + (1 + (1 + (1 + 0))))) }] {}
        impl ReduceShape<
            Axes4<
                { { 0 } },
                { { 1 + 0 } },
                { { 1 + (1 + 0) } },
                { { 1 + (1 + (1 + 0)) } },
            >,
        > for [usize; { 1 + (1 + (1 + (1 + (1 + (1 + 0))))) }] {
            type Reduced = [usize; { 1 + (1 + 0) }];
        }
        impl<
            A: Dim,
            B: Dim,
            C: Dim,
            D: Dim,
        > ReduceShapeTo<(D,), Axes3<{ { 0 } }, { { 1 + 0 } }, { { 1 + (1 + 0) } }>>
        for (A, B, C, D) {}
        impl<
            A: Dim,
            B: Dim,
            C: Dim,
            D: Dim,
        > ReduceShape<Axes3<{ { 0 } }, { { 1 + 0 } }, { { 1 + (1 + 0) } }>>
        for (A, B, C, D) {
            type Reduced = (D,);
        }
        impl ReduceShapeTo<
            [usize; { 1 + 0 }],
            Axes3<{ { 0 } }, { { 1 + 0 } }, { { 1 + (1 + 0) } }>,
        > for [usize; { 1 + (1 + (1 + (1 + 0))) }] {}
        impl ReduceShape<Axes3<{ { 0 } }, { { 1 + 0 } }, { { 1 + (1 + 0) } }>>
        for [usize; { 1 + (1 + (1 + (1 + 0))) }] {
            type Reduced = [usize; { 1 + 0 }];
        }
        impl<
            A: Dim,
            B: Dim,
            C: Dim,
            D: Dim,
            E: Dim,
        > ReduceShapeTo<
            (D,),
            Axes4<
                { { 0 } },
                { { 1 + 0 } },
                { { 1 + (1 + 0) } },
                { { 1 + (1 + (1 + (1 + 0))) } },
            >,
        > for (A, B, C, D, E) {}
        impl<
            A: Dim,
            B: Dim,
            C: Dim,
            D: Dim,
            E: Dim,
        > ReduceShape<
            Axes4<
                { { 0 } },
                { { 1 + 0 } },
                { { 1 + (1 + 0) } },
                { { 1 + (1 + (1 + (1 + 0))) } },
            >,
        > for (A, B, C, D, E) {
            type Reduced = (D,);
        }
        impl ReduceShapeTo<
            [usize; { 1 + 0 }],
            Axes4<
                { { 0 } },
                { { 1 + 0 } },
                { { 1 + (1 + 0) } },
                { { 1 + (1 + (1 + (1 + 0))) } },
            >,
        > for [usize; { 1 + (1 + (1 + (1 + (1 + 0)))) }] {}
        impl ReduceShape<
            Axes4<
                { { 0 } },
                { { 1 + 0 } },
                { { 1 + (1 + 0) } },
                { { 1 + (1 + (1 + (1 + 0))) } },
            >,
        > for [usize; { 1 + (1 + (1 + (1 + (1 + 0)))) }] {
            type Reduced = [usize; { 1 + 0 }];
        }
        impl<
            A: Dim,
            B: Dim,
            C: Dim,
            D: Dim,
            E: Dim,
            F: Dim,
        > ReduceShapeTo<
            (D,),
            Axes5<
                { { 0 } },
                { { 1 + 0 } },
                { { 1 + (1 + 0) } },
                { { 1 + (1 + (1 + (1 + 0))) } },
                { { 1 + (1 + (1 + (1 + (1 + 0)))) } },
            >,
        > for (A, B, C, D, E, F) {}
        impl<
            A: Dim,
            B: Dim,
            C: Dim,
            D: Dim,
            E: Dim,
            F: Dim,
        > ReduceShape<
            Axes5<
                { { 0 } },
                { { 1 + 0 } },
                { { 1 + (1 + 0) } },
                { { 1 + (1 + (1 + (1 + 0))) } },
                { { 1 + (1 + (1 + (1 + (1 + 0)))) } },
            >,
        > for (A, B, C, D, E, F) {
            type Reduced = (D,);
        }
        impl ReduceShapeTo<
            [usize; { 1 + 0 }],
            Axes5<
                { { 0 } },
                { { 1 + 0 } },
                { { 1 + (1 + 0) } },
                { { 1 + (1 + (1 + (1 + 0))) } },
                { { 1 + (1 + (1 + (1 + (1 + 0)))) } },
            >,
        > for [usize; { 1 + (1 + (1 + (1 + (1 + (1 + 0))))) }] {}
        impl ReduceShape<
            Axes5<
                { { 0 } },
                { { 1 + 0 } },
                { { 1 + (1 + 0) } },
                { { 1 + (1 + (1 + (1 + 0))) } },
                { { 1 + (1 + (1 + (1 + (1 + 0)))) } },
            >,
        > for [usize; { 1 + (1 + (1 + (1 + (1 + (1 + 0))))) }] {
            type Reduced = [usize; { 1 + 0 }];
        }
        impl<
            A: Dim,
            B: Dim,
            C: Dim,
            D: Dim,
            E: Dim,
            F: Dim,
        > ReduceShapeTo<
            (D, F),
            Axes4<
                { { 0 } },
                { { 1 + 0 } },
                { { 1 + (1 + 0) } },
                { { 1 + (1 + (1 + (1 + 0))) } },
            >,
        > for (A, B, C, D, E, F) {}
        impl<
            A: Dim,
            B: Dim,
            C: Dim,
            D: Dim,
            E: Dim,
            F: Dim,
        > ReduceShape<
            Axes4<
                { { 0 } },
                { { 1 + 0 } },
                { { 1 + (1 + 0) } },
                { { 1 + (1 + (1 + (1 + 0))) } },
            >,
        > for (A, B, C, D, E, F) {
            type Reduced = (D, F);
        }
        impl ReduceShapeTo<
            [usize; { 1 + (1 + 0) }],
            Axes4<
                { { 0 } },
                { { 1 + 0 } },
                { { 1 + (1 + 0) } },
                { { 1 + (1 + (1 + (1 + 0))) } },
            >,
        > for [usize; { 1 + (1 + (1 + (1 + (1 + (1 + 0))))) }] {}
        impl ReduceShape<
            Axes4<
                { { 0 } },
                { { 1 + 0 } },
                { { 1 + (1 + 0) } },
                { { 1 + (1 + (1 + (1 + 0))) } },
            >,
        > for [usize; { 1 + (1 + (1 + (1 + (1 + (1 + 0))))) }] {
            type Reduced = [usize; { 1 + (1 + 0) }];
        }
        impl<
            A: Dim,
            B: Dim,
            C: Dim,
            D: Dim,
            E: Dim,
        > ReduceShapeTo<(D, E), Axes3<{ { 0 } }, { { 1 + 0 } }, { { 1 + (1 + 0) } }>>
        for (A, B, C, D, E) {}
        impl<
            A: Dim,
            B: Dim,
            C: Dim,
            D: Dim,
            E: Dim,
        > ReduceShape<Axes3<{ { 0 } }, { { 1 + 0 } }, { { 1 + (1 + 0) } }>>
        for (A, B, C, D, E) {
            type Reduced = (D, E);
        }
        impl ReduceShapeTo<
            [usize; { 1 + (1 + 0) }],
            Axes3<{ { 0 } }, { { 1 + 0 } }, { { 1 + (1 + 0) } }>,
        > for [usize; { 1 + (1 + (1 + (1 + (1 + 0)))) }] {}
        impl ReduceShape<Axes3<{ { 0 } }, { { 1 + 0 } }, { { 1 + (1 + 0) } }>>
        for [usize; { 1 + (1 + (1 + (1 + (1 + 0)))) }] {
            type Reduced = [usize; { 1 + (1 + 0) }];
        }
        impl<
            A: Dim,
            B: Dim,
            C: Dim,
            D: Dim,
            E: Dim,
            F: Dim,
        > ReduceShapeTo<
            (D, E),
            Axes4<
                { { 0 } },
                { { 1 + 0 } },
                { { 1 + (1 + 0) } },
                { { 1 + (1 + (1 + (1 + (1 + 0)))) } },
            >,
        > for (A, B, C, D, E, F) {}
        impl<
            A: Dim,
            B: Dim,
            C: Dim,
            D: Dim,
            E: Dim,
            F: Dim,
        > ReduceShape<
            Axes4<
                { { 0 } },
                { { 1 + 0 } },
                { { 1 + (1 + 0) } },
                { { 1 + (1 + (1 + (1 + (1 + 0)))) } },
            >,
        > for (A, B, C, D, E, F) {
            type Reduced = (D, E);
        }
        impl ReduceShapeTo<
            [usize; { 1 + (1 + 0) }],
            Axes4<
                { { 0 } },
                { { 1 + 0 } },
                { { 1 + (1 + 0) } },
                { { 1 + (1 + (1 + (1 + (1 + 0)))) } },
            >,
        > for [usize; { 1 + (1 + (1 + (1 + (1 + (1 + 0))))) }] {}
        impl ReduceShape<
            Axes4<
                { { 0 } },
                { { 1 + 0 } },
                { { 1 + (1 + 0) } },
                { { 1 + (1 + (1 + (1 + (1 + 0)))) } },
            >,
        > for [usize; { 1 + (1 + (1 + (1 + (1 + (1 + 0))))) }] {
            type Reduced = [usize; { 1 + (1 + 0) }];
        }
        impl<
            A: Dim,
            B: Dim,
            C: Dim,
            D: Dim,
            E: Dim,
            F: Dim,
        > ReduceShapeTo<(D, E, F), Axes3<{ { 0 } }, { { 1 + 0 } }, { { 1 + (1 + 0) } }>>
        for (A, B, C, D, E, F) {}
        impl<
            A: Dim,
            B: Dim,
            C: Dim,
            D: Dim,
            E: Dim,
            F: Dim,
        > ReduceShape<Axes3<{ { 0 } }, { { 1 + 0 } }, { { 1 + (1 + 0) } }>>
        for (A, B, C, D, E, F) {
            type Reduced = (D, E, F);
        }
        impl ReduceShapeTo<
            [usize; { 1 + (1 + (1 + 0)) }],
            Axes3<{ { 0 } }, { { 1 + 0 } }, { { 1 + (1 + 0) } }>,
        > for [usize; { 1 + (1 + (1 + (1 + (1 + (1 + 0))))) }] {}
        impl ReduceShape<Axes3<{ { 0 } }, { { 1 + 0 } }, { { 1 + (1 + 0) } }>>
        for [usize; { 1 + (1 + (1 + (1 + (1 + (1 + 0))))) }] {
            type Reduced = [usize; { 1 + (1 + (1 + 0)) }];
        }
        impl<A: Dim, B: Dim, C: Dim> ReduceShapeTo<(C,), Axes2<{ { 0 } }, { { 1 + 0 } }>>
        for (A, B, C) {}
        impl<A: Dim, B: Dim, C: Dim> ReduceShape<Axes2<{ { 0 } }, { { 1 + 0 } }>>
        for (A, B, C) {
            type Reduced = (C,);
        }
        impl ReduceShapeTo<[usize; { 1 + 0 }], Axes2<{ { 0 } }, { { 1 + 0 } }>>
        for [usize; { 1 + (1 + (1 + 0)) }] {}
        impl ReduceShape<Axes2<{ { 0 } }, { { 1 + 0 } }>>
        for [usize; { 1 + (1 + (1 + 0)) }] {
            type Reduced = [usize; { 1 + 0 }];
        }
        impl<
            A: Dim,
            B: Dim,
            C: Dim,
            D: Dim,
        > ReduceShapeTo<(C,), Axes3<{ { 0 } }, { { 1 + 0 } }, { { 1 + (1 + (1 + 0)) } }>>
        for (A, B, C, D) {}
        impl<
            A: Dim,
            B: Dim,
            C: Dim,
            D: Dim,
        > ReduceShape<Axes3<{ { 0 } }, { { 1 + 0 } }, { { 1 + (1 + (1 + 0)) } }>>
        for (A, B, C, D) {
            type Reduced = (C,);
        }
        impl ReduceShapeTo<
            [usize; { 1 + 0 }],
            Axes3<{ { 0 } }, { { 1 + 0 } }, { { 1 + (1 + (1 + 0)) } }>,
        > for [usize; { 1 + (1 + (1 + (1 + 0))) }] {}
        impl ReduceShape<Axes3<{ { 0 } }, { { 1 + 0 } }, { { 1 + (1 + (1 + 0)) } }>>
        for [usize; { 1 + (1 + (1 + (1 + 0))) }] {
            type Reduced = [usize; { 1 + 0 }];
        }
        impl<
            A: Dim,
            B: Dim,
            C: Dim,
            D: Dim,
            E: Dim,
        > ReduceShapeTo<
            (C,),
            Axes4<
                { { 0 } },
                { { 1 + 0 } },
                { { 1 + (1 + (1 + 0)) } },
                { { 1 + (1 + (1 + (1 + 0))) } },
            >,
        > for (A, B, C, D, E) {}
        impl<
            A: Dim,
            B: Dim,
            C: Dim,
            D: Dim,
            E: Dim,
        > ReduceShape<
            Axes4<
                { { 0 } },
                { { 1 + 0 } },
                { { 1 + (1 + (1 + 0)) } },
                { { 1 + (1 + (1 + (1 + 0))) } },
            >,
        > for (A, B, C, D, E) {
            type Reduced = (C,);
        }
        impl ReduceShapeTo<
            [usize; { 1 + 0 }],
            Axes4<
                { { 0 } },
                { { 1 + 0 } },
                { { 1 + (1 + (1 + 0)) } },
                { { 1 + (1 + (1 + (1 + 0))) } },
            >,
        > for [usize; { 1 + (1 + (1 + (1 + (1 + 0)))) }] {}
        impl ReduceShape<
            Axes4<
                { { 0 } },
                { { 1 + 0 } },
                { { 1 + (1 + (1 + 0)) } },
                { { 1 + (1 + (1 + (1 + 0))) } },
            >,
        > for [usize; { 1 + (1 + (1 + (1 + (1 + 0)))) }] {
            type Reduced = [usize; { 1 + 0 }];
        }
        impl<
            A: Dim,
            B: Dim,
            C: Dim,
            D: Dim,
            E: Dim,
            F: Dim,
        > ReduceShapeTo<
            (C,),
            Axes5<
                { { 0 } },
                { { 1 + 0 } },
                { { 1 + (1 + (1 + 0)) } },
                { { 1 + (1 + (1 + (1 + 0))) } },
                { { 1 + (1 + (1 + (1 + (1 + 0)))) } },
            >,
        > for (A, B, C, D, E, F) {}
        impl<
            A: Dim,
            B: Dim,
            C: Dim,
            D: Dim,
            E: Dim,
            F: Dim,
        > ReduceShape<
            Axes5<
                { { 0 } },
                { { 1 + 0 } },
                { { 1 + (1 + (1 + 0)) } },
                { { 1 + (1 + (1 + (1 + 0))) } },
                { { 1 + (1 + (1 + (1 + (1 + 0)))) } },
            >,
        > for (A, B, C, D, E, F) {
            type Reduced = (C,);
        }
        impl ReduceShapeTo<
            [usize; { 1 + 0 }],
            Axes5<
                { { 0 } },
                { { 1 + 0 } },
                { { 1 + (1 + (1 + 0)) } },
                { { 1 + (1 + (1 + (1 + 0))) } },
                { { 1 + (1 + (1 + (1 + (1 + 0)))) } },
            >,
        > for [usize; { 1 + (1 + (1 + (1 + (1 + (1 + 0))))) }] {}
        impl ReduceShape<
            Axes5<
                { { 0 } },
                { { 1 + 0 } },
                { { 1 + (1 + (1 + 0)) } },
                { { 1 + (1 + (1 + (1 + 0))) } },
                { { 1 + (1 + (1 + (1 + (1 + 0)))) } },
            >,
        > for [usize; { 1 + (1 + (1 + (1 + (1 + (1 + 0))))) }] {
            type Reduced = [usize; { 1 + 0 }];
        }
        impl<
            A: Dim,
            B: Dim,
            C: Dim,
            D: Dim,
            E: Dim,
            F: Dim,
        > ReduceShapeTo<
            (C, F),
            Axes4<
                { { 0 } },
                { { 1 + 0 } },
                { { 1 + (1 + (1 + 0)) } },
                { { 1 + (1 + (1 + (1 + 0))) } },
            >,
        > for (A, B, C, D, E, F) {}
        impl<
            A: Dim,
            B: Dim,
            C: Dim,
            D: Dim,
            E: Dim,
            F: Dim,
        > ReduceShape<
            Axes4<
                { { 0 } },
                { { 1 + 0 } },
                { { 1 + (1 + (1 + 0)) } },
                { { 1 + (1 + (1 + (1 + 0))) } },
            >,
        > for (A, B, C, D, E, F) {
            type Reduced = (C, F);
        }
        impl ReduceShapeTo<
            [usize; { 1 + (1 + 0) }],
            Axes4<
                { { 0 } },
                { { 1 + 0 } },
                { { 1 + (1 + (1 + 0)) } },
                { { 1 + (1 + (1 + (1 + 0))) } },
            >,
        > for [usize; { 1 + (1 + (1 + (1 + (1 + (1 + 0))))) }] {}
        impl ReduceShape<
            Axes4<
                { { 0 } },
                { { 1 + 0 } },
                { { 1 + (1 + (1 + 0)) } },
                { { 1 + (1 + (1 + (1 + 0))) } },
            >,
        > for [usize; { 1 + (1 + (1 + (1 + (1 + (1 + 0))))) }] {
            type Reduced = [usize; { 1 + (1 + 0) }];
        }
        impl<
            A: Dim,
            B: Dim,
            C: Dim,
            D: Dim,
            E: Dim,
        > ReduceShapeTo<
            (C, E),
            Axes3<{ { 0 } }, { { 1 + 0 } }, { { 1 + (1 + (1 + 0)) } }>,
        > for (A, B, C, D, E) {}
        impl<
            A: Dim,
            B: Dim,
            C: Dim,
            D: Dim,
            E: Dim,
        > ReduceShape<Axes3<{ { 0 } }, { { 1 + 0 } }, { { 1 + (1 + (1 + 0)) } }>>
        for (A, B, C, D, E) {
            type Reduced = (C, E);
        }
        impl ReduceShapeTo<
            [usize; { 1 + (1 + 0) }],
            Axes3<{ { 0 } }, { { 1 + 0 } }, { { 1 + (1 + (1 + 0)) } }>,
        > for [usize; { 1 + (1 + (1 + (1 + (1 + 0)))) }] {}
        impl ReduceShape<Axes3<{ { 0 } }, { { 1 + 0 } }, { { 1 + (1 + (1 + 0)) } }>>
        for [usize; { 1 + (1 + (1 + (1 + (1 + 0)))) }] {
            type Reduced = [usize; { 1 + (1 + 0) }];
        }
        impl<
            A: Dim,
            B: Dim,
            C: Dim,
            D: Dim,
            E: Dim,
            F: Dim,
        > ReduceShapeTo<
            (C, E),
            Axes4<
                { { 0 } },
                { { 1 + 0 } },
                { { 1 + (1 + (1 + 0)) } },
                { { 1 + (1 + (1 + (1 + (1 + 0)))) } },
            >,
        > for (A, B, C, D, E, F) {}
        impl<
            A: Dim,
            B: Dim,
            C: Dim,
            D: Dim,
            E: Dim,
            F: Dim,
        > ReduceShape<
            Axes4<
                { { 0 } },
                { { 1 + 0 } },
                { { 1 + (1 + (1 + 0)) } },
                { { 1 + (1 + (1 + (1 + (1 + 0)))) } },
            >,
        > for (A, B, C, D, E, F) {
            type Reduced = (C, E);
        }
        impl ReduceShapeTo<
            [usize; { 1 + (1 + 0) }],
            Axes4<
                { { 0 } },
                { { 1 + 0 } },
                { { 1 + (1 + (1 + 0)) } },
                { { 1 + (1 + (1 + (1 + (1 + 0)))) } },
            >,
        > for [usize; { 1 + (1 + (1 + (1 + (1 + (1 + 0))))) }] {}
        impl ReduceShape<
            Axes4<
                { { 0 } },
                { { 1 + 0 } },
                { { 1 + (1 + (1 + 0)) } },
                { { 1 + (1 + (1 + (1 + (1 + 0)))) } },
            >,
        > for [usize; { 1 + (1 + (1 + (1 + (1 + (1 + 0))))) }] {
            type Reduced = [usize; { 1 + (1 + 0) }];
        }
        impl<
            A: Dim,
            B: Dim,
            C: Dim,
            D: Dim,
            E: Dim,
            F: Dim,
        > ReduceShapeTo<
            (C, E, F),
            Axes3<{ { 0 } }, { { 1 + 0 } }, { { 1 + (1 + (1 + 0)) } }>,
        > for (A, B, C, D, E, F) {}
        impl<
            A: Dim,
            B: Dim,
            C: Dim,
            D: Dim,
            E: Dim,
            F: Dim,
        > ReduceShape<Axes3<{ { 0 } }, { { 1 + 0 } }, { { 1 + (1 + (1 + 0)) } }>>
        for (A, B, C, D, E, F) {
            type Reduced = (C, E, F);
        }
        impl ReduceShapeTo<
            [usize; { 1 + (1 + (1 + 0)) }],
            Axes3<{ { 0 } }, { { 1 + 0 } }, { { 1 + (1 + (1 + 0)) } }>,
        > for [usize; { 1 + (1 + (1 + (1 + (1 + (1 + 0))))) }] {}
        impl ReduceShape<Axes3<{ { 0 } }, { { 1 + 0 } }, { { 1 + (1 + (1 + 0)) } }>>
        for [usize; { 1 + (1 + (1 + (1 + (1 + (1 + 0))))) }] {
            type Reduced = [usize; { 1 + (1 + (1 + 0)) }];
        }
        impl<
            A: Dim,
            B: Dim,
            C: Dim,
            D: Dim,
        > ReduceShapeTo<(C, D), Axes2<{ { 0 } }, { { 1 + 0 } }>> for (A, B, C, D) {}
        impl<A: Dim, B: Dim, C: Dim, D: Dim> ReduceShape<Axes2<{ { 0 } }, { { 1 + 0 } }>>
        for (A, B, C, D) {
            type Reduced = (C, D);
        }
        impl ReduceShapeTo<[usize; { 1 + (1 + 0) }], Axes2<{ { 0 } }, { { 1 + 0 } }>>
        for [usize; { 1 + (1 + (1 + (1 + 0))) }] {}
        impl ReduceShape<Axes2<{ { 0 } }, { { 1 + 0 } }>>
        for [usize; { 1 + (1 + (1 + (1 + 0))) }] {
            type Reduced = [usize; { 1 + (1 + 0) }];
        }
        impl<
            A: Dim,
            B: Dim,
            C: Dim,
            D: Dim,
            E: Dim,
        > ReduceShapeTo<
            (C, D),
            Axes3<{ { 0 } }, { { 1 + 0 } }, { { 1 + (1 + (1 + (1 + 0))) } }>,
        > for (A, B, C, D, E) {}
        impl<
            A: Dim,
            B: Dim,
            C: Dim,
            D: Dim,
            E: Dim,
        > ReduceShape<Axes3<{ { 0 } }, { { 1 + 0 } }, { { 1 + (1 + (1 + (1 + 0))) } }>>
        for (A, B, C, D, E) {
            type Reduced = (C, D);
        }
        impl ReduceShapeTo<
            [usize; { 1 + (1 + 0) }],
            Axes3<{ { 0 } }, { { 1 + 0 } }, { { 1 + (1 + (1 + (1 + 0))) } }>,
        > for [usize; { 1 + (1 + (1 + (1 + (1 + 0)))) }] {}
        impl ReduceShape<
            Axes3<{ { 0 } }, { { 1 + 0 } }, { { 1 + (1 + (1 + (1 + 0))) } }>,
        > for [usize; { 1 + (1 + (1 + (1 + (1 + 0)))) }] {
            type Reduced = [usize; { 1 + (1 + 0) }];
        }
        impl<
            A: Dim,
            B: Dim,
            C: Dim,
            D: Dim,
            E: Dim,
            F: Dim,
        > ReduceShapeTo<
            (C, D),
            Axes4<
                { { 0 } },
                { { 1 + 0 } },
                { { 1 + (1 + (1 + (1 + 0))) } },
                { { 1 + (1 + (1 + (1 + (1 + 0)))) } },
            >,
        > for (A, B, C, D, E, F) {}
        impl<
            A: Dim,
            B: Dim,
            C: Dim,
            D: Dim,
            E: Dim,
            F: Dim,
        > ReduceShape<
            Axes4<
                { { 0 } },
                { { 1 + 0 } },
                { { 1 + (1 + (1 + (1 + 0))) } },
                { { 1 + (1 + (1 + (1 + (1 + 0)))) } },
            >,
        > for (A, B, C, D, E, F) {
            type Reduced = (C, D);
        }
        impl ReduceShapeTo<
            [usize; { 1 + (1 + 0) }],
            Axes4<
                { { 0 } },
                { { 1 + 0 } },
                { { 1 + (1 + (1 + (1 + 0))) } },
                { { 1 + (1 + (1 + (1 + (1 + 0)))) } },
            >,
        > for [usize; { 1 + (1 + (1 + (1 + (1 + (1 + 0))))) }] {}
        impl ReduceShape<
            Axes4<
                { { 0 } },
                { { 1 + 0 } },
                { { 1 + (1 + (1 + (1 + 0))) } },
                { { 1 + (1 + (1 + (1 + (1 + 0)))) } },
            >,
        > for [usize; { 1 + (1 + (1 + (1 + (1 + (1 + 0))))) }] {
            type Reduced = [usize; { 1 + (1 + 0) }];
        }
        impl<
            A: Dim,
            B: Dim,
            C: Dim,
            D: Dim,
            E: Dim,
            F: Dim,
        > ReduceShapeTo<
            (C, D, F),
            Axes3<{ { 0 } }, { { 1 + 0 } }, { { 1 + (1 + (1 + (1 + 0))) } }>,
        > for (A, B, C, D, E, F) {}
        impl<
            A: Dim,
            B: Dim,
            C: Dim,
            D: Dim,
            E: Dim,
            F: Dim,
        > ReduceShape<Axes3<{ { 0 } }, { { 1 + 0 } }, { { 1 + (1 + (1 + (1 + 0))) } }>>
        for (A, B, C, D, E, F) {
            type Reduced = (C, D, F);
        }
        impl ReduceShapeTo<
            [usize; { 1 + (1 + (1 + 0)) }],
            Axes3<{ { 0 } }, { { 1 + 0 } }, { { 1 + (1 + (1 + (1 + 0))) } }>,
        > for [usize; { 1 + (1 + (1 + (1 + (1 + (1 + 0))))) }] {}
        impl ReduceShape<
            Axes3<{ { 0 } }, { { 1 + 0 } }, { { 1 + (1 + (1 + (1 + 0))) } }>,
        > for [usize; { 1 + (1 + (1 + (1 + (1 + (1 + 0))))) }] {
            type Reduced = [usize; { 1 + (1 + (1 + 0)) }];
        }
        impl<
            A: Dim,
            B: Dim,
            C: Dim,
            D: Dim,
            E: Dim,
        > ReduceShapeTo<(C, D, E), Axes2<{ { 0 } }, { { 1 + 0 } }>> for (A, B, C, D, E) {}
        impl<
            A: Dim,
            B: Dim,
            C: Dim,
            D: Dim,
            E: Dim,
        > ReduceShape<Axes2<{ { 0 } }, { { 1 + 0 } }>> for (A, B, C, D, E) {
            type Reduced = (C, D, E);
        }
        impl ReduceShapeTo<
            [usize; { 1 + (1 + (1 + 0)) }],
            Axes2<{ { 0 } }, { { 1 + 0 } }>,
        > for [usize; { 1 + (1 + (1 + (1 + (1 + 0)))) }] {}
        impl ReduceShape<Axes2<{ { 0 } }, { { 1 + 0 } }>>
        for [usize; { 1 + (1 + (1 + (1 + (1 + 0)))) }] {
            type Reduced = [usize; { 1 + (1 + (1 + 0)) }];
        }
        impl<
            A: Dim,
            B: Dim,
            C: Dim,
            D: Dim,
            E: Dim,
            F: Dim,
        > ReduceShapeTo<
            (C, D, E),
            Axes3<{ { 0 } }, { { 1 + 0 } }, { { 1 + (1 + (1 + (1 + (1 + 0)))) } }>,
        > for (A, B, C, D, E, F) {}
        impl<
            A: Dim,
            B: Dim,
            C: Dim,
            D: Dim,
            E: Dim,
            F: Dim,
        > ReduceShape<
            Axes3<{ { 0 } }, { { 1 + 0 } }, { { 1 + (1 + (1 + (1 + (1 + 0)))) } }>,
        > for (A, B, C, D, E, F) {
            type Reduced = (C, D, E);
        }
        impl ReduceShapeTo<
            [usize; { 1 + (1 + (1 + 0)) }],
            Axes3<{ { 0 } }, { { 1 + 0 } }, { { 1 + (1 + (1 + (1 + (1 + 0)))) } }>,
        > for [usize; { 1 + (1 + (1 + (1 + (1 + (1 + 0))))) }] {}
        impl ReduceShape<
            Axes3<{ { 0 } }, { { 1 + 0 } }, { { 1 + (1 + (1 + (1 + (1 + 0)))) } }>,
        > for [usize; { 1 + (1 + (1 + (1 + (1 + (1 + 0))))) }] {
            type Reduced = [usize; { 1 + (1 + (1 + 0)) }];
        }
        impl<
            A: Dim,
            B: Dim,
            C: Dim,
            D: Dim,
            E: Dim,
            F: Dim,
        > ReduceShapeTo<(C, D, E, F), Axes2<{ { 0 } }, { { 1 + 0 } }>>
        for (A, B, C, D, E, F) {}
        impl<
            A: Dim,
            B: Dim,
            C: Dim,
            D: Dim,
            E: Dim,
            F: Dim,
        > ReduceShape<Axes2<{ { 0 } }, { { 1 + 0 } }>> for (A, B, C, D, E, F) {
            type Reduced = (C, D, E, F);
        }
        impl ReduceShapeTo<
            [usize; { 1 + (1 + (1 + (1 + 0))) }],
            Axes2<{ { 0 } }, { { 1 + 0 } }>,
        > for [usize; { 1 + (1 + (1 + (1 + (1 + (1 + 0))))) }] {}
        impl ReduceShape<Axes2<{ { 0 } }, { { 1 + 0 } }>>
        for [usize; { 1 + (1 + (1 + (1 + (1 + (1 + 0))))) }] {
            type Reduced = [usize; { 1 + (1 + (1 + (1 + 0))) }];
        }
        impl<A: Dim, B: Dim> ReduceShapeTo<(B,), Axis<{ { 0 } }>> for (A, B) {}
        impl<A: Dim, B: Dim> ReduceShape<Axis<{ { 0 } }>> for (A, B) {
            type Reduced = (B,);
        }
        impl ReduceShapeTo<[usize; { 1 + 0 }], Axis<{ { 0 } }>>
        for [usize; { 1 + (1 + 0) }] {}
        impl ReduceShape<Axis<{ { 0 } }>> for [usize; { 1 + (1 + 0) }] {
            type Reduced = [usize; { 1 + 0 }];
        }
        impl<
            A: Dim,
            B: Dim,
            C: Dim,
        > ReduceShapeTo<(B,), Axes2<{ { 0 } }, { { 1 + (1 + 0) } }>> for (A, B, C) {}
        impl<A: Dim, B: Dim, C: Dim> ReduceShape<Axes2<{ { 0 } }, { { 1 + (1 + 0) } }>>
        for (A, B, C) {
            type Reduced = (B,);
        }
        impl ReduceShapeTo<[usize; { 1 + 0 }], Axes2<{ { 0 } }, { { 1 + (1 + 0) } }>>
        for [usize; { 1 + (1 + (1 + 0)) }] {}
        impl ReduceShape<Axes2<{ { 0 } }, { { 1 + (1 + 0) } }>>
        for [usize; { 1 + (1 + (1 + 0)) }] {
            type Reduced = [usize; { 1 + 0 }];
        }
        impl<
            A: Dim,
            B: Dim,
            C: Dim,
            D: Dim,
        > ReduceShapeTo<
            (B,),
            Axes3<{ { 0 } }, { { 1 + (1 + 0) } }, { { 1 + (1 + (1 + 0)) } }>,
        > for (A, B, C, D) {}
        impl<
            A: Dim,
            B: Dim,
            C: Dim,
            D: Dim,
        > ReduceShape<Axes3<{ { 0 } }, { { 1 + (1 + 0) } }, { { 1 + (1 + (1 + 0)) } }>>
        for (A, B, C, D) {
            type Reduced = (B,);
        }
        impl ReduceShapeTo<
            [usize; { 1 + 0 }],
            Axes3<{ { 0 } }, { { 1 + (1 + 0) } }, { { 1 + (1 + (1 + 0)) } }>,
        > for [usize; { 1 + (1 + (1 + (1 + 0))) }] {}
        impl ReduceShape<
            Axes3<{ { 0 } }, { { 1 + (1 + 0) } }, { { 1 + (1 + (1 + 0)) } }>,
        > for [usize; { 1 + (1 + (1 + (1 + 0))) }] {
            type Reduced = [usize; { 1 + 0 }];
        }
        impl<
            A: Dim,
            B: Dim,
            C: Dim,
            D: Dim,
            E: Dim,
        > ReduceShapeTo<
            (B,),
            Axes4<
                { { 0 } },
                { { 1 + (1 + 0) } },
                { { 1 + (1 + (1 + 0)) } },
                { { 1 + (1 + (1 + (1 + 0))) } },
            >,
        > for (A, B, C, D, E) {}
        impl<
            A: Dim,
            B: Dim,
            C: Dim,
            D: Dim,
            E: Dim,
        > ReduceShape<
            Axes4<
                { { 0 } },
                { { 1 + (1 + 0) } },
                { { 1 + (1 + (1 + 0)) } },
                { { 1 + (1 + (1 + (1 + 0))) } },
            >,
        > for (A, B, C, D, E) {
            type Reduced = (B,);
        }
        impl ReduceShapeTo<
            [usize; { 1 + 0 }],
            Axes4<
                { { 0 } },
                { { 1 + (1 + 0) } },
                { { 1 + (1 + (1 + 0)) } },
                { { 1 + (1 + (1 + (1 + 0))) } },
            >,
        > for [usize; { 1 + (1 + (1 + (1 + (1 + 0)))) }] {}
        impl ReduceShape<
            Axes4<
                { { 0 } },
                { { 1 + (1 + 0) } },
                { { 1 + (1 + (1 + 0)) } },
                { { 1 + (1 + (1 + (1 + 0))) } },
            >,
        > for [usize; { 1 + (1 + (1 + (1 + (1 + 0)))) }] {
            type Reduced = [usize; { 1 + 0 }];
        }
        impl<
            A: Dim,
            B: Dim,
            C: Dim,
            D: Dim,
            E: Dim,
            F: Dim,
        > ReduceShapeTo<
            (B,),
            Axes5<
                { { 0 } },
                { { 1 + (1 + 0) } },
                { { 1 + (1 + (1 + 0)) } },
                { { 1 + (1 + (1 + (1 + 0))) } },
                { { 1 + (1 + (1 + (1 + (1 + 0)))) } },
            >,
        > for (A, B, C, D, E, F) {}
        impl<
            A: Dim,
            B: Dim,
            C: Dim,
            D: Dim,
            E: Dim,
            F: Dim,
        > ReduceShape<
            Axes5<
                { { 0 } },
                { { 1 + (1 + 0) } },
                { { 1 + (1 + (1 + 0)) } },
                { { 1 + (1 + (1 + (1 + 0))) } },
                { { 1 + (1 + (1 + (1 + (1 + 0)))) } },
            >,
        > for (A, B, C, D, E, F) {
            type Reduced = (B,);
        }
        impl ReduceShapeTo<
            [usize; { 1 + 0 }],
            Axes5<
                { { 0 } },
                { { 1 + (1 + 0) } },
                { { 1 + (1 + (1 + 0)) } },
                { { 1 + (1 + (1 + (1 + 0))) } },
                { { 1 + (1 + (1 + (1 + (1 + 0)))) } },
            >,
        > for [usize; { 1 + (1 + (1 + (1 + (1 + (1 + 0))))) }] {}
        impl ReduceShape<
            Axes5<
                { { 0 } },
                { { 1 + (1 + 0) } },
                { { 1 + (1 + (1 + 0)) } },
                { { 1 + (1 + (1 + (1 + 0))) } },
                { { 1 + (1 + (1 + (1 + (1 + 0)))) } },
            >,
        > for [usize; { 1 + (1 + (1 + (1 + (1 + (1 + 0))))) }] {
            type Reduced = [usize; { 1 + 0 }];
        }
        impl<
            A: Dim,
            B: Dim,
            C: Dim,
            D: Dim,
            E: Dim,
            F: Dim,
        > ReduceShapeTo<
            (B, F),
            Axes4<
                { { 0 } },
                { { 1 + (1 + 0) } },
                { { 1 + (1 + (1 + 0)) } },
                { { 1 + (1 + (1 + (1 + 0))) } },
            >,
        > for (A, B, C, D, E, F) {}
        impl<
            A: Dim,
            B: Dim,
            C: Dim,
            D: Dim,
            E: Dim,
            F: Dim,
        > ReduceShape<
            Axes4<
                { { 0 } },
                { { 1 + (1 + 0) } },
                { { 1 + (1 + (1 + 0)) } },
                { { 1 + (1 + (1 + (1 + 0))) } },
            >,
        > for (A, B, C, D, E, F) {
            type Reduced = (B, F);
        }
        impl ReduceShapeTo<
            [usize; { 1 + (1 + 0) }],
            Axes4<
                { { 0 } },
                { { 1 + (1 + 0) } },
                { { 1 + (1 + (1 + 0)) } },
                { { 1 + (1 + (1 + (1 + 0))) } },
            >,
        > for [usize; { 1 + (1 + (1 + (1 + (1 + (1 + 0))))) }] {}
        impl ReduceShape<
            Axes4<
                { { 0 } },
                { { 1 + (1 + 0) } },
                { { 1 + (1 + (1 + 0)) } },
                { { 1 + (1 + (1 + (1 + 0))) } },
            >,
        > for [usize; { 1 + (1 + (1 + (1 + (1 + (1 + 0))))) }] {
            type Reduced = [usize; { 1 + (1 + 0) }];
        }
        impl<
            A: Dim,
            B: Dim,
            C: Dim,
            D: Dim,
            E: Dim,
        > ReduceShapeTo<
            (B, E),
            Axes3<{ { 0 } }, { { 1 + (1 + 0) } }, { { 1 + (1 + (1 + 0)) } }>,
        > for (A, B, C, D, E) {}
        impl<
            A: Dim,
            B: Dim,
            C: Dim,
            D: Dim,
            E: Dim,
        > ReduceShape<Axes3<{ { 0 } }, { { 1 + (1 + 0) } }, { { 1 + (1 + (1 + 0)) } }>>
        for (A, B, C, D, E) {
            type Reduced = (B, E);
        }
        impl ReduceShapeTo<
            [usize; { 1 + (1 + 0) }],
            Axes3<{ { 0 } }, { { 1 + (1 + 0) } }, { { 1 + (1 + (1 + 0)) } }>,
        > for [usize; { 1 + (1 + (1 + (1 + (1 + 0)))) }] {}
        impl ReduceShape<
            Axes3<{ { 0 } }, { { 1 + (1 + 0) } }, { { 1 + (1 + (1 + 0)) } }>,
        > for [usize; { 1 + (1 + (1 + (1 + (1 + 0)))) }] {
            type Reduced = [usize; { 1 + (1 + 0) }];
        }
        impl<
            A: Dim,
            B: Dim,
            C: Dim,
            D: Dim,
            E: Dim,
            F: Dim,
        > ReduceShapeTo<
            (B, E),
            Axes4<
                { { 0 } },
                { { 1 + (1 + 0) } },
                { { 1 + (1 + (1 + 0)) } },
                { { 1 + (1 + (1 + (1 + (1 + 0)))) } },
            >,
        > for (A, B, C, D, E, F) {}
        impl<
            A: Dim,
            B: Dim,
            C: Dim,
            D: Dim,
            E: Dim,
            F: Dim,
        > ReduceShape<
            Axes4<
                { { 0 } },
                { { 1 + (1 + 0) } },
                { { 1 + (1 + (1 + 0)) } },
                { { 1 + (1 + (1 + (1 + (1 + 0)))) } },
            >,
        > for (A, B, C, D, E, F) {
            type Reduced = (B, E);
        }
        impl ReduceShapeTo<
            [usize; { 1 + (1 + 0) }],
            Axes4<
                { { 0 } },
                { { 1 + (1 + 0) } },
                { { 1 + (1 + (1 + 0)) } },
                { { 1 + (1 + (1 + (1 + (1 + 0)))) } },
            >,
        > for [usize; { 1 + (1 + (1 + (1 + (1 + (1 + 0))))) }] {}
        impl ReduceShape<
            Axes4<
                { { 0 } },
                { { 1 + (1 + 0) } },
                { { 1 + (1 + (1 + 0)) } },
                { { 1 + (1 + (1 + (1 + (1 + 0)))) } },
            >,
        > for [usize; { 1 + (1 + (1 + (1 + (1 + (1 + 0))))) }] {
            type Reduced = [usize; { 1 + (1 + 0) }];
        }
        impl<
            A: Dim,
            B: Dim,
            C: Dim,
            D: Dim,
            E: Dim,
            F: Dim,
        > ReduceShapeTo<
            (B, E, F),
            Axes3<{ { 0 } }, { { 1 + (1 + 0) } }, { { 1 + (1 + (1 + 0)) } }>,
        > for (A, B, C, D, E, F) {}
        impl<
            A: Dim,
            B: Dim,
            C: Dim,
            D: Dim,
            E: Dim,
            F: Dim,
        > ReduceShape<Axes3<{ { 0 } }, { { 1 + (1 + 0) } }, { { 1 + (1 + (1 + 0)) } }>>
        for (A, B, C, D, E, F) {
            type Reduced = (B, E, F);
        }
        impl ReduceShapeTo<
            [usize; { 1 + (1 + (1 + 0)) }],
            Axes3<{ { 0 } }, { { 1 + (1 + 0) } }, { { 1 + (1 + (1 + 0)) } }>,
        > for [usize; { 1 + (1 + (1 + (1 + (1 + (1 + 0))))) }] {}
        impl ReduceShape<
            Axes3<{ { 0 } }, { { 1 + (1 + 0) } }, { { 1 + (1 + (1 + 0)) } }>,
        > for [usize; { 1 + (1 + (1 + (1 + (1 + (1 + 0))))) }] {
            type Reduced = [usize; { 1 + (1 + (1 + 0)) }];
        }
        impl<
            A: Dim,
            B: Dim,
            C: Dim,
            D: Dim,
        > ReduceShapeTo<(B, D), Axes2<{ { 0 } }, { { 1 + (1 + 0) } }>> for (A, B, C, D) {}
        impl<
            A: Dim,
            B: Dim,
            C: Dim,
            D: Dim,
        > ReduceShape<Axes2<{ { 0 } }, { { 1 + (1 + 0) } }>> for (A, B, C, D) {
            type Reduced = (B, D);
        }
        impl ReduceShapeTo<
            [usize; { 1 + (1 + 0) }],
            Axes2<{ { 0 } }, { { 1 + (1 + 0) } }>,
        > for [usize; { 1 + (1 + (1 + (1 + 0))) }] {}
        impl ReduceShape<Axes2<{ { 0 } }, { { 1 + (1 + 0) } }>>
        for [usize; { 1 + (1 + (1 + (1 + 0))) }] {
            type Reduced = [usize; { 1 + (1 + 0) }];
        }
        impl<
            A: Dim,
            B: Dim,
            C: Dim,
            D: Dim,
            E: Dim,
        > ReduceShapeTo<
            (B, D),
            Axes3<{ { 0 } }, { { 1 + (1 + 0) } }, { { 1 + (1 + (1 + (1 + 0))) } }>,
        > for (A, B, C, D, E) {}
        impl<
            A: Dim,
            B: Dim,
            C: Dim,
            D: Dim,
            E: Dim,
        > ReduceShape<
            Axes3<{ { 0 } }, { { 1 + (1 + 0) } }, { { 1 + (1 + (1 + (1 + 0))) } }>,
        > for (A, B, C, D, E) {
            type Reduced = (B, D);
        }
        impl ReduceShapeTo<
            [usize; { 1 + (1 + 0) }],
            Axes3<{ { 0 } }, { { 1 + (1 + 0) } }, { { 1 + (1 + (1 + (1 + 0))) } }>,
        > for [usize; { 1 + (1 + (1 + (1 + (1 + 0)))) }] {}
        impl ReduceShape<
            Axes3<{ { 0 } }, { { 1 + (1 + 0) } }, { { 1 + (1 + (1 + (1 + 0))) } }>,
        > for [usize; { 1 + (1 + (1 + (1 + (1 + 0)))) }] {
            type Reduced = [usize; { 1 + (1 + 0) }];
        }
        impl<
            A: Dim,
            B: Dim,
            C: Dim,
            D: Dim,
            E: Dim,
            F: Dim,
        > ReduceShapeTo<
            (B, D),
            Axes4<
                { { 0 } },
                { { 1 + (1 + 0) } },
                { { 1 + (1 + (1 + (1 + 0))) } },
                { { 1 + (1 + (1 + (1 + (1 + 0)))) } },
            >,
        > for (A, B, C, D, E, F) {}
        impl<
            A: Dim,
            B: Dim,
            C: Dim,
            D: Dim,
            E: Dim,
            F: Dim,
        > ReduceShape<
            Axes4<
                { { 0 } },
                { { 1 + (1 + 0) } },
                { { 1 + (1 + (1 + (1 + 0))) } },
                { { 1 + (1 + (1 + (1 + (1 + 0)))) } },
            >,
        > for (A, B, C, D, E, F) {
            type Reduced = (B, D);
        }
        impl ReduceShapeTo<
            [usize; { 1 + (1 + 0) }],
            Axes4<
                { { 0 } },
                { { 1 + (1 + 0) } },
                { { 1 + (1 + (1 + (1 + 0))) } },
                { { 1 + (1 + (1 + (1 + (1 + 0)))) } },
            >,
        > for [usize; { 1 + (1 + (1 + (1 + (1 + (1 + 0))))) }] {}
        impl ReduceShape<
            Axes4<
                { { 0 } },
                { { 1 + (1 + 0) } },
                { { 1 + (1 + (1 + (1 + 0))) } },
                { { 1 + (1 + (1 + (1 + (1 + 0)))) } },
            >,
        > for [usize; { 1 + (1 + (1 + (1 + (1 + (1 + 0))))) }] {
            type Reduced = [usize; { 1 + (1 + 0) }];
        }
        impl<
            A: Dim,
            B: Dim,
            C: Dim,
            D: Dim,
            E: Dim,
            F: Dim,
        > ReduceShapeTo<
            (B, D, F),
            Axes3<{ { 0 } }, { { 1 + (1 + 0) } }, { { 1 + (1 + (1 + (1 + 0))) } }>,
        > for (A, B, C, D, E, F) {}
        impl<
            A: Dim,
            B: Dim,
            C: Dim,
            D: Dim,
            E: Dim,
            F: Dim,
        > ReduceShape<
            Axes3<{ { 0 } }, { { 1 + (1 + 0) } }, { { 1 + (1 + (1 + (1 + 0))) } }>,
        > for (A, B, C, D, E, F) {
            type Reduced = (B, D, F);
        }
        impl ReduceShapeTo<
            [usize; { 1 + (1 + (1 + 0)) }],
            Axes3<{ { 0 } }, { { 1 + (1 + 0) } }, { { 1 + (1 + (1 + (1 + 0))) } }>,
        > for [usize; { 1 + (1 + (1 + (1 + (1 + (1 + 0))))) }] {}
        impl ReduceShape<
            Axes3<{ { 0 } }, { { 1 + (1 + 0) } }, { { 1 + (1 + (1 + (1 + 0))) } }>,
        > for [usize; { 1 + (1 + (1 + (1 + (1 + (1 + 0))))) }] {
            type Reduced = [usize; { 1 + (1 + (1 + 0)) }];
        }
        impl<
            A: Dim,
            B: Dim,
            C: Dim,
            D: Dim,
            E: Dim,
        > ReduceShapeTo<(B, D, E), Axes2<{ { 0 } }, { { 1 + (1 + 0) } }>>
        for (A, B, C, D, E) {}
        impl<
            A: Dim,
            B: Dim,
            C: Dim,
            D: Dim,
            E: Dim,
        > ReduceShape<Axes2<{ { 0 } }, { { 1 + (1 + 0) } }>> for (A, B, C, D, E) {
            type Reduced = (B, D, E);
        }
        impl ReduceShapeTo<
            [usize; { 1 + (1 + (1 + 0)) }],
            Axes2<{ { 0 } }, { { 1 + (1 + 0) } }>,
        > for [usize; { 1 + (1 + (1 + (1 + (1 + 0)))) }] {}
        impl ReduceShape<Axes2<{ { 0 } }, { { 1 + (1 + 0) } }>>
        for [usize; { 1 + (1 + (1 + (1 + (1 + 0)))) }] {
            type Reduced = [usize; { 1 + (1 + (1 + 0)) }];
        }
        impl<
            A: Dim,
            B: Dim,
            C: Dim,
            D: Dim,
            E: Dim,
            F: Dim,
        > ReduceShapeTo<
            (B, D, E),
            Axes3<{ { 0 } }, { { 1 + (1 + 0) } }, { { 1 + (1 + (1 + (1 + (1 + 0)))) } }>,
        > for (A, B, C, D, E, F) {}
        impl<
            A: Dim,
            B: Dim,
            C: Dim,
            D: Dim,
            E: Dim,
            F: Dim,
        > ReduceShape<
            Axes3<{ { 0 } }, { { 1 + (1 + 0) } }, { { 1 + (1 + (1 + (1 + (1 + 0)))) } }>,
        > for (A, B, C, D, E, F) {
            type Reduced = (B, D, E);
        }
        impl ReduceShapeTo<
            [usize; { 1 + (1 + (1 + 0)) }],
            Axes3<{ { 0 } }, { { 1 + (1 + 0) } }, { { 1 + (1 + (1 + (1 + (1 + 0)))) } }>,
        > for [usize; { 1 + (1 + (1 + (1 + (1 + (1 + 0))))) }] {}
        impl ReduceShape<
            Axes3<{ { 0 } }, { { 1 + (1 + 0) } }, { { 1 + (1 + (1 + (1 + (1 + 0)))) } }>,
        > for [usize; { 1 + (1 + (1 + (1 + (1 + (1 + 0))))) }] {
            type Reduced = [usize; { 1 + (1 + (1 + 0)) }];
        }
        impl<
            A: Dim,
            B: Dim,
            C: Dim,
            D: Dim,
            E: Dim,
            F: Dim,
        > ReduceShapeTo<(B, D, E, F), Axes2<{ { 0 } }, { { 1 + (1 + 0) } }>>
        for (A, B, C, D, E, F) {}
        impl<
            A: Dim,
            B: Dim,
            C: Dim,
            D: Dim,
            E: Dim,
            F: Dim,
        > ReduceShape<Axes2<{ { 0 } }, { { 1 + (1 + 0) } }>> for (A, B, C, D, E, F) {
            type Reduced = (B, D, E, F);
        }
        impl ReduceShapeTo<
            [usize; { 1 + (1 + (1 + (1 + 0))) }],
            Axes2<{ { 0 } }, { { 1 + (1 + 0) } }>,
        > for [usize; { 1 + (1 + (1 + (1 + (1 + (1 + 0))))) }] {}
        impl ReduceShape<Axes2<{ { 0 } }, { { 1 + (1 + 0) } }>>
        for [usize; { 1 + (1 + (1 + (1 + (1 + (1 + 0))))) }] {
            type Reduced = [usize; { 1 + (1 + (1 + (1 + 0))) }];
        }
        impl<A: Dim, B: Dim, C: Dim> ReduceShapeTo<(B, C), Axis<{ { 0 } }>>
        for (A, B, C) {}
        impl<A: Dim, B: Dim, C: Dim> ReduceShape<Axis<{ { 0 } }>> for (A, B, C) {
            type Reduced = (B, C);
        }
        impl ReduceShapeTo<[usize; { 1 + (1 + 0) }], Axis<{ { 0 } }>>
        for [usize; { 1 + (1 + (1 + 0)) }] {}
        impl ReduceShape<Axis<{ { 0 } }>> for [usize; { 1 + (1 + (1 + 0)) }] {
            type Reduced = [usize; { 1 + (1 + 0) }];
        }
        impl<
            A: Dim,
            B: Dim,
            C: Dim,
            D: Dim,
        > ReduceShapeTo<(B, C), Axes2<{ { 0 } }, { { 1 + (1 + (1 + 0)) } }>>
        for (A, B, C, D) {}
        impl<
            A: Dim,
            B: Dim,
            C: Dim,
            D: Dim,
        > ReduceShape<Axes2<{ { 0 } }, { { 1 + (1 + (1 + 0)) } }>> for (A, B, C, D) {
            type Reduced = (B, C);
        }
        impl ReduceShapeTo<
            [usize; { 1 + (1 + 0) }],
            Axes2<{ { 0 } }, { { 1 + (1 + (1 + 0)) } }>,
        > for [usize; { 1 + (1 + (1 + (1 + 0))) }] {}
        impl ReduceShape<Axes2<{ { 0 } }, { { 1 + (1 + (1 + 0)) } }>>
        for [usize; { 1 + (1 + (1 + (1 + 0))) }] {
            type Reduced = [usize; { 1 + (1 + 0) }];
        }
        impl<
            A: Dim,
            B: Dim,
            C: Dim,
            D: Dim,
            E: Dim,
        > ReduceShapeTo<
            (B, C),
            Axes3<{ { 0 } }, { { 1 + (1 + (1 + 0)) } }, { { 1 + (1 + (1 + (1 + 0))) } }>,
        > for (A, B, C, D, E) {}
        impl<
            A: Dim,
            B: Dim,
            C: Dim,
            D: Dim,
            E: Dim,
        > ReduceShape<
            Axes3<{ { 0 } }, { { 1 + (1 + (1 + 0)) } }, { { 1 + (1 + (1 + (1 + 0))) } }>,
        > for (A, B, C, D, E) {
            type Reduced = (B, C);
        }
        impl ReduceShapeTo<
            [usize; { 1 + (1 + 0) }],
            Axes3<{ { 0 } }, { { 1 + (1 + (1 + 0)) } }, { { 1 + (1 + (1 + (1 + 0))) } }>,
        > for [usize; { 1 + (1 + (1 + (1 + (1 + 0)))) }] {}
        impl ReduceShape<
            Axes3<{ { 0 } }, { { 1 + (1 + (1 + 0)) } }, { { 1 + (1 + (1 + (1 + 0))) } }>,
        > for [usize; { 1 + (1 + (1 + (1 + (1 + 0)))) }] {
            type Reduced = [usize; { 1 + (1 + 0) }];
        }
        impl<
            A: Dim,
            B: Dim,
            C: Dim,
            D: Dim,
            E: Dim,
            F: Dim,
        > ReduceShapeTo<
            (B, C),
            Axes4<
                { { 0 } },
                { { 1 + (1 + (1 + 0)) } },
                { { 1 + (1 + (1 + (1 + 0))) } },
                { { 1 + (1 + (1 + (1 + (1 + 0)))) } },
            >,
        > for (A, B, C, D, E, F) {}
        impl<
            A: Dim,
            B: Dim,
            C: Dim,
            D: Dim,
            E: Dim,
            F: Dim,
        > ReduceShape<
            Axes4<
                { { 0 } },
                { { 1 + (1 + (1 + 0)) } },
                { { 1 + (1 + (1 + (1 + 0))) } },
                { { 1 + (1 + (1 + (1 + (1 + 0)))) } },
            >,
        > for (A, B, C, D, E, F) {
            type Reduced = (B, C);
        }
        impl ReduceShapeTo<
            [usize; { 1 + (1 + 0) }],
            Axes4<
                { { 0 } },
                { { 1 + (1 + (1 + 0)) } },
                { { 1 + (1 + (1 + (1 + 0))) } },
                { { 1 + (1 + (1 + (1 + (1 + 0)))) } },
            >,
        > for [usize; { 1 + (1 + (1 + (1 + (1 + (1 + 0))))) }] {}
        impl ReduceShape<
            Axes4<
                { { 0 } },
                { { 1 + (1 + (1 + 0)) } },
                { { 1 + (1 + (1 + (1 + 0))) } },
                { { 1 + (1 + (1 + (1 + (1 + 0)))) } },
            >,
        > for [usize; { 1 + (1 + (1 + (1 + (1 + (1 + 0))))) }] {
            type Reduced = [usize; { 1 + (1 + 0) }];
        }
        impl<
            A: Dim,
            B: Dim,
            C: Dim,
            D: Dim,
            E: Dim,
            F: Dim,
        > ReduceShapeTo<
            (B, C, F),
            Axes3<{ { 0 } }, { { 1 + (1 + (1 + 0)) } }, { { 1 + (1 + (1 + (1 + 0))) } }>,
        > for (A, B, C, D, E, F) {}
        impl<
            A: Dim,
            B: Dim,
            C: Dim,
            D: Dim,
            E: Dim,
            F: Dim,
        > ReduceShape<
            Axes3<{ { 0 } }, { { 1 + (1 + (1 + 0)) } }, { { 1 + (1 + (1 + (1 + 0))) } }>,
        > for (A, B, C, D, E, F) {
            type Reduced = (B, C, F);
        }
        impl ReduceShapeTo<
            [usize; { 1 + (1 + (1 + 0)) }],
            Axes3<{ { 0 } }, { { 1 + (1 + (1 + 0)) } }, { { 1 + (1 + (1 + (1 + 0))) } }>,
        > for [usize; { 1 + (1 + (1 + (1 + (1 + (1 + 0))))) }] {}
        impl ReduceShape<
            Axes3<{ { 0 } }, { { 1 + (1 + (1 + 0)) } }, { { 1 + (1 + (1 + (1 + 0))) } }>,
        > for [usize; { 1 + (1 + (1 + (1 + (1 + (1 + 0))))) }] {
            type Reduced = [usize; { 1 + (1 + (1 + 0)) }];
        }
        impl<
            A: Dim,
            B: Dim,
            C: Dim,
            D: Dim,
            E: Dim,
        > ReduceShapeTo<(B, C, E), Axes2<{ { 0 } }, { { 1 + (1 + (1 + 0)) } }>>
        for (A, B, C, D, E) {}
        impl<
            A: Dim,
            B: Dim,
            C: Dim,
            D: Dim,
            E: Dim,
        > ReduceShape<Axes2<{ { 0 } }, { { 1 + (1 + (1 + 0)) } }>> for (A, B, C, D, E) {
            type Reduced = (B, C, E);
        }
        impl ReduceShapeTo<
            [usize; { 1 + (1 + (1 + 0)) }],
            Axes2<{ { 0 } }, { { 1 + (1 + (1 + 0)) } }>,
        > for [usize; { 1 + (1 + (1 + (1 + (1 + 0)))) }] {}
        impl ReduceShape<Axes2<{ { 0 } }, { { 1 + (1 + (1 + 0)) } }>>
        for [usize; { 1 + (1 + (1 + (1 + (1 + 0)))) }] {
            type Reduced = [usize; { 1 + (1 + (1 + 0)) }];
        }
        impl<
            A: Dim,
            B: Dim,
            C: Dim,
            D: Dim,
            E: Dim,
            F: Dim,
        > ReduceShapeTo<
            (B, C, E),
            Axes3<
                { { 0 } },
                { { 1 + (1 + (1 + 0)) } },
                { { 1 + (1 + (1 + (1 + (1 + 0)))) } },
            >,
        > for (A, B, C, D, E, F) {}
        impl<
            A: Dim,
            B: Dim,
            C: Dim,
            D: Dim,
            E: Dim,
            F: Dim,
        > ReduceShape<
            Axes3<
                { { 0 } },
                { { 1 + (1 + (1 + 0)) } },
                { { 1 + (1 + (1 + (1 + (1 + 0)))) } },
            >,
        > for (A, B, C, D, E, F) {
            type Reduced = (B, C, E);
        }
        impl ReduceShapeTo<
            [usize; { 1 + (1 + (1 + 0)) }],
            Axes3<
                { { 0 } },
                { { 1 + (1 + (1 + 0)) } },
                { { 1 + (1 + (1 + (1 + (1 + 0)))) } },
            >,
        > for [usize; { 1 + (1 + (1 + (1 + (1 + (1 + 0))))) }] {}
        impl ReduceShape<
            Axes3<
                { { 0 } },
                { { 1 + (1 + (1 + 0)) } },
                { { 1 + (1 + (1 + (1 + (1 + 0)))) } },
            >,
        > for [usize; { 1 + (1 + (1 + (1 + (1 + (1 + 0))))) }] {
            type Reduced = [usize; { 1 + (1 + (1 + 0)) }];
        }
        impl<
            A: Dim,
            B: Dim,
            C: Dim,
            D: Dim,
            E: Dim,
            F: Dim,
        > ReduceShapeTo<(B, C, E, F), Axes2<{ { 0 } }, { { 1 + (1 + (1 + 0)) } }>>
        for (A, B, C, D, E, F) {}
        impl<
            A: Dim,
            B: Dim,
            C: Dim,
            D: Dim,
            E: Dim,
            F: Dim,
        > ReduceShape<Axes2<{ { 0 } }, { { 1 + (1 + (1 + 0)) } }>>
        for (A, B, C, D, E, F) {
            type Reduced = (B, C, E, F);
        }
        impl ReduceShapeTo<
            [usize; { 1 + (1 + (1 + (1 + 0))) }],
            Axes2<{ { 0 } }, { { 1 + (1 + (1 + 0)) } }>,
        > for [usize; { 1 + (1 + (1 + (1 + (1 + (1 + 0))))) }] {}
        impl ReduceShape<Axes2<{ { 0 } }, { { 1 + (1 + (1 + 0)) } }>>
        for [usize; { 1 + (1 + (1 + (1 + (1 + (1 + 0))))) }] {
            type Reduced = [usize; { 1 + (1 + (1 + (1 + 0))) }];
        }
        impl<A: Dim, B: Dim, C: Dim, D: Dim> ReduceShapeTo<(B, C, D), Axis<{ { 0 } }>>
        for (A, B, C, D) {}
        impl<A: Dim, B: Dim, C: Dim, D: Dim> ReduceShape<Axis<{ { 0 } }>>
        for (A, B, C, D) {
            type Reduced = (B, C, D);
        }
        impl ReduceShapeTo<[usize; { 1 + (1 + (1 + 0)) }], Axis<{ { 0 } }>>
        for [usize; { 1 + (1 + (1 + (1 + 0))) }] {}
        impl ReduceShape<Axis<{ { 0 } }>> for [usize; { 1 + (1 + (1 + (1 + 0))) }] {
            type Reduced = [usize; { 1 + (1 + (1 + 0)) }];
        }
        impl<
            A: Dim,
            B: Dim,
            C: Dim,
            D: Dim,
            E: Dim,
        > ReduceShapeTo<(B, C, D), Axes2<{ { 0 } }, { { 1 + (1 + (1 + (1 + 0))) } }>>
        for (A, B, C, D, E) {}
        impl<
            A: Dim,
            B: Dim,
            C: Dim,
            D: Dim,
            E: Dim,
        > ReduceShape<Axes2<{ { 0 } }, { { 1 + (1 + (1 + (1 + 0))) } }>>
        for (A, B, C, D, E) {
            type Reduced = (B, C, D);
        }
        impl ReduceShapeTo<
            [usize; { 1 + (1 + (1 + 0)) }],
            Axes2<{ { 0 } }, { { 1 + (1 + (1 + (1 + 0))) } }>,
        > for [usize; { 1 + (1 + (1 + (1 + (1 + 0)))) }] {}
        impl ReduceShape<Axes2<{ { 0 } }, { { 1 + (1 + (1 + (1 + 0))) } }>>
        for [usize; { 1 + (1 + (1 + (1 + (1 + 0)))) }] {
            type Reduced = [usize; { 1 + (1 + (1 + 0)) }];
        }
        impl<
            A: Dim,
            B: Dim,
            C: Dim,
            D: Dim,
            E: Dim,
            F: Dim,
        > ReduceShapeTo<
            (B, C, D),
            Axes3<
                { { 0 } },
                { { 1 + (1 + (1 + (1 + 0))) } },
                { { 1 + (1 + (1 + (1 + (1 + 0)))) } },
            >,
        > for (A, B, C, D, E, F) {}
        impl<
            A: Dim,
            B: Dim,
            C: Dim,
            D: Dim,
            E: Dim,
            F: Dim,
        > ReduceShape<
            Axes3<
                { { 0 } },
                { { 1 + (1 + (1 + (1 + 0))) } },
                { { 1 + (1 + (1 + (1 + (1 + 0)))) } },
            >,
        > for (A, B, C, D, E, F) {
            type Reduced = (B, C, D);
        }
        impl ReduceShapeTo<
            [usize; { 1 + (1 + (1 + 0)) }],
            Axes3<
                { { 0 } },
                { { 1 + (1 + (1 + (1 + 0))) } },
                { { 1 + (1 + (1 + (1 + (1 + 0)))) } },
            >,
        > for [usize; { 1 + (1 + (1 + (1 + (1 + (1 + 0))))) }] {}
        impl ReduceShape<
            Axes3<
                { { 0 } },
                { { 1 + (1 + (1 + (1 + 0))) } },
                { { 1 + (1 + (1 + (1 + (1 + 0)))) } },
            >,
        > for [usize; { 1 + (1 + (1 + (1 + (1 + (1 + 0))))) }] {
            type Reduced = [usize; { 1 + (1 + (1 + 0)) }];
        }
        impl<
            A: Dim,
            B: Dim,
            C: Dim,
            D: Dim,
            E: Dim,
            F: Dim,
        > ReduceShapeTo<(B, C, D, F), Axes2<{ { 0 } }, { { 1 + (1 + (1 + (1 + 0))) } }>>
        for (A, B, C, D, E, F) {}
        impl<
            A: Dim,
            B: Dim,
            C: Dim,
            D: Dim,
            E: Dim,
            F: Dim,
        > ReduceShape<Axes2<{ { 0 } }, { { 1 + (1 + (1 + (1 + 0))) } }>>
        for (A, B, C, D, E, F) {
            type Reduced = (B, C, D, F);
        }
        impl ReduceShapeTo<
            [usize; { 1 + (1 + (1 + (1 + 0))) }],
            Axes2<{ { 0 } }, { { 1 + (1 + (1 + (1 + 0))) } }>,
        > for [usize; { 1 + (1 + (1 + (1 + (1 + (1 + 0))))) }] {}
        impl ReduceShape<Axes2<{ { 0 } }, { { 1 + (1 + (1 + (1 + 0))) } }>>
        for [usize; { 1 + (1 + (1 + (1 + (1 + (1 + 0))))) }] {
            type Reduced = [usize; { 1 + (1 + (1 + (1 + 0))) }];
        }
        impl<
            A: Dim,
            B: Dim,
            C: Dim,
            D: Dim,
            E: Dim,
        > ReduceShapeTo<(B, C, D, E), Axis<{ { 0 } }>> for (A, B, C, D, E) {}
        impl<A: Dim, B: Dim, C: Dim, D: Dim, E: Dim> ReduceShape<Axis<{ { 0 } }>>
        for (A, B, C, D, E) {
            type Reduced = (B, C, D, E);
        }
        impl ReduceShapeTo<[usize; { 1 + (1 + (1 + (1 + 0))) }], Axis<{ { 0 } }>>
        for [usize; { 1 + (1 + (1 + (1 + (1 + 0)))) }] {}
        impl ReduceShape<Axis<{ { 0 } }>>
        for [usize; { 1 + (1 + (1 + (1 + (1 + 0)))) }] {
            type Reduced = [usize; { 1 + (1 + (1 + (1 + 0))) }];
        }
        impl<
            A: Dim,
            B: Dim,
            C: Dim,
            D: Dim,
            E: Dim,
            F: Dim,
        > ReduceShapeTo<
            (B, C, D, E),
            Axes2<{ { 0 } }, { { 1 + (1 + (1 + (1 + (1 + 0)))) } }>,
        > for (A, B, C, D, E, F) {}
        impl<
            A: Dim,
            B: Dim,
            C: Dim,
            D: Dim,
            E: Dim,
            F: Dim,
        > ReduceShape<Axes2<{ { 0 } }, { { 1 + (1 + (1 + (1 + (1 + 0)))) } }>>
        for (A, B, C, D, E, F) {
            type Reduced = (B, C, D, E);
        }
        impl ReduceShapeTo<
            [usize; { 1 + (1 + (1 + (1 + 0))) }],
            Axes2<{ { 0 } }, { { 1 + (1 + (1 + (1 + (1 + 0)))) } }>,
        > for [usize; { 1 + (1 + (1 + (1 + (1 + (1 + 0))))) }] {}
        impl ReduceShape<Axes2<{ { 0 } }, { { 1 + (1 + (1 + (1 + (1 + 0)))) } }>>
        for [usize; { 1 + (1 + (1 + (1 + (1 + (1 + 0))))) }] {
            type Reduced = [usize; { 1 + (1 + (1 + (1 + 0))) }];
        }
        impl<
            A: Dim,
            B: Dim,
            C: Dim,
            D: Dim,
            E: Dim,
            F: Dim,
        > ReduceShapeTo<(B, C, D, E, F), Axis<{ { 0 } }>> for (A, B, C, D, E, F) {}
        impl<A: Dim, B: Dim, C: Dim, D: Dim, E: Dim, F: Dim> ReduceShape<Axis<{ { 0 } }>>
        for (A, B, C, D, E, F) {
            type Reduced = (B, C, D, E, F);
        }
        impl ReduceShapeTo<[usize; { 1 + (1 + (1 + (1 + (1 + 0)))) }], Axis<{ { 0 } }>>
        for [usize; { 1 + (1 + (1 + (1 + (1 + (1 + 0))))) }] {}
        impl ReduceShape<Axis<{ { 0 } }>>
        for [usize; { 1 + (1 + (1 + (1 + (1 + (1 + 0))))) }] {
            type Reduced = [usize; { 1 + (1 + (1 + (1 + (1 + 0)))) }];
        }
        impl<A: Dim, B: Dim> ReduceShapeTo<(A,), Axis<{ { 1 + 0 } }>> for (A, B) {}
        impl<A: Dim, B: Dim> ReduceShape<Axis<{ { 1 + 0 } }>> for (A, B) {
            type Reduced = (A,);
        }
        impl ReduceShapeTo<[usize; { 1 + 0 }], Axis<{ { 1 + 0 } }>>
        for [usize; { 1 + (1 + 0) }] {}
        impl ReduceShape<Axis<{ { 1 + 0 } }>> for [usize; { 1 + (1 + 0) }] {
            type Reduced = [usize; { 1 + 0 }];
        }
        impl<
            A: Dim,
            B: Dim,
            C: Dim,
        > ReduceShapeTo<(A,), Axes2<{ { 1 + 0 } }, { { 1 + (1 + 0) } }>> for (A, B, C) {}
        impl<
            A: Dim,
            B: Dim,
            C: Dim,
        > ReduceShape<Axes2<{ { 1 + 0 } }, { { 1 + (1 + 0) } }>> for (A, B, C) {
            type Reduced = (A,);
        }
        impl ReduceShapeTo<[usize; { 1 + 0 }], Axes2<{ { 1 + 0 } }, { { 1 + (1 + 0) } }>>
        for [usize; { 1 + (1 + (1 + 0)) }] {}
        impl ReduceShape<Axes2<{ { 1 + 0 } }, { { 1 + (1 + 0) } }>>
        for [usize; { 1 + (1 + (1 + 0)) }] {
            type Reduced = [usize; { 1 + 0 }];
        }
        impl<
            A: Dim,
            B: Dim,
            C: Dim,
            D: Dim,
        > ReduceShapeTo<
            (A,),
            Axes3<{ { 1 + 0 } }, { { 1 + (1 + 0) } }, { { 1 + (1 + (1 + 0)) } }>,
        > for (A, B, C, D) {}
        impl<
            A: Dim,
            B: Dim,
            C: Dim,
            D: Dim,
        > ReduceShape<
            Axes3<{ { 1 + 0 } }, { { 1 + (1 + 0) } }, { { 1 + (1 + (1 + 0)) } }>,
        > for (A, B, C, D) {
            type Reduced = (A,);
        }
        impl ReduceShapeTo<
            [usize; { 1 + 0 }],
            Axes3<{ { 1 + 0 } }, { { 1 + (1 + 0) } }, { { 1 + (1 + (1 + 0)) } }>,
        > for [usize; { 1 + (1 + (1 + (1 + 0))) }] {}
        impl ReduceShape<
            Axes3<{ { 1 + 0 } }, { { 1 + (1 + 0) } }, { { 1 + (1 + (1 + 0)) } }>,
        > for [usize; { 1 + (1 + (1 + (1 + 0))) }] {
            type Reduced = [usize; { 1 + 0 }];
        }
        impl<
            A: Dim,
            B: Dim,
            C: Dim,
            D: Dim,
            E: Dim,
        > ReduceShapeTo<
            (A,),
            Axes4<
                { { 1 + 0 } },
                { { 1 + (1 + 0) } },
                { { 1 + (1 + (1 + 0)) } },
                { { 1 + (1 + (1 + (1 + 0))) } },
            >,
        > for (A, B, C, D, E) {}
        impl<
            A: Dim,
            B: Dim,
            C: Dim,
            D: Dim,
            E: Dim,
        > ReduceShape<
            Axes4<
                { { 1 + 0 } },
                { { 1 + (1 + 0) } },
                { { 1 + (1 + (1 + 0)) } },
                { { 1 + (1 + (1 + (1 + 0))) } },
            >,
        > for (A, B, C, D, E) {
            type Reduced = (A,);
        }
        impl ReduceShapeTo<
            [usize; { 1 + 0 }],
            Axes4<
                { { 1 + 0 } },
                { { 1 + (1 + 0) } },
                { { 1 + (1 + (1 + 0)) } },
                { { 1 + (1 + (1 + (1 + 0))) } },
            >,
        > for [usize; { 1 + (1 + (1 + (1 + (1 + 0)))) }] {}
        impl ReduceShape<
            Axes4<
                { { 1 + 0 } },
                { { 1 + (1 + 0) } },
                { { 1 + (1 + (1 + 0)) } },
                { { 1 + (1 + (1 + (1 + 0))) } },
            >,
        > for [usize; { 1 + (1 + (1 + (1 + (1 + 0)))) }] {
            type Reduced = [usize; { 1 + 0 }];
        }
        impl<
            A: Dim,
            B: Dim,
            C: Dim,
            D: Dim,
            E: Dim,
            F: Dim,
        > ReduceShapeTo<
            (A,),
            Axes5<
                { { 1 + 0 } },
                { { 1 + (1 + 0) } },
                { { 1 + (1 + (1 + 0)) } },
                { { 1 + (1 + (1 + (1 + 0))) } },
                { { 1 + (1 + (1 + (1 + (1 + 0)))) } },
            >,
        > for (A, B, C, D, E, F) {}
        impl<
            A: Dim,
            B: Dim,
            C: Dim,
            D: Dim,
            E: Dim,
            F: Dim,
        > ReduceShape<
            Axes5<
                { { 1 + 0 } },
                { { 1 + (1 + 0) } },
                { { 1 + (1 + (1 + 0)) } },
                { { 1 + (1 + (1 + (1 + 0))) } },
                { { 1 + (1 + (1 + (1 + (1 + 0)))) } },
            >,
        > for (A, B, C, D, E, F) {
            type Reduced = (A,);
        }
        impl ReduceShapeTo<
            [usize; { 1 + 0 }],
            Axes5<
                { { 1 + 0 } },
                { { 1 + (1 + 0) } },
                { { 1 + (1 + (1 + 0)) } },
                { { 1 + (1 + (1 + (1 + 0))) } },
                { { 1 + (1 + (1 + (1 + (1 + 0)))) } },
            >,
        > for [usize; { 1 + (1 + (1 + (1 + (1 + (1 + 0))))) }] {}
        impl ReduceShape<
            Axes5<
                { { 1 + 0 } },
                { { 1 + (1 + 0) } },
                { { 1 + (1 + (1 + 0)) } },
                { { 1 + (1 + (1 + (1 + 0))) } },
                { { 1 + (1 + (1 + (1 + (1 + 0)))) } },
            >,
        > for [usize; { 1 + (1 + (1 + (1 + (1 + (1 + 0))))) }] {
            type Reduced = [usize; { 1 + 0 }];
        }
        impl<
            A: Dim,
            B: Dim,
            C: Dim,
            D: Dim,
            E: Dim,
            F: Dim,
        > ReduceShapeTo<
            (A, F),
            Axes4<
                { { 1 + 0 } },
                { { 1 + (1 + 0) } },
                { { 1 + (1 + (1 + 0)) } },
                { { 1 + (1 + (1 + (1 + 0))) } },
            >,
        > for (A, B, C, D, E, F) {}
        impl<
            A: Dim,
            B: Dim,
            C: Dim,
            D: Dim,
            E: Dim,
            F: Dim,
        > ReduceShape<
            Axes4<
                { { 1 + 0 } },
                { { 1 + (1 + 0) } },
                { { 1 + (1 + (1 + 0)) } },
                { { 1 + (1 + (1 + (1 + 0))) } },
            >,
        > for (A, B, C, D, E, F) {
            type Reduced = (A, F);
        }
        impl ReduceShapeTo<
            [usize; { 1 + (1 + 0) }],
            Axes4<
                { { 1 + 0 } },
                { { 1 + (1 + 0) } },
                { { 1 + (1 + (1 + 0)) } },
                { { 1 + (1 + (1 + (1 + 0))) } },
            >,
        > for [usize; { 1 + (1 + (1 + (1 + (1 + (1 + 0))))) }] {}
        impl ReduceShape<
            Axes4<
                { { 1 + 0 } },
                { { 1 + (1 + 0) } },
                { { 1 + (1 + (1 + 0)) } },
                { { 1 + (1 + (1 + (1 + 0))) } },
            >,
        > for [usize; { 1 + (1 + (1 + (1 + (1 + (1 + 0))))) }] {
            type Reduced = [usize; { 1 + (1 + 0) }];
        }
        impl<
            A: Dim,
            B: Dim,
            C: Dim,
            D: Dim,
            E: Dim,
        > ReduceShapeTo<
            (A, E),
            Axes3<{ { 1 + 0 } }, { { 1 + (1 + 0) } }, { { 1 + (1 + (1 + 0)) } }>,
        > for (A, B, C, D, E) {}
        impl<
            A: Dim,
            B: Dim,
            C: Dim,
            D: Dim,
            E: Dim,
        > ReduceShape<
            Axes3<{ { 1 + 0 } }, { { 1 + (1 + 0) } }, { { 1 + (1 + (1 + 0)) } }>,
        > for (A, B, C, D, E) {
            type Reduced = (A, E);
        }
        impl ReduceShapeTo<
            [usize; { 1 + (1 + 0) }],
            Axes3<{ { 1 + 0 } }, { { 1 + (1 + 0) } }, { { 1 + (1 + (1 + 0)) } }>,
        > for [usize; { 1 + (1 + (1 + (1 + (1 + 0)))) }] {}
        impl ReduceShape<
            Axes3<{ { 1 + 0 } }, { { 1 + (1 + 0) } }, { { 1 + (1 + (1 + 0)) } }>,
        > for [usize; { 1 + (1 + (1 + (1 + (1 + 0)))) }] {
            type Reduced = [usize; { 1 + (1 + 0) }];
        }
        impl<
            A: Dim,
            B: Dim,
            C: Dim,
            D: Dim,
            E: Dim,
            F: Dim,
        > ReduceShapeTo<
            (A, E),
            Axes4<
                { { 1 + 0 } },
                { { 1 + (1 + 0) } },
                { { 1 + (1 + (1 + 0)) } },
                { { 1 + (1 + (1 + (1 + (1 + 0)))) } },
            >,
        > for (A, B, C, D, E, F) {}
        impl<
            A: Dim,
            B: Dim,
            C: Dim,
            D: Dim,
            E: Dim,
            F: Dim,
        > ReduceShape<
            Axes4<
                { { 1 + 0 } },
                { { 1 + (1 + 0) } },
                { { 1 + (1 + (1 + 0)) } },
                { { 1 + (1 + (1 + (1 + (1 + 0)))) } },
            >,
        > for (A, B, C, D, E, F) {
            type Reduced = (A, E);
        }
        impl ReduceShapeTo<
            [usize; { 1 + (1 + 0) }],
            Axes4<
                { { 1 + 0 } },
                { { 1 + (1 + 0) } },
                { { 1 + (1 + (1 + 0)) } },
                { { 1 + (1 + (1 + (1 + (1 + 0)))) } },
            >,
        > for [usize; { 1 + (1 + (1 + (1 + (1 + (1 + 0))))) }] {}
        impl ReduceShape<
            Axes4<
                { { 1 + 0 } },
                { { 1 + (1 + 0) } },
                { { 1 + (1 + (1 + 0)) } },
                { { 1 + (1 + (1 + (1 + (1 + 0)))) } },
            >,
        > for [usize; { 1 + (1 + (1 + (1 + (1 + (1 + 0))))) }] {
            type Reduced = [usize; { 1 + (1 + 0) }];
        }
        impl<
            A: Dim,
            B: Dim,
            C: Dim,
            D: Dim,
            E: Dim,
            F: Dim,
        > ReduceShapeTo<
            (A, E, F),
            Axes3<{ { 1 + 0 } }, { { 1 + (1 + 0) } }, { { 1 + (1 + (1 + 0)) } }>,
        > for (A, B, C, D, E, F) {}
        impl<
            A: Dim,
            B: Dim,
            C: Dim,
            D: Dim,
            E: Dim,
            F: Dim,
        > ReduceShape<
            Axes3<{ { 1 + 0 } }, { { 1 + (1 + 0) } }, { { 1 + (1 + (1 + 0)) } }>,
        > for (A, B, C, D, E, F) {
            type Reduced = (A, E, F);
        }
        impl ReduceShapeTo<
            [usize; { 1 + (1 + (1 + 0)) }],
            Axes3<{ { 1 + 0 } }, { { 1 + (1 + 0) } }, { { 1 + (1 + (1 + 0)) } }>,
        > for [usize; { 1 + (1 + (1 + (1 + (1 + (1 + 0))))) }] {}
        impl ReduceShape<
            Axes3<{ { 1 + 0 } }, { { 1 + (1 + 0) } }, { { 1 + (1 + (1 + 0)) } }>,
        > for [usize; { 1 + (1 + (1 + (1 + (1 + (1 + 0))))) }] {
            type Reduced = [usize; { 1 + (1 + (1 + 0)) }];
        }
        impl<
            A: Dim,
            B: Dim,
            C: Dim,
            D: Dim,
        > ReduceShapeTo<(A, D), Axes2<{ { 1 + 0 } }, { { 1 + (1 + 0) } }>>
        for (A, B, C, D) {}
        impl<
            A: Dim,
            B: Dim,
            C: Dim,
            D: Dim,
        > ReduceShape<Axes2<{ { 1 + 0 } }, { { 1 + (1 + 0) } }>> for (A, B, C, D) {
            type Reduced = (A, D);
        }
        impl ReduceShapeTo<
            [usize; { 1 + (1 + 0) }],
            Axes2<{ { 1 + 0 } }, { { 1 + (1 + 0) } }>,
        > for [usize; { 1 + (1 + (1 + (1 + 0))) }] {}
        impl ReduceShape<Axes2<{ { 1 + 0 } }, { { 1 + (1 + 0) } }>>
        for [usize; { 1 + (1 + (1 + (1 + 0))) }] {
            type Reduced = [usize; { 1 + (1 + 0) }];
        }
        impl<
            A: Dim,
            B: Dim,
            C: Dim,
            D: Dim,
            E: Dim,
        > ReduceShapeTo<
            (A, D),
            Axes3<{ { 1 + 0 } }, { { 1 + (1 + 0) } }, { { 1 + (1 + (1 + (1 + 0))) } }>,
        > for (A, B, C, D, E) {}
        impl<
            A: Dim,
            B: Dim,
            C: Dim,
            D: Dim,
            E: Dim,
        > ReduceShape<
            Axes3<{ { 1 + 0 } }, { { 1 + (1 + 0) } }, { { 1 + (1 + (1 + (1 + 0))) } }>,
        > for (A, B, C, D, E) {
            type Reduced = (A, D);
        }
        impl ReduceShapeTo<
            [usize; { 1 + (1 + 0) }],
            Axes3<{ { 1 + 0 } }, { { 1 + (1 + 0) } }, { { 1 + (1 + (1 + (1 + 0))) } }>,
        > for [usize; { 1 + (1 + (1 + (1 + (1 + 0)))) }] {}
        impl ReduceShape<
            Axes3<{ { 1 + 0 } }, { { 1 + (1 + 0) } }, { { 1 + (1 + (1 + (1 + 0))) } }>,
        > for [usize; { 1 + (1 + (1 + (1 + (1 + 0)))) }] {
            type Reduced = [usize; { 1 + (1 + 0) }];
        }
        impl<
            A: Dim,
            B: Dim,
            C: Dim,
            D: Dim,
            E: Dim,
            F: Dim,
        > ReduceShapeTo<
            (A, D),
            Axes4<
                { { 1 + 0 } },
                { { 1 + (1 + 0) } },
                { { 1 + (1 + (1 + (1 + 0))) } },
                { { 1 + (1 + (1 + (1 + (1 + 0)))) } },
            >,
        > for (A, B, C, D, E, F) {}
        impl<
            A: Dim,
            B: Dim,
            C: Dim,
            D: Dim,
            E: Dim,
            F: Dim,
        > ReduceShape<
            Axes4<
                { { 1 + 0 } },
                { { 1 + (1 + 0) } },
                { { 1 + (1 + (1 + (1 + 0))) } },
                { { 1 + (1 + (1 + (1 + (1 + 0)))) } },
            >,
        > for (A, B, C, D, E, F) {
            type Reduced = (A, D);
        }
        impl ReduceShapeTo<
            [usize; { 1 + (1 + 0) }],
            Axes4<
                { { 1 + 0 } },
                { { 1 + (1 + 0) } },
                { { 1 + (1 + (1 + (1 + 0))) } },
                { { 1 + (1 + (1 + (1 + (1 + 0)))) } },
            >,
        > for [usize; { 1 + (1 + (1 + (1 + (1 + (1 + 0))))) }] {}
        impl ReduceShape<
            Axes4<
                { { 1 + 0 } },
                { { 1 + (1 + 0) } },
                { { 1 + (1 + (1 + (1 + 0))) } },
                { { 1 + (1 + (1 + (1 + (1 + 0)))) } },
            >,
        > for [usize; { 1 + (1 + (1 + (1 + (1 + (1 + 0))))) }] {
            type Reduced = [usize; { 1 + (1 + 0) }];
        }
        impl<
            A: Dim,
            B: Dim,
            C: Dim,
            D: Dim,
            E: Dim,
            F: Dim,
        > ReduceShapeTo<
            (A, D, F),
            Axes3<{ { 1 + 0 } }, { { 1 + (1 + 0) } }, { { 1 + (1 + (1 + (1 + 0))) } }>,
        > for (A, B, C, D, E, F) {}
        impl<
            A: Dim,
            B: Dim,
            C: Dim,
            D: Dim,
            E: Dim,
            F: Dim,
        > ReduceShape<
            Axes3<{ { 1 + 0 } }, { { 1 + (1 + 0) } }, { { 1 + (1 + (1 + (1 + 0))) } }>,
        > for (A, B, C, D, E, F) {
            type Reduced = (A, D, F);
        }
        impl ReduceShapeTo<
            [usize; { 1 + (1 + (1 + 0)) }],
            Axes3<{ { 1 + 0 } }, { { 1 + (1 + 0) } }, { { 1 + (1 + (1 + (1 + 0))) } }>,
        > for [usize; { 1 + (1 + (1 + (1 + (1 + (1 + 0))))) }] {}
        impl ReduceShape<
            Axes3<{ { 1 + 0 } }, { { 1 + (1 + 0) } }, { { 1 + (1 + (1 + (1 + 0))) } }>,
        > for [usize; { 1 + (1 + (1 + (1 + (1 + (1 + 0))))) }] {
            type Reduced = [usize; { 1 + (1 + (1 + 0)) }];
        }
        impl<
            A: Dim,
            B: Dim,
            C: Dim,
            D: Dim,
            E: Dim,
        > ReduceShapeTo<(A, D, E), Axes2<{ { 1 + 0 } }, { { 1 + (1 + 0) } }>>
        for (A, B, C, D, E) {}
        impl<
            A: Dim,
            B: Dim,
            C: Dim,
            D: Dim,
            E: Dim,
        > ReduceShape<Axes2<{ { 1 + 0 } }, { { 1 + (1 + 0) } }>> for (A, B, C, D, E) {
            type Reduced = (A, D, E);
        }
        impl ReduceShapeTo<
            [usize; { 1 + (1 + (1 + 0)) }],
            Axes2<{ { 1 + 0 } }, { { 1 + (1 + 0) } }>,
        > for [usize; { 1 + (1 + (1 + (1 + (1 + 0)))) }] {}
        impl ReduceShape<Axes2<{ { 1 + 0 } }, { { 1 + (1 + 0) } }>>
        for [usize; { 1 + (1 + (1 + (1 + (1 + 0)))) }] {
            type Reduced = [usize; { 1 + (1 + (1 + 0)) }];
        }
        impl<
            A: Dim,
            B: Dim,
            C: Dim,
            D: Dim,
            E: Dim,
            F: Dim,
        > ReduceShapeTo<
            (A, D, E),
            Axes3<
                { { 1 + 0 } },
                { { 1 + (1 + 0) } },
                { { 1 + (1 + (1 + (1 + (1 + 0)))) } },
            >,
        > for (A, B, C, D, E, F) {}
        impl<
            A: Dim,
            B: Dim,
            C: Dim,
            D: Dim,
            E: Dim,
            F: Dim,
        > ReduceShape<
            Axes3<
                { { 1 + 0 } },
                { { 1 + (1 + 0) } },
                { { 1 + (1 + (1 + (1 + (1 + 0)))) } },
            >,
        > for (A, B, C, D, E, F) {
            type Reduced = (A, D, E);
        }
        impl ReduceShapeTo<
            [usize; { 1 + (1 + (1 + 0)) }],
            Axes3<
                { { 1 + 0 } },
                { { 1 + (1 + 0) } },
                { { 1 + (1 + (1 + (1 + (1 + 0)))) } },
            >,
        > for [usize; { 1 + (1 + (1 + (1 + (1 + (1 + 0))))) }] {}
        impl ReduceShape<
            Axes3<
                { { 1 + 0 } },
                { { 1 + (1 + 0) } },
                { { 1 + (1 + (1 + (1 + (1 + 0)))) } },
            >,
        > for [usize; { 1 + (1 + (1 + (1 + (1 + (1 + 0))))) }] {
            type Reduced = [usize; { 1 + (1 + (1 + 0)) }];
        }
        impl<
            A: Dim,
            B: Dim,
            C: Dim,
            D: Dim,
            E: Dim,
            F: Dim,
        > ReduceShapeTo<(A, D, E, F), Axes2<{ { 1 + 0 } }, { { 1 + (1 + 0) } }>>
        for (A, B, C, D, E, F) {}
        impl<
            A: Dim,
            B: Dim,
            C: Dim,
            D: Dim,
            E: Dim,
            F: Dim,
        > ReduceShape<Axes2<{ { 1 + 0 } }, { { 1 + (1 + 0) } }>> for (A, B, C, D, E, F) {
            type Reduced = (A, D, E, F);
        }
        impl ReduceShapeTo<
            [usize; { 1 + (1 + (1 + (1 + 0))) }],
            Axes2<{ { 1 + 0 } }, { { 1 + (1 + 0) } }>,
        > for [usize; { 1 + (1 + (1 + (1 + (1 + (1 + 0))))) }] {}
        impl ReduceShape<Axes2<{ { 1 + 0 } }, { { 1 + (1 + 0) } }>>
        for [usize; { 1 + (1 + (1 + (1 + (1 + (1 + 0))))) }] {
            type Reduced = [usize; { 1 + (1 + (1 + (1 + 0))) }];
        }
        impl<A: Dim, B: Dim, C: Dim> ReduceShapeTo<(A, C), Axis<{ { 1 + 0 } }>>
        for (A, B, C) {}
        impl<A: Dim, B: Dim, C: Dim> ReduceShape<Axis<{ { 1 + 0 } }>> for (A, B, C) {
            type Reduced = (A, C);
        }
        impl ReduceShapeTo<[usize; { 1 + (1 + 0) }], Axis<{ { 1 + 0 } }>>
        for [usize; { 1 + (1 + (1 + 0)) }] {}
        impl ReduceShape<Axis<{ { 1 + 0 } }>> for [usize; { 1 + (1 + (1 + 0)) }] {
            type Reduced = [usize; { 1 + (1 + 0) }];
        }
        impl<
            A: Dim,
            B: Dim,
            C: Dim,
            D: Dim,
        > ReduceShapeTo<(A, C), Axes2<{ { 1 + 0 } }, { { 1 + (1 + (1 + 0)) } }>>
        for (A, B, C, D) {}
        impl<
            A: Dim,
            B: Dim,
            C: Dim,
            D: Dim,
        > ReduceShape<Axes2<{ { 1 + 0 } }, { { 1 + (1 + (1 + 0)) } }>> for (A, B, C, D) {
            type Reduced = (A, C);
        }
        impl ReduceShapeTo<
            [usize; { 1 + (1 + 0) }],
            Axes2<{ { 1 + 0 } }, { { 1 + (1 + (1 + 0)) } }>,
        > for [usize; { 1 + (1 + (1 + (1 + 0))) }] {}
        impl ReduceShape<Axes2<{ { 1 + 0 } }, { { 1 + (1 + (1 + 0)) } }>>
        for [usize; { 1 + (1 + (1 + (1 + 0))) }] {
            type Reduced = [usize; { 1 + (1 + 0) }];
        }
        impl<
            A: Dim,
            B: Dim,
            C: Dim,
            D: Dim,
            E: Dim,
        > ReduceShapeTo<
            (A, C),
            Axes3<
                { { 1 + 0 } },
                { { 1 + (1 + (1 + 0)) } },
                { { 1 + (1 + (1 + (1 + 0))) } },
            >,
        > for (A, B, C, D, E) {}
        impl<
            A: Dim,
            B: Dim,
            C: Dim,
            D: Dim,
            E: Dim,
        > ReduceShape<
            Axes3<
                { { 1 + 0 } },
                { { 1 + (1 + (1 + 0)) } },
                { { 1 + (1 + (1 + (1 + 0))) } },
            >,
        > for (A, B, C, D, E) {
            type Reduced = (A, C);
        }
        impl ReduceShapeTo<
            [usize; { 1 + (1 + 0) }],
            Axes3<
                { { 1 + 0 } },
                { { 1 + (1 + (1 + 0)) } },
                { { 1 + (1 + (1 + (1 + 0))) } },
            >,
        > for [usize; { 1 + (1 + (1 + (1 + (1 + 0)))) }] {}
        impl ReduceShape<
            Axes3<
                { { 1 + 0 } },
                { { 1 + (1 + (1 + 0)) } },
                { { 1 + (1 + (1 + (1 + 0))) } },
            >,
        > for [usize; { 1 + (1 + (1 + (1 + (1 + 0)))) }] {
            type Reduced = [usize; { 1 + (1 + 0) }];
        }
        impl<
            A: Dim,
            B: Dim,
            C: Dim,
            D: Dim,
            E: Dim,
            F: Dim,
        > ReduceShapeTo<
            (A, C),
            Axes4<
                { { 1 + 0 } },
                { { 1 + (1 + (1 + 0)) } },
                { { 1 + (1 + (1 + (1 + 0))) } },
                { { 1 + (1 + (1 + (1 + (1 + 0)))) } },
            >,
        > for (A, B, C, D, E, F) {}
        impl<
            A: Dim,
            B: Dim,
            C: Dim,
            D: Dim,
            E: Dim,
            F: Dim,
        > ReduceShape<
            Axes4<
                { { 1 + 0 } },
                { { 1 + (1 + (1 + 0)) } },
                { { 1 + (1 + (1 + (1 + 0))) } },
                { { 1 + (1 + (1 + (1 + (1 + 0)))) } },
            >,
        > for (A, B, C, D, E, F) {
            type Reduced = (A, C);
        }
        impl ReduceShapeTo<
            [usize; { 1 + (1 + 0) }],
            Axes4<
                { { 1 + 0 } },
                { { 1 + (1 + (1 + 0)) } },
                { { 1 + (1 + (1 + (1 + 0))) } },
                { { 1 + (1 + (1 + (1 + (1 + 0)))) } },
            >,
        > for [usize; { 1 + (1 + (1 + (1 + (1 + (1 + 0))))) }] {}
        impl ReduceShape<
            Axes4<
                { { 1 + 0 } },
                { { 1 + (1 + (1 + 0)) } },
                { { 1 + (1 + (1 + (1 + 0))) } },
                { { 1 + (1 + (1 + (1 + (1 + 0)))) } },
            >,
        > for [usize; { 1 + (1 + (1 + (1 + (1 + (1 + 0))))) }] {
            type Reduced = [usize; { 1 + (1 + 0) }];
        }
        impl<
            A: Dim,
            B: Dim,
            C: Dim,
            D: Dim,
            E: Dim,
            F: Dim,
        > ReduceShapeTo<
            (A, C, F),
            Axes3<
                { { 1 + 0 } },
                { { 1 + (1 + (1 + 0)) } },
                { { 1 + (1 + (1 + (1 + 0))) } },
            >,
        > for (A, B, C, D, E, F) {}
        impl<
            A: Dim,
            B: Dim,
            C: Dim,
            D: Dim,
            E: Dim,
            F: Dim,
        > ReduceShape<
            Axes3<
                { { 1 + 0 } },
                { { 1 + (1 + (1 + 0)) } },
                { { 1 + (1 + (1 + (1 + 0))) } },
            >,
        > for (A, B, C, D, E, F) {
            type Reduced = (A, C, F);
        }
        impl ReduceShapeTo<
            [usize; { 1 + (1 + (1 + 0)) }],
            Axes3<
                { { 1 + 0 } },
                { { 1 + (1 + (1 + 0)) } },
                { { 1 + (1 + (1 + (1 + 0))) } },
            >,
        > for [usize; { 1 + (1 + (1 + (1 + (1 + (1 + 0))))) }] {}
        impl ReduceShape<
            Axes3<
                { { 1 + 0 } },
                { { 1 + (1 + (1 + 0)) } },
                { { 1 + (1 + (1 + (1 + 0))) } },
            >,
        > for [usize; { 1 + (1 + (1 + (1 + (1 + (1 + 0))))) }] {
            type Reduced = [usize; { 1 + (1 + (1 + 0)) }];
        }
        impl<
            A: Dim,
            B: Dim,
            C: Dim,
            D: Dim,
            E: Dim,
        > ReduceShapeTo<(A, C, E), Axes2<{ { 1 + 0 } }, { { 1 + (1 + (1 + 0)) } }>>
        for (A, B, C, D, E) {}
        impl<
            A: Dim,
            B: Dim,
            C: Dim,
            D: Dim,
            E: Dim,
        > ReduceShape<Axes2<{ { 1 + 0 } }, { { 1 + (1 + (1 + 0)) } }>>
        for (A, B, C, D, E) {
            type Reduced = (A, C, E);
        }
        impl ReduceShapeTo<
            [usize; { 1 + (1 + (1 + 0)) }],
            Axes2<{ { 1 + 0 } }, { { 1 + (1 + (1 + 0)) } }>,
        > for [usize; { 1 + (1 + (1 + (1 + (1 + 0)))) }] {}
        impl ReduceShape<Axes2<{ { 1 + 0 } }, { { 1 + (1 + (1 + 0)) } }>>
        for [usize; { 1 + (1 + (1 + (1 + (1 + 0)))) }] {
            type Reduced = [usize; { 1 + (1 + (1 + 0)) }];
        }
        impl<
            A: Dim,
            B: Dim,
            C: Dim,
            D: Dim,
            E: Dim,
            F: Dim,
        > ReduceShapeTo<
            (A, C, E),
            Axes3<
                { { 1 + 0 } },
                { { 1 + (1 + (1 + 0)) } },
                { { 1 + (1 + (1 + (1 + (1 + 0)))) } },
            >,
        > for (A, B, C, D, E, F) {}
        impl<
            A: Dim,
            B: Dim,
            C: Dim,
            D: Dim,
            E: Dim,
            F: Dim,
        > ReduceShape<
            Axes3<
                { { 1 + 0 } },
                { { 1 + (1 + (1 + 0)) } },
                { { 1 + (1 + (1 + (1 + (1 + 0)))) } },
            >,
        > for (A, B, C, D, E, F) {
            type Reduced = (A, C, E);
        }
        impl ReduceShapeTo<
            [usize; { 1 + (1 + (1 + 0)) }],
            Axes3<
                { { 1 + 0 } },
                { { 1 + (1 + (1 + 0)) } },
                { { 1 + (1 + (1 + (1 + (1 + 0)))) } },
            >,
        > for [usize; { 1 + (1 + (1 + (1 + (1 + (1 + 0))))) }] {}
        impl ReduceShape<
            Axes3<
                { { 1 + 0 } },
                { { 1 + (1 + (1 + 0)) } },
                { { 1 + (1 + (1 + (1 + (1 + 0)))) } },
            >,
        > for [usize; { 1 + (1 + (1 + (1 + (1 + (1 + 0))))) }] {
            type Reduced = [usize; { 1 + (1 + (1 + 0)) }];
        }
        impl<
            A: Dim,
            B: Dim,
            C: Dim,
            D: Dim,
            E: Dim,
            F: Dim,
        > ReduceShapeTo<(A, C, E, F), Axes2<{ { 1 + 0 } }, { { 1 + (1 + (1 + 0)) } }>>
        for (A, B, C, D, E, F) {}
        impl<
            A: Dim,
            B: Dim,
            C: Dim,
            D: Dim,
            E: Dim,
            F: Dim,
        > ReduceShape<Axes2<{ { 1 + 0 } }, { { 1 + (1 + (1 + 0)) } }>>
        for (A, B, C, D, E, F) {
            type Reduced = (A, C, E, F);
        }
        impl ReduceShapeTo<
            [usize; { 1 + (1 + (1 + (1 + 0))) }],
            Axes2<{ { 1 + 0 } }, { { 1 + (1 + (1 + 0)) } }>,
        > for [usize; { 1 + (1 + (1 + (1 + (1 + (1 + 0))))) }] {}
        impl ReduceShape<Axes2<{ { 1 + 0 } }, { { 1 + (1 + (1 + 0)) } }>>
        for [usize; { 1 + (1 + (1 + (1 + (1 + (1 + 0))))) }] {
            type Reduced = [usize; { 1 + (1 + (1 + (1 + 0))) }];
        }
        impl<
            A: Dim,
            B: Dim,
            C: Dim,
            D: Dim,
        > ReduceShapeTo<(A, C, D), Axis<{ { 1 + 0 } }>> for (A, B, C, D) {}
        impl<A: Dim, B: Dim, C: Dim, D: Dim> ReduceShape<Axis<{ { 1 + 0 } }>>
        for (A, B, C, D) {
            type Reduced = (A, C, D);
        }
        impl ReduceShapeTo<[usize; { 1 + (1 + (1 + 0)) }], Axis<{ { 1 + 0 } }>>
        for [usize; { 1 + (1 + (1 + (1 + 0))) }] {}
        impl ReduceShape<Axis<{ { 1 + 0 } }>> for [usize; { 1 + (1 + (1 + (1 + 0))) }] {
            type Reduced = [usize; { 1 + (1 + (1 + 0)) }];
        }
        impl<
            A: Dim,
            B: Dim,
            C: Dim,
            D: Dim,
            E: Dim,
        > ReduceShapeTo<(A, C, D), Axes2<{ { 1 + 0 } }, { { 1 + (1 + (1 + (1 + 0))) } }>>
        for (A, B, C, D, E) {}
        impl<
            A: Dim,
            B: Dim,
            C: Dim,
            D: Dim,
            E: Dim,
        > ReduceShape<Axes2<{ { 1 + 0 } }, { { 1 + (1 + (1 + (1 + 0))) } }>>
        for (A, B, C, D, E) {
            type Reduced = (A, C, D);
        }
        impl ReduceShapeTo<
            [usize; { 1 + (1 + (1 + 0)) }],
            Axes2<{ { 1 + 0 } }, { { 1 + (1 + (1 + (1 + 0))) } }>,
        > for [usize; { 1 + (1 + (1 + (1 + (1 + 0)))) }] {}
        impl ReduceShape<Axes2<{ { 1 + 0 } }, { { 1 + (1 + (1 + (1 + 0))) } }>>
        for [usize; { 1 + (1 + (1 + (1 + (1 + 0)))) }] {
            type Reduced = [usize; { 1 + (1 + (1 + 0)) }];
        }
        impl<
            A: Dim,
            B: Dim,
            C: Dim,
            D: Dim,
            E: Dim,
            F: Dim,
        > ReduceShapeTo<
            (A, C, D),
            Axes3<
                { { 1 + 0 } },
                { { 1 + (1 + (1 + (1 + 0))) } },
                { { 1 + (1 + (1 + (1 + (1 + 0)))) } },
            >,
        > for (A, B, C, D, E, F) {}
        impl<
            A: Dim,
            B: Dim,
            C: Dim,
            D: Dim,
            E: Dim,
            F: Dim,
        > ReduceShape<
            Axes3<
                { { 1 + 0 } },
                { { 1 + (1 + (1 + (1 + 0))) } },
                { { 1 + (1 + (1 + (1 + (1 + 0)))) } },
            >,
        > for (A, B, C, D, E, F) {
            type Reduced = (A, C, D);
        }
        impl ReduceShapeTo<
            [usize; { 1 + (1 + (1 + 0)) }],
            Axes3<
                { { 1 + 0 } },
                { { 1 + (1 + (1 + (1 + 0))) } },
                { { 1 + (1 + (1 + (1 + (1 + 0)))) } },
            >,
        > for [usize; { 1 + (1 + (1 + (1 + (1 + (1 + 0))))) }] {}
        impl ReduceShape<
            Axes3<
                { { 1 + 0 } },
                { { 1 + (1 + (1 + (1 + 0))) } },
                { { 1 + (1 + (1 + (1 + (1 + 0)))) } },
            >,
        > for [usize; { 1 + (1 + (1 + (1 + (1 + (1 + 0))))) }] {
            type Reduced = [usize; { 1 + (1 + (1 + 0)) }];
        }
        impl<
            A: Dim,
            B: Dim,
            C: Dim,
            D: Dim,
            E: Dim,
            F: Dim,
        > ReduceShapeTo<
            (A, C, D, F),
            Axes2<{ { 1 + 0 } }, { { 1 + (1 + (1 + (1 + 0))) } }>,
        > for (A, B, C, D, E, F) {}
        impl<
            A: Dim,
            B: Dim,
            C: Dim,
            D: Dim,
            E: Dim,
            F: Dim,
        > ReduceShape<Axes2<{ { 1 + 0 } }, { { 1 + (1 + (1 + (1 + 0))) } }>>
        for (A, B, C, D, E, F) {
            type Reduced = (A, C, D, F);
        }
        impl ReduceShapeTo<
            [usize; { 1 + (1 + (1 + (1 + 0))) }],
            Axes2<{ { 1 + 0 } }, { { 1 + (1 + (1 + (1 + 0))) } }>,
        > for [usize; { 1 + (1 + (1 + (1 + (1 + (1 + 0))))) }] {}
        impl ReduceShape<Axes2<{ { 1 + 0 } }, { { 1 + (1 + (1 + (1 + 0))) } }>>
        for [usize; { 1 + (1 + (1 + (1 + (1 + (1 + 0))))) }] {
            type Reduced = [usize; { 1 + (1 + (1 + (1 + 0))) }];
        }
        impl<
            A: Dim,
            B: Dim,
            C: Dim,
            D: Dim,
            E: Dim,
        > ReduceShapeTo<(A, C, D, E), Axis<{ { 1 + 0 } }>> for (A, B, C, D, E) {}
        impl<A: Dim, B: Dim, C: Dim, D: Dim, E: Dim> ReduceShape<Axis<{ { 1 + 0 } }>>
        for (A, B, C, D, E) {
            type Reduced = (A, C, D, E);
        }
        impl ReduceShapeTo<[usize; { 1 + (1 + (1 + (1 + 0))) }], Axis<{ { 1 + 0 } }>>
        for [usize; { 1 + (1 + (1 + (1 + (1 + 0)))) }] {}
        impl ReduceShape<Axis<{ { 1 + 0 } }>>
        for [usize; { 1 + (1 + (1 + (1 + (1 + 0)))) }] {
            type Reduced = [usize; { 1 + (1 + (1 + (1 + 0))) }];
        }
        impl<
            A: Dim,
            B: Dim,
            C: Dim,
            D: Dim,
            E: Dim,
            F: Dim,
        > ReduceShapeTo<
            (A, C, D, E),
            Axes2<{ { 1 + 0 } }, { { 1 + (1 + (1 + (1 + (1 + 0)))) } }>,
        > for (A, B, C, D, E, F) {}
        impl<
            A: Dim,
            B: Dim,
            C: Dim,
            D: Dim,
            E: Dim,
            F: Dim,
        > ReduceShape<Axes2<{ { 1 + 0 } }, { { 1 + (1 + (1 + (1 + (1 + 0)))) } }>>
        for (A, B, C, D, E, F) {
            type Reduced = (A, C, D, E);
        }
        impl ReduceShapeTo<
            [usize; { 1 + (1 + (1 + (1 + 0))) }],
            Axes2<{ { 1 + 0 } }, { { 1 + (1 + (1 + (1 + (1 + 0)))) } }>,
        > for [usize; { 1 + (1 + (1 + (1 + (1 + (1 + 0))))) }] {}
        impl ReduceShape<Axes2<{ { 1 + 0 } }, { { 1 + (1 + (1 + (1 + (1 + 0)))) } }>>
        for [usize; { 1 + (1 + (1 + (1 + (1 + (1 + 0))))) }] {
            type Reduced = [usize; { 1 + (1 + (1 + (1 + 0))) }];
        }
        impl<
            A: Dim,
            B: Dim,
            C: Dim,
            D: Dim,
            E: Dim,
            F: Dim,
        > ReduceShapeTo<(A, C, D, E, F), Axis<{ { 1 + 0 } }>> for (A, B, C, D, E, F) {}
        impl<
            A: Dim,
            B: Dim,
            C: Dim,
            D: Dim,
            E: Dim,
            F: Dim,
        > ReduceShape<Axis<{ { 1 + 0 } }>> for (A, B, C, D, E, F) {
            type Reduced = (A, C, D, E, F);
        }
        impl ReduceShapeTo<
            [usize; { 1 + (1 + (1 + (1 + (1 + 0)))) }],
            Axis<{ { 1 + 0 } }>,
        > for [usize; { 1 + (1 + (1 + (1 + (1 + (1 + 0))))) }] {}
        impl ReduceShape<Axis<{ { 1 + 0 } }>>
        for [usize; { 1 + (1 + (1 + (1 + (1 + (1 + 0))))) }] {
            type Reduced = [usize; { 1 + (1 + (1 + (1 + (1 + 0)))) }];
        }
        impl<A: Dim, B: Dim, C: Dim> ReduceShapeTo<(A, B), Axis<{ { 1 + (1 + 0) } }>>
        for (A, B, C) {}
        impl<A: Dim, B: Dim, C: Dim> ReduceShape<Axis<{ { 1 + (1 + 0) } }>>
        for (A, B, C) {
            type Reduced = (A, B);
        }
        impl ReduceShapeTo<[usize; { 1 + (1 + 0) }], Axis<{ { 1 + (1 + 0) } }>>
        for [usize; { 1 + (1 + (1 + 0)) }] {}
        impl ReduceShape<Axis<{ { 1 + (1 + 0) } }>> for [usize; { 1 + (1 + (1 + 0)) }] {
            type Reduced = [usize; { 1 + (1 + 0) }];
        }
        impl<
            A: Dim,
            B: Dim,
            C: Dim,
            D: Dim,
        > ReduceShapeTo<(A, B), Axes2<{ { 1 + (1 + 0) } }, { { 1 + (1 + (1 + 0)) } }>>
        for (A, B, C, D) {}
        impl<
            A: Dim,
            B: Dim,
            C: Dim,
            D: Dim,
        > ReduceShape<Axes2<{ { 1 + (1 + 0) } }, { { 1 + (1 + (1 + 0)) } }>>
        for (A, B, C, D) {
            type Reduced = (A, B);
        }
        impl ReduceShapeTo<
            [usize; { 1 + (1 + 0) }],
            Axes2<{ { 1 + (1 + 0) } }, { { 1 + (1 + (1 + 0)) } }>,
        > for [usize; { 1 + (1 + (1 + (1 + 0))) }] {}
        impl ReduceShape<Axes2<{ { 1 + (1 + 0) } }, { { 1 + (1 + (1 + 0)) } }>>
        for [usize; { 1 + (1 + (1 + (1 + 0))) }] {
            type Reduced = [usize; { 1 + (1 + 0) }];
        }
        impl<
            A: Dim,
            B: Dim,
            C: Dim,
            D: Dim,
            E: Dim,
        > ReduceShapeTo<
            (A, B),
            Axes3<
                { { 1 + (1 + 0) } },
                { { 1 + (1 + (1 + 0)) } },
                { { 1 + (1 + (1 + (1 + 0))) } },
            >,
        > for (A, B, C, D, E) {}
        impl<
            A: Dim,
            B: Dim,
            C: Dim,
            D: Dim,
            E: Dim,
        > ReduceShape<
            Axes3<
                { { 1 + (1 + 0) } },
                { { 1 + (1 + (1 + 0)) } },
                { { 1 + (1 + (1 + (1 + 0))) } },
            >,
        > for (A, B, C, D, E) {
            type Reduced = (A, B);
        }
        impl ReduceShapeTo<
            [usize; { 1 + (1 + 0) }],
            Axes3<
                { { 1 + (1 + 0) } },
                { { 1 + (1 + (1 + 0)) } },
                { { 1 + (1 + (1 + (1 + 0))) } },
            >,
        > for [usize; { 1 + (1 + (1 + (1 + (1 + 0)))) }] {}
        impl ReduceShape<
            Axes3<
                { { 1 + (1 + 0) } },
                { { 1 + (1 + (1 + 0)) } },
                { { 1 + (1 + (1 + (1 + 0))) } },
            >,
        > for [usize; { 1 + (1 + (1 + (1 + (1 + 0)))) }] {
            type Reduced = [usize; { 1 + (1 + 0) }];
        }
        impl<
            A: Dim,
            B: Dim,
            C: Dim,
            D: Dim,
            E: Dim,
            F: Dim,
        > ReduceShapeTo<
            (A, B),
            Axes4<
                { { 1 + (1 + 0) } },
                { { 1 + (1 + (1 + 0)) } },
                { { 1 + (1 + (1 + (1 + 0))) } },
                { { 1 + (1 + (1 + (1 + (1 + 0)))) } },
            >,
        > for (A, B, C, D, E, F) {}
        impl<
            A: Dim,
            B: Dim,
            C: Dim,
            D: Dim,
            E: Dim,
            F: Dim,
        > ReduceShape<
            Axes4<
                { { 1 + (1 + 0) } },
                { { 1 + (1 + (1 + 0)) } },
                { { 1 + (1 + (1 + (1 + 0))) } },
                { { 1 + (1 + (1 + (1 + (1 + 0)))) } },
            >,
        > for (A, B, C, D, E, F) {
            type Reduced = (A, B);
        }
        impl ReduceShapeTo<
            [usize; { 1 + (1 + 0) }],
            Axes4<
                { { 1 + (1 + 0) } },
                { { 1 + (1 + (1 + 0)) } },
                { { 1 + (1 + (1 + (1 + 0))) } },
                { { 1 + (1 + (1 + (1 + (1 + 0)))) } },
            >,
        > for [usize; { 1 + (1 + (1 + (1 + (1 + (1 + 0))))) }] {}
        impl ReduceShape<
            Axes4<
                { { 1 + (1 + 0) } },
                { { 1 + (1 + (1 + 0)) } },
                { { 1 + (1 + (1 + (1 + 0))) } },
                { { 1 + (1 + (1 + (1 + (1 + 0)))) } },
            >,
        > for [usize; { 1 + (1 + (1 + (1 + (1 + (1 + 0))))) }] {
            type Reduced = [usize; { 1 + (1 + 0) }];
        }
        impl<
            A: Dim,
            B: Dim,
            C: Dim,
            D: Dim,
            E: Dim,
            F: Dim,
        > ReduceShapeTo<
            (A, B, F),
            Axes3<
                { { 1 + (1 + 0) } },
                { { 1 + (1 + (1 + 0)) } },
                { { 1 + (1 + (1 + (1 + 0))) } },
            >,
        > for (A, B, C, D, E, F) {}
        impl<
            A: Dim,
            B: Dim,
            C: Dim,
            D: Dim,
            E: Dim,
            F: Dim,
        > ReduceShape<
            Axes3<
                { { 1 + (1 + 0) } },
                { { 1 + (1 + (1 + 0)) } },
                { { 1 + (1 + (1 + (1 + 0))) } },
            >,
        > for (A, B, C, D, E, F) {
            type Reduced = (A, B, F);
        }
        impl ReduceShapeTo<
            [usize; { 1 + (1 + (1 + 0)) }],
            Axes3<
                { { 1 + (1 + 0) } },
                { { 1 + (1 + (1 + 0)) } },
                { { 1 + (1 + (1 + (1 + 0))) } },
            >,
        > for [usize; { 1 + (1 + (1 + (1 + (1 + (1 + 0))))) }] {}
        impl ReduceShape<
            Axes3<
                { { 1 + (1 + 0) } },
                { { 1 + (1 + (1 + 0)) } },
                { { 1 + (1 + (1 + (1 + 0))) } },
            >,
        > for [usize; { 1 + (1 + (1 + (1 + (1 + (1 + 0))))) }] {
            type Reduced = [usize; { 1 + (1 + (1 + 0)) }];
        }
        impl<
            A: Dim,
            B: Dim,
            C: Dim,
            D: Dim,
            E: Dim,
        > ReduceShapeTo<(A, B, E), Axes2<{ { 1 + (1 + 0) } }, { { 1 + (1 + (1 + 0)) } }>>
        for (A, B, C, D, E) {}
        impl<
            A: Dim,
            B: Dim,
            C: Dim,
            D: Dim,
            E: Dim,
        > ReduceShape<Axes2<{ { 1 + (1 + 0) } }, { { 1 + (1 + (1 + 0)) } }>>
        for (A, B, C, D, E) {
            type Reduced = (A, B, E);
        }
        impl ReduceShapeTo<
            [usize; { 1 + (1 + (1 + 0)) }],
            Axes2<{ { 1 + (1 + 0) } }, { { 1 + (1 + (1 + 0)) } }>,
        > for [usize; { 1 + (1 + (1 + (1 + (1 + 0)))) }] {}
        impl ReduceShape<Axes2<{ { 1 + (1 + 0) } }, { { 1 + (1 + (1 + 0)) } }>>
        for [usize; { 1 + (1 + (1 + (1 + (1 + 0)))) }] {
            type Reduced = [usize; { 1 + (1 + (1 + 0)) }];
        }
        impl<
            A: Dim,
            B: Dim,
            C: Dim,
            D: Dim,
            E: Dim,
            F: Dim,
        > ReduceShapeTo<
            (A, B, E),
            Axes3<
                { { 1 + (1 + 0) } },
                { { 1 + (1 + (1 + 0)) } },
                { { 1 + (1 + (1 + (1 + (1 + 0)))) } },
            >,
        > for (A, B, C, D, E, F) {}
        impl<
            A: Dim,
            B: Dim,
            C: Dim,
            D: Dim,
            E: Dim,
            F: Dim,
        > ReduceShape<
            Axes3<
                { { 1 + (1 + 0) } },
                { { 1 + (1 + (1 + 0)) } },
                { { 1 + (1 + (1 + (1 + (1 + 0)))) } },
            >,
        > for (A, B, C, D, E, F) {
            type Reduced = (A, B, E);
        }
        impl ReduceShapeTo<
            [usize; { 1 + (1 + (1 + 0)) }],
            Axes3<
                { { 1 + (1 + 0) } },
                { { 1 + (1 + (1 + 0)) } },
                { { 1 + (1 + (1 + (1 + (1 + 0)))) } },
            >,
        > for [usize; { 1 + (1 + (1 + (1 + (1 + (1 + 0))))) }] {}
        impl ReduceShape<
            Axes3<
                { { 1 + (1 + 0) } },
                { { 1 + (1 + (1 + 0)) } },
                { { 1 + (1 + (1 + (1 + (1 + 0)))) } },
            >,
        > for [usize; { 1 + (1 + (1 + (1 + (1 + (1 + 0))))) }] {
            type Reduced = [usize; { 1 + (1 + (1 + 0)) }];
        }
        impl<
            A: Dim,
            B: Dim,
            C: Dim,
            D: Dim,
            E: Dim,
            F: Dim,
        > ReduceShapeTo<
            (A, B, E, F),
            Axes2<{ { 1 + (1 + 0) } }, { { 1 + (1 + (1 + 0)) } }>,
        > for (A, B, C, D, E, F) {}
        impl<
            A: Dim,
            B: Dim,
            C: Dim,
            D: Dim,
            E: Dim,
            F: Dim,
        > ReduceShape<Axes2<{ { 1 + (1 + 0) } }, { { 1 + (1 + (1 + 0)) } }>>
        for (A, B, C, D, E, F) {
            type Reduced = (A, B, E, F);
        }
        impl ReduceShapeTo<
            [usize; { 1 + (1 + (1 + (1 + 0))) }],
            Axes2<{ { 1 + (1 + 0) } }, { { 1 + (1 + (1 + 0)) } }>,
        > for [usize; { 1 + (1 + (1 + (1 + (1 + (1 + 0))))) }] {}
        impl ReduceShape<Axes2<{ { 1 + (1 + 0) } }, { { 1 + (1 + (1 + 0)) } }>>
        for [usize; { 1 + (1 + (1 + (1 + (1 + (1 + 0))))) }] {
            type Reduced = [usize; { 1 + (1 + (1 + (1 + 0))) }];
        }
        impl<
            A: Dim,
            B: Dim,
            C: Dim,
            D: Dim,
        > ReduceShapeTo<(A, B, D), Axis<{ { 1 + (1 + 0) } }>> for (A, B, C, D) {}
        impl<A: Dim, B: Dim, C: Dim, D: Dim> ReduceShape<Axis<{ { 1 + (1 + 0) } }>>
        for (A, B, C, D) {
            type Reduced = (A, B, D);
        }
        impl ReduceShapeTo<[usize; { 1 + (1 + (1 + 0)) }], Axis<{ { 1 + (1 + 0) } }>>
        for [usize; { 1 + (1 + (1 + (1 + 0))) }] {}
        impl ReduceShape<Axis<{ { 1 + (1 + 0) } }>>
        for [usize; { 1 + (1 + (1 + (1 + 0))) }] {
            type Reduced = [usize; { 1 + (1 + (1 + 0)) }];
        }
        impl<
            A: Dim,
            B: Dim,
            C: Dim,
            D: Dim,
            E: Dim,
        > ReduceShapeTo<
            (A, B, D),
            Axes2<{ { 1 + (1 + 0) } }, { { 1 + (1 + (1 + (1 + 0))) } }>,
        > for (A, B, C, D, E) {}
        impl<
            A: Dim,
            B: Dim,
            C: Dim,
            D: Dim,
            E: Dim,
        > ReduceShape<Axes2<{ { 1 + (1 + 0) } }, { { 1 + (1 + (1 + (1 + 0))) } }>>
        for (A, B, C, D, E) {
            type Reduced = (A, B, D);
        }
        impl ReduceShapeTo<
            [usize; { 1 + (1 + (1 + 0)) }],
            Axes2<{ { 1 + (1 + 0) } }, { { 1 + (1 + (1 + (1 + 0))) } }>,
        > for [usize; { 1 + (1 + (1 + (1 + (1 + 0)))) }] {}
        impl ReduceShape<Axes2<{ { 1 + (1 + 0) } }, { { 1 + (1 + (1 + (1 + 0))) } }>>
        for [usize; { 1 + (1 + (1 + (1 + (1 + 0)))) }] {
            type Reduced = [usize; { 1 + (1 + (1 + 0)) }];
        }
        impl<
            A: Dim,
            B: Dim,
            C: Dim,
            D: Dim,
            E: Dim,
            F: Dim,
        > ReduceShapeTo<
            (A, B, D),
            Axes3<
                { { 1 + (1 + 0) } },
                { { 1 + (1 + (1 + (1 + 0))) } },
                { { 1 + (1 + (1 + (1 + (1 + 0)))) } },
            >,
        > for (A, B, C, D, E, F) {}
        impl<
            A: Dim,
            B: Dim,
            C: Dim,
            D: Dim,
            E: Dim,
            F: Dim,
        > ReduceShape<
            Axes3<
                { { 1 + (1 + 0) } },
                { { 1 + (1 + (1 + (1 + 0))) } },
                { { 1 + (1 + (1 + (1 + (1 + 0)))) } },
            >,
        > for (A, B, C, D, E, F) {
            type Reduced = (A, B, D);
        }
        impl ReduceShapeTo<
            [usize; { 1 + (1 + (1 + 0)) }],
            Axes3<
                { { 1 + (1 + 0) } },
                { { 1 + (1 + (1 + (1 + 0))) } },
                { { 1 + (1 + (1 + (1 + (1 + 0)))) } },
            >,
        > for [usize; { 1 + (1 + (1 + (1 + (1 + (1 + 0))))) }] {}
        impl ReduceShape<
            Axes3<
                { { 1 + (1 + 0) } },
                { { 1 + (1 + (1 + (1 + 0))) } },
                { { 1 + (1 + (1 + (1 + (1 + 0)))) } },
            >,
        > for [usize; { 1 + (1 + (1 + (1 + (1 + (1 + 0))))) }] {
            type Reduced = [usize; { 1 + (1 + (1 + 0)) }];
        }
        impl<
            A: Dim,
            B: Dim,
            C: Dim,
            D: Dim,
            E: Dim,
            F: Dim,
        > ReduceShapeTo<
            (A, B, D, F),
            Axes2<{ { 1 + (1 + 0) } }, { { 1 + (1 + (1 + (1 + 0))) } }>,
        > for (A, B, C, D, E, F) {}
        impl<
            A: Dim,
            B: Dim,
            C: Dim,
            D: Dim,
            E: Dim,
            F: Dim,
        > ReduceShape<Axes2<{ { 1 + (1 + 0) } }, { { 1 + (1 + (1 + (1 + 0))) } }>>
        for (A, B, C, D, E, F) {
            type Reduced = (A, B, D, F);
        }
        impl ReduceShapeTo<
            [usize; { 1 + (1 + (1 + (1 + 0))) }],
            Axes2<{ { 1 + (1 + 0) } }, { { 1 + (1 + (1 + (1 + 0))) } }>,
        > for [usize; { 1 + (1 + (1 + (1 + (1 + (1 + 0))))) }] {}
        impl ReduceShape<Axes2<{ { 1 + (1 + 0) } }, { { 1 + (1 + (1 + (1 + 0))) } }>>
        for [usize; { 1 + (1 + (1 + (1 + (1 + (1 + 0))))) }] {
            type Reduced = [usize; { 1 + (1 + (1 + (1 + 0))) }];
        }
        impl<
            A: Dim,
            B: Dim,
            C: Dim,
            D: Dim,
            E: Dim,
        > ReduceShapeTo<(A, B, D, E), Axis<{ { 1 + (1 + 0) } }>> for (A, B, C, D, E) {}
        impl<
            A: Dim,
            B: Dim,
            C: Dim,
            D: Dim,
            E: Dim,
        > ReduceShape<Axis<{ { 1 + (1 + 0) } }>> for (A, B, C, D, E) {
            type Reduced = (A, B, D, E);
        }
        impl ReduceShapeTo<
            [usize; { 1 + (1 + (1 + (1 + 0))) }],
            Axis<{ { 1 + (1 + 0) } }>,
        > for [usize; { 1 + (1 + (1 + (1 + (1 + 0)))) }] {}
        impl ReduceShape<Axis<{ { 1 + (1 + 0) } }>>
        for [usize; { 1 + (1 + (1 + (1 + (1 + 0)))) }] {
            type Reduced = [usize; { 1 + (1 + (1 + (1 + 0))) }];
        }
        impl<
            A: Dim,
            B: Dim,
            C: Dim,
            D: Dim,
            E: Dim,
            F: Dim,
        > ReduceShapeTo<
            (A, B, D, E),
            Axes2<{ { 1 + (1 + 0) } }, { { 1 + (1 + (1 + (1 + (1 + 0)))) } }>,
        > for (A, B, C, D, E, F) {}
        impl<
            A: Dim,
            B: Dim,
            C: Dim,
            D: Dim,
            E: Dim,
            F: Dim,
        > ReduceShape<Axes2<{ { 1 + (1 + 0) } }, { { 1 + (1 + (1 + (1 + (1 + 0)))) } }>>
        for (A, B, C, D, E, F) {
            type Reduced = (A, B, D, E);
        }
        impl ReduceShapeTo<
            [usize; { 1 + (1 + (1 + (1 + 0))) }],
            Axes2<{ { 1 + (1 + 0) } }, { { 1 + (1 + (1 + (1 + (1 + 0)))) } }>,
        > for [usize; { 1 + (1 + (1 + (1 + (1 + (1 + 0))))) }] {}
        impl ReduceShape<
            Axes2<{ { 1 + (1 + 0) } }, { { 1 + (1 + (1 + (1 + (1 + 0)))) } }>,
        > for [usize; { 1 + (1 + (1 + (1 + (1 + (1 + 0))))) }] {
            type Reduced = [usize; { 1 + (1 + (1 + (1 + 0))) }];
        }
        impl<
            A: Dim,
            B: Dim,
            C: Dim,
            D: Dim,
            E: Dim,
            F: Dim,
        > ReduceShapeTo<(A, B, D, E, F), Axis<{ { 1 + (1 + 0) } }>>
        for (A, B, C, D, E, F) {}
        impl<
            A: Dim,
            B: Dim,
            C: Dim,
            D: Dim,
            E: Dim,
            F: Dim,
        > ReduceShape<Axis<{ { 1 + (1 + 0) } }>> for (A, B, C, D, E, F) {
            type Reduced = (A, B, D, E, F);
        }
        impl ReduceShapeTo<
            [usize; { 1 + (1 + (1 + (1 + (1 + 0)))) }],
            Axis<{ { 1 + (1 + 0) } }>,
        > for [usize; { 1 + (1 + (1 + (1 + (1 + (1 + 0))))) }] {}
        impl ReduceShape<Axis<{ { 1 + (1 + 0) } }>>
        for [usize; { 1 + (1 + (1 + (1 + (1 + (1 + 0))))) }] {
            type Reduced = [usize; { 1 + (1 + (1 + (1 + (1 + 0)))) }];
        }
        impl<
            A: Dim,
            B: Dim,
            C: Dim,
            D: Dim,
        > ReduceShapeTo<(A, B, C), Axis<{ { 1 + (1 + (1 + 0)) } }>> for (A, B, C, D) {}
        impl<A: Dim, B: Dim, C: Dim, D: Dim> ReduceShape<Axis<{ { 1 + (1 + (1 + 0)) } }>>
        for (A, B, C, D) {
            type Reduced = (A, B, C);
        }
        impl ReduceShapeTo<
            [usize; { 1 + (1 + (1 + 0)) }],
            Axis<{ { 1 + (1 + (1 + 0)) } }>,
        > for [usize; { 1 + (1 + (1 + (1 + 0))) }] {}
        impl ReduceShape<Axis<{ { 1 + (1 + (1 + 0)) } }>>
        for [usize; { 1 + (1 + (1 + (1 + 0))) }] {
            type Reduced = [usize; { 1 + (1 + (1 + 0)) }];
        }
        impl<
            A: Dim,
            B: Dim,
            C: Dim,
            D: Dim,
            E: Dim,
        > ReduceShapeTo<
            (A, B, C),
            Axes2<{ { 1 + (1 + (1 + 0)) } }, { { 1 + (1 + (1 + (1 + 0))) } }>,
        > for (A, B, C, D, E) {}
        impl<
            A: Dim,
            B: Dim,
            C: Dim,
            D: Dim,
            E: Dim,
        > ReduceShape<Axes2<{ { 1 + (1 + (1 + 0)) } }, { { 1 + (1 + (1 + (1 + 0))) } }>>
        for (A, B, C, D, E) {
            type Reduced = (A, B, C);
        }
        impl ReduceShapeTo<
            [usize; { 1 + (1 + (1 + 0)) }],
            Axes2<{ { 1 + (1 + (1 + 0)) } }, { { 1 + (1 + (1 + (1 + 0))) } }>,
        > for [usize; { 1 + (1 + (1 + (1 + (1 + 0)))) }] {}
        impl ReduceShape<
            Axes2<{ { 1 + (1 + (1 + 0)) } }, { { 1 + (1 + (1 + (1 + 0))) } }>,
        > for [usize; { 1 + (1 + (1 + (1 + (1 + 0)))) }] {
            type Reduced = [usize; { 1 + (1 + (1 + 0)) }];
        }
        impl<
            A: Dim,
            B: Dim,
            C: Dim,
            D: Dim,
            E: Dim,
            F: Dim,
        > ReduceShapeTo<
            (A, B, C),
            Axes3<
                { { 1 + (1 + (1 + 0)) } },
                { { 1 + (1 + (1 + (1 + 0))) } },
                { { 1 + (1 + (1 + (1 + (1 + 0)))) } },
            >,
        > for (A, B, C, D, E, F) {}
        impl<
            A: Dim,
            B: Dim,
            C: Dim,
            D: Dim,
            E: Dim,
            F: Dim,
        > ReduceShape<
            Axes3<
                { { 1 + (1 + (1 + 0)) } },
                { { 1 + (1 + (1 + (1 + 0))) } },
                { { 1 + (1 + (1 + (1 + (1 + 0)))) } },
            >,
        > for (A, B, C, D, E, F) {
            type Reduced = (A, B, C);
        }
        impl ReduceShapeTo<
            [usize; { 1 + (1 + (1 + 0)) }],
            Axes3<
                { { 1 + (1 + (1 + 0)) } },
                { { 1 + (1 + (1 + (1 + 0))) } },
                { { 1 + (1 + (1 + (1 + (1 + 0)))) } },
            >,
        > for [usize; { 1 + (1 + (1 + (1 + (1 + (1 + 0))))) }] {}
        impl ReduceShape<
            Axes3<
                { { 1 + (1 + (1 + 0)) } },
                { { 1 + (1 + (1 + (1 + 0))) } },
                { { 1 + (1 + (1 + (1 + (1 + 0)))) } },
            >,
        > for [usize; { 1 + (1 + (1 + (1 + (1 + (1 + 0))))) }] {
            type Reduced = [usize; { 1 + (1 + (1 + 0)) }];
        }
        impl<
            A: Dim,
            B: Dim,
            C: Dim,
            D: Dim,
            E: Dim,
            F: Dim,
        > ReduceShapeTo<
            (A, B, C, F),
            Axes2<{ { 1 + (1 + (1 + 0)) } }, { { 1 + (1 + (1 + (1 + 0))) } }>,
        > for (A, B, C, D, E, F) {}
        impl<
            A: Dim,
            B: Dim,
            C: Dim,
            D: Dim,
            E: Dim,
            F: Dim,
        > ReduceShape<Axes2<{ { 1 + (1 + (1 + 0)) } }, { { 1 + (1 + (1 + (1 + 0))) } }>>
        for (A, B, C, D, E, F) {
            type Reduced = (A, B, C, F);
        }
        impl ReduceShapeTo<
            [usize; { 1 + (1 + (1 + (1 + 0))) }],
            Axes2<{ { 1 + (1 + (1 + 0)) } }, { { 1 + (1 + (1 + (1 + 0))) } }>,
        > for [usize; { 1 + (1 + (1 + (1 + (1 + (1 + 0))))) }] {}
        impl ReduceShape<
            Axes2<{ { 1 + (1 + (1 + 0)) } }, { { 1 + (1 + (1 + (1 + 0))) } }>,
        > for [usize; { 1 + (1 + (1 + (1 + (1 + (1 + 0))))) }] {
            type Reduced = [usize; { 1 + (1 + (1 + (1 + 0))) }];
        }
        impl<
            A: Dim,
            B: Dim,
            C: Dim,
            D: Dim,
            E: Dim,
        > ReduceShapeTo<(A, B, C, E), Axis<{ { 1 + (1 + (1 + 0)) } }>>
        for (A, B, C, D, E) {}
        impl<
            A: Dim,
            B: Dim,
            C: Dim,
            D: Dim,
            E: Dim,
        > ReduceShape<Axis<{ { 1 + (1 + (1 + 0)) } }>> for (A, B, C, D, E) {
            type Reduced = (A, B, C, E);
        }
        impl ReduceShapeTo<
            [usize; { 1 + (1 + (1 + (1 + 0))) }],
            Axis<{ { 1 + (1 + (1 + 0)) } }>,
        > for [usize; { 1 + (1 + (1 + (1 + (1 + 0)))) }] {}
        impl ReduceShape<Axis<{ { 1 + (1 + (1 + 0)) } }>>
        for [usize; { 1 + (1 + (1 + (1 + (1 + 0)))) }] {
            type Reduced = [usize; { 1 + (1 + (1 + (1 + 0))) }];
        }
        impl<
            A: Dim,
            B: Dim,
            C: Dim,
            D: Dim,
            E: Dim,
            F: Dim,
        > ReduceShapeTo<
            (A, B, C, E),
            Axes2<{ { 1 + (1 + (1 + 0)) } }, { { 1 + (1 + (1 + (1 + (1 + 0)))) } }>,
        > for (A, B, C, D, E, F) {}
        impl<
            A: Dim,
            B: Dim,
            C: Dim,
            D: Dim,
            E: Dim,
            F: Dim,
        > ReduceShape<
            Axes2<{ { 1 + (1 + (1 + 0)) } }, { { 1 + (1 + (1 + (1 + (1 + 0)))) } }>,
        > for (A, B, C, D, E, F) {
            type Reduced = (A, B, C, E);
        }
        impl ReduceShapeTo<
            [usize; { 1 + (1 + (1 + (1 + 0))) }],
            Axes2<{ { 1 + (1 + (1 + 0)) } }, { { 1 + (1 + (1 + (1 + (1 + 0)))) } }>,
        > for [usize; { 1 + (1 + (1 + (1 + (1 + (1 + 0))))) }] {}
        impl ReduceShape<
            Axes2<{ { 1 + (1 + (1 + 0)) } }, { { 1 + (1 + (1 + (1 + (1 + 0)))) } }>,
        > for [usize; { 1 + (1 + (1 + (1 + (1 + (1 + 0))))) }] {
            type Reduced = [usize; { 1 + (1 + (1 + (1 + 0))) }];
        }
        impl<
            A: Dim,
            B: Dim,
            C: Dim,
            D: Dim,
            E: Dim,
            F: Dim,
        > ReduceShapeTo<(A, B, C, E, F), Axis<{ { 1 + (1 + (1 + 0)) } }>>
        for (A, B, C, D, E, F) {}
        impl<
            A: Dim,
            B: Dim,
            C: Dim,
            D: Dim,
            E: Dim,
            F: Dim,
        > ReduceShape<Axis<{ { 1 + (1 + (1 + 0)) } }>> for (A, B, C, D, E, F) {
            type Reduced = (A, B, C, E, F);
        }
        impl ReduceShapeTo<
            [usize; { 1 + (1 + (1 + (1 + (1 + 0)))) }],
            Axis<{ { 1 + (1 + (1 + 0)) } }>,
        > for [usize; { 1 + (1 + (1 + (1 + (1 + (1 + 0))))) }] {}
        impl ReduceShape<Axis<{ { 1 + (1 + (1 + 0)) } }>>
        for [usize; { 1 + (1 + (1 + (1 + (1 + (1 + 0))))) }] {
            type Reduced = [usize; { 1 + (1 + (1 + (1 + (1 + 0)))) }];
        }
        impl<
            A: Dim,
            B: Dim,
            C: Dim,
            D: Dim,
            E: Dim,
        > ReduceShapeTo<(A, B, C, D), Axis<{ { 1 + (1 + (1 + (1 + 0))) } }>>
        for (A, B, C, D, E) {}
        impl<
            A: Dim,
            B: Dim,
            C: Dim,
            D: Dim,
            E: Dim,
        > ReduceShape<Axis<{ { 1 + (1 + (1 + (1 + 0))) } }>> for (A, B, C, D, E) {
            type Reduced = (A, B, C, D);
        }
        impl ReduceShapeTo<
            [usize; { 1 + (1 + (1 + (1 + 0))) }],
            Axis<{ { 1 + (1 + (1 + (1 + 0))) } }>,
        > for [usize; { 1 + (1 + (1 + (1 + (1 + 0)))) }] {}
        impl ReduceShape<Axis<{ { 1 + (1 + (1 + (1 + 0))) } }>>
        for [usize; { 1 + (1 + (1 + (1 + (1 + 0)))) }] {
            type Reduced = [usize; { 1 + (1 + (1 + (1 + 0))) }];
        }
        impl<
            A: Dim,
            B: Dim,
            C: Dim,
            D: Dim,
            E: Dim,
            F: Dim,
        > ReduceShapeTo<
            (A, B, C, D),
            Axes2<{ { 1 + (1 + (1 + (1 + 0))) } }, { { 1 + (1 + (1 + (1 + (1 + 0)))) } }>,
        > for (A, B, C, D, E, F) {}
        impl<
            A: Dim,
            B: Dim,
            C: Dim,
            D: Dim,
            E: Dim,
            F: Dim,
        > ReduceShape<
            Axes2<{ { 1 + (1 + (1 + (1 + 0))) } }, { { 1 + (1 + (1 + (1 + (1 + 0)))) } }>,
        > for (A, B, C, D, E, F) {
            type Reduced = (A, B, C, D);
        }
        impl ReduceShapeTo<
            [usize; { 1 + (1 + (1 + (1 + 0))) }],
            Axes2<{ { 1 + (1 + (1 + (1 + 0))) } }, { { 1 + (1 + (1 + (1 + (1 + 0)))) } }>,
        > for [usize; { 1 + (1 + (1 + (1 + (1 + (1 + 0))))) }] {}
        impl ReduceShape<
            Axes2<{ { 1 + (1 + (1 + (1 + 0))) } }, { { 1 + (1 + (1 + (1 + (1 + 0)))) } }>,
        > for [usize; { 1 + (1 + (1 + (1 + (1 + (1 + 0))))) }] {
            type Reduced = [usize; { 1 + (1 + (1 + (1 + 0))) }];
        }
        impl<
            A: Dim,
            B: Dim,
            C: Dim,
            D: Dim,
            E: Dim,
            F: Dim,
        > ReduceShapeTo<(A, B, C, D, F), Axis<{ { 1 + (1 + (1 + (1 + 0))) } }>>
        for (A, B, C, D, E, F) {}
        impl<
            A: Dim,
            B: Dim,
            C: Dim,
            D: Dim,
            E: Dim,
            F: Dim,
        > ReduceShape<Axis<{ { 1 + (1 + (1 + (1 + 0))) } }>> for (A, B, C, D, E, F) {
            type Reduced = (A, B, C, D, F);
        }
        impl ReduceShapeTo<
            [usize; { 1 + (1 + (1 + (1 + (1 + 0)))) }],
            Axis<{ { 1 + (1 + (1 + (1 + 0))) } }>,
        > for [usize; { 1 + (1 + (1 + (1 + (1 + (1 + 0))))) }] {}
        impl ReduceShape<Axis<{ { 1 + (1 + (1 + (1 + 0))) } }>>
        for [usize; { 1 + (1 + (1 + (1 + (1 + (1 + 0))))) }] {
            type Reduced = [usize; { 1 + (1 + (1 + (1 + (1 + 0)))) }];
        }
        impl<
            A: Dim,
            B: Dim,
            C: Dim,
            D: Dim,
            E: Dim,
            F: Dim,
        > ReduceShapeTo<(A, B, C, D, E), Axis<{ { 1 + (1 + (1 + (1 + (1 + 0)))) } }>>
        for (A, B, C, D, E, F) {}
        impl<
            A: Dim,
            B: Dim,
            C: Dim,
            D: Dim,
            E: Dim,
            F: Dim,
        > ReduceShape<Axis<{ { 1 + (1 + (1 + (1 + (1 + 0)))) } }>>
        for (A, B, C, D, E, F) {
            type Reduced = (A, B, C, D, E);
        }
        impl ReduceShapeTo<
            [usize; { 1 + (1 + (1 + (1 + (1 + 0)))) }],
            Axis<{ { 1 + (1 + (1 + (1 + (1 + 0)))) } }>,
        > for [usize; { 1 + (1 + (1 + (1 + (1 + (1 + 0))))) }] {}
        impl ReduceShape<Axis<{ { 1 + (1 + (1 + (1 + (1 + 0)))) } }>>
        for [usize; { 1 + (1 + (1 + (1 + (1 + (1 + 0))))) }] {
            type Reduced = [usize; { 1 + (1 + (1 + (1 + (1 + 0)))) }];
        }
        /// Internal implementation for broadcasting strides
        pub trait BroadcastStridesTo<S: Shape, Ax>: Shape + BroadcastShapeTo<S, Ax> {
            fn check(&self, dst: &S);
            fn broadcast_strides(&self, strides: Self::Concrete) -> S::Concrete;
        }
        impl<Src: Shape, Dst: Shape, Ax: Axes> BroadcastStridesTo<Dst, Ax> for Src
        where
            Self: BroadcastShapeTo<Dst, Ax>,
        {
            #[inline(always)]
            fn check(&self, dst: &Dst) {
                let src_dims = self.concrete();
                let dst_dims = dst.concrete();
                let mut j = 0;
                for i in 0..Dst::NUM_DIMS {
                    if !Ax::as_array().into_iter().any(|x| x == i as isize) {
                        match (&dst_dims[i], &src_dims[j]) {
                            (left_val, right_val) => {
                                if !(*left_val == *right_val) {
                                    let kind = ::core::panicking::AssertKind::Eq;
                                    ::core::panicking::assert_failed(
                                        kind,
                                        &*left_val,
                                        &*right_val,
                                        ::core::option::Option::None,
                                    );
                                }
                            }
                        };
                        j += 1;
                    }
                }
            }
            #[inline(always)]
            fn broadcast_strides(&self, strides: Self::Concrete) -> Dst::Concrete {
                let mut new_strides: Dst::Concrete = Default::default();
                let mut j = 0;
                for i in 0..Dst::NUM_DIMS {
                    if !Ax::as_array().into_iter().any(|x| x == i as isize) {
                        new_strides[i] = strides[j];
                        j += 1;
                    }
                }
                new_strides
            }
        }
        /// Internal implementation for reducing a shape
        pub trait ReduceStridesTo<S: Shape, Ax>: Shape + ReduceShapeTo<S, Ax> {
            fn reduced(&self) -> S;
        }
        impl<Src: Shape, Dst: Shape, Ax: Axes> ReduceStridesTo<Dst, Ax> for Src
        where
            Self: ReduceShapeTo<Dst, Ax>,
        {
            #[inline(always)]
            fn reduced(&self) -> Dst {
                let src_dims = self.concrete();
                let mut dst_dims: Dst::Concrete = Default::default();
                let mut i_dst = 0;
                for i_src in 0..Src::NUM_DIMS {
                    if !Ax::as_array().into_iter().any(|x| x == i_src as isize) {
                        dst_dims[i_dst] = src_dims[i_src];
                        i_dst += 1;
                    }
                }
                Dst::from_concrete(&dst_dims).unwrap()
            }
        }
    }
    pub use broadcasts::{
        BroadcastShapeTo, BroadcastStridesTo, ReduceShape, ReduceShapeTo, ReduceStridesTo,
    };
    mod permutes {
        use super::*;
        /// Marker for shapes that can be permuted into `Dst` by using `Ax`
        /// as the new indices.
        ///
        /// E.g. `PermuteShapeTo<_, Axes2<1, 0>>` would mean you can reverse
        /// axes 0 and 1.
        pub trait PermuteShapeTo<Dst, Ax> {}
        pub trait PermuteStridesTo<S: Shape, Ax>: Shape + PermuteShapeTo<S, Ax> {
            fn permuted(&self) -> S;
            fn permute_strides(&self, strides: Self::Concrete) -> S::Concrete;
        }
        impl<Src: Shape, Dst: Shape, Ax: Axes> PermuteStridesTo<Dst, Ax> for Src
        where
            Self: PermuteShapeTo<Dst, Ax>,
        {
            #[inline(always)]
            fn permuted(&self) -> Dst {
                let src_dims = self.concrete();
                let mut dst_dims: Dst::Concrete = Default::default();
                for (i_dst, i_src) in Ax::as_array().into_iter().enumerate() {
                    dst_dims[i_dst] = src_dims[i_src as usize];
                }
                Dst::from_concrete(&dst_dims).unwrap()
            }
            #[inline(always)]
            fn permute_strides(&self, src_strides: Self::Concrete) -> Dst::Concrete {
                let mut dst_strides: Dst::Concrete = Default::default();
                for (i, idx) in Ax::as_array().into_iter().enumerate() {
                    dst_strides[i] = src_strides[idx as usize];
                }
                dst_strides
            }
        }
        impl<D1: Dim, D2: Dim> PermuteShapeTo<(D2, D1), Axes2<1, 0>> for (D1, D2) {}
        impl<D1: Dim, D2: Dim, D3: Dim> PermuteShapeTo<(D1, D3, D2), Axes3<0, 2, 1>>
        for (D1, D2, D3) {}
        impl<D1: Dim, D2: Dim, D3: Dim> PermuteShapeTo<(D2, D1, D3), Axes3<1, 0, 2>>
        for (D1, D2, D3) {}
        impl<D1: Dim, D2: Dim, D3: Dim> PermuteShapeTo<(D2, D3, D1), Axes3<1, 2, 0>>
        for (D1, D2, D3) {}
        impl<D1: Dim, D2: Dim, D3: Dim> PermuteShapeTo<(D3, D1, D2), Axes3<2, 0, 1>>
        for (D1, D2, D3) {}
        impl<D1: Dim, D2: Dim, D3: Dim> PermuteShapeTo<(D3, D2, D1), Axes3<2, 1, 0>>
        for (D1, D2, D3) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
        > PermuteShapeTo<(D1, D2, D3, D4), Axes4<0, 1, 2, 3>> for (D1, D2, D3, D4) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
        > PermuteShapeTo<(D1, D2, D4, D3), Axes4<0, 1, 3, 2>> for (D1, D2, D3, D4) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
        > PermuteShapeTo<(D1, D3, D2, D4), Axes4<0, 2, 1, 3>> for (D1, D2, D3, D4) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
        > PermuteShapeTo<(D1, D3, D4, D2), Axes4<0, 2, 3, 1>> for (D1, D2, D3, D4) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
        > PermuteShapeTo<(D1, D4, D2, D3), Axes4<0, 3, 1, 2>> for (D1, D2, D3, D4) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
        > PermuteShapeTo<(D1, D4, D3, D2), Axes4<0, 3, 2, 1>> for (D1, D2, D3, D4) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
        > PermuteShapeTo<(D2, D1, D3, D4), Axes4<1, 0, 2, 3>> for (D1, D2, D3, D4) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
        > PermuteShapeTo<(D2, D1, D4, D3), Axes4<1, 0, 3, 2>> for (D1, D2, D3, D4) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
        > PermuteShapeTo<(D2, D3, D1, D4), Axes4<1, 2, 0, 3>> for (D1, D2, D3, D4) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
        > PermuteShapeTo<(D2, D3, D4, D1), Axes4<1, 2, 3, 0>> for (D1, D2, D3, D4) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
        > PermuteShapeTo<(D2, D4, D1, D3), Axes4<1, 3, 0, 2>> for (D1, D2, D3, D4) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
        > PermuteShapeTo<(D2, D4, D3, D1), Axes4<1, 3, 2, 0>> for (D1, D2, D3, D4) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
        > PermuteShapeTo<(D3, D1, D2, D4), Axes4<2, 0, 1, 3>> for (D1, D2, D3, D4) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
        > PermuteShapeTo<(D3, D1, D4, D2), Axes4<2, 0, 3, 1>> for (D1, D2, D3, D4) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
        > PermuteShapeTo<(D3, D2, D1, D4), Axes4<2, 1, 0, 3>> for (D1, D2, D3, D4) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
        > PermuteShapeTo<(D3, D2, D4, D1), Axes4<2, 1, 3, 0>> for (D1, D2, D3, D4) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
        > PermuteShapeTo<(D3, D4, D1, D2), Axes4<2, 3, 0, 1>> for (D1, D2, D3, D4) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
        > PermuteShapeTo<(D3, D4, D2, D1), Axes4<2, 3, 1, 0>> for (D1, D2, D3, D4) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
        > PermuteShapeTo<(D4, D1, D2, D3), Axes4<3, 0, 1, 2>> for (D1, D2, D3, D4) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
        > PermuteShapeTo<(D4, D1, D3, D2), Axes4<3, 0, 2, 1>> for (D1, D2, D3, D4) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
        > PermuteShapeTo<(D4, D2, D1, D3), Axes4<3, 1, 0, 2>> for (D1, D2, D3, D4) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
        > PermuteShapeTo<(D4, D2, D3, D1), Axes4<3, 1, 2, 0>> for (D1, D2, D3, D4) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
        > PermuteShapeTo<(D4, D3, D1, D2), Axes4<3, 2, 0, 1>> for (D1, D2, D3, D4) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
        > PermuteShapeTo<(D4, D3, D2, D1), Axes4<3, 2, 1, 0>> for (D1, D2, D3, D4) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
        > PermuteShapeTo<(D1, D2, D3, D4, D5), Axes5<0, 1, 2, 3, 4>>
        for (D1, D2, D3, D4, D5) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
        > PermuteShapeTo<(D1, D2, D3, D5, D4), Axes5<0, 1, 2, 4, 3>>
        for (D1, D2, D3, D4, D5) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
        > PermuteShapeTo<(D1, D2, D4, D3, D5), Axes5<0, 1, 3, 2, 4>>
        for (D1, D2, D3, D4, D5) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
        > PermuteShapeTo<(D1, D2, D4, D5, D3), Axes5<0, 1, 3, 4, 2>>
        for (D1, D2, D3, D4, D5) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
        > PermuteShapeTo<(D1, D2, D5, D3, D4), Axes5<0, 1, 4, 2, 3>>
        for (D1, D2, D3, D4, D5) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
        > PermuteShapeTo<(D1, D2, D5, D4, D3), Axes5<0, 1, 4, 3, 2>>
        for (D1, D2, D3, D4, D5) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
        > PermuteShapeTo<(D1, D3, D2, D4, D5), Axes5<0, 2, 1, 3, 4>>
        for (D1, D2, D3, D4, D5) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
        > PermuteShapeTo<(D1, D3, D2, D5, D4), Axes5<0, 2, 1, 4, 3>>
        for (D1, D2, D3, D4, D5) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
        > PermuteShapeTo<(D1, D3, D4, D2, D5), Axes5<0, 2, 3, 1, 4>>
        for (D1, D2, D3, D4, D5) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
        > PermuteShapeTo<(D1, D3, D4, D5, D2), Axes5<0, 2, 3, 4, 1>>
        for (D1, D2, D3, D4, D5) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
        > PermuteShapeTo<(D1, D3, D5, D2, D4), Axes5<0, 2, 4, 1, 3>>
        for (D1, D2, D3, D4, D5) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
        > PermuteShapeTo<(D1, D3, D5, D4, D2), Axes5<0, 2, 4, 3, 1>>
        for (D1, D2, D3, D4, D5) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
        > PermuteShapeTo<(D1, D4, D2, D3, D5), Axes5<0, 3, 1, 2, 4>>
        for (D1, D2, D3, D4, D5) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
        > PermuteShapeTo<(D1, D4, D2, D5, D3), Axes5<0, 3, 1, 4, 2>>
        for (D1, D2, D3, D4, D5) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
        > PermuteShapeTo<(D1, D4, D3, D2, D5), Axes5<0, 3, 2, 1, 4>>
        for (D1, D2, D3, D4, D5) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
        > PermuteShapeTo<(D1, D4, D3, D5, D2), Axes5<0, 3, 2, 4, 1>>
        for (D1, D2, D3, D4, D5) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
        > PermuteShapeTo<(D1, D4, D5, D2, D3), Axes5<0, 3, 4, 1, 2>>
        for (D1, D2, D3, D4, D5) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
        > PermuteShapeTo<(D1, D4, D5, D3, D2), Axes5<0, 3, 4, 2, 1>>
        for (D1, D2, D3, D4, D5) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
        > PermuteShapeTo<(D1, D5, D2, D3, D4), Axes5<0, 4, 1, 2, 3>>
        for (D1, D2, D3, D4, D5) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
        > PermuteShapeTo<(D1, D5, D2, D4, D3), Axes5<0, 4, 1, 3, 2>>
        for (D1, D2, D3, D4, D5) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
        > PermuteShapeTo<(D1, D5, D3, D2, D4), Axes5<0, 4, 2, 1, 3>>
        for (D1, D2, D3, D4, D5) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
        > PermuteShapeTo<(D1, D5, D3, D4, D2), Axes5<0, 4, 2, 3, 1>>
        for (D1, D2, D3, D4, D5) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
        > PermuteShapeTo<(D1, D5, D4, D2, D3), Axes5<0, 4, 3, 1, 2>>
        for (D1, D2, D3, D4, D5) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
        > PermuteShapeTo<(D1, D5, D4, D3, D2), Axes5<0, 4, 3, 2, 1>>
        for (D1, D2, D3, D4, D5) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
        > PermuteShapeTo<(D2, D1, D3, D4, D5), Axes5<1, 0, 2, 3, 4>>
        for (D1, D2, D3, D4, D5) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
        > PermuteShapeTo<(D2, D1, D3, D5, D4), Axes5<1, 0, 2, 4, 3>>
        for (D1, D2, D3, D4, D5) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
        > PermuteShapeTo<(D2, D1, D4, D3, D5), Axes5<1, 0, 3, 2, 4>>
        for (D1, D2, D3, D4, D5) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
        > PermuteShapeTo<(D2, D1, D4, D5, D3), Axes5<1, 0, 3, 4, 2>>
        for (D1, D2, D3, D4, D5) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
        > PermuteShapeTo<(D2, D1, D5, D3, D4), Axes5<1, 0, 4, 2, 3>>
        for (D1, D2, D3, D4, D5) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
        > PermuteShapeTo<(D2, D1, D5, D4, D3), Axes5<1, 0, 4, 3, 2>>
        for (D1, D2, D3, D4, D5) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
        > PermuteShapeTo<(D2, D3, D1, D4, D5), Axes5<1, 2, 0, 3, 4>>
        for (D1, D2, D3, D4, D5) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
        > PermuteShapeTo<(D2, D3, D1, D5, D4), Axes5<1, 2, 0, 4, 3>>
        for (D1, D2, D3, D4, D5) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
        > PermuteShapeTo<(D2, D3, D4, D1, D5), Axes5<1, 2, 3, 0, 4>>
        for (D1, D2, D3, D4, D5) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
        > PermuteShapeTo<(D2, D3, D4, D5, D1), Axes5<1, 2, 3, 4, 0>>
        for (D1, D2, D3, D4, D5) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
        > PermuteShapeTo<(D2, D3, D5, D1, D4), Axes5<1, 2, 4, 0, 3>>
        for (D1, D2, D3, D4, D5) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
        > PermuteShapeTo<(D2, D3, D5, D4, D1), Axes5<1, 2, 4, 3, 0>>
        for (D1, D2, D3, D4, D5) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
        > PermuteShapeTo<(D2, D4, D1, D3, D5), Axes5<1, 3, 0, 2, 4>>
        for (D1, D2, D3, D4, D5) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
        > PermuteShapeTo<(D2, D4, D1, D5, D3), Axes5<1, 3, 0, 4, 2>>
        for (D1, D2, D3, D4, D5) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
        > PermuteShapeTo<(D2, D4, D3, D1, D5), Axes5<1, 3, 2, 0, 4>>
        for (D1, D2, D3, D4, D5) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
        > PermuteShapeTo<(D2, D4, D3, D5, D1), Axes5<1, 3, 2, 4, 0>>
        for (D1, D2, D3, D4, D5) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
        > PermuteShapeTo<(D2, D4, D5, D1, D3), Axes5<1, 3, 4, 0, 2>>
        for (D1, D2, D3, D4, D5) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
        > PermuteShapeTo<(D2, D4, D5, D3, D1), Axes5<1, 3, 4, 2, 0>>
        for (D1, D2, D3, D4, D5) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
        > PermuteShapeTo<(D2, D5, D1, D3, D4), Axes5<1, 4, 0, 2, 3>>
        for (D1, D2, D3, D4, D5) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
        > PermuteShapeTo<(D2, D5, D1, D4, D3), Axes5<1, 4, 0, 3, 2>>
        for (D1, D2, D3, D4, D5) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
        > PermuteShapeTo<(D2, D5, D3, D1, D4), Axes5<1, 4, 2, 0, 3>>
        for (D1, D2, D3, D4, D5) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
        > PermuteShapeTo<(D2, D5, D3, D4, D1), Axes5<1, 4, 2, 3, 0>>
        for (D1, D2, D3, D4, D5) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
        > PermuteShapeTo<(D2, D5, D4, D1, D3), Axes5<1, 4, 3, 0, 2>>
        for (D1, D2, D3, D4, D5) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
        > PermuteShapeTo<(D2, D5, D4, D3, D1), Axes5<1, 4, 3, 2, 0>>
        for (D1, D2, D3, D4, D5) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
        > PermuteShapeTo<(D3, D1, D2, D4, D5), Axes5<2, 0, 1, 3, 4>>
        for (D1, D2, D3, D4, D5) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
        > PermuteShapeTo<(D3, D1, D2, D5, D4), Axes5<2, 0, 1, 4, 3>>
        for (D1, D2, D3, D4, D5) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
        > PermuteShapeTo<(D3, D1, D4, D2, D5), Axes5<2, 0, 3, 1, 4>>
        for (D1, D2, D3, D4, D5) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
        > PermuteShapeTo<(D3, D1, D4, D5, D2), Axes5<2, 0, 3, 4, 1>>
        for (D1, D2, D3, D4, D5) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
        > PermuteShapeTo<(D3, D1, D5, D2, D4), Axes5<2, 0, 4, 1, 3>>
        for (D1, D2, D3, D4, D5) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
        > PermuteShapeTo<(D3, D1, D5, D4, D2), Axes5<2, 0, 4, 3, 1>>
        for (D1, D2, D3, D4, D5) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
        > PermuteShapeTo<(D3, D2, D1, D4, D5), Axes5<2, 1, 0, 3, 4>>
        for (D1, D2, D3, D4, D5) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
        > PermuteShapeTo<(D3, D2, D1, D5, D4), Axes5<2, 1, 0, 4, 3>>
        for (D1, D2, D3, D4, D5) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
        > PermuteShapeTo<(D3, D2, D4, D1, D5), Axes5<2, 1, 3, 0, 4>>
        for (D1, D2, D3, D4, D5) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
        > PermuteShapeTo<(D3, D2, D4, D5, D1), Axes5<2, 1, 3, 4, 0>>
        for (D1, D2, D3, D4, D5) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
        > PermuteShapeTo<(D3, D2, D5, D1, D4), Axes5<2, 1, 4, 0, 3>>
        for (D1, D2, D3, D4, D5) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
        > PermuteShapeTo<(D3, D2, D5, D4, D1), Axes5<2, 1, 4, 3, 0>>
        for (D1, D2, D3, D4, D5) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
        > PermuteShapeTo<(D3, D4, D1, D2, D5), Axes5<2, 3, 0, 1, 4>>
        for (D1, D2, D3, D4, D5) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
        > PermuteShapeTo<(D3, D4, D1, D5, D2), Axes5<2, 3, 0, 4, 1>>
        for (D1, D2, D3, D4, D5) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
        > PermuteShapeTo<(D3, D4, D2, D1, D5), Axes5<2, 3, 1, 0, 4>>
        for (D1, D2, D3, D4, D5) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
        > PermuteShapeTo<(D3, D4, D2, D5, D1), Axes5<2, 3, 1, 4, 0>>
        for (D1, D2, D3, D4, D5) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
        > PermuteShapeTo<(D3, D4, D5, D1, D2), Axes5<2, 3, 4, 0, 1>>
        for (D1, D2, D3, D4, D5) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
        > PermuteShapeTo<(D3, D4, D5, D2, D1), Axes5<2, 3, 4, 1, 0>>
        for (D1, D2, D3, D4, D5) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
        > PermuteShapeTo<(D3, D5, D1, D2, D4), Axes5<2, 4, 0, 1, 3>>
        for (D1, D2, D3, D4, D5) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
        > PermuteShapeTo<(D3, D5, D1, D4, D2), Axes5<2, 4, 0, 3, 1>>
        for (D1, D2, D3, D4, D5) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
        > PermuteShapeTo<(D3, D5, D2, D1, D4), Axes5<2, 4, 1, 0, 3>>
        for (D1, D2, D3, D4, D5) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
        > PermuteShapeTo<(D3, D5, D2, D4, D1), Axes5<2, 4, 1, 3, 0>>
        for (D1, D2, D3, D4, D5) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
        > PermuteShapeTo<(D3, D5, D4, D1, D2), Axes5<2, 4, 3, 0, 1>>
        for (D1, D2, D3, D4, D5) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
        > PermuteShapeTo<(D3, D5, D4, D2, D1), Axes5<2, 4, 3, 1, 0>>
        for (D1, D2, D3, D4, D5) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
        > PermuteShapeTo<(D4, D1, D2, D3, D5), Axes5<3, 0, 1, 2, 4>>
        for (D1, D2, D3, D4, D5) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
        > PermuteShapeTo<(D4, D1, D2, D5, D3), Axes5<3, 0, 1, 4, 2>>
        for (D1, D2, D3, D4, D5) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
        > PermuteShapeTo<(D4, D1, D3, D2, D5), Axes5<3, 0, 2, 1, 4>>
        for (D1, D2, D3, D4, D5) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
        > PermuteShapeTo<(D4, D1, D3, D5, D2), Axes5<3, 0, 2, 4, 1>>
        for (D1, D2, D3, D4, D5) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
        > PermuteShapeTo<(D4, D1, D5, D2, D3), Axes5<3, 0, 4, 1, 2>>
        for (D1, D2, D3, D4, D5) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
        > PermuteShapeTo<(D4, D1, D5, D3, D2), Axes5<3, 0, 4, 2, 1>>
        for (D1, D2, D3, D4, D5) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
        > PermuteShapeTo<(D4, D2, D1, D3, D5), Axes5<3, 1, 0, 2, 4>>
        for (D1, D2, D3, D4, D5) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
        > PermuteShapeTo<(D4, D2, D1, D5, D3), Axes5<3, 1, 0, 4, 2>>
        for (D1, D2, D3, D4, D5) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
        > PermuteShapeTo<(D4, D2, D3, D1, D5), Axes5<3, 1, 2, 0, 4>>
        for (D1, D2, D3, D4, D5) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
        > PermuteShapeTo<(D4, D2, D3, D5, D1), Axes5<3, 1, 2, 4, 0>>
        for (D1, D2, D3, D4, D5) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
        > PermuteShapeTo<(D4, D2, D5, D1, D3), Axes5<3, 1, 4, 0, 2>>
        for (D1, D2, D3, D4, D5) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
        > PermuteShapeTo<(D4, D2, D5, D3, D1), Axes5<3, 1, 4, 2, 0>>
        for (D1, D2, D3, D4, D5) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
        > PermuteShapeTo<(D4, D3, D1, D2, D5), Axes5<3, 2, 0, 1, 4>>
        for (D1, D2, D3, D4, D5) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
        > PermuteShapeTo<(D4, D3, D1, D5, D2), Axes5<3, 2, 0, 4, 1>>
        for (D1, D2, D3, D4, D5) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
        > PermuteShapeTo<(D4, D3, D2, D1, D5), Axes5<3, 2, 1, 0, 4>>
        for (D1, D2, D3, D4, D5) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
        > PermuteShapeTo<(D4, D3, D2, D5, D1), Axes5<3, 2, 1, 4, 0>>
        for (D1, D2, D3, D4, D5) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
        > PermuteShapeTo<(D4, D3, D5, D1, D2), Axes5<3, 2, 4, 0, 1>>
        for (D1, D2, D3, D4, D5) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
        > PermuteShapeTo<(D4, D3, D5, D2, D1), Axes5<3, 2, 4, 1, 0>>
        for (D1, D2, D3, D4, D5) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
        > PermuteShapeTo<(D4, D5, D1, D2, D3), Axes5<3, 4, 0, 1, 2>>
        for (D1, D2, D3, D4, D5) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
        > PermuteShapeTo<(D4, D5, D1, D3, D2), Axes5<3, 4, 0, 2, 1>>
        for (D1, D2, D3, D4, D5) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
        > PermuteShapeTo<(D4, D5, D2, D1, D3), Axes5<3, 4, 1, 0, 2>>
        for (D1, D2, D3, D4, D5) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
        > PermuteShapeTo<(D4, D5, D2, D3, D1), Axes5<3, 4, 1, 2, 0>>
        for (D1, D2, D3, D4, D5) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
        > PermuteShapeTo<(D4, D5, D3, D1, D2), Axes5<3, 4, 2, 0, 1>>
        for (D1, D2, D3, D4, D5) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
        > PermuteShapeTo<(D4, D5, D3, D2, D1), Axes5<3, 4, 2, 1, 0>>
        for (D1, D2, D3, D4, D5) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
        > PermuteShapeTo<(D5, D1, D2, D3, D4), Axes5<4, 0, 1, 2, 3>>
        for (D1, D2, D3, D4, D5) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
        > PermuteShapeTo<(D5, D1, D2, D4, D3), Axes5<4, 0, 1, 3, 2>>
        for (D1, D2, D3, D4, D5) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
        > PermuteShapeTo<(D5, D1, D3, D2, D4), Axes5<4, 0, 2, 1, 3>>
        for (D1, D2, D3, D4, D5) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
        > PermuteShapeTo<(D5, D1, D3, D4, D2), Axes5<4, 0, 2, 3, 1>>
        for (D1, D2, D3, D4, D5) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
        > PermuteShapeTo<(D5, D1, D4, D2, D3), Axes5<4, 0, 3, 1, 2>>
        for (D1, D2, D3, D4, D5) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
        > PermuteShapeTo<(D5, D1, D4, D3, D2), Axes5<4, 0, 3, 2, 1>>
        for (D1, D2, D3, D4, D5) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
        > PermuteShapeTo<(D5, D2, D1, D3, D4), Axes5<4, 1, 0, 2, 3>>
        for (D1, D2, D3, D4, D5) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
        > PermuteShapeTo<(D5, D2, D1, D4, D3), Axes5<4, 1, 0, 3, 2>>
        for (D1, D2, D3, D4, D5) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
        > PermuteShapeTo<(D5, D2, D3, D1, D4), Axes5<4, 1, 2, 0, 3>>
        for (D1, D2, D3, D4, D5) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
        > PermuteShapeTo<(D5, D2, D3, D4, D1), Axes5<4, 1, 2, 3, 0>>
        for (D1, D2, D3, D4, D5) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
        > PermuteShapeTo<(D5, D2, D4, D1, D3), Axes5<4, 1, 3, 0, 2>>
        for (D1, D2, D3, D4, D5) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
        > PermuteShapeTo<(D5, D2, D4, D3, D1), Axes5<4, 1, 3, 2, 0>>
        for (D1, D2, D3, D4, D5) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
        > PermuteShapeTo<(D5, D3, D1, D2, D4), Axes5<4, 2, 0, 1, 3>>
        for (D1, D2, D3, D4, D5) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
        > PermuteShapeTo<(D5, D3, D1, D4, D2), Axes5<4, 2, 0, 3, 1>>
        for (D1, D2, D3, D4, D5) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
        > PermuteShapeTo<(D5, D3, D2, D1, D4), Axes5<4, 2, 1, 0, 3>>
        for (D1, D2, D3, D4, D5) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
        > PermuteShapeTo<(D5, D3, D2, D4, D1), Axes5<4, 2, 1, 3, 0>>
        for (D1, D2, D3, D4, D5) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
        > PermuteShapeTo<(D5, D3, D4, D1, D2), Axes5<4, 2, 3, 0, 1>>
        for (D1, D2, D3, D4, D5) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
        > PermuteShapeTo<(D5, D3, D4, D2, D1), Axes5<4, 2, 3, 1, 0>>
        for (D1, D2, D3, D4, D5) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
        > PermuteShapeTo<(D5, D4, D1, D2, D3), Axes5<4, 3, 0, 1, 2>>
        for (D1, D2, D3, D4, D5) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
        > PermuteShapeTo<(D5, D4, D1, D3, D2), Axes5<4, 3, 0, 2, 1>>
        for (D1, D2, D3, D4, D5) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
        > PermuteShapeTo<(D5, D4, D2, D1, D3), Axes5<4, 3, 1, 0, 2>>
        for (D1, D2, D3, D4, D5) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
        > PermuteShapeTo<(D5, D4, D2, D3, D1), Axes5<4, 3, 1, 2, 0>>
        for (D1, D2, D3, D4, D5) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
        > PermuteShapeTo<(D5, D4, D3, D1, D2), Axes5<4, 3, 2, 0, 1>>
        for (D1, D2, D3, D4, D5) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
        > PermuteShapeTo<(D5, D4, D3, D2, D1), Axes5<4, 3, 2, 1, 0>>
        for (D1, D2, D3, D4, D5) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D1, D2, D3, D4, D5, D6), Axes6<0, 1, 2, 3, 4, 5>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D1, D2, D3, D4, D6, D5), Axes6<0, 1, 2, 3, 5, 4>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D1, D2, D3, D5, D4, D6), Axes6<0, 1, 2, 4, 3, 5>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D1, D2, D3, D5, D6, D4), Axes6<0, 1, 2, 4, 5, 3>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D1, D2, D3, D6, D4, D5), Axes6<0, 1, 2, 5, 3, 4>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D1, D2, D3, D6, D5, D4), Axes6<0, 1, 2, 5, 4, 3>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D1, D2, D4, D3, D5, D6), Axes6<0, 1, 3, 2, 4, 5>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D1, D2, D4, D3, D6, D5), Axes6<0, 1, 3, 2, 5, 4>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D1, D2, D4, D5, D3, D6), Axes6<0, 1, 3, 4, 2, 5>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D1, D2, D4, D5, D6, D3), Axes6<0, 1, 3, 4, 5, 2>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D1, D2, D4, D6, D3, D5), Axes6<0, 1, 3, 5, 2, 4>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D1, D2, D4, D6, D5, D3), Axes6<0, 1, 3, 5, 4, 2>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D1, D2, D5, D3, D4, D6), Axes6<0, 1, 4, 2, 3, 5>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D1, D2, D5, D3, D6, D4), Axes6<0, 1, 4, 2, 5, 3>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D1, D2, D5, D4, D3, D6), Axes6<0, 1, 4, 3, 2, 5>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D1, D2, D5, D4, D6, D3), Axes6<0, 1, 4, 3, 5, 2>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D1, D2, D5, D6, D3, D4), Axes6<0, 1, 4, 5, 2, 3>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D1, D2, D5, D6, D4, D3), Axes6<0, 1, 4, 5, 3, 2>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D1, D2, D6, D3, D4, D5), Axes6<0, 1, 5, 2, 3, 4>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D1, D2, D6, D3, D5, D4), Axes6<0, 1, 5, 2, 4, 3>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D1, D2, D6, D4, D3, D5), Axes6<0, 1, 5, 3, 2, 4>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D1, D2, D6, D4, D5, D3), Axes6<0, 1, 5, 3, 4, 2>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D1, D2, D6, D5, D3, D4), Axes6<0, 1, 5, 4, 2, 3>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D1, D2, D6, D5, D4, D3), Axes6<0, 1, 5, 4, 3, 2>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D1, D3, D2, D4, D5, D6), Axes6<0, 2, 1, 3, 4, 5>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D1, D3, D2, D4, D6, D5), Axes6<0, 2, 1, 3, 5, 4>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D1, D3, D2, D5, D4, D6), Axes6<0, 2, 1, 4, 3, 5>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D1, D3, D2, D5, D6, D4), Axes6<0, 2, 1, 4, 5, 3>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D1, D3, D2, D6, D4, D5), Axes6<0, 2, 1, 5, 3, 4>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D1, D3, D2, D6, D5, D4), Axes6<0, 2, 1, 5, 4, 3>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D1, D3, D4, D2, D5, D6), Axes6<0, 2, 3, 1, 4, 5>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D1, D3, D4, D2, D6, D5), Axes6<0, 2, 3, 1, 5, 4>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D1, D3, D4, D5, D2, D6), Axes6<0, 2, 3, 4, 1, 5>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D1, D3, D4, D5, D6, D2), Axes6<0, 2, 3, 4, 5, 1>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D1, D3, D4, D6, D2, D5), Axes6<0, 2, 3, 5, 1, 4>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D1, D3, D4, D6, D5, D2), Axes6<0, 2, 3, 5, 4, 1>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D1, D3, D5, D2, D4, D6), Axes6<0, 2, 4, 1, 3, 5>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D1, D3, D5, D2, D6, D4), Axes6<0, 2, 4, 1, 5, 3>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D1, D3, D5, D4, D2, D6), Axes6<0, 2, 4, 3, 1, 5>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D1, D3, D5, D4, D6, D2), Axes6<0, 2, 4, 3, 5, 1>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D1, D3, D5, D6, D2, D4), Axes6<0, 2, 4, 5, 1, 3>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D1, D3, D5, D6, D4, D2), Axes6<0, 2, 4, 5, 3, 1>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D1, D3, D6, D2, D4, D5), Axes6<0, 2, 5, 1, 3, 4>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D1, D3, D6, D2, D5, D4), Axes6<0, 2, 5, 1, 4, 3>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D1, D3, D6, D4, D2, D5), Axes6<0, 2, 5, 3, 1, 4>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D1, D3, D6, D4, D5, D2), Axes6<0, 2, 5, 3, 4, 1>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D1, D3, D6, D5, D2, D4), Axes6<0, 2, 5, 4, 1, 3>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D1, D3, D6, D5, D4, D2), Axes6<0, 2, 5, 4, 3, 1>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D1, D4, D2, D3, D5, D6), Axes6<0, 3, 1, 2, 4, 5>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D1, D4, D2, D3, D6, D5), Axes6<0, 3, 1, 2, 5, 4>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D1, D4, D2, D5, D3, D6), Axes6<0, 3, 1, 4, 2, 5>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D1, D4, D2, D5, D6, D3), Axes6<0, 3, 1, 4, 5, 2>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D1, D4, D2, D6, D3, D5), Axes6<0, 3, 1, 5, 2, 4>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D1, D4, D2, D6, D5, D3), Axes6<0, 3, 1, 5, 4, 2>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D1, D4, D3, D2, D5, D6), Axes6<0, 3, 2, 1, 4, 5>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D1, D4, D3, D2, D6, D5), Axes6<0, 3, 2, 1, 5, 4>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D1, D4, D3, D5, D2, D6), Axes6<0, 3, 2, 4, 1, 5>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D1, D4, D3, D5, D6, D2), Axes6<0, 3, 2, 4, 5, 1>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D1, D4, D3, D6, D2, D5), Axes6<0, 3, 2, 5, 1, 4>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D1, D4, D3, D6, D5, D2), Axes6<0, 3, 2, 5, 4, 1>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D1, D4, D5, D2, D3, D6), Axes6<0, 3, 4, 1, 2, 5>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D1, D4, D5, D2, D6, D3), Axes6<0, 3, 4, 1, 5, 2>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D1, D4, D5, D3, D2, D6), Axes6<0, 3, 4, 2, 1, 5>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D1, D4, D5, D3, D6, D2), Axes6<0, 3, 4, 2, 5, 1>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D1, D4, D5, D6, D2, D3), Axes6<0, 3, 4, 5, 1, 2>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D1, D4, D5, D6, D3, D2), Axes6<0, 3, 4, 5, 2, 1>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D1, D4, D6, D2, D3, D5), Axes6<0, 3, 5, 1, 2, 4>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D1, D4, D6, D2, D5, D3), Axes6<0, 3, 5, 1, 4, 2>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D1, D4, D6, D3, D2, D5), Axes6<0, 3, 5, 2, 1, 4>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D1, D4, D6, D3, D5, D2), Axes6<0, 3, 5, 2, 4, 1>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D1, D4, D6, D5, D2, D3), Axes6<0, 3, 5, 4, 1, 2>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D1, D4, D6, D5, D3, D2), Axes6<0, 3, 5, 4, 2, 1>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D1, D5, D2, D3, D4, D6), Axes6<0, 4, 1, 2, 3, 5>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D1, D5, D2, D3, D6, D4), Axes6<0, 4, 1, 2, 5, 3>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D1, D5, D2, D4, D3, D6), Axes6<0, 4, 1, 3, 2, 5>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D1, D5, D2, D4, D6, D3), Axes6<0, 4, 1, 3, 5, 2>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D1, D5, D2, D6, D3, D4), Axes6<0, 4, 1, 5, 2, 3>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D1, D5, D2, D6, D4, D3), Axes6<0, 4, 1, 5, 3, 2>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D1, D5, D3, D2, D4, D6), Axes6<0, 4, 2, 1, 3, 5>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D1, D5, D3, D2, D6, D4), Axes6<0, 4, 2, 1, 5, 3>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D1, D5, D3, D4, D2, D6), Axes6<0, 4, 2, 3, 1, 5>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D1, D5, D3, D4, D6, D2), Axes6<0, 4, 2, 3, 5, 1>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D1, D5, D3, D6, D2, D4), Axes6<0, 4, 2, 5, 1, 3>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D1, D5, D3, D6, D4, D2), Axes6<0, 4, 2, 5, 3, 1>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D1, D5, D4, D2, D3, D6), Axes6<0, 4, 3, 1, 2, 5>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D1, D5, D4, D2, D6, D3), Axes6<0, 4, 3, 1, 5, 2>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D1, D5, D4, D3, D2, D6), Axes6<0, 4, 3, 2, 1, 5>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D1, D5, D4, D3, D6, D2), Axes6<0, 4, 3, 2, 5, 1>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D1, D5, D4, D6, D2, D3), Axes6<0, 4, 3, 5, 1, 2>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D1, D5, D4, D6, D3, D2), Axes6<0, 4, 3, 5, 2, 1>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D1, D5, D6, D2, D3, D4), Axes6<0, 4, 5, 1, 2, 3>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D1, D5, D6, D2, D4, D3), Axes6<0, 4, 5, 1, 3, 2>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D1, D5, D6, D3, D2, D4), Axes6<0, 4, 5, 2, 1, 3>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D1, D5, D6, D3, D4, D2), Axes6<0, 4, 5, 2, 3, 1>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D1, D5, D6, D4, D2, D3), Axes6<0, 4, 5, 3, 1, 2>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D1, D5, D6, D4, D3, D2), Axes6<0, 4, 5, 3, 2, 1>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D1, D6, D2, D3, D4, D5), Axes6<0, 5, 1, 2, 3, 4>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D1, D6, D2, D3, D5, D4), Axes6<0, 5, 1, 2, 4, 3>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D1, D6, D2, D4, D3, D5), Axes6<0, 5, 1, 3, 2, 4>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D1, D6, D2, D4, D5, D3), Axes6<0, 5, 1, 3, 4, 2>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D1, D6, D2, D5, D3, D4), Axes6<0, 5, 1, 4, 2, 3>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D1, D6, D2, D5, D4, D3), Axes6<0, 5, 1, 4, 3, 2>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D1, D6, D3, D2, D4, D5), Axes6<0, 5, 2, 1, 3, 4>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D1, D6, D3, D2, D5, D4), Axes6<0, 5, 2, 1, 4, 3>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D1, D6, D3, D4, D2, D5), Axes6<0, 5, 2, 3, 1, 4>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D1, D6, D3, D4, D5, D2), Axes6<0, 5, 2, 3, 4, 1>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D1, D6, D3, D5, D2, D4), Axes6<0, 5, 2, 4, 1, 3>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D1, D6, D3, D5, D4, D2), Axes6<0, 5, 2, 4, 3, 1>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D1, D6, D4, D2, D3, D5), Axes6<0, 5, 3, 1, 2, 4>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D1, D6, D4, D2, D5, D3), Axes6<0, 5, 3, 1, 4, 2>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D1, D6, D4, D3, D2, D5), Axes6<0, 5, 3, 2, 1, 4>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D1, D6, D4, D3, D5, D2), Axes6<0, 5, 3, 2, 4, 1>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D1, D6, D4, D5, D2, D3), Axes6<0, 5, 3, 4, 1, 2>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D1, D6, D4, D5, D3, D2), Axes6<0, 5, 3, 4, 2, 1>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D1, D6, D5, D2, D3, D4), Axes6<0, 5, 4, 1, 2, 3>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D1, D6, D5, D2, D4, D3), Axes6<0, 5, 4, 1, 3, 2>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D1, D6, D5, D3, D2, D4), Axes6<0, 5, 4, 2, 1, 3>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D1, D6, D5, D3, D4, D2), Axes6<0, 5, 4, 2, 3, 1>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D1, D6, D5, D4, D2, D3), Axes6<0, 5, 4, 3, 1, 2>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D1, D6, D5, D4, D3, D2), Axes6<0, 5, 4, 3, 2, 1>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D2, D1, D3, D4, D5, D6), Axes6<1, 0, 2, 3, 4, 5>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D2, D1, D3, D4, D6, D5), Axes6<1, 0, 2, 3, 5, 4>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D2, D1, D3, D5, D4, D6), Axes6<1, 0, 2, 4, 3, 5>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D2, D1, D3, D5, D6, D4), Axes6<1, 0, 2, 4, 5, 3>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D2, D1, D3, D6, D4, D5), Axes6<1, 0, 2, 5, 3, 4>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D2, D1, D3, D6, D5, D4), Axes6<1, 0, 2, 5, 4, 3>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D2, D1, D4, D3, D5, D6), Axes6<1, 0, 3, 2, 4, 5>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D2, D1, D4, D3, D6, D5), Axes6<1, 0, 3, 2, 5, 4>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D2, D1, D4, D5, D3, D6), Axes6<1, 0, 3, 4, 2, 5>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D2, D1, D4, D5, D6, D3), Axes6<1, 0, 3, 4, 5, 2>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D2, D1, D4, D6, D3, D5), Axes6<1, 0, 3, 5, 2, 4>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D2, D1, D4, D6, D5, D3), Axes6<1, 0, 3, 5, 4, 2>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D2, D1, D5, D3, D4, D6), Axes6<1, 0, 4, 2, 3, 5>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D2, D1, D5, D3, D6, D4), Axes6<1, 0, 4, 2, 5, 3>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D2, D1, D5, D4, D3, D6), Axes6<1, 0, 4, 3, 2, 5>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D2, D1, D5, D4, D6, D3), Axes6<1, 0, 4, 3, 5, 2>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D2, D1, D5, D6, D3, D4), Axes6<1, 0, 4, 5, 2, 3>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D2, D1, D5, D6, D4, D3), Axes6<1, 0, 4, 5, 3, 2>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D2, D1, D6, D3, D4, D5), Axes6<1, 0, 5, 2, 3, 4>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D2, D1, D6, D3, D5, D4), Axes6<1, 0, 5, 2, 4, 3>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D2, D1, D6, D4, D3, D5), Axes6<1, 0, 5, 3, 2, 4>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D2, D1, D6, D4, D5, D3), Axes6<1, 0, 5, 3, 4, 2>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D2, D1, D6, D5, D3, D4), Axes6<1, 0, 5, 4, 2, 3>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D2, D1, D6, D5, D4, D3), Axes6<1, 0, 5, 4, 3, 2>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D2, D3, D1, D4, D5, D6), Axes6<1, 2, 0, 3, 4, 5>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D2, D3, D1, D4, D6, D5), Axes6<1, 2, 0, 3, 5, 4>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D2, D3, D1, D5, D4, D6), Axes6<1, 2, 0, 4, 3, 5>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D2, D3, D1, D5, D6, D4), Axes6<1, 2, 0, 4, 5, 3>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D2, D3, D1, D6, D4, D5), Axes6<1, 2, 0, 5, 3, 4>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D2, D3, D1, D6, D5, D4), Axes6<1, 2, 0, 5, 4, 3>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D2, D3, D4, D1, D5, D6), Axes6<1, 2, 3, 0, 4, 5>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D2, D3, D4, D1, D6, D5), Axes6<1, 2, 3, 0, 5, 4>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D2, D3, D4, D5, D1, D6), Axes6<1, 2, 3, 4, 0, 5>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D2, D3, D4, D5, D6, D1), Axes6<1, 2, 3, 4, 5, 0>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D2, D3, D4, D6, D1, D5), Axes6<1, 2, 3, 5, 0, 4>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D2, D3, D4, D6, D5, D1), Axes6<1, 2, 3, 5, 4, 0>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D2, D3, D5, D1, D4, D6), Axes6<1, 2, 4, 0, 3, 5>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D2, D3, D5, D1, D6, D4), Axes6<1, 2, 4, 0, 5, 3>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D2, D3, D5, D4, D1, D6), Axes6<1, 2, 4, 3, 0, 5>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D2, D3, D5, D4, D6, D1), Axes6<1, 2, 4, 3, 5, 0>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D2, D3, D5, D6, D1, D4), Axes6<1, 2, 4, 5, 0, 3>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D2, D3, D5, D6, D4, D1), Axes6<1, 2, 4, 5, 3, 0>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D2, D3, D6, D1, D4, D5), Axes6<1, 2, 5, 0, 3, 4>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D2, D3, D6, D1, D5, D4), Axes6<1, 2, 5, 0, 4, 3>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D2, D3, D6, D4, D1, D5), Axes6<1, 2, 5, 3, 0, 4>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D2, D3, D6, D4, D5, D1), Axes6<1, 2, 5, 3, 4, 0>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D2, D3, D6, D5, D1, D4), Axes6<1, 2, 5, 4, 0, 3>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D2, D3, D6, D5, D4, D1), Axes6<1, 2, 5, 4, 3, 0>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D2, D4, D1, D3, D5, D6), Axes6<1, 3, 0, 2, 4, 5>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D2, D4, D1, D3, D6, D5), Axes6<1, 3, 0, 2, 5, 4>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D2, D4, D1, D5, D3, D6), Axes6<1, 3, 0, 4, 2, 5>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D2, D4, D1, D5, D6, D3), Axes6<1, 3, 0, 4, 5, 2>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D2, D4, D1, D6, D3, D5), Axes6<1, 3, 0, 5, 2, 4>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D2, D4, D1, D6, D5, D3), Axes6<1, 3, 0, 5, 4, 2>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D2, D4, D3, D1, D5, D6), Axes6<1, 3, 2, 0, 4, 5>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D2, D4, D3, D1, D6, D5), Axes6<1, 3, 2, 0, 5, 4>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D2, D4, D3, D5, D1, D6), Axes6<1, 3, 2, 4, 0, 5>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D2, D4, D3, D5, D6, D1), Axes6<1, 3, 2, 4, 5, 0>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D2, D4, D3, D6, D1, D5), Axes6<1, 3, 2, 5, 0, 4>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D2, D4, D3, D6, D5, D1), Axes6<1, 3, 2, 5, 4, 0>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D2, D4, D5, D1, D3, D6), Axes6<1, 3, 4, 0, 2, 5>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D2, D4, D5, D1, D6, D3), Axes6<1, 3, 4, 0, 5, 2>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D2, D4, D5, D3, D1, D6), Axes6<1, 3, 4, 2, 0, 5>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D2, D4, D5, D3, D6, D1), Axes6<1, 3, 4, 2, 5, 0>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D2, D4, D5, D6, D1, D3), Axes6<1, 3, 4, 5, 0, 2>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D2, D4, D5, D6, D3, D1), Axes6<1, 3, 4, 5, 2, 0>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D2, D4, D6, D1, D3, D5), Axes6<1, 3, 5, 0, 2, 4>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D2, D4, D6, D1, D5, D3), Axes6<1, 3, 5, 0, 4, 2>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D2, D4, D6, D3, D1, D5), Axes6<1, 3, 5, 2, 0, 4>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D2, D4, D6, D3, D5, D1), Axes6<1, 3, 5, 2, 4, 0>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D2, D4, D6, D5, D1, D3), Axes6<1, 3, 5, 4, 0, 2>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D2, D4, D6, D5, D3, D1), Axes6<1, 3, 5, 4, 2, 0>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D2, D5, D1, D3, D4, D6), Axes6<1, 4, 0, 2, 3, 5>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D2, D5, D1, D3, D6, D4), Axes6<1, 4, 0, 2, 5, 3>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D2, D5, D1, D4, D3, D6), Axes6<1, 4, 0, 3, 2, 5>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D2, D5, D1, D4, D6, D3), Axes6<1, 4, 0, 3, 5, 2>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D2, D5, D1, D6, D3, D4), Axes6<1, 4, 0, 5, 2, 3>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D2, D5, D1, D6, D4, D3), Axes6<1, 4, 0, 5, 3, 2>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D2, D5, D3, D1, D4, D6), Axes6<1, 4, 2, 0, 3, 5>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D2, D5, D3, D1, D6, D4), Axes6<1, 4, 2, 0, 5, 3>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D2, D5, D3, D4, D1, D6), Axes6<1, 4, 2, 3, 0, 5>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D2, D5, D3, D4, D6, D1), Axes6<1, 4, 2, 3, 5, 0>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D2, D5, D3, D6, D1, D4), Axes6<1, 4, 2, 5, 0, 3>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D2, D5, D3, D6, D4, D1), Axes6<1, 4, 2, 5, 3, 0>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D2, D5, D4, D1, D3, D6), Axes6<1, 4, 3, 0, 2, 5>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D2, D5, D4, D1, D6, D3), Axes6<1, 4, 3, 0, 5, 2>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D2, D5, D4, D3, D1, D6), Axes6<1, 4, 3, 2, 0, 5>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D2, D5, D4, D3, D6, D1), Axes6<1, 4, 3, 2, 5, 0>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D2, D5, D4, D6, D1, D3), Axes6<1, 4, 3, 5, 0, 2>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D2, D5, D4, D6, D3, D1), Axes6<1, 4, 3, 5, 2, 0>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D2, D5, D6, D1, D3, D4), Axes6<1, 4, 5, 0, 2, 3>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D2, D5, D6, D1, D4, D3), Axes6<1, 4, 5, 0, 3, 2>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D2, D5, D6, D3, D1, D4), Axes6<1, 4, 5, 2, 0, 3>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D2, D5, D6, D3, D4, D1), Axes6<1, 4, 5, 2, 3, 0>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D2, D5, D6, D4, D1, D3), Axes6<1, 4, 5, 3, 0, 2>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D2, D5, D6, D4, D3, D1), Axes6<1, 4, 5, 3, 2, 0>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D2, D6, D1, D3, D4, D5), Axes6<1, 5, 0, 2, 3, 4>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D2, D6, D1, D3, D5, D4), Axes6<1, 5, 0, 2, 4, 3>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D2, D6, D1, D4, D3, D5), Axes6<1, 5, 0, 3, 2, 4>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D2, D6, D1, D4, D5, D3), Axes6<1, 5, 0, 3, 4, 2>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D2, D6, D1, D5, D3, D4), Axes6<1, 5, 0, 4, 2, 3>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D2, D6, D1, D5, D4, D3), Axes6<1, 5, 0, 4, 3, 2>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D2, D6, D3, D1, D4, D5), Axes6<1, 5, 2, 0, 3, 4>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D2, D6, D3, D1, D5, D4), Axes6<1, 5, 2, 0, 4, 3>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D2, D6, D3, D4, D1, D5), Axes6<1, 5, 2, 3, 0, 4>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D2, D6, D3, D4, D5, D1), Axes6<1, 5, 2, 3, 4, 0>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D2, D6, D3, D5, D1, D4), Axes6<1, 5, 2, 4, 0, 3>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D2, D6, D3, D5, D4, D1), Axes6<1, 5, 2, 4, 3, 0>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D2, D6, D4, D1, D3, D5), Axes6<1, 5, 3, 0, 2, 4>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D2, D6, D4, D1, D5, D3), Axes6<1, 5, 3, 0, 4, 2>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D2, D6, D4, D3, D1, D5), Axes6<1, 5, 3, 2, 0, 4>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D2, D6, D4, D3, D5, D1), Axes6<1, 5, 3, 2, 4, 0>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D2, D6, D4, D5, D1, D3), Axes6<1, 5, 3, 4, 0, 2>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D2, D6, D4, D5, D3, D1), Axes6<1, 5, 3, 4, 2, 0>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D2, D6, D5, D1, D3, D4), Axes6<1, 5, 4, 0, 2, 3>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D2, D6, D5, D1, D4, D3), Axes6<1, 5, 4, 0, 3, 2>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D2, D6, D5, D3, D1, D4), Axes6<1, 5, 4, 2, 0, 3>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D2, D6, D5, D3, D4, D1), Axes6<1, 5, 4, 2, 3, 0>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D2, D6, D5, D4, D1, D3), Axes6<1, 5, 4, 3, 0, 2>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D2, D6, D5, D4, D3, D1), Axes6<1, 5, 4, 3, 2, 0>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D3, D1, D2, D4, D5, D6), Axes6<2, 0, 1, 3, 4, 5>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D3, D1, D2, D4, D6, D5), Axes6<2, 0, 1, 3, 5, 4>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D3, D1, D2, D5, D4, D6), Axes6<2, 0, 1, 4, 3, 5>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D3, D1, D2, D5, D6, D4), Axes6<2, 0, 1, 4, 5, 3>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D3, D1, D2, D6, D4, D5), Axes6<2, 0, 1, 5, 3, 4>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D3, D1, D2, D6, D5, D4), Axes6<2, 0, 1, 5, 4, 3>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D3, D1, D4, D2, D5, D6), Axes6<2, 0, 3, 1, 4, 5>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D3, D1, D4, D2, D6, D5), Axes6<2, 0, 3, 1, 5, 4>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D3, D1, D4, D5, D2, D6), Axes6<2, 0, 3, 4, 1, 5>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D3, D1, D4, D5, D6, D2), Axes6<2, 0, 3, 4, 5, 1>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D3, D1, D4, D6, D2, D5), Axes6<2, 0, 3, 5, 1, 4>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D3, D1, D4, D6, D5, D2), Axes6<2, 0, 3, 5, 4, 1>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D3, D1, D5, D2, D4, D6), Axes6<2, 0, 4, 1, 3, 5>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D3, D1, D5, D2, D6, D4), Axes6<2, 0, 4, 1, 5, 3>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D3, D1, D5, D4, D2, D6), Axes6<2, 0, 4, 3, 1, 5>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D3, D1, D5, D4, D6, D2), Axes6<2, 0, 4, 3, 5, 1>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D3, D1, D5, D6, D2, D4), Axes6<2, 0, 4, 5, 1, 3>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D3, D1, D5, D6, D4, D2), Axes6<2, 0, 4, 5, 3, 1>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D3, D1, D6, D2, D4, D5), Axes6<2, 0, 5, 1, 3, 4>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D3, D1, D6, D2, D5, D4), Axes6<2, 0, 5, 1, 4, 3>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D3, D1, D6, D4, D2, D5), Axes6<2, 0, 5, 3, 1, 4>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D3, D1, D6, D4, D5, D2), Axes6<2, 0, 5, 3, 4, 1>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D3, D1, D6, D5, D2, D4), Axes6<2, 0, 5, 4, 1, 3>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D3, D1, D6, D5, D4, D2), Axes6<2, 0, 5, 4, 3, 1>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D3, D2, D1, D4, D5, D6), Axes6<2, 1, 0, 3, 4, 5>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D3, D2, D1, D4, D6, D5), Axes6<2, 1, 0, 3, 5, 4>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D3, D2, D1, D5, D4, D6), Axes6<2, 1, 0, 4, 3, 5>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D3, D2, D1, D5, D6, D4), Axes6<2, 1, 0, 4, 5, 3>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D3, D2, D1, D6, D4, D5), Axes6<2, 1, 0, 5, 3, 4>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D3, D2, D1, D6, D5, D4), Axes6<2, 1, 0, 5, 4, 3>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D3, D2, D4, D1, D5, D6), Axes6<2, 1, 3, 0, 4, 5>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D3, D2, D4, D1, D6, D5), Axes6<2, 1, 3, 0, 5, 4>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D3, D2, D4, D5, D1, D6), Axes6<2, 1, 3, 4, 0, 5>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D3, D2, D4, D5, D6, D1), Axes6<2, 1, 3, 4, 5, 0>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D3, D2, D4, D6, D1, D5), Axes6<2, 1, 3, 5, 0, 4>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D3, D2, D4, D6, D5, D1), Axes6<2, 1, 3, 5, 4, 0>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D3, D2, D5, D1, D4, D6), Axes6<2, 1, 4, 0, 3, 5>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D3, D2, D5, D1, D6, D4), Axes6<2, 1, 4, 0, 5, 3>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D3, D2, D5, D4, D1, D6), Axes6<2, 1, 4, 3, 0, 5>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D3, D2, D5, D4, D6, D1), Axes6<2, 1, 4, 3, 5, 0>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D3, D2, D5, D6, D1, D4), Axes6<2, 1, 4, 5, 0, 3>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D3, D2, D5, D6, D4, D1), Axes6<2, 1, 4, 5, 3, 0>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D3, D2, D6, D1, D4, D5), Axes6<2, 1, 5, 0, 3, 4>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D3, D2, D6, D1, D5, D4), Axes6<2, 1, 5, 0, 4, 3>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D3, D2, D6, D4, D1, D5), Axes6<2, 1, 5, 3, 0, 4>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D3, D2, D6, D4, D5, D1), Axes6<2, 1, 5, 3, 4, 0>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D3, D2, D6, D5, D1, D4), Axes6<2, 1, 5, 4, 0, 3>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D3, D2, D6, D5, D4, D1), Axes6<2, 1, 5, 4, 3, 0>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D3, D4, D1, D2, D5, D6), Axes6<2, 3, 0, 1, 4, 5>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D3, D4, D1, D2, D6, D5), Axes6<2, 3, 0, 1, 5, 4>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D3, D4, D1, D5, D2, D6), Axes6<2, 3, 0, 4, 1, 5>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D3, D4, D1, D5, D6, D2), Axes6<2, 3, 0, 4, 5, 1>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D3, D4, D1, D6, D2, D5), Axes6<2, 3, 0, 5, 1, 4>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D3, D4, D1, D6, D5, D2), Axes6<2, 3, 0, 5, 4, 1>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D3, D4, D2, D1, D5, D6), Axes6<2, 3, 1, 0, 4, 5>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D3, D4, D2, D1, D6, D5), Axes6<2, 3, 1, 0, 5, 4>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D3, D4, D2, D5, D1, D6), Axes6<2, 3, 1, 4, 0, 5>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D3, D4, D2, D5, D6, D1), Axes6<2, 3, 1, 4, 5, 0>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D3, D4, D2, D6, D1, D5), Axes6<2, 3, 1, 5, 0, 4>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D3, D4, D2, D6, D5, D1), Axes6<2, 3, 1, 5, 4, 0>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D3, D4, D5, D1, D2, D6), Axes6<2, 3, 4, 0, 1, 5>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D3, D4, D5, D1, D6, D2), Axes6<2, 3, 4, 0, 5, 1>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D3, D4, D5, D2, D1, D6), Axes6<2, 3, 4, 1, 0, 5>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D3, D4, D5, D2, D6, D1), Axes6<2, 3, 4, 1, 5, 0>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D3, D4, D5, D6, D1, D2), Axes6<2, 3, 4, 5, 0, 1>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D3, D4, D5, D6, D2, D1), Axes6<2, 3, 4, 5, 1, 0>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D3, D4, D6, D1, D2, D5), Axes6<2, 3, 5, 0, 1, 4>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D3, D4, D6, D1, D5, D2), Axes6<2, 3, 5, 0, 4, 1>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D3, D4, D6, D2, D1, D5), Axes6<2, 3, 5, 1, 0, 4>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D3, D4, D6, D2, D5, D1), Axes6<2, 3, 5, 1, 4, 0>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D3, D4, D6, D5, D1, D2), Axes6<2, 3, 5, 4, 0, 1>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D3, D4, D6, D5, D2, D1), Axes6<2, 3, 5, 4, 1, 0>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D3, D5, D1, D2, D4, D6), Axes6<2, 4, 0, 1, 3, 5>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D3, D5, D1, D2, D6, D4), Axes6<2, 4, 0, 1, 5, 3>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D3, D5, D1, D4, D2, D6), Axes6<2, 4, 0, 3, 1, 5>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D3, D5, D1, D4, D6, D2), Axes6<2, 4, 0, 3, 5, 1>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D3, D5, D1, D6, D2, D4), Axes6<2, 4, 0, 5, 1, 3>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D3, D5, D1, D6, D4, D2), Axes6<2, 4, 0, 5, 3, 1>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D3, D5, D2, D1, D4, D6), Axes6<2, 4, 1, 0, 3, 5>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D3, D5, D2, D1, D6, D4), Axes6<2, 4, 1, 0, 5, 3>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D3, D5, D2, D4, D1, D6), Axes6<2, 4, 1, 3, 0, 5>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D3, D5, D2, D4, D6, D1), Axes6<2, 4, 1, 3, 5, 0>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D3, D5, D2, D6, D1, D4), Axes6<2, 4, 1, 5, 0, 3>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D3, D5, D2, D6, D4, D1), Axes6<2, 4, 1, 5, 3, 0>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D3, D5, D4, D1, D2, D6), Axes6<2, 4, 3, 0, 1, 5>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D3, D5, D4, D1, D6, D2), Axes6<2, 4, 3, 0, 5, 1>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D3, D5, D4, D2, D1, D6), Axes6<2, 4, 3, 1, 0, 5>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D3, D5, D4, D2, D6, D1), Axes6<2, 4, 3, 1, 5, 0>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D3, D5, D4, D6, D1, D2), Axes6<2, 4, 3, 5, 0, 1>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D3, D5, D4, D6, D2, D1), Axes6<2, 4, 3, 5, 1, 0>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D3, D5, D6, D1, D2, D4), Axes6<2, 4, 5, 0, 1, 3>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D3, D5, D6, D1, D4, D2), Axes6<2, 4, 5, 0, 3, 1>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D3, D5, D6, D2, D1, D4), Axes6<2, 4, 5, 1, 0, 3>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D3, D5, D6, D2, D4, D1), Axes6<2, 4, 5, 1, 3, 0>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D3, D5, D6, D4, D1, D2), Axes6<2, 4, 5, 3, 0, 1>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D3, D5, D6, D4, D2, D1), Axes6<2, 4, 5, 3, 1, 0>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D3, D6, D1, D2, D4, D5), Axes6<2, 5, 0, 1, 3, 4>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D3, D6, D1, D2, D5, D4), Axes6<2, 5, 0, 1, 4, 3>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D3, D6, D1, D4, D2, D5), Axes6<2, 5, 0, 3, 1, 4>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D3, D6, D1, D4, D5, D2), Axes6<2, 5, 0, 3, 4, 1>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D3, D6, D1, D5, D2, D4), Axes6<2, 5, 0, 4, 1, 3>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D3, D6, D1, D5, D4, D2), Axes6<2, 5, 0, 4, 3, 1>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D3, D6, D2, D1, D4, D5), Axes6<2, 5, 1, 0, 3, 4>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D3, D6, D2, D1, D5, D4), Axes6<2, 5, 1, 0, 4, 3>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D3, D6, D2, D4, D1, D5), Axes6<2, 5, 1, 3, 0, 4>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D3, D6, D2, D4, D5, D1), Axes6<2, 5, 1, 3, 4, 0>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D3, D6, D2, D5, D1, D4), Axes6<2, 5, 1, 4, 0, 3>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D3, D6, D2, D5, D4, D1), Axes6<2, 5, 1, 4, 3, 0>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D3, D6, D4, D1, D2, D5), Axes6<2, 5, 3, 0, 1, 4>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D3, D6, D4, D1, D5, D2), Axes6<2, 5, 3, 0, 4, 1>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D3, D6, D4, D2, D1, D5), Axes6<2, 5, 3, 1, 0, 4>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D3, D6, D4, D2, D5, D1), Axes6<2, 5, 3, 1, 4, 0>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D3, D6, D4, D5, D1, D2), Axes6<2, 5, 3, 4, 0, 1>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D3, D6, D4, D5, D2, D1), Axes6<2, 5, 3, 4, 1, 0>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D3, D6, D5, D1, D2, D4), Axes6<2, 5, 4, 0, 1, 3>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D3, D6, D5, D1, D4, D2), Axes6<2, 5, 4, 0, 3, 1>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D3, D6, D5, D2, D1, D4), Axes6<2, 5, 4, 1, 0, 3>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D3, D6, D5, D2, D4, D1), Axes6<2, 5, 4, 1, 3, 0>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D3, D6, D5, D4, D1, D2), Axes6<2, 5, 4, 3, 0, 1>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D3, D6, D5, D4, D2, D1), Axes6<2, 5, 4, 3, 1, 0>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D4, D1, D2, D3, D5, D6), Axes6<3, 0, 1, 2, 4, 5>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D4, D1, D2, D3, D6, D5), Axes6<3, 0, 1, 2, 5, 4>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D4, D1, D2, D5, D3, D6), Axes6<3, 0, 1, 4, 2, 5>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D4, D1, D2, D5, D6, D3), Axes6<3, 0, 1, 4, 5, 2>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D4, D1, D2, D6, D3, D5), Axes6<3, 0, 1, 5, 2, 4>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D4, D1, D2, D6, D5, D3), Axes6<3, 0, 1, 5, 4, 2>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D4, D1, D3, D2, D5, D6), Axes6<3, 0, 2, 1, 4, 5>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D4, D1, D3, D2, D6, D5), Axes6<3, 0, 2, 1, 5, 4>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D4, D1, D3, D5, D2, D6), Axes6<3, 0, 2, 4, 1, 5>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D4, D1, D3, D5, D6, D2), Axes6<3, 0, 2, 4, 5, 1>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D4, D1, D3, D6, D2, D5), Axes6<3, 0, 2, 5, 1, 4>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D4, D1, D3, D6, D5, D2), Axes6<3, 0, 2, 5, 4, 1>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D4, D1, D5, D2, D3, D6), Axes6<3, 0, 4, 1, 2, 5>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D4, D1, D5, D2, D6, D3), Axes6<3, 0, 4, 1, 5, 2>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D4, D1, D5, D3, D2, D6), Axes6<3, 0, 4, 2, 1, 5>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D4, D1, D5, D3, D6, D2), Axes6<3, 0, 4, 2, 5, 1>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D4, D1, D5, D6, D2, D3), Axes6<3, 0, 4, 5, 1, 2>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D4, D1, D5, D6, D3, D2), Axes6<3, 0, 4, 5, 2, 1>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D4, D1, D6, D2, D3, D5), Axes6<3, 0, 5, 1, 2, 4>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D4, D1, D6, D2, D5, D3), Axes6<3, 0, 5, 1, 4, 2>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D4, D1, D6, D3, D2, D5), Axes6<3, 0, 5, 2, 1, 4>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D4, D1, D6, D3, D5, D2), Axes6<3, 0, 5, 2, 4, 1>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D4, D1, D6, D5, D2, D3), Axes6<3, 0, 5, 4, 1, 2>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D4, D1, D6, D5, D3, D2), Axes6<3, 0, 5, 4, 2, 1>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D4, D2, D1, D3, D5, D6), Axes6<3, 1, 0, 2, 4, 5>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D4, D2, D1, D3, D6, D5), Axes6<3, 1, 0, 2, 5, 4>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D4, D2, D1, D5, D3, D6), Axes6<3, 1, 0, 4, 2, 5>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D4, D2, D1, D5, D6, D3), Axes6<3, 1, 0, 4, 5, 2>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D4, D2, D1, D6, D3, D5), Axes6<3, 1, 0, 5, 2, 4>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D4, D2, D1, D6, D5, D3), Axes6<3, 1, 0, 5, 4, 2>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D4, D2, D3, D1, D5, D6), Axes6<3, 1, 2, 0, 4, 5>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D4, D2, D3, D1, D6, D5), Axes6<3, 1, 2, 0, 5, 4>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D4, D2, D3, D5, D1, D6), Axes6<3, 1, 2, 4, 0, 5>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D4, D2, D3, D5, D6, D1), Axes6<3, 1, 2, 4, 5, 0>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D4, D2, D3, D6, D1, D5), Axes6<3, 1, 2, 5, 0, 4>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D4, D2, D3, D6, D5, D1), Axes6<3, 1, 2, 5, 4, 0>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D4, D2, D5, D1, D3, D6), Axes6<3, 1, 4, 0, 2, 5>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D4, D2, D5, D1, D6, D3), Axes6<3, 1, 4, 0, 5, 2>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D4, D2, D5, D3, D1, D6), Axes6<3, 1, 4, 2, 0, 5>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D4, D2, D5, D3, D6, D1), Axes6<3, 1, 4, 2, 5, 0>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D4, D2, D5, D6, D1, D3), Axes6<3, 1, 4, 5, 0, 2>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D4, D2, D5, D6, D3, D1), Axes6<3, 1, 4, 5, 2, 0>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D4, D2, D6, D1, D3, D5), Axes6<3, 1, 5, 0, 2, 4>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D4, D2, D6, D1, D5, D3), Axes6<3, 1, 5, 0, 4, 2>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D4, D2, D6, D3, D1, D5), Axes6<3, 1, 5, 2, 0, 4>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D4, D2, D6, D3, D5, D1), Axes6<3, 1, 5, 2, 4, 0>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D4, D2, D6, D5, D1, D3), Axes6<3, 1, 5, 4, 0, 2>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D4, D2, D6, D5, D3, D1), Axes6<3, 1, 5, 4, 2, 0>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D4, D3, D1, D2, D5, D6), Axes6<3, 2, 0, 1, 4, 5>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D4, D3, D1, D2, D6, D5), Axes6<3, 2, 0, 1, 5, 4>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D4, D3, D1, D5, D2, D6), Axes6<3, 2, 0, 4, 1, 5>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D4, D3, D1, D5, D6, D2), Axes6<3, 2, 0, 4, 5, 1>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D4, D3, D1, D6, D2, D5), Axes6<3, 2, 0, 5, 1, 4>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D4, D3, D1, D6, D5, D2), Axes6<3, 2, 0, 5, 4, 1>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D4, D3, D2, D1, D5, D6), Axes6<3, 2, 1, 0, 4, 5>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D4, D3, D2, D1, D6, D5), Axes6<3, 2, 1, 0, 5, 4>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D4, D3, D2, D5, D1, D6), Axes6<3, 2, 1, 4, 0, 5>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D4, D3, D2, D5, D6, D1), Axes6<3, 2, 1, 4, 5, 0>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D4, D3, D2, D6, D1, D5), Axes6<3, 2, 1, 5, 0, 4>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D4, D3, D2, D6, D5, D1), Axes6<3, 2, 1, 5, 4, 0>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D4, D3, D5, D1, D2, D6), Axes6<3, 2, 4, 0, 1, 5>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D4, D3, D5, D1, D6, D2), Axes6<3, 2, 4, 0, 5, 1>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D4, D3, D5, D2, D1, D6), Axes6<3, 2, 4, 1, 0, 5>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D4, D3, D5, D2, D6, D1), Axes6<3, 2, 4, 1, 5, 0>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D4, D3, D5, D6, D1, D2), Axes6<3, 2, 4, 5, 0, 1>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D4, D3, D5, D6, D2, D1), Axes6<3, 2, 4, 5, 1, 0>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D4, D3, D6, D1, D2, D5), Axes6<3, 2, 5, 0, 1, 4>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D4, D3, D6, D1, D5, D2), Axes6<3, 2, 5, 0, 4, 1>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D4, D3, D6, D2, D1, D5), Axes6<3, 2, 5, 1, 0, 4>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D4, D3, D6, D2, D5, D1), Axes6<3, 2, 5, 1, 4, 0>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D4, D3, D6, D5, D1, D2), Axes6<3, 2, 5, 4, 0, 1>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D4, D3, D6, D5, D2, D1), Axes6<3, 2, 5, 4, 1, 0>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D4, D5, D1, D2, D3, D6), Axes6<3, 4, 0, 1, 2, 5>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D4, D5, D1, D2, D6, D3), Axes6<3, 4, 0, 1, 5, 2>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D4, D5, D1, D3, D2, D6), Axes6<3, 4, 0, 2, 1, 5>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D4, D5, D1, D3, D6, D2), Axes6<3, 4, 0, 2, 5, 1>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D4, D5, D1, D6, D2, D3), Axes6<3, 4, 0, 5, 1, 2>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D4, D5, D1, D6, D3, D2), Axes6<3, 4, 0, 5, 2, 1>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D4, D5, D2, D1, D3, D6), Axes6<3, 4, 1, 0, 2, 5>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D4, D5, D2, D1, D6, D3), Axes6<3, 4, 1, 0, 5, 2>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D4, D5, D2, D3, D1, D6), Axes6<3, 4, 1, 2, 0, 5>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D4, D5, D2, D3, D6, D1), Axes6<3, 4, 1, 2, 5, 0>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D4, D5, D2, D6, D1, D3), Axes6<3, 4, 1, 5, 0, 2>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D4, D5, D2, D6, D3, D1), Axes6<3, 4, 1, 5, 2, 0>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D4, D5, D3, D1, D2, D6), Axes6<3, 4, 2, 0, 1, 5>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D4, D5, D3, D1, D6, D2), Axes6<3, 4, 2, 0, 5, 1>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D4, D5, D3, D2, D1, D6), Axes6<3, 4, 2, 1, 0, 5>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D4, D5, D3, D2, D6, D1), Axes6<3, 4, 2, 1, 5, 0>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D4, D5, D3, D6, D1, D2), Axes6<3, 4, 2, 5, 0, 1>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D4, D5, D3, D6, D2, D1), Axes6<3, 4, 2, 5, 1, 0>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D4, D5, D6, D1, D2, D3), Axes6<3, 4, 5, 0, 1, 2>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D4, D5, D6, D1, D3, D2), Axes6<3, 4, 5, 0, 2, 1>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D4, D5, D6, D2, D1, D3), Axes6<3, 4, 5, 1, 0, 2>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D4, D5, D6, D2, D3, D1), Axes6<3, 4, 5, 1, 2, 0>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D4, D5, D6, D3, D1, D2), Axes6<3, 4, 5, 2, 0, 1>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D4, D5, D6, D3, D2, D1), Axes6<3, 4, 5, 2, 1, 0>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D4, D6, D1, D2, D3, D5), Axes6<3, 5, 0, 1, 2, 4>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D4, D6, D1, D2, D5, D3), Axes6<3, 5, 0, 1, 4, 2>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D4, D6, D1, D3, D2, D5), Axes6<3, 5, 0, 2, 1, 4>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D4, D6, D1, D3, D5, D2), Axes6<3, 5, 0, 2, 4, 1>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D4, D6, D1, D5, D2, D3), Axes6<3, 5, 0, 4, 1, 2>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D4, D6, D1, D5, D3, D2), Axes6<3, 5, 0, 4, 2, 1>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D4, D6, D2, D1, D3, D5), Axes6<3, 5, 1, 0, 2, 4>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D4, D6, D2, D1, D5, D3), Axes6<3, 5, 1, 0, 4, 2>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D4, D6, D2, D3, D1, D5), Axes6<3, 5, 1, 2, 0, 4>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D4, D6, D2, D3, D5, D1), Axes6<3, 5, 1, 2, 4, 0>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D4, D6, D2, D5, D1, D3), Axes6<3, 5, 1, 4, 0, 2>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D4, D6, D2, D5, D3, D1), Axes6<3, 5, 1, 4, 2, 0>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D4, D6, D3, D1, D2, D5), Axes6<3, 5, 2, 0, 1, 4>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D4, D6, D3, D1, D5, D2), Axes6<3, 5, 2, 0, 4, 1>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D4, D6, D3, D2, D1, D5), Axes6<3, 5, 2, 1, 0, 4>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D4, D6, D3, D2, D5, D1), Axes6<3, 5, 2, 1, 4, 0>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D4, D6, D3, D5, D1, D2), Axes6<3, 5, 2, 4, 0, 1>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D4, D6, D3, D5, D2, D1), Axes6<3, 5, 2, 4, 1, 0>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D4, D6, D5, D1, D2, D3), Axes6<3, 5, 4, 0, 1, 2>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D4, D6, D5, D1, D3, D2), Axes6<3, 5, 4, 0, 2, 1>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D4, D6, D5, D2, D1, D3), Axes6<3, 5, 4, 1, 0, 2>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D4, D6, D5, D2, D3, D1), Axes6<3, 5, 4, 1, 2, 0>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D4, D6, D5, D3, D1, D2), Axes6<3, 5, 4, 2, 0, 1>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D4, D6, D5, D3, D2, D1), Axes6<3, 5, 4, 2, 1, 0>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D5, D1, D2, D3, D4, D6), Axes6<4, 0, 1, 2, 3, 5>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D5, D1, D2, D3, D6, D4), Axes6<4, 0, 1, 2, 5, 3>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D5, D1, D2, D4, D3, D6), Axes6<4, 0, 1, 3, 2, 5>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D5, D1, D2, D4, D6, D3), Axes6<4, 0, 1, 3, 5, 2>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D5, D1, D2, D6, D3, D4), Axes6<4, 0, 1, 5, 2, 3>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D5, D1, D2, D6, D4, D3), Axes6<4, 0, 1, 5, 3, 2>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D5, D1, D3, D2, D4, D6), Axes6<4, 0, 2, 1, 3, 5>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D5, D1, D3, D2, D6, D4), Axes6<4, 0, 2, 1, 5, 3>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D5, D1, D3, D4, D2, D6), Axes6<4, 0, 2, 3, 1, 5>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D5, D1, D3, D4, D6, D2), Axes6<4, 0, 2, 3, 5, 1>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D5, D1, D3, D6, D2, D4), Axes6<4, 0, 2, 5, 1, 3>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D5, D1, D3, D6, D4, D2), Axes6<4, 0, 2, 5, 3, 1>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D5, D1, D4, D2, D3, D6), Axes6<4, 0, 3, 1, 2, 5>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D5, D1, D4, D2, D6, D3), Axes6<4, 0, 3, 1, 5, 2>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D5, D1, D4, D3, D2, D6), Axes6<4, 0, 3, 2, 1, 5>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D5, D1, D4, D3, D6, D2), Axes6<4, 0, 3, 2, 5, 1>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D5, D1, D4, D6, D2, D3), Axes6<4, 0, 3, 5, 1, 2>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D5, D1, D4, D6, D3, D2), Axes6<4, 0, 3, 5, 2, 1>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D5, D1, D6, D2, D3, D4), Axes6<4, 0, 5, 1, 2, 3>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D5, D1, D6, D2, D4, D3), Axes6<4, 0, 5, 1, 3, 2>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D5, D1, D6, D3, D2, D4), Axes6<4, 0, 5, 2, 1, 3>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D5, D1, D6, D3, D4, D2), Axes6<4, 0, 5, 2, 3, 1>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D5, D1, D6, D4, D2, D3), Axes6<4, 0, 5, 3, 1, 2>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D5, D1, D6, D4, D3, D2), Axes6<4, 0, 5, 3, 2, 1>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D5, D2, D1, D3, D4, D6), Axes6<4, 1, 0, 2, 3, 5>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D5, D2, D1, D3, D6, D4), Axes6<4, 1, 0, 2, 5, 3>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D5, D2, D1, D4, D3, D6), Axes6<4, 1, 0, 3, 2, 5>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D5, D2, D1, D4, D6, D3), Axes6<4, 1, 0, 3, 5, 2>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D5, D2, D1, D6, D3, D4), Axes6<4, 1, 0, 5, 2, 3>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D5, D2, D1, D6, D4, D3), Axes6<4, 1, 0, 5, 3, 2>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D5, D2, D3, D1, D4, D6), Axes6<4, 1, 2, 0, 3, 5>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D5, D2, D3, D1, D6, D4), Axes6<4, 1, 2, 0, 5, 3>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D5, D2, D3, D4, D1, D6), Axes6<4, 1, 2, 3, 0, 5>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D5, D2, D3, D4, D6, D1), Axes6<4, 1, 2, 3, 5, 0>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D5, D2, D3, D6, D1, D4), Axes6<4, 1, 2, 5, 0, 3>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D5, D2, D3, D6, D4, D1), Axes6<4, 1, 2, 5, 3, 0>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D5, D2, D4, D1, D3, D6), Axes6<4, 1, 3, 0, 2, 5>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D5, D2, D4, D1, D6, D3), Axes6<4, 1, 3, 0, 5, 2>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D5, D2, D4, D3, D1, D6), Axes6<4, 1, 3, 2, 0, 5>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D5, D2, D4, D3, D6, D1), Axes6<4, 1, 3, 2, 5, 0>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D5, D2, D4, D6, D1, D3), Axes6<4, 1, 3, 5, 0, 2>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D5, D2, D4, D6, D3, D1), Axes6<4, 1, 3, 5, 2, 0>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D5, D2, D6, D1, D3, D4), Axes6<4, 1, 5, 0, 2, 3>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D5, D2, D6, D1, D4, D3), Axes6<4, 1, 5, 0, 3, 2>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D5, D2, D6, D3, D1, D4), Axes6<4, 1, 5, 2, 0, 3>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D5, D2, D6, D3, D4, D1), Axes6<4, 1, 5, 2, 3, 0>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D5, D2, D6, D4, D1, D3), Axes6<4, 1, 5, 3, 0, 2>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D5, D2, D6, D4, D3, D1), Axes6<4, 1, 5, 3, 2, 0>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D5, D3, D1, D2, D4, D6), Axes6<4, 2, 0, 1, 3, 5>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D5, D3, D1, D2, D6, D4), Axes6<4, 2, 0, 1, 5, 3>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D5, D3, D1, D4, D2, D6), Axes6<4, 2, 0, 3, 1, 5>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D5, D3, D1, D4, D6, D2), Axes6<4, 2, 0, 3, 5, 1>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D5, D3, D1, D6, D2, D4), Axes6<4, 2, 0, 5, 1, 3>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D5, D3, D1, D6, D4, D2), Axes6<4, 2, 0, 5, 3, 1>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D5, D3, D2, D1, D4, D6), Axes6<4, 2, 1, 0, 3, 5>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D5, D3, D2, D1, D6, D4), Axes6<4, 2, 1, 0, 5, 3>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D5, D3, D2, D4, D1, D6), Axes6<4, 2, 1, 3, 0, 5>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D5, D3, D2, D4, D6, D1), Axes6<4, 2, 1, 3, 5, 0>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D5, D3, D2, D6, D1, D4), Axes6<4, 2, 1, 5, 0, 3>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D5, D3, D2, D6, D4, D1), Axes6<4, 2, 1, 5, 3, 0>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D5, D3, D4, D1, D2, D6), Axes6<4, 2, 3, 0, 1, 5>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D5, D3, D4, D1, D6, D2), Axes6<4, 2, 3, 0, 5, 1>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D5, D3, D4, D2, D1, D6), Axes6<4, 2, 3, 1, 0, 5>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D5, D3, D4, D2, D6, D1), Axes6<4, 2, 3, 1, 5, 0>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D5, D3, D4, D6, D1, D2), Axes6<4, 2, 3, 5, 0, 1>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D5, D3, D4, D6, D2, D1), Axes6<4, 2, 3, 5, 1, 0>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D5, D3, D6, D1, D2, D4), Axes6<4, 2, 5, 0, 1, 3>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D5, D3, D6, D1, D4, D2), Axes6<4, 2, 5, 0, 3, 1>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D5, D3, D6, D2, D1, D4), Axes6<4, 2, 5, 1, 0, 3>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D5, D3, D6, D2, D4, D1), Axes6<4, 2, 5, 1, 3, 0>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D5, D3, D6, D4, D1, D2), Axes6<4, 2, 5, 3, 0, 1>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D5, D3, D6, D4, D2, D1), Axes6<4, 2, 5, 3, 1, 0>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D5, D4, D1, D2, D3, D6), Axes6<4, 3, 0, 1, 2, 5>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D5, D4, D1, D2, D6, D3), Axes6<4, 3, 0, 1, 5, 2>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D5, D4, D1, D3, D2, D6), Axes6<4, 3, 0, 2, 1, 5>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D5, D4, D1, D3, D6, D2), Axes6<4, 3, 0, 2, 5, 1>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D5, D4, D1, D6, D2, D3), Axes6<4, 3, 0, 5, 1, 2>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D5, D4, D1, D6, D3, D2), Axes6<4, 3, 0, 5, 2, 1>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D5, D4, D2, D1, D3, D6), Axes6<4, 3, 1, 0, 2, 5>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D5, D4, D2, D1, D6, D3), Axes6<4, 3, 1, 0, 5, 2>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D5, D4, D2, D3, D1, D6), Axes6<4, 3, 1, 2, 0, 5>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D5, D4, D2, D3, D6, D1), Axes6<4, 3, 1, 2, 5, 0>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D5, D4, D2, D6, D1, D3), Axes6<4, 3, 1, 5, 0, 2>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D5, D4, D2, D6, D3, D1), Axes6<4, 3, 1, 5, 2, 0>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D5, D4, D3, D1, D2, D6), Axes6<4, 3, 2, 0, 1, 5>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D5, D4, D3, D1, D6, D2), Axes6<4, 3, 2, 0, 5, 1>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D5, D4, D3, D2, D1, D6), Axes6<4, 3, 2, 1, 0, 5>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D5, D4, D3, D2, D6, D1), Axes6<4, 3, 2, 1, 5, 0>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D5, D4, D3, D6, D1, D2), Axes6<4, 3, 2, 5, 0, 1>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D5, D4, D3, D6, D2, D1), Axes6<4, 3, 2, 5, 1, 0>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D5, D4, D6, D1, D2, D3), Axes6<4, 3, 5, 0, 1, 2>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D5, D4, D6, D1, D3, D2), Axes6<4, 3, 5, 0, 2, 1>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D5, D4, D6, D2, D1, D3), Axes6<4, 3, 5, 1, 0, 2>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D5, D4, D6, D2, D3, D1), Axes6<4, 3, 5, 1, 2, 0>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D5, D4, D6, D3, D1, D2), Axes6<4, 3, 5, 2, 0, 1>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D5, D4, D6, D3, D2, D1), Axes6<4, 3, 5, 2, 1, 0>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D5, D6, D1, D2, D3, D4), Axes6<4, 5, 0, 1, 2, 3>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D5, D6, D1, D2, D4, D3), Axes6<4, 5, 0, 1, 3, 2>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D5, D6, D1, D3, D2, D4), Axes6<4, 5, 0, 2, 1, 3>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D5, D6, D1, D3, D4, D2), Axes6<4, 5, 0, 2, 3, 1>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D5, D6, D1, D4, D2, D3), Axes6<4, 5, 0, 3, 1, 2>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D5, D6, D1, D4, D3, D2), Axes6<4, 5, 0, 3, 2, 1>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D5, D6, D2, D1, D3, D4), Axes6<4, 5, 1, 0, 2, 3>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D5, D6, D2, D1, D4, D3), Axes6<4, 5, 1, 0, 3, 2>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D5, D6, D2, D3, D1, D4), Axes6<4, 5, 1, 2, 0, 3>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D5, D6, D2, D3, D4, D1), Axes6<4, 5, 1, 2, 3, 0>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D5, D6, D2, D4, D1, D3), Axes6<4, 5, 1, 3, 0, 2>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D5, D6, D2, D4, D3, D1), Axes6<4, 5, 1, 3, 2, 0>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D5, D6, D3, D1, D2, D4), Axes6<4, 5, 2, 0, 1, 3>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D5, D6, D3, D1, D4, D2), Axes6<4, 5, 2, 0, 3, 1>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D5, D6, D3, D2, D1, D4), Axes6<4, 5, 2, 1, 0, 3>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D5, D6, D3, D2, D4, D1), Axes6<4, 5, 2, 1, 3, 0>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D5, D6, D3, D4, D1, D2), Axes6<4, 5, 2, 3, 0, 1>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D5, D6, D3, D4, D2, D1), Axes6<4, 5, 2, 3, 1, 0>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D5, D6, D4, D1, D2, D3), Axes6<4, 5, 3, 0, 1, 2>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D5, D6, D4, D1, D3, D2), Axes6<4, 5, 3, 0, 2, 1>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D5, D6, D4, D2, D1, D3), Axes6<4, 5, 3, 1, 0, 2>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D5, D6, D4, D2, D3, D1), Axes6<4, 5, 3, 1, 2, 0>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D5, D6, D4, D3, D1, D2), Axes6<4, 5, 3, 2, 0, 1>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D5, D6, D4, D3, D2, D1), Axes6<4, 5, 3, 2, 1, 0>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D6, D1, D2, D3, D4, D5), Axes6<5, 0, 1, 2, 3, 4>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D6, D1, D2, D3, D5, D4), Axes6<5, 0, 1, 2, 4, 3>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D6, D1, D2, D4, D3, D5), Axes6<5, 0, 1, 3, 2, 4>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D6, D1, D2, D4, D5, D3), Axes6<5, 0, 1, 3, 4, 2>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D6, D1, D2, D5, D3, D4), Axes6<5, 0, 1, 4, 2, 3>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D6, D1, D2, D5, D4, D3), Axes6<5, 0, 1, 4, 3, 2>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D6, D1, D3, D2, D4, D5), Axes6<5, 0, 2, 1, 3, 4>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D6, D1, D3, D2, D5, D4), Axes6<5, 0, 2, 1, 4, 3>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D6, D1, D3, D4, D2, D5), Axes6<5, 0, 2, 3, 1, 4>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D6, D1, D3, D4, D5, D2), Axes6<5, 0, 2, 3, 4, 1>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D6, D1, D3, D5, D2, D4), Axes6<5, 0, 2, 4, 1, 3>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D6, D1, D3, D5, D4, D2), Axes6<5, 0, 2, 4, 3, 1>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D6, D1, D4, D2, D3, D5), Axes6<5, 0, 3, 1, 2, 4>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D6, D1, D4, D2, D5, D3), Axes6<5, 0, 3, 1, 4, 2>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D6, D1, D4, D3, D2, D5), Axes6<5, 0, 3, 2, 1, 4>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D6, D1, D4, D3, D5, D2), Axes6<5, 0, 3, 2, 4, 1>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D6, D1, D4, D5, D2, D3), Axes6<5, 0, 3, 4, 1, 2>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D6, D1, D4, D5, D3, D2), Axes6<5, 0, 3, 4, 2, 1>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D6, D1, D5, D2, D3, D4), Axes6<5, 0, 4, 1, 2, 3>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D6, D1, D5, D2, D4, D3), Axes6<5, 0, 4, 1, 3, 2>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D6, D1, D5, D3, D2, D4), Axes6<5, 0, 4, 2, 1, 3>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D6, D1, D5, D3, D4, D2), Axes6<5, 0, 4, 2, 3, 1>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D6, D1, D5, D4, D2, D3), Axes6<5, 0, 4, 3, 1, 2>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D6, D1, D5, D4, D3, D2), Axes6<5, 0, 4, 3, 2, 1>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D6, D2, D1, D3, D4, D5), Axes6<5, 1, 0, 2, 3, 4>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D6, D2, D1, D3, D5, D4), Axes6<5, 1, 0, 2, 4, 3>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D6, D2, D1, D4, D3, D5), Axes6<5, 1, 0, 3, 2, 4>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D6, D2, D1, D4, D5, D3), Axes6<5, 1, 0, 3, 4, 2>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D6, D2, D1, D5, D3, D4), Axes6<5, 1, 0, 4, 2, 3>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D6, D2, D1, D5, D4, D3), Axes6<5, 1, 0, 4, 3, 2>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D6, D2, D3, D1, D4, D5), Axes6<5, 1, 2, 0, 3, 4>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D6, D2, D3, D1, D5, D4), Axes6<5, 1, 2, 0, 4, 3>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D6, D2, D3, D4, D1, D5), Axes6<5, 1, 2, 3, 0, 4>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D6, D2, D3, D4, D5, D1), Axes6<5, 1, 2, 3, 4, 0>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D6, D2, D3, D5, D1, D4), Axes6<5, 1, 2, 4, 0, 3>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D6, D2, D3, D5, D4, D1), Axes6<5, 1, 2, 4, 3, 0>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D6, D2, D4, D1, D3, D5), Axes6<5, 1, 3, 0, 2, 4>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D6, D2, D4, D1, D5, D3), Axes6<5, 1, 3, 0, 4, 2>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D6, D2, D4, D3, D1, D5), Axes6<5, 1, 3, 2, 0, 4>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D6, D2, D4, D3, D5, D1), Axes6<5, 1, 3, 2, 4, 0>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D6, D2, D4, D5, D1, D3), Axes6<5, 1, 3, 4, 0, 2>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D6, D2, D4, D5, D3, D1), Axes6<5, 1, 3, 4, 2, 0>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D6, D2, D5, D1, D3, D4), Axes6<5, 1, 4, 0, 2, 3>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D6, D2, D5, D1, D4, D3), Axes6<5, 1, 4, 0, 3, 2>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D6, D2, D5, D3, D1, D4), Axes6<5, 1, 4, 2, 0, 3>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D6, D2, D5, D3, D4, D1), Axes6<5, 1, 4, 2, 3, 0>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D6, D2, D5, D4, D1, D3), Axes6<5, 1, 4, 3, 0, 2>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D6, D2, D5, D4, D3, D1), Axes6<5, 1, 4, 3, 2, 0>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D6, D3, D1, D2, D4, D5), Axes6<5, 2, 0, 1, 3, 4>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D6, D3, D1, D2, D5, D4), Axes6<5, 2, 0, 1, 4, 3>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D6, D3, D1, D4, D2, D5), Axes6<5, 2, 0, 3, 1, 4>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D6, D3, D1, D4, D5, D2), Axes6<5, 2, 0, 3, 4, 1>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D6, D3, D1, D5, D2, D4), Axes6<5, 2, 0, 4, 1, 3>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D6, D3, D1, D5, D4, D2), Axes6<5, 2, 0, 4, 3, 1>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D6, D3, D2, D1, D4, D5), Axes6<5, 2, 1, 0, 3, 4>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D6, D3, D2, D1, D5, D4), Axes6<5, 2, 1, 0, 4, 3>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D6, D3, D2, D4, D1, D5), Axes6<5, 2, 1, 3, 0, 4>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D6, D3, D2, D4, D5, D1), Axes6<5, 2, 1, 3, 4, 0>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D6, D3, D2, D5, D1, D4), Axes6<5, 2, 1, 4, 0, 3>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D6, D3, D2, D5, D4, D1), Axes6<5, 2, 1, 4, 3, 0>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D6, D3, D4, D1, D2, D5), Axes6<5, 2, 3, 0, 1, 4>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D6, D3, D4, D1, D5, D2), Axes6<5, 2, 3, 0, 4, 1>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D6, D3, D4, D2, D1, D5), Axes6<5, 2, 3, 1, 0, 4>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D6, D3, D4, D2, D5, D1), Axes6<5, 2, 3, 1, 4, 0>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D6, D3, D4, D5, D1, D2), Axes6<5, 2, 3, 4, 0, 1>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D6, D3, D4, D5, D2, D1), Axes6<5, 2, 3, 4, 1, 0>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D6, D3, D5, D1, D2, D4), Axes6<5, 2, 4, 0, 1, 3>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D6, D3, D5, D1, D4, D2), Axes6<5, 2, 4, 0, 3, 1>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D6, D3, D5, D2, D1, D4), Axes6<5, 2, 4, 1, 0, 3>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D6, D3, D5, D2, D4, D1), Axes6<5, 2, 4, 1, 3, 0>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D6, D3, D5, D4, D1, D2), Axes6<5, 2, 4, 3, 0, 1>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D6, D3, D5, D4, D2, D1), Axes6<5, 2, 4, 3, 1, 0>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D6, D4, D1, D2, D3, D5), Axes6<5, 3, 0, 1, 2, 4>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D6, D4, D1, D2, D5, D3), Axes6<5, 3, 0, 1, 4, 2>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D6, D4, D1, D3, D2, D5), Axes6<5, 3, 0, 2, 1, 4>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D6, D4, D1, D3, D5, D2), Axes6<5, 3, 0, 2, 4, 1>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D6, D4, D1, D5, D2, D3), Axes6<5, 3, 0, 4, 1, 2>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D6, D4, D1, D5, D3, D2), Axes6<5, 3, 0, 4, 2, 1>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D6, D4, D2, D1, D3, D5), Axes6<5, 3, 1, 0, 2, 4>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D6, D4, D2, D1, D5, D3), Axes6<5, 3, 1, 0, 4, 2>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D6, D4, D2, D3, D1, D5), Axes6<5, 3, 1, 2, 0, 4>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D6, D4, D2, D3, D5, D1), Axes6<5, 3, 1, 2, 4, 0>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D6, D4, D2, D5, D1, D3), Axes6<5, 3, 1, 4, 0, 2>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D6, D4, D2, D5, D3, D1), Axes6<5, 3, 1, 4, 2, 0>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D6, D4, D3, D1, D2, D5), Axes6<5, 3, 2, 0, 1, 4>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D6, D4, D3, D1, D5, D2), Axes6<5, 3, 2, 0, 4, 1>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D6, D4, D3, D2, D1, D5), Axes6<5, 3, 2, 1, 0, 4>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D6, D4, D3, D2, D5, D1), Axes6<5, 3, 2, 1, 4, 0>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D6, D4, D3, D5, D1, D2), Axes6<5, 3, 2, 4, 0, 1>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D6, D4, D3, D5, D2, D1), Axes6<5, 3, 2, 4, 1, 0>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D6, D4, D5, D1, D2, D3), Axes6<5, 3, 4, 0, 1, 2>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D6, D4, D5, D1, D3, D2), Axes6<5, 3, 4, 0, 2, 1>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D6, D4, D5, D2, D1, D3), Axes6<5, 3, 4, 1, 0, 2>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D6, D4, D5, D2, D3, D1), Axes6<5, 3, 4, 1, 2, 0>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D6, D4, D5, D3, D1, D2), Axes6<5, 3, 4, 2, 0, 1>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D6, D4, D5, D3, D2, D1), Axes6<5, 3, 4, 2, 1, 0>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D6, D5, D1, D2, D3, D4), Axes6<5, 4, 0, 1, 2, 3>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D6, D5, D1, D2, D4, D3), Axes6<5, 4, 0, 1, 3, 2>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D6, D5, D1, D3, D2, D4), Axes6<5, 4, 0, 2, 1, 3>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D6, D5, D1, D3, D4, D2), Axes6<5, 4, 0, 2, 3, 1>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D6, D5, D1, D4, D2, D3), Axes6<5, 4, 0, 3, 1, 2>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D6, D5, D1, D4, D3, D2), Axes6<5, 4, 0, 3, 2, 1>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D6, D5, D2, D1, D3, D4), Axes6<5, 4, 1, 0, 2, 3>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D6, D5, D2, D1, D4, D3), Axes6<5, 4, 1, 0, 3, 2>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D6, D5, D2, D3, D1, D4), Axes6<5, 4, 1, 2, 0, 3>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D6, D5, D2, D3, D4, D1), Axes6<5, 4, 1, 2, 3, 0>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D6, D5, D2, D4, D1, D3), Axes6<5, 4, 1, 3, 0, 2>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D6, D5, D2, D4, D3, D1), Axes6<5, 4, 1, 3, 2, 0>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D6, D5, D3, D1, D2, D4), Axes6<5, 4, 2, 0, 1, 3>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D6, D5, D3, D1, D4, D2), Axes6<5, 4, 2, 0, 3, 1>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D6, D5, D3, D2, D1, D4), Axes6<5, 4, 2, 1, 0, 3>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D6, D5, D3, D2, D4, D1), Axes6<5, 4, 2, 1, 3, 0>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D6, D5, D3, D4, D1, D2), Axes6<5, 4, 2, 3, 0, 1>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D6, D5, D3, D4, D2, D1), Axes6<5, 4, 2, 3, 1, 0>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D6, D5, D4, D1, D2, D3), Axes6<5, 4, 3, 0, 1, 2>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D6, D5, D4, D1, D3, D2), Axes6<5, 4, 3, 0, 2, 1>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D6, D5, D4, D2, D1, D3), Axes6<5, 4, 3, 1, 0, 2>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D6, D5, D4, D2, D3, D1), Axes6<5, 4, 3, 1, 2, 0>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D6, D5, D4, D3, D1, D2), Axes6<5, 4, 3, 2, 0, 1>>
        for (D1, D2, D3, D4, D5, D6) {}
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
        > PermuteShapeTo<(D6, D5, D4, D3, D2, D1), Axes6<5, 4, 3, 2, 1, 0>>
        for (D1, D2, D3, D4, D5, D6) {}
    }
    mod replace_dim {
        use super::{Axes, Axis, Dim, Shape};
        /// Marker for shapes that can be indexed and have a dimension removed
        pub trait RemoveDimTo<Dst: Shape, Idx: Shape>: Shape {
            type Ax: Axes<Array = [isize; 1]>;
            /// All dimensions of idx should be the same as the dimensions of Self
            #[inline(always)]
            fn check(&self, idx: &Idx) {
                if !(Idx::NUM_DIMS <= Self::NUM_DIMS) {
                    ::core::panicking::panic(
                        "assertion failed: Idx::NUM_DIMS <= Self::NUM_DIMS",
                    )
                }
                let src_dims = self.concrete();
                let idx_dims = idx.concrete();
                for i in 0..Idx::NUM_DIMS {
                    match (&src_dims[i], &idx_dims[i]) {
                        (left_val, right_val) => {
                            if !(*left_val == *right_val) {
                                let kind = ::core::panicking::AssertKind::Eq;
                                ::core::panicking::assert_failed(
                                    kind,
                                    &*left_val,
                                    &*right_val,
                                    ::core::option::Option::Some(
                                        format_args!("dimension {0} not the same", i),
                                    ),
                                );
                            }
                        }
                    };
                }
            }
            #[inline]
            fn remove(&self, _: Idx) -> Dst {
                let ax = Self::Ax::as_array()[0] as usize;
                let src_dims = self.concrete();
                let mut dst_dims: Dst::Concrete = Default::default();
                let mut i_dst = 0;
                for i_src in 0..Self::NUM_DIMS {
                    if i_src != ax {
                        dst_dims[i_dst] = src_dims[i_src];
                        i_dst += 1;
                    }
                }
                Dst::from_concrete(&dst_dims).unwrap()
            }
        }
        /// Marker for shapes that can be indexed and have a dimension replaced with a new one
        pub trait ReplaceDimTo<Dst: Shape, Idx: Shape>: Shape {
            type Ax: Axes<Array = [isize; 1]>;
            /// All dimensions of idx *up to last dimension* (which is new)
            /// should be the same as the dimensions of Self
            #[inline(always)]
            fn check(&self, idx: &Idx) {
                if Self::NUM_DIMS == Dst::NUM_DIMS {
                    if !(Idx::NUM_DIMS <= Self::NUM_DIMS) {
                        ::core::panicking::panic(
                            "assertion failed: Idx::NUM_DIMS <= Self::NUM_DIMS",
                        )
                    }
                    let src_dims = self.concrete();
                    let idx_dims = idx.concrete();
                    for i in 0..Idx::NUM_DIMS - 1 {
                        match (&src_dims[i], &idx_dims[i]) {
                            (left_val, right_val) => {
                                if !(*left_val == *right_val) {
                                    let kind = ::core::panicking::AssertKind::Eq;
                                    ::core::panicking::assert_failed(
                                        kind,
                                        &*left_val,
                                        &*right_val,
                                        ::core::option::Option::Some(
                                            format_args!("dimension {0} not the same", i),
                                        ),
                                    );
                                }
                            }
                        };
                    }
                } else {}
            }
            #[inline]
            fn replace(&self, idx: Idx) -> Dst {
                let ax = Self::Ax::as_array()[0] as usize;
                if Self::NUM_DIMS == Dst::NUM_DIMS {
                    let src_dims = self.concrete();
                    let mut dst_dims: Dst::Concrete = Default::default();
                    for i in 0..Dst::NUM_DIMS {
                        dst_dims[i] = src_dims[i];
                    }
                    dst_dims[ax] = idx.concrete().into_iter().last().unwrap();
                    Dst::from_concrete(&dst_dims).unwrap()
                } else {
                    match (&Dst::NUM_DIMS, &(Self::NUM_DIMS + 1)) {
                        (left_val, right_val) => {
                            if !(*left_val == *right_val) {
                                let kind = ::core::panicking::AssertKind::Eq;
                                ::core::panicking::assert_failed(
                                    kind,
                                    &*left_val,
                                    &*right_val,
                                    ::core::option::Option::None,
                                );
                            }
                        }
                    };
                    match (&Self::NUM_DIMS, &<Idx as Shape>::NUM_DIMS) {
                        (left_val, right_val) => {
                            if !(*left_val == *right_val) {
                                let kind = ::core::panicking::AssertKind::Eq;
                                ::core::panicking::assert_failed(
                                    kind,
                                    &*left_val,
                                    &*right_val,
                                    ::core::option::Option::None,
                                );
                            }
                        }
                    };
                    match (&ax, &0) {
                        (left_val, right_val) => {
                            if !(*left_val == *right_val) {
                                let kind = ::core::panicking::AssertKind::Eq;
                                ::core::panicking::assert_failed(
                                    kind,
                                    &*left_val,
                                    &*right_val,
                                    ::core::option::Option::None,
                                );
                            }
                        }
                    };
                    let src_dims = self.concrete();
                    let idx_dims = idx.concrete();
                    let mut dst_dims: Dst::Concrete = Default::default();
                    for i in 0..Dst::NUM_DIMS {
                        dst_dims[i] = if i < Self::NUM_DIMS {
                            idx_dims[i]
                        } else {
                            src_dims[i - 1]
                        };
                    }
                    Dst::from_concrete(&dst_dims).unwrap()
                }
            }
        }
        impl<
            A: Dim,
            B: Dim,
            C: Dim,
            D: Dim,
            E: Dim,
            F: Dim,
            New: Dim,
        > ReplaceDimTo<(New, B, C, D, E, F), (New,)> for (A, B, C, D, E, F) {
            type Ax = Axis<0>;
        }
        impl<
            A: Dim,
            B: Dim,
            C: Dim,
            D: Dim,
            E: Dim,
            F: Dim,
        > RemoveDimTo<(B, C, D, E, F), ()> for (A, B, C, D, E, F) {
            type Ax = Axis<0>;
        }
        impl<
            A: Dim,
            B: Dim,
            C: Dim,
            D: Dim,
            E: Dim,
            F: Dim,
            New: Dim,
        > ReplaceDimTo<(A, New, C, D, E, F), (A, New)> for (A, B, C, D, E, F) {
            type Ax = Axis<1>;
        }
        impl<
            A: Dim,
            B: Dim,
            C: Dim,
            D: Dim,
            E: Dim,
            F: Dim,
        > RemoveDimTo<(A, C, D, E, F), (A,)> for (A, B, C, D, E, F) {
            type Ax = Axis<1>;
        }
        impl<
            A: Dim,
            B: Dim,
            C: Dim,
            D: Dim,
            E: Dim,
            F: Dim,
            New: Dim,
        > ReplaceDimTo<(A, B, New, D, E, F), (A, B, New)> for (A, B, C, D, E, F) {
            type Ax = Axis<2>;
        }
        impl<
            A: Dim,
            B: Dim,
            C: Dim,
            D: Dim,
            E: Dim,
            F: Dim,
        > RemoveDimTo<(A, B, D, E, F), (A, B)> for (A, B, C, D, E, F) {
            type Ax = Axis<2>;
        }
        impl<
            A: Dim,
            B: Dim,
            C: Dim,
            D: Dim,
            E: Dim,
            F: Dim,
            New: Dim,
        > ReplaceDimTo<(A, B, C, New, E, F), (A, B, C, New)> for (A, B, C, D, E, F) {
            type Ax = Axis<3>;
        }
        impl<
            A: Dim,
            B: Dim,
            C: Dim,
            D: Dim,
            E: Dim,
            F: Dim,
        > RemoveDimTo<(A, B, C, E, F), (A, B, C)> for (A, B, C, D, E, F) {
            type Ax = Axis<3>;
        }
        impl<
            A: Dim,
            B: Dim,
            C: Dim,
            D: Dim,
            E: Dim,
            F: Dim,
            New: Dim,
        > ReplaceDimTo<(A, B, C, D, New, F), (A, B, C, D, New)> for (A, B, C, D, E, F) {
            type Ax = Axis<4>;
        }
        impl<
            A: Dim,
            B: Dim,
            C: Dim,
            D: Dim,
            E: Dim,
            F: Dim,
        > RemoveDimTo<(A, B, C, D, F), (A, B, C, D)> for (A, B, C, D, E, F) {
            type Ax = Axis<4>;
        }
        impl<
            A: Dim,
            B: Dim,
            C: Dim,
            D: Dim,
            E: Dim,
            F: Dim,
            New: Dim,
        > ReplaceDimTo<(A, B, C, D, E, New), (A, B, C, D, E, New)>
        for (A, B, C, D, E, F) {
            type Ax = Axis<5>;
        }
        impl<
            A: Dim,
            B: Dim,
            C: Dim,
            D: Dim,
            E: Dim,
            F: Dim,
        > RemoveDimTo<(A, B, C, D, E), (A, B, C, D, E)> for (A, B, C, D, E, F) {
            type Ax = Axis<5>;
        }
        impl<
            B: Dim,
            C: Dim,
            D: Dim,
            E: Dim,
            F: Dim,
            New: Dim,
        > ReplaceDimTo<(New, C, D, E, F), (New,)> for (B, C, D, E, F) {
            type Ax = Axis<0>;
        }
        impl<B: Dim, C: Dim, D: Dim, E: Dim, F: Dim> RemoveDimTo<(C, D, E, F), ()>
        for (B, C, D, E, F) {
            type Ax = Axis<0>;
        }
        impl<
            B: Dim,
            C: Dim,
            D: Dim,
            E: Dim,
            F: Dim,
            New: Dim,
        > ReplaceDimTo<(B, New, D, E, F), (B, New)> for (B, C, D, E, F) {
            type Ax = Axis<1>;
        }
        impl<B: Dim, C: Dim, D: Dim, E: Dim, F: Dim> RemoveDimTo<(B, D, E, F), (B,)>
        for (B, C, D, E, F) {
            type Ax = Axis<1>;
        }
        impl<
            B: Dim,
            C: Dim,
            D: Dim,
            E: Dim,
            F: Dim,
            New: Dim,
        > ReplaceDimTo<(B, C, New, E, F), (B, C, New)> for (B, C, D, E, F) {
            type Ax = Axis<2>;
        }
        impl<B: Dim, C: Dim, D: Dim, E: Dim, F: Dim> RemoveDimTo<(B, C, E, F), (B, C)>
        for (B, C, D, E, F) {
            type Ax = Axis<2>;
        }
        impl<
            B: Dim,
            C: Dim,
            D: Dim,
            E: Dim,
            F: Dim,
            New: Dim,
        > ReplaceDimTo<(B, C, D, New, F), (B, C, D, New)> for (B, C, D, E, F) {
            type Ax = Axis<3>;
        }
        impl<B: Dim, C: Dim, D: Dim, E: Dim, F: Dim> RemoveDimTo<(B, C, D, F), (B, C, D)>
        for (B, C, D, E, F) {
            type Ax = Axis<3>;
        }
        impl<
            B: Dim,
            C: Dim,
            D: Dim,
            E: Dim,
            F: Dim,
            New: Dim,
        > ReplaceDimTo<(B, C, D, E, New), (B, C, D, E, New)> for (B, C, D, E, F) {
            type Ax = Axis<4>;
        }
        impl<
            B: Dim,
            C: Dim,
            D: Dim,
            E: Dim,
            F: Dim,
        > RemoveDimTo<(B, C, D, E), (B, C, D, E)> for (B, C, D, E, F) {
            type Ax = Axis<4>;
        }
        impl<
            C: Dim,
            D: Dim,
            E: Dim,
            F: Dim,
            New: Dim,
        > ReplaceDimTo<(New, D, E, F), (New,)> for (C, D, E, F) {
            type Ax = Axis<0>;
        }
        impl<C: Dim, D: Dim, E: Dim, F: Dim> RemoveDimTo<(D, E, F), ()>
        for (C, D, E, F) {
            type Ax = Axis<0>;
        }
        impl<
            C: Dim,
            D: Dim,
            E: Dim,
            F: Dim,
            New: Dim,
        > ReplaceDimTo<(C, New, E, F), (C, New)> for (C, D, E, F) {
            type Ax = Axis<1>;
        }
        impl<C: Dim, D: Dim, E: Dim, F: Dim> RemoveDimTo<(C, E, F), (C,)>
        for (C, D, E, F) {
            type Ax = Axis<1>;
        }
        impl<
            C: Dim,
            D: Dim,
            E: Dim,
            F: Dim,
            New: Dim,
        > ReplaceDimTo<(C, D, New, F), (C, D, New)> for (C, D, E, F) {
            type Ax = Axis<2>;
        }
        impl<C: Dim, D: Dim, E: Dim, F: Dim> RemoveDimTo<(C, D, F), (C, D)>
        for (C, D, E, F) {
            type Ax = Axis<2>;
        }
        impl<
            C: Dim,
            D: Dim,
            E: Dim,
            F: Dim,
            New: Dim,
        > ReplaceDimTo<(C, D, E, New), (C, D, E, New)> for (C, D, E, F) {
            type Ax = Axis<3>;
        }
        impl<C: Dim, D: Dim, E: Dim, F: Dim> RemoveDimTo<(C, D, E), (C, D, E)>
        for (C, D, E, F) {
            type Ax = Axis<3>;
        }
        impl<D: Dim, E: Dim, F: Dim, New: Dim> ReplaceDimTo<(New, E, F), (New,)>
        for (D, E, F) {
            type Ax = Axis<0>;
        }
        impl<D: Dim, E: Dim, F: Dim> RemoveDimTo<(E, F), ()> for (D, E, F) {
            type Ax = Axis<0>;
        }
        impl<D: Dim, E: Dim, F: Dim, New: Dim> ReplaceDimTo<(D, New, F), (D, New)>
        for (D, E, F) {
            type Ax = Axis<1>;
        }
        impl<D: Dim, E: Dim, F: Dim> RemoveDimTo<(D, F), (D,)> for (D, E, F) {
            type Ax = Axis<1>;
        }
        impl<D: Dim, E: Dim, F: Dim, New: Dim> ReplaceDimTo<(D, E, New), (D, E, New)>
        for (D, E, F) {
            type Ax = Axis<2>;
        }
        impl<D: Dim, E: Dim, F: Dim> RemoveDimTo<(D, E), (D, E)> for (D, E, F) {
            type Ax = Axis<2>;
        }
        impl<E: Dim, F: Dim, New: Dim> ReplaceDimTo<(New, F), (New,)> for (E, F) {
            type Ax = Axis<0>;
        }
        impl<E: Dim, F: Dim> RemoveDimTo<(F,), ()> for (E, F) {
            type Ax = Axis<0>;
        }
        impl<E: Dim, F: Dim, New: Dim> ReplaceDimTo<(E, New), (E, New)> for (E, F) {
            type Ax = Axis<1>;
        }
        impl<E: Dim, F: Dim> RemoveDimTo<(E,), (E,)> for (E, F) {
            type Ax = Axis<1>;
        }
        impl<F: Dim, New: Dim> ReplaceDimTo<(New,), (New,)> for (F,) {
            type Ax = Axis<0>;
        }
        impl<F: Dim> RemoveDimTo<(), ()> for (F,) {
            type Ax = Axis<0>;
        }
        impl<
            Batch: Dim,
            Seq: Dim,
            S1: Dim,
            S2: Dim,
        > ReplaceDimTo<(Batch, Seq, S2), (Batch, Seq)> for (S1, S2) {
            type Ax = Axis<0>;
        }
    }
    mod shape {
        use super::{Axes, Axis, HasAxes};
        use super::{Axes2, Axes3, Axes4, Axes5, Axes6};
        use super::{Const, ConstDim, Dim};
        /// Represents either `[T; N]` or `Vec<T>`
        pub trait Array<T>: IntoIterator<Item = T> {
            type Dim: Dim;
            fn dim(&self) -> Self::Dim;
        }
        impl<T, const N: usize> Array<T> for [T; N] {
            type Dim = Const<N>;
            fn dim(&self) -> Self::Dim {
                Const
            }
        }
        impl<T> Array<T> for std::vec::Vec<T> {
            type Dim = usize;
            fn dim(&self) -> Self::Dim {
                self.len()
            }
        }
        /// A collection of dimensions ([Dim]) that change how a multi-dimensional
        /// array is interacted with.
        pub trait Shape: 'static + std::fmt::Debug + Clone + Copy + Send + Sync + Eq + PartialEq + HasAxes<
                Self::AllAxes,
            > + HasAxes<Self::LastAxis> {
            /// The number of dimensions the shape has
            const NUM_DIMS: usize;
            /// Is `[usize; Self::NUM_DIMS]`, but that is not usable yet.
            type Concrete: std::fmt::Debug
                + Clone
                + Copy
                + Default
                + Eq
                + PartialEq
                + std::ops::Index<usize, Output = usize>
                + std::ops::IndexMut<usize>
                + Send
                + Sync
                + IntoIterator<Item = usize>
                + Into<std::vec::Vec<usize>>
                + AsRef<[usize]>;
            /// All the axes of this shape
            type AllAxes: Axes;
            /// The last axis of this shape
            type LastAxis: Axes;
            fn concrete(&self) -> Self::Concrete;
            fn from_concrete(concrete: &Self::Concrete) -> Option<Self>;
            /// The number of elements in this shape; the product of all dimensions.
            #[inline(always)]
            fn num_elements(&self) -> usize {
                self.concrete().into_iter().product()
            }
            /// The strides of how this shape is layed out in memory.
            #[inline(always)]
            fn strides(&self) -> Self::Concrete {
                let sizes = self.concrete();
                let mut strides: Self::Concrete = Default::default();
                strides[Self::NUM_DIMS - 1] = 1;
                for i in (0..(Self::NUM_DIMS - 1)).rev() {
                    strides[i] = strides[i + 1] * sizes[i + 1];
                }
                strides
            }
        }
        /// Represents a [Shape] that has all [ConstDim]s
        pub trait ConstShape: Default + Shape {
            const NUMEL: usize;
        }
        /// Represents something that has a [Shape].
        pub trait HasShape {
            type WithShape<New: Shape>: HasShape<Shape = New>;
            type Shape: Shape;
            fn shape(&self) -> &Self::Shape;
        }
        impl<S: Shape> HasShape for S {
            type WithShape<New: Shape> = New;
            type Shape = Self;
            fn shape(&self) -> &Self::Shape {
                self
            }
        }
        /// Compile time known shape with 0 dimensions
        pub type Rank0 = ();
        /// Compile time known shape with 1 dimensions
        pub type Rank1<const M: usize> = (Const<M>,);
        /// Compile time known shape with 2 dimensions
        pub type Rank2<const M: usize, const N: usize> = (Const<M>, Const<N>);
        /// Compile time known shape with 3 dimensions
        pub type Rank3<const M: usize, const N: usize, const O: usize> = (
            Const<M>,
            Const<N>,
            Const<O>,
        );
        /// Compile time known shape with 4 dimensions
        pub type Rank4<const M: usize, const N: usize, const O: usize, const P: usize> = (
            Const<M>,
            Const<N>,
            Const<O>,
            Const<P>,
        );
        /// Compile time known shape with 5 dimensions
        pub type Rank5<
            const M: usize,
            const N: usize,
            const O: usize,
            const P: usize,
            const Q: usize,
        > = (Const<M>, Const<N>, Const<O>, Const<P>, Const<Q>);
        #[rustfmt::skip]
        /// Compile time known shape with 6 dimensions
        pub type Rank6<
            const M: usize,
            const N: usize,
            const O: usize,
            const P: usize,
            const Q: usize,
            const R: usize,
        > = (Const<M>, Const<N>, Const<O>, Const<P>, Const<Q>, Const<R>);
        impl Shape for () {
            const NUM_DIMS: usize = 0;
            type Concrete = [usize; 0];
            type AllAxes = Axis<0>;
            type LastAxis = Axis<0>;
            #[inline(always)]
            fn concrete(&self) -> Self::Concrete {
                []
            }
            #[inline(always)]
            fn strides(&self) -> Self::Concrete {
                []
            }
            #[inline(always)]
            fn from_concrete(_: &Self::Concrete) -> Option<Self> {
                Some(())
            }
        }
        impl ConstShape for () {
            const NUMEL: usize = 1;
        }
        impl<D1: Dim> Shape for (D1,) {
            const NUM_DIMS: usize = 1;
            type Concrete = [usize; 1];
            type AllAxes = Axis<0>;
            type LastAxis = Axis<{ 1 - 1 }>;
            #[inline(always)]
            fn concrete(&self) -> Self::Concrete {
                [self.0.size()]
            }
            #[inline(always)]
            fn from_concrete(concrete: &Self::Concrete) -> Option<Self> {
                Some((Dim::from_size(concrete[0])?,))
            }
        }
        impl<D1: ConstDim> ConstShape for (D1,) {
            const NUMEL: usize = D1::SIZE * 1;
        }
        impl Shape for [usize; 1] {
            const NUM_DIMS: usize = 1;
            type Concrete = Self;
            type AllAxes = Axis<0>;
            type LastAxis = Axis<{ 1 - 1 }>;
            fn concrete(&self) -> Self::Concrete {
                *self
            }
            fn from_concrete(concrete: &Self::Concrete) -> Option<Self> {
                Some(*concrete)
            }
        }
        impl<D1: Dim, D2: Dim> Shape for (D1, D2) {
            const NUM_DIMS: usize = 2;
            type Concrete = [usize; 2];
            type AllAxes = Axes2<0, 1>;
            type LastAxis = Axis<{ 2 - 1 }>;
            #[inline(always)]
            fn concrete(&self) -> Self::Concrete {
                [self.0.size(), self.1.size()]
            }
            #[inline(always)]
            fn from_concrete(concrete: &Self::Concrete) -> Option<Self> {
                Some((Dim::from_size(concrete[0])?, Dim::from_size(concrete[1])?))
            }
        }
        impl<D1: ConstDim, D2: ConstDim> ConstShape for (D1, D2) {
            const NUMEL: usize = D1::SIZE * D2::SIZE * 1;
        }
        impl Shape for [usize; 2] {
            const NUM_DIMS: usize = 2;
            type Concrete = Self;
            type AllAxes = Axes2<0, 1>;
            type LastAxis = Axis<{ 2 - 1 }>;
            fn concrete(&self) -> Self::Concrete {
                *self
            }
            fn from_concrete(concrete: &Self::Concrete) -> Option<Self> {
                Some(*concrete)
            }
        }
        impl<D1: Dim, D2: Dim, D3: Dim> Shape for (D1, D2, D3) {
            const NUM_DIMS: usize = 3;
            type Concrete = [usize; 3];
            type AllAxes = Axes3<0, 1, 2>;
            type LastAxis = Axis<{ 3 - 1 }>;
            #[inline(always)]
            fn concrete(&self) -> Self::Concrete {
                [self.0.size(), self.1.size(), self.2.size()]
            }
            #[inline(always)]
            fn from_concrete(concrete: &Self::Concrete) -> Option<Self> {
                Some((
                    Dim::from_size(concrete[0])?,
                    Dim::from_size(concrete[1])?,
                    Dim::from_size(concrete[2])?,
                ))
            }
        }
        impl<D1: ConstDim, D2: ConstDim, D3: ConstDim> ConstShape for (D1, D2, D3) {
            const NUMEL: usize = D1::SIZE * D2::SIZE * D3::SIZE * 1;
        }
        impl Shape for [usize; 3] {
            const NUM_DIMS: usize = 3;
            type Concrete = Self;
            type AllAxes = Axes3<0, 1, 2>;
            type LastAxis = Axis<{ 3 - 1 }>;
            fn concrete(&self) -> Self::Concrete {
                *self
            }
            fn from_concrete(concrete: &Self::Concrete) -> Option<Self> {
                Some(*concrete)
            }
        }
        impl<D1: Dim, D2: Dim, D3: Dim, D4: Dim> Shape for (D1, D2, D3, D4) {
            const NUM_DIMS: usize = 4;
            type Concrete = [usize; 4];
            type AllAxes = Axes4<0, 1, 2, 3>;
            type LastAxis = Axis<{ 4 - 1 }>;
            #[inline(always)]
            fn concrete(&self) -> Self::Concrete {
                [self.0.size(), self.1.size(), self.2.size(), self.3.size()]
            }
            #[inline(always)]
            fn from_concrete(concrete: &Self::Concrete) -> Option<Self> {
                Some((
                    Dim::from_size(concrete[0])?,
                    Dim::from_size(concrete[1])?,
                    Dim::from_size(concrete[2])?,
                    Dim::from_size(concrete[3])?,
                ))
            }
        }
        impl<D1: ConstDim, D2: ConstDim, D3: ConstDim, D4: ConstDim> ConstShape
        for (D1, D2, D3, D4) {
            const NUMEL: usize = D1::SIZE * D2::SIZE * D3::SIZE * D4::SIZE * 1;
        }
        impl Shape for [usize; 4] {
            const NUM_DIMS: usize = 4;
            type Concrete = Self;
            type AllAxes = Axes4<0, 1, 2, 3>;
            type LastAxis = Axis<{ 4 - 1 }>;
            fn concrete(&self) -> Self::Concrete {
                *self
            }
            fn from_concrete(concrete: &Self::Concrete) -> Option<Self> {
                Some(*concrete)
            }
        }
        impl<D1: Dim, D2: Dim, D3: Dim, D4: Dim, D5: Dim> Shape
        for (D1, D2, D3, D4, D5) {
            const NUM_DIMS: usize = 5;
            type Concrete = [usize; 5];
            type AllAxes = Axes5<0, 1, 2, 3, 4>;
            type LastAxis = Axis<{ 5 - 1 }>;
            #[inline(always)]
            fn concrete(&self) -> Self::Concrete {
                [
                    self.0.size(),
                    self.1.size(),
                    self.2.size(),
                    self.3.size(),
                    self.4.size(),
                ]
            }
            #[inline(always)]
            fn from_concrete(concrete: &Self::Concrete) -> Option<Self> {
                Some((
                    Dim::from_size(concrete[0])?,
                    Dim::from_size(concrete[1])?,
                    Dim::from_size(concrete[2])?,
                    Dim::from_size(concrete[3])?,
                    Dim::from_size(concrete[4])?,
                ))
            }
        }
        impl<
            D1: ConstDim,
            D2: ConstDim,
            D3: ConstDim,
            D4: ConstDim,
            D5: ConstDim,
        > ConstShape for (D1, D2, D3, D4, D5) {
            const NUMEL: usize = D1::SIZE * D2::SIZE * D3::SIZE * D4::SIZE * D5::SIZE
                * 1;
        }
        impl Shape for [usize; 5] {
            const NUM_DIMS: usize = 5;
            type Concrete = Self;
            type AllAxes = Axes5<0, 1, 2, 3, 4>;
            type LastAxis = Axis<{ 5 - 1 }>;
            fn concrete(&self) -> Self::Concrete {
                *self
            }
            fn from_concrete(concrete: &Self::Concrete) -> Option<Self> {
                Some(*concrete)
            }
        }
        impl<D1: Dim, D2: Dim, D3: Dim, D4: Dim, D5: Dim, D6: Dim> Shape
        for (D1, D2, D3, D4, D5, D6) {
            const NUM_DIMS: usize = 6;
            type Concrete = [usize; 6];
            type AllAxes = Axes6<0, 1, 2, 3, 4, 5>;
            type LastAxis = Axis<{ 6 - 1 }>;
            #[inline(always)]
            fn concrete(&self) -> Self::Concrete {
                [
                    self.0.size(),
                    self.1.size(),
                    self.2.size(),
                    self.3.size(),
                    self.4.size(),
                    self.5.size(),
                ]
            }
            #[inline(always)]
            fn from_concrete(concrete: &Self::Concrete) -> Option<Self> {
                Some((
                    Dim::from_size(concrete[0])?,
                    Dim::from_size(concrete[1])?,
                    Dim::from_size(concrete[2])?,
                    Dim::from_size(concrete[3])?,
                    Dim::from_size(concrete[4])?,
                    Dim::from_size(concrete[5])?,
                ))
            }
        }
        impl<
            D1: ConstDim,
            D2: ConstDim,
            D3: ConstDim,
            D4: ConstDim,
            D5: ConstDim,
            D6: ConstDim,
        > ConstShape for (D1, D2, D3, D4, D5, D6) {
            const NUMEL: usize = D1::SIZE * D2::SIZE * D3::SIZE * D4::SIZE * D5::SIZE
                * D6::SIZE * 1;
        }
        impl Shape for [usize; 6] {
            const NUM_DIMS: usize = 6;
            type Concrete = Self;
            type AllAxes = Axes6<0, 1, 2, 3, 4, 5>;
            type LastAxis = Axis<{ 6 - 1 }>;
            fn concrete(&self) -> Self::Concrete {
                *self
            }
            fn from_concrete(concrete: &Self::Concrete) -> Option<Self> {
                Some(*concrete)
            }
        }
        /// Marker for shapes that have the same number of elements as `Dst`
        pub trait AssertSameNumel<Dst: ConstShape>: ConstShape {
            const TYPE_CHECK: ();
            fn assert_same_numel() {
                #[allow(clippy::let_unit_value)]
                let _ = <Self as AssertSameNumel<Dst>>::TYPE_CHECK;
            }
        }
        impl<Src: ConstShape, Dst: ConstShape> AssertSameNumel<Dst> for Src {
            const TYPE_CHECK: () = if !(Src::NUMEL == Dst::NUMEL) {
                ::core::panicking::panic("assertion failed: Src::NUMEL == Dst::NUMEL")
            };
        }
    }
    pub use shape::Array;
    pub use shape::{ConstShape, HasShape, Shape};
    pub use shape::{Rank0, Rank1, Rank2, Rank3, Rank4, Rank5, Rank6};
    mod slice {
        use super::*;
        use std::ops::{Bound, RangeBounds};
        fn get_start_bound(bound: Bound<&usize>) -> usize {
            match bound {
                Bound::Included(x) => *x,
                Bound::Excluded(x) => x + 1,
                Bound::Unbounded => 0,
            }
        }
        fn get_end_bound(bound: Bound<&usize>, size: usize) -> usize {
            match bound {
                Bound::Excluded(x) => *x,
                Bound::Included(x) => x + 1,
                Bound::Unbounded => size,
            }
        }
        pub trait SliceDim<R: RangeBounds<usize>>: Dim {
            type Sliced: Dim;
            fn slice(&self, range: &R) -> Option<Self::Sliced> {
                let size = self.size();
                let start_bound = get_start_bound(range.start_bound());
                let end_bound = get_end_bound(range.end_bound(), size);
                (end_bound <= size && start_bound <= end_bound)
                    .then_some(end_bound - start_bound)
                    .and_then(Self::Sliced::from_size)
            }
        }
        impl<D: Dim> SliceDim<std::ops::Range<usize>> for D {
            type Sliced = usize;
        }
        impl<D: Dim> SliceDim<std::ops::RangeTo<usize>> for D {
            type Sliced = usize;
        }
        impl<D: Dim> SliceDim<std::ops::RangeFrom<usize>> for D {
            type Sliced = usize;
        }
        impl<D: Dim> SliceDim<std::ops::RangeInclusive<usize>> for D {
            type Sliced = usize;
        }
        impl<D: Dim> SliceDim<std::ops::RangeToInclusive<usize>> for D {
            type Sliced = usize;
        }
        impl<D: Dim> SliceDim<std::ops::RangeFull> for D {
            type Sliced = D;
            fn slice(&self, _: &std::ops::RangeFull) -> Option<D> {
                Some(*self)
            }
        }
        pub trait SliceShape<R>: Shape {
            type Sliced: Shape<Concrete = Self::Concrete>;
            fn slice(&self, range: &R) -> Option<Self::Sliced>;
            fn first_idx_in_slice(&self, range: &R) -> usize;
        }
        impl SliceShape<()> for () {
            type Sliced = Self;
            fn slice(&self, _range: &()) -> Option<Self> {
                Some(())
            }
            fn first_idx_in_slice(&self, _range: &()) -> usize {
                0
            }
        }
        use super::broadcasts::length;
        impl<D1: Dim, R1: RangeBounds<usize>> SliceShape<(R1,)> for (D1,)
        where
            D1: SliceDim<R1>,
        {
            type Sliced = (D1::Sliced,);
            fn slice(&self, range: &(R1,)) -> Option<Self::Sliced> {
                Some((self.0.slice(&range.0)?,))
            }
            fn first_idx_in_slice(&self, range: &(R1,)) -> usize {
                let strides = self.strides();
                get_start_bound(range.0.start_bound()) * strides[0] + 0
            }
        }
        impl<R1: RangeBounds<usize>> SliceShape<(R1,)> for [usize; { 1 + 0 }]
        where
            usize: SliceDim<R1>,
        {
            type Sliced = (<usize as SliceDim<R1>>::Sliced,);
            fn slice(&self, range: &(R1,)) -> Option<Self::Sliced> {
                Some((self[0].slice(&range.0)?,))
            }
            fn first_idx_in_slice(&self, range: &(R1,)) -> usize {
                let strides = self.strides();
                get_start_bound(range.0.start_bound()) * strides[0] + 0
            }
        }
        impl<
            D1: Dim,
            D2: Dim,
            R1: RangeBounds<usize>,
            R2: RangeBounds<usize>,
        > SliceShape<(R1, R2)> for (D1, D2)
        where
            D1: SliceDim<R1>,
            D2: SliceDim<R2>,
        {
            type Sliced = (D1::Sliced, D2::Sliced);
            fn slice(&self, range: &(R1, R2)) -> Option<Self::Sliced> {
                Some((self.0.slice(&range.0)?, self.1.slice(&range.1)?))
            }
            fn first_idx_in_slice(&self, range: &(R1, R2)) -> usize {
                let strides = self.strides();
                get_start_bound(range.0.start_bound()) * strides[0]
                    + get_start_bound(range.1.start_bound()) * strides[1] + 0
            }
        }
        impl<R1: RangeBounds<usize>, R2: RangeBounds<usize>> SliceShape<(R1, R2)>
        for [usize; { 1 + (1 + 0) }]
        where
            usize: SliceDim<R1>,
            usize: SliceDim<R2>,
        {
            type Sliced = (
                <usize as SliceDim<R1>>::Sliced,
                <usize as SliceDim<R2>>::Sliced,
            );
            fn slice(&self, range: &(R1, R2)) -> Option<Self::Sliced> {
                Some((self[0].slice(&range.0)?, self[1].slice(&range.1)?))
            }
            fn first_idx_in_slice(&self, range: &(R1, R2)) -> usize {
                let strides = self.strides();
                get_start_bound(range.0.start_bound()) * strides[0]
                    + get_start_bound(range.1.start_bound()) * strides[1] + 0
            }
        }
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            R1: RangeBounds<usize>,
            R2: RangeBounds<usize>,
            R3: RangeBounds<usize>,
        > SliceShape<(R1, R2, R3)> for (D1, D2, D3)
        where
            D1: SliceDim<R1>,
            D2: SliceDim<R2>,
            D3: SliceDim<R3>,
        {
            type Sliced = (D1::Sliced, D2::Sliced, D3::Sliced);
            fn slice(&self, range: &(R1, R2, R3)) -> Option<Self::Sliced> {
                Some((
                    self.0.slice(&range.0)?,
                    self.1.slice(&range.1)?,
                    self.2.slice(&range.2)?,
                ))
            }
            fn first_idx_in_slice(&self, range: &(R1, R2, R3)) -> usize {
                let strides = self.strides();
                get_start_bound(range.0.start_bound()) * strides[0]
                    + get_start_bound(range.1.start_bound()) * strides[1]
                    + get_start_bound(range.2.start_bound()) * strides[2] + 0
            }
        }
        impl<
            R1: RangeBounds<usize>,
            R2: RangeBounds<usize>,
            R3: RangeBounds<usize>,
        > SliceShape<(R1, R2, R3)> for [usize; { 1 + (1 + (1 + 0)) }]
        where
            usize: SliceDim<R1>,
            usize: SliceDim<R2>,
            usize: SliceDim<R3>,
        {
            type Sliced = (
                <usize as SliceDim<R1>>::Sliced,
                <usize as SliceDim<R2>>::Sliced,
                <usize as SliceDim<R3>>::Sliced,
            );
            fn slice(&self, range: &(R1, R2, R3)) -> Option<Self::Sliced> {
                Some((
                    self[0].slice(&range.0)?,
                    self[1].slice(&range.1)?,
                    self[2].slice(&range.2)?,
                ))
            }
            fn first_idx_in_slice(&self, range: &(R1, R2, R3)) -> usize {
                let strides = self.strides();
                get_start_bound(range.0.start_bound()) * strides[0]
                    + get_start_bound(range.1.start_bound()) * strides[1]
                    + get_start_bound(range.2.start_bound()) * strides[2] + 0
            }
        }
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            R1: RangeBounds<usize>,
            R2: RangeBounds<usize>,
            R3: RangeBounds<usize>,
            R4: RangeBounds<usize>,
        > SliceShape<(R1, R2, R3, R4)> for (D1, D2, D3, D4)
        where
            D1: SliceDim<R1>,
            D2: SliceDim<R2>,
            D3: SliceDim<R3>,
            D4: SliceDim<R4>,
        {
            type Sliced = (D1::Sliced, D2::Sliced, D3::Sliced, D4::Sliced);
            fn slice(&self, range: &(R1, R2, R3, R4)) -> Option<Self::Sliced> {
                Some((
                    self.0.slice(&range.0)?,
                    self.1.slice(&range.1)?,
                    self.2.slice(&range.2)?,
                    self.3.slice(&range.3)?,
                ))
            }
            fn first_idx_in_slice(&self, range: &(R1, R2, R3, R4)) -> usize {
                let strides = self.strides();
                get_start_bound(range.0.start_bound()) * strides[0]
                    + get_start_bound(range.1.start_bound()) * strides[1]
                    + get_start_bound(range.2.start_bound()) * strides[2]
                    + get_start_bound(range.3.start_bound()) * strides[3] + 0
            }
        }
        impl<
            R1: RangeBounds<usize>,
            R2: RangeBounds<usize>,
            R3: RangeBounds<usize>,
            R4: RangeBounds<usize>,
        > SliceShape<(R1, R2, R3, R4)> for [usize; { 1 + (1 + (1 + (1 + 0))) }]
        where
            usize: SliceDim<R1>,
            usize: SliceDim<R2>,
            usize: SliceDim<R3>,
            usize: SliceDim<R4>,
        {
            type Sliced = (
                <usize as SliceDim<R1>>::Sliced,
                <usize as SliceDim<R2>>::Sliced,
                <usize as SliceDim<R3>>::Sliced,
                <usize as SliceDim<R4>>::Sliced,
            );
            fn slice(&self, range: &(R1, R2, R3, R4)) -> Option<Self::Sliced> {
                Some((
                    self[0].slice(&range.0)?,
                    self[1].slice(&range.1)?,
                    self[2].slice(&range.2)?,
                    self[3].slice(&range.3)?,
                ))
            }
            fn first_idx_in_slice(&self, range: &(R1, R2, R3, R4)) -> usize {
                let strides = self.strides();
                get_start_bound(range.0.start_bound()) * strides[0]
                    + get_start_bound(range.1.start_bound()) * strides[1]
                    + get_start_bound(range.2.start_bound()) * strides[2]
                    + get_start_bound(range.3.start_bound()) * strides[3] + 0
            }
        }
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            R1: RangeBounds<usize>,
            R2: RangeBounds<usize>,
            R3: RangeBounds<usize>,
            R4: RangeBounds<usize>,
            R5: RangeBounds<usize>,
        > SliceShape<(R1, R2, R3, R4, R5)> for (D1, D2, D3, D4, D5)
        where
            D1: SliceDim<R1>,
            D2: SliceDim<R2>,
            D3: SliceDim<R3>,
            D4: SliceDim<R4>,
            D5: SliceDim<R5>,
        {
            type Sliced = (D1::Sliced, D2::Sliced, D3::Sliced, D4::Sliced, D5::Sliced);
            fn slice(&self, range: &(R1, R2, R3, R4, R5)) -> Option<Self::Sliced> {
                Some((
                    self.0.slice(&range.0)?,
                    self.1.slice(&range.1)?,
                    self.2.slice(&range.2)?,
                    self.3.slice(&range.3)?,
                    self.4.slice(&range.4)?,
                ))
            }
            fn first_idx_in_slice(&self, range: &(R1, R2, R3, R4, R5)) -> usize {
                let strides = self.strides();
                get_start_bound(range.0.start_bound()) * strides[0]
                    + get_start_bound(range.1.start_bound()) * strides[1]
                    + get_start_bound(range.2.start_bound()) * strides[2]
                    + get_start_bound(range.3.start_bound()) * strides[3]
                    + get_start_bound(range.4.start_bound()) * strides[4] + 0
            }
        }
        impl<
            R1: RangeBounds<usize>,
            R2: RangeBounds<usize>,
            R3: RangeBounds<usize>,
            R4: RangeBounds<usize>,
            R5: RangeBounds<usize>,
        > SliceShape<(R1, R2, R3, R4, R5)> for [usize; { 1 + (1 + (1 + (1 + (1 + 0)))) }]
        where
            usize: SliceDim<R1>,
            usize: SliceDim<R2>,
            usize: SliceDim<R3>,
            usize: SliceDim<R4>,
            usize: SliceDim<R5>,
        {
            type Sliced = (
                <usize as SliceDim<R1>>::Sliced,
                <usize as SliceDim<R2>>::Sliced,
                <usize as SliceDim<R3>>::Sliced,
                <usize as SliceDim<R4>>::Sliced,
                <usize as SliceDim<R5>>::Sliced,
            );
            fn slice(&self, range: &(R1, R2, R3, R4, R5)) -> Option<Self::Sliced> {
                Some((
                    self[0].slice(&range.0)?,
                    self[1].slice(&range.1)?,
                    self[2].slice(&range.2)?,
                    self[3].slice(&range.3)?,
                    self[4].slice(&range.4)?,
                ))
            }
            fn first_idx_in_slice(&self, range: &(R1, R2, R3, R4, R5)) -> usize {
                let strides = self.strides();
                get_start_bound(range.0.start_bound()) * strides[0]
                    + get_start_bound(range.1.start_bound()) * strides[1]
                    + get_start_bound(range.2.start_bound()) * strides[2]
                    + get_start_bound(range.3.start_bound()) * strides[3]
                    + get_start_bound(range.4.start_bound()) * strides[4] + 0
            }
        }
        impl<
            D1: Dim,
            D2: Dim,
            D3: Dim,
            D4: Dim,
            D5: Dim,
            D6: Dim,
            R1: RangeBounds<usize>,
            R2: RangeBounds<usize>,
            R3: RangeBounds<usize>,
            R4: RangeBounds<usize>,
            R5: RangeBounds<usize>,
            R6: RangeBounds<usize>,
        > SliceShape<(R1, R2, R3, R4, R5, R6)> for (D1, D2, D3, D4, D5, D6)
        where
            D1: SliceDim<R1>,
            D2: SliceDim<R2>,
            D3: SliceDim<R3>,
            D4: SliceDim<R4>,
            D5: SliceDim<R5>,
            D6: SliceDim<R6>,
        {
            type Sliced = (
                D1::Sliced,
                D2::Sliced,
                D3::Sliced,
                D4::Sliced,
                D5::Sliced,
                D6::Sliced,
            );
            fn slice(&self, range: &(R1, R2, R3, R4, R5, R6)) -> Option<Self::Sliced> {
                Some((
                    self.0.slice(&range.0)?,
                    self.1.slice(&range.1)?,
                    self.2.slice(&range.2)?,
                    self.3.slice(&range.3)?,
                    self.4.slice(&range.4)?,
                    self.5.slice(&range.5)?,
                ))
            }
            fn first_idx_in_slice(&self, range: &(R1, R2, R3, R4, R5, R6)) -> usize {
                let strides = self.strides();
                get_start_bound(range.0.start_bound()) * strides[0]
                    + get_start_bound(range.1.start_bound()) * strides[1]
                    + get_start_bound(range.2.start_bound()) * strides[2]
                    + get_start_bound(range.3.start_bound()) * strides[3]
                    + get_start_bound(range.4.start_bound()) * strides[4]
                    + get_start_bound(range.5.start_bound()) * strides[5] + 0
            }
        }
        impl<
            R1: RangeBounds<usize>,
            R2: RangeBounds<usize>,
            R3: RangeBounds<usize>,
            R4: RangeBounds<usize>,
            R5: RangeBounds<usize>,
            R6: RangeBounds<usize>,
        > SliceShape<(R1, R2, R3, R4, R5, R6)>
        for [usize; { 1 + (1 + (1 + (1 + (1 + (1 + 0))))) }]
        where
            usize: SliceDim<R1>,
            usize: SliceDim<R2>,
            usize: SliceDim<R3>,
            usize: SliceDim<R4>,
            usize: SliceDim<R5>,
            usize: SliceDim<R6>,
        {
            type Sliced = (
                <usize as SliceDim<R1>>::Sliced,
                <usize as SliceDim<R2>>::Sliced,
                <usize as SliceDim<R3>>::Sliced,
                <usize as SliceDim<R4>>::Sliced,
                <usize as SliceDim<R5>>::Sliced,
                <usize as SliceDim<R6>>::Sliced,
            );
            fn slice(&self, range: &(R1, R2, R3, R4, R5, R6)) -> Option<Self::Sliced> {
                Some((
                    self[0].slice(&range.0)?,
                    self[1].slice(&range.1)?,
                    self[2].slice(&range.2)?,
                    self[3].slice(&range.3)?,
                    self[4].slice(&range.4)?,
                    self[5].slice(&range.5)?,
                ))
            }
            fn first_idx_in_slice(&self, range: &(R1, R2, R3, R4, R5, R6)) -> usize {
                let strides = self.strides();
                get_start_bound(range.0.start_bound()) * strides[0]
                    + get_start_bound(range.1.start_bound()) * strides[1]
                    + get_start_bound(range.2.start_bound()) * strides[2]
                    + get_start_bound(range.3.start_bound()) * strides[3]
                    + get_start_bound(range.4.start_bound()) * strides[4]
                    + get_start_bound(range.5.start_bound()) * strides[5] + 0
            }
        }
    }
    pub use slice::SliceShape;
    /// Marker for shapes that can be converted using their concrete types.
    pub trait RealizeShapeTo<Dst: Shape>: Shape {
        fn realized(&self) -> Option<Dst>;
    }
    impl<Src: Shape<Concrete = Dst::Concrete>, Dst: Shape> RealizeShapeTo<Dst> for Src {
        #[inline(always)]
        fn realized(&self) -> Option<Dst> {
            Dst::from_concrete(&self.concrete())
        }
    }
}
