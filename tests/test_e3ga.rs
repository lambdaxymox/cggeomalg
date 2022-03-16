extern crate cggeomalg;
extern crate approx;
extern crate num_traits;


#[cfg(test)]
mod e3ga_tests {
    use cggeomalg::e3ga::{
        EuclideanMultivector3,
    };
    use approx::{
        assert_relative_eq,
    };


    #[test]
    fn test_components1() {
        let mv = EuclideanMultivector3::new(1, 2, 3, 4, 5, 6, 7, 8);

        assert_eq!(mv[0], 1);
        assert_eq!(mv[1], 2);
        assert_eq!(mv[2], 3);
        assert_eq!(mv[3], 4);
        assert_eq!(mv[4], 5);
        assert_eq!(mv[5], 6);
        assert_eq!(mv[6], 7);
        assert_eq!(mv[7], 8);
    }

    #[test]
    fn test_components2() {
        let mv = EuclideanMultivector3::new(1, 2, 3, 4, 5, 6, 7, 8);

        assert_eq!(mv.scalar, mv[0]);
        assert_eq!(mv.e1, mv[1]);
        assert_eq!(mv.e2, mv[2]);
        assert_eq!(mv.e3, mv[3]);
        assert_eq!(mv.e12, mv[4]);
        assert_eq!(mv.e23, mv[5]);
        assert_eq!(mv.e31, mv[6]);
        assert_eq!(mv.e123, mv[7]);
    }

    #[test]
    fn test_as_ref() {
        let mv = EuclideanMultivector3::new(1, 2, 3, 4, 5, 6, 7, 8);
        let v_ref: &[isize; 8] = mv.as_ref();

        assert_eq!(v_ref, &[1, 2, 3, 4, 5, 6, 7, 8]);
    }

    #[test]
    fn test_as_mut() {
        let mut mv = EuclideanMultivector3::new(1, 2, 3, 4, 5, 6, 7, 8);
        let v_ref: &mut [isize; 8] = mv.as_mut();

        assert_eq!(v_ref, &mut [1, 2, 3, 4, 5, 6, 7, 8]);
    }

    #[test]
    fn test_multivector_addition() {
        let mv1: EuclideanMultivector3<isize> = EuclideanMultivector3::new(1, 2, 3, 4, 5, 6, 7, 8);
        let mv2: EuclideanMultivector3<isize> = EuclideanMultivector3::new(9, 10, 11, 12, 13, 14, 15, 16);
        let expected = EuclideanMultivector3::new(10, 12, 14, 16, 18, 20, 22, 24);
        let result = mv1 + mv2;

        assert_eq!(result, expected);
    }

    #[test]
    fn test_multivector_plus_zero() {
        let mv1 = EuclideanMultivector3::new(1, 2, 3, 4, 5, 6, 7, 8);
        let zero = EuclideanMultivector3::zero();
        let expected = mv1;
        let result = mv1 + zero;

        assert_eq!(result, expected);
    }

    #[test]
    fn test_zero_plus_multivector() {
        let mv1 = EuclideanMultivector3::new(1, 2, 3, 4, 5, 6, 7, 8);
        let zero = EuclideanMultivector3::zero();
        let expected = mv1;
        let result = zero + mv1;

        assert_eq!(result, expected);
    }

    #[test]
    fn test_multivector_subtraction() {
        let mv1: EuclideanMultivector3<isize> = EuclideanMultivector3::new(4, 6, 1, 7, 3, 6, 2, 8);
        let mv2: EuclideanMultivector3<isize> = EuclideanMultivector3::new(1, 6, 7, 10, 19, 1, 0, 4);
        let expected = EuclideanMultivector3::new(3, 0, -6, -3, -16, 5, 2, 4);
        let result = mv1 - mv2;

        assert_eq!(result, expected);
    }

    #[test]
    fn test_multivector_minus_zero() {
        let mv: EuclideanMultivector3<isize> = EuclideanMultivector3::new(5, 75, 2, 92, 12, 213, 9, 83);
        let zero = EuclideanMultivector3::zero();
        let expected = mv;
        let result = mv - zero;

        assert_eq!(result, expected);
    }

    #[test]
    fn test_zero_minus_multivector() {
        let mv: EuclideanMultivector3<isize> = EuclideanMultivector3::new(5, 75, 2, 92, 12, 213, 9, 83);
        let zero = EuclideanMultivector3::zero();
        let expected = -mv;
        let result = zero - mv;

        assert_eq!(result, expected);
    }

    #[test]
    fn test_multivector_minus_multivector_equals_zero() {
        let mv = EuclideanMultivector3::new(5, 75, 2, 92, 12, 213, 9, 83);
        let zero = EuclideanMultivector3::zero();
        
        assert_eq!(mv - mv, zero);
    }

    #[test]
    fn test_multivector_additive_inverse() {
        let mv = EuclideanMultivector3::new(5, 75, 2, 92, 12, 213, 9, 83);
        let zero = EuclideanMultivector3::zero();
        
        assert_eq!(-mv + mv, zero);
        assert_eq!(mv + (-mv), zero);
    }

    #[test]
    fn test_scalar_multiplication() {
        let mv: EuclideanMultivector3<isize> = EuclideanMultivector3::new(1, 2, 3, 4, 5, 6, 7, 8);
        let scalar = 9;
        let expected = EuclideanMultivector3::new(9, 18, 27, 36, 45, 54, 63, 72);
        let result = mv * scalar;

        assert_eq!(result, expected);
    }

    #[test]
    fn test_scalar_division() {
        let mv = EuclideanMultivector3::new(1_f64, 2_f64, 3_f64, 4_f64, 5_f64, 6_f64, 7_f64, 8_f64);
        let scalar = 9_f64;
        let expected = EuclideanMultivector3::new(
            1_f64 / 9_f64, 
            2_f64 / 9_f64, 3_f64 / 9_f64, 4_f64 / 9_f64,
            5_f64 / 9_f64,6_f64 / 9_f64, 7_f64 / 9_f64,
            8_f64 / 9_f64
        );
        let result = mv / scalar;

        assert_relative_eq!(result, expected, epsilon = 1e-10);
    }

    #[test]
    #[should_panic]
    fn test_out_of_bounds_array_access() {
        let mv = EuclideanMultivector3::new(1, 2, 3, 4, 5, 6, 7, 8);

        assert_eq!(mv[8], mv[8]);
    }

    #[test]
    fn test_multivector_times_scalar_zero_is_zero() {
        let mv: EuclideanMultivector3<isize> = EuclideanMultivector3::new(1, 2, 3, 4, 5, 6, 7, 8);
        let expected: EuclideanMultivector3<isize> = EuclideanMultivector3::zero();
        let result = mv * 0;

        assert_eq!(result, expected);
    }

    #[test]
    fn test_scalar_zero_times_multivector_is_zero() {
        let mv: EuclideanMultivector3<isize> = EuclideanMultivector3::new(1, 2, 3, 4, 5, 6, 7, 8);
        let expected: EuclideanMultivector3<isize> = EuclideanMultivector3::zero();
        let result = 0 * mv;

        assert_eq!(result, expected);
    }
 
    #[test]
    fn test_outer_product_scalar_e1() {
        let scalar = EuclideanMultivector3::unit_scalar() * 3_f64;
        let e1: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e1();
        let expected = e1 * scalar;

        assert_eq!(scalar ^ e1, expected);
        assert_eq!(e1 ^ scalar, expected);
    }

    #[test]
    fn test_outer_product_scalar_e2() {
        let scalar = EuclideanMultivector3::unit_scalar() * 3_f64;
        let e2: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e2();
        let expected = e2 * scalar;

        assert_eq!(scalar ^ e2, expected);
        assert_eq!(e2 ^ scalar, expected);
    }

    #[test]
    fn test_outer_product_scalar_e3() {
        let scalar = EuclideanMultivector3::unit_scalar() * 3_f64;
        let e3: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e3();
        let expected = e3 * scalar;

        assert_eq!(scalar ^ e3, expected);
        assert_eq!(e3 ^ scalar, expected);
    }

    #[test]
    fn test_outer_product_scalar_e12() {
        let scalar = EuclideanMultivector3::unit_scalar() * 3_f64;
        let e12: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e12();
        let expected = e12 * scalar;

        assert_eq!(scalar ^ e12, expected);
        assert_eq!(e12 ^ scalar, expected);
    }

    #[test]
    fn test_outer_product_scalar_e23() {
        let scalar = EuclideanMultivector3::unit_scalar() * 3_f64;
        let e23: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e23();
        let expected = e23 * scalar;

        assert_eq!(scalar ^ e23, expected);
        assert_eq!(e23 ^ scalar, expected);
    }

    #[test]
    fn test_outer_product_scalar_e31() {
        let scalar = EuclideanMultivector3::unit_scalar() * 3_f64;
        let e31: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e31();
        let expected = e31 * scalar;

        assert_eq!(scalar ^ e31, expected);
        assert_eq!(e31 ^ scalar, expected);
    }

    #[test]
    fn test_outer_product_scalar_e123() {
        let scalar = EuclideanMultivector3::unit_scalar() * 3_f64;
        let e123: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e123();
        let expected = e123 * scalar;

        assert_eq!(scalar ^ e123, expected);
        assert_eq!(e123 ^ scalar, expected);
    }

    #[test]
    fn test_outer_product_e1_e1() {
        let e1: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e1();
        let zero: EuclideanMultivector3<f64> = EuclideanMultivector3::zero();

        assert_eq!(e1 ^ e1, zero);
    }

    #[test]
    fn test_outer_product_e1_e2() {
        let e1: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e1();
        let e2: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e2();
        let e12: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e12();

        assert_eq!(e1 ^ e2, e12);
    }

    #[test]
    fn test_outer_product_e1_e3() {
        let e1: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e1();
        let e3: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e3();
        let e31: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e31();

        assert_eq!(e1 ^ e3, -e31);
    }

    #[test]
    fn test_outer_product_e1_e12() {
        let e1: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e1();
        let e12: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e12();
        let zero: EuclideanMultivector3<f64> = EuclideanMultivector3::zero();

        assert_eq!(e1 ^ e12, zero);
    }

    #[test]
    fn test_outer_product_e1_e23() {
        let e1: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e1();
        let e23: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e23();
        let e123: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e123();

        assert_eq!(e1 ^ e23, e123);
    }

    #[test]
    fn test_outer_product_e1_e31() {
        let e1: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e1();
        let e31: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e31();
        let zero: EuclideanMultivector3<f64> = EuclideanMultivector3::zero();

        assert_eq!(e1 ^ e31, zero);
    }

    #[test]
    fn test_outer_product_e1_e123() {
        let e1: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e1();
        let e123: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e123();
        let zero: EuclideanMultivector3<f64> = EuclideanMultivector3::zero();

        assert_eq!(e1 ^ e123, zero);
    }

    #[test]
    fn test_outer_product_e2_e1() {
        let e1: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e1();
        let e2: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e2();
        let e12: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e12();

        assert_eq!(e2 ^ e1, -e12);
    }

    #[test]
    fn test_outer_product_e2_e2() {
        let e2: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e2();
        let zero: EuclideanMultivector3<f64> = EuclideanMultivector3::zero();

        assert_eq!(e2 ^ e2, zero);
    }

    #[test]
    fn test_outer_product_e2_e3() {
        let e2: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e2();
        let e3: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e3();
        let e23: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e23();

        assert_eq!(e2 ^ e3, e23);
    }

    #[test]
    fn test_outer_product_e2_e12() {
        let e2: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e2();
        let e12: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e12();
        let zero: EuclideanMultivector3<f64> = EuclideanMultivector3::zero();

        assert_eq!(e2 ^ e12, zero);
    }

    #[test]
    fn test_outer_product_e2_e23() {
        let e2: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e2();
        let e23: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e23();
        let zero: EuclideanMultivector3<f64> = EuclideanMultivector3::zero();

        assert_eq!(e2 ^ e23, zero);
    }

    #[test]
    fn test_outer_product_e2_e31() {
        let e2: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e2();
        let e31: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e31();
        let e123: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e123();

        assert_eq!(e2 ^ e31, e123);
    }

    #[test]
    fn test_outer_product_e2_e123() {
        let e2: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e2();
        let e123: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e123();
        let zero: EuclideanMultivector3<f64> = EuclideanMultivector3::zero();

        assert_eq!(e2 ^ e123, zero);
    }

    #[test]
    fn test_outer_product_e3_e1() {
        let e1: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e1();
        let e3: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e3();
        let e31: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e31();

        assert_eq!(e3 ^ e1, e31);
    }

    #[test]
    fn test_outer_product_e3_e2() {
        let e2: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e2();
        let e3: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e3();
        let e23: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e23();

        assert_eq!(e3 ^ e2, -e23);
    }

    #[test]
    fn test_outer_product_e3_e3() {
        let e3: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e3();
        let zero: EuclideanMultivector3<f64> = EuclideanMultivector3::zero();

        assert_eq!(e3 ^ e3, zero);
    }

    #[test]
    fn test_outer_product_e3_e12() {
        let e3: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e3();
        let e12: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e12();
        let e123: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e123();

        assert_eq!(e3 ^ e12, e123);
    }

    #[test]
    fn test_outer_product_e3_e23() {
        let e3: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e3();
        let e23: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e23();
        let zero: EuclideanMultivector3<f64> = EuclideanMultivector3::zero();

        assert_eq!(e3 ^ e23, zero);
    }

    #[test]
    fn test_outer_product_e3_e31() {
        let e3: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e3();
        let e31: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e31();
        let zero: EuclideanMultivector3<f64> = EuclideanMultivector3::zero();

        assert_eq!(e3 ^ e31, zero);
    }

    #[test]
    fn test_outer_product_e3_e123() {
        let e3: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e3();
        let e123: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e123();
        let zero: EuclideanMultivector3<f64> = EuclideanMultivector3::zero();

        assert_eq!(e3 ^ e123, zero);
    }

    #[test]
    fn test_outer_product_e12_e1() {
        let e1: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e1();
        let e12: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e12();
        let zero: EuclideanMultivector3<f64> = EuclideanMultivector3::zero();

        assert_eq!(e12 ^ e1, zero);
    }

    #[test]
    fn test_outer_product_e12_e2() {
        let e2: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e2();
        let e12: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e12();
        let zero: EuclideanMultivector3<f64> = EuclideanMultivector3::zero();

        assert_eq!(e12 ^ e2, zero);
    }

    #[test]
    fn test_outer_product_e12_e3() {
        let e3: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e3();
        let e12: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e12();
        let e123: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e123();

        assert_eq!(e12 ^ e3, e123);
    }

    #[test]
    fn test_outer_product_e12_e12() {
        let e12: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e12();
        let zero: EuclideanMultivector3<f64> = EuclideanMultivector3::zero();

        assert_eq!(e12 ^ e12, zero);
    }

    #[test]
    fn test_outer_product_e12_e23() {
        let e12: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e12();
        let e23: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e23();
        let zero: EuclideanMultivector3<f64> = EuclideanMultivector3::zero();

        assert_eq!(e12 ^ e23, zero);
    }

    #[test]
    fn test_outer_product_e12_e31() {
        let e12: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e12();
        let e31: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e31();
        let zero: EuclideanMultivector3<f64> = EuclideanMultivector3::zero();

        assert_eq!(e12 ^ e31, zero);
    }

    #[test]
    fn test_outer_product_e12_e123() {
        let e12: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e12();
        let e123: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e123();
        let zero: EuclideanMultivector3<f64> = EuclideanMultivector3::zero();

        assert_eq!(e12 ^ e123, zero);
    }

    #[test]
    fn test_outer_product_e23_e1() {
        let e1: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e1();
        let e23: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e23();
        let e123: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e123();

        assert_eq!(e23 ^ e1, e123);
    }

    #[test]
    fn test_outer_product_e23_e2() {
        let e2: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e2();
        let e23: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e23();
        let zero: EuclideanMultivector3<f64> = EuclideanMultivector3::zero();

        assert_eq!(e23 ^ e2, zero);
    }

    #[test]
    fn test_outer_product_e23_e3() {
        let e3: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e3();
        let e23: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e23();
        let zero: EuclideanMultivector3<f64> = EuclideanMultivector3::zero();

        assert_eq!(e23 ^ e3, zero);
    }

    #[test]
    fn test_outer_product_e23_e12() {
        let e12: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e12();
        let e23: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e23();
        let zero: EuclideanMultivector3<f64> = EuclideanMultivector3::zero();

        assert_eq!(e23 ^ e12, zero);
    }

    #[test]
    fn test_outer_product_e23_e23() {
        let e23: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e23();
        let zero: EuclideanMultivector3<f64> = EuclideanMultivector3::zero();

        assert_eq!(e23 ^ e23, zero);
    }

    #[test]
    fn test_outer_product_e23_e31() {
        let e23: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e23();
        let e31: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e31();
        let zero: EuclideanMultivector3<f64> = EuclideanMultivector3::zero();

        assert_eq!(e23 ^ e31, zero);
    }

    #[test]
    fn test_outer_product_e23_e123() {
        let e23: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e23();
        let e123: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e123();
        let zero: EuclideanMultivector3<f64> = EuclideanMultivector3::zero();

        assert_eq!(e23 ^ e123, zero);
    }

    #[test]
    fn test_outer_product_e31_e1() {
        let e1: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e1();
        let e31: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e31();
        let zero: EuclideanMultivector3<f64> = EuclideanMultivector3::zero();

        assert_eq!(e31 ^ e1, zero);
    }

    #[test]
    fn test_outer_product_e31_e2() {
        let e2: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e2();
        let e31: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e31();
        let e123: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e123();

        assert_eq!(e31 ^ e2, e123);
    }

    #[test]
    fn test_outer_product_e31_e3() {
        let e3: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e3();
        let e31: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e31();
        let zero: EuclideanMultivector3<f64> = EuclideanMultivector3::zero();

        assert_eq!(e31 ^ e3, zero);
    }

    #[test]
    fn test_outer_product_e31_e12() {
        let e12: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e12();
        let e31: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e31();
        let zero: EuclideanMultivector3<f64> = EuclideanMultivector3::zero();

        assert_eq!(e31 ^ e12, zero);
    }

    #[test]
    fn test_outer_product_e31_e23() {
        let e23: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e23();
        let e31: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e31();
        let zero: EuclideanMultivector3<f64> = EuclideanMultivector3::zero();

        assert_eq!(e31 ^ e23, zero);
    }

    #[test]
    fn test_outer_product_e31_e31() {
        let e31: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e31();
        let zero: EuclideanMultivector3<f64> = EuclideanMultivector3::zero();

        assert_eq!(e31 ^ e31, zero);
    }

    #[test]
    fn test_outer_product_e31_e123() {
        let e31: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e31();
        let e123: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e123();
        let zero: EuclideanMultivector3<f64> = EuclideanMultivector3::zero();

        assert_eq!(e31 ^ e123, zero);
    }

    #[test]
    fn test_outer_product_e123_e1() {
        let e1: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e1();
        let e123: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e123();
        let zero: EuclideanMultivector3<f64> = EuclideanMultivector3::zero();

        assert_eq!(e123 ^ e1, zero);
    }

    #[test]
    fn test_outer_product_e123_e2() {
        let e2: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e2();
        let e123: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e123();
        let zero: EuclideanMultivector3<f64> = EuclideanMultivector3::zero();

        assert_eq!(e123 ^ e2, zero);
    }

    #[test]
    fn test_outer_product_e123_e3() {
        let e3: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e3();
        let e123: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e123();
        let zero: EuclideanMultivector3<f64> = EuclideanMultivector3::zero();

        assert_eq!(e123 ^ e3, zero);
    }

    #[test]
    fn test_outer_product_e123_e12() {
        let e12: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e12();
        let e123: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e123();
        let zero: EuclideanMultivector3<f64> = EuclideanMultivector3::zero();

        assert_eq!(e123 ^ e12, zero);
    }

    #[test]
    fn test_outer_product_e123_e23() {
        let e23: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e23();
        let e123: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e123();
        let zero: EuclideanMultivector3<f64> = EuclideanMultivector3::zero();

        assert_eq!(e123 ^ e23, zero);
    }

    #[test]
    fn test_outer_product_e123_e31() {
        let e31: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e31();
        let e123: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e123();
        let zero: EuclideanMultivector3<f64> = EuclideanMultivector3::zero();

        assert_eq!(e123 ^ e31, zero);
    }

    #[test]
    fn test_outer_product_e123_e123() {
        let e123: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e123();
        let zero: EuclideanMultivector3<f64> = EuclideanMultivector3::zero();

        assert_eq!(e123 ^ e123, zero);
    }

    #[test]
    fn test_geometric_product_e1_e1() {
        let e1: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e1();
        let one: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_scalar();

        assert_eq!(e1 * e1, one);
    }

    #[test]
    fn test_geometric_product_e1_e2() {
        let e1: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e1();
        let e2: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e2();
        let e12: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e12();

        assert_eq!(e1 * e2, e12);
    }

    #[test]
    fn test_geometric_product_e1_e3() {
        let e1: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e1();
        let e3: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e3();
        let e31: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e31();

        assert_eq!(e1 * e3, -e31);
    }

    #[test]
    fn test_geometric_product_e1_e12() {
        let e1: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e1();
        let e2: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e2();
        let e12: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e12();

        assert_eq!(e1 * e12, e2);
    }

    #[test]
    fn test_geometric_product_e1_e23() {
        let e1: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e1();
        let e23: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e23();
        let e123: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e123();

        assert_eq!(e1 * e23, e123);
    }

    #[test]
    fn test_geometric_product_e1_e31() {
        let e1: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e1();
        let e3: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e3();
        let e31: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e31();

        assert_eq!(e1 * e31, -e3);
    }

    #[test]
    fn test_geometric_product_e1_e123() {
        let e1: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e1();
        let e23: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e23();
        let e123: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e123();

        assert_eq!(e1 * e123, e23);
    }

    #[test]
    fn test_geometric_product_e2_e1() {
        let e1: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e1();
        let e2: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e2();
        let e12: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e12();

        assert_eq!(e2 * e1, -e12);
    }

    #[test]
    fn test_geometric_product_e2_e2() {
        let e2: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e2();
        let one: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_scalar();

        assert_eq!(e2 * e2, one);
    }

    #[test]
    fn test_geometric_product_e2_e3() {
        let e2: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e2();
        let e3: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e3();
        let e23: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e23();

        assert_eq!(e2 * e3, e23);
    }

    #[test]
    fn test_geometric_product_e2_e12() {
        let e1: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e1();
        let e2: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e2();
        let e12: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e12();

        assert_eq!(e2 * e12, -e1);
    }

    #[test]
    fn test_geometric_product_e2_e23() {
        let e2: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e2();
        let e3: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e3();
        let e23: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e23();

        assert_eq!(e2 * e23, e3);
    }

    #[test]
    fn test_geometric_product_e2_e31() {
        let e2: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e2();
        let e31: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e31();
        let e123: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e123();

        assert_eq!(e2 * e31, e123);
    }

    #[test]
    fn test_geometric_product_e2_e123() {
        let e2: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e2();
        let e31: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e31();
        let e123: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e123();

        assert_eq!(e2 * e123, e31);
    }

    #[test]
    fn test_geometric_product_e3_e1() {
        let e1: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e1();
        let e3: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e3();
        let e31: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e31();

        assert_eq!(e3 * e1, e31);
    }

    #[test]
    fn test_geometric_product_e3_e2() {
        let e2: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e2();
        let e3: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e3();
        let e23: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e23();

        assert_eq!(e3 * e2, -e23);
    }

    #[test]
    fn test_geometric_product_e3_e3() {
        let e3: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e3();
        let one: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_scalar();

        assert_eq!(e3 * e3, one);
    }

    #[test]
    fn test_geometric_product_e3_e12() {
        let e3: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e3();
        let e12: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e12();
        let e123: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e123();

        assert_eq!(e3 * e12, e123);
    }

    #[test]
    fn test_geometric_product_e3_e23() {
        let e2: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e2();
        let e3: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e3();
        let e23: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e23();

        assert_eq!(e3 * e23, -e2);
    }

    #[test]
    fn test_geometric_product_e3_e31() {
        let e1: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e1();
        let e3: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e3();
        let e31: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e31();

        assert_eq!(e3 * e31, e1);
    }

    #[test]
    fn test_geometric_product_e3_e123() {
        let e3: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e3();
        let e12: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e12();
        let e123: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e123();

        assert_eq!(e3 * e123, e12);
    }

    #[test]
    fn test_geometric_product_e12_e1() {
        let e1: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e1();
        let e2: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e2();
        let e12: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e12();

        assert_eq!(e12 * e1, -e2);
    }

    #[test]
    fn test_geometric_product_e12_e2() {
        let e1: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e1();
        let e2: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e2();
        let e12: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e12();

        assert_eq!(e12 * e2, e1);
    }

    #[test]
    fn test_geometric_product_e12_e3() {
        let e3: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e3();
        let e12: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e12();
        let e123: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e123();

        assert_eq!(e12 * e3, e123);
    }

    #[test]
    fn test_geometric_product_e12_e12() {
        let e12: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e12();
        let one: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_scalar();

        assert_eq!(e12 * e12, -one);
    }

    #[test]
    fn test_geometric_product_e12_e23() {
        let e12: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e12();
        let e23: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e23();
        let e31: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e31();

        assert_eq!(e12 * e23, -e31);
    }

    #[test]
    fn test_geometric_product_e12_e31() {
        let e12: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e12();
        let e23: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e23();
        let e31: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e31();

        assert_eq!(e12 * e31, e23);
    }

    #[test]
    fn test_geometric_product_e12_e123() {
        let e3: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e3();
        let e12: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e12();
        let e123: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e123();

        assert_eq!(e12 * e123, -e3);
    }

    #[test]
    fn test_geometric_product_e23_e1() {
        let e1: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e1();
        let e23: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e23();
        let e123: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e123();

        assert_eq!(e23 * e1, e123);
    }

    #[test]
    fn test_geometric_product_e23_e2() {
        let e2: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e2();
        let e3: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e3();
        let e23: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e23();

        assert_eq!(e23 * e2, -e3);
    }

    #[test]
    fn test_geometric_product_e23_e3() {
        let e2: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e2();
        let e3: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e3();
        let e23: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e23();

        assert_eq!(e23 * e3, e2);
    }

    #[test]
    fn test_geometric_product_e23_e12() {
        let e12: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e12();
        let e23: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e23();
        let e31: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e31();

        assert_eq!(e23 * e12, e31);
    }

    #[test]
    fn test_geometric_product_e23_e23() {
        let e23: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e23();
        let one: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_scalar();

        assert_eq!(e23 * e23, -one);
    }

    #[test]
    fn test_geometric_product_e23_e31() {
        let e12: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e12();
        let e23: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e23();
        let e31: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e31();

        assert_eq!(e23 * e31, -e12);
    }

    #[test]
    fn test_geometric_product_e23_e123() {
        let e1: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e1();
        let e23: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e23();
        let e123: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e123();

        assert_eq!(e23 * e123, -e1);
    }

    #[test]
    fn test_geometric_product_e31_e1() {
        let e1: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e1();
        let e3: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e3();
        let e31: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e31();

        assert_eq!(e31 * e1, e3);
    }

    #[test]
    fn test_geometric_product_e31_e2() {
        let e2: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e2();
        let e31: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e31();
        let e123: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e123();

        assert_eq!(e31 * e2, e123);
    }

    #[test]
    fn test_geometric_product_e31_e3() {
        let e1: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e1();
        let e3: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e3();
        let e31: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e31();

        assert_eq!(e31 * e3, -e1);
    }

    #[test]
    fn test_geometric_product_e31_e12() {
        let e12: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e12();
        let e23: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e23();
        let e31: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e31();

        assert_eq!(e31 * e12, -e23);
    }

    #[test]
    fn test_geometric_product_e31_e23() {
        let e12: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e12();
        let e23: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e23();
        let e31: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e31();

        assert_eq!(e31 * e23, e12);
    }

    #[test]
    fn test_geometric_product_e31_e31() {
        let e31: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e31();
        let one: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_scalar();

        assert_eq!(e31 * e31, -one);
    }

    #[test]
    fn test_geometric_product_e31_e123() {
        let e2: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e2();
        let e31: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e31();
        let e123: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e123();
        
        assert_eq!(e31 * e123, -e2);
    }

    #[test]
    fn test_geometric_product_e123_e1() {
        let e1: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e1();
        let e23: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e23();
        let e123: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e123();

        assert_eq!(e123 * e1, e23);
    }

    #[test]
    fn test_geometric_product_e123_e2() {
        let e2: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e2();
        let e31: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e31();
        let e123: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e123();

        assert_eq!(e123 * e2, e31);
    }

    #[test]
    fn test_geometric_product_e123_e3() {
        let e3: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e3();
        let e12: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e12();
        let e123: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e123();

        assert_eq!(e123 * e3, e12);
    }

    #[test]
    fn test_geometric_product_e123_e12() {
        let e3: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e3();
        let e12: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e12();
        let e123: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e123();

        assert_eq!(e123 * e12, -e3);
    }

    #[test]
    fn test_geometric_product_e123_e23() {
        let e1: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e1();
        let e23: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e23();
        let e123: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e123();

        assert_eq!(e123 * e23, -e1);
    }

    #[test]
    fn test_geometric_product_e123_e31() {
        let e2: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e2();
        let e31: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e31();
        let e123: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e123();

        assert_eq!(e123 * e31, -e2);
    }

    #[test]
    fn test_geometric_product_e123_e123() {
        let e123: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e123();
        let one: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_scalar();

        assert_eq!(e123 * e123, -one);
    }

    #[test]
    fn test_geometric_product_multivectors() {
        let mv1 = EuclideanMultivector3::new(
            1_f64, 2_f64, 3_f64, 4_f64, 5_f64, 6_f64, 7_f64, 8_f64
        );
        let mv2 = EuclideanMultivector3::new(
            9_f64, 10_f64, 11_f64, 12_f64, 13_f64, 14_f64, 15_f64, 16_f64
        );
        let expected = EuclideanMultivector3::new(
            -272_f64, -188_f64, -202_f64, -120_f64, 218_f64, 156_f64, 238_f64, 410_f64
        );
        let result = mv1 * mv2;

        assert_eq!(result, expected);
    }

    #[test]
    fn test_geometric_product_zero_multivector() {
        let mv = EuclideanMultivector3::new(
            34_f64, 955_f64, 123_f64, 68_f64, -15_f64, -24_f64, 235_f64, 3_f64
        );
        let zero: EuclideanMultivector3<f64> = EuclideanMultivector3::zero();

        assert_eq!(zero * mv, zero);
    }

    #[test]
    fn test_geometric_product_multivector_zero() {
        let mv = EuclideanMultivector3::new(
            34_f64, 955_f64, 123_f64, 68_f64, -15_f64, -24_f64, 235_f64, 3_f64
        );
        let zero: EuclideanMultivector3<f64> = EuclideanMultivector3::zero();

        assert_eq!(mv * zero, zero);
    }

    #[test]
    fn test_geometric_product_one_multivector() {
        let mv = EuclideanMultivector3::new(
            34_f64, 955_f64, 123_f64, 68_f64, -15_f64, -24_f64, 235_f64, 3_f64
        );
        let one: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_scalar();

        assert_eq!(one * mv, mv);
    }

    #[test]
    fn test_geometric_product_multivector_one() {
        let mv = EuclideanMultivector3::new(
            34_f64, 955_f64, 123_f64, 68_f64, -15_f64, -24_f64, 235_f64, 3_f64
        );
        let one: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_scalar();

        assert_eq!(mv * one, mv);
    }

    #[test]
    fn test_multivector_grade0() {
        let mv: EuclideanMultivector3<isize> = EuclideanMultivector3::new(
            1, 1, 1, 1, 1, 1, 1, 1
        );
        let expected = EuclideanMultivector3::new(
            1, 0, 0, 0, 0, 0, 0, 0
        );
        let result = mv.grade(0);
    
        assert_eq!(result, expected);
    }

    #[test]
    fn test_multivector_grade1() {
        let mv: EuclideanMultivector3<isize> = EuclideanMultivector3::new(
            1, 1, 1, 1, 1, 1, 1, 1
        );
        let expected = EuclideanMultivector3::new(
            0, 1, 1, 1, 0, 0, 0, 0
        );
        let result = mv.grade(1);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_multivector_grade2() {
        let mv: EuclideanMultivector3<isize> = EuclideanMultivector3::new(
            1, 1, 1, 1, 1, 1, 1, 1
        );
        let expected = EuclideanMultivector3::new(
            0, 0, 0, 0, 1, 1, 1, 0);
        let result = mv.grade(2);
    
        assert_eq!(result, expected);
    }

    #[test]
    fn test_multivector_grade3() {
        let mv: EuclideanMultivector3<isize> = EuclideanMultivector3::new(
            1, 1, 1, 1, 1, 1, 1, 1
        );
        let expected: EuclideanMultivector3<isize> = EuclideanMultivector3::new(
            0, 0, 0, 0, 0, 0, 0, 1
        );

        assert_eq!(mv.grade(3), expected);
    }

    #[test]
    fn test_multivector_grade4() {
        let mv: EuclideanMultivector3<isize> = EuclideanMultivector3::new(
            1, 1, 1, 1, 1, 1, 1, 1
        );
        let zero: EuclideanMultivector3<isize> = EuclideanMultivector3::zero();

        assert_eq!(mv.grade(4), zero);
    }

    #[test]
    fn test_multivector_grade_large() {
        let mv: EuclideanMultivector3<isize> = EuclideanMultivector3::new(
            1, 1, 1, 1, 1, 1, 1, 1
        );
        let zero: EuclideanMultivector3<isize> = EuclideanMultivector3::zero();

        assert_eq!(mv.grade(usize::MAX), zero);
    }

    #[test]
    fn test_multivector_reverse1() {
        let mv = EuclideanMultivector3::new(
            1_f64, 2_f64, 3_f64, 4_f64, 5_f64, 6_f64, 7_f64, 8_f64
        );
        let expected = EuclideanMultivector3::new(
            1_f64, 2_f64, 3_f64, 4_f64, -5_f64, -6_f64, -7_f64, -8_f64
        );
        let result = mv.reverse();

        assert_eq!(result, expected);
    }

    #[test]
    fn test_multivector_reverse2() {
        let one: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_scalar();
        let e1: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e1();
        let e2: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e2();
        let e3: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e3();
        let e12: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e12();
        let e23: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e23();
        let e31: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e31();
        let e123: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e123();

        assert_eq!(one.reverse(), one);
        assert_eq!(e1.reverse(), e1);
        assert_eq!(e2.reverse(), e2);
        assert_eq!(e3.reverse(), e3);
        assert_eq!(e12.reverse(), -e12);
        assert_eq!(e23.reverse(), -e23);
        assert_eq!(e31.reverse(), -e31);
        assert_eq!(e123.reverse(), -e123);
    }

    #[test]
    fn test_multivector_inverse() {
        let mv = EuclideanMultivector3::new(
            1_f64, 2_f64, 3_f64, 4_f64, 5_f64, 6_f64, 7_f64, 8_f64
        );
        let expected = EuclideanMultivector3::new(
            -0.083333333333333,
            0.05982905982906, 0.068376068376068, 0.044871794871795,
            -0.053418803418803, -0.034188034188034, -0.047008547008547,
            0.027777777777778
        );
        let result = mv.inverse().unwrap();

        assert_relative_eq!(result, expected, epsilon = 1e-10);
    }

    #[test]
    fn test_multivector_times_inverse1() {
        let mv = EuclideanMultivector3::new(
            3_f64, 35_f64, 13_f64, 94_f64, 2_f64, 2089_f64, 120_f64, 3_f64
        );
        let mv_inv = mv.inverse().unwrap();
        let expected: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_scalar();
        let result = mv * mv_inv;

        assert_relative_eq!(result, expected, epsilon = 1e-10);
    }

    #[test]
    fn test_multivector_times_inverse2() {
        let mv = EuclideanMultivector3::new(
            3_f64, 35_f64, 13_f64, 94_f64, 2_f64, 2089_f64, 120_f64, 3_f64
        );
        let mv_inv = mv.inverse().unwrap();
        let expected: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_scalar();
        let result = mv_inv * mv;

        assert_relative_eq!(result, expected, epsilon = 1e-10);
    }

    #[test]
    fn test_volume_element_inverse() {
        let e123: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e123();
        let expected = -e123;
        let result = e123.inverse().unwrap();
        
        assert_eq!(result, expected);
    }

    #[test]
    fn test_vector_inverse1() {
        let e1: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e1();
        let mv = e1 * 2_f64;
        let expected = e1 * (1_f64 / 2_f64);
        let result = mv.inverse().unwrap();
        
        assert_eq!(result, expected);
    }

    #[test]
    fn test_vector_inverse2() {
        let e2: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e2();
        let mv = e2 * 2_f64;
        let expected = e2 * (1_f64 / 2_f64);
        let result = mv.inverse().unwrap();
        
        assert_eq!(result, expected);
    }

    #[test]
    fn test_vector_inverse3() {
        let e3: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e3();
        let mv = e3 * 2_f64;
        let expected = e3 * (1_f64 / 2_f64);
        let result = mv.inverse().unwrap();
        
        assert_eq!(result, expected);
    }

    #[test]
    fn test_vector_inverse4() {
        let e1: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e1();
        let e2: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e2();
        let e3: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e3();
        let mv = e1 * 3_f64 + e2 * 5_f64 + e3 * 7_f64;
        let magnitude_squared = 83_f64;
        let expected = 
            e1 * (3_f64 / magnitude_squared) + e2 * (5_f64 / magnitude_squared) + e3 * (7_f64 / magnitude_squared);
        let result = mv.inverse().unwrap();

        assert_relative_eq!(result, expected, epsilon = 1e-10);
    }

    #[test]
    fn test_scalar_inverse() {
        let unit_scalar: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_scalar();
        let scalar = unit_scalar * 2_f64;
        let expected = unit_scalar *  (1_f64 / 2_f64);
        let result = scalar.inverse().unwrap();

        assert_eq!(result, expected);
    }

    #[test]
    fn test_zero_multivector_is_not_invertible() {
        let zero: EuclideanMultivector3<f64> = EuclideanMultivector3::zero();

        assert!(!zero.is_invertible());
    }

    #[test]
    fn test_scalar_product_e1_e1() {
        let e1: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e1();
        let one: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_scalar();

        assert_eq!(e1 | e1, one);
    }

    #[test]
    fn test_scalar_product_e1_e2() {
        let e1: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e1();
        let e2: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e2();
        let zero: EuclideanMultivector3<f64> = EuclideanMultivector3::zero();

        assert_eq!(e1 | e2, zero);
    }

    #[test]
    fn test_scalar_product_e1_e3() {
        let e1: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e1();
        let e3: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e3();
        let zero: EuclideanMultivector3<f64> = EuclideanMultivector3::zero();

        assert_eq!(e1 | e3, zero);
    }

    #[test]
    fn test_scalar_product_e1_e12() {
        let e1: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e1();
        let e12: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e12();
        let zero: EuclideanMultivector3<f64> = EuclideanMultivector3::zero();

        assert_eq!(e1 | e12, zero);
    }

    #[test]
    fn test_scalar_product_e1_e23() {
        let e1: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e1();
        let e23: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e23();
        let zero: EuclideanMultivector3<f64> = EuclideanMultivector3::zero();

        assert_eq!(e1 | e23, zero);
    }

    #[test]
    fn test_scalar_product_e1_e31() {
        let e1: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e1();
        let e31: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e31();
        let zero: EuclideanMultivector3<f64> = EuclideanMultivector3::zero();

        assert_eq!(e1 | e31, zero);
    }

    #[test]
    fn test_scalar_product_e1_e123() {
        let e1: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e1();
        let e123: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e123();
        let zero: EuclideanMultivector3<f64> = EuclideanMultivector3::zero();

        assert_eq!(e1 | e123, zero);
    }

    #[test]
    fn test_scalar_product_e2_e1() {
        let e1: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e1();
        let e2: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e2();
        let zero: EuclideanMultivector3<f64> = EuclideanMultivector3::zero();

        assert_eq!(e2 | e1, zero);
    }

    #[test]
    fn test_scalar_product_e2_e2() {
        let e2: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e2();
        let one: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_scalar();

        assert_eq!(e2 | e2, one);
    }

    #[test]
    fn test_scalar_product_e2_e3() {
        let e2: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e2();
        let e3: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e3();
        let zero: EuclideanMultivector3<f64> = EuclideanMultivector3::zero();

        assert_eq!(e2 | e3, zero);
    }

    #[test]
    fn test_scalar_product_e2_e12() {
        let e2: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e2();
        let e12: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e12();
        let zero: EuclideanMultivector3<f64> = EuclideanMultivector3::zero();

        assert_eq!(e2 | e12, zero);
    }

    #[test]
    fn test_scalar_product_e2_e23() {
        let e2: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e2();
        let e23: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e23();
        let zero: EuclideanMultivector3<f64> = EuclideanMultivector3::zero();

        assert_eq!(e2 | e23, zero);
    }

    #[test]
    fn test_scalar_product_e2_e31() {
        let e2: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e2();
        let e31: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e31();
        let zero: EuclideanMultivector3<f64> = EuclideanMultivector3::zero();

        assert_eq!(e2 | e31, zero);
    }

    #[test]
    fn test_scalar_product_e2_e123() {
        let e2: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e2();
        let e123: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e123();
        let zero: EuclideanMultivector3<f64> = EuclideanMultivector3::zero();

        assert_eq!(e2 | e123, zero);
    }

    #[test]
    fn test_scalar_product_e3_e1() {
        let e1: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e1();
        let e3: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e3();
        let zero: EuclideanMultivector3<f64> = EuclideanMultivector3::zero();

        assert_eq!(e3 | e1, zero);
    }

    #[test]
    fn test_scalar_product_e3_e2() {
        let e2: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e2();
        let e3: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e3();
        let zero: EuclideanMultivector3<f64> = EuclideanMultivector3::zero();

        assert_eq!(e3 | e2, zero);
    }

    #[test]
    fn test_scalar_product_e3_e3() {
        let e3: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e3();
        let one: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_scalar();

        assert_eq!(e3 | e3, one);
    }

    #[test]
    fn test_scalar_product_e3_e12() {
        let e3: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e3();
        let e12: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e12();
        let zero: EuclideanMultivector3<f64> = EuclideanMultivector3::zero();

        assert_eq!(e3 | e12, zero);
    }

    #[test]
    fn test_scalar_product_e3_e23() {
        let e3: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e3();
        let e23: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e23();
        let zero: EuclideanMultivector3<f64> = EuclideanMultivector3::zero();

        assert_eq!(e3 | e23, zero);
    }

    #[test]
    fn test_scalar_product_e3_e31() {
        let e3: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e3();
        let e31: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e31();
        let zero: EuclideanMultivector3<f64> = EuclideanMultivector3::zero();

        assert_eq!(e3 | e31, zero);
    }

    #[test]
    fn test_scalar_product_e3_e123() {
        let e3: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e3();
        let e123: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e123();
        let zero: EuclideanMultivector3<f64> = EuclideanMultivector3::zero();

        assert_eq!(e3 | e123, zero);
    }

    #[test]
    fn test_scalar_product_e12_e1() {
        let e1: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e1();
        let e12: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e12();
        let zero: EuclideanMultivector3<f64> = EuclideanMultivector3::zero();

        assert_eq!(e12 | e1, zero);
    }

    #[test]
    fn test_scalar_product_e12_e2() {
        let e2: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e2();
        let e12: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e12();
        let zero: EuclideanMultivector3<f64> = EuclideanMultivector3::zero();

        assert_eq!(e12 | e2, zero);
    }

    #[test]
    fn test_scalar_product_e12_e3() {
        let e3: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e3();
        let e12: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e12();
        let zero: EuclideanMultivector3<f64> = EuclideanMultivector3::zero();

        assert_eq!(e12 | e3, zero);
    }

    #[test]
    fn test_scalar_product_e12_e12() {
        let e12: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e12();
        let one: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_scalar();

        assert_eq!(e12 | e12, one);
    }

    #[test]
    fn test_scalar_product_e12_e23() {
        let e12: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e12();
        let e23: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e23();
        let zero: EuclideanMultivector3<f64> = EuclideanMultivector3::zero();

        assert_eq!(e12 | e23, zero);
    }

    #[test]
    fn test_scalar_product_e12_e31() {
        let e12: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e12();
        let e31: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e31();
        let zero: EuclideanMultivector3<f64> = EuclideanMultivector3::zero();

        assert_eq!(e12 | e31, zero);
    }

    #[test]
    fn test_scalar_product_e12_e123() {
        let e12: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e12();
        let e123: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e123();
        let zero: EuclideanMultivector3<f64> = EuclideanMultivector3::zero();

        assert_eq!(e12 | e123, zero);
    }

    #[test]
    fn test_scalar_product_e23_e1() {
        let e1: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e1();
        let e23: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e23();
        let zero: EuclideanMultivector3<f64> = EuclideanMultivector3::zero();

        assert_eq!(e23 | e1, zero);
    }

    #[test]
    fn test_scalar_product_e23_e2() {
        let e2: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e2();
        let e23: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e23();
        let zero: EuclideanMultivector3<f64> = EuclideanMultivector3::zero();

        assert_eq!(e23 | e2, zero);
    }

    #[test]
    fn test_scalar_product_e23_e3() {
        let e3: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e3();
        let e23: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e23();
        let zero: EuclideanMultivector3<f64> = EuclideanMultivector3::zero();

        assert_eq!(e23 | e3, zero);
    }

    #[test]
    fn test_scalar_product_e23_e12() {
        let e12: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e12();
        let e23: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e23();
        let zero: EuclideanMultivector3<f64> = EuclideanMultivector3::zero();

        assert_eq!(e23 | e12, zero);
    }

    #[test]
    fn test_scalar_product_e23_e23() {
        let e23: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e23();
        let one: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_scalar();

        assert_eq!(e23 | e23, one);
    }

    #[test]
    fn test_scalar_product_e23_e31() {
        let e23: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e23();
        let e31: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e31();
        let zero: EuclideanMultivector3<f64> = EuclideanMultivector3::zero();

        assert_eq!(e23 | e31, zero);
    }

    #[test]
    fn test_scalar_product_e23_e123() {
        let e23: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e23();
        let e123: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e123();
        let zero: EuclideanMultivector3<f64> = EuclideanMultivector3::zero();

        assert_eq!(e23 | e123, zero);
    }

    #[test]
    fn test_scalar_product_e31_e1() {
        let e1: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e1();
        let e31: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e31();
        let zero: EuclideanMultivector3<f64> = EuclideanMultivector3::zero();

        assert_eq!(e31 | e1, zero);
    }

    #[test]
    fn test_scalar_product_e31_e2() {
        let e2: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e2();
        let e31: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e31();
        let zero: EuclideanMultivector3<f64> = EuclideanMultivector3::zero();

        assert_eq!(e31 | e2, zero);
    }

    #[test]
    fn test_scalar_product_e31_e3() {
        let e3: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e3();
        let e31: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e31();
        let zero: EuclideanMultivector3<f64> = EuclideanMultivector3::zero();

        assert_eq!(e31 | e3, zero);
    }

    #[test]
    fn test_scalar_product_e31_e12() {
        let e12: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e12();
        let e31: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e31();
        let zero: EuclideanMultivector3<f64> = EuclideanMultivector3::zero();

        assert_eq!(e31 | e12, zero);
    }

    #[test]
    fn test_scalar_product_e31_e23() {
        let e23: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e23();
        let e31: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e31();
        let zero: EuclideanMultivector3<f64> = EuclideanMultivector3::zero();

        assert_eq!(e31 | e23, zero);
    }

    #[test]
    fn test_scalar_product_e31_e31() {
        let e31: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e31();
        let one: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_scalar();

        assert_eq!(e31 | e31, one);
    }

    #[test]
    fn test_scalar_product_e31_e123() {
        let e31: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e31();
        let e123: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e123();
        let zero: EuclideanMultivector3<f64> = EuclideanMultivector3::zero();
    
        assert_eq!(e31 | e123, zero);
    }

    #[test]
    fn test_scalar_product_e123_e1() {
        let e1: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e1();
        let e123: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e123();
        let zero: EuclideanMultivector3<f64> = EuclideanMultivector3::zero();

        assert_eq!(e123 | e1, zero);
    }

    #[test]
    fn test_scalar_product_e123_e2() {
        let e2: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e2();
        let e123: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e123();
        let zero: EuclideanMultivector3<f64> = EuclideanMultivector3::zero();

        assert_eq!(e123 | e2, zero);
    }

    #[test]
    fn test_scalar_product_e123_e3() {
        let e3: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e3();
        let e123: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e123();
        let zero: EuclideanMultivector3<f64> = EuclideanMultivector3::zero();

        assert_eq!(e123 | e3, zero);
    }

    #[test]
    fn test_scalar_product_e123_e12() {
        let e12: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e12();
        let e123: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e123();
        let zero: EuclideanMultivector3<f64> = EuclideanMultivector3::zero();

        assert_eq!(e123 | e12, zero);
    }

    #[test]
    fn test_scalar_product_e123_e23() {
        let e23: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e23();
        let e123: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e123();
        let zero: EuclideanMultivector3<f64> = EuclideanMultivector3::zero();

        assert_eq!(e123 | e23, zero);
    }

    #[test]
    fn test_scalar_product_e123_e31() {
        let e31: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e31();
        let e123: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e123();
        let zero: EuclideanMultivector3<f64> = EuclideanMultivector3::zero();

        assert_eq!(e123 | e31, zero);
    }

    #[test]
    fn test_scalar_product_e123_e123() {
        let e123: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e123();
        let one: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_scalar();

        assert_eq!(e123 | e123, one);
    }

    #[test]
    fn test_scalar_product() {
        let mv1 = EuclideanMultivector3::new(
            1_f64, 2_f64, 3_f64, 4_f64, 5_f64, 6_f64, 7_f64, 8_f64
        );
        let mv2 = EuclideanMultivector3::new(
            9_f64, 10_f64, 11_f64, 12_f64, 13_f64, 14_f64, 15_f64, 16_f64
        );
        let expected = EuclideanMultivector3::from_scalar(492_f64);
        let result = mv1 | mv2;

        assert_eq!(result, expected);
    }

    #[test]
    fn test_scalar_product_zero_multivector() {
        let mv = EuclideanMultivector3::new(
            1_f64, 2_f64, 3_f64, 4_f64, 5_f64, 6_f64, 7_f64, 8_f64
        );
        let zero: EuclideanMultivector3<f64> = EuclideanMultivector3::zero();

        assert_eq!(zero | mv, zero);
    }

    #[test]
    fn test_scalar_product_multivector_zero() {
        let mv = EuclideanMultivector3::new(
            1_f64, 2_f64, 3_f64, 4_f64, 5_f64, 6_f64, 7_f64, 8_f64
        );
        let zero: EuclideanMultivector3<f64> = EuclideanMultivector3::zero();

        assert_eq!(mv | zero, zero);
    }

    #[test]
    fn test_scalar_product_one_multivector() {
        let mv = EuclideanMultivector3::new(
            1_f64, 2_f64, 3_f64, 4_f64, 5_f64, 6_f64, 7_f64, 8_f64
        );
        let one: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_scalar();

        assert_eq!(one | mv, one);
    }

    #[test]
    fn test_scalar_product_multivector_one() {
        let mv = EuclideanMultivector3::new(
            1_f64, 2_f64, 3_f64, 4_f64, 5_f64, 6_f64, 7_f64, 8_f64
        );
        let one: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_scalar();

        assert_eq!(mv | one, one);
    }

    #[test]
    fn test_left_contraction_scalar_e1() {
        let scalar_part = 3_f64;
        let scalar = EuclideanMultivector3::from_scalar(scalar_part);
        let e1: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e1();
        let expected = EuclideanMultivector3::new(
            0_f64, scalar_part, 0_f64, 0_f64, 0_f64, 0_f64, 0_f64, 0_f64
        );
        let result = scalar << e1;

        assert_eq!(result, expected);
    }

    #[test]
    fn test_left_contraction_scalar_e2() {
        let scalar_part = 3_f64;
        let scalar = EuclideanMultivector3::from_scalar(scalar_part);
        let e2: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e2();
        let expected = EuclideanMultivector3::new(
            0_f64, 0_f64, scalar_part, 0_f64, 0_f64, 0_f64, 0_f64, 0_f64
        );
        let result = scalar << e2;

        assert_eq!(result, expected);
    }

    #[test]
    fn test_left_contraction_scalar_e3() {
        let scalar_part = 3_f64;
        let scalar = EuclideanMultivector3::from_scalar(scalar_part);
        let e3: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e3();
        let expected = EuclideanMultivector3::new(
            0_f64, 0_f64, 0_f64, scalar_part, 0_f64, 0_f64, 0_f64, 0_f64
        );
        let result = scalar << e3;

        assert_eq!(result, expected);
    }

    #[test]
    fn test_left_contraction_scalar_e12() {
        let scalar_part = 3_f64;
        let scalar = EuclideanMultivector3::from_scalar(scalar_part);
        let e12: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e12();
        let expected = EuclideanMultivector3::new(
            0_f64, 0_f64, 0_f64, 0_f64, scalar_part, 0_f64, 0_f64, 0_f64
        );
        let result = scalar << e12;

        assert_eq!(result, expected);
    }

    #[test]
    fn test_left_contraction_scalar_e23() {
        let scalar_part = 3_f64;
        let scalar = EuclideanMultivector3::from_scalar(scalar_part);
        let e23: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e23();
        let expected = EuclideanMultivector3::new(
            0_f64, 0_f64, 0_f64, 0_f64, 0_f64, scalar_part, 0_f64, 0_f64
        );
        let result = scalar << e23;

        assert_eq!(result, expected);
    }

    #[test]
    fn test_left_contraction_scalar_e31() {
        let scalar_part = 3_f64;
        let scalar = EuclideanMultivector3::from_scalar(scalar_part);
        let e31: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e31();
        let expected = EuclideanMultivector3::new(
            0_f64, 0_f64, 0_f64, 0_f64, 0_f64, 0_f64, scalar_part, 0_f64
        );
        let result = scalar << e31;

        assert_eq!(result, expected);
    }

    #[test]
    fn test_left_contraction_scalar_e123() {
        let scalar_part = 3_f64;
        let scalar = EuclideanMultivector3::from_scalar(scalar_part);
        let e123: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e123();
        let expected = EuclideanMultivector3::new(
            0_f64, 0_f64, 0_f64, 0_f64, 0_f64, 0_f64, 0_f64, scalar_part
        );
        let result = scalar << e123;

        assert_eq!(result, expected);
    }

    #[test]
    fn test_left_contraction_e1_scalar() {
        let scalar_part = 3_f64;
        let scalar = EuclideanMultivector3::from_scalar(scalar_part);
        let e1: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e1();
        let expected = EuclideanMultivector3::zero();
        let result = e1 << scalar;

        assert_eq!(result, expected);
    }

    #[test]
    fn test_left_contraction_e2_scalar() {
        let scalar_part = 3_f64;
        let scalar = EuclideanMultivector3::from_scalar(scalar_part);
        let e2: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e2();
        let expected = EuclideanMultivector3::zero();
        let result = e2 << scalar;

        assert_eq!(result, expected);
    }

    #[test]
    fn test_left_contraction_e3_scalar() {
        let scalar_part = 3_f64;
        let scalar = EuclideanMultivector3::from_scalar(scalar_part);
        let e3: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e3();
        let expected = EuclideanMultivector3::zero();
        let result = e3 << scalar;

        assert_eq!(result, expected);
    }

    #[test]
    fn test_left_contraction_e12_scalar() {
        let scalar_part = 3_f64;
        let scalar = EuclideanMultivector3::from_scalar(scalar_part);
        let e12: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e12();
        let expected = EuclideanMultivector3::zero();
        let result = e12 << scalar;

        assert_eq!(result, expected);
    }

    #[test]
    fn test_left_contraction_e23_scalar() {
        let scalar_part = 3_f64;
        let scalar = EuclideanMultivector3::from_scalar(scalar_part);
        let e23: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e23();
        let expected = EuclideanMultivector3::zero();
        let result = e23 << scalar;

        assert_eq!(result, expected);
    }

    #[test]
    fn test_left_contraction_e31_scalar() {
        let scalar_part = 3_f64;
        let scalar = EuclideanMultivector3::from_scalar(scalar_part);
        let e31: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e31();
        let expected = EuclideanMultivector3::zero();
        let result = e31 << scalar;

        assert_eq!(result, expected);
    }

    #[test]
    fn test_left_contraction_e123_scalar() {
        let scalar_part = 3_f64;
        let scalar = EuclideanMultivector3::from_scalar(scalar_part);
        let e123: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e123();
        let expected = EuclideanMultivector3::zero();
        let result = e123 << scalar;

        assert_eq!(result, expected);
    }

    #[test]
    fn test_left_contraction_e1_e1() {
        let e1: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e1();
        let one: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_scalar();

        assert_eq!(e1 << e1, one);
    }

    #[test]
    fn test_left_contraction_e1_e2() {
        let e1: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e1();
        let e2: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e2();
        let zero: EuclideanMultivector3<f64> = EuclideanMultivector3::zero();

        assert_eq!(e1 << e2, zero);
    }

    #[test]
    fn test_left_contraction_e1_e3() {
        let e1: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e1();
        let e3: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e3();
        let zero: EuclideanMultivector3<f64> = EuclideanMultivector3::zero();

        assert_eq!(e1 << e3, zero);
    }

    #[test]
    fn test_left_contraction_e1_e12() {
        let e1: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e1();
        let e2: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e2();
        let e12: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e12();

        assert_eq!(e1 << e12, e2);
    }

    #[test]
    fn test_left_contraction_e1_e23() {
        let e1: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e1();
        let e23: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e23();
        let zero: EuclideanMultivector3<f64> = EuclideanMultivector3::zero();

        assert_eq!(e1 << e23, zero);
    }

    #[test]
    fn test_left_contraction_e1_e31() {
        let e1: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e1();
        let e3: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e3();
        let e31: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e31();

        assert_eq!(e1 << e31, -e3);
    }

    #[test]
    fn test_left_contraction_e1_e123() {
        let e1: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e1();
        let e23: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e23();
        let e123: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e123();

        assert_eq!(e1 << e123, e23);
    }

    #[test]
    fn test_left_contraction_e2_e1() {
        let e1: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e1();
        let e2: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e2();
        let zero: EuclideanMultivector3<f64> = EuclideanMultivector3::zero();

        assert_eq!(e2 << e1, zero);
    }

    #[test]
    fn test_left_contraction_e2_e2() {
        let e2: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e2();
        let one: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_scalar();

        assert_eq!(e2 << e2, one);
    }

    #[test]
    fn test_left_contraction_e2_e3() {
        let e2: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e2();
        let e3: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e3();
        let zero: EuclideanMultivector3<f64> = EuclideanMultivector3::zero();

        assert_eq!(e2 << e3, zero);
    }

    #[test]
    fn test_left_contraction_e2_e12() {
        let e1: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e1();
        let e2: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e2();
        let e12: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e12();

        assert_eq!(e2 << e12, -e1);
    }

    #[test]
    fn test_left_contraction_e2_e23() {
        let e2: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e2();
        let e3: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e3();
        let e23: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e23();

        assert_eq!(e2 << e23, e3);
    }

    #[test]
    fn test_left_contraction_e2_e31() {
        let e2: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e2();
        let e31: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e31();
        let zero: EuclideanMultivector3<f64> = EuclideanMultivector3::zero();

        assert_eq!(e2 << e31, zero);
    }

    #[test]
    fn test_left_contraction_e2_e123() {
        let e2: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e2();
        let e31: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e31();
        let e123: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e123();

        assert_eq!(e2 << e123, e31);
    }

    #[test]
    fn test_left_contraction_e3_e1() {
        let e1: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e1();
        let e3: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e3();
        let zero: EuclideanMultivector3<f64> = EuclideanMultivector3::zero();

        assert_eq!(e3 << e1, zero);
    }

    #[test]
    fn test_left_contraction_e3_e2() {
        let e2: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e2();
        let e3: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e3();
        let zero: EuclideanMultivector3<f64> = EuclideanMultivector3::zero();

        assert_eq!(e3 << e2, zero);
    }

    #[test]
    fn test_left_contraction_e3_e3() {
        let e3: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e3();
        let one: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_scalar();

        assert_eq!(e3 << e3, one);
    }

    #[test]
    fn test_left_contraction_e3_e12() {
        let e3: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e3();
        let e12: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e12();
        let zero: EuclideanMultivector3<f64> = EuclideanMultivector3::zero();

        assert_eq!(e3 << e12, zero);
    }

    #[test]
    fn test_left_contraction_e3_e23() {
        let e2: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e2();
        let e3: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e3();
        let e23: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e23();

        assert_eq!(e2 << e23, e3);
    }

    #[test]
    fn test_left_contraction_e3_e31() {
        let e2: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e2();
        let e31: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e31();
        let zero: EuclideanMultivector3<f64> = EuclideanMultivector3::zero();

        assert_eq!(e2 << e31, zero);
    }

    #[test]
    fn test_left_contraction_e3_e123() {
        let e2: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e2();
        let e31: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e31();
        let e123: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e123();

        assert_eq!(e2 << e123, e31);
    }

    #[test]
    fn test_left_contraction_e12_e1() {
        let e1: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e1();
        let e12: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e12();
        let zero: EuclideanMultivector3<f64> = EuclideanMultivector3::zero();

        assert_eq!(e12 << e1, zero);
    }

    #[test]
    fn test_left_contraction_e12_e2() {
        let e2: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e2();
        let e12: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e12();
        let zero: EuclideanMultivector3<f64> = EuclideanMultivector3::zero();

        assert_eq!(e12 << e2, zero);
    }

    #[test]
    fn test_left_contraction_e12_e3() {
        let e3: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e3();
        let e12: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e12();
        let zero: EuclideanMultivector3<f64> = EuclideanMultivector3::zero();

        assert_eq!(e12 << e3, zero);
    }

    #[test]
    fn test_left_contraction_e12_e12() {
        let e12: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e12();
        let one: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_scalar();

        assert_eq!(e12 << e12, -one);
    }

    #[test]
    fn test_left_contraction_e12_e23() {
        let e12: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e12();
        let e23: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e23();
        let zero: EuclideanMultivector3<f64> = EuclideanMultivector3::zero();

        assert_eq!(e12 << e23, zero);
    }

    #[test]
    fn test_left_contraction_e12_e31() {
        let e12: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e12();
        let e31: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e31();
        let zero: EuclideanMultivector3<f64> = EuclideanMultivector3::zero();

        assert_eq!(e12 << e31, zero);
    }

    #[test]
    fn test_left_contraction_e12_e123() {
        let e3: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e3();
        let e12: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e12();
        let e123: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e123();

        assert_eq!(e12 << e123, -e3);
    }

    #[test]
    fn test_left_contraction_e23_e1() {
        let e1: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e1();
        let e23: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e23();
        let zero: EuclideanMultivector3<f64> = EuclideanMultivector3::zero();

        assert_eq!(e23 << e1, zero);
    }

    #[test]
    fn test_left_contraction_e23_e2() {
        let e2: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e2();
        let e23: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e23();
        let zero: EuclideanMultivector3<f64> = EuclideanMultivector3::zero();

        assert_eq!(e23 << e2, zero);
    }

    #[test]
    fn test_left_contraction_e23_e3() {
        let e3: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e3();
        let e23: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e23();
        let zero: EuclideanMultivector3<f64> = EuclideanMultivector3::zero();

        assert_eq!(e23 << e3, zero);
    }

    #[test]
    fn test_left_contraction_e23_e12() {
        let e12: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e12();
        let e23: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e23();
        let zero: EuclideanMultivector3<f64> = EuclideanMultivector3::zero();

        assert_eq!(e23 << e12, zero);
    }

    #[test]
    fn test_left_contraction_e23_e23() {
        let e23: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e23();
        let one: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_scalar();

        assert_eq!(e23 << e23, -one);
    }

    #[test]
    fn test_left_contraction_e23_e31() {
        let e23: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e23();
        let e31: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e31();
        let zero: EuclideanMultivector3<f64> = EuclideanMultivector3::zero();

        assert_eq!(e23 << e31, zero);
    }

    #[test]
    fn test_left_contraction_e23_e123() {
        let e1: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e1();
        let e23: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e23();
        let e123: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e123();

        assert_eq!(e23 << e123, -e1);
    }

    #[test]
    fn test_left_contraction_e31_e1() {
        let e1: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e1();
        let e31: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e31();
        let zero: EuclideanMultivector3<f64> = EuclideanMultivector3::zero();

        assert_eq!(e31 << e1, zero);
    }

    #[test]
    fn test_left_contraction_e31_e2() {
        let e2: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e2();
        let e31: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e31();
        let zero: EuclideanMultivector3<f64> = EuclideanMultivector3::zero();

        assert_eq!(e31 << e2, zero);
    }

    #[test]
    fn test_left_contraction_e31_e3() {
        let e3: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e3();
        let e31: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e31();
        let zero: EuclideanMultivector3<f64> = EuclideanMultivector3::zero();

        assert_eq!(e31 << e3, zero);
    }

    #[test]
    fn test_left_contraction_e31_e12() {
        let e12: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e12();
        let e31: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e31();
        let zero: EuclideanMultivector3<f64> = EuclideanMultivector3::zero();

        assert_eq!(e31 << e12, zero);
    }

    #[test]
    fn test_left_contraction_e31_e23() {
        let e23: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e23();
        let e31: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e31();
        let zero: EuclideanMultivector3<f64> = EuclideanMultivector3::zero();

        assert_eq!(e31 << e23, zero);
    }

    #[test]
    fn test_left_contraction_e31_e31() {
        let e31: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e31();
        let one: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_scalar();

        assert_eq!(e31 << e31, -one);
    }

    #[test]
    fn test_left_contraction_e31_e123() {
        let e2: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e2();
        let e31: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e31();
        let e123: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e123();

        assert_eq!(e31 << e123, -e2);
    }

    #[test]
    fn test_left_contraction_e123_e1() {
        let e1: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e1();
        let e123: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e123();
        let zero: EuclideanMultivector3<f64> = EuclideanMultivector3::zero();

        assert_eq!(e123 << e1, zero);
    }

    #[test]
    fn test_left_contraction_e123_e2() {
        let e2: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e2();
        let e123: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e123();
        let zero: EuclideanMultivector3<f64> = EuclideanMultivector3::zero();

        assert_eq!(e123 << e2, zero);
    }

    #[test]
    fn test_left_contraction_e123_e3() {
        let e3: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e3();
        let e123: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e123();
        let zero: EuclideanMultivector3<f64> = EuclideanMultivector3::zero();

        assert_eq!(e123 << e3, zero);
    }

    #[test]
    fn test_left_contraction_e123_e12() {
        let e12: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e12();
        let e123: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e123();
        let zero: EuclideanMultivector3<f64> = EuclideanMultivector3::zero();

        assert_eq!(e123 << e12, zero);
    }

    #[test]
    fn test_left_contraction_e123_e23() {
        let e23: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e23();
        let e123: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e123();
        let zero: EuclideanMultivector3<f64> = EuclideanMultivector3::zero();

        assert_eq!(e123 << e23, zero);
    }

    #[test]
    fn test_left_contraction_e123_e31() {
        let e31: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e31();
        let e123: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e123();
        let zero: EuclideanMultivector3<f64> = EuclideanMultivector3::zero();

        assert_eq!(e123 << e31, zero);
    }

    #[test]
    fn test_left_contraction_e123_e123() {
        let e123: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e123();
        let one: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_scalar();

        assert_eq!(e123 << e123, -one);
    }

    #[test]
    fn test_left_contraction_scalar_multivector() {
        let scalar_part = 3_f64;
        let scalar = EuclideanMultivector3::from_scalar(scalar_part);
        let mv = EuclideanMultivector3::new(
            1_f64, 2_f64, 3_f64, 4_f64, 5_f64, 6_f64, 7_f64, 8_f64
        );
        let expected = EuclideanMultivector3::new(
            3_f64, 6_f64, 9_f64, 12_f64, 15_f64, 18_f64, 21_f64, 24_f64
        );
        let result = scalar << mv;

        assert_eq!(result, expected);
    }

    #[test]
    fn test_left_contraction_multivector_multivector() {
        let mv1 = EuclideanMultivector3::new(
            1_f64, 2_f64, 3_f64, 4_f64, 5_f64, 6_f64, 7_f64, 8_f64
        );
        let mv2 = EuclideanMultivector3::new(
            3_f64, 5_f64, 7_f64, 9_f64, 11_f64, 13_f64, 15_f64, 17_f64
        );
        let expected = EuclideanMultivector3::new(
            -304_f64, -70_f64, -142_f64, -67_f64, 79_f64, 47_f64, 66_f64, 17_f64
        );
        let result = mv1 << mv2;

        assert_eq!(result, expected);
    }

    #[test]
    fn test_left_contraction_multivector_with_self() {
        let mv = EuclideanMultivector3::new(
            1_f64, 2_f64, 3_f64, 4_f64, 5_f64, 6_f64, 7_f64, 8_f64
        );
        let expected = EuclideanMultivector3::new(
            -144_f64, -33_f64, -67_f64, -32_f64, 37_f64, 22_f64, 31_f64, 8_f64
        );
        let result = mv << mv;

        assert_eq!(result, expected);
    }

    #[test]
    fn test_left_contraction_zero_multivector() {
        let mv = EuclideanMultivector3::new(
            1_f64, 2_f64, 3_f64, 4_f64, 5_f64, 6_f64, 7_f64, 8_f64
        );
        let zero = EuclideanMultivector3::zero();
    
        assert_eq!(zero << mv, zero);
    }

    #[test]
    fn test_left_contraction_multivector_zero() {
        let mv = EuclideanMultivector3::new(
            1_f64, 2_f64, 3_f64, 4_f64, 5_f64, 6_f64, 7_f64, 8_f64
        );
        let zero = EuclideanMultivector3::zero();

        assert_eq!(mv << zero, zero);
    }

    #[test]
    fn test_left_contraction_one_multivector() {
        let mv = EuclideanMultivector3::new(
            1_f64, 2_f64, 3_f64, 4_f64, 5_f64, 6_f64, 7_f64, 8_f64
        );
        let one = EuclideanMultivector3::unit_scalar();
    
        assert_eq!(mv << one, one);
    }

    #[test]
    fn test_left_contraction_multivector_one() {
        let mv = EuclideanMultivector3::new(
            1_f64, 2_f64, 3_f64, 4_f64, 5_f64, 6_f64, 7_f64, 8_f64
        );
        let one = EuclideanMultivector3::unit_scalar();
    
        assert_eq!(one << mv, mv);
    }

    #[test]
    fn test_left_contraction_e12_e21() {
        let e1: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e1();
        let e2: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e2();
        let one: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_scalar();

        assert_eq!((e1 ^ e2) << (e2 ^ e1), one);
    }

    #[test]
    fn test_left_contraction_e21_e12() {
        let e1: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e1();
        let e2: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e2();
        let one: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_scalar();

        assert_eq!((e2 ^ e1) << (e1 ^ e2), one);
    }

    #[test]
    fn test_left_contraction_e23_e32() {
        let e2: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e2();
        let e3: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e3();
        let one: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_scalar();

        assert_eq!((e2 ^ e3) << (e3 ^ e2), one);
    }

    #[test]
    fn test_left_contraction_e32_e23() {
        let e2: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e2();
        let e3: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e3();
        let one: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_scalar();

        assert_eq!((e3 ^ e2) << (e2 ^ e3), one);
    }

    #[test]
    fn test_left_contraction_e31_e13() {
        let e1: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e1();
        let e3: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e3();
        let one: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_scalar();

        assert_eq!((e3 ^ e1) << (e1 ^ e3), one);
    }

    #[test]
    fn test_left_contraction_e13_e31() {
        let e1: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e1();
        let e3: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e3();
        let one: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_scalar();

        assert_eq!((e1 ^ e3) << (e3 ^ e1), one);
    }

    #[test]
    fn test_left_contraction_e123_e321() {
        let e1: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e1();
        let e2: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e2();
        let e3: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e3();
        let one: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_scalar();

        assert_eq!((e1 ^ e2 ^ e3) << (e3 ^ e2 ^ e1), one);
    }

    #[test]
    fn test_left_contraction_e321_e123() {
        let e1: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e1();
        let e2: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e2();
        let e3: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e3();
        let one: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_scalar();

        assert_eq!((e3 ^ e2 ^ e1) << (e1 ^ e2 ^ e3), one);
    }

    #[test]
    fn test_right_contraction_scalar_e1() {
        let scalar_part = 3_f64;
        let scalar = EuclideanMultivector3::from_scalar(scalar_part);
        let e1: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e1();
        let expected = EuclideanMultivector3::zero();
        let result = scalar >> e1;

        assert_eq!(result, expected);
    }

    #[test]
    fn test_right_contraction_scalar_e2() {
        let scalar_part = 3_f64;
        let scalar = EuclideanMultivector3::from_scalar(scalar_part);
        let e2: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e2();
        let expected = EuclideanMultivector3::zero();
        let result = scalar >> e2;

        assert_eq!(result, expected);
    }

    #[test]
    fn test_right_contraction_scalar_e3() {
        let scalar_part = 3_f64;
        let scalar = EuclideanMultivector3::from_scalar(scalar_part);
        let e3: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e3();
        let expected = EuclideanMultivector3::zero();
        let result = scalar >> e3;

        assert_eq!(result, expected);
    }

    #[test]
    fn test_right_contraction_scalar_e12() {
        let scalar_part = 3_f64;
        let scalar = EuclideanMultivector3::from_scalar(scalar_part);
        let e12: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e12();
        let expected = EuclideanMultivector3::zero();
        let result = scalar >> e12;

        assert_eq!(result, expected);
    }

    #[test]
    fn test_right_contraction_scalar_e23() {
        let scalar_part = 3_f64;
        let scalar = EuclideanMultivector3::from_scalar(scalar_part);
        let e23: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e23();
        let expected = EuclideanMultivector3::zero();
        let result = scalar >> e23;

        assert_eq!(result, expected);
    }

    #[test]
    fn test_right_contraction_scalar_e31() {
        let scalar_part = 3_f64;
        let scalar = EuclideanMultivector3::from_scalar(scalar_part);
        let e31: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e31();
        let expected = EuclideanMultivector3::zero();
        let result = scalar >> e31;

        assert_eq!(result, expected);
    }

    #[test]
    fn test_right_contraction_scalar_e123() {
        let scalar_part = 3_f64;
        let scalar = EuclideanMultivector3::from_scalar(scalar_part);
        let e123: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e123();
        let expected = EuclideanMultivector3::zero();
        let result = scalar >> e123;

        assert_eq!(result, expected);
    }

    #[test]
    fn test_right_contraction_e1_scalar() {
        let scalar_part = 3_f64;
        let scalar = EuclideanMultivector3::from_scalar(scalar_part);
        let e1: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e1();
        let expected = EuclideanMultivector3::new(
            0_f64, scalar_part, 0_f64, 0_f64, 0_f64, 0_f64, 0_f64, 0_f64
        );
        let result = e1 >> scalar;

        assert_eq!(result, expected);
    }

    #[test]
    fn test_right_contraction_e2_scalar() {
        let scalar_part = 3_f64;
        let scalar = EuclideanMultivector3::from_scalar(scalar_part);
        let e2: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e2();
        let expected = EuclideanMultivector3::new(
            0_f64, 0_f64, scalar_part, 0_f64, 0_f64, 0_f64, 0_f64, 0_f64
        );
        let result = e2 >> scalar;

        assert_eq!(result, expected);
    }

    #[test]
    fn test_right_contraction_e3_scalar() {
        let scalar_part = 3_f64;
        let scalar = EuclideanMultivector3::from_scalar(scalar_part);
        let e3: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e3();
        let expected = EuclideanMultivector3::new(
            0_f64, 0_f64, 0_f64, scalar_part, 0_f64, 0_f64, 0_f64, 0_f64
        );
        let result = e3 >> scalar;

        assert_eq!(result, expected);
    }

    #[test]
    fn test_right_contraction_e12_scalar() {
        let scalar_part = 3_f64;
        let scalar = EuclideanMultivector3::from_scalar(scalar_part);
        let e12: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e12();
        let expected = EuclideanMultivector3::new(
            0_f64, 0_f64, 0_f64, 0_f64, scalar_part, 0_f64, 0_f64, 0_f64
        );
        let result = e12 >> scalar;

        assert_eq!(result, expected);
    }

    #[test]
    fn test_right_contraction_e23_scalar() {
        let scalar_part = 3_f64;
        let scalar = EuclideanMultivector3::from_scalar(scalar_part);
        let e23: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e23();
        let expected = EuclideanMultivector3::new(
            0_f64, 0_f64, 0_f64, 0_f64, 0_f64, scalar_part, 0_f64, 0_f64
        );
        let result = e23 >> scalar;

        assert_eq!(result, expected);
    }

    #[test]
    fn test_right_contraction_e31_scalar() {
        let scalar_part = 3_f64;
        let scalar = EuclideanMultivector3::from_scalar(scalar_part);
        let e31: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e31();
        let expected = EuclideanMultivector3::new(
            0_f64, 0_f64, 0_f64, 0_f64, 0_f64, 0_f64, scalar_part, 0_f64
        );
        let result = e31 >> scalar;

        assert_eq!(result, expected);
    }

    #[test]
    fn test_right_contraction_e123_scalar() {
        let scalar_part = 3_f64;
        let scalar = EuclideanMultivector3::from_scalar(scalar_part);
        let e123: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e123();
        let expected = EuclideanMultivector3::new(
            0_f64, 0_f64, 0_f64, 0_f64, 0_f64, 0_f64, 0_f64, scalar_part
        );
        let result = e123 >> scalar;

        assert_eq!(result, expected);
    }

    #[test]
    fn test_right_contraction_e1_e1() {
        let e1: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e1();
        let one: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_scalar();

        assert_eq!(e1 >> e1, one);
    }

    #[test]
    fn test_right_contraction_e1_e2() {
        let e1: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e1();
        let e2: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e2();
        let zero: EuclideanMultivector3<f64> = EuclideanMultivector3::zero();

        assert_eq!(e1 >> e2, zero);
    }

    #[test]
    fn test_right_contraction_e1_e3() {
        let e1: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e1();
        let e3: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e3();
        let zero: EuclideanMultivector3<f64> = EuclideanMultivector3::zero();

        assert_eq!(e1 >> e3, zero);
    }

    #[test]
    fn test_right_contraction_e1_e12() {
        let e1: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e1();
        let e12: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e12();
        let zero: EuclideanMultivector3<f64> = EuclideanMultivector3::zero();

        assert_eq!(e1 >> e12, zero);
    }

    #[test]
    fn test_right_contraction_e1_e23() {
        let e1: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e1();
        let e23: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e23();
        let zero: EuclideanMultivector3<f64> = EuclideanMultivector3::zero();

        assert_eq!(e1 >> e23, zero);
    }

    #[test]
    fn test_right_contraction_e1_e31() {
        let e1: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e1();
        let e31: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e31();
        let zero: EuclideanMultivector3<f64> = EuclideanMultivector3::zero();

        assert_eq!(e1 >> e31, zero);
    }

    #[test]
    fn test_right_contraction_e1_e123() {
        let e1: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e1();
        let e123: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e123();
        let zero: EuclideanMultivector3<f64> = EuclideanMultivector3::zero();

        assert_eq!(e1 >> e123, zero);
    }

    #[test]
    fn test_right_contraction_e2_e1() {
        let e1: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e1();
        let e2: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e2();
        let zero: EuclideanMultivector3<f64> = EuclideanMultivector3::zero();

        assert_eq!(e2 >> e1, zero);
    }

    #[test]
    fn test_right_contraction_e2_e2() {
        let e2: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e2();
        let one: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_scalar();

        assert_eq!(e2 >> e2, one);
    }

    #[test]
    fn test_right_contraction_e2_e3() {
        let e2: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e2();
        let e3: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e3();
        let zero: EuclideanMultivector3<f64> = EuclideanMultivector3::zero();

        assert_eq!(e2 >> e3, zero);
    }

    #[test]
    fn test_right_contraction_e2_e12() {
        let e2: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e2();
        let e12: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e12();
        let zero: EuclideanMultivector3<f64> = EuclideanMultivector3::zero();

        assert_eq!(e2 >> e12, zero);
    }

    #[test]
    fn test_right_contraction_e2_e23() {
        let e2: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e2();
        let e23: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e23();
        let zero: EuclideanMultivector3<f64> = EuclideanMultivector3::zero();

        assert_eq!(e2 >> e23, zero);
    }

    #[test]
    fn test_right_contraction_e2_e31() {
        let e2: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e2();
        let e31: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e31();
        let zero: EuclideanMultivector3<f64> = EuclideanMultivector3::zero();

        assert_eq!(e2 >> e31, zero);
    }

    #[test]
    fn test_right_contraction_e2_e123() {
        let e2: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e2();
        let e123: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e123();
        let zero: EuclideanMultivector3<f64> = EuclideanMultivector3::zero();

        assert_eq!(e2 >> e123, zero);
    }

    #[test]
    fn test_right_contraction_e3_e1() {
        let e1: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e1();
        let e3: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e3();
        let zero: EuclideanMultivector3<f64> = EuclideanMultivector3::zero();

        assert_eq!(e3 >> e1, zero);
    }

    #[test]
    fn test_right_contraction_e3_e2() {
        let e2: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e2();
        let e3: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e3();
        let zero: EuclideanMultivector3<f64> = EuclideanMultivector3::zero();

        assert_eq!(e3 >> e2, zero);
    }

    #[test]
    fn test_right_contraction_e3_e3() {
        let e3: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e3();
        let one: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_scalar();

        assert_eq!(e3 >> e3, one);
    }

    #[test]
    fn test_right_contraction_e3_e12() {
        let e3: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e3();
        let e12: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e12();
        let zero: EuclideanMultivector3<f64> = EuclideanMultivector3::zero();

        assert_eq!(e3 >> e12, zero);
    }

    #[test]
    fn test_right_contraction_e3_e23() {
        let e3: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e3();
        let e23: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e23();
        let zero: EuclideanMultivector3<f64> = EuclideanMultivector3::zero();

        assert_eq!(e3 >> e23, zero);
    }

    #[test]
    fn test_right_contraction_e3_e31() {
        let e3: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e3();
        let e31: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e31();
        let zero: EuclideanMultivector3<f64> = EuclideanMultivector3::zero();

        assert_eq!(e3 >> e31, zero);
    }

    #[test]
    fn test_right_contraction_e3_e123() {
        let e3: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e3();
        let e123: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e123();
        let zero: EuclideanMultivector3<f64> = EuclideanMultivector3::zero();

        assert_eq!(e3 >> e123, zero);
    }

    #[test]
    fn test_right_contraction_e12_e1() {
        let e1: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e1();
        let e2: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e2();
        let e12: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e12();

        assert_eq!(e12 >> e1, -e2);
    }

    #[test]
    fn test_right_contraction_e12_e2() {
        let e1: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e1();
        let e2: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e2();
        let e12: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e12();

        assert_eq!(e12 >> e2, e1);
    }

    #[test]
    fn test_right_contraction_e12_e3() {
        let e3: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e3();
        let e12: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e12();
        let zero: EuclideanMultivector3<f64> = EuclideanMultivector3::zero();

        assert_eq!(e12 >> e3, zero);
    }

    #[test]
    fn test_right_contraction_e12_e12() {
        let e12: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e12();
        let one: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_scalar();

        assert_eq!(e12 >> e12, -one);
    }

    #[test]
    fn test_right_contraction_e12_e23() {
        let e12: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e12();
        let e23: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e23();
        let zero: EuclideanMultivector3<f64> = EuclideanMultivector3::zero();

        assert_eq!(e12 >> e23, zero);
    }

    #[test]
    fn test_right_contraction_e12_e31() {
        let e12: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e12();
        let e31: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e31();
        let zero: EuclideanMultivector3<f64> = EuclideanMultivector3::zero();

        assert_eq!(e12 >> e31, zero);
    }

    #[test]
    fn test_right_contraction_e12_e123() {
        let e12: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e12();
        let e123: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e123();
        let zero: EuclideanMultivector3<f64> = EuclideanMultivector3::zero();

        assert_eq!(e12 >> e123, zero);
    }


    #[test]
    fn test_right_contraction_e23_e1() {
        let e1: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e1();
        let e23: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e23();
        let zero: EuclideanMultivector3<f64> = EuclideanMultivector3::zero();

        assert_eq!(e23 >> e1, zero);
    }

    #[test]
    fn test_right_contraction_e23_e2() {
        let e2: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e2();
        let e3: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e3();
        let e23: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e23();

        assert_eq!(e23 >> e2, -e3);
    }

    #[test]
    fn test_right_contraction_e23_e3() {
        let e2: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e2();
        let e3: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e3();
        let e23: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e23();

        assert_eq!(e23 >> e3, e2);
    }

    #[test]
    fn test_right_contraction_e23_e12() {
        let e12: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e12();
        let e23: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e23();
        let zero: EuclideanMultivector3<f64> = EuclideanMultivector3::zero();

        assert_eq!(e23 >> e12, zero);
    }

    #[test]
    fn test_right_contraction_e23_e23() {
        let e23: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e23();
        let one: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_scalar();

        assert_eq!(e23 >> e23, -one);
    }

    #[test]
    fn test_right_contraction_e23_e31() {
        let e23: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e23();
        let e31: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e31();
        let zero: EuclideanMultivector3<f64> = EuclideanMultivector3::zero();

        assert_eq!(e23 >> e31, zero);
    }

    #[test]
    fn test_right_contraction_e23_e123() {
        let e23: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e23();
        let e123: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e123();
        let zero: EuclideanMultivector3<f64> = EuclideanMultivector3::zero();

        assert_eq!(e23 >> e123, zero);
    }

    #[test]
    fn test_right_contraction_e31_e1() {
        let e1: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e1();
        let e3: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e3();
        let e31: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e31();

        assert_eq!(e31 >> e1, e3);
    }

    #[test]
    fn test_right_contraction_e31_e2() {
        let e2: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e2();
        let e31: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e31();
        let zero: EuclideanMultivector3<f64> = EuclideanMultivector3::zero();

        assert_eq!(e31 >> e2, zero);
    }

    #[test]
    fn test_right_contraction_e31_e3() {
        let e1: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e1();
        let e3: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e3();
        let e31: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e31();

        assert_eq!(e31 >> e3, -e1);
    }

    #[test]
    fn test_right_contraction_e31_e12() {
        let e12: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e12();
        let e31: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e31();
        let zero: EuclideanMultivector3<f64> = EuclideanMultivector3::zero();

        assert_eq!(e31 >> e12, zero);
    }

    #[test]
    fn test_right_contraction_e31_e23() {
        let e23: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e23();
        let e31: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e31();
        let zero: EuclideanMultivector3<f64> = EuclideanMultivector3::zero();

        assert_eq!(e31 >> e23, zero);
    }

    #[test]
    fn test_right_contraction_e31_e31() {
        let e31: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e31();
        let one: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_scalar();

        assert_eq!(e31 >> e31, -one);
    }

    #[test]
    fn test_right_contraction_e31_e123() {
        let e31: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e31();
        let e123: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e123();
        let zero: EuclideanMultivector3<f64> = EuclideanMultivector3::zero();

        assert_eq!(e31 >> e123, zero);
    }

    #[test]
    fn test_right_contraction_e123_e1() {
        let e1: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e1();
        let e23: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e23();
        let e123: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e123();

        assert_eq!(e123 >> e1, e23);
    }

    #[test]
    fn test_right_contraction_e123_e2() {
        let e2: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e2();
        let e31: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e31();
        let e123: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e123();

        assert_eq!(e123 >> e2, e31);
    }

    #[test]
    fn test_right_contraction_e123_e3() {
        let e3: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e3();
        let e12: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e12();
        let e123: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e123();

        assert_eq!(e123 >> e3, e12);
    }

    #[test]
    fn test_right_contraction_e123_e12() {
        let e3: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e3();
        let e12: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e12();
        let e123: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e123();

        assert_eq!(e123 >> e12, -e3);
    }

    #[test]
    fn test_right_contraction_e123_e23() {
        let e1: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e1();
        let e23: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e23();
        let e123: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e123();

        assert_eq!(e123 >> e23, -e1);
    }

    #[test]
    fn test_right_contraction_e123_e31() {
        let e2: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e2();
        let e31: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e31();
        let e123: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e123();

        assert_eq!(e123 >> e31, -e2);
    }

    #[test]
    fn test_right_contraction_e123_e123() {
        let e123: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e123();
        let one: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_scalar();

        assert_eq!(e123 >> e123, -one);
    }

    #[test]
    fn test_right_contraction_scalar_multivector() {
        let scalar_part = 3_f64;
        let scalar = EuclideanMultivector3::from_scalar(scalar_part);
        let mv = EuclideanMultivector3::new(
            1_f64, 2_f64, 3_f64, 4_f64, 5_f64, 6_f64, 7_f64, 8_f64
        );
        let expected = EuclideanMultivector3::new(
            3_f64, 0_f64, 0_f64, 0_f64, 0_f64, 0_f64, 0_f64, 0_f64
        );
        let result = scalar >> mv;

        assert_eq!(result, expected);
    }

    #[test]
    fn test_right_contraction_multivector_multivector() {
        let mv1 = EuclideanMultivector3::new(
            1_f64, 2_f64, 3_f64, 4_f64, 5_f64, 6_f64, 7_f64, 8_f64
        );
        let mv2 = EuclideanMultivector3::new(
            3_f64, 5_f64, 7_f64, 9_f64, 11_f64, 13_f64, 15_f64, 17_f64
        );
        let expected = EuclideanMultivector3::new(
            -304_f64, -126_f64, -82_f64, -83_f64, 87_f64, 58_f64, 77_f64, 24_f64
        );
        let result = mv1 >> mv2;

        assert_eq!(result, expected);
    }

    #[test]
    fn test_right_contraction_multivector_with_self() {
        let mv = EuclideanMultivector3::new(
            1_f64, 2_f64, 3_f64, 4_f64, 5_f64, 6_f64, 7_f64, 8_f64
        );
        let expected = EuclideanMultivector3::new(
            -144_f64, -59_f64, -39_f64, -40_f64, 37_f64, 22_f64, 31_f64, 8_f64
        );
        let result = mv >> mv;

        assert_eq!(result, expected);
    }

    #[test]
    fn test_right_contraction_zero_multivector() {
        let mv = EuclideanMultivector3::new(
            1_f64, 2_f64, 3_f64, 4_f64, 5_f64, 6_f64, 7_f64, 8_f64
        );
        let zero = EuclideanMultivector3::zero();
    
        assert_eq!(zero >> mv, zero);
    }

    #[test]
    fn test_right_contraction_multivector_zero() {
        let mv = EuclideanMultivector3::new(
            1_f64, 2_f64, 3_f64, 4_f64, 5_f64, 6_f64, 7_f64, 8_f64
        );
        let zero = EuclideanMultivector3::zero();

        assert_eq!(mv >> zero, zero);
    }

    #[test]
    fn test_right_contraction_one_multivector() {
        let mv = EuclideanMultivector3::new(
            1_f64, 2_f64, 3_f64, 4_f64, 5_f64, 6_f64, 7_f64, 8_f64
        );
        let one = EuclideanMultivector3::unit_scalar();
    
        assert_eq!(mv >> one, mv);
    }

    #[test]
    fn test_right_contraction_multivector_one() {
        let mv = EuclideanMultivector3::new(
            1_f64, 2_f64, 3_f64, 4_f64, 5_f64, 6_f64, 7_f64, 8_f64
        );
        let one = EuclideanMultivector3::unit_scalar();
    
        assert_eq!(one >> mv, one);
    }

    #[test]
    fn test_right_contraction_e12_e21() {
        let e1: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e1();
        let e2: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e2();
        let one: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_scalar();

        assert_eq!((e1 ^ e2) >> (e2 ^ e1), one);
    }

    #[test]
    fn test_right_contraction_e21_e12() {
        let e1: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e1();
        let e2: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e2();
        let one: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_scalar();

        assert_eq!((e2 ^ e1) >> (e1 ^ e2), one);
    }

    #[test]
    fn test_right_contraction_e23_e32() {
        let e2: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e2();
        let e3: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e3();
        let one: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_scalar();

        assert_eq!((e2 ^ e3) >> (e3 ^ e2), one);
    }

    #[test]
    fn test_right_contraction_e32_e23() {
        let e2: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e2();
        let e3: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e3();
        let one: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_scalar();

        assert_eq!((e3 ^ e2) >> (e2 ^ e3), one);
    }

    #[test]
    fn test_right_contraction_e31_e13() {
        let e1: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e1();
        let e3: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e3();
        let one: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_scalar();

        assert_eq!((e3 ^ e1) >> (e1 ^ e3), one);
    }

    #[test]
    fn test_right_contraction_e13_e31() {
        let e1: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e1();
        let e3: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e3();
        let one: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_scalar();

        assert_eq!((e1 ^ e3) >> (e3 ^ e1), one);
    }

    #[test]
    fn test_right_contraction_e123_e321() {
        let e1: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e1();
        let e2: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e2();
        let e3: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e3();
        let one: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_scalar();

        assert_eq!((e1 ^ e2 ^ e3) >> (e3 ^ e2 ^ e1), one);
    }

    #[test]
    fn test_right_contraction_e321_e123() {
        let e1: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e1();
        let e2: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e2();
        let e3: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e3();
        let one: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_scalar();

        assert_eq!((e3 ^ e2 ^ e1) >> (e1 ^ e2 ^ e3), one);
    }

    #[test]
    fn test_commutator_scalar_scalar() {
        let scalar1 = EuclideanMultivector3::from_scalar(6_f64);
        let scalar2 = EuclideanMultivector3::from_scalar(45_f64);
        let expected = EuclideanMultivector3::zero();
        let result = scalar1.commutator(&scalar2);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_commutator_scalar_e1() {
        let scalar = EuclideanMultivector3::from_scalar(6_f64);
        let e1 = EuclideanMultivector3::unit_e1();
        let expected = EuclideanMultivector3::zero();
        let result = scalar.commutator(&e1);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_commutator_e1_scalar() {
        let scalar = EuclideanMultivector3::from_scalar(6_f64);
        let e1 = EuclideanMultivector3::unit_e1();
        let expected = EuclideanMultivector3::zero();
        let result = e1.commutator(&scalar);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_commutator_scalar_e2() {
        let scalar = EuclideanMultivector3::from_scalar(6_f64);
        let e2 = EuclideanMultivector3::unit_e2();
        let expected = EuclideanMultivector3::zero();
        let result = scalar.commutator(&e2);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_commutator_e2_scalar() {
        let scalar = EuclideanMultivector3::from_scalar(6_f64);
        let e2 = EuclideanMultivector3::unit_e2();
        let expected = EuclideanMultivector3::zero();
        let result = e2.commutator(&scalar);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_commutator_scalar_e3() {
        let scalar = EuclideanMultivector3::from_scalar(6_f64);
        let e3 = EuclideanMultivector3::unit_e3();
        let expected = EuclideanMultivector3::zero();
        let result = scalar.commutator(&e3);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_commutator_e3_scalar() {
        let scalar = EuclideanMultivector3::from_scalar(6_f64);
        let e3 = EuclideanMultivector3::unit_e3();
        let expected = EuclideanMultivector3::zero();
        let result = e3.commutator(&scalar);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_commutator_scalar_e12() {
        let scalar = EuclideanMultivector3::from_scalar(6_f64);
        let e12 = EuclideanMultivector3::unit_e12();
        let expected = EuclideanMultivector3::zero();
        let result = scalar.commutator(&e12);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_commutator_e12_scalar() {
        let scalar = EuclideanMultivector3::from_scalar(6_f64);
        let e12 = EuclideanMultivector3::unit_e12();
        let expected = EuclideanMultivector3::zero();
        let result = e12.commutator(&scalar);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_commutator_scalar_e23() {
        let scalar = EuclideanMultivector3::from_scalar(6_f64);
        let e23 = EuclideanMultivector3::unit_e23();
        let expected = EuclideanMultivector3::zero();
        let result = scalar.commutator(&e23);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_commutator_e23_scalar() {
        let scalar = EuclideanMultivector3::from_scalar(6_f64);
        let e23 = EuclideanMultivector3::unit_e23();
        let expected = EuclideanMultivector3::zero();
        let result = e23.commutator(&scalar);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_commutator_scalar_e31() {
        let scalar = EuclideanMultivector3::from_scalar(6_f64);
        let e31 = EuclideanMultivector3::unit_e31();
        let expected = EuclideanMultivector3::zero();
        let result = scalar.commutator(&e31);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_commutator_e31_scalar() {
        let scalar = EuclideanMultivector3::from_scalar(6_f64);
        let e31 = EuclideanMultivector3::unit_e31();
        let expected = EuclideanMultivector3::zero();
        let result = e31.commutator(&scalar);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_commutator_scalar_e123() {
        let scalar = EuclideanMultivector3::from_scalar(6_f64);
        let e123 = EuclideanMultivector3::unit_e123();
        let expected = EuclideanMultivector3::zero();
        let result = scalar.commutator(&e123);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_commutator_e123_scalar() {
        let scalar = EuclideanMultivector3::from_scalar(6_f64);
        let e123 = EuclideanMultivector3::unit_e123();
        let expected = EuclideanMultivector3::zero();
        let result = e123.commutator(&scalar);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_commutator_scalar_multivector() {
        let scalar = EuclideanMultivector3::from_scalar(7_f64);
        let mv = EuclideanMultivector3::new(
            1_f64, 2_f64, 3_f64, 4_f64, 5_f64, 6_f64, 7_f64, 8_f64
        );
        let expected: EuclideanMultivector3<f64> = EuclideanMultivector3::zero();
        let result = scalar.commutator(&mv);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_commutator_multivector_scalar() {
        let scalar = EuclideanMultivector3::from_scalar(7_f64);
        let mv = EuclideanMultivector3::new(
            1_f64, 2_f64, 3_f64, 4_f64, 5_f64, 6_f64, 7_f64, 8_f64
        );
        let expected: EuclideanMultivector3<f64> = EuclideanMultivector3::zero();
        let result = mv.commutator(&scalar);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_dual_e1() {
        let e1: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e1();
        let e1_dual = e1.dual();
        let zero: EuclideanMultivector3<f64> = EuclideanMultivector3::zero();

        assert_eq!(e1 << e1_dual, zero);
        assert_eq!(e1 >> e1_dual, zero);
    }

    #[test]
    fn test_dual_e2() {
        let e2: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e2();
        let e2_dual = e2.dual();
        let zero: EuclideanMultivector3<f64> = EuclideanMultivector3::zero();

        assert_eq!(e2 << e2_dual, zero);
        assert_eq!(e2 >> e2_dual, zero);
    }

    #[test]
    fn test_dual_e3() {
        let e3: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e3();
        let e3_dual = e3.dual();
        let zero: EuclideanMultivector3<f64> = EuclideanMultivector3::zero();

        assert_eq!(e3 << e3_dual, zero);
        assert_eq!(e3 >> e3_dual, zero);
    }

    #[test]
    fn test_dual_e12() {
        let e12: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e12();
        let e12_dual = e12.dual();
        let zero: EuclideanMultivector3<f64> = EuclideanMultivector3::zero();

        assert_eq!(e12 << e12_dual, zero);
        assert_eq!(e12 >> e12_dual, zero);
    }

    #[test]
    fn test_dual_e23() {
        let e23: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e23();
        let e23_dual = e23.dual();
        let zero: EuclideanMultivector3<f64> = EuclideanMultivector3::zero();

        assert_eq!(e23 << e23_dual, zero);
        assert_eq!(e23 >> e23_dual, zero);
    }

    #[test]
    fn test_dual_e31() {
        let e31: EuclideanMultivector3<f64> = EuclideanMultivector3::unit_e31();
        let e31_dual = e31.dual();
        let zero: EuclideanMultivector3<f64> = EuclideanMultivector3::zero();

        assert_eq!(e31 << e31_dual, zero);
        assert_eq!(e31 >> e31_dual, zero);
    }
}

