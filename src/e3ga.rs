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

impl<S> EuclideanMultivector3<S> 
where
    S: Scalar
{
    /// Construct the additive unit (zero) multivector.
    /// 
    /// # Example
    /// 
    /// ```
    /// # use cggeomalg::e3ga::{
    /// #     EuclideanMultivector3,
    /// # };
    /// #
    /// let mv = EuclideanMultivector3::new(
    ///     1_f64, 2_f64, 3_f64, 4_f64, 5_f64, 6_f64, 7_f64, 8_f64
    /// );
    /// let zero: EuclideanMultivector3<f64> = EuclideanMultivector3::zero();
    /// 
    /// assert_eq!(mv + zero, mv);
    /// assert_eq!(zero + mv, mv);
    /// ```
    #[inline]
    pub fn zero() -> Self {
        Self {
            data: [S::zero(); 8],
        }
    }

    /// Determine whether a multivector is the zero mutlivector.
    /// 
    /// # Example
    /// 
    /// ```
    /// # use cggeomalg::e3ga::{
    /// #     EuclideanMultivector3,
    /// # };
    /// #
    /// let zero: EuclideanMultivector3<f64> = EuclideanMultivector3::zero();
    /// 
    /// assert!(zero.is_zero());
    /// 
    /// let mv: EuclideanMultivector3<f64> = EuclideanMultivector3::new(
    ///     3_f64, 
    ///     84_f64, 83_f64, 61_f64,
    ///     345_f64, 7_f64, 6_f64,
    ///     45_f64
    /// );
    /// 
    /// assert!(!mv.is_zero());
    /// ```
    #[inline]
    pub fn is_zero(&self) -> bool {
        self.data[0].is_zero() &&
        self.data[1].is_zero() &&
        self.data[2].is_zero() &&
        self.data[3].is_zero() &&
        self.data[4].is_zero() &&
        self.data[5].is_zero() &&
        self.data[6].is_zero() &&
        self.data[7].is_zero()
    }

    /// Construct a new multivector from the scalar part only.
    /// 
    /// A scalar is a multivector whose vector, bivector, etc. components are
    /// all zero.
    /// 
    /// # Example
    /// 
    /// ```
    /// # use cggeomalg::e3ga::{
    /// #     EuclideanMultivector3,
    /// # };
    /// #
    /// let scalar_part = 2;
    /// let scalar = EuclideanMultivector3::from_scalar(scalar_part);
    /// 
    /// assert_eq!(scalar.scalar, scalar_part);
    /// assert_eq!(scalar.e1, 0);
    /// assert_eq!(scalar.e2, 0);
    /// assert_eq!(scalar.e3, 0);
    /// assert_eq!(scalar.e12, 0);
    /// assert_eq!(scalar.e23, 0);
    /// assert_eq!(scalar.e31, 0);
    /// assert_eq!(scalar.e123, 0);
    /// ```
    #[inline]
    pub fn from_scalar(scalar: S) -> Self {
        Self::new(
            scalar, 
            S::zero(), S::zero(), S::zero(), 
            S::zero(), S::zero(), S::zero(), 
            S::zero()
        )
    }

    /// Returns the unit scalar multivector.
    ///
    /// # Example
    /// 
    /// ```
    /// # use cggeomalg::e3ga::{
    /// #     EuclideanMultivector3,
    /// # };
    /// #
    /// let unit_scalar: EuclideanMultivector3<isize> = EuclideanMultivector3::unit_scalar();
    /// 
    /// assert_eq!(unit_scalar.scalar, 1);
    /// assert_eq!(unit_scalar.e1, 0);
    /// assert_eq!(unit_scalar.e2, 0);
    /// assert_eq!(unit_scalar.e3, 0);
    /// assert_eq!(unit_scalar.e12, 0);
    /// assert_eq!(unit_scalar.e23, 0);
    /// assert_eq!(unit_scalar.e31, 0);
    /// assert_eq!(unit_scalar.e123, 0);
    /// ```
    #[inline]
    pub fn unit_scalar() -> Self {
        Self::new(
            S::one(), 
            S::zero(), S::zero(), S::zero(),
            S::zero(), S::zero(), S::zero(),
            S::zero()
        )
    }

    /// Returns the unit `x`-axis vector.
    ///
    /// # Example
    /// 
    /// ```
    /// # use cggeomalg::e3ga::{
    /// #     EuclideanMultivector3,
    /// # };
    /// #
    /// let unit_e1: EuclideanMultivector3<isize> = EuclideanMultivector3::unit_e1();
    /// 
    /// assert_eq!(unit_e1.scalar, 0);
    /// assert_eq!(unit_e1.e1, 1);
    /// assert_eq!(unit_e1.e2, 0);
    /// assert_eq!(unit_e1.e3, 0);
    /// assert_eq!(unit_e1.e12, 0);
    /// assert_eq!(unit_e1.e23, 0);
    /// assert_eq!(unit_e1.e31, 0);
    /// assert_eq!(unit_e1.e123, 0);
    /// ```
    #[inline]
    pub fn unit_e1() -> Self {
        Self::new(
            S::zero(), 
            S::one(), S::zero(), S::zero(),
            S::zero(), S::zero(), S::zero(),
            S::zero()
        )
    }

    /// Returns the unit `y`-axis vector.
    ///
    /// # Example
    /// 
    /// ```
    /// # use cggeomalg::e3ga::{
    /// #     EuclideanMultivector3,
    /// # };
    /// #
    /// let unit_e2: EuclideanMultivector3<isize> = EuclideanMultivector3::unit_e2();
    /// 
    /// assert_eq!(unit_e2.scalar, 0);
    /// assert_eq!(unit_e2.e1, 0);
    /// assert_eq!(unit_e2.e2, 1);
    /// assert_eq!(unit_e2.e3, 0);
    /// assert_eq!(unit_e2.e12, 0);
    /// assert_eq!(unit_e2.e23, 0);
    /// assert_eq!(unit_e2.e31, 0);
    /// assert_eq!(unit_e2.e123, 0);
    /// ```
    #[inline]
    pub fn unit_e2() -> Self {
        Self::new(
            S::zero(), 
            S::zero(), S::one(), S::zero(),
            S::zero(), S::zero(), S::zero(),
            S::zero()
        )
    }

    /// Returns the unit `z`-axis vector.
    ///
    /// # Example
    /// 
    /// ```
    /// # use cggeomalg::e3ga::{
    /// #     EuclideanMultivector3,
    /// # };
    /// #
    /// let unit_e3: EuclideanMultivector3<isize> = EuclideanMultivector3::unit_e3();
    /// 
    /// assert_eq!(unit_e3.scalar, 0);
    /// assert_eq!(unit_e3.e1, 0);
    /// assert_eq!(unit_e3.e2, 0);
    /// assert_eq!(unit_e3.e3, 1);
    /// assert_eq!(unit_e3.e12, 0);
    /// assert_eq!(unit_e3.e23, 0);
    /// assert_eq!(unit_e3.e31, 0);
    /// assert_eq!(unit_e3.e123, 0);
    /// ```
    #[inline]
    pub fn unit_e3() -> Self {
        Self::new(
            S::zero(), 
            S::zero(), S::zero(), S::one(),
            S::zero(), S::zero(), S::zero(),
            S::zero()
        )
    }

    /// Returns the unit `xy`-plane bivector for three-dimensional Euclidean space.
    ///
    /// # Example
    /// 
    /// ```
    /// # use cggeomalg::e3ga::{
    /// #     EuclideanMultivector3,
    /// # };
    /// #
    /// let unit_e12: EuclideanMultivector3<isize> = EuclideanMultivector3::unit_e12();
    /// 
    /// assert_eq!(unit_e12.scalar, 0);
    /// assert_eq!(unit_e12.e1, 0);
    /// assert_eq!(unit_e12.e2, 0);
    /// assert_eq!(unit_e12.e3, 0);
    /// assert_eq!(unit_e12.e12, 1);
    /// assert_eq!(unit_e12.e23, 0);
    /// assert_eq!(unit_e12.e31, 0);
    /// assert_eq!(unit_e12.e123, 0);
    /// ```
    #[inline]
    pub fn unit_e12() -> Self {
        Self::new(
            S::zero(), 
            S::zero(), S::zero(), S::zero(),
            S::one(), S::zero(), S::zero(),
            S::zero()
        )
    }

    /// Returns the unit `xy`-plane bivector for three-dimensional Euclidean space.
    ///
    /// # Example
    /// 
    /// ```
    /// # use cggeomalg::e3ga::{
    /// #     EuclideanMultivector3,
    /// # };
    /// #
    /// let unit_e23: EuclideanMultivector3<isize> = EuclideanMultivector3::unit_e23();
    /// 
    /// assert_eq!(unit_e23.scalar, 0);
    /// assert_eq!(unit_e23.e1, 0);
    /// assert_eq!(unit_e23.e2, 0);
    /// assert_eq!(unit_e23.e3, 0);
    /// assert_eq!(unit_e23.e12, 0);
    /// assert_eq!(unit_e23.e23, 1);
    /// assert_eq!(unit_e23.e31, 0);
    /// assert_eq!(unit_e23.e123, 0);
    /// ```
    #[inline]
    pub fn unit_e23() -> Self {
        Self::new(
            S::zero(), 
            S::zero(), S::zero(), S::zero(),
            S::zero(), S::one(), S::zero(),
            S::zero()
        )
    }

    /// Returns the unit `zx`-plane bivector for three-dimensional Euclidean space.
    ///
    /// # Example
    /// 
    /// ```
    /// # use cggeomalg::e3ga::{
    /// #     EuclideanMultivector3,
    /// # };
    /// #
    /// let unit_e31: EuclideanMultivector3<isize> = EuclideanMultivector3::unit_e31();
    /// 
    /// assert_eq!(unit_e31.scalar, 0);
    /// assert_eq!(unit_e31.e1, 0);
    /// assert_eq!(unit_e31.e2, 0);
    /// assert_eq!(unit_e31.e3, 0);
    /// assert_eq!(unit_e31.e12, 0);
    /// assert_eq!(unit_e31.e23, 0);
    /// assert_eq!(unit_e31.e31, 1);
    /// assert_eq!(unit_e31.e123, 0);
    /// ```
    #[inline]
    pub fn unit_e31() -> Self {
        Self::new(
            S::zero(), 
            S::zero(), S::zero(), S::zero(),
            S::zero(), S::zero(), S::one(),
            S::zero()
        )
    }

    /// Returns the unit volume element for three-dimensional Euclidean space.
    /// 
    /// # Example
    /// 
    /// ```
    /// # use cggeomalg::e3ga::{
    /// #     EuclideanMultivector3,
    /// # };
    /// #
    /// let unit_e123: EuclideanMultivector3<isize> = EuclideanMultivector3::unit_e123();
    /// 
    /// assert_eq!(unit_e123.scalar, 0);
    /// assert_eq!(unit_e123.e1, 0);
    /// assert_eq!(unit_e123.e2, 0);
    /// assert_eq!(unit_e123.e3, 0);
    /// assert_eq!(unit_e123.e12, 0);
    /// assert_eq!(unit_e123.e23, 0);
    /// assert_eq!(unit_e123.e31, 0);
    /// assert_eq!(unit_e123.e123, 1);
    /// ```
    #[inline]
    pub fn unit_e123() -> Self {
        Self::new(
            S::zero(), 
            S::zero(), S::zero(), S::zero(),
            S::zero(), S::zero(), S::zero(),
            S::one()
        )
    }

    /// Returns the unit volume element for three-dimensional Euclidean space. 
    /// 
    /// This is a synonym for `unit_e123`.
    #[inline(always)]
    pub fn pseudoscalar() -> Self {
        Self::unit_e123()
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
    /// # use cggeomalg::e3ga::{
    /// #     EuclideanMultivector3,
    /// # };
    /// #
    /// let mv: EuclideanMultivector3<isize> = EuclideanMultivector3::new(
    ///     1, 1, 1, 1, 1, 1, 1, 1
    /// );
    /// let expected_0 = EuclideanMultivector3::new(1, 0, 0, 0, 0, 0, 0, 0);
    /// let mv_0 = mv.grade(0);
    /// let expected_1 = EuclideanMultivector3::new(0, 1, 1, 1, 0, 0, 0, 0);
    /// let mv_1 = mv.grade(1);
    /// let expected_2 = EuclideanMultivector3::new(0, 0, 0, 0, 1, 1, 1, 0);
    /// let mv_2 = mv.grade(2);
    /// let expected_3 = EuclideanMultivector3::new(0, 0, 0, 0, 0, 0, 0, 1);
    /// let mv_3 = mv.grade(3);
    /// 
    /// assert_eq!(mv_0, expected_0);
    /// assert_eq!(mv_1, expected_1);
    /// assert_eq!(mv_2, expected_2);
    /// assert_eq!(mv_3, expected_3);
    /// 
    /// // Any grade larger than three should be zero.
    /// let zero: EuclideanMultivector3<isize> = EuclideanMultivector3::zero();
    /// assert_eq!(mv.grade(4), zero);
    /// assert_eq!(mv.grade(usize::MAX), zero);
    /// ```
    #[inline]
    pub fn grade(&self, grade: usize) -> Self {
        match grade {
            0 => Self::new(
                    self.data[0], 
                    S::zero(), S::zero(), S::zero(),
                    S::zero(), S::zero(), S::zero(),
                    S::zero()
                ),
            1 => Self::new(
                    S::zero(), 
                    self.data[1], self.data[2], self.data[3], 
                    S::zero(), S::zero(), S::zero(),
                    S::zero()
                ),
            2 => Self::new(
                    S::zero(), 
                    S::zero(), S::zero(), S::zero(), 
                    self.data[4], self.data[5], self.data[6],
                    S::zero()
                ),
            3 => Self::new(
                    S::zero(),
                    S::zero(), S::zero(), S::zero(),
                    S::zero(), S::zero(), S::zero(),
                    self.data[7]
            ),
            _ => Self::zero()
        }
    }
}

impl<S> ops::Index<usize> for EuclideanMultivector3<S> 
where
    S: Scalar
{
    type Output = S;

    #[inline]
    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}

impl<S> ops::IndexMut<usize> for EuclideanMultivector3<S> 
where
    S: Scalar
{
    #[inline]
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.data[index]
    }
}

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

impl<S> fmt::Display for EuclideanMultivector3<S>
where
    S: fmt::Display
{
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(
            formatter, 
            "{} + {}^e1 + {}^e2 + {}^e3 + {}^e12 + {}^e23 + {}^e31 + {}^e123",
            self.data[0], 
            self.data[1], self.data[2], self.data[3],
            self.data[4], self.data[5], self.data[6],
            self.data[7]
        )
    }
}


impl<S> EuclideanMultivector3<S> 
where
    S: ScalarSigned
{
    /// Compute the reverse of a multivector.
    /// 
    /// The reverse of a three-dimensional multivector `mv`, for each grade of 
    /// multivector is given by
    /// ```text
    /// When mv is a scalar, rev(mv) := mv
    /// When mv is a vector, rev(mv) := mv
    /// When mv is a bivector, rev(mv) := -mv
    /// When mv is a trivector, rev(mv) := -mv
    /// ```
    /// In particular, let `v1`, `v2`, and `v3` be vectors
    /// ```text
    /// When v = v1 is a vector,
    /// rev(v) = rev(v1) = -v1 = -v.
    /// When B = v1 ^ v2 is a 2-blade, 
    /// rev(B) = rev(v1 ^ v2) = (rev(v2)) ^ (rev(v1)) = (-v2) ^ (-v1) 
    ///        = v2 ^ v1 
    ///        = -(v1 ^ v2)
    ///        = -B.
    /// When T = v1 ^ v2 ^ v3 is a 3-blade, 
    /// rev(T) = rev(v1 ^ v2 ^ v3) = (rev(v3)) ^ (rev(v1 ^ v2)) = (rev(v3)) ^ ((rev(v2)) ^ (rev(v1)))
    ///        = v3 ^ v2 ^ v1 
    ///        = -(v1 ^ v2 ^ v3)
    ///        = -T.
    /// ```
    /// Then for an arbitrary three-dimensional multivector `mv = a + v + B + T`,
    /// where `a` is a scalar, `v` is a vector, `B` is a bivector, and `T` is a trivector.
    /// The reverse of the multivector is given by linearity
    /// ```text
    /// rev(mv) = rev(a + v + B + T)
    ///         = rev(a) + rev(v) + rev(B) + rev(T)
    ///         = a + v - B - T
    /// ```
    /// 
    /// # Reversion In Euclidean Space
    /// 
    /// Let `mv = a0 + a1 * e1 + a2 * e2 + a3 * e3 + a12 * e12 + a23 * e23 + a31 * e31 + a123 * e123` 
    /// be a three-dimensional Euclidean multivector. The reversion of each 
    /// basis blade is given by
    /// ```text
    /// rev(1)    = 1
    /// rev(e1)   = e1
    /// rev(e2)   = e2
    /// rev(e3)   = e3
    /// rev(e12)  = rev(e1 * e2) = (rev(e2)) * (rev(e1)) = e2 * e1 = -(e1 * e2) = -e12
    /// rev(e23)  = rev(e2 * e3) = (rev(e3)) * (rev(e2)) = e3 * e2 = -(e2 * e3) = -e23
    /// rev(e31)  = rev(e3 * e1) = (rev(e1)) * (rev(e3)) = e1 * e3 = -(e3 * e1) = -e31
    /// rev(e123) = rev(e1 * e2 * e3) 
    ///           = (rev(e3)) * (rev(e1 * e2)) 
    ///           = e3 * ((rev(e2)) * (rev(e1))) 
    ///           = e3 * e2 * e1 
    ///           = -e123
    /// ```
    /// The reversion of a general multivector in the basis 
    /// `{1, e1, e2, e3, e12, e23, e31, e123}` is the following
    /// ```text
    /// rev(mv) = rev(a0 + a1 * e1 + a2 * e2 + a3 * e3 + a12 * e12 + a23 * e23 + a31 * e31 + a123 * e123)
    ///     = rev(a0) + rev(a1 * e1) + rev(a2 * e2) + rev(a3 * e3) 
    ///               + rev(a12 * e12) + rev(a23 * e23) + rev(a31 * e31) 
    ///               + rev(a123 * e123)
    ///     = rev(a0) + a1 * (rev(e1)) + a2 * (rev(e2)) + a3 * (rev(e3)) 
    ///               + a12 * (rev(e12)) + a23 * (rev(e23)) + a31 * (rev(e31))
    ///               + a123 * (rev(e123))
    ///     = a0 + a1 * e1 + a2 * e2 + a3 * e3 - a12 * e12 - a23 * e23 - a31 * e31 - a123 * e123
    /// ```
    /// We illustrate this with an example.
    /// 
    /// # Example
    /// 
    /// ```
    /// # use cggeomalg::e3ga::{
    /// #     EuclideanMultivector3,
    /// # };
    /// #
    /// let mv = EuclideanMultivector3::new(
    ///     1_i32, 1_i32, 1_i32, 1_i32, 2_i32, 2_i32, 2_i32, 3_i32
    /// );
    /// let expected = EuclideanMultivector3::new(
    ///     1_i32, 1_i32, 1_i32, 1_i32, -2_i32, -2_i32, -2_i32, -3_i32
    /// );
    /// let result = mv.reverse();
    /// 
    /// assert_eq!(result, expected);
    /// ```
    pub fn reverse(&self) -> Self {
        Self::new(
            self.data[0],
            self.data[1], self.data[2], self.data[3],
            -self.data[4], -self.data[5], -self.data[6],
            -self.data[7]
        )
    }

    /// Compute the reverse of a multivector mutably in place.
    /// 
    /// # Example
    /// 
    /// ```
    /// # use cggeomalg::e3ga::{
    /// #     EuclideanMultivector3,
    /// # };
    /// #
    /// let mut result = EuclideanMultivector3::new(
    ///     1_i32, 1_i32, 1_i32, 1_i32, 2_i32, 2_i32, 2_i32, 3_i32
    /// );
    /// let expected = EuclideanMultivector3::new(
    ///     1_i32, 1_i32, 1_i32, 1_i32, -2_i32, -2_i32, -2_i32, -3_i32
    /// );
    /// result.reverse_mut();
    /// 
    /// assert_eq!(result, expected);
    /// ```
    pub fn reverse_mut(&mut self) {
        self.data[4] = -self.data[4];
        self.data[5] = -self.data[5];
        self.data[6] = -self.data[6];
        self.data[7] = -self.data[7];
    }

    /// Compute the conjugate of a multivector.
    /// 
    /// The conjugate of a three-dimensional multivector `mv`, for each grade of 
    /// multivector is given by
    /// ```text
    /// When mv is a scalar, conj(mv) := mv
    /// When mv is a vector, conj(mv) := -mv
    /// When mv is a bivector, conj(mv) := -mv
    /// When mv is a trivector, conj(mv) := mv
    /// ```
    /// The conjugate of a three-dimensional multivector extends to an arbitrary 
    /// multivector `mv` by linearity. Let `mv = a + v + B + T` be an arbitrary
    /// three-dimensional Euclidean multivector where `a` is a scalar, `v` is a 
    /// vector, `B` is a bivector, and `T` is a trivector. Then the conjugate of 
    /// `mv` is given by
    /// ```text
    /// conj(mv) = conj(a + v + B + T)
    ///          = conj(a) + conj(v) + conj(B) + conj(T)
    ///          = a + (-v) + (-B) + T
    ///          = a - v - B + T
    /// ```
    /// 
    /// # Conjugate In Euclidean Space
    /// 
    /// The conjugate of each basis blade in the basis 
    /// `{1, e1, e2, e3, e12, e23, e31, e123}` are given by
    /// ```text
    /// conj(1)    = 1
    /// conj(e1)   = -e1
    /// conj(e2)   = -e2
    /// conj(e3)   = -e3
    /// conj(e12)  = -e12
    /// conj(e23)  = -e23
    /// conj(e31)  = -e31
    /// conj(e123) = e123
    /// ```
    /// Let `mv = a0 + a1 * e1 + a2 * e2 + a3 * e3 + a12 * e12 + a23 * e23 + a31 * e31 + a123 * e123`
    /// be a general multivector. The conjugate of `mv` is given by
    /// ```text
    /// conj(mv) = conj(a0 + a1 * e1 + a2 * e2 + a3 * e3 + a12 * e12 + a23 * e23 + a31 * e31 + a123 * e123)
    ///          = conj(a0) + conj(a1 * e1) + conj(a2 * e2) + conj(a3 * e3)
    ///                     + conj(a12 * e12) + conj(a23 * e23) + conj(a31 * e31)
    ///                     + conj(a123 * e123)
    ///          = a0 + a1 * conj(e1) + a2 * conj(e2) + a3 * conj(e3)
    ///               + a12 * conj(e12) + a23 * conj(e23) + a31 * conj(e31)
    ///               + a123 * conj(e123)
    ///          = a0 + a1 * (-e1) + a2 * (-e2) + a3 * (-e3)
    ///               + a12 * (-e12) + a23 * (-e23) + a31 * (-e31)
    ///               + a123 * e123
    ///          = a0 - a1 * e1 - a2 * e2 - a3 * e3
    ///               - a12 * e12  - a23 * e23 - a31 * e31
    ///               + a123 * e123
    /// ```
    /// We illustrate this with an example.
    /// 
    /// # Example
    /// 
    /// ```
    /// # use cggeomalg::e3ga::{
    /// #     EuclideanMultivector3,
    /// # };
    /// #
    /// let mv = EuclideanMultivector3::new(
    ///     1_i32, 2_i32, 3_i32, 4_i32, 5_i32, 6_i32, 7_i32, 8_i32
    /// );
    /// let expected = EuclideanMultivector3::new(
    ///     1_i32, -2_i32, -3_i32, -4_i32, -5_i32, -6_i32, -7_i32, 8_i32
    /// );
    /// let result = mv.conjugate();
    /// 
    /// assert_eq!(result, expected);
    /// ```
    pub fn conjugate(&self) -> Self {
        Self::new(
            self.data[0], 
            -self.data[1], -self.data[2], -self.data[3],
            -self.data[4], -self.data[5], -self.data[6],
            self.data[7]
        )
    }

    /// Compute the conjugate of a multivector mutably in place.
    /// 
    /// # Example
    /// 
    /// ```
    /// # use cggeomalg::e3ga::{
    /// #     EuclideanMultivector3,
    /// # };
    /// #
    /// let mut result = EuclideanMultivector3::new(
    ///     1_i32, 2_i32, 3_i32, 4_i32, 5_i32, 6_i32, 7_i32, 8_i32
    /// );
    /// let expected = EuclideanMultivector3::new(
    ///     1_i32, -2_i32, -3_i32, -4_i32, -5_i32, -6_i32, -7_i32, 8_i32
    /// );
    /// result.conjugate_mut();
    /// 
    /// assert_eq!(result, expected);
    /// ```
    pub fn conjugate_mut(&mut self) {
        self.data[1] = -self.data[1];
        self.data[2] = -self.data[2];
        self.data[3] = -self.data[3];
        self.data[4] = -self.data[4];
        self.data[5] = -self.data[5];
        self.data[6] = -self.data[6];
    }
/*
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
*/
}

impl<S> ops::Not for EuclideanMultivector3<S> 
where
    S: ScalarSigned
{
    type Output = EuclideanMultivector3<S>;

    #[inline]
    fn not(self) -> Self::Output {
        let mut result = Self::Output::zero();
        result.data[0] = -self.data[7];
        result.data[1] = -self.data[6];
        result.data[2] =  self.data[5];
        result.data[3] = -self.data[4];
        result.data[4] =  self.data[3];
        result.data[5] = -self.data[2];
        result.data[6] =  self.data[1];
        result.data[7] =  self.data[0];
        
        result
    }
}

impl<S> ops::Not for &EuclideanMultivector3<S> 
where
    S: ScalarSigned
{
    type Output = EuclideanMultivector3<S>;

    #[inline]
    fn not(self) -> Self::Output {
        let mut result = Self::Output::zero();
        result.data[0] = -self.data[7];
        result.data[1] = -self.data[6];
        result.data[2] =  self.data[5];
        result.data[3] = -self.data[4];
        result.data[4] =  self.data[3];
        result.data[5] = -self.data[2];
        result.data[6] =  self.data[1];
        result.data[7] =  self.data[0];
        
        result
    }
}

impl<S> ops::Neg for EuclideanMultivector3<S>
where
    S: ScalarSigned
{
    type Output = EuclideanMultivector3<S>;

    #[inline]
    fn neg(self) -> Self::Output {
        let result_1    = -self.data[0];
        let result_e1   = -self.data[1];
        let result_e2   = -self.data[2];
        let result_e3   = -self.data[3];
        let result_e12  = -self.data[4];
        let result_e23  = -self.data[5];
        let result_e31  = -self.data[6];
        let result_e123 = -self.data[7];

        EuclideanMultivector3::new(
            result_1, 
            result_e1, result_e2, result_e3, 
            result_e12, result_e23, result_e31, 
            result_e123
        )
    }
}

impl<S> ops::Neg for &EuclideanMultivector3<S>
where
    S: ScalarSigned
{
    type Output = EuclideanMultivector3<S>;

    #[inline]
    fn neg(self) -> Self::Output {
        let result_1    = -self.data[0];
        let result_e1   = -self.data[1];
        let result_e2   = -self.data[2];
        let result_e3   = -self.data[3];
        let result_e12  = -self.data[4];
        let result_e23  = -self.data[5];
        let result_e31  = -self.data[6];
        let result_e123 = -self.data[7];

        EuclideanMultivector3::new(
            result_1, 
            result_e1, result_e2, result_e3, 
            result_e12, result_e23, result_e31, 
            result_e123
        )
    }
}

impl<S> ops::Mul<EuclideanMultivector3<S>> for EuclideanMultivector3<S>
where
    S: Scalar
{
    type Output = EuclideanMultivector3<S>;

    #[inline]
    fn mul(self, other: EuclideanMultivector3<S>) -> Self::Output {
        let a = self;
        let b = other;
        let result_1    = a[0] * b[0] + a[1] * b[1] + a[2] * b[2] + a[3] * b[3] - a[4] * b[4] - a[5] * b[5] - a[6] * b[6] - a[7] * b[7];
        let result_e1   = a[0] * b[1] + a[1] * b[0] - a[2] * b[4] + a[3] * b[6] + a[4] * b[2] - a[5] * b[7] - a[6] * b[3] - a[7] * b[5];
        let result_e2   = a[0] * b[2] + a[1] * b[4] + a[2] * b[0] - a[3] * b[5] - a[4] * b[1] + a[5] * b[3] - a[6] * b[7] - a[7] * b[6];
        let result_e3   = a[0] * b[3] - a[1] * b[6] + a[2] * b[5] + a[3] * b[0] - a[4] * b[7] - a[5] * b[2] + a[6] * b[1] - a[7] * b[4];
        let result_e12  = a[0] * b[4] + a[1] * b[2] - a[2] * b[1] + a[3] * b[7] + a[4] * b[0] - a[5] * b[6] + a[6] * b[5] + a[7] * b[3];
        let result_e23  = a[0] * b[5] + a[1] * b[7] + a[2] * b[3] - a[3] * b[2] + a[4] * b[6] + a[5] * b[0] - a[6] * b[4] + a[7] * b[1];
        let result_e31  = a[0] * b[6] - a[1] * b[3] + a[2] * b[7] + a[3] * b[1] - a[4] * b[5] + a[5] * b[4] + a[6] * b[0] + a[7] * b[2];
        let result_e123 = a[0] * b[7] + a[1] * b[5] + a[2] * b[6] + a[3] * b[4] + a[4] * b[3] + a[5] * b[1] + a[6] * b[2] + a[7] * b[0];

        EuclideanMultivector3::new(
            result_1, 
            result_e1, result_e2, result_e3, 
            result_e12, result_e23, result_e31,
            result_e123
        )
    }
}

impl<S> ops::Mul<&EuclideanMultivector3<S>> for EuclideanMultivector3<S>
where
    S: Scalar
{
    type Output = EuclideanMultivector3<S>;

    #[inline]
    fn mul(self, other: &EuclideanMultivector3<S>) -> Self::Output {
        let a = self;
        let b = other;
        let result_1    = a[0] * b[0] + a[1] * b[1] + a[2] * b[2] + a[3] * b[3] - a[4] * b[4] - a[5] * b[5] - a[6] * b[6] - a[7] * b[7];
        let result_e1   = a[0] * b[1] + a[1] * b[0] - a[2] * b[4] + a[3] * b[6] + a[4] * b[2] - a[5] * b[7] - a[6] * b[3] - a[7] * b[5];
        let result_e2   = a[0] * b[2] + a[1] * b[4] + a[2] * b[0] - a[3] * b[5] - a[4] * b[1] + a[5] * b[3] - a[6] * b[7] - a[7] * b[6];
        let result_e3   = a[0] * b[3] - a[1] * b[6] + a[2] * b[5] + a[3] * b[0] - a[4] * b[7] - a[5] * b[2] + a[6] * b[1] - a[7] * b[4];
        let result_e12  = a[0] * b[4] + a[1] * b[2] - a[2] * b[1] + a[3] * b[7] + a[4] * b[0] - a[5] * b[6] + a[6] * b[5] + a[7] * b[3];
        let result_e23  = a[0] * b[5] + a[1] * b[7] + a[2] * b[3] - a[3] * b[2] + a[4] * b[6] + a[5] * b[0] - a[6] * b[4] + a[7] * b[1];
        let result_e31  = a[0] * b[6] - a[1] * b[3] + a[2] * b[7] + a[3] * b[1] - a[4] * b[5] + a[5] * b[4] + a[6] * b[0] + a[7] * b[2];
        let result_e123 = a[0] * b[7] + a[1] * b[5] + a[2] * b[6] + a[3] * b[4] + a[4] * b[3] + a[5] * b[1] + a[6] * b[2] + a[7] * b[0];

        EuclideanMultivector3::new(
            result_1, 
            result_e1, result_e2, result_e3, 
            result_e12, result_e23, result_e31,
            result_e123
        )
    }
}

impl<S> ops::Mul<EuclideanMultivector3<S>> for &EuclideanMultivector3<S>
where
    S: Scalar
{
    type Output = EuclideanMultivector3<S>;

    #[inline]
    fn mul(self, other: EuclideanMultivector3<S>) -> Self::Output {
        let a = self;
        let b = other;
        let result_1    = a[0] * b[0] + a[1] * b[1] + a[2] * b[2] + a[3] * b[3] - a[4] * b[4] - a[5] * b[5] - a[6] * b[6] - a[7] * b[7];
        let result_e1   = a[0] * b[1] + a[1] * b[0] - a[2] * b[4] + a[3] * b[6] + a[4] * b[2] - a[5] * b[7] - a[6] * b[3] - a[7] * b[5];
        let result_e2   = a[0] * b[2] + a[1] * b[4] + a[2] * b[0] - a[3] * b[5] - a[4] * b[1] + a[5] * b[3] - a[6] * b[7] - a[7] * b[6];
        let result_e3   = a[0] * b[3] - a[1] * b[6] + a[2] * b[5] + a[3] * b[0] - a[4] * b[7] - a[5] * b[2] + a[6] * b[1] - a[7] * b[4];
        let result_e12  = a[0] * b[4] + a[1] * b[2] - a[2] * b[1] + a[3] * b[7] + a[4] * b[0] - a[5] * b[6] + a[6] * b[5] + a[7] * b[3];
        let result_e23  = a[0] * b[5] + a[1] * b[7] + a[2] * b[3] - a[3] * b[2] + a[4] * b[6] + a[5] * b[0] - a[6] * b[4] + a[7] * b[1];
        let result_e31  = a[0] * b[6] - a[1] * b[3] + a[2] * b[7] + a[3] * b[1] - a[4] * b[5] + a[5] * b[4] + a[6] * b[0] + a[7] * b[2];
        let result_e123 = a[0] * b[7] + a[1] * b[5] + a[2] * b[6] + a[3] * b[4] + a[4] * b[3] + a[5] * b[1] + a[6] * b[2] + a[7] * b[0];

        EuclideanMultivector3::new(
            result_1, 
            result_e1, result_e2, result_e3, 
            result_e12, result_e23, result_e31,
            result_e123
        )
    }
}

impl<'a, 'b, S> ops::Mul<&'b EuclideanMultivector3<S>> for &'a EuclideanMultivector3<S>
where
    S: Scalar
{
    type Output = EuclideanMultivector3<S>;

    #[inline]
    fn mul(self, other: &'b EuclideanMultivector3<S>) -> Self::Output {
        let a = self;
        let b = other;
        let result_1    = a[0] * b[0] + a[1] * b[1] + a[2] * b[2] + a[3] * b[3] - a[4] * b[4] - a[5] * b[5] - a[6] * b[6] - a[7] * b[7];
        let result_e1   = a[0] * b[1] + a[1] * b[0] - a[2] * b[4] + a[3] * b[6] + a[4] * b[2] - a[5] * b[7] - a[6] * b[3] - a[7] * b[5];
        let result_e2   = a[0] * b[2] + a[1] * b[4] + a[2] * b[0] - a[3] * b[5] - a[4] * b[1] + a[5] * b[3] - a[6] * b[7] - a[7] * b[6];
        let result_e3   = a[0] * b[3] - a[1] * b[6] + a[2] * b[5] + a[3] * b[0] - a[4] * b[7] - a[5] * b[2] + a[6] * b[1] - a[7] * b[4];
        let result_e12  = a[0] * b[4] + a[1] * b[2] - a[2] * b[1] + a[3] * b[7] + a[4] * b[0] - a[5] * b[6] + a[6] * b[5] + a[7] * b[3];
        let result_e23  = a[0] * b[5] + a[1] * b[7] + a[2] * b[3] - a[3] * b[2] + a[4] * b[6] + a[5] * b[0] - a[6] * b[4] + a[7] * b[1];
        let result_e31  = a[0] * b[6] - a[1] * b[3] + a[2] * b[7] + a[3] * b[1] - a[4] * b[5] + a[5] * b[4] + a[6] * b[0] + a[7] * b[2];
        let result_e123 = a[0] * b[7] + a[1] * b[5] + a[2] * b[6] + a[3] * b[4] + a[4] * b[3] + a[5] * b[1] + a[6] * b[2] + a[7] * b[0];

        EuclideanMultivector3::new(
            result_1, 
            result_e1, result_e2, result_e3, 
            result_e12, result_e23, result_e31,
            result_e123
        )
    }
}

impl<S> ops::Mul<S> for EuclideanMultivector3<S> 
where
    S: Scalar
{
    type Output = EuclideanMultivector3<S>;

    #[inline]
    fn mul(self, other: S) -> Self::Output {
        let a = self;
        let b = other;
        let result_1    = a[0] * b;
        let result_e1   = a[1] * b;
        let result_e2   = a[2] * b;
        let result_e3   = a[3] * b;
        let result_e12  = a[4] * b;
        let result_e23  = a[5] * b;
        let result_e31  = a[6] * b;
        let result_e123 = a[7] * b;
        
        EuclideanMultivector3::new(
            result_1, 
            result_e1, result_e2, result_e3,
            result_e12, result_e23, result_e31,
            result_e123
        )
    }
}

impl<S> ops::Mul<S> for &EuclideanMultivector3<S> 
where
    S: Scalar
{
    type Output = EuclideanMultivector3<S>;

    #[inline]
    fn mul(self, other: S) -> Self::Output {
        let a = self;
        let b = other;
        let result_1    = a[0] * b;
        let result_e1   = a[1] * b;
        let result_e2   = a[2] * b;
        let result_e3   = a[3] * b;
        let result_e12  = a[4] * b;
        let result_e23  = a[5] * b;
        let result_e31  = a[6] * b;
        let result_e123 = a[7] * b;
        
        EuclideanMultivector3::new(
            result_1, 
            result_e1, result_e2, result_e3,
            result_e12, result_e23, result_e31,
            result_e123
        )
    }
}

impl<S> ops::BitXor<EuclideanMultivector3<S>> for EuclideanMultivector3<S> 
where
    S: Scalar
{
    type Output = EuclideanMultivector3<S>;

    #[inline]
    fn bitxor(self, other: EuclideanMultivector3<S>) -> Self::Output {
        let a = self;
        let b = other;
        let result_1    = a[0] * b[0];
        let result_e1   = a[0] * b[1] + a[1] * b[0];
        let result_e2   = a[0] * b[2] + a[2] * b[0];
        let result_e3   = a[0] * b[3] + a[3] * b[0];
        let result_e12  = a[0] * b[4] + a[1] * b[2] - a[2] * b[1] + a[4] * b[0];
        let result_e23  = a[0] * b[5] + a[2] * b[3] - a[3] * b[2] + a[5] * b[0];
        let result_e31  = a[0] * b[6] - a[1] * b[3] + a[3] * b[1] + a[6] * b[0];
        let result_e123 = a[0] * b[7] + a[1] * b[5] + a[2] * b[6] + a[3] * b[4] + a[4] * b[3] + a[5] * b[1] + a[6] * b[2] + a[7] * b[0];
        
        EuclideanMultivector3::new(
            result_1, 
            result_e1, result_e2, result_e3, 
            result_e12, result_e23, result_e31, 
            result_e123
        )
    }
}

impl<S> ops::BitXor<&EuclideanMultivector3<S>> for EuclideanMultivector3<S> 
where
    S: Scalar
{
    type Output = EuclideanMultivector3<S>;

    #[inline]
    fn bitxor(self, other: &EuclideanMultivector3<S>) -> Self::Output {
        let a = self;
        let b = other;
        let result_1    = a[0] * b[0];
        let result_e1   = a[0] * b[1] + a[1] * b[0];
        let result_e2   = a[0] * b[2] + a[2] * b[0];
        let result_e3   = a[0] * b[3] + a[3] * b[0];
        let result_e12  = a[0] * b[4] + a[1] * b[2] - a[2] * b[1] + a[4] * b[0];
        let result_e23  = a[0] * b[5] + a[2] * b[3] - a[3] * b[2] + a[5] * b[0];
        let result_e31  = a[0] * b[6] - a[1] * b[3] + a[3] * b[1] + a[6] * b[0];
        let result_e123 = a[0] * b[7] + a[1] * b[5] + a[2] * b[6] + a[3] * b[4] + a[4] * b[3] + a[5] * b[1] + a[6] * b[2] + a[7] * b[0];
        
        EuclideanMultivector3::new(
            result_1, 
            result_e1, result_e2, result_e3, 
            result_e12, result_e23, result_e31, 
            result_e123
        )
    }
}

impl<S> ops::BitXor<EuclideanMultivector3<S>> for &EuclideanMultivector3<S> 
where
    S: Scalar
{
    type Output = EuclideanMultivector3<S>;

    #[inline]
    fn bitxor(self, other: EuclideanMultivector3<S>) -> Self::Output {
        let a = self;
        let b = other;
        let result_1    = a[0] * b[0];
        let result_e1   = a[0] * b[1] + a[1] * b[0];
        let result_e2   = a[0] * b[2] + a[2] * b[0];
        let result_e3   = a[0] * b[3] + a[3] * b[0];
        let result_e12  = a[0] * b[4] + a[1] * b[2] - a[2] * b[1] + a[4] * b[0];
        let result_e23  = a[0] * b[5] + a[2] * b[3] - a[3] * b[2] + a[5] * b[0];
        let result_e31  = a[0] * b[6] - a[1] * b[3] + a[3] * b[1] + a[6] * b[0];
        let result_e123 = a[0] * b[7] + a[1] * b[5] + a[2] * b[6] + a[3] * b[4] + a[4] * b[3] + a[5] * b[1] + a[6] * b[2] + a[7] * b[0];
        
        EuclideanMultivector3::new(
            result_1, 
            result_e1, result_e2, result_e3, 
            result_e12, result_e23, result_e31, 
            result_e123
        )
    }
}

impl<'a, 'b, S> ops::BitXor<&'b EuclideanMultivector3<S>> for &'a EuclideanMultivector3<S> 
where
    S: Scalar
{
    type Output = EuclideanMultivector3<S>;

    #[inline]
    fn bitxor(self, other: &'b EuclideanMultivector3<S>) -> Self::Output {
        let a = self;
        let b = other;
        let result_1    = a[0] * b[0];
        let result_e1   = a[0] * b[1] + a[1] * b[0];
        let result_e2   = a[0] * b[2] + a[2] * b[0];
        let result_e3   = a[0] * b[3] + a[3] * b[0];
        let result_e12  = a[0] * b[4] + a[1] * b[2] - a[2] * b[1] + a[4] * b[0];
        let result_e23  = a[0] * b[5] + a[2] * b[3] - a[3] * b[2] + a[5] * b[0];
        let result_e31  = a[0] * b[6] - a[1] * b[3] + a[3] * b[1] + a[6] * b[0];
        let result_e123 = a[0] * b[7] + a[1] * b[5] + a[2] * b[6] + a[3] * b[4] + a[4] * b[3] + a[5] * b[1] + a[6] * b[2] + a[7] * b[0];
        
        EuclideanMultivector3::new(
            result_1, 
            result_e1, result_e2, result_e3, 
            result_e12, result_e23, result_e31, 
            result_e123
        )
    }
}

impl<S> ops::BitXor<S> for EuclideanMultivector3<S>
where
    S: Scalar
{
    type Output = EuclideanMultivector3<S>;

    #[inline]
    fn bitxor(self, other: S) -> Self::Output {
        let a = self;
        let result_1    = a[0] * other;
        let result_e1   = a[1] * other;
        let result_e2   = a[2] * other;
        let result_e3   = a[3] * other;
        let result_e12  = a[4] * other;
        let result_e23  = a[5] * other;
        let result_e31  = a[6] * other;
        let result_e123 = a[7] * other;

        EuclideanMultivector3::new(
            result_1, 
            result_e1, result_e2, result_e3,
            result_e12, result_e23, result_e31,
            result_e123
        )
    }
}

impl<S> ops::BitXor<S> for &EuclideanMultivector3<S>
where
    S: Scalar
{
    type Output = EuclideanMultivector3<S>;

    #[inline]
    fn bitxor(self, other: S) -> Self::Output {
        let a = self;
        let result_1    = a[0] * other;
        let result_e1   = a[1] * other;
        let result_e2   = a[2] * other;
        let result_e3   = a[3] * other;
        let result_e12  = a[4] * other;
        let result_e23  = a[5] * other;
        let result_e31  = a[6] * other;
        let result_e123 = a[7] * other;

        EuclideanMultivector3::new(
            result_1, 
            result_e1, result_e2, result_e3,
            result_e12, result_e23, result_e31,
            result_e123
        )
    }
}
/*
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
*/
impl<S> ops::BitOr<S> for EuclideanMultivector3<S>
where
    S: Scalar
{
    type Output = EuclideanMultivector3<S>;

    #[inline]
    fn bitor(self, other: S) -> Self::Output {
        let a = self;
        let result_1 = a[0] * other;

        EuclideanMultivector3::new(
            result_1, 
            S::zero(), S::zero(), S::zero(),
            S::zero(), S::zero(), S::zero(),
            S::zero()
        )
    }
}

impl<S> ops::BitOr<S> for &EuclideanMultivector3<S>
where
    S: Scalar
{
    type Output = EuclideanMultivector3<S>;

    #[inline]
    fn bitor(self, other: S) -> Self::Output {
        let a = self;
        let result_1 = a[0] * other;

        EuclideanMultivector3::new(
            result_1, 
            S::zero(), S::zero(), S::zero(),
            S::zero(), S::zero(), S::zero(),
            S::zero()
        )
    }
}

impl<S> ops::Add<EuclideanMultivector3<S>> for EuclideanMultivector3<S>
where
    S: Scalar
{
    type Output = EuclideanMultivector3<S>;

    #[inline]
    fn add(self, other: EuclideanMultivector3<S>) -> Self::Output {
        let a = self;
        let b = other;
        let result_1    = a[0] + b[0];
        let result_e1   = a[1] + b[1];
        let result_e2   = a[2] + b[2];
        let result_e3   = a[3] + b[3];
        let result_e12  = a[4] + b[4];
        let result_e23  = a[5] + b[5];
        let result_e31  = a[6] + b[6];
        let result_e123 = a[7] + b[7];
        
        EuclideanMultivector3::new(
            result_1, 
            result_e1, result_e2, result_e3,
            result_e12, result_e23, result_e31,
            result_e123
        )
    }
}

impl<S> ops::Add<&EuclideanMultivector3<S>> for EuclideanMultivector3<S>
where
    S: Scalar
{
    type Output = EuclideanMultivector3<S>;

    #[inline]
    fn add(self, other: &EuclideanMultivector3<S>) -> Self::Output {
        let a = self;
        let b = other;
        let result_1    = a[0] + b[0];
        let result_e1   = a[1] + b[1];
        let result_e2   = a[2] + b[2];
        let result_e3   = a[3] + b[3];
        let result_e12  = a[4] + b[4];
        let result_e23  = a[5] + b[5];
        let result_e31  = a[6] + b[6];
        let result_e123 = a[7] + b[7];
        
        EuclideanMultivector3::new(
            result_1, 
            result_e1, result_e2, result_e3,
            result_e12, result_e23, result_e31,
            result_e123
        )
    }
}

impl<S> ops::Add<EuclideanMultivector3<S>> for &EuclideanMultivector3<S>
where
    S: Scalar
{
    type Output = EuclideanMultivector3<S>;

    #[inline]
    fn add(self, other: EuclideanMultivector3<S>) -> Self::Output {
        let a = self;
        let b = other;
        let result_1    = a[0] + b[0];
        let result_e1   = a[1] + b[1];
        let result_e2   = a[2] + b[2];
        let result_e3   = a[3] + b[3];
        let result_e12  = a[4] + b[4];
        let result_e23  = a[5] + b[5];
        let result_e31  = a[6] + b[6];
        let result_e123 = a[7] + b[7];
        
        EuclideanMultivector3::new(
            result_1, 
            result_e1, result_e2, result_e3,
            result_e12, result_e23, result_e31,
            result_e123
        )
    }
}

impl<'a, 'b, S> ops::Add<&'b EuclideanMultivector3<S>> for &'a EuclideanMultivector3<S>
where
    S: Scalar
{
    type Output = EuclideanMultivector3<S>;

    #[inline]
    fn add(self, other: &'b EuclideanMultivector3<S>) -> Self::Output {
        let a = self;
        let b = other;
        let result_1    = a[0] + b[0];
        let result_e1   = a[1] + b[1];
        let result_e2   = a[2] + b[2];
        let result_e3   = a[3] + b[3];
        let result_e12  = a[4] + b[4];
        let result_e23  = a[5] + b[5];
        let result_e31  = a[6] + b[6];
        let result_e123 = a[7] + b[7];
        
        EuclideanMultivector3::new(
            result_1, 
            result_e1, result_e2, result_e3,
            result_e12, result_e23, result_e31,
            result_e123
        )
    }
}

impl<S> ops::Add<S> for EuclideanMultivector3<S>
where
    S: Scalar
{
    type Output = EuclideanMultivector3<S>;

    #[inline]
    fn add(self, other: S) -> Self::Output {
        let a = self;
        let b = other;
        let result_1    = a[0] + b;
        let result_e1   = a[1];
        let result_e2   = a[2];
        let result_e3   = a[3];
        let result_e12  = a[4];
        let result_e23  = a[5];
        let result_e31  = a[6];
        let result_e123 = a[7];
        
        EuclideanMultivector3::new(
            result_1, 
            result_e1, result_e2, result_e3, 
            result_e12, result_e23, result_e31, 
            result_e123
        )
    }
}

impl<S> ops::Add<S> for &EuclideanMultivector3<S>
where
    S: Scalar
{
    type Output = EuclideanMultivector3<S>;

    #[inline]
    fn add(self, other: S) -> Self::Output {
        let a = self;
        let b = other;
        let result_1    = a[0] + b;
        let result_e1   = a[1];
        let result_e2   = a[2];
        let result_e3   = a[3];
        let result_e12  = a[4];
        let result_e23  = a[5];
        let result_e31  = a[6];
        let result_e123 = a[7];
        
        EuclideanMultivector3::new(
            result_1, 
            result_e1, result_e2, result_e3, 
            result_e12, result_e23, result_e31, 
            result_e123
        )
    }
}

impl<S> ops::Sub<EuclideanMultivector3<S>> for EuclideanMultivector3<S>
where
    S: Scalar
{
    type Output = EuclideanMultivector3<S>;

    #[inline]
    fn sub(self, other: EuclideanMultivector3<S>) -> Self::Output {
        let a = self;
        let b = other;
        let result_1    = a[0] - b[0];
        let result_e1   = a[1] - b[1];
        let result_e2   = a[2] - b[2];
        let result_e3   = a[3] - b[3];
        let result_e12  = a[4] - b[4];
        let result_e23  = a[5] - b[5];
        let result_e31  = a[6] - b[6];
        let result_e123 = a[7] - b[7];
        
        EuclideanMultivector3::new(
            result_1, 
            result_e1, result_e2, result_e3,
            result_e12, result_e23, result_e31,
            result_e123
        )
    }
}

impl<S> ops::Sub<&EuclideanMultivector3<S>> for EuclideanMultivector3<S>
where
    S: Scalar
{
    type Output = EuclideanMultivector3<S>;

    #[inline]
    fn sub(self, other: &EuclideanMultivector3<S>) -> Self::Output {
        let a = self;
        let b = other;
        let result_1    = a[0] - b[0];
        let result_e1   = a[1] - b[1];
        let result_e2   = a[2] - b[2];
        let result_e3   = a[3] - b[3];
        let result_e12  = a[4] - b[4];
        let result_e23  = a[5] - b[5];
        let result_e31  = a[6] - b[6];
        let result_e123 = a[7] - b[7];
        
        EuclideanMultivector3::new(
            result_1, 
            result_e1, result_e2, result_e3,
            result_e12, result_e23, result_e31,
            result_e123
        )
    }
}

impl<S> ops::Sub<EuclideanMultivector3<S>> for &EuclideanMultivector3<S>
where
    S: Scalar
{
    type Output = EuclideanMultivector3<S>;

    #[inline]
    fn sub(self, other: EuclideanMultivector3<S>) -> Self::Output {
        let a = self;
        let b = other;
        let result_1    = a[0] - b[0];
        let result_e1   = a[1] - b[1];
        let result_e2   = a[2] - b[2];
        let result_e3   = a[3] - b[3];
        let result_e12  = a[4] - b[4];
        let result_e23  = a[5] - b[5];
        let result_e31  = a[6] - b[6];
        let result_e123 = a[7] - b[7];
        
        EuclideanMultivector3::new(
            result_1, 
            result_e1, result_e2, result_e3,
            result_e12, result_e23, result_e31,
            result_e123
        )
    }
}

impl<'a, 'b, S> ops::Sub<&'b EuclideanMultivector3<S>> for &'a EuclideanMultivector3<S>
where
    S: Scalar
{
    type Output = EuclideanMultivector3<S>;

    #[inline]
    fn sub(self, other: &'b EuclideanMultivector3<S>) -> Self::Output {
        let a = self;
        let b = other;
        let result_1    = a[0] - b[0];
        let result_e1   = a[1] - b[1];
        let result_e2   = a[2] - b[2];
        let result_e3   = a[3] - b[3];
        let result_e12  = a[4] - b[4];
        let result_e23  = a[5] - b[5];
        let result_e31  = a[6] - b[6];
        let result_e123 = a[7] - b[7];
        
        EuclideanMultivector3::new(
            result_1, 
            result_e1, result_e2, result_e3,
            result_e12, result_e23, result_e31,
            result_e123
        )
    }
}

impl<S> ops::Sub<S> for EuclideanMultivector3<S>
where
    S: Scalar
{
    type Output = EuclideanMultivector3<S>;

    #[inline]
    fn sub(self, other: S) -> Self::Output {
        let a = self;
        let b = other;
        let result_1    = a[0] - b;
        let result_e1   = a[1];
        let result_e2   = a[2];
        let result_e3   = a[3];
        let result_e12  = a[4];
        let result_e23  = a[5];
        let result_e31  = a[6];
        let result_e123 = a[7];
        
        EuclideanMultivector3::new(
            result_1, 
            result_e1, result_e2, result_e3,
            result_e12, result_e23, result_e31,
            result_e123
        )
    }
}

impl<S> ops::Sub<S> for &EuclideanMultivector3<S>
where
    S: Scalar
{
    type Output = EuclideanMultivector3<S>;

    #[inline]
    fn sub(self, other: S) -> Self::Output {
        let a = self;
        let b = other;
        let result_1    = a[0] - b;
        let result_e1   = a[1];
        let result_e2   = a[2];
        let result_e3   = a[3];
        let result_e12  = a[4];
        let result_e23  = a[5];
        let result_e31  = a[6];
        let result_e123 = a[7];
        
        EuclideanMultivector3::new(
            result_1, 
            result_e1, result_e2, result_e3,
            result_e12, result_e23, result_e31,
            result_e123
        )
    }
}

impl<S> approx::AbsDiffEq for EuclideanMultivector3<S>
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
        S::abs_diff_eq(&self[3], &other[3], epsilon) &&
        S::abs_diff_eq(&self[4], &other[4], epsilon) &&
        S::abs_diff_eq(&self[5], &other[5], epsilon) &&
        S::abs_diff_eq(&self[6], &other[6], epsilon) &&
        S::abs_diff_eq(&self[7], &other[7], epsilon)
    }
}

impl<S> approx::RelativeEq for EuclideanMultivector3<S>
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
        S::relative_eq(&self[3], &other[3], epsilon, max_relative) &&
        S::relative_eq(&self[4], &other[4], epsilon, max_relative) &&
        S::relative_eq(&self[5], &other[5], epsilon, max_relative) &&
        S::relative_eq(&self[6], &other[6], epsilon, max_relative) &&
        S::relative_eq(&self[7], &other[7], epsilon, max_relative)
    }
}

impl<S> approx::UlpsEq for EuclideanMultivector3<S>
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
        S::ulps_eq(&self[3], &other[3], epsilon, max_ulps) &&
        S::ulps_eq(&self[4], &other[4], epsilon, max_ulps) &&
        S::ulps_eq(&self[5], &other[5], epsilon, max_ulps) &&
        S::ulps_eq(&self[6], &other[6], epsilon, max_ulps) &&
        S::ulps_eq(&self[7], &other[7], epsilon, max_ulps)
    }
}
/*
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

*/
impl_coords!(ViewG3, { scalar, e1, e2, e3, e12, e23, e31, e123 });
impl_coords_deref!(EuclideanMultivector3, ViewG3);


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

impl_scalar_multivector_add_ops!(u8    => EuclideanMultivector3<u8>    => EuclideanMultivector3<u8>,    {0}, {1, 2, 3, 4, 5, 6, 7});
impl_scalar_multivector_add_ops!(u16   => EuclideanMultivector3<u16>   => EuclideanMultivector3<u16>,   {0}, {1, 2, 3, 4, 5, 6, 7});
impl_scalar_multivector_add_ops!(u32   => EuclideanMultivector3<u32>   => EuclideanMultivector3<u32>,   {0}, {1, 2, 3, 4, 5, 6, 7});
impl_scalar_multivector_add_ops!(u64   => EuclideanMultivector3<u64>   => EuclideanMultivector3<u64>,   {0}, {1, 2, 3, 4, 5, 6, 7});
impl_scalar_multivector_add_ops!(u128  => EuclideanMultivector3<u128>  => EuclideanMultivector3<u128>,  {0}, {1, 2, 3, 4, 5, 6, 7});
impl_scalar_multivector_add_ops!(usize => EuclideanMultivector3<usize> => EuclideanMultivector3<usize>, {0}, {1, 2, 3, 4, 5, 6, 7});
impl_scalar_multivector_add_ops!(i8    => EuclideanMultivector3<i8>    => EuclideanMultivector3<i8>,    {0}, {1, 2, 3, 4, 5, 6, 7});
impl_scalar_multivector_add_ops!(i16   => EuclideanMultivector3<i16>   => EuclideanMultivector3<i16>,   {0}, {1, 2, 3, 4, 5, 6, 7});
impl_scalar_multivector_add_ops!(i32   => EuclideanMultivector3<i32>   => EuclideanMultivector3<i32>,   {0}, {1, 2, 3, 4, 5, 6, 7});
impl_scalar_multivector_add_ops!(i64   => EuclideanMultivector3<i64>   => EuclideanMultivector3<i64>,   {0}, {1, 2, 3, 4, 5, 6, 7});
impl_scalar_multivector_add_ops!(i128  => EuclideanMultivector3<i128>  => EuclideanMultivector3<i128>,  {0}, {1, 2, 3, 4, 5, 6, 7});
impl_scalar_multivector_add_ops!(isize => EuclideanMultivector3<isize> => EuclideanMultivector3<isize>, {0}, {1, 2, 3, 4, 5, 6, 7});
impl_scalar_multivector_add_ops!(f32   => EuclideanMultivector3<f32>   => EuclideanMultivector3<f32>,   {0}, {1, 2, 3, 4, 5, 6, 7});
impl_scalar_multivector_add_ops!(f64   => EuclideanMultivector3<f64>   => EuclideanMultivector3<f64>,   {0}, {1, 2, 3, 4, 5, 6, 7});


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

impl_scalar_multivector_sub_ops!(u8    => EuclideanMultivector3<u8>    => EuclideanMultivector3<u8>,    {0}, {1, 2, 3, 4, 5, 6, 7});
impl_scalar_multivector_sub_ops!(u16   => EuclideanMultivector3<u16>   => EuclideanMultivector3<u16>,   {0}, {1, 2, 3, 4, 5, 6, 7});
impl_scalar_multivector_sub_ops!(u32   => EuclideanMultivector3<u32>   => EuclideanMultivector3<u32>,   {0}, {1, 2, 3, 4, 5, 6, 7});
impl_scalar_multivector_sub_ops!(u64   => EuclideanMultivector3<u64>   => EuclideanMultivector3<u64>,   {0}, {1, 2, 3, 4, 5, 6, 7});
impl_scalar_multivector_sub_ops!(u128  => EuclideanMultivector3<u128>  => EuclideanMultivector3<u128>,  {0}, {1, 2, 3, 4, 5, 6, 7});
impl_scalar_multivector_sub_ops!(usize => EuclideanMultivector3<usize> => EuclideanMultivector3<usize>, {0}, {1, 2, 3, 4, 5, 6, 7});
impl_scalar_multivector_sub_ops!(i8    => EuclideanMultivector3<i8>    => EuclideanMultivector3<i8>,    {0}, {1, 2, 3, 4, 5, 6, 7});
impl_scalar_multivector_sub_ops!(i16   => EuclideanMultivector3<i16>   => EuclideanMultivector3<i16>,   {0}, {1, 2, 3, 4, 5, 6, 7});
impl_scalar_multivector_sub_ops!(i32   => EuclideanMultivector3<i32>   => EuclideanMultivector3<i32>,   {0}, {1, 2, 3, 4, 5, 6, 7});
impl_scalar_multivector_sub_ops!(i64   => EuclideanMultivector3<i64>   => EuclideanMultivector3<i64>,   {0}, {1, 2, 3, 4, 5, 6, 7});
impl_scalar_multivector_sub_ops!(i128  => EuclideanMultivector3<i128>  => EuclideanMultivector3<i128>,  {0}, {1, 2, 3, 4, 5, 6, 7});
impl_scalar_multivector_sub_ops!(isize => EuclideanMultivector3<isize> => EuclideanMultivector3<isize>, {0}, {1, 2, 3, 4, 5, 6, 7});
impl_scalar_multivector_sub_ops!(f32   => EuclideanMultivector3<f32>   => EuclideanMultivector3<f32>,   {0}, {1, 2, 3, 4, 5, 6, 7});
impl_scalar_multivector_sub_ops!(f64   => EuclideanMultivector3<f64>   => EuclideanMultivector3<f64>,   {0}, {1, 2, 3, 4, 5, 6, 7});


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

impl_scalar_multivector_mul_ops!(u8    => EuclideanMultivector3<u8>    => EuclideanMultivector3<u8>,    {0, 1, 2, 3, 4, 5, 6, 7});
impl_scalar_multivector_mul_ops!(u16   => EuclideanMultivector3<u16>   => EuclideanMultivector3<u16>,   {0, 1, 2, 3, 4, 5, 6, 7});
impl_scalar_multivector_mul_ops!(u32   => EuclideanMultivector3<u32>   => EuclideanMultivector3<u32>,   {0, 1, 2, 3, 4, 5, 6, 7});
impl_scalar_multivector_mul_ops!(u64   => EuclideanMultivector3<u64>   => EuclideanMultivector3<u64>,   {0, 1, 2, 3, 4, 5, 6, 7});
impl_scalar_multivector_mul_ops!(u128  => EuclideanMultivector3<u128>  => EuclideanMultivector3<u128>,  {0, 1, 2, 3, 4, 5, 6, 7});
impl_scalar_multivector_mul_ops!(usize => EuclideanMultivector3<usize> => EuclideanMultivector3<usize>, {0, 1, 2, 3, 4, 5, 6, 7});
impl_scalar_multivector_mul_ops!(i8    => EuclideanMultivector3<i8>    => EuclideanMultivector3<i8>,    {0, 1, 2, 3, 4, 5, 6, 7});
impl_scalar_multivector_mul_ops!(i16   => EuclideanMultivector3<i16>   => EuclideanMultivector3<i16>,   {0, 1, 2, 3, 4, 5, 6, 7});
impl_scalar_multivector_mul_ops!(i32   => EuclideanMultivector3<i32>   => EuclideanMultivector3<i32>,   {0, 1, 2, 3, 4, 5, 6, 7});
impl_scalar_multivector_mul_ops!(i64   => EuclideanMultivector3<i64>   => EuclideanMultivector3<i64>,   {0, 1, 2, 3, 4, 5, 6, 7});
impl_scalar_multivector_mul_ops!(i128  => EuclideanMultivector3<i128>  => EuclideanMultivector3<i128>,  {0, 1, 2, 3, 4, 5, 6, 7});
impl_scalar_multivector_mul_ops!(isize => EuclideanMultivector3<isize> => EuclideanMultivector3<isize>, {0, 1, 2, 3, 4, 5, 6, 7});
impl_scalar_multivector_mul_ops!(f32   => EuclideanMultivector3<f32>   => EuclideanMultivector3<f32>,   {0, 1, 2, 3, 4, 5, 6, 7});
impl_scalar_multivector_mul_ops!(f64   => EuclideanMultivector3<f64>   => EuclideanMultivector3<f64>,   {0, 1, 2, 3, 4, 5, 6, 7});


macro_rules! impl_scalar_multivector_bitor_ops {
    ($Lhs:ty) => {
        impl ops::BitOr<EuclideanMultivector3<$Lhs>> for $Lhs {
            type Output = EuclideanMultivector3<$Lhs>;

            #[inline]
            fn bitor(self, other: EuclideanMultivector3<$Lhs>) -> Self::Output {
                let mut result = Self::Output::zero();
                result[0] = self * other[0];
        
                result
            }
        }

        impl ops::BitOr<&EuclideanMultivector3<$Lhs>> for $Lhs {
            type Output = EuclideanMultivector3<$Lhs>;

            #[inline]
            fn bitor(self, other: &EuclideanMultivector3<$Lhs>) -> Self::Output {
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
        impl ops::BitXor<EuclideanMultivector3<$Lhs>> for $Lhs {
            type Output = EuclideanMultivector3<$Lhs>;

            #[inline]
            fn bitxor(self, other: EuclideanMultivector3<$Lhs>) -> Self::Output {
                let mut result = Self::Output::zero();
                result[0] = self * other[0];
                result[1] = self * other[1];
                result[2] = self * other[2];
                result[3] = self * other[3];
                result[4] = self * other[4];
                result[5] = self * other[5];
                result[6] = self * other[6];
                result[7] = self * other[7];
        
                result
            }
        }

        impl ops::BitXor<&EuclideanMultivector3<$Lhs>> for $Lhs {
            type Output = EuclideanMultivector3<$Lhs>;

            #[inline]
            fn bitxor(self, other: &EuclideanMultivector3<$Lhs>) -> Self::Output {
                let mut result = Self::Output::zero();
                result[0] = self * other[0];
                result[1] = self * other[1];
                result[2] = self * other[2];
                result[3] = self * other[3];
                result[4] = self * other[4];
                result[5] = self * other[5];
                result[6] = self * other[6];
                result[7] = self * other[7];
        
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

/*
macro_rules! impl_scalar_multivector_div_ops {
    ($Lhs:ty) => {
        impl ops::Div<EuclideanMultivector3<$Lhs>> for $Lhs {
            type Output = EuclideanMultivector3<$Lhs>;

            #[inline]
            fn div(self, other: EuclideanMultivector3<$Lhs>) -> Self::Output {
                let result = other.inverse();
                assert!(result.is_some(), "Attempt to divide by a multivector with zero magnitude: {:?}", other);
                let mut result = result.unwrap();
                result[0] = self * result[0];
                result[1] = self * result[1];
                result[2] = self * result[2];
                result[3] = self * result[3];
                result[4] = self * result[4];
                result[5] = self * result[5];
                result[6] = self * result[6];
                result[7] = self * result[7];

                result
            }
        }

        impl ops::Div<&EuclideanMultivector3<$Lhs>> for $Lhs {
            type Output = EuclideanMultivector3<$Lhs>;

            #[inline]
            fn div(self, other: &EuclideanMultivector3<$Lhs>) -> Self::Output {
                let result = other.inverse();
                assert!(result.is_some(), "Attempt to divide by a multivector with zero magnitude: {:?}", other);
                let mut result = result.unwrap();
                result[0] = self * result[0];
                result[1] = self * result[1];
                result[2] = self * result[2];
                result[3] = self * result[3];
                result[4] = self * result[4];
                result[5] = self * result[5];
                result[6] = self * result[6];
                result[7] = self * result[7];

                result
            }
        }
    }
}

impl_scalar_multivector_div_ops!(f32);
impl_scalar_multivector_div_ops!(f64);

*/