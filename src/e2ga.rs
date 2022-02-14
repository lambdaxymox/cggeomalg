use crate::scalar::{
    Scalar,
    ScalarSigned,
    ScalarFloat,
};
use std::ops::{
    Index,
    IndexMut,
};

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum BasisElement {
    /// The `1` , or scalar component of a Euclidean multivector.
    C,
    E1,
    E2,
    E12,
}

pub struct EuclideanMultivector2<S> {
    data: [S; 4],
}

impl<S> EuclideanMultivector2<S> 
where
    S: Scalar
{
    #[inline]
    pub fn new(c: S, e1: S, e2: S, e12: S) -> Self {
        Self {
            data: [c, e1, e2, e12]
        }
    }

    #[inline]
    pub fn zero() -> Self {
        Self {
            data: [S::zero(), S::zero(), S::zero(), S::zero()],
        }
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.data.len()
    }
}

impl<S> Index<usize> for EuclideanMultivector2<S> 
where
    S: Scalar
{
    type Output = S;

    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}

impl<S> IndexMut<usize> for EuclideanMultivector2<S> 
where
    S: Scalar
{
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.data[index]
    }
}

impl<S> Index<BasisElement> for EuclideanMultivector2<S> 
where
    S: Scalar
{
    type Output = S;

    fn index(&self, index: BasisElement) -> &Self::Output {
        let idx = match index {
            BasisElement::C => 0,
            BasisElement::E1 => 1,
            BasisElement::E2 => 2,
            BasisElement::E12 => 3,
        };

        &self.data[idx]
    }
}

impl<S> IndexMut<BasisElement> for EuclideanMultivector2<S>
where
    S: Scalar
{
    fn index_mut(&mut self, index: BasisElement) -> &mut Self::Output {
        let idx = match index {
            BasisElement::C => 0,
            BasisElement::E1 => 1,
            BasisElement::E2 => 2,
            BasisElement::E12 => 3,
        };

        &mut self.data[idx]
    }
}