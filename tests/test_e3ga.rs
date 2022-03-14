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
}

