use core::fmt;
use core::ops::{
    AddAssign,
    DivAssign,
    MulAssign,
    Neg,
    RemAssign,
    SubAssign,
};
use num_traits::{
    Float,
    Num,
    NumCast,
};


/// A data type with this trait has the properties of a
/// set of scalar numbers underlying vector and matrix
/// data types.
pub trait Scalar
where
    Self: Copy,
    Self: Clone,
    Self: fmt::Debug,
    Self: Num,
    Self: NumCast,
    Self: PartialOrd,
    Self: AddAssign,
    Self: SubAssign,
    Self: MulAssign,
    Self: DivAssign,
    Self: RemAssign,
{
}

impl<T> Scalar for T where
    T: Copy + Clone + fmt::Debug + Num + NumCast + PartialOrd + AddAssign + SubAssign + MulAssign + DivAssign + RemAssign
{
}

/// Scalar numbers with a notion of subtraction and have additive
/// inverses.
pub trait ScalarSigned
where
    Self: Scalar + Neg<Output = Self>,
{
}

impl<T> ScalarSigned for T where T: Scalar + Neg<Output = T> {}

pub trait ScalarCmp:
    approx_cmp::AbsDiffEq<Tolerance = Self>
    + approx_cmp::AbsDiffAllEq<AllTolerance = Self>
    + approx_cmp::AssertAbsDiffEq<DebugAbsDiff = Self, DebugTolerance = Self>
    + approx_cmp::AssertAbsDiffAllEq<AllDebugTolerance = Self>
    + approx_cmp::RelativeEq<Tolerance = Self>
    + approx_cmp::RelativeAllEq<AllTolerance = Self>
    + approx_cmp::AssertRelativeEq<DebugAbsDiff = Self, DebugTolerance = Self>
    + approx_cmp::AssertRelativeAllEq<AllDebugTolerance = Self>
    + approx_cmp::UlpsEq<Tolerance = Self, UlpsTolerance = Self::IntegerRepr>
    + approx_cmp::UlpsAllEq<AllTolerance = Self, AllUlpsTolerance = Self::IntegerRepr>
    + approx_cmp::AssertUlpsEq<
        DebugAbsDiff = Self,
        DebugUlpsDiff = Option<Self::IntegerRepr>,
        DebugTolerance = Self,
        DebugUlpsTolerance = Self::IntegerRepr,
    > + approx_cmp::AssertUlpsAllEq<AllDebugTolerance = Self>
{
    type IntegerRepr: Copy + Clone + fmt::Debug;

    fn default_epsilon() -> Self;

    fn default_max_ulps() -> Self::IntegerRepr;
}

/// Scalar numbers that have the properties of finite precision
/// floating point arithmetic.
pub trait ScalarFloat: Scalar + ScalarCmp + Float {}

impl<T> ScalarFloat for T where T: Scalar + ScalarCmp + Float {}

impl ScalarCmp for f32 {
    type IntegerRepr = u32;

    fn default_epsilon() -> Self {
        num_traits::Float::epsilon()
    }

    fn default_max_ulps() -> Self::IntegerRepr {
        4
    }
}

impl ScalarCmp for f64 {
    type IntegerRepr = u64;

    fn default_epsilon() -> Self {
        num_traits::Float::epsilon()
    }

    fn default_max_ulps() -> Self::IntegerRepr {
        4
    }
}
