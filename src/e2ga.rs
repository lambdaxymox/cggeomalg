use crate::scalar::{
    Scalar,
    ScalarSigned,
    ScalarFloat,
};
use std::ops::{
    Index,
    IndexMut,
};
use std::fmt;


#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum BasisElement {
    /// The `1` , or scalar component of a Euclidean multivector.
    C,
    E1,
    E2,
    E12,
}

impl fmt::Display for BasisElement {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        let disp = match *self {
            BasisElement::C => "1",
            BasisElement::E1 => "e1",
            BasisElement::E2 => "e2",
            BasisElement::E12 => "e1 /\\ e2",
        };

        write!(formatter, "{}", disp)
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
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

    #[inline]
    pub fn unit_c() -> Self {
        Self::new(S::one(), S::zero(), S::zero(), S::zero())
    }

    #[inline]
    pub fn unit_e1() -> Self {
        Self::new(S::zero(), S::one(), S::zero(), S::zero())
    }

    #[inline]
    pub fn unit_e2() -> Self {
        Self::new(S::zero(), S::zero(), S::one(), S::zero())
    }

    #[inline]
    pub fn unit_e12() -> Self {
        Self::new(S::zero(), S::zero(), S::zero(), S::one())
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

impl<S> fmt::Display for EuclideanMultivector2<S>
where
    S: fmt::Display
{
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        use BasisElement::*;
        write!(
            formatter, 
            "{} {} + {} {} + {} {} + {} {}",
            self.data[0], C, self.data[1], E1, self.data[2], E2, self.data[3], E12
        )
    }
}