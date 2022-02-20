use crate::scalar::{
    Scalar,
    ScalarSigned,
    ScalarFloat,
};
use crate::{
    impl_coords,
    impl_coords_deref,
};
use core::ops::{
    Deref,
    DerefMut,
    Index,
    IndexMut,
    Add,
    BitXor,
    BitOr,
    Div,
    Mul,
    Not,
    Sub,
};
use core::fmt;


#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct EuclideanMultivector2<S> {
    data: [S; 4],
}

impl<S> EuclideanMultivector2<S>
where
    S: Copy
{
    /// Get a pointer to the underlying array.
    #[inline]
    pub fn as_ptr(&self) -> *const S {
        &self.data[0]
    }

    /// Get a mutable pointer to the underlying array.
    #[inline]
    pub fn as_mut_ptr(&mut self) -> *mut S {
        &mut self.data[0]
    }

    /// Get a slice of the underlying elements of the data type.
    #[inline]
    pub fn as_slice(&self) -> &[S] {
        <Self as AsRef<[S; 4]>>::as_ref(self)
    }
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

impl<S> AsRef<[S; 4]> for EuclideanMultivector2<S> {
    #[inline]
    fn as_ref(&self) -> &[S; 4] {
        unsafe {
            &*(self as *const EuclideanMultivector2<S> as *const [S; 4])
        }
    }
}

impl<S> AsMut<[S; 4]> for EuclideanMultivector2<S> {
    #[inline]
    fn as_mut(&mut self) -> &mut [S; 4] {
        unsafe {
            &mut *(self as *mut EuclideanMultivector2<S> as *mut [S; 4])
        }
    }
}

impl<S> AsRef<(S, S, S, S)> for EuclideanMultivector2<S> {
    #[inline]
    fn as_ref(&self) -> &(S, S, S, S) {
        unsafe {
            &*(self as *const EuclideanMultivector2<S> as *const (S, S, S, S))
        }
    }
}

impl<S> AsMut<(S, S, S, S)> for EuclideanMultivector2<S> {
    #[inline]
    fn as_mut(&mut self) -> &mut (S, S, S, S) {
        unsafe {
            &mut *(self as *mut EuclideanMultivector2<S> as *mut (S, S, S, S))
        }
    }
}

impl<S> fmt::Display for EuclideanMultivector2<S>
where
    S: fmt::Display
{
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(
            formatter, 
            "{} + {} e1 + {} e2 + {} e1 /\\ e2",
            self.data[0], self.data[1], self.data[2], self.data[3]
        )
    }
}

impl<S> EuclideanMultivector2<S> 
where
    S: ScalarSigned
{
    pub fn reverse(&self) -> Self {
        Self::new(self.data[0], self.data[1], self.data[2], -self.data[3])
    }

    pub fn reverse_mut(&mut self) {
        self.data[3] = -self.data[3];
    }

    pub fn conjugate(&self) -> Self {
        Self::new(self.data[0], -self.data[1], -self.data[2], -self.data[3])
    }

    pub fn conjugate_mut(&mut self) {
        self.data[1] = -self.data[1];
        self.data[2] = -self.data[2];
        self.data[3] = -self.data[3];
    }

    pub fn involute(&self) -> Self {
        Self::new(self.data[0], -self.data[1], -self.data[2], self.data[3])
    }

    pub fn involute_mut(&mut self) {
        self.data[1] = -self.data[1];
        self.data[2] = -self.data[2];
    }

    pub fn dual(&self) -> Self {
        Self::new(-self.data[3], -self.data[2], self.data[1], self.data[0])
    }

    pub fn dual_mut(&mut self) {
        let mut result = Self::zero();
        result.data[0] = -self.data[3];
        result.data[1] = -self.data[2];
        result.data[2] = self.data[1];
        result.data[3] = self.data[0];
        *self = result;
    }
}

impl<S> Not for EuclideanMultivector2<S> 
where
    S: ScalarSigned
{
    type Output = EuclideanMultivector2<S>;

    fn not(self) -> Self::Output {
        let mut result = Self::zero();
        result.data[0] = -self.data[3];
        result.data[1] = -self.data[2];
        result.data[2] = self.data[1];
        result.data[3] = self.data[0];
        
        result
    }
}

impl<S> Not for &EuclideanMultivector2<S> 
where
    S: ScalarSigned
{
    type Output = EuclideanMultivector2<S>;

    fn not(self) -> Self::Output {
        let mut result = EuclideanMultivector2::zero();
        result.data[0] = -self.data[3];
        result.data[1] = -self.data[2];
        result.data[2] = self.data[1];
        result.data[3] = self.data[0];
        
        result
    }
}

impl<S> Mul<EuclideanMultivector2<S>> for EuclideanMultivector2<S>
where
    S: Scalar
{
    type Output = EuclideanMultivector2<S>;

    fn mul(self, b: EuclideanMultivector2<S>) -> Self::Output {
        let a = self;
        let result_1   = a[0] * b[0] + a[1] * b[1] + a[2] * b[2] - a[3] * b[3];
        let result_e1  = a[0] * b[1] + a[1] * b[0] - a[2] * b[3] + a[3] * b[2];
        let result_e2  = a[0] * b[2] + a[1] * b[3] + a[2] * b[0] - a[3] * b[1];
        let result_e12 = a[0] * b[3] + a[1] * b[2] - a[2] * b[1] + a[3] * b[0];

        EuclideanMultivector2::new(result_1, result_e1, result_e2, result_e12)
    }
}

impl<S> Mul<&EuclideanMultivector2<S>> for EuclideanMultivector2<S>
where
    S: Scalar
{
    type Output = EuclideanMultivector2<S>;

    fn mul(self, b: &EuclideanMultivector2<S>) -> Self::Output {
        let a = self;
        let result_1   = a[0] * b[0] + a[1] * b[1] + a[2] * b[2] - a[3] * b[3];
        let result_e1  = a[0] * b[1] + a[1] * b[0] - a[2] * b[3] + a[3] * b[2];
        let result_e2  = a[0] * b[2] + a[1] * b[3] + a[2] * b[0] - a[3] * b[1];
        let result_e12 = a[0] * b[3] + a[1] * b[2] - a[2] * b[1] + a[3] * b[0];

        EuclideanMultivector2::new(result_1, result_e1, result_e2, result_e12)
    }
}

impl<S> Mul<EuclideanMultivector2<S>> for &EuclideanMultivector2<S>
where
    S: Scalar
{
    type Output = EuclideanMultivector2<S>;

    fn mul(self, b: EuclideanMultivector2<S>) -> Self::Output {
        let a = self;
        let result_1   = a[0] * b[0] + a[1] * b[1] + a[2] * b[2] - a[3] * b[3];
        let result_e1  = a[0] * b[1] + a[1] * b[0] - a[2] * b[3] + a[3] * b[2];
        let result_e2  = a[0] * b[2] + a[1] * b[3] + a[2] * b[0] - a[3] * b[1];
        let result_e12 = a[0] * b[3] + a[1] * b[2] - a[2] * b[1] + a[3] * b[0];

        EuclideanMultivector2::new(result_1, result_e1, result_e2, result_e12)
    }
}

impl<'a, 'b, S> Mul<&'b EuclideanMultivector2<S>> for &'a EuclideanMultivector2<S>
where
    S: Scalar
{
    type Output = EuclideanMultivector2<S>;

    fn mul(self, b: &'b EuclideanMultivector2<S>) -> Self::Output {
        let a = self;
        let result_1   = a[0] * b[0] + a[1] * b[1] + a[2] * b[2] - a[3] * b[3];
        let result_e1  = a[0] * b[1] + a[1] * b[0] - a[2] * b[3] + a[3] * b[2];
        let result_e2  = a[0] * b[2] + a[1] * b[3] + a[2] * b[0] - a[3] * b[1];
        let result_e12 = a[0] * b[3] + a[1] * b[2] - a[2] * b[1] + a[3] * b[0];

        EuclideanMultivector2::new(result_1, result_e1, result_e2, result_e12)
    }
}

impl<S> BitXor<EuclideanMultivector2<S>> for EuclideanMultivector2<S> 
where
    S: Scalar
{
    type Output = EuclideanMultivector2<S>;

    fn bitxor(self, b: EuclideanMultivector2<S>) -> Self::Output {
        let a = self;
        let result_1   = a[0] * b[0];
        let result_e1  = a[0] * b[1] + a[1] * b[0];
        let result_e2  = a[0] * b[2] + a[2] * b[0];
        let result_e12 = a[0] * b[3] + a[1] * b[2] - a[2] * b[1] + a[3] * b[0];
        
        EuclideanMultivector2::new(result_1, result_e1, result_e2, result_e12)
    }
}

impl<S> BitXor<&EuclideanMultivector2<S>> for EuclideanMultivector2<S> 
where
    S: Scalar
{
    type Output = EuclideanMultivector2<S>;

    fn bitxor(self, b: &EuclideanMultivector2<S>) -> Self::Output {
        let a = self;
        let result_1   = a[0] * b[0];
        let result_e1  = a[0] * b[1] + a[1] * b[0];
        let result_e2  = a[0] * b[2] + a[2] * b[0];
        let result_e12 = a[0] * b[3] + a[1] * b[2] - a[2] * b[1] + a[3] * b[0];
        
        EuclideanMultivector2::new(result_1, result_e1, result_e2, result_e12)
    }
}

impl<S> BitXor<EuclideanMultivector2<S>> for &EuclideanMultivector2<S> 
where
    S: Scalar
{
    type Output = EuclideanMultivector2<S>;

    fn bitxor(self, b: EuclideanMultivector2<S>) -> Self::Output {
        let a = self;
        let result_1   = a[0] * b[0];
        let result_e1  = a[0] * b[1] + a[1] * b[0];
        let result_e2  = a[0] * b[2] + a[2] * b[0];
        let result_e12 = a[0] * b[3] + a[1] * b[2] - a[2] * b[1] + a[3] * b[0];
        
        EuclideanMultivector2::new(result_1, result_e1, result_e2, result_e12)
    }
}

impl<'a, 'b, S> BitXor<&'b EuclideanMultivector2<S>> for &'a EuclideanMultivector2<S> 
where
    S: Scalar
{
    type Output = EuclideanMultivector2<S>;

    fn bitxor(self, b: &'b EuclideanMultivector2<S>) -> Self::Output {
        let a = self;
        let result_1   = a[0] * b[0];
        let result_e1  = a[0] * b[1] + a[1] * b[0];
        let result_e2  = a[0] * b[2] + a[2] * b[0];
        let result_e12 = a[0] * b[3] + a[1] * b[2] - a[2] * b[1] + a[3] * b[0];
        
        EuclideanMultivector2::new(result_1, result_e1, result_e2, result_e12)
    }
}

impl<S> BitOr<EuclideanMultivector2<S>> for EuclideanMultivector2<S> 
where
    S: Scalar
{
    type Output = EuclideanMultivector2<S>;

    fn bitor(self, b: EuclideanMultivector2<S>) -> Self::Output {
        let a = self;
        let result_1   = a[0] * b[0] + a[1] * b[1] + a[2] * b[2] - a[3] * b[3];
        let result_e1  = a[0] * b[1] + a[1] * b[0] - a[2] * b[3] + a[3] * b[2];
        let result_e2  = a[0] * b[2] + a[1] * b[3] + a[2] * b[0] - a[3] * b[1];
        let result_e12 = a[0] * b[3] + a[3] * b[0];
        
        EuclideanMultivector2::new(result_1, result_e1, result_e2, result_e12)
    }
}

impl<S> BitOr<&EuclideanMultivector2<S>> for EuclideanMultivector2<S> 
where
    S: Scalar
{
    type Output = EuclideanMultivector2<S>;

    fn bitor(self, b: &EuclideanMultivector2<S>) -> Self::Output {
        let a = self;
        let result_1   = a[0] * b[0] + a[1] * b[1] + a[2] * b[2] - a[3] * b[3];
        let result_e1  = a[0] * b[1] + a[1] * b[0] - a[2] * b[3] + a[3] * b[2];
        let result_e2  = a[0] * b[2] + a[1] * b[3] + a[2] * b[0] - a[3] * b[1];
        let result_e12 = a[0] * b[3] + a[3] * b[0];
        
        EuclideanMultivector2::new(result_1, result_e1, result_e2, result_e12)
    }
}

impl<S> BitOr<EuclideanMultivector2<S>> for &EuclideanMultivector2<S> 
where
    S: Scalar
{
    type Output = EuclideanMultivector2<S>;

    fn bitor(self, b: EuclideanMultivector2<S>) -> Self::Output {
        let a = self;
        let result_1   = a[0] * b[0] + a[1] * b[1] + a[2] * b[2] - a[3] * b[3];
        let result_e1  = a[0] * b[1] + a[1] * b[0] - a[2] * b[3] + a[3] * b[2];
        let result_e2  = a[0] * b[2] + a[1] * b[3] + a[2] * b[0] - a[3] * b[1];
        let result_e12 = a[0] * b[3] + a[3] * b[0];
        
        EuclideanMultivector2::new(result_1, result_e1, result_e2, result_e12)
    }
}

impl<'a, 'b, S> BitOr<&'b EuclideanMultivector2<S>> for &'a EuclideanMultivector2<S> 
where
    S: Scalar
{
    type Output = EuclideanMultivector2<S>;

    fn bitor(self, b: &'b EuclideanMultivector2<S>) -> Self::Output {
        let a = self;
        let result_1   = a[0] * b[0] + a[1] * b[1] + a[2] * b[2] - a[3] * b[3];
        let result_e1  = a[0] * b[1] + a[1] * b[0] - a[2] * b[3] + a[3] * b[2];
        let result_e2  = a[0] * b[2] + a[1] * b[3] + a[2] * b[0] - a[3] * b[1];
        let result_e12 = a[0] * b[3] + a[3] * b[0];
        
        EuclideanMultivector2::new(result_1, result_e1, result_e2, result_e12)
    }
}

impl<S> Add<EuclideanMultivector2<S>> for EuclideanMultivector2<S>
where
    S: Scalar
{
    type Output = EuclideanMultivector2<S>;

    fn add(self, b: EuclideanMultivector2<S>) -> Self::Output {
        let a = self;
        let result_1   = a[0] + b[0];
        let result_e1  = a[1] + b[1];
        let result_e2  = a[2] + b[2];
        let result_e12 = a[3] + b[3];
        
        EuclideanMultivector2::new(result_1, result_e1, result_e2, result_e12)
    }
}

impl<S> Add<&EuclideanMultivector2<S>> for EuclideanMultivector2<S>
where
    S: Scalar
{
    type Output = EuclideanMultivector2<S>;

    fn add(self, b: &EuclideanMultivector2<S>) -> Self::Output {
        let a = self;
        let result_1   = a[0] + b[0];
        let result_e1  = a[1] + b[1];
        let result_e2  = a[2] + b[2];
        let result_e12 = a[3] + b[3];
        
        EuclideanMultivector2::new(result_1, result_e1, result_e2, result_e12)
    }
}

impl<S> Add<EuclideanMultivector2<S>> for &EuclideanMultivector2<S>
where
    S: Scalar
{
    type Output = EuclideanMultivector2<S>;

    fn add(self, b: EuclideanMultivector2<S>) -> Self::Output {
        let a = self;
        let result_1   = a[0] + b[0];
        let result_e1  = a[1] + b[1];
        let result_e2  = a[2] + b[2];
        let result_e12 = a[3] + b[3];
        
        EuclideanMultivector2::new(result_1, result_e1, result_e2, result_e12)
    }
}

impl<'a, 'b, S> Add<&'b EuclideanMultivector2<S>> for &'a EuclideanMultivector2<S>
where
    S: Scalar
{
    type Output = EuclideanMultivector2<S>;

    fn add(self, b: &'b EuclideanMultivector2<S>) -> Self::Output {
        let a = self;
        let result_1   = a[0] + b[0];
        let result_e1  = a[1] + b[1];
        let result_e2  = a[2] + b[2];
        let result_e12 = a[3] + b[3];
        
        EuclideanMultivector2::new(result_1, result_e1, result_e2, result_e12)
    }
}

impl<S> Sub<EuclideanMultivector2<S>> for EuclideanMultivector2<S>
where
    S: Scalar
{
    type Output = EuclideanMultivector2<S>;

    fn sub(self, b: EuclideanMultivector2<S>) -> Self::Output {
        let a = self;
        let result_1   = a[0] - b[0];
        let result_e1  = a[1] - b[1];
        let result_e2  = a[2] - b[2];
        let result_e12 = a[3] - b[3];
        
        EuclideanMultivector2::new(result_1, result_e1, result_e2, result_e12)
    }
}

impl<S> Sub<&EuclideanMultivector2<S>> for EuclideanMultivector2<S>
where
    S: Scalar
{
    type Output = EuclideanMultivector2<S>;

    fn sub(self, b: &EuclideanMultivector2<S>) -> Self::Output {
        let a = self;
        let result_1   = a[0] - b[0];
        let result_e1  = a[1] - b[1];
        let result_e2  = a[2] - b[2];
        let result_e12 = a[3] - b[3];
        
        EuclideanMultivector2::new(result_1, result_e1, result_e2, result_e12)
    }
}

impl<S> Sub<EuclideanMultivector2<S>> for &EuclideanMultivector2<S>
where
    S: Scalar
{
    type Output = EuclideanMultivector2<S>;

    fn sub(self, b: EuclideanMultivector2<S>) -> Self::Output {
        let a = self;
        let result_1   = a[0] - b[0];
        let result_e1  = a[1] - b[1];
        let result_e2  = a[2] - b[2];
        let result_e12 = a[3] - b[3];
        
        EuclideanMultivector2::new(result_1, result_e1, result_e2, result_e12)
    }
}

impl<'a, 'b, S> Sub<&'b EuclideanMultivector2<S>> for &'a EuclideanMultivector2<S>
where
    S: Scalar
{
    type Output = EuclideanMultivector2<S>;

    fn sub(self, b: &'b EuclideanMultivector2<S>) -> Self::Output {
        let a = self;
        let result_1   = a[0] - b[0];
        let result_e1  = a[1] - b[1];
        let result_e2  = a[2] - b[2];
        let result_e12 = a[3] - b[3];
        
        EuclideanMultivector2::new(result_1, result_e1, result_e2, result_e12)
    }
}

impl<S> Mul<S> for EuclideanMultivector2<S> 
where
    S: Scalar
{
    type Output = EuclideanMultivector2<S>;

    fn mul(self, b: S) -> Self::Output {
        let a = self;
        let result_1   = a[0] * b;
        let result_e1  = a[1] * b;
        let result_e2  = a[2] * b;
        let result_e12 = a[3] * b;
        
        EuclideanMultivector2::new(result_1, result_e1, result_e2, result_e12)
    }
}

impl<S> Mul<S> for &EuclideanMultivector2<S> 
where
    S: Scalar
{
    type Output = EuclideanMultivector2<S>;

    fn mul(self, b: S) -> Self::Output {
        let a = self;
        let result_1   = a[0] * b;
        let result_e1  = a[1] * b;
        let result_e2  = a[2] * b;
        let result_e12 = a[3] * b;
        
        EuclideanMultivector2::new(result_1, result_e1, result_e2, result_e12)
    }
}

impl<S> Add<S> for EuclideanMultivector2<S>
where
    S: Scalar
{
    type Output = EuclideanMultivector2<S>;

    fn add(self, b: S) -> Self::Output {
        let a = self;
        let result_1   = a[0] + b;
        let result_e1  = a[1];
        let result_e2  = a[2];
        let result_e12 = a[3];
        
        EuclideanMultivector2::new(result_1, result_e1, result_e2, result_e12)
    }
}

impl<S> Add<S> for &EuclideanMultivector2<S>
where
    S: Scalar
{
    type Output = EuclideanMultivector2<S>;

    fn add(self, b: S) -> Self::Output {
        let a = self;
        let result_1   = a[0] + b;
        let result_e1  = a[1];
        let result_e2  = a[2];
        let result_e12 = a[3];
        
        EuclideanMultivector2::new(result_1, result_e1, result_e2, result_e12)
    }
}

impl<S> Sub<S> for EuclideanMultivector2<S>
where
    S: Scalar
{
    type Output = EuclideanMultivector2<S>;

    fn sub(self, b: S) -> Self::Output {
        let a = self;
        let result_1   = a[0] - b;
        let result_e1  = a[1];
        let result_e2  = a[2];
        let result_e12 = a[3];
        
        EuclideanMultivector2::new(result_1, result_e1, result_e2, result_e12)
    }
}

impl<S> Sub<S> for &EuclideanMultivector2<S>
where
    S: Scalar
{
    type Output = EuclideanMultivector2<S>;

    fn sub(self, b: S) -> Self::Output {
        let a = self;
        let result_1   = a[0] - b;
        let result_e1  = a[1];
        let result_e2  = a[2];
        let result_e12 = a[3];
        
        EuclideanMultivector2::new(result_1, result_e1, result_e2, result_e12)
    }
}

impl<S> EuclideanMultivector2<S> 
where
    S: ScalarFloat
{
    pub fn magnitude_squared(&self) -> S {
        let scalar_part = (self * self.conjugate())[0];

        scalar_part.abs()
    }

    pub fn magnitude(&self) -> S {
        self.magnitude_squared().sqrt()
    }

    pub fn imagnitude_squared(&self) -> S {
        self.dual().magnitude_squared()
    }

    pub fn imagnitude(&self) -> S {
        self.dual().magnitude()
    }

    pub fn normalize(&self) -> Self {
        self * (S::one() / self.magnitude())
    }
    
    pub fn normalize_to(&self, magnitude: S) -> Self {
        self * (magnitude / self.magnitude())
    }

    pub fn distance_squared(&self, other: &Self) -> S {
        (self - other).magnitude_squared()
    }

    pub fn distance(&self, other: &Self) -> S {
        (self - other).magnitude()
    }
}

impl<S> EuclideanMultivector2<S>
where
    S: ScalarFloat
{
    fn inverse_unchecked(&self) -> Self {
        let magnitude_squared = self.magnitude_squared();
        let inv_magnitude_squared = S::one() / magnitude_squared;

        self.reverse() * inv_magnitude_squared
    }

    pub fn inverse(&self) -> Option<Self> {
        let magnitude_squared = self.magnitude_squared();
        if magnitude_squared.is_zero() {
            None
        } else {
            let inv_magnitude_squared = S::one() / magnitude_squared;
            Some(self.reverse() * inv_magnitude_squared)
        }
    }
}

impl<S> Div<S> for EuclideanMultivector2<S>
where
    S: ScalarFloat
{
    type Output = EuclideanMultivector2<S>;

    fn div(self, other: S) -> Self::Output {
        let one_over_other = S::one() / other;
        let result_1 =   self.data[0] * one_over_other;
        let result_e1 =  self.data[1] * one_over_other;
        let result_e2 =  self.data[2] * one_over_other;
        let result_e12 = self.data[3] * one_over_other;

        EuclideanMultivector2::new(result_1, result_e1, result_e2, result_e12)
    }
}

impl<S> Div<S> for &EuclideanMultivector2<S>
where
    S: ScalarFloat
{
    type Output = EuclideanMultivector2<S>;

    fn div(self, other: S) -> Self::Output {
        let one_over_other = S::one() / other;
        let result_1 =   self.data[0] * one_over_other;
        let result_e1 =  self.data[1] * one_over_other;
        let result_e2 =  self.data[2] * one_over_other;
        let result_e12 = self.data[3] * one_over_other;

        EuclideanMultivector2::new(result_1, result_e1, result_e2, result_e12)
    }
}

impl<S> Div<EuclideanMultivector2<S>> for EuclideanMultivector2<S>
where
    S: ScalarFloat
{
    type Output = EuclideanMultivector2<S>;

    fn div(self, other: EuclideanMultivector2<S>) -> Self::Output {
        self * other.inverse_unchecked()
    }
}

impl<S> Div<&EuclideanMultivector2<S>> for EuclideanMultivector2<S>
where
    S: ScalarFloat
{
    type Output = EuclideanMultivector2<S>;

    fn div(self, other: &EuclideanMultivector2<S>) -> Self::Output {
        self * other.inverse_unchecked()
    }
}

impl<S> Div<EuclideanMultivector2<S>> for &EuclideanMultivector2<S>
where
    S: ScalarFloat
{
    type Output = EuclideanMultivector2<S>;

    fn div(self, other: EuclideanMultivector2<S>) -> Self::Output {
        self * other.inverse_unchecked()
    }
}

impl<'a, 'b, S> Div<&'b EuclideanMultivector2<S>> for &'a EuclideanMultivector2<S>
where
    S: ScalarFloat
{
    type Output = EuclideanMultivector2<S>;

    fn div(self, other: &'b EuclideanMultivector2<S>) -> Self::Output {
        self * other.inverse_unchecked()
    }
}

impl_coords!(E2ga, { scalar, e1, e2, e12 });
impl_coords_deref!(EuclideanMultivector2, E2ga);

