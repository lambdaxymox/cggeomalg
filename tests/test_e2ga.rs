extern crate cggeomalg;
extern crate num_traits;


#[cfg(test)]
mod e2ga_test {
    use cggeomalg::e2ga::{
        EuclideanMultivector2,
    };


    #[test]
    fn test_components1() {
        let mv = EuclideanMultivector2::new(1, 2, 3, 4);

        assert_eq!(mv[0], 1);
        assert_eq!(mv[1], 2);
        assert_eq!(mv[2], 3);
        assert_eq!(mv[3], 4);
    }

    #[test]
    fn test_components2() {
        let mv: EuclideanMultivector2<isize> = EuclideanMultivector2::new(1, 2, 3, 4);

        assert_eq!(mv.scalar, mv[0]);
        assert_eq!(mv.e1, mv[1]);
        assert_eq!(mv.e2, mv[2]);
        assert_eq!(mv.e12, mv[3]);
    }

    #[test]
    fn test_as_ref() {
        let mv = EuclideanMultivector2::new(1, 2, 3, 4);
        let v_ref: &[isize; 4] = mv.as_ref();

        assert_eq!(v_ref, &[1, 2, 3, 4]);
    }

    #[test]
    fn test_as_mut() {
        let mut mv = EuclideanMultivector2::new(1, 2, 3, 4);
        let v_ref: &mut [isize; 4] = mv.as_mut();

        assert_eq!(v_ref, &mut [1, 2, 3, 4]);
    }


    #[test]
    fn test_multivector_addition() {
        let mv1: EuclideanMultivector2<isize> = EuclideanMultivector2::new(1, 2, 3, 4);
        let mv2: EuclideanMultivector2<isize> = EuclideanMultivector2::new(5, 6, 7, 8);
        let expected = EuclideanMultivector2::new(6, 8, 10, 12);
        let result = mv1 + mv2;

        assert_eq!(result, expected);
    }

    #[test]
    fn test_mutlivector_plus_zero() {
        let mv1 = EuclideanMultivector2::new(1, 2, 3, 4);
        let zero = EuclideanMultivector2::zero();
        let expected = mv1;
        let result = mv1 + zero;

        assert_eq!(result, expected);
    }

    #[test]
    fn test_zero_plus_multivector() {
        let mv1 = EuclideanMultivector2::new(1, 2, 3, 4);
        let zero = EuclideanMultivector2::zero();
        let expected = mv1;
        let result = zero + mv1;

        assert_eq!(result, expected);
    }

    #[test]
    fn test_multivector_subtraction() {
        let mv1: EuclideanMultivector2<isize> = EuclideanMultivector2::new(4, 6, 1, 7);
        let mv2: EuclideanMultivector2<isize> = EuclideanMultivector2::new(1, 6, 7, 10);
        let expected = EuclideanMultivector2::new(3, 0, -6, -3);
        let result = mv1 - mv2;

        assert_eq!(result, expected);
    }

    #[test]
    fn test_multivector_minus_zero() {
        let mv: EuclideanMultivector2<isize> = EuclideanMultivector2::new(5, 75, 2, 92);
        let zero = EuclideanMultivector2::zero();
        let expected = mv;
        let result = mv - zero;

        assert_eq!(result, expected);
    }

    #[test]
    fn test_zero_minus_multivector() {
        let mv: EuclideanMultivector2<isize> = EuclideanMultivector2::new(5, 75, 2, 92);
        let zero = EuclideanMultivector2::zero();
        let expected = -mv;
        let result = zero - mv;

        assert_eq!(result, expected);
    }

    #[test]
    fn test_multivector_minus_multivector_equals_zero() {
        let mv = EuclideanMultivector2::new(5, 75, 2, 92);
        let zero = EuclideanMultivector2::zero();
        
        assert_eq!(mv - mv, zero);
    }

    #[test]
    fn test_multivector_additive_inverse() {
        let mv = EuclideanMultivector2::new(5, 75, 2, 92);
        let zero = EuclideanMultivector2::zero();
        
        assert_eq!(-mv + mv, zero);
        assert_eq!(mv + (-mv), zero);
    }

    #[test]
    fn test_scalar_multiplication() {
        let mv: EuclideanMultivector2<isize> = EuclideanMultivector2::new(1, 2, 3, 4);
        let scalar = 9;
        let expected = EuclideanMultivector2::new(9, 18, 27, 36);
        let result = mv * scalar;

        assert_eq!(result, expected);
    }

    #[test]
    fn test_scalar_division() {
        let mv = EuclideanMultivector2::new(1_f64, 2_f64, 3_f64, 4_f64);
        let scalar = 9_f64;
        let expected = EuclideanMultivector2::new(
            1_f64 / 9_f64, 
            2_f64 / 9_f64, 
            3_f64 / 9_f64, 
            4_f64 / 9_f64
        );
        let result = mv / scalar;

        assert_eq!(result, expected);
    }

    #[test]
    #[should_panic]
    fn test_out_of_bounds_array_access() {
        let mv = EuclideanMultivector2::new(1, 2, 3, 4);

        assert_eq!(mv[4], mv[4]);
    }

    #[test]
    fn test_multivector_times_scalar_zero_is_zero() {
        let mv: EuclideanMultivector2<isize> = EuclideanMultivector2::new(1, 2, 3, 4);
        let expected: EuclideanMultivector2<isize> = EuclideanMultivector2::zero();
        let result = mv * 0;

        assert_eq!(result, expected);
    }

    #[test]
    fn test_scalar_zero_times_multivector_is_zero() {
        let mv: EuclideanMultivector2<isize> = EuclideanMultivector2::new(1, 2, 3, 4);
        let expected: EuclideanMultivector2<isize> = EuclideanMultivector2::zero();

        assert_eq!(true, false);
    }

    #[test]
    fn test_geometric_product_e1_e2() {
        let e1 = EuclideanMultivector2::unit_e1();
        let e2 = EuclideanMultivector2::unit_e2();
        let e12: EuclideanMultivector2<f64> = EuclideanMultivector2::unit_e12();

        assert_eq!(e1 * e2, e12);
    }

    #[test]
    fn test_geometric_product_e2_e1() {
        let e1 = EuclideanMultivector2::unit_e1();
        let e2 = EuclideanMultivector2::unit_e2();
        let e12: EuclideanMultivector2<f64> = EuclideanMultivector2::unit_e12();

        assert_eq!(e2 * e1, -e12);
    }

    #[test]
    fn test_geometric_product_e1_e12() {
        let e1 = EuclideanMultivector2::unit_e1();
        let e2 = EuclideanMultivector2::unit_e2();
        let e12: EuclideanMultivector2<f64> = EuclideanMultivector2::unit_e12();

        assert_eq!(e1 * e12, e2);
    }

    #[test]
    fn test_geometric_product_e12_e1() {
        let e1 = EuclideanMultivector2::unit_e1();
        let e2 = EuclideanMultivector2::unit_e2();
        let e12: EuclideanMultivector2<f64> = EuclideanMultivector2::unit_e12();

        assert_eq!(e12 * e1, -e2);
    }

    #[test]
    fn test_geometric_product_e2_e12() {
        let e1 = EuclideanMultivector2::unit_e1();
        let e2 = EuclideanMultivector2::unit_e2();
        let e12: EuclideanMultivector2<f64> = EuclideanMultivector2::unit_e12();

        assert_eq!(e2 * e12, -e1);
    }

    #[test]
    fn test_geometric_product_e12_e2() {
        let e1 = EuclideanMultivector2::unit_e1();
        let e2 = EuclideanMultivector2::unit_e2();
        let e12: EuclideanMultivector2<f64> = EuclideanMultivector2::unit_e12();

        assert_eq!(e12 * e2, e1);
    }

    #[test]
    fn test_geometric_product_e12_e12() {
        let e12: EuclideanMultivector2<f64> = EuclideanMultivector2::unit_e12();
        let one = EuclideanMultivector2::unit_scalar();

        assert_eq!(e12 * e12, -one);
    }

    #[test]
    fn test_geometric_product_e12_e21() {
        let e12: EuclideanMultivector2<f64> = EuclideanMultivector2::unit_e12();
        let e21: EuclideanMultivector2<f64> = -EuclideanMultivector2::unit_e12();
        let one = EuclideanMultivector2::unit_scalar();

        assert_eq!(e12 * e21, one);
    }

    #[test]
    fn test_geometric_product_e21_e12() {
        let e12: EuclideanMultivector2<f64> = EuclideanMultivector2::unit_e12();
        let e21: EuclideanMultivector2<f64> = -EuclideanMultivector2::unit_e12();
        let one = EuclideanMultivector2::unit_scalar();


        assert_eq!(e21 * e12, one);
    }

    #[test]
    fn test_magnitude1() {
        let mv = EuclideanMultivector2::new(4.0, 0.0, 0.0, 0.0);
        let expected = 4.0;
        let result = mv.magnitude();

        assert_eq!(result, expected);
    }

    #[test]
    fn test_magnitude2() {
        let mv = EuclideanMultivector2::new(0.0, 4.0, 0.0, 0.0);
        let expected = 4.0;
        let result = mv.magnitude();

        assert_eq!(result, expected);
    }

    #[test]
    fn test_magnitude3() {
        let mv = EuclideanMultivector2::new(0.0, 0.0, 4.0, 0.0);
        let expected = 4.0;
        let result = mv.magnitude();

        assert_eq!(result, expected);
    }

    #[test]
    fn test_magnitude4() {
        let mv = EuclideanMultivector2::new(0.0, 0.0, 0.0, 4.0);
        let expected = 4.0;
        let result = mv.magnitude();

        assert_eq!(result, expected);
    }

    #[test]
    fn test_magnitude_unit_multivectors() {
        let unit_scalar: EuclideanMultivector2<f64> = EuclideanMultivector2::unit_scalar();
        let unit_e1: EuclideanMultivector2<f64> = EuclideanMultivector2::unit_e1();
        let unit_e2: EuclideanMultivector2<f64> = EuclideanMultivector2::unit_e2();
        let unit_e12: EuclideanMultivector2<f64> = EuclideanMultivector2::unit_e12();

        assert_eq!(unit_scalar.magnitude_squared(), 1.0);
        assert_eq!(unit_scalar.magnitude(), 1.0);
        assert_eq!(unit_e1.magnitude_squared(), 1.0);
        assert_eq!(unit_e1.magnitude(), 1.0);
        assert_eq!(unit_e2.magnitude_squared(), 1.0);
        assert_eq!(unit_e2.magnitude(), 1.0);
        assert_eq!(unit_e12.magnitude_squared(), 1.0);
        assert_eq!(unit_e12.magnitude(), 1.0);
    }

    #[test]
    fn test_magnitude_zero_multivector() {
        let zero: EuclideanMultivector2<f64> = EuclideanMultivector2::zero();

        assert_eq!(zero.magnitude(), 0.0);
    }

    #[test]
    fn test_magnitude_arbitrary_multivector1() {
        let mv = EuclideanMultivector2::new(1.0, 2.0, 3.0, 4.0);
        let expected = 30.0;
        let result = mv.magnitude_squared();

        assert_eq!(result, expected);
    }

    #[test]
    fn test_magnitude_arbitrary_multivector2() {
        let mv = EuclideanMultivector2::new(3.0, 35.0, 13.0, 94.0);
        let expected = 10239.0;
        let result = mv.magnitude_squared();

        assert_eq!(result, expected);
    }

    #[test]
    fn test_multivector_reverse1() {
        let mv = EuclideanMultivector2::new(1.0, 2.0, 3.0, 4.0);
        let expected = EuclideanMultivector2::new(1.0, 2.0, 3.0, -4.0);;
        let result = mv.reverse();

        assert_eq!(result, expected);
    }

    #[test]
    fn test_multivector_reverse2() {
        let unit_scalar: EuclideanMultivector2<f64> = EuclideanMultivector2::unit_scalar();
        let unit_e1: EuclideanMultivector2<f64> = EuclideanMultivector2::unit_e1();
        let unit_e2: EuclideanMultivector2<f64> = EuclideanMultivector2::unit_e2();
        let unit_e12: EuclideanMultivector2<f64> = EuclideanMultivector2::unit_e12();

        assert_eq!(unit_scalar.reverse(), unit_scalar);
        assert_eq!(unit_e1.reverse(), unit_e1);
        assert_eq!(unit_e2.reverse(), unit_e2);
        assert_eq!(unit_e12.reverse(), -unit_e12);
    }

    #[test]
    fn test_multivector_inverse() {
        let mv = EuclideanMultivector2::new(1.0, 2.0, 3.0, 4.0);
        let expected = EuclideanMultivector2::new(0.25, -0.5, -0.75, -1.0);
        let result = mv.inverse().unwrap();

        assert_eq!(result, expected);
    }

    #[test]
    fn test_multivector_times_inverse1() {
        let mv = EuclideanMultivector2::new(3.0, 35.0, 13.0, 94.0);
        let mv_inv = mv.inverse().unwrap();
        let expected: EuclideanMultivector2<f64> = EuclideanMultivector2::unit_scalar();
        let result = mv * mv_inv;

        assert_eq!(result, expected);
    }

    #[test]
    fn test_multivector_times_inverse2() {
        let mv = EuclideanMultivector2::new(3.0, 35.0, 13.0, 94.0);
        let mv_inv = mv.inverse().unwrap();
        let expected: EuclideanMultivector2<f64> = EuclideanMultivector2::unit_scalar();
        let result = mv_inv * mv;

        assert_eq!(result, expected);
    }

    #[test]
    fn test_volume_element_inverse() {
        let e12: EuclideanMultivector2<f64> = EuclideanMultivector2::unit_e12();
        let expected = -e12;
        let result = e12.inverse().unwrap();
        
        assert_eq!(result, expected);
    }

    #[test]
    fn test_vector_inverse1() {
        let e1: EuclideanMultivector2<f64> = EuclideanMultivector2::unit_e1();
        let mv = e1 * 2_f64;
        let expected = e1 * (1_f64 / 2_f64);
        let result = mv.inverse().unwrap();
        
        assert_eq!(result, expected);
    }

    #[test]
    fn test_vector_inverse2() {
        let e2: EuclideanMultivector2<f64> = EuclideanMultivector2::unit_e2();
        let mv = e2 * 2_f64;
        let expected = e2 * (1_f64 / 2_f64);
        let result = mv.inverse().unwrap();
        
        assert_eq!(result, expected);
    }

    #[test]
    fn test_scalar_inverse() {
        let unit_scalar: EuclideanMultivector2<f64> = EuclideanMultivector2::unit_scalar();
        let scalar = unit_scalar * 2_f64;
        let expected = unit_scalar *  (1_f64 / 2_f64);
        let result = scalar.inverse().unwrap();

        assert_eq!(result, expected);
    }

    #[test]
    fn test_zero_multivector_is_not_invertible() {
        let zero: EuclideanMultivector2<f64> = EuclideanMultivector2::zero();

        assert!(!zero.is_invertible());
    }

    /// In an Euclidean geometric algebra, the square of the volume 
    /// element should be negative one. That is, let `I` denote the volume element. 
    /// Then
    /// ```text
    /// I^2 := I * I == -1
    /// ```
    #[test]
    fn test_volume_element_squared_equals_negative_one() {
        let e12: EuclideanMultivector2<f64> = EuclideanMultivector2::unit_e12();
        let one = EuclideanMultivector2::unit_scalar();

        assert_eq!(e12 * e12, -one);
    }
}

