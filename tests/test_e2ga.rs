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
        let unit_scalar: EuclideanMultivector2<f64> = EuclideanMultivector2::unit_c();
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
    fn test_multivector_inverse() {
        let mv = EuclideanMultivector2::new(3.0, 35.0, 13.0, 94.0);
        let expected = EuclideanMultivector2::new(3.0, 35.0, 13.0, 94.0);
        let result = mv.inverse().unwrap();

        assert_eq!(result, expected);
    }

    #[test]
    fn test_multivector_times_inverse1() {
        let mv = EuclideanMultivector2::new(3.0, 35.0, 13.0, 94.0);
        let mv_inv = mv.inverse().unwrap();
        let expected: EuclideanMultivector2<f64> = EuclideanMultivector2::unit_c();
        let result = mv * mv_inv;

        assert_eq!(result, expected);
    }

    #[test]
    fn test_multivector_times_inverse2() {
        let mv = EuclideanMultivector2::new(3.0, 35.0, 13.0, 94.0);
        let mv_inv = mv.inverse().unwrap();
        let expected: EuclideanMultivector2<f64> = EuclideanMultivector2::unit_c();
        let result = mv_inv * mv;

        assert_eq!(result, expected);
    }
}

