use crate::scalar::{
    Scalar,
    ScalarSigned,
    ScalarFloat,
};

const BASIS_COUNT: usize = 4;

pub enum Basis {
    /// The `1` , or scalar component of a Euclidean multivector.
    S1,
    E1,
    E2,
    E12,
}

pub struct EuclideanMultivector2<S> {
    data: [S; BASIS_COUNT],
}

impl<S> EuclideanMultivector2<S> 
where
    S: Scalar
{
    #[inline]
    pub fn zero() -> Self {
        Self {
            data: [S::zero(), S::zero(), S::zero(), S::zero()],
        }
    }
}

