//! Module for data type related traits and structs.
//!

/// Represents a type where all 0 bits is a valid pattern.
pub trait SafeZeros {}

/// Represents a unit type, but no arithmetic.
pub trait Unit:
    'static
    + Copy
    + Clone
    + Default
    + std::fmt::Debug
    + PartialEq
    + PartialOrd
    + Send
    + Sync
    + std::marker::Unpin
    + SafeZeros
{
    const ONE: Self;
}

macro_rules! unit {
    ($type:ty, $one:expr) => {
        impl SafeZeros for $type {}
        impl Unit for $type {
            const ONE: Self = $one;
        }
    };
}

unit!(f32, 1.0);
unit!(f64, 1.0);
unit!(usize, 1);
unit!(isize, 1);
unit!(u8, 1);
unit!(i8, 1);
unit!(u16, 1);
unit!(i16, 1);
unit!(u32, 1);
unit!(i32, 1);
unit!(u64, 1);
unit!(i64, 1);
unit!(u128, 1);
unit!(i128, 1);
unit!(bool, true);

/// Represents something that has a [Unit].
pub trait HasUnitType {
    type Unit: Unit;
}

/// Represents a data type or element of an array that can have
/// arithmatic operations applied to it. The main difference
/// between [Dtype] and [Unit] is that [`bool`] is [Unit], but
/// not [Dtype].
pub trait Dtype:
    Unit
    + std::ops::Add<Self, Output = Self>
    + std::ops::Sub<Self, Output = Self>
    + std::ops::Mul<Self, Output = Self>
    + std::ops::Div<Self, Output = Self>
    + std::ops::AddAssign
    + std::ops::SubAssign
    + std::ops::MulAssign
    + std::ops::DivAssign
    + num_traits::FromPrimitive
    + num_traits::ToPrimitive
{
}
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

/// Trait for floating-point numbers.
pub trait Float: Dtype + std::ops::Neg<Output = Self> {
    const SMOL: Self;
    fn exp(self) -> Self;
    fn ln(self) -> Self;
    fn min(self, other: Self) -> Self;
    fn max(self, other: Self) -> Self;
}

impl Float for f32 {
    const SMOL: f32 = 1.0e-20;

    #[inline(always)]
    fn exp(self) -> Self {
        f32::exp(self)
    }
    #[inline(always)]
    fn ln(self) -> Self {
        f32::ln(self)
    }
    #[inline(always)]
    fn min(self, other: Self) -> Self {
        f32::min(self, other)
    }
    #[inline(always)]
    fn max(self, other: Self) -> Self {
        f32::max(self, other)
    }
}
impl Float for f64 {
    const SMOL: f64 = 1.0e-20;

    #[inline(always)]
    fn exp(self) -> Self {
        f64::exp(self)
    }
    #[inline(always)]
    fn ln(self) -> Self {
        f64::ln(self)
    }
    #[inline(always)]
    fn min(self, other: Self) -> Self {
        f64::min(self, other)
    }
    #[inline(always)]
    fn max(self, other: Self) -> Self {
        f64::max(self, other)
    }
}
