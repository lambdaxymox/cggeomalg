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
}

