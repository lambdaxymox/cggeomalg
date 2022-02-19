extern crate cggeomalg;
extern crate num_traits;


#[cfg(test)]
mod e2ga_test {
    use cggeomalg::e2ga::{
        EuclideanMultivector2,
    };


    #[test]
    fn test_multivector_addition() {
        let mv1: EuclideanMultivector2<isize> = EuclideanMultivector2::new(1, 2, 3, 4);
        let mv2: EuclideanMultivector2<isize> = EuclideanMultivector2::new(5, 6, 7, 8);
        let expected = EuclideanMultivector2::new(6, 6, 10, 12);
        let result = mv1 + mv2;

        assert_eq!(result, expected);
    }


}

