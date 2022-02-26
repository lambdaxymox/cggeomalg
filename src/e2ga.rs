use crate::scalar::{
    Scalar,
    ScalarSigned,
    ScalarFloat,
};
use crate::{
    impl_coords,
    impl_coords_deref,
};
use core::ops;
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
    #[inline]
    pub fn new(scalar: S, e1: S, e2: S, e12: S) -> Self {
        Self {
            data: [scalar, e1, e2, e12]
        }
    }

    /// Get a pointer to the underlying component array.
    #[inline]
    pub fn as_ptr(&self) -> *const S {
        &self.data[0]
    }

    /// Get a mutable pointer to the underlying component array.
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
    pub fn zero() -> Self {
        Self {
            data: [S::zero(), S::zero(), S::zero(), S::zero()],
        }
    }

    #[inline]
    pub fn is_zero(&self) -> bool {
        self.data[0].is_zero() &&
        self.data[1].is_zero() &&
        self.data[2].is_zero() &&
        self.data[3].is_zero()
    }

    /// Construct a new multivector from the scalar part only.
    /// 
    /// A scalar is a multivector whose vector, bivector, etc. components are
    /// all zero.
    /// 
    /// # Example
    /// 
    /// ```
    /// # use cggeomalg::e2ga::{
    /// #     EuclideanMultivector2,
    /// # };
    /// #
    /// let scalar_part = 2;
    /// let scalar = EuclideanMultivector2::from_scalar(scalar_part);
    /// 
    /// assert_eq!(scalar.scalar, scalar_part);
    /// assert_eq!(scalar.e1, 0);
    /// assert_eq!(scalar.e2, 0);
    /// assert_eq!(scalar.e12, 0);
    /// ```
    #[inline]
    pub fn from_scalar(scalar: S) -> Self {
        Self::new(scalar, S::zero(), S::zero(), S::zero())
    }

    /// Returns the number of components in a multivector.
    #[inline]
    pub fn len(&self) -> usize {
        self.data.len()
    }

    #[inline]
    pub fn unit_scalar() -> Self {
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

    /// Returns the unit volume elements for `G2`. This is a synonym from `unit_e12`.
    #[inline(always)]
    pub fn pseudoscalar() -> Self {
        Self::unit_e12()
    }

    /// Project the multivector onto the grade `grade`.
    /// 
    /// Return a multivector where the components of each grade other than 
    /// input grade are zero. For each grade larger than the dimension of the 
    /// underlying vector space, the grade projection is always zero. In this 
    /// case, any grade projection onto a grade larger than two will be zero.
    /// 
    /// # Example
    /// ```
    /// # use cggeomalg::e2ga::{
    /// #     EuclideanMultivector2,
    /// # };
    /// #
    /// let mv: EuclideanMultivector2<isize> = EuclideanMultivector2::new(1, 1, 1, 1);
    /// let expected_0 = EuclideanMultivector2::new(1, 0, 0, 0);
    /// let mv_0 = mv.grade(0);
    /// let expected_1 = EuclideanMultivector2::new(0, 1, 1, 0);
    /// let mv_1 = mv.grade(1);
    /// let expected_2 = EuclideanMultivector2::new(0, 0, 0, 1);
    /// let mv_2 = mv.grade(2);
    /// 
    /// assert_eq!(mv_0, expected_0);
    /// assert_eq!(mv_1, expected_1);
    /// assert_eq!(mv_2, expected_2);
    /// 
    /// // Any grade larger than two should be zero.
    /// let zero: EuclideanMultivector2<isize> = EuclideanMultivector2::zero();
    /// assert_eq!(mv.grade(3), zero);
    /// assert_eq!(mv.grade(usize::MAX), zero);
    /// ```
    #[inline]
    pub fn grade(&self, grade: usize) -> Self {
        match grade {
            0 => Self::new(self.data[0], S::zero(), S::zero(), S::zero()),
            1 => Self::new(S::zero(), self.data[1], self.data[2], S::zero()),
            2 => Self::new(S::zero(), S::zero(), S::zero(), self.data[3]),
            _ => Self::zero()
        }
    }
}

impl<S> ops::Index<usize> for EuclideanMultivector2<S> 
where
    S: Scalar
{
    type Output = S;

    #[inline]
    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}

impl<S> ops::IndexMut<usize> for EuclideanMultivector2<S> 
where
    S: Scalar
{
    #[inline]
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
            "{} + {} e1 + {} e2 + {} e12",
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
        // self.data[0] =  self.data[0];
        self.data[1] = -self.data[1];
        self.data[2] = -self.data[2];
        self.data[3] = -self.data[3];
    }

    pub fn involute(&self) -> Self {
        Self::new(self.data[0], -self.data[1], -self.data[2], self.data[3])
    }

    pub fn involute_mut(&mut self) {
        // self.data[0] =  self.data[0];
        self.data[1] = -self.data[1];
        self.data[2] = -self.data[2];
        // self.data[3] =  self.data[3];
    }

    pub fn dual(&self) -> Self {
        Self::new(-self.data[3], -self.data[2], self.data[1], self.data[0])
    }

    pub fn dual_mut(&mut self) {
        let mut result = Self::zero();
        result.data[0] = -self.data[3];
        result.data[1] = -self.data[2];
        result.data[2] =  self.data[1];
        result.data[3] =  self.data[0];
        *self = result;
    }

    #[inline]
    pub fn inv_pseudoscalar() -> Self {
        -Self::unit_e12()
    }
}

impl<S> ops::Not for EuclideanMultivector2<S> 
where
    S: ScalarSigned
{
    type Output = EuclideanMultivector2<S>;

    #[inline]
    fn not(self) -> Self::Output {
        let mut result = Self::zero();
        result.data[0] = -self.data[3];
        result.data[1] = -self.data[2];
        result.data[2] =  self.data[1];
        result.data[3] =  self.data[0];
        
        result
    }
}

impl<S> ops::Not for &EuclideanMultivector2<S> 
where
    S: ScalarSigned
{
    type Output = EuclideanMultivector2<S>;

    #[inline]
    fn not(self) -> Self::Output {
        let mut result = EuclideanMultivector2::zero();
        result.data[0] = -self.data[3];
        result.data[1] = -self.data[2];
        result.data[2] =  self.data[1];
        result.data[3] =  self.data[0];
        
        result
    }
}

impl<S> ops::Neg for EuclideanMultivector2<S>
where
    S: ScalarSigned
{
    type Output = EuclideanMultivector2<S>;

    #[inline]
    fn neg(self) -> Self::Output {
        let result_1   = -self.data[0];
        let result_e1  = -self.data[1];
        let result_e2  = -self.data[2];
        let result_e12 = -self.data[3];

        EuclideanMultivector2::new(result_1, result_e1, result_e2, result_e12)
    }
}

impl<S> ops::Neg for &EuclideanMultivector2<S>
where
    S: ScalarSigned
{
    type Output = EuclideanMultivector2<S>;

    #[inline]
    fn neg(self) -> Self::Output {
        let result_1   = -self.data[0];
        let result_e1  = -self.data[1];
        let result_e2  = -self.data[2];
        let result_e12 = -self.data[3];

        EuclideanMultivector2::new(result_1, result_e1, result_e2, result_e12)
    }
}

impl<S> ops::Mul<EuclideanMultivector2<S>> for EuclideanMultivector2<S>
where
    S: Scalar
{
    type Output = EuclideanMultivector2<S>;

    #[inline]
    fn mul(self, other: EuclideanMultivector2<S>) -> Self::Output {
        let a = self;
        let b = other;
        let result_1   = a[0] * b[0] + a[1] * b[1] + a[2] * b[2] - a[3] * b[3];
        let result_e1  = a[0] * b[1] + a[1] * b[0] - a[2] * b[3] + a[3] * b[2];
        let result_e2  = a[0] * b[2] + a[1] * b[3] + a[2] * b[0] - a[3] * b[1];
        let result_e12 = a[0] * b[3] + a[1] * b[2] - a[2] * b[1] + a[3] * b[0];

        EuclideanMultivector2::new(result_1, result_e1, result_e2, result_e12)
    }
}

impl<S> ops::Mul<&EuclideanMultivector2<S>> for EuclideanMultivector2<S>
where
    S: Scalar
{
    type Output = EuclideanMultivector2<S>;

    #[inline]
    fn mul(self, other: &EuclideanMultivector2<S>) -> Self::Output {
        let a = self;
        let b = other;
        let result_1   = a[0] * b[0] + a[1] * b[1] + a[2] * b[2] - a[3] * b[3];
        let result_e1  = a[0] * b[1] + a[1] * b[0] - a[2] * b[3] + a[3] * b[2];
        let result_e2  = a[0] * b[2] + a[1] * b[3] + a[2] * b[0] - a[3] * b[1];
        let result_e12 = a[0] * b[3] + a[1] * b[2] - a[2] * b[1] + a[3] * b[0];

        EuclideanMultivector2::new(result_1, result_e1, result_e2, result_e12)
    }
}

impl<S> ops::Mul<EuclideanMultivector2<S>> for &EuclideanMultivector2<S>
where
    S: Scalar
{
    type Output = EuclideanMultivector2<S>;

    #[inline]
    fn mul(self, other: EuclideanMultivector2<S>) -> Self::Output {
        let a = self;
        let b = other;
        let result_1   = a[0] * b[0] + a[1] * b[1] + a[2] * b[2] - a[3] * b[3];
        let result_e1  = a[0] * b[1] + a[1] * b[0] - a[2] * b[3] + a[3] * b[2];
        let result_e2  = a[0] * b[2] + a[1] * b[3] + a[2] * b[0] - a[3] * b[1];
        let result_e12 = a[0] * b[3] + a[1] * b[2] - a[2] * b[1] + a[3] * b[0];

        EuclideanMultivector2::new(result_1, result_e1, result_e2, result_e12)
    }
}

impl<'a, 'b, S> ops::Mul<&'b EuclideanMultivector2<S>> for &'a EuclideanMultivector2<S>
where
    S: Scalar
{
    type Output = EuclideanMultivector2<S>;

    #[inline]
    fn mul(self, other: &'b EuclideanMultivector2<S>) -> Self::Output {
        let a = self;
        let b = other;
        let result_1   = a[0] * b[0] + a[1] * b[1] + a[2] * b[2] - a[3] * b[3];
        let result_e1  = a[0] * b[1] + a[1] * b[0] - a[2] * b[3] + a[3] * b[2];
        let result_e2  = a[0] * b[2] + a[1] * b[3] + a[2] * b[0] - a[3] * b[1];
        let result_e12 = a[0] * b[3] + a[1] * b[2] - a[2] * b[1] + a[3] * b[0];

        EuclideanMultivector2::new(result_1, result_e1, result_e2, result_e12)
    }
}

impl<S> ops::BitXor<EuclideanMultivector2<S>> for EuclideanMultivector2<S> 
where
    S: Scalar
{
    type Output = EuclideanMultivector2<S>;

    #[inline]
    fn bitxor(self, other: EuclideanMultivector2<S>) -> Self::Output {
        let a = self;
        let b = other;
        let result_1   = a[0] * b[0];
        let result_e1  = a[0] * b[1] + a[1] * b[0];
        let result_e2  = a[0] * b[2] + a[2] * b[0];
        let result_e12 = a[0] * b[3] + a[1] * b[2] - a[2] * b[1] + a[3] * b[0];
        
        EuclideanMultivector2::new(result_1, result_e1, result_e2, result_e12)
    }
}

impl<S> ops::BitXor<&EuclideanMultivector2<S>> for EuclideanMultivector2<S> 
where
    S: Scalar
{
    type Output = EuclideanMultivector2<S>;

    #[inline]
    fn bitxor(self, other: &EuclideanMultivector2<S>) -> Self::Output {
        let a = self;
        let b = other;
        let result_1   = a[0] * b[0];
        let result_e1  = a[0] * b[1] + a[1] * b[0];
        let result_e2  = a[0] * b[2] + a[2] * b[0];
        let result_e12 = a[0] * b[3] + a[1] * b[2] - a[2] * b[1] + a[3] * b[0];
        
        EuclideanMultivector2::new(result_1, result_e1, result_e2, result_e12)
    }
}

impl<S> ops::BitXor<EuclideanMultivector2<S>> for &EuclideanMultivector2<S> 
where
    S: Scalar
{
    type Output = EuclideanMultivector2<S>;

    #[inline]
    fn bitxor(self, other: EuclideanMultivector2<S>) -> Self::Output {
        let a = self;
        let b = other;
        let result_1   = a[0] * b[0];
        let result_e1  = a[0] * b[1] + a[1] * b[0];
        let result_e2  = a[0] * b[2] + a[2] * b[0];
        let result_e12 = a[0] * b[3] + a[1] * b[2] - a[2] * b[1] + a[3] * b[0];
        
        EuclideanMultivector2::new(result_1, result_e1, result_e2, result_e12)
    }
}

impl<'a, 'b, S> ops::BitXor<&'b EuclideanMultivector2<S>> for &'a EuclideanMultivector2<S> 
where
    S: Scalar
{
    type Output = EuclideanMultivector2<S>;

    #[inline]
    fn bitxor(self, other: &'b EuclideanMultivector2<S>) -> Self::Output {
        let a = self;
        let b = other;
        let result_1   = a[0] * b[0];
        let result_e1  = a[0] * b[1] + a[1] * b[0];
        let result_e2  = a[0] * b[2] + a[2] * b[0];
        let result_e12 = a[0] * b[3] + a[1] * b[2] - a[2] * b[1] + a[3] * b[0];
        
        EuclideanMultivector2::new(result_1, result_e1, result_e2, result_e12)
    }
}

impl<S> ops::BitOr<EuclideanMultivector2<S>> for EuclideanMultivector2<S> 
where
    S: Scalar
{
    type Output = EuclideanMultivector2<S>;

    #[inline]
    fn bitor(self, other: EuclideanMultivector2<S>) -> Self::Output {
        let a = self;
        let b = other;
        // All of the results of calculating a | b := (~a) * b are included for reference.
        let result_1   = a[0] * b[0] + a[1] * b[1] + a[2] * b[2] + a[3] * b[3];
        // let result_e1  = a[0] * b[1] + a[1] * b[0] - a[2] * b[3] - a[3] * b[2];
        // let result_e2  = a[0] * b[2] + a[1] * b[3] + a[2] * b[0] + a[3] * b[1];
        // let result_e12 = a[0] * b[3] + a[1] * b[2] - a[2] * b[1] - a[3] * b[0];
        
        EuclideanMultivector2::new(result_1, S::zero(), S::zero(), S::zero())
    }
}

impl<S> ops::BitOr<&EuclideanMultivector2<S>> for EuclideanMultivector2<S> 
where
    S: Scalar
{
    type Output = EuclideanMultivector2<S>;

    #[inline]
    fn bitor(self, other: &EuclideanMultivector2<S>) -> Self::Output {
        let a = self;
        let b = other;
        // All of the results of calculating a | b := (~a) * b are included for reference.
        let result_1   = a[0] * b[0] + a[1] * b[1] + a[2] * b[2] + a[3] * b[3];
        // let result_e1  = a[0] * b[1] + a[1] * b[0] - a[2] * b[3] - a[3] * b[2];
        // let result_e2  = a[0] * b[2] + a[1] * b[3] + a[2] * b[0] + a[3] * b[1];
        // let result_e12 = a[0] * b[3] + a[1] * b[2] - a[2] * b[1] - a[3] * b[0];
        
        EuclideanMultivector2::new(result_1, S::zero(), S::zero(), S::zero())
    }
}

impl<S> ops::BitOr<EuclideanMultivector2<S>> for &EuclideanMultivector2<S> 
where
    S: Scalar
{
    type Output = EuclideanMultivector2<S>;

    #[inline]
    fn bitor(self, other: EuclideanMultivector2<S>) -> Self::Output {
        let a = self;
        let b = other;
        // All of the results of calculating a | b := (~a) * b are included for reference.
        let result_1   = a[0] * b[0] + a[1] * b[1] + a[2] * b[2] + a[3] * b[3];
        // let result_e1  = a[0] * b[1] + a[1] * b[0] - a[2] * b[3] - a[3] * b[2];
        // let result_e2  = a[0] * b[2] + a[1] * b[3] + a[2] * b[0] + a[3] * b[1];
        // let result_e12 = a[0] * b[3] + a[1] * b[2] - a[2] * b[1] - a[3] * b[0];
        
        EuclideanMultivector2::new(result_1, S::zero(), S::zero(), S::zero())
    }
}

impl<'a, 'b, S> ops::BitOr<&'b EuclideanMultivector2<S>> for &'a EuclideanMultivector2<S> 
where
    S: Scalar
{
    type Output = EuclideanMultivector2<S>;

    #[inline]
    fn bitor(self, other: &'b EuclideanMultivector2<S>) -> Self::Output {
        let a = self;
        let b = other;
        // All of the results of calculating a | b := (~a) * b are included for reference.
        let result_1   = a[0] * b[0] + a[1] * b[1] + a[2] * b[2] + a[3] * b[3];
        // let result_e1  = a[0] * b[1] + a[1] * b[0] - a[2] * b[3] - a[3] * b[2];
        // let result_e2  = a[0] * b[2] + a[1] * b[3] + a[2] * b[0] + a[3] * b[1];
        // let result_e12 = a[0] * b[3] + a[1] * b[2] - a[2] * b[1] - a[3] * b[0];
        
        EuclideanMultivector2::new(result_1, S::zero(), S::zero(), S::zero())
    }
}

impl<S> ops::Add<EuclideanMultivector2<S>> for EuclideanMultivector2<S>
where
    S: Scalar
{
    type Output = EuclideanMultivector2<S>;

    #[inline]
    fn add(self, other: EuclideanMultivector2<S>) -> Self::Output {
        let a = self;
        let b = other;
        let result_1   = a[0] + b[0];
        let result_e1  = a[1] + b[1];
        let result_e2  = a[2] + b[2];
        let result_e12 = a[3] + b[3];
        
        EuclideanMultivector2::new(result_1, result_e1, result_e2, result_e12)
    }
}

impl<S> ops::Add<&EuclideanMultivector2<S>> for EuclideanMultivector2<S>
where
    S: Scalar
{
    type Output = EuclideanMultivector2<S>;

    #[inline]
    fn add(self, other: &EuclideanMultivector2<S>) -> Self::Output {
        let a = self;
        let b = other;
        let result_1   = a[0] + b[0];
        let result_e1  = a[1] + b[1];
        let result_e2  = a[2] + b[2];
        let result_e12 = a[3] + b[3];
        
        EuclideanMultivector2::new(result_1, result_e1, result_e2, result_e12)
    }
}

impl<S> ops::Add<EuclideanMultivector2<S>> for &EuclideanMultivector2<S>
where
    S: Scalar
{
    type Output = EuclideanMultivector2<S>;

    #[inline]
    fn add(self, other: EuclideanMultivector2<S>) -> Self::Output {
        let a = self;
        let b = other;
        let result_1   = a[0] + b[0];
        let result_e1  = a[1] + b[1];
        let result_e2  = a[2] + b[2];
        let result_e12 = a[3] + b[3];
        
        EuclideanMultivector2::new(result_1, result_e1, result_e2, result_e12)
    }
}

impl<'a, 'b, S> ops::Add<&'b EuclideanMultivector2<S>> for &'a EuclideanMultivector2<S>
where
    S: Scalar
{
    type Output = EuclideanMultivector2<S>;

    #[inline]
    fn add(self, other: &'b EuclideanMultivector2<S>) -> Self::Output {
        let a = self;
        let b = other;
        let result_1   = a[0] + b[0];
        let result_e1  = a[1] + b[1];
        let result_e2  = a[2] + b[2];
        let result_e12 = a[3] + b[3];
        
        EuclideanMultivector2::new(result_1, result_e1, result_e2, result_e12)
    }
}

impl<S> ops::Sub<EuclideanMultivector2<S>> for EuclideanMultivector2<S>
where
    S: Scalar
{
    type Output = EuclideanMultivector2<S>;

    #[inline]
    fn sub(self, other: EuclideanMultivector2<S>) -> Self::Output {
        let a = self;
        let b = other;
        let result_1   = a[0] - b[0];
        let result_e1  = a[1] - b[1];
        let result_e2  = a[2] - b[2];
        let result_e12 = a[3] - b[3];
        
        EuclideanMultivector2::new(result_1, result_e1, result_e2, result_e12)
    }
}

impl<S> ops::Sub<&EuclideanMultivector2<S>> for EuclideanMultivector2<S>
where
    S: Scalar
{
    type Output = EuclideanMultivector2<S>;

    #[inline]
    fn sub(self, other: &EuclideanMultivector2<S>) -> Self::Output {
        let a = self;
        let b = other;
        let result_1   = a[0] - b[0];
        let result_e1  = a[1] - b[1];
        let result_e2  = a[2] - b[2];
        let result_e12 = a[3] - b[3];
        
        EuclideanMultivector2::new(result_1, result_e1, result_e2, result_e12)
    }
}

impl<S> ops::Sub<EuclideanMultivector2<S>> for &EuclideanMultivector2<S>
where
    S: Scalar
{
    type Output = EuclideanMultivector2<S>;

    #[inline]
    fn sub(self, other: EuclideanMultivector2<S>) -> Self::Output {
        let a = self;
        let b = other;
        let result_1   = a[0] - b[0];
        let result_e1  = a[1] - b[1];
        let result_e2  = a[2] - b[2];
        let result_e12 = a[3] - b[3];
        
        EuclideanMultivector2::new(result_1, result_e1, result_e2, result_e12)
    }
}

impl<'a, 'b, S> ops::Sub<&'b EuclideanMultivector2<S>> for &'a EuclideanMultivector2<S>
where
    S: Scalar
{
    type Output = EuclideanMultivector2<S>;

    #[inline]
    fn sub(self, other: &'b EuclideanMultivector2<S>) -> Self::Output {
        let a = self;
        let b = other;
        let result_1   = a[0] - b[0];
        let result_e1  = a[1] - b[1];
        let result_e2  = a[2] - b[2];
        let result_e12 = a[3] - b[3];
        
        EuclideanMultivector2::new(result_1, result_e1, result_e2, result_e12)
    }
}

impl<S> ops::Mul<S> for EuclideanMultivector2<S> 
where
    S: Scalar
{
    type Output = EuclideanMultivector2<S>;

    #[inline]
    fn mul(self, other: S) -> Self::Output {
        let a = self;
        let b = other;
        let result_1   = a[0] * b;
        let result_e1  = a[1] * b;
        let result_e2  = a[2] * b;
        let result_e12 = a[3] * b;
        
        EuclideanMultivector2::new(result_1, result_e1, result_e2, result_e12)
    }
}

impl<S> ops::Mul<S> for &EuclideanMultivector2<S> 
where
    S: Scalar
{
    type Output = EuclideanMultivector2<S>;

    #[inline]
    fn mul(self, other: S) -> Self::Output {
        let a = self;
        let b = other;
        let result_1   = a[0] * b;
        let result_e1  = a[1] * b;
        let result_e2  = a[2] * b;
        let result_e12 = a[3] * b;
        
        EuclideanMultivector2::new(result_1, result_e1, result_e2, result_e12)
    }
}

impl<S> ops::Add<S> for EuclideanMultivector2<S>
where
    S: Scalar
{
    type Output = EuclideanMultivector2<S>;

    #[inline]
    fn add(self, other: S) -> Self::Output {
        let a = self;
        let b = other;
        let result_1   = a[0] + b;
        let result_e1  = a[1];
        let result_e2  = a[2];
        let result_e12 = a[3];
        
        EuclideanMultivector2::new(result_1, result_e1, result_e2, result_e12)
    }
}

impl<S> ops::Add<S> for &EuclideanMultivector2<S>
where
    S: Scalar
{
    type Output = EuclideanMultivector2<S>;

    #[inline]
    fn add(self, other: S) -> Self::Output {
        let a = self;
        let b = other;
        let result_1   = a[0] + b;
        let result_e1  = a[1];
        let result_e2  = a[2];
        let result_e12 = a[3];
        
        EuclideanMultivector2::new(result_1, result_e1, result_e2, result_e12)
    }
}

impl<S> ops::Sub<S> for EuclideanMultivector2<S>
where
    S: Scalar
{
    type Output = EuclideanMultivector2<S>;

    #[inline]
    fn sub(self, other: S) -> Self::Output {
        let a = self;
        let b = other;
        let result_1   = a[0] - b;
        let result_e1  = a[1];
        let result_e2  = a[2];
        let result_e12 = a[3];
        
        EuclideanMultivector2::new(result_1, result_e1, result_e2, result_e12)
    }
}

impl<S> ops::Sub<S> for &EuclideanMultivector2<S>
where
    S: Scalar
{
    type Output = EuclideanMultivector2<S>;

    #[inline]
    fn sub(self, other: S) -> Self::Output {
        let a = self;
        let b = other;
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
        let scalar_part = (self * self.reverse())[0];

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
        let conjugate = self.conjugate();
        let self_times_conjugate = (self * conjugate)[0];

        conjugate / self_times_conjugate
    }

    #[inline]
    pub fn is_invertible(&self) -> bool {
        !self.magnitude_squared().is_zero()
    }

    pub fn inverse(&self) -> Option<Self> {
        let magnitude_squared = self.magnitude_squared();
        if magnitude_squared.is_zero() {
            None
        } else {
            let conjugate = self.conjugate();
            let self_times_conjugate = (self * conjugate)[0];

            Some(conjugate / self_times_conjugate)
        }
    }

    pub fn commutator(&self, other: &Self) -> Self {
        let self_times_other = self * other;
        let other_times_self = other * self;
        let one_over_two = S::one() / (S::one() + S::one());

        (self_times_other - other_times_self) * one_over_two
    }

    #[inline(always)]
    pub fn x(&self, other: &Self) -> Self {
        self.commutator(other)
    }

    pub fn anticommutator(&self, other: &Self) -> Self {
        let self_times_other = self * other;
        let other_times_self = other * self;
        let one_over_two = S::one() / (S::one() + S::one());

        (self_times_other + other_times_self) * one_over_two
    }
}

impl<S> ops::Div<S> for EuclideanMultivector2<S>
where
    S: ScalarFloat
{
    type Output = EuclideanMultivector2<S>;

    #[inline]
    fn div(self, other: S) -> Self::Output {
        let one_over_other = S::one() / other;
        let result_1   = self.data[0] * one_over_other;
        let result_e1  = self.data[1] * one_over_other;
        let result_e2  = self.data[2] * one_over_other;
        let result_e12 = self.data[3] * one_over_other;

        EuclideanMultivector2::new(result_1, result_e1, result_e2, result_e12)
    }
}

impl<S> ops::Div<S> for &EuclideanMultivector2<S>
where
    S: ScalarFloat
{
    type Output = EuclideanMultivector2<S>;

    #[inline]
    fn div(self, other: S) -> Self::Output {
        let one_over_other = S::one() / other;
        let result_1   = self.data[0] * one_over_other;
        let result_e1  = self.data[1] * one_over_other;
        let result_e2  = self.data[2] * one_over_other;
        let result_e12 = self.data[3] * one_over_other;

        EuclideanMultivector2::new(result_1, result_e1, result_e2, result_e12)
    }
}

impl<S> ops::Div<EuclideanMultivector2<S>> for EuclideanMultivector2<S>
where
    S: ScalarFloat
{
    type Output = EuclideanMultivector2<S>;

    #[allow(clippy::suspicious_arithmetic_impl)]
    #[inline]
    fn div(self, other: EuclideanMultivector2<S>) -> Self::Output {
        self * other.inverse_unchecked()
    }
}

impl<S> ops::Div<&EuclideanMultivector2<S>> for EuclideanMultivector2<S>
where
    S: ScalarFloat
{
    type Output = EuclideanMultivector2<S>;

    #[allow(clippy::suspicious_arithmetic_impl)]
    #[inline]
    fn div(self, other: &EuclideanMultivector2<S>) -> Self::Output {
        self * other.inverse_unchecked()
    }
}

impl<S> ops::Div<EuclideanMultivector2<S>> for &EuclideanMultivector2<S>
where
    S: ScalarFloat
{
    type Output = EuclideanMultivector2<S>;

    #[allow(clippy::suspicious_arithmetic_impl)]
    #[inline]
    fn div(self, other: EuclideanMultivector2<S>) -> Self::Output {
        self * other.inverse_unchecked()
    }
}

impl<'a, 'b, S> ops::Div<&'b EuclideanMultivector2<S>> for &'a EuclideanMultivector2<S>
where
    S: ScalarFloat
{
    type Output = EuclideanMultivector2<S>;

    #[allow(clippy::suspicious_arithmetic_impl)]
    #[inline]
    fn div(self, other: &'b EuclideanMultivector2<S>) -> Self::Output {
        self * other.inverse_unchecked()
    }
}

impl<S> ops::Shl<EuclideanMultivector2<S>> for EuclideanMultivector2<S>
where
    S: Scalar
{
    type Output = EuclideanMultivector2<S>;

    #[inline]
    fn shl(self, other: EuclideanMultivector2<S>) -> Self::Output {
        let a = self;
        let b = other;
        let result_1   = a[0] * b[0] + a[1] * b[1] + a[2] * b[2] - a[3] * b[3];
        let result_e1  = a[0] * b[1] - a[2] * b[3];
        let result_e2  = a[0] * b[2] + a[1] * b[3];
        let result_e12 = a[0] * b[3];

        EuclideanMultivector2::new(result_1, result_e1, result_e2, result_e12)
    }
}

impl<S> ops::Shl<&EuclideanMultivector2<S>> for EuclideanMultivector2<S>
where
    S: Scalar
{
    type Output = EuclideanMultivector2<S>;

    #[inline]
    fn shl(self, other: &EuclideanMultivector2<S>) -> Self::Output {
        let a = self;
        let b = other;
        let result_1   = a[0] * b[0] + a[1] * b[1] + a[2] * b[2] + a[3] * b[3];
        let result_e1  = a[0] * b[1] - a[2] * b[3];
        let result_e2  = a[0] * b[2] + a[1] * b[3];
        let result_e12 = a[0] * b[3];

        EuclideanMultivector2::new(result_1, result_e1, result_e2, result_e12)
    }
}

impl<S> ops::Shl<EuclideanMultivector2<S>> for &EuclideanMultivector2<S>
where
    S: Scalar
{
    type Output = EuclideanMultivector2<S>;

    #[inline]
    fn shl(self, other: EuclideanMultivector2<S>) -> Self::Output {
        let a = self;
        let b = other;
        let result_1   = a[0] * b[0] + a[1] * b[1] + a[2] * b[2] + a[3] * b[3];
        let result_e1  = a[0] * b[1] - a[2] * b[3];
        let result_e2  = a[0] * b[2] + a[1] * b[3];
        let result_e12 = a[0] * b[3];

        EuclideanMultivector2::new(result_1, result_e1, result_e2, result_e12)
    }
}

impl<'a, 'b, S> ops::Shl<&'b EuclideanMultivector2<S>> for &'a EuclideanMultivector2<S>
where
    S: Scalar
{
    type Output = EuclideanMultivector2<S>;

    #[inline]
    fn shl(self, other: &'b EuclideanMultivector2<S>) -> Self::Output {
        let a = self;
        let b = other;
        let result_1   = a[0] * b[0] + a[1] * b[1] + a[2] * b[2] + a[3] * b[3];
        let result_e1  = a[0] * b[1] - a[2] * b[3];
        let result_e2  = a[0] * b[2] + a[1] * b[3];
        let result_e12 = a[0] * b[3];

        EuclideanMultivector2::new(result_1, result_e1, result_e2, result_e12)
    }
}

impl<S> ops::Shr<EuclideanMultivector2<S>> for EuclideanMultivector2<S>
where
    S: Scalar
{
    type Output = EuclideanMultivector2<S>;

    #[inline]
    fn shr(self, other: EuclideanMultivector2<S>) -> Self::Output {
        let a = self;
        let b = other;
        let result_1   = a[0] * b[0] + a[1] * b[1] + a[2] * b[2] - a[3] * b[3];
        let result_e1  = a[1] * b[0] + a[3] * b[2];
        let result_e2  = a[2] * b[0] - a[3] * b[1];
        let result_e12 = a[3] * b[0];

        EuclideanMultivector2::new(result_1, result_e1, result_e2, result_e12)
    }   
}

impl<S> ops::Shr<&EuclideanMultivector2<S>> for EuclideanMultivector2<S>
where
    S: Scalar
{
    type Output = EuclideanMultivector2<S>;

    #[inline]
    fn shr(self, other: &EuclideanMultivector2<S>) -> Self::Output {
        let a = self;
        let b = other;
        let result_1   = a[0] * b[0] + a[1] * b[1] + a[2] * b[2] - a[3] * b[3];
        let result_e1  = a[1] * b[0] + a[3] * b[2];
        let result_e2  = a[2] * b[0] - a[3] * b[1];
        let result_e12 = a[3] * b[0];

        EuclideanMultivector2::new(result_1, result_e1, result_e2, result_e12)
    }
}

impl<S> ops::Shr<EuclideanMultivector2<S>> for &EuclideanMultivector2<S>
where
    S: Scalar
{
    type Output = EuclideanMultivector2<S>;

    #[inline]
    fn shr(self, other: EuclideanMultivector2<S>) -> Self::Output {
        let a = self;
        let b = other;
        let result_1   = a[0] * b[0] + a[1] * b[1] + a[2] * b[2] - a[3] * b[3];
        let result_e1  = a[1] * b[0] + a[3] * b[2];
        let result_e2  = a[2] * b[0] - a[3] * b[1];
        let result_e12 = a[3] * b[0];

        EuclideanMultivector2::new(result_1, result_e1, result_e2, result_e12)
    }
}

impl<'a, 'b, S> ops::Shr<&'b EuclideanMultivector2<S>> for &'a EuclideanMultivector2<S>
where
    S: Scalar
{
    type Output = EuclideanMultivector2<S>;

    #[inline]
    fn shr(self, other: &'b EuclideanMultivector2<S>) -> Self::Output {
        let a = self;
        let b = other;
        let result_1   = a[0] * b[0] + a[1] * b[1] + a[2] * b[2] - a[3] * b[3];
        let result_e1  = a[1] * b[0] + a[3] * b[2];
        let result_e2  = a[2] * b[0] - a[3] * b[1];
        let result_e12 = a[3] * b[0];

        EuclideanMultivector2::new(result_1, result_e1, result_e2, result_e12)
    }
}

impl_coords!(E2ga, { scalar, e1, e2, e12 });
impl_coords_deref!(EuclideanMultivector2, E2ga);

macro_rules! impl_scalar_multivector_add_sub_ops {
    ($Lhs:ty => $Rhs:ty => $Output:ty, { $scalar_index:expr }, { $($other_index:expr),* }) => {
        impl ops::Add<$Rhs> for $Lhs {
            type Output = $Output;

            #[inline]
            fn add(self, other: $Rhs) -> $Output {
                Self::Output::new(self + other[$scalar_index], $(other[$other_index]),* )
            }
        }

        impl ops::Add<&$Rhs> for $Lhs {
            type Output = $Output;

            #[inline]
            fn add(self, other: &$Rhs) -> $Output {
                Self::Output::new(self + other[$scalar_index], $(other[$other_index]),* )
            }
        }

        impl ops::Sub<$Rhs> for $Lhs {
            type Output = $Output;

            #[inline]
            fn sub(self, other: $Rhs) -> Self::Output {
                Self::Output::new(self + other[$scalar_index], $(other[$other_index]),* )
            }
        }

        impl ops::Sub<&$Rhs> for $Lhs {
            type Output = $Output;

            #[inline]
            fn sub(self, other: &$Rhs) -> Self::Output {
                Self::Output::new(self + other[$scalar_index], $(other[$other_index]),* )
            }
        }
    }
}

impl_scalar_multivector_add_sub_ops!(u8    => EuclideanMultivector2<u8> => EuclideanMultivector2<u8>,       {0}, {1, 2, 3});
impl_scalar_multivector_add_sub_ops!(u16   => EuclideanMultivector2<u16> => EuclideanMultivector2<u16>,     {0}, {1, 2, 3});
impl_scalar_multivector_add_sub_ops!(u32   => EuclideanMultivector2<u32> => EuclideanMultivector2<u32>,     {0}, {1, 2, 3});
impl_scalar_multivector_add_sub_ops!(u64   => EuclideanMultivector2<u64> => EuclideanMultivector2<u64>,     {0}, {1, 2, 3});
impl_scalar_multivector_add_sub_ops!(u128  => EuclideanMultivector2<u128> => EuclideanMultivector2<u128>,   {0}, {1, 2, 3});
impl_scalar_multivector_add_sub_ops!(usize => EuclideanMultivector2<usize> => EuclideanMultivector2<usize>, {0}, {1, 2, 3});
impl_scalar_multivector_add_sub_ops!(i8    => EuclideanMultivector2<i8> => EuclideanMultivector2<i8>,       {0}, {1, 2, 3});
impl_scalar_multivector_add_sub_ops!(i16   => EuclideanMultivector2<i16> => EuclideanMultivector2<i16>,     {0}, {1, 2, 3});
impl_scalar_multivector_add_sub_ops!(i32   => EuclideanMultivector2<i32> => EuclideanMultivector2<i32>,     {0}, {1, 2, 3});
impl_scalar_multivector_add_sub_ops!(i64   => EuclideanMultivector2<i64> => EuclideanMultivector2<i64>,     {0}, {1, 2, 3});
impl_scalar_multivector_add_sub_ops!(i128  => EuclideanMultivector2<i128> => EuclideanMultivector2<i128>,   {0}, {1, 2, 3});
impl_scalar_multivector_add_sub_ops!(isize => EuclideanMultivector2<isize> => EuclideanMultivector2<isize>, {0}, {1, 2, 3});
impl_scalar_multivector_add_sub_ops!(f32   => EuclideanMultivector2<f32> => EuclideanMultivector2<f32>,     {0}, {1, 2, 3});
impl_scalar_multivector_add_sub_ops!(f64   => EuclideanMultivector2<f64> => EuclideanMultivector2<f64>,     {0}, {1, 2, 3});

