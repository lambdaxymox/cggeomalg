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


/// A stack-allocated, three-dimensional Euclidean multivector.
/// 
/// Euclidean multivectors use a `e12/e23/e31` bitvector order.
#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub struct EuclideanMultivector3<S> {
    data: [S; 8],
}

impl<S> EuclideanMultivector3<S> {
    /// Construct a new general multivector.
    #[inline]
    pub const fn new(scalar: S, e1: S, e2: S, e3: S, e12: S, e23: S, e31: S, e123: S) -> Self {
        Self {
            data: [scalar, e1, e2, e3, e12, e23, e31, e123]
        }
    }

    /// Returns the number of components in a multivector.
    /// 
    /// # Example
    /// 
    /// ```
    /// # use cggeomalg::e3ga::{
    /// #     EuclideanMultivector3,
    /// # };
    /// #
    /// let mv = EuclideanMultivector3::new(1, 1, 1, 1, 1, 1, 1, 1);
    /// 
    /// assert_eq!(mv.len(), 8);
    /// ```
    #[inline]
    pub const fn len(&self) -> usize {
        8
    }

    /// Get a pointer to the underlying component array.
    #[inline]
    pub const fn as_ptr(&self) -> *const S {
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
        <Self as AsRef<[S; 8]>>::as_ref(self)
    }
}
/*
impl<S> EuclideanMultivector2<S> 
where
    S: Scalar
{
    /// Construct the additive unit (zero) multivector.
    /// 
    /// # Example
    /// 
    /// ```
    /// # use cggeomalg::e2ga::{
    /// #     EuclideanMultivector2,
    /// # };
    /// #
    /// let mv = EuclideanMultivector2::new(1_f64, 2_f64, 3_f64, 4_f64);
    /// let zero: EuclideanMultivector2<f64> = EuclideanMultivector2::zero();
    /// 
    /// assert_eq!(mv + zero, mv);
    /// assert_eq!(zero + mv, mv);
    /// ```
    #[inline]
    pub fn zero() -> Self {
        Self {
            data: [S::zero(), S::zero(), S::zero(), S::zero()],
        }
    }

    /// Determine whether a multivector is the zero mutlivector.
    /// 
    /// # Example
    /// 
    /// ```
    /// # use cggeomalg::e2ga::{
    /// #     EuclideanMultivector2,
    /// # };
    /// #
    /// let zero: EuclideanMultivector2<f64> = EuclideanMultivector2::zero();
    /// 
    /// assert!(zero.is_zero());
    /// 
    /// let mv: EuclideanMultivector2<f64> = EuclideanMultivector2::new(3_f64, 84_f64, 83_f64, 61_f64);
    /// 
    /// assert!(!mv.is_zero());
    /// ```
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

    /// Returns the unit scalar multivector.
    ///
    /// # Example
    /// 
    /// ```
    /// # use cggeomalg::e2ga::{
    /// #     EuclideanMultivector2,
    /// # };
    /// #
    /// let unit_scalar: EuclideanMultivector2<isize> = EuclideanMultivector2::unit_scalar();
    /// 
    /// assert_eq!(unit_scalar.scalar, 1);
    /// assert_eq!(unit_scalar.e1, 0);
    /// assert_eq!(unit_scalar.e2, 0);
    /// assert_eq!(unit_scalar.e12, 0);
    /// ```
    #[inline]
    pub fn unit_scalar() -> Self {
        Self::new(S::one(), S::zero(), S::zero(), S::zero())
    }

    /// Returns the unit `x`-axis vector.
    ///
    /// # Example
    /// 
    /// ```
    /// # use cggeomalg::e2ga::{
    /// #     EuclideanMultivector2,
    /// # };
    /// #
    /// let unit_e1: EuclideanMultivector2<isize> = EuclideanMultivector2::unit_e1();
    /// 
    /// assert_eq!(unit_e1.scalar, 0);
    /// assert_eq!(unit_e1.e1, 1);
    /// assert_eq!(unit_e1.e2, 0);
    /// assert_eq!(unit_e1.e12, 0);
    /// ```
    #[inline]
    pub fn unit_e1() -> Self {
        Self::new(S::zero(), S::one(), S::zero(), S::zero())
    }

    /// Returns the unit `y`-axis vector.
    ///
    /// # Example
    /// 
    /// ```
    /// # use cggeomalg::e2ga::{
    /// #     EuclideanMultivector2,
    /// # };
    /// #
    /// let unit_e2: EuclideanMultivector2<isize> = EuclideanMultivector2::unit_e2();
    /// 
    /// assert_eq!(unit_e2.scalar, 0);
    /// assert_eq!(unit_e2.e1, 0);
    /// assert_eq!(unit_e2.e2, 1);
    /// assert_eq!(unit_e2.e12, 0);
    /// ```
    #[inline]
    pub fn unit_e2() -> Self {
        Self::new(S::zero(), S::zero(), S::one(), S::zero())
    }

    /// Returns the unit volume element for two-dimensional Euclidean space.
    ///
    /// # Example
    /// 
    /// ```
    /// # use cggeomalg::e2ga::{
    /// #     EuclideanMultivector2,
    /// # };
    /// #
    /// let unit_e12: EuclideanMultivector2<isize> = EuclideanMultivector2::unit_e12();
    /// 
    /// assert_eq!(unit_e12.scalar, 0);
    /// assert_eq!(unit_e12.e1, 0);
    /// assert_eq!(unit_e12.e2, 0);
    /// assert_eq!(unit_e12.e12, 1);
    /// ```
    #[inline]
    pub fn unit_e12() -> Self {
        Self::new(S::zero(), S::zero(), S::zero(), S::one())
    }

    /// Returns the unit volume elements for two-dimensional Euclidean space. 
    /// 
    /// This is a synonym for `unit_e12`.
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
*/
impl<S> AsRef<[S; 8]> for EuclideanMultivector3<S> {
    #[inline]
    fn as_ref(&self) -> &[S; 8] {
        unsafe {
            &*(self as *const EuclideanMultivector3<S> as *const [S; 8])
        }
    }
}

impl<S> AsMut<[S; 8]> for EuclideanMultivector3<S> {
    #[inline]
    fn as_mut(&mut self) -> &mut [S; 8] {
        unsafe {
            &mut *(self as *mut EuclideanMultivector3<S> as *mut [S; 8])
        }
    }
}

impl<S> AsRef<(S, S, S, S, S, S, S, S)> for EuclideanMultivector3<S> {
    #[inline]
    fn as_ref(&self) -> &(S, S, S, S, S, S, S, S) {
        unsafe {
            &*(self as *const EuclideanMultivector3<S> as *const (S, S, S, S, S, S, S, S))
        }
    }
}

impl<S> AsMut<(S, S, S, S, S, S, S, S)> for EuclideanMultivector3<S> {
    #[inline]
    fn as_mut(&mut self) -> &mut (S, S, S, S, S, S, S, S) {
        unsafe {
            &mut *(self as *mut EuclideanMultivector3<S> as *mut (S, S, S, S, S, S, S, S))
        }
    }
}
/*
impl<S> fmt::Display for EuclideanMultivector3<S>
where
    S: fmt::Display
{
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(
            formatter, 
            "{} + {}^e1 + {}^e2 + {}^e12",
            self.data[0], self.data[1], self.data[2], self.data[3]
        )
    }
}
*/
/*
impl<S> EuclideanMultivector2<S> 
where
    S: ScalarSigned
{
    /// Compute the reverse of a multivector.
    /// 
    /// The reverse of a two-dimensional multivector `mv`, for each grade of 
    /// multivector is given by
    /// ```text
    /// ~mv := mv when mv is a scalar
    /// ~mv := mv when mv is a vector
    /// let mv := v1 ^ v2, where v1 and v2 are vectors. Then
    /// ~mv := v2 ^ v1.
    /// ```
    /// Then for an arbitrary two-dimensional multivector `mv = a + v1 + v2 ^ v3`,
    /// where `a` is a scalar, `v1`, `v2`, and `v3` are vectors, we get the 
    /// reverse of a general multivector by linearity
    /// ```text
    /// ~mv = ~(a + v1 +   v2 ^ v2)
    ///     = ~a + ~v1 + ~(v2 ^ v3)
    ///     =  a +  v1 +   v3 ^ v2
    /// ```
    /// where the last line follows from the definition of reversion of k-vectors 
    /// on each grade.
    /// 
    /// # Reversion In Euclidean Space
    /// 
    /// In particular, let `mv = a0 + a1 * e1 + a2 * e2 + a12 * e12` be a 
    /// two-dimensional Euclidean multivector. Then the reversion of `mv` is given
    /// by
    /// ```text
    /// ~mv = ~(a0 + a1 * e1 + a2 * e2 + a12 * e12)
    ///     = ~a0  + ~(a1 * e1)  + ~(a2 * e2)   + ~(a12 * e12)
    ///     = ~a0  +  a1 * (~e1) +   a2 * (~e2) +   a12 * (~e12)
    ///     =  a0  +  a1 * e1    +   a2 * e2    +   a12 * ~(e1 ^ e2)
    ///     =  a0  +  a1 * e1    +   a2 * e2    +   a12 * (e2 ^ e1)
    ///     =  a0  +  a1 * e1    +   a2 * e2    +   a12 * (-(e1 ^ e2))
    ///     =  a0  +  a1 * e1    +   a2 * e2    -   a12 * e12
    /// ```
    /// We illustrate this with an example.
    /// 
    /// # Example
    /// 
    /// ```
    /// # use cggeomalg::e2ga::{
    /// #     EuclideanMultivector2,
    /// # };
    /// #
    /// let mv = EuclideanMultivector2::new(1_i32, 1_i32, 1_i32, 2_i32);
    /// let expected = EuclideanMultivector2::new(1_i32, 1_i32, 1_i32, -2_i32);
    /// let result = mv.reverse();
    /// 
    /// assert_eq!(result, expected);
    /// ```
    pub fn reverse(&self) -> Self {
        Self::new(self.data[0], self.data[1], self.data[2], -self.data[3])
    }

    /// Compute the reverse of a multivector mutably in place.
    /// 
    /// # Example
    /// 
    /// ```
    /// # use cggeomalg::e2ga::{
    /// #     EuclideanMultivector2,
    /// # };
    /// #
    /// let mut result = EuclideanMultivector2::new(1_i32, 1_i32, 1_i32, 2_i32);
    /// let expected = EuclideanMultivector2::new(1_i32, 1_i32, 1_i32, -2_i32);
    /// result.reverse_mut();
    /// 
    /// assert_eq!(result, expected);
    /// ``` 
    pub fn reverse_mut(&mut self) {
        self.data[3] = -self.data[3];
    }

    /// Compute the conjugate of a multivector.
    /// 
    /// The conjugate of a two-dimensional multivector `mv`, for each grade of 
    /// multivector is given by
    /// ```text
    /// mv_conj := mv when mv is scalar
    /// mv_conj := -mv when mv is a vector
    /// Let mv = a * b be where a and b are versors. Then
    /// mv_conj := (a * b).conjugate() = b.conjugate() * a.conjugate()
    /// ```
    /// The conjugate of a two-dimensional multivector extends to an arbitrary 
    /// multivector `mv` by linearity. Let `mv = a + v1 + v2 ^ v3` be an arbitrary
    /// two-dimenional Euclidean multivector where `a` is a scalar, and `v1`, 
    /// `v2`, and `v3` are vectors. Then the conjugate of `mv` is given by
    /// ```text
    /// mv.conjugate() = (a + v1 + v2 ^ v3).conjugate()
    ///                = a.conjugate() + v1.conjugate() + (v2 ^ v3).conjugate() 
    ///                = a - v1 + (v2 * v3 - v2.dot(v3)).conjugate()
    ///                = a - v1 + (v2 * v3).conjugate() - (v2.dot(v3)).conjugate()
    ///                = a - v1 + v3.conjugate() * v2.conjugate() - v2.dot(v3)
    ///                = a - v1 + (-v3) * (-v2) - v3.dot(v2)
    ///                = a - v1 + v3 * v2 - v3.dot(v2)
    ///                = a - v1 + v3 ^ v2
    ///                = a - v1 - v2 ^ v3
    /// ```
    /// 
    /// # Conjugate In Euclidean Space
    /// 
    /// In particular, let `mv = a0 + a1 * e1 + a2 * e2 + a12 * e12` be a 
    /// two-dimensional Euclidean multivector. Then the conjugate of `mv` is given
    /// by
    /// ```text
    /// mv.conjugate() = (a0 + a1 * e1 + a2 * e2 + a12 * e12).conjugate()
    ///                = a0.conjugate() + (a1 * e1).conjugate() + (a2 * e2).conjugate() 
    ///                                 + (a12 * e12).conjugate()
    ///                = a0 + a1 * e1.conjugate() + a2 * e2.conjugate() 
    ///                     + a12 * e12.conjugate()
    ///                = a0 + a1 * (-e1) + a2 * (-e2) + a12 * (e1 ^ e2).conjugate()
    ///                = a0 - a1 * e1 - a2 * e2 + a12 * (e2.conjugate() ^ e1.conjugate())
    ///                = a0 - a1 * e1 - a2 * e2 + a12 * (-e2) ^ (-e1)
    ///                = a0 - a1 * e1 - a2 * e2 + a12 * (e2 ^ v1)
    ///                = a0 - a1 * e1 - a2 * e2 - a12 * e12
    /// ```
    /// We illustrate this with an example.
    /// 
    /// # Example
    /// 
    /// ```
    /// # use cggeomalg::e2ga::{
    /// #     EuclideanMultivector2,
    /// # };
    /// #
    /// let mv = EuclideanMultivector2::new(1_i32, 2_i32, 3_i32, 4_i32);
    /// let expected = EuclideanMultivector2::new(1_i32, -2_i32, -3_i32, -4_i32);
    /// let result = mv.conjugate();
    /// 
    /// assert_eq!(result, expected);
    /// ```
    pub fn conjugate(&self) -> Self {
        Self::new(self.data[0], -self.data[1], -self.data[2], -self.data[3])
    }

    /// Compute the conjugate of a multivector mutably in place.
    /// 
    /// # Example
    /// 
    /// ```
    /// # use cggeomalg::e2ga::{
    /// #     EuclideanMultivector2,
    /// # };
    /// #
    /// let mut result = EuclideanMultivector2::new(1_i32, 2_i32, 3_i32, 4_i32);
    /// let expected = EuclideanMultivector2::new(1_i32, -2_i32, -3_i32, -4_i32);
    /// result.conjugate_mut();
    /// 
    /// assert_eq!(result, expected);
    /// ```
    pub fn conjugate_mut(&mut self) {
        // self.data[0] =  self.data[0];
        self.data[1] = -self.data[1];
        self.data[2] = -self.data[2];
        self.data[3] = -self.data[3];
    }

    /// Compute the grade involution of a multivector.
    /// 
    /// The grade involution of a multivector `mv` is defined by
    /// ```text
    /// mv* := mv when mv is a scalar
    /// mv* := -mv when mv is a vector
    /// Let mv = v1 * v2 where a and b are versors. Then
    /// mv* := (v1 * v2)* = v1* * v2*
    /// ```
    /// Then for an arbitrary two-dimensional multivector `mv = a + v1 + v2 ^ v3`,
    /// where `a` is a scalar, `v1`, `v2`, and `v3` are vectors, we get the 
    /// grade involution of a general multivector by linearity
    /// ```text
    /// mv* = (a + v1 + v2 ^ v3)*
    ///     = a* + v1* + (v2 ^ v3)*
    ///     = a  - v1  + (v2 * v3 - v2.dot(v3))*
    ///     = a  - v1  + (v2 * v3)* - v2.dot(v3)
    ///     = a  - v1  + (v2*) * (v3*) - v2.dot(v3)
    ///     = a  - v1  + (-v2) * (-v3) - v2.dot(v3)
    ///     = a  - v1  + (v2 * v3 - v2.dot(v3))
    ///     = a  - v1  + v2 ^ v3
    /// ```
    /// 
    /// # Involution In Euclidean Space
    /// 
    /// In particular, let `mv = a0 + a1 * e1 + a2 * e2 + a12 * e12` be a 
    /// two-dimensional Euclidean multivector. Then the involution of `mv` is 
    /// given by
    /// ```text
    /// mv* = (a0 + a1 * e1 + a2 * e2 + a12 * e12)*
    ///     =  a0* + (a1 * e1)* + (a2 * e2)* + (a12 * e12)*
    ///     =  a0* + a1 * (e1*) + a2 * (e2*) + a12 * (e12*)
    ///     =  a0  - a1 * e1    - a2 * e2    + a12 * ((e1 * e2)*)
    ///     =  a0  - a1 * e1    - a2 * e2    + a12 * (e1*) * (e2*)
    ///     =  a0  - a1 * e1    - a2 * e2    + a12 * (-e1) * (-e2)
    ///     =  a0  - a1 * e1    - a2 * e2    + a12 * e12
    /// ```
    /// We illustrate this with an example.
    /// 
    /// # Example
    /// 
    /// ```
    /// # use cggeomalg::e2ga::{
    /// #     EuclideanMultivector2,
    /// # };
    /// #
    /// let mv = EuclideanMultivector2::new(1_i32, 2_i32, 3_i32, 4_i32);
    /// let expected = EuclideanMultivector2::new(1_i32, -2_i32, -3_i32, 4_i32);
    /// let result = mv.involute();
    /// 
    /// assert_eq!(result, expected);
    /// ```
    pub fn involute(&self) -> Self {
        Self::new(self.data[0], -self.data[1], -self.data[2], self.data[3])
    }

    /// Compute the grade involution of a multivector mutably in place.
    /// 
    /// # Example
    /// 
    /// ```
    /// # use cggeomalg::e2ga::{
    /// #     EuclideanMultivector2,
    /// # };
    /// #
    /// let mut result = EuclideanMultivector2::new(1_i32, 2_i32, 3_i32, 4_i32);
    /// let expected = EuclideanMultivector2::new(1_i32, -2_i32, -3_i32, 4_i32);
    /// result.involute_mut();
    /// 
    /// assert_eq!(result, expected);
    /// ```
    pub fn involute_mut(&mut self) {
        // self.data[0] =  self.data[0];
        self.data[1] = -self.data[1];
        self.data[2] = -self.data[2];
        // self.data[3] =  self.data[3];
    }

    /// Compute the dual of a multivector.
    pub fn dual(&self) -> Self {
        Self::new(-self.data[3], -self.data[2], self.data[1], self.data[0])
    }

    /// Computer the dual of a multivector mutably in place.
    pub fn dual_mut(&mut self) {
        let mut result = Self::zero();
        result.data[0] = -self.data[3];
        result.data[1] = -self.data[2];
        result.data[2] =  self.data[1];
        result.data[3] =  self.data[0];
        *self = result;
    }

    /// Construct the inverse pseudoscalar of the geometric algebra.
    /// 
    /// In the case of the two-dimensional Euclidean geometric algebra, the
    /// inverse of the pseudoscalar is the two-blade `e12_inv = -e12`.
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

impl<S> ops::BitXor<S> for EuclideanMultivector2<S>
where
    S: Scalar
{
    type Output = EuclideanMultivector2<S>;

    #[inline]
    fn bitxor(self, other: S) -> Self::Output {
        let a = self;
        let result_1   = a[0] * other;
        let result_e1  = a[1] * other;
        let result_e2  = a[2] * other;
        let result_e12 = a[3] * other;

        EuclideanMultivector2::new(result_1, result_e1, result_e2, result_e12)
    }
}

impl<S> ops::BitXor<S> for &EuclideanMultivector2<S>
where
    S: Scalar
{
    type Output = EuclideanMultivector2<S>;

    #[inline]
    fn bitxor(self, other: S) -> Self::Output {
        let a = self;
        let result_1   = a[0] * other;
        let result_e1  = a[1] * other;
        let result_e2  = a[2] * other;
        let result_e12 = a[3] * other;

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

impl<S> ops::BitOr<S> for EuclideanMultivector2<S>
where
    S: Scalar
{
    type Output = EuclideanMultivector2<S>;

    #[inline]
    fn bitor(self, other: S) -> Self::Output {
        let a = self;
        let result_1 = a[0] * other;

        EuclideanMultivector2::new(result_1, S::zero(), S::zero(), S::zero())
    }
}

impl<S> ops::BitOr<S> for &EuclideanMultivector2<S>
where
    S: Scalar
{
    type Output = EuclideanMultivector2<S>;

    #[inline]
    fn bitor(self, other: S) -> Self::Output {
        let a = self;
        let result_1 = a[0] * other;

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

impl<S> approx::AbsDiffEq for EuclideanMultivector2<S>
where
    S: ScalarFloat
{
    type Epsilon = <S as approx::AbsDiffEq>::Epsilon;

    #[inline]
    fn default_epsilon() -> Self::Epsilon {
        S::default_epsilon()
    }

    #[inline]
    fn abs_diff_eq(&self, other: &Self, epsilon: Self::Epsilon) -> bool {
        S::abs_diff_eq(&self[0], &other[0], epsilon) &&
        S::abs_diff_eq(&self[1], &other[1], epsilon) &&
        S::abs_diff_eq(&self[2], &other[2], epsilon) &&
        S::abs_diff_eq(&self[3], &other[3], epsilon)
    }
}

impl<S> approx::RelativeEq for EuclideanMultivector2<S>
where
    S: ScalarFloat
{
    #[inline]
    fn default_max_relative() -> S::Epsilon {
        S::default_max_relative()
    }

    #[inline]
    fn relative_eq(&self, other: &Self, epsilon: S::Epsilon, max_relative: S::Epsilon) -> bool {
        S::relative_eq(&self[0], &other[0], epsilon, max_relative) &&
        S::relative_eq(&self[1], &other[1], epsilon, max_relative) &&
        S::relative_eq(&self[2], &other[2], epsilon, max_relative) &&
        S::relative_eq(&self[3], &other[3], epsilon, max_relative)
    }
}

impl<S> approx::UlpsEq for EuclideanMultivector2<S>
where
    S: ScalarFloat
{
    #[inline]
    fn default_max_ulps() -> u32 {
        S::default_max_ulps()
    }

    #[inline]
    fn ulps_eq(&self, other: &Self, epsilon: S::Epsilon, max_ulps: u32) -> bool {
        S::ulps_eq(&self[0], &other[0], epsilon, max_ulps) &&
        S::ulps_eq(&self[1], &other[1], epsilon, max_ulps) &&
        S::ulps_eq(&self[2], &other[2], epsilon, max_ulps) &&
        S::ulps_eq(&self[3], &other[3], epsilon, max_ulps)
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
        approx::ulps_ne!(self.magnitude_squared(), S::zero())
    }

    pub fn inverse(&self) -> Option<Self> {
        let magnitude_squared = self.magnitude_squared();
        if magnitude_squared.is_zero() {
            None
        } else {
            Some(self.inverse_unchecked())
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

impl<S> ops::Shl<S> for EuclideanMultivector2<S>
where
    S: Scalar
{
    type Output = EuclideanMultivector2<S>;

    #[inline]
    fn shl(self, other: S) -> Self::Output {
        let a = self;
        let result_1   = a[0] * other;
        let result_e1  = S::zero();
        let result_e2  = S::zero();
        let result_e12 = S::zero();

        EuclideanMultivector2::new(result_1, result_e1, result_e2, result_e12)
    }
}

impl<S> ops::Shl<S> for &EuclideanMultivector2<S>
where
    S: Scalar
{
    type Output = EuclideanMultivector2<S>;

    #[inline]
    fn shl(self, other: S) -> Self::Output {
        let a = self;
        let result_1   = a[0] * other;
        let result_e1  = S::zero();
        let result_e2  = S::zero();
        let result_e12 = S::zero();

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

impl<S> ops::Shr<S> for EuclideanMultivector2<S>
where
    S: Scalar
{
    type Output = EuclideanMultivector2<S>;

    #[inline]
    fn shr(self, other: S) -> Self::Output {
        let a = self;
        let result_1   = a[0] * other;
        let result_e1  = a[1] * other;
        let result_e2  = a[2] * other;
        let result_e12 = a[3] * other;

        EuclideanMultivector2::new(result_1, result_e1, result_e2, result_e12)
    }
}

impl<S> ops::Shr<S> for &EuclideanMultivector2<S>
where
    S: Scalar
{
    type Output = EuclideanMultivector2<S>;

    #[inline]
    fn shr(self, other: S) -> Self::Output {
        let a = self;
        let result_1   = a[0] * other;
        let result_e1  = a[1] * other;
        let result_e2  = a[2] * other;
        let result_e12 = a[3] * other;

        EuclideanMultivector2::new(result_1, result_e1, result_e2, result_e12)
    }
}


impl_coords!(ViewG2, { scalar, e1, e2, e12 });
impl_coords_deref!(EuclideanMultivector2, ViewG2);


macro_rules! impl_scalar_multivector_add_ops {
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
    }
}

impl_scalar_multivector_add_ops!(u8    => EuclideanMultivector2<u8>    => EuclideanMultivector2<u8>,    {0}, {1, 2, 3});
impl_scalar_multivector_add_ops!(u16   => EuclideanMultivector2<u16>   => EuclideanMultivector2<u16>,   {0}, {1, 2, 3});
impl_scalar_multivector_add_ops!(u32   => EuclideanMultivector2<u32>   => EuclideanMultivector2<u32>,   {0}, {1, 2, 3});
impl_scalar_multivector_add_ops!(u64   => EuclideanMultivector2<u64>   => EuclideanMultivector2<u64>,   {0}, {1, 2, 3});
impl_scalar_multivector_add_ops!(u128  => EuclideanMultivector2<u128>  => EuclideanMultivector2<u128>,  {0}, {1, 2, 3});
impl_scalar_multivector_add_ops!(usize => EuclideanMultivector2<usize> => EuclideanMultivector2<usize>, {0}, {1, 2, 3});
impl_scalar_multivector_add_ops!(i8    => EuclideanMultivector2<i8>    => EuclideanMultivector2<i8>,    {0}, {1, 2, 3});
impl_scalar_multivector_add_ops!(i16   => EuclideanMultivector2<i16>   => EuclideanMultivector2<i16>,   {0}, {1, 2, 3});
impl_scalar_multivector_add_ops!(i32   => EuclideanMultivector2<i32>   => EuclideanMultivector2<i32>,   {0}, {1, 2, 3});
impl_scalar_multivector_add_ops!(i64   => EuclideanMultivector2<i64>   => EuclideanMultivector2<i64>,   {0}, {1, 2, 3});
impl_scalar_multivector_add_ops!(i128  => EuclideanMultivector2<i128>  => EuclideanMultivector2<i128>,  {0}, {1, 2, 3});
impl_scalar_multivector_add_ops!(isize => EuclideanMultivector2<isize> => EuclideanMultivector2<isize>, {0}, {1, 2, 3});
impl_scalar_multivector_add_ops!(f32   => EuclideanMultivector2<f32>   => EuclideanMultivector2<f32>,   {0}, {1, 2, 3});
impl_scalar_multivector_add_ops!(f64   => EuclideanMultivector2<f64>   => EuclideanMultivector2<f64>,   {0}, {1, 2, 3});


macro_rules! impl_scalar_multivector_sub_ops {
    ($Lhs:ty => $Rhs:ty => $Output:ty, { $scalar_index:expr }, { $($other_index:expr),* }) => {
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

impl_scalar_multivector_sub_ops!(u8    => EuclideanMultivector2<u8>    => EuclideanMultivector2<u8>,    {0}, {1, 2, 3});
impl_scalar_multivector_sub_ops!(u16   => EuclideanMultivector2<u16>   => EuclideanMultivector2<u16>,   {0}, {1, 2, 3});
impl_scalar_multivector_sub_ops!(u32   => EuclideanMultivector2<u32>   => EuclideanMultivector2<u32>,   {0}, {1, 2, 3});
impl_scalar_multivector_sub_ops!(u64   => EuclideanMultivector2<u64>   => EuclideanMultivector2<u64>,   {0}, {1, 2, 3});
impl_scalar_multivector_sub_ops!(u128  => EuclideanMultivector2<u128>  => EuclideanMultivector2<u128>,  {0}, {1, 2, 3});
impl_scalar_multivector_sub_ops!(usize => EuclideanMultivector2<usize> => EuclideanMultivector2<usize>, {0}, {1, 2, 3});
impl_scalar_multivector_sub_ops!(i8    => EuclideanMultivector2<i8>    => EuclideanMultivector2<i8>,    {0}, {1, 2, 3});
impl_scalar_multivector_sub_ops!(i16   => EuclideanMultivector2<i16>   => EuclideanMultivector2<i16>,   {0}, {1, 2, 3});
impl_scalar_multivector_sub_ops!(i32   => EuclideanMultivector2<i32>   => EuclideanMultivector2<i32>,   {0}, {1, 2, 3});
impl_scalar_multivector_sub_ops!(i64   => EuclideanMultivector2<i64>   => EuclideanMultivector2<i64>,   {0}, {1, 2, 3});
impl_scalar_multivector_sub_ops!(i128  => EuclideanMultivector2<i128>  => EuclideanMultivector2<i128>,  {0}, {1, 2, 3});
impl_scalar_multivector_sub_ops!(isize => EuclideanMultivector2<isize> => EuclideanMultivector2<isize>, {0}, {1, 2, 3});
impl_scalar_multivector_sub_ops!(f32   => EuclideanMultivector2<f32>   => EuclideanMultivector2<f32>,   {0}, {1, 2, 3});
impl_scalar_multivector_sub_ops!(f64   => EuclideanMultivector2<f64>   => EuclideanMultivector2<f64>,   {0}, {1, 2, 3});


macro_rules! impl_scalar_multivector_mul_ops {
    ($Lhs:ty => $Rhs:ty => $Output:ty, { $($index:expr),* }) => {
        impl ops::Mul<$Rhs> for $Lhs {
            type Output = $Output;

            #[inline]
            fn mul(self, other: $Rhs) -> Self::Output {
                Self::Output::new( $(self * other[$index]),* )
            }
        }

        impl ops::Mul<&$Rhs> for $Lhs {
            type Output = $Output;

            #[inline]
            fn mul(self, other: &$Rhs) -> Self::Output {
                Self::Output::new( $(self * other[$index]),* )
            }
        }
    }
}

impl_scalar_multivector_mul_ops!(u8    => EuclideanMultivector2<u8>    => EuclideanMultivector2<u8>,    {0, 1, 2, 3});
impl_scalar_multivector_mul_ops!(u16   => EuclideanMultivector2<u16>   => EuclideanMultivector2<u16>,   {0, 1, 2, 3});
impl_scalar_multivector_mul_ops!(u32   => EuclideanMultivector2<u32>   => EuclideanMultivector2<u32>,   {0, 1, 2, 3});
impl_scalar_multivector_mul_ops!(u64   => EuclideanMultivector2<u64>   => EuclideanMultivector2<u64>,   {0, 1, 2, 3});
impl_scalar_multivector_mul_ops!(u128  => EuclideanMultivector2<u128>  => EuclideanMultivector2<u128>,  {0, 1, 2, 3});
impl_scalar_multivector_mul_ops!(usize => EuclideanMultivector2<usize> => EuclideanMultivector2<usize>, {0, 1, 2, 3});
impl_scalar_multivector_mul_ops!(i8    => EuclideanMultivector2<i8>    => EuclideanMultivector2<i8>,    {0, 1, 2, 3});
impl_scalar_multivector_mul_ops!(i16   => EuclideanMultivector2<i16>   => EuclideanMultivector2<i16>,   {0, 1, 2, 3});
impl_scalar_multivector_mul_ops!(i32   => EuclideanMultivector2<i32>   => EuclideanMultivector2<i32>,   {0, 1, 2, 3});
impl_scalar_multivector_mul_ops!(i64   => EuclideanMultivector2<i64>   => EuclideanMultivector2<i64>,   {0, 1, 2, 3});
impl_scalar_multivector_mul_ops!(i128  => EuclideanMultivector2<i128>  => EuclideanMultivector2<i128>,  {0, 1, 2, 3});
impl_scalar_multivector_mul_ops!(isize => EuclideanMultivector2<isize> => EuclideanMultivector2<isize>, {0, 1, 2, 3});
impl_scalar_multivector_mul_ops!(f32   => EuclideanMultivector2<f32>   => EuclideanMultivector2<f32>,   {0, 1, 2, 3});
impl_scalar_multivector_mul_ops!(f64   => EuclideanMultivector2<f64>   => EuclideanMultivector2<f64>,   {0, 1, 2, 3});


macro_rules! impl_scalar_multivector_bitor_ops {
    ($Lhs:ty) => {
        impl ops::BitOr<EuclideanMultivector2<$Lhs>> for $Lhs {
            type Output = EuclideanMultivector2<$Lhs>;

            #[inline]
            fn bitor(self, other: EuclideanMultivector2<$Lhs>) -> Self::Output {
                let mut result = Self::Output::zero();
                result[0] = self * other[0];
        
                result
            }
        }

        impl ops::BitOr<&EuclideanMultivector2<$Lhs>> for $Lhs {
            type Output = EuclideanMultivector2<$Lhs>;

            #[inline]
            fn bitor(self, other: &EuclideanMultivector2<$Lhs>) -> Self::Output {
                let mut result = Self::Output::zero();
                result[0] = self * other[0];
        
                result
            }
        }
    }
}

impl_scalar_multivector_bitor_ops!(u8);
impl_scalar_multivector_bitor_ops!(u16);
impl_scalar_multivector_bitor_ops!(u32);
impl_scalar_multivector_bitor_ops!(u64);
impl_scalar_multivector_bitor_ops!(u128);
impl_scalar_multivector_bitor_ops!(usize);
impl_scalar_multivector_bitor_ops!(i8);
impl_scalar_multivector_bitor_ops!(i16);
impl_scalar_multivector_bitor_ops!(i32);
impl_scalar_multivector_bitor_ops!(i64);
impl_scalar_multivector_bitor_ops!(i128);
impl_scalar_multivector_bitor_ops!(isize);
impl_scalar_multivector_bitor_ops!(f32);
impl_scalar_multivector_bitor_ops!(f64);


macro_rules! impl_scalar_multivector_bitxor_ops {
    ($Lhs:ty) => {
        impl ops::BitXor<EuclideanMultivector2<$Lhs>> for $Lhs {
            type Output = EuclideanMultivector2<$Lhs>;

            #[inline]
            fn bitxor(self, other: EuclideanMultivector2<$Lhs>) -> Self::Output {
                let mut result = Self::Output::zero();
                result[0] = self * other[0];
                result[1] = self * other[1];
                result[2] = self * other[2];
                result[3] = self * other[3];
        
                result
            }
        }

        impl ops::BitXor<&EuclideanMultivector2<$Lhs>> for $Lhs {
            type Output = EuclideanMultivector2<$Lhs>;

            #[inline]
            fn bitxor(self, other: &EuclideanMultivector2<$Lhs>) -> Self::Output {
                let mut result = Self::Output::zero();
                result[0] = self * other[0];
                result[1] = self * other[1];
                result[2] = self * other[2];
                result[3] = self * other[3];
        
                result
            }
        }
    }
}

impl_scalar_multivector_bitxor_ops!(u8);
impl_scalar_multivector_bitxor_ops!(u16);
impl_scalar_multivector_bitxor_ops!(u32);
impl_scalar_multivector_bitxor_ops!(u64);
impl_scalar_multivector_bitxor_ops!(u128);
impl_scalar_multivector_bitxor_ops!(usize);
impl_scalar_multivector_bitxor_ops!(i8);
impl_scalar_multivector_bitxor_ops!(i16);
impl_scalar_multivector_bitxor_ops!(i32);
impl_scalar_multivector_bitxor_ops!(i64);
impl_scalar_multivector_bitxor_ops!(i128);
impl_scalar_multivector_bitxor_ops!(isize);
impl_scalar_multivector_bitxor_ops!(f32);
impl_scalar_multivector_bitxor_ops!(f64);


macro_rules! impl_scalar_multivector_div_ops {
    ($Lhs:ty) => {
        impl ops::Div<EuclideanMultivector2<$Lhs>> for $Lhs {
            type Output = EuclideanMultivector2<$Lhs>;

            #[inline]
            fn div(self, other: EuclideanMultivector2<$Lhs>) -> Self::Output {
                let result = other.inverse();
                assert!(result.is_some(), "Attempt to divide by a multivector with zero magnitude: {:?}", other);
                let mut result = result.unwrap();
                result[0] = self * result[0];
                result[1] = self * result[1];
                result[2] = self * result[2];
                result[3] = self * result[3];

                result
            }
        }

        impl ops::Div<&EuclideanMultivector2<$Lhs>> for $Lhs {
            type Output = EuclideanMultivector2<$Lhs>;

            #[inline]
            fn div(self, other: &EuclideanMultivector2<$Lhs>) -> Self::Output {
                let result = other.inverse();
                assert!(result.is_some(), "Attempt to divide by a multivector with zero magnitude: {:?}", other);
                let mut result = result.unwrap();
                result[0] = self * result[0];
                result[1] = self * result[1];
                result[2] = self * result[2];
                result[3] = self * result[3];

                result
            }
        }
    }
}

impl_scalar_multivector_div_ops!(f32);
impl_scalar_multivector_div_ops!(f64);

*/