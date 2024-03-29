use crate::scalar::{
    Scalar,
    ScalarFloat,
    ScalarSigned,
};
use crate::{
    impl_coords,
    impl_coords_deref,
};
use approx_cmp::ulps_ne;
use core::fmt;
use core::ops;


/// A stack-allocated, two-dimensional Euclidean multivector
/// in the basis the orthonormal basis `{1, e1, e2, e12}`.
#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub struct EuclideanMultivector2<S> {
    data: [S; 4],
}

impl<S> EuclideanMultivector2<S> {
    /// Construct a new general multivector.
    #[inline]
    pub const fn new(scalar: S, e1: S, e2: S, e12: S) -> Self {
        Self {
            data: [scalar, e1, e2, e12],
        }
    }

    /// Returns the number of components in a multivector.
    ///
    /// # Example
    ///
    /// ```
    /// # use cggeomalg::e2ga::EuclideanMultivector2;
    /// #
    /// let mv = EuclideanMultivector2::new(1, 1, 1, 1);
    ///
    /// assert_eq!(mv.len(), 4);
    /// ```
    #[inline]
    pub const fn len(&self) -> usize {
        4
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
        <Self as AsRef<[S; 4]>>::as_ref(self)
    }
}

impl<S> EuclideanMultivector2<S>
where
    S: Scalar,
{
    /// Construct the additive unit (zero) multivector.
    ///
    /// # Example
    ///
    /// ```
    /// # use cggeomalg::e2ga::EuclideanMultivector2;
    /// #
    /// let mv = EuclideanMultivector2::new(1_f64, 2_f64, 3_f64, 4_f64);
    /// let zero: EuclideanMultivector2<f64> = EuclideanMultivector2::zero();
    ///
    /// assert_eq!(mv + zero, mv);
    /// assert_eq!(zero + mv, mv);
    /// ```
    #[inline]
    pub fn zero() -> Self {
        Self { data: [S::zero(); 4] }
    }

    /// Determine whether a multivector is the zero mutlivector.
    ///
    /// # Example
    ///
    /// ```
    /// # use cggeomalg::e2ga::EuclideanMultivector2;
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
        self.data[0].is_zero() && self.data[1].is_zero() && self.data[2].is_zero() && self.data[3].is_zero()
    }

    /// Construct a new multivector from the scalar part only.
    ///
    /// A scalar is a multivector whose vector, bivector, etc. components are
    /// all zero.
    ///
    /// # Example
    ///
    /// ```
    /// # use cggeomalg::e2ga::EuclideanMultivector2;
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
    /// # use cggeomalg::e2ga::EuclideanMultivector2;
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
    /// # use cggeomalg::e2ga::EuclideanMultivector2;
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
    /// # use cggeomalg::e2ga::EuclideanMultivector2;
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
    /// # use cggeomalg::e2ga::EuclideanMultivector2;
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

    /// Returns the unit volume element for two-dimensional Euclidean space.
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
    /// # use cggeomalg::e2ga::EuclideanMultivector2;
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
            _ => Self::zero(),
        }
    }

    /// Compute the left contraction of `self` with `other`.
    ///
    /// This is a synonym for the `<<` operator.
    #[inline]
    pub fn left_contract(&self, other: &Self) -> Self {
        self << other
    }

    /// Compute the right contraction of `self` with `other`.
    ///
    /// This is a synonym for the `>>` operator.
    #[inline]
    pub fn right_contract(&self, other: &Self) -> Self {
        self >> other
    }

    /// Compute the scalar product of `self` and `other`.
    ///
    /// This is a synonym for the `|` operator.
    #[inline]
    pub fn scalar_product(&self, other: &Self) -> Self {
        self | other
    }

    /// Compute the outer product of `self` and `other`.
    ///
    /// This is a synonym for the `^` operator.
    #[inline]
    pub fn outer_product(&self, other: &Self) -> Self {
        self ^ other
    }
}

impl<S> ops::Index<usize> for EuclideanMultivector2<S>
where
    S: Scalar,
{
    type Output = S;

    #[inline]
    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}

impl<S> ops::IndexMut<usize> for EuclideanMultivector2<S>
where
    S: Scalar,
{
    #[inline]
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.data[index]
    }
}

impl<S> AsRef<[S; 4]> for EuclideanMultivector2<S> {
    #[inline]
    fn as_ref(&self) -> &[S; 4] {
        unsafe { &*(self as *const EuclideanMultivector2<S> as *const [S; 4]) }
    }
}

impl<S> AsMut<[S; 4]> for EuclideanMultivector2<S> {
    #[inline]
    fn as_mut(&mut self) -> &mut [S; 4] {
        unsafe { &mut *(self as *mut EuclideanMultivector2<S> as *mut [S; 4]) }
    }
}

impl<S> AsRef<(S, S, S, S)> for EuclideanMultivector2<S> {
    #[inline]
    fn as_ref(&self) -> &(S, S, S, S) {
        unsafe { &*(self as *const EuclideanMultivector2<S> as *const (S, S, S, S)) }
    }
}

impl<S> AsMut<(S, S, S, S)> for EuclideanMultivector2<S> {
    #[inline]
    fn as_mut(&mut self) -> &mut (S, S, S, S) {
        unsafe { &mut *(self as *mut EuclideanMultivector2<S> as *mut (S, S, S, S)) }
    }
}

impl<S> fmt::Display for EuclideanMultivector2<S>
where
    S: fmt::Display,
{
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(
            formatter,
            "{} + {}^e1 + {}^e2 + {}^e12",
            self.data[0], self.data[1], self.data[2], self.data[3]
        )
    }
}

impl<S> EuclideanMultivector2<S>
where
    S: ScalarSigned,
{
    /// Compute the reverse of a multivector.
    ///
    /// The reverse of a two-dimensional multivector `mv`, for each grade of
    /// multivector is given by
    /// ```text
    /// When mv is a scalar, rev(mv) := mv
    /// When mv is a vector, rev(mv) := mv
    /// When mv is a bivector, rev(mv) := -mv
    /// ```
    /// In particular, let `v1` and `v2` be vectors,
    /// ```text
    /// When v = v1 is a vector,
    /// rev(v) = rev(v1) = v1 = v.
    /// When B = v1 ^ v2 is a 2-blade,
    /// rev(B) = rev(v1 ^ v2) = (rev(v2)) ^ (rev(v1))
    ///    = v2 ^ v1
    ///    = -(v1 ^ v2)
    ///    = -B.
    /// ```
    /// Then for an arbitrary two-dimensional multivector `mv = a + v + B`,
    /// where `a` is a scalar, `v` is a vector, and `B` is a bivector, we get the
    /// reverse of a general multivector by linearity
    /// ```text
    /// rev(mv) = rev(a + v + B)
    ///     = rev(a) + rev(v) + rev(B)
    ///     =  a +  v - B
    /// ```
    /// where the last line follows from the definition of reversion of k-vectors
    /// on each grade.
    ///
    /// # Reversion In Euclidean Space
    ///
    /// The reversion of each basis blade in the basis `{1, e1, e2, e12}` are given by
    /// ```text
    /// rev(1)   = 1
    /// rev(e1)  = e1
    /// rev(e2)  = e2
    /// rev(e12) = rev(e1 * e2) = (rev(e2)) * (rev(e1)) = e2 * e1 = -(e1 * e2) = -e12
    /// ```
    /// The reversion of a general multivector in the basis `{1, e1, e2, e12}` is
    /// the following
    /// ```text
    /// rev(mv) = rev(a0 + a1 * e1 + a2 * e2 + a12 * e12)
    ///         = rev(a0) + rev(a1 * e1) + rev(a2 * e2) + rev(a12 * e12)
    ///         = rev(a0) + a1 * rev(e1) + a2 * rev(e2) + a12 * rev(e12)
    ///         = a0      + a1 * e1      + a2 * e2      + a12 * (-e12)
    ///         = a0      + a1 * e1      + a2 * e2      - a12 * e12
    /// ```
    /// We illustrate this with an example.
    ///
    /// # Example
    ///
    /// ```
    /// # use cggeomalg::e2ga::EuclideanMultivector2;
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
    /// # use cggeomalg::e2ga::EuclideanMultivector2;
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
    /// When mv is a scalar, conj(mv) := mv
    /// When mv is a vector, conj(mv) := -mv
    /// When mv is a bivector, conj(mv) := -mv
    /// ```
    /// The conjugate of a two-dimensional multivector extends to an arbitrary
    /// multivector `mv` by linearity. Let `mv = a + v + B` be an arbitrary
    /// two-dimensional Euclidean multivector where `a` is a scalar, `v` is a vector,
    /// `B` is a bivectors. Then the conjugate of `mv` is given by
    /// ```text
    /// conj(mv) = conj(a + v + B)
    ///          = conj(a) + conj(v) + conj(B)
    ///          = a + (-v) + (-B)
    ///          = a - v - B
    /// ```
    ///
    /// # Conjugate In Euclidean Space
    ///
    /// The conjugate of each basis blade in the basis `{1, e1, e2, e12}` are
    /// given by
    /// ```text
    /// conj(1)   = 1
    /// conj(e1)  = -e1
    /// conj(e2)  = -e2
    /// conj(e12) = -e12
    /// ```
    /// Let `mv = a0 + a1 * e1 + a2 * e2 + a12 * e12` be a general multivector.
    /// The conjugate of `mv` is given by
    /// ```text
    /// conj(mv) = conj(a0 + a1 * e1 + a2 * e2 + a12 * e12)
    ///          = conj(a0) + conj(a1 * e1) + conj(a2 * e2) + conj(a12 * e12)
    ///          = conj(a0) + a1 * conj(e1) + a2 * conj(e2) + a12 * conj(e12)
    ///          = a0 - a1 * e1 - a2 * e2 - a12 * e12
    /// ```
    /// We illustrate this with an example.
    ///
    /// # Example
    ///
    /// ```
    /// # use cggeomalg::e2ga::EuclideanMultivector2;
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
    /// # use cggeomalg::e2ga::EuclideanMultivector2;
    /// #
    /// let mut result = EuclideanMultivector2::new(1_i32, 2_i32, 3_i32, 4_i32);
    /// let expected = EuclideanMultivector2::new(1_i32, -2_i32, -3_i32, -4_i32);
    /// result.conjugate_mut();
    ///
    /// assert_eq!(result, expected);
    /// ```
    pub fn conjugate_mut(&mut self) {
        self.data[1] = -self.data[1];
        self.data[2] = -self.data[2];
        self.data[3] = -self.data[3];
    }

    /// Compute the grade involution of a multivector.
    ///
    /// The grade involution of a multivector `mv` is defined by
    /// ```text
    /// When mv is a scalar, invol(mv) := mv
    /// When mv is a vector, invol(mv) := -mv
    /// When mv is a bivector, invol(mv) := mv
    /// ```
    /// The grade involution of a two-dimensional multivector `mv = a + v + B`,
    /// where `a` is a scalar, `v` is a vector, and `B` is a bivector, is
    /// given by linearity
    /// ```text
    /// invol(mv) = invol(a + v + B)
    ///           = invol(a) + invol(v) + invol(B)
    ///           = a - v + B
    /// ```
    ///
    /// # Involution In Euclidean Space
    ///
    /// The grade involution of each basis blade in the basis `{1, e1, e2, e12}` are
    /// given by
    /// ```text
    /// invol(1)   = 1
    /// invol(e1)  = -e1
    /// invol(e2)  = -e2
    /// invol(e12) = e12
    /// ```
    /// Let `mv = a0 + a1 * e1 + a2 * e2 + a12 * e12` be a general multivector.
    /// The grade involution of `mv` is given by
    /// ```text
    /// invol(mv) = invol(a0 + a1 * e1 + a2 * e2 + a12 * e12)
    ///           = invol(a0) + invol(a1 * e1) + invol(a2 * e2) + invol(a12 * e12)
    ///           = invol(a0) + a1 * (invol(e1)) + a2 * (invol(e2)) + a12 * (invol(e12))
    ///           = a0 + a1 * (-e1) + a2 * (-e2) + a12 * e12
    ///           = a0 - a1 * e1    - a2 * e2    + a12 * e12
    /// ```
    /// We illustrate this with an example.
    ///
    /// # Example
    ///
    /// ```
    /// # use cggeomalg::e2ga::EuclideanMultivector2;
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
    /// # use cggeomalg::e2ga::EuclideanMultivector2;
    /// #
    /// let mut result = EuclideanMultivector2::new(1_i32, 2_i32, 3_i32, 4_i32);
    /// let expected = EuclideanMultivector2::new(1_i32, -2_i32, -3_i32, 4_i32);
    /// result.involute_mut();
    ///
    /// assert_eq!(result, expected);
    /// ```
    pub fn involute_mut(&mut self) {
        self.data[1] = -self.data[1];
        self.data[2] = -self.data[2];
    }

    /// Compute the dual of a multivector.
    ///
    /// The dual is also known as the orthogonal complement.
    ///
    /// The dual of a multivector `mv` is defined by
    /// ```text
    /// dual(mv) := mv << inv(e12) == mv * inv(e12)
    /// ```
    /// where `<<` denotes the left contraction, and `inv` denotes the inverse
    /// operator.
    ///
    /// # Duality In Euclidean Space
    ///
    /// In two-dimensional Euclidean geometric algebra, the dual of the elements
    /// of the bases `{1, e1, e2, e12}` is given by
    /// ```text
    /// dual(1)   = -e12
    /// dual(e1)  = -e2
    /// dual(e2)  = e1
    /// dual(e12) = 1
    /// ```
    /// The dual of a multivector `mv = a0 + a1 * e1 + a2 * e2 + a12 * e12` is
    /// given by
    /// ```text
    /// dual(mv) = dual(a0 + a1 * e1 + a2 * e2 + a12 * e12)
    ///          = dual(a0) + dual(a1 * e1) + dual(a2 * e2) + dual(a12 * e12)
    ///          = a0 * (dual1)) + a1 * (dual(e1)) + a2 * (dual(e2)) + a12 * (dual(e12))
    ///          = a0 * (-e12) + a1 * (-e2) + a2 * e1 + a12
    ///          = a12 + a2 * e1 - a1 * e2 - a0 * e12
    /// ```
    ///
    /// # Example
    ///
    /// ```
    /// # use cggeomalg::e2ga::EuclideanMultivector2;
    /// #
    /// let mv = EuclideanMultivector2::new(1, 2, 3, 4);
    /// let expected = EuclideanMultivector2::new(4, 3, -2, -1);
    /// let result = mv.dual();
    ///
    /// assert_eq!(result, expected);
    ///
    /// let e12: EuclideanMultivector2<i32> = EuclideanMultivector2::unit_e12();
    ///
    /// assert_eq!(result * e12, mv);
    /// ```
    pub fn dual(&self) -> Self {
        Self::new(self.data[3], self.data[2], -self.data[1], -self.data[0])
    }

    /// Compute the dual of a multivector mutably in place.
    ///
    /// # Example
    ///
    /// ```
    /// # use cggeomalg::e2ga::EuclideanMultivector2;
    /// #
    /// let mut result = EuclideanMultivector2::new(1, 2, 3, 4);
    /// let expected = EuclideanMultivector2::new(4, 3, -2, -1);
    /// result.dual_mut();
    ///
    /// assert_eq!(result, expected);
    /// ```
    pub fn dual_mut(&mut self) {
        let mut result = Self::zero();
        result.data[0] = self.data[3];
        result.data[1] = self.data[2];
        result.data[2] = -self.data[1];
        result.data[3] = -self.data[0];
        *self = result;
    }

    /// Construct the inverse pseudoscalar of the geometric algebra.
    ///
    /// In the case of the two-dimensional Euclidean geometric algebra, the
    /// inverse of the pseudoscalar is the two-blade `inv(e12) = -e12`.
    ///
    /// # Example
    ///
    /// ```
    /// # use cggeomalg::e3ga::EuclideanMultivector3;
    /// #
    /// let ps: EuclideanMultivector3<f64> = EuclideanMultivector3::pseudoscalar();
    /// let ps_inv: EuclideanMultivector3<f64> = EuclideanMultivector3::inv_pseudoscalar();
    /// let one: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_scalar();
    ///
    /// assert_eq!(ps * ps_inv, one);
    /// assert_eq!(ps_inv * ps, one);
    /// ```
    #[inline]
    pub fn inv_pseudoscalar() -> Self {
        -Self::unit_e12()
    }
}

impl<S> ops::Not for EuclideanMultivector2<S>
where
    S: ScalarSigned,
{
    type Output = EuclideanMultivector2<S>;

    #[rustfmt::skip]
    #[inline]
    fn not(self) -> Self::Output {
        let mut result = Self::Output::zero();
        result.data[0] =  self.data[3];
        result.data[1] =  self.data[2];
        result.data[2] = -self.data[1];
        result.data[3] = -self.data[0];

        result
    }
}

impl<S> ops::Not for &EuclideanMultivector2<S>
where
    S: ScalarSigned,
{
    type Output = EuclideanMultivector2<S>;

    #[rustfmt::skip]
    #[inline]
    fn not(self) -> Self::Output {
        let mut result = Self::Output::zero();
        result.data[0] =  self.data[3];
        result.data[1] =  self.data[2];
        result.data[2] = -self.data[1];
        result.data[3] = -self.data[0];

        result
    }
}

impl<S> ops::Neg for EuclideanMultivector2<S>
where
    S: ScalarSigned,
{
    type Output = EuclideanMultivector2<S>;

    #[rustfmt::skip]
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
    S: ScalarSigned,
{
    type Output = EuclideanMultivector2<S>;

    #[rustfmt::skip]
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
    S: Scalar,
{
    type Output = EuclideanMultivector2<S>;

    #[rustfmt::skip]
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
    S: Scalar,
{
    type Output = EuclideanMultivector2<S>;

    #[rustfmt::skip]
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
    S: Scalar,
{
    type Output = EuclideanMultivector2<S>;

    #[rustfmt::skip]
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
    S: Scalar,
{
    type Output = EuclideanMultivector2<S>;

    #[rustfmt::skip]
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
    S: Scalar,
{
    type Output = EuclideanMultivector2<S>;

    #[rustfmt::skip]
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
    S: Scalar,
{
    type Output = EuclideanMultivector2<S>;

    #[rustfmt::skip]
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
    S: Scalar,
{
    type Output = EuclideanMultivector2<S>;

    #[rustfmt::skip]
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
    S: Scalar,
{
    type Output = EuclideanMultivector2<S>;

    #[rustfmt::skip]
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
    S: Scalar,
{
    type Output = EuclideanMultivector2<S>;

    #[rustfmt::skip]
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
    S: Scalar,
{
    type Output = EuclideanMultivector2<S>;

    #[rustfmt::skip]
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
    S: Scalar,
{
    type Output = EuclideanMultivector2<S>;

    #[rustfmt::skip]
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
    S: Scalar,
{
    type Output = EuclideanMultivector2<S>;

    #[inline]
    fn bitxor(self, other: S) -> Self::Output {
        let a = self;
        let result_1 = a[0] * other;
        let result_e1 = a[1] * other;
        let result_e2 = a[2] * other;
        let result_e12 = a[3] * other;

        EuclideanMultivector2::new(result_1, result_e1, result_e2, result_e12)
    }
}

impl<S> ops::BitOr<EuclideanMultivector2<S>> for EuclideanMultivector2<S>
where
    S: Scalar,
{
    type Output = EuclideanMultivector2<S>;

    #[inline]
    fn bitor(self, other: EuclideanMultivector2<S>) -> Self::Output {
        let a = self;
        let b = other;
        let result_1 = a[0] * b[0] + a[1] * b[1] + a[2] * b[2] + a[3] * b[3];

        EuclideanMultivector2::from_scalar(result_1)
    }
}

impl<S> ops::BitOr<&EuclideanMultivector2<S>> for EuclideanMultivector2<S>
where
    S: Scalar,
{
    type Output = EuclideanMultivector2<S>;

    #[inline]
    fn bitor(self, other: &EuclideanMultivector2<S>) -> Self::Output {
        let a = self;
        let b = other;
        let result_1 = a[0] * b[0] + a[1] * b[1] + a[2] * b[2] + a[3] * b[3];

        EuclideanMultivector2::from_scalar(result_1)
    }
}

impl<S> ops::BitOr<EuclideanMultivector2<S>> for &EuclideanMultivector2<S>
where
    S: Scalar,
{
    type Output = EuclideanMultivector2<S>;

    #[inline]
    fn bitor(self, other: EuclideanMultivector2<S>) -> Self::Output {
        let a = self;
        let b = other;
        let result_1 = a[0] * b[0] + a[1] * b[1] + a[2] * b[2] + a[3] * b[3];

        EuclideanMultivector2::from_scalar(result_1)
    }
}

impl<'a, 'b, S> ops::BitOr<&'b EuclideanMultivector2<S>> for &'a EuclideanMultivector2<S>
where
    S: Scalar,
{
    type Output = EuclideanMultivector2<S>;

    #[inline]
    fn bitor(self, other: &'b EuclideanMultivector2<S>) -> Self::Output {
        let a = self;
        let b = other;
        let result_1 = a[0] * b[0] + a[1] * b[1] + a[2] * b[2] + a[3] * b[3];

        EuclideanMultivector2::from_scalar(result_1)
    }
}

impl<S> ops::BitOr<S> for EuclideanMultivector2<S>
where
    S: Scalar,
{
    type Output = EuclideanMultivector2<S>;

    #[inline]
    fn bitor(self, other: S) -> Self::Output {
        let a = self;
        let result_1 = a[0] * other;

        EuclideanMultivector2::from_scalar(result_1)
    }
}

impl<S> ops::BitOr<S> for &EuclideanMultivector2<S>
where
    S: Scalar,
{
    type Output = EuclideanMultivector2<S>;

    #[inline]
    fn bitor(self, other: S) -> Self::Output {
        let a = self;
        let result_1 = a[0] * other;

        EuclideanMultivector2::from_scalar(result_1)
    }
}

impl<S> ops::Add<EuclideanMultivector2<S>> for EuclideanMultivector2<S>
where
    S: Scalar,
{
    type Output = EuclideanMultivector2<S>;

    #[rustfmt::skip]
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
    S: Scalar,
{
    type Output = EuclideanMultivector2<S>;

    #[rustfmt::skip]
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
    S: Scalar,
{
    type Output = EuclideanMultivector2<S>;

    #[rustfmt::skip]
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
    S: Scalar,
{
    type Output = EuclideanMultivector2<S>;

    #[rustfmt::skip]
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
    S: Scalar,
{
    type Output = EuclideanMultivector2<S>;

    #[rustfmt::skip]
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
    S: Scalar,
{
    type Output = EuclideanMultivector2<S>;

    #[rustfmt::skip]
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
    S: Scalar,
{
    type Output = EuclideanMultivector2<S>;

    #[rustfmt::skip]
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
    S: Scalar,
{
    type Output = EuclideanMultivector2<S>;

    #[rustfmt::skip]
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
    S: Scalar,
{
    type Output = EuclideanMultivector2<S>;

    #[rustfmt::skip]
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
    S: Scalar,
{
    type Output = EuclideanMultivector2<S>;

    #[rustfmt::skip]
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
    S: Scalar,
{
    type Output = EuclideanMultivector2<S>;

    #[rustfmt::skip]
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
    S: Scalar,
{
    type Output = EuclideanMultivector2<S>;

    #[rustfmt::skip]
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

impl<S> approx_cmp::AbsDiffEq for EuclideanMultivector2<S>
where
    S: ScalarFloat,
{
    type Tolerance = EuclideanMultivector2<<S as approx_cmp::AbsDiffEq>::Tolerance>;

    #[inline]
    fn abs_diff_eq(&self, other: &Self, max_abs_diff: &Self::Tolerance) -> bool {
        approx_cmp::AbsDiffEq::abs_diff_eq(&self.data, &other.data, &max_abs_diff.data)
    }
}

impl<S> approx_cmp::AbsDiffAllEq for EuclideanMultivector2<S>
where
    S: ScalarFloat,
{
    type AllTolerance = <S as approx_cmp::AbsDiffAllEq>::AllTolerance;

    #[inline]
    fn abs_diff_all_eq(&self, other: &Self, max_abs_diff: &Self::AllTolerance) -> bool {
        approx_cmp::AbsDiffAllEq::abs_diff_all_eq(&self.data, &other.data, max_abs_diff)
    }
}

impl<S> approx_cmp::AssertAbsDiffEq for EuclideanMultivector2<S>
where
    S: ScalarFloat,
{
    type DebugAbsDiff = EuclideanMultivector2<<S as approx_cmp::AssertAbsDiffEq>::DebugAbsDiff>;
    type DebugTolerance = EuclideanMultivector2<<S as approx_cmp::AssertAbsDiffEq>::DebugTolerance>;

    #[inline]
    fn debug_abs_diff(&self, other: &Self) -> Self::DebugAbsDiff {
        let data = approx_cmp::AssertAbsDiffEq::debug_abs_diff(&self.data, &other.data);

        EuclideanMultivector2 { data }
    }

    #[inline]
    fn debug_abs_diff_tolerance(&self, other: &Self, max_abs_diff: &Self::Tolerance) -> Self::DebugTolerance {
        let data = approx_cmp::AssertAbsDiffEq::debug_abs_diff_tolerance(&self.data, &other.data, &max_abs_diff.data);

        EuclideanMultivector2 { data }
    }
}

impl<S> approx_cmp::AssertAbsDiffAllEq for EuclideanMultivector2<S>
where
    S: ScalarFloat,
{
    type AllDebugTolerance = EuclideanMultivector2<<S as approx_cmp::AssertAbsDiffAllEq>::AllDebugTolerance>;

    #[inline]
    fn debug_abs_diff_all_tolerance(&self, other: &Self, max_abs_diff: &Self::AllTolerance) -> Self::AllDebugTolerance {
        let data = approx_cmp::AssertAbsDiffAllEq::debug_abs_diff_all_tolerance(&self.data, &other.data, max_abs_diff);

        EuclideanMultivector2 { data }
    }
}

impl<S> approx_cmp::RelativeEq for EuclideanMultivector2<S>
where
    S: ScalarFloat,
{
    type Tolerance = EuclideanMultivector2<<S as approx_cmp::RelativeEq>::Tolerance>;

    #[inline]
    fn relative_eq(&self, other: &Self, max_abs_diff: &Self::Tolerance, max_relative: &Self::Tolerance) -> bool {
        approx_cmp::RelativeEq::relative_eq(&self.data, &other.data, &max_abs_diff.data, &max_relative.data)
    }
}

impl<S> approx_cmp::RelativeAllEq for EuclideanMultivector2<S>
where
    S: ScalarFloat,
{
    type AllTolerance = <S as approx_cmp::RelativeAllEq>::AllTolerance;

    #[inline]
    fn relative_all_eq(&self, other: &Self, max_abs_diff: &Self::AllTolerance, max_relative: &Self::AllTolerance) -> bool {
        approx_cmp::RelativeAllEq::relative_all_eq(&self.data, &other.data, max_abs_diff, max_relative)
    }
}

impl<S> approx_cmp::AssertRelativeEq for EuclideanMultivector2<S>
where
    S: ScalarFloat,
{
    type DebugAbsDiff = EuclideanMultivector2<<S as approx_cmp::AssertRelativeEq>::DebugAbsDiff>;
    type DebugTolerance = EuclideanMultivector2<<S as approx_cmp::AssertRelativeEq>::DebugTolerance>;

    #[inline]
    fn debug_abs_diff(&self, other: &Self) -> Self::DebugAbsDiff {
        let data = approx_cmp::AssertRelativeEq::debug_abs_diff(&self.data, &other.data);

        EuclideanMultivector2 { data }
    }

    #[inline]
    fn debug_abs_diff_tolerance(&self, other: &Self, max_abs_diff: &Self::Tolerance) -> Self::DebugTolerance {
        let data = approx_cmp::AssertRelativeEq::debug_abs_diff_tolerance(&self.data, &other.data, &max_abs_diff.data);

        EuclideanMultivector2 { data }
    }

    #[inline]
    fn debug_relative_tolerance(&self, other: &Self, max_relative: &Self::Tolerance) -> Self::DebugTolerance {
        let data = approx_cmp::AssertRelativeEq::debug_relative_tolerance(&self.data, &other.data, &max_relative.data);

        EuclideanMultivector2 { data }
    }
}

impl<S> approx_cmp::AssertRelativeAllEq for EuclideanMultivector2<S>
where
    S: ScalarFloat,
{
    type AllDebugTolerance = EuclideanMultivector2<<S as approx_cmp::AssertRelativeAllEq>::AllDebugTolerance>;

    #[inline]
    fn debug_abs_diff_all_tolerance(&self, other: &Self, max_abs_diff: &Self::AllTolerance) -> Self::AllDebugTolerance {
        let data = approx_cmp::AssertRelativeAllEq::debug_abs_diff_all_tolerance(&self.data, &other.data, max_abs_diff);

        EuclideanMultivector2 { data }
    }

    #[inline]
    fn debug_relative_all_tolerance(&self, other: &Self, max_relative: &Self::AllTolerance) -> Self::AllDebugTolerance {
        let data = approx_cmp::AssertRelativeAllEq::debug_relative_all_tolerance(&self.data, &other.data, max_relative);

        EuclideanMultivector2 { data }
    }
}

impl<S> approx_cmp::UlpsEq for EuclideanMultivector2<S>
where
    S: ScalarFloat,
{
    type Tolerance = EuclideanMultivector2<<S as approx_cmp::UlpsEq>::Tolerance>;
    type UlpsTolerance = EuclideanMultivector2<<S as approx_cmp::UlpsEq>::UlpsTolerance>;

    #[inline]
    fn ulps_eq(&self, other: &Self, max_abs_diff: &Self::Tolerance, max_ulps: &Self::UlpsTolerance) -> bool {
        approx_cmp::UlpsEq::ulps_eq(&self.data, &other.data, &max_abs_diff.data, &max_ulps.data)
    }
}

impl<S> approx_cmp::UlpsAllEq for EuclideanMultivector2<S>
where
    S: ScalarFloat,
{
    type AllTolerance = <S as approx_cmp::UlpsAllEq>::AllTolerance;
    type AllUlpsTolerance = <S as approx_cmp::UlpsAllEq>::AllUlpsTolerance;

    #[inline]
    fn ulps_all_eq(&self, other: &Self, max_abs_diff: &Self::AllTolerance, max_ulps: &Self::AllUlpsTolerance) -> bool {
        approx_cmp::UlpsAllEq::ulps_all_eq(&self.data, &other.data, max_abs_diff, max_ulps)
    }
}

impl<S> approx_cmp::AssertUlpsEq for EuclideanMultivector2<S>
where
    S: ScalarFloat,
{
    type DebugAbsDiff = EuclideanMultivector2<<S as approx_cmp::AssertUlpsEq>::DebugAbsDiff>;
    type DebugUlpsDiff = EuclideanMultivector2<<S as approx_cmp::AssertUlpsEq>::DebugUlpsDiff>;
    type DebugTolerance = EuclideanMultivector2<<S as approx_cmp::AssertUlpsEq>::DebugTolerance>;
    type DebugUlpsTolerance = EuclideanMultivector2<<S as approx_cmp::AssertUlpsEq>::DebugUlpsTolerance>;

    #[inline]
    fn debug_abs_diff(&self, other: &Self) -> Self::DebugAbsDiff {
        let data = approx_cmp::AssertUlpsEq::debug_abs_diff(&self.data, &other.data);

        EuclideanMultivector2 { data }
    }

    #[inline]
    fn debug_ulps_diff(&self, other: &Self) -> Self::DebugUlpsDiff {
        let data = approx_cmp::AssertUlpsEq::debug_ulps_diff(&self.data, &other.data);

        EuclideanMultivector2 { data }
    }

    #[inline]
    fn debug_abs_diff_tolerance(&self, other: &Self, max_abs_diff: &Self::Tolerance) -> Self::DebugTolerance {
        let data = approx_cmp::AssertUlpsEq::debug_abs_diff_tolerance(&self.data, &other.data, &max_abs_diff.data);

        EuclideanMultivector2 { data }
    }

    #[inline]
    fn debug_ulps_tolerance(&self, other: &Self, max_ulps: &Self::UlpsTolerance) -> Self::DebugUlpsTolerance {
        let data = approx_cmp::AssertUlpsEq::debug_ulps_tolerance(&self.data, &other.data, &max_ulps.data);

        EuclideanMultivector2 { data }
    }
}

impl<S> approx_cmp::AssertUlpsAllEq for EuclideanMultivector2<S>
where
    S: ScalarFloat,
{
    type AllDebugTolerance = EuclideanMultivector2<<S as approx_cmp::AssertUlpsAllEq>::AllDebugTolerance>;
    type AllDebugUlpsTolerance = EuclideanMultivector2<<S as approx_cmp::AssertUlpsAllEq>::AllDebugUlpsTolerance>;

    #[inline]
    fn debug_abs_diff_all_tolerance(&self, other: &Self, max_abs_diff: &Self::AllTolerance) -> Self::AllDebugTolerance {
        let data = approx_cmp::AssertUlpsAllEq::debug_abs_diff_all_tolerance(&self.data, &other.data, max_abs_diff);

        EuclideanMultivector2 { data }
    }

    #[inline]
    fn debug_ulps_all_tolerance(&self, other: &Self, max_ulps: &Self::AllUlpsTolerance) -> Self::AllDebugUlpsTolerance {
        let data = approx_cmp::AssertUlpsAllEq::debug_ulps_all_tolerance(&self.data, &other.data, max_ulps);

        EuclideanMultivector2 { data }
    }
}

impl<S> EuclideanMultivector2<S>
where
    S: ScalarFloat,
{
    /// Calculate the squared magnitude of a multivector.
    pub fn magnitude_squared(&self) -> S {
        let scalar_part = (self * self.reverse())[0];

        scalar_part.abs()
    }

    /// Calculate the magnitude of a multivector.
    pub fn magnitude(&self) -> S {
        self.magnitude_squared().sqrt()
    }

    /// Normalize a multivector to a unit multivector.
    pub fn normalize(&self) -> Self {
        self * (S::one() / self.magnitude())
    }

    /// Normalize a multivector to a specified magnitude.
    pub fn normalize_to(&self, magnitude: S) -> Self {
        self * (magnitude / self.magnitude())
    }

    /// Calculate the squared Euclidean distance between two multivectors.
    pub fn distance_squared(&self, other: &Self) -> S {
        (self - other).magnitude_squared()
    }

    /// Calculate the Euclidean distance between two multivectors.
    pub fn distance(&self, other: &Self) -> S {
        (self - other).magnitude()
    }
}

impl<S> EuclideanMultivector2<S>
where
    S: ScalarFloat,
{
    /// Determine whether a multivector is invertible.
    ///
    /// # Example
    ///
    /// ```
    /// # use cggeomalg::e2ga::EuclideanMultivector2;
    /// #
    /// let e12: EuclideanMultivector2<f64> = EuclideanMultivector2::unit_e12();
    ///
    /// assert!(e12.is_invertible());
    /// ```
    #[inline]
    pub fn is_invertible(&self) -> bool {
        ulps_ne!(
            self.magnitude_squared(),
            S::zero(),
            abs_diff_all <= S::default_epsilon(),
            ulps_all <= S::default_max_ulps()
        )
    }

    /// Compute the multiplicative inverse of a multivector.
    ///
    /// The inverse of a multivector `mv` is a multivector `mv_inv`
    /// such that
    /// ```text
    /// mv * mv_inv = mv_inv * mv = 1
    /// ```
    /// Even though the geometric product is noncommutative, in
    /// dimension two, the left and right inverses are both identical. For
    /// more information on the inversion of multivectors in general, see [1].
    ///
    /// # Example
    ///
    /// ```
    /// # use approx_cmp::assert_relative_eq;
    /// # use cggeomalg::e2ga::EuclideanMultivector2;
    /// #
    /// let mv = EuclideanMultivector2::new(13_f64, -4_f64, 98_f64, 4_f64);
    /// let mv_inv = mv.inverse().unwrap();
    /// let one: EuclideanMultivector2<f64> = EuclideanMultivector2::unit_scalar();
    ///
    /// assert_relative_eq!(mv * mv_inv, one, abs_diff_all <= 1e-10, relative_all <= f64::EPSILON);
    /// assert_relative_eq!(mv_inv * mv, one, abs_diff_all <= 1e-10, relative_all <= f64::EPSILON);
    /// ```
    ///
    /// # References
    ///
    /// [1] _Eckhard Hitzer, Stephen Sangwine. Multivector and multivector matrix
    ///     inverse in real Clifford algebras. Applied Mathematics and Computation
    ///     (311) (2017) 375-389. Elsevier. DOI:10.1016/j.amc.2017.05.027._
    pub fn inverse(&self) -> Option<Self> {
        let magnitude_squared = self.magnitude_squared();
        if magnitude_squared.is_zero() {
            None
        } else {
            Some(self.inverse_unchecked())
        }
    }

    fn inverse_unchecked(&self) -> Self {
        let conjugate = self.conjugate();
        let denominator = (self * conjugate)[0];

        conjugate / denominator
    }

    /// Compute the commutator of two multivectors.
    ///
    /// The commutator of multivectors `mv1` and `mv2` is given by
    /// ```text
    /// comm(mv1, mv2) := (mv1 * mv2 - mv2 * mv1) / 2
    /// ```
    /// where `*` denotes the geometric product.
    ///
    /// # Example
    ///
    /// ```
    /// # use cggeomalg::e2ga::EuclideanMultivector2;
    /// #
    /// let mv1 = EuclideanMultivector2::from_scalar(2_f64);
    /// let mv2 = EuclideanMultivector2::from_scalar(3_f64);
    /// let expected: EuclideanMultivector2<f64> = EuclideanMultivector2::zero();
    /// let result = mv1.commutator(&mv2);
    ///
    /// assert_eq!(result, expected);
    /// ```
    pub fn commutator(&self, other: &Self) -> Self {
        let self_times_other = self * other;
        let other_times_self = other * self;
        let one_over_two = S::one() / (S::one() + S::one());

        (self_times_other - other_times_self) * one_over_two
    }

    /// Compute the commutator of two multivectors.
    ///
    /// This is a synonym for `commutator`.
    #[inline(always)]
    pub fn x(&self, other: &Self) -> Self {
        self.commutator(other)
    }

    /// Compute the anticommutator of two multivectors.
    ///
    /// The anticommutator of multivectors `mv1` and `mv2` is given by
    /// ```text
    /// anticomm(mv1, mv2) := (mv1 * mv2 + mv2 * mv1) / 2
    /// ```
    /// where `*` denotes the geometric product.
    ///
    /// # Example
    ///
    /// ```
    /// # use cggeomalg::e2ga::EuclideanMultivector2;
    /// #
    /// let mv1 = EuclideanMultivector2::from_scalar(2_f64);
    /// let mv2 = EuclideanMultivector2::from_scalar(3_f64);
    /// let expected: EuclideanMultivector2<f64> = mv1 * mv2;
    /// let result = mv1.anticommutator(&mv2);
    ///
    /// assert_eq!(result, expected);
    /// ```
    pub fn anticommutator(&self, other: &Self) -> Self {
        let self_times_other = self * other;
        let other_times_self = other * self;
        let one_over_two = S::one() / (S::one() + S::one());

        (self_times_other + other_times_self) * one_over_two
    }
}

impl<S> ops::Div<S> for EuclideanMultivector2<S>
where
    S: ScalarFloat,
{
    type Output = EuclideanMultivector2<S>;

    #[rustfmt::skip]
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
    S: ScalarFloat,
{
    type Output = EuclideanMultivector2<S>;

    #[rustfmt::skip]
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
    S: ScalarFloat,
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
    S: ScalarFloat,
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
    S: ScalarFloat,
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
    S: ScalarFloat,
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
    S: Scalar,
{
    type Output = EuclideanMultivector2<S>;

    #[rustfmt::skip]
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
    S: Scalar,
{
    type Output = EuclideanMultivector2<S>;

    #[rustfmt::skip]
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
    S: Scalar,
{
    type Output = EuclideanMultivector2<S>;

    #[rustfmt::skip]
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
    S: Scalar,
{
    type Output = EuclideanMultivector2<S>;

    #[rustfmt::skip]
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
    S: Scalar,
{
    type Output = EuclideanMultivector2<S>;

    #[inline]
    fn shl(self, other: S) -> Self::Output {
        let a = self;
        let result_1 = a[0] * other;

        EuclideanMultivector2::from_scalar(result_1)
    }
}

impl<S> ops::Shl<S> for &EuclideanMultivector2<S>
where
    S: Scalar,
{
    type Output = EuclideanMultivector2<S>;

    #[inline]
    fn shl(self, other: S) -> Self::Output {
        let a = self;
        let result_1 = a[0] * other;

        EuclideanMultivector2::from_scalar(result_1)
    }
}

impl<S> ops::Shr<EuclideanMultivector2<S>> for EuclideanMultivector2<S>
where
    S: Scalar,
{
    type Output = EuclideanMultivector2<S>;

    #[rustfmt::skip]
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
    S: Scalar,
{
    type Output = EuclideanMultivector2<S>;

    #[rustfmt::skip]
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
    S: Scalar,
{
    type Output = EuclideanMultivector2<S>;

    #[rustfmt::skip]
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
    S: Scalar,
{
    type Output = EuclideanMultivector2<S>;

    #[rustfmt::skip]
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
    S: Scalar,
{
    type Output = EuclideanMultivector2<S>;

    #[rustfmt::skip]
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
    S: Scalar,
{
    type Output = EuclideanMultivector2<S>;

    #[rustfmt::skip]
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
    };
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
    };
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
                assert!(
                    result.is_some(),
                    "Attempt to divide by a multivector with zero magnitude: {:?}",
                    other
                );
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
                assert!(
                    result.is_some(),
                    "Attempt to divide by a multivector with zero magnitude: {:?}",
                    other
                );
                let mut result = result.unwrap();
                result[0] = self * result[0];
                result[1] = self * result[1];
                result[2] = self * result[2];
                result[3] = self * result[3];

                result
            }
        }
    };
}

impl_scalar_multivector_div_ops!(f32);
impl_scalar_multivector_div_ops!(f64);
