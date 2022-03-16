extern crate cggeomalg;
extern crate criterion;
extern crate rand;
extern crate rand_isaac;


use cggeomalg::e2ga::{
    EuclideanMultivector2,
};
use core::ops::{
    Add,
    Sub,
    Mul,
    BitXor,
    BitOr,
    Shl,
    Shr,
};

use rand::{
    Rng, 
    prelude::Distribution,
    distributions::Standard,
};

use rand_isaac::{
    IsaacRng,
};

use criterion::{
    criterion_group,
    criterion_main,
};

fn gen_scalar<S>() -> S
where
    Standard: Distribution<S>
{
    use rand::SeedableRng;
    let mut rng = IsaacRng::seed_from_u64(0);

    rng.gen()
}

fn gen_multivector2<S>() -> EuclideanMultivector2<S> 
where 
    Standard: Distribution<S> 
{
    use rand::SeedableRng;
    let mut rng = IsaacRng::seed_from_u64(0);
    
    EuclideanMultivector2::new(rng.gen(), rng.gen(), rng.gen(), rng.gen())
}

macro_rules! bench_binop(
    ($name: ident, $scalar_type:ty, $type1:ty, $type2:ty, $generator_t1:ident, $generator_t2:ident, $binop:ident) => {
        fn $name(bh: &mut criterion::Criterion) {
            let a = $generator_t1::<$scalar_type>();
            let b = $generator_t2::<$scalar_type>();

            bh.bench_function(stringify!($name), move |bh| bh.iter(|| {
                a.$binop(b)
            }));
        }
    }
);

macro_rules! bench_binop_ref(
    ($name: ident, $scalar_type:ty, $type1:ty, $type2:ty, $generator_t1:ident, $generator_t2:ident, $binop:ident) => {
        fn $name(bh: &mut criterion::Criterion) {
            let a = $generator_t1::<$scalar_type>();
            let b = $generator_t2::<$scalar_type>();

            bh.bench_function(stringify!($name), move |bh| bh.iter(|| {
                a.$binop(&b)
            }));
        }
    }
);

macro_rules! bench_unop(
    ($name:ident, $scalar_type:ty, $ty:ty, $generator:ident, $unop:ident) => {
        fn $name(bh: &mut criterion::Criterion) {
            let v = $generator::<$scalar_type>();

            bh.bench_function(stringify!($name), move |bh| bh.iter(|| {
                v.$unop()
            }));
        }
    }
);

bench_binop!(
    multivector2_add_multivector2_f32, 
    f32, EuclideanMultivector2<f32>, EuclideanMultivector2<f32>, gen_multivector2, gen_multivector2, add
);
bench_binop!(
    multivector2_sub_multivector2_f32,
    f32, EuclideanMultivector2<f32>, EuclideanMultivector2<f32>, gen_multivector2, gen_multivector2, sub
);
bench_binop!(
    multivector2_mul_multivector2_f32,
    f32, EuclideanMultivector2<f32>, EuclideanMultivector2<f32>, gen_multivector2, gen_multivector2, mul
);
bench_binop!(
    multivector2_outer_product_multivector2_f32,
    f32, EuclideanMultivector2<f32>, EuclideanMultivector2<f32>, gen_multivector2, gen_multivector2, bitxor
);
bench_binop!(
    multivector2_scalar_product_multivector2_f32,
    f32, EuclideanMultivector2<f32>, EuclideanMultivector2<f32>, gen_multivector2, gen_multivector2, bitor
);
bench_binop!(
    multivector2_left_contract_multivector2_f32,
    f32, EuclideanMultivector2<f32>, EuclideanMultivector2<f32>, gen_multivector2, gen_multivector2, shl
);
bench_binop!(
    multivector2_right_contract_multivector2_f32,
    f32, EuclideanMultivector2<f32>, EuclideanMultivector2<f32>, gen_multivector2, gen_multivector2, shr
);

bench_binop_ref!(
    multivector2_commutator_multivector2_f32,
    f32, EuclideanMultivector2<f32>, EuclideanMultivector2<f32>, gen_multivector2, gen_multivector2, commutator
);
bench_binop_ref!(
    multivector2_anticommutator_multivector2_f32,
    f32, EuclideanMultivector2<f32>, EuclideanMultivector2<f32>, gen_multivector2, gen_multivector2, anticommutator
);

bench_unop!(multivector2_magnitude_f32, f32, EuclideanMultivector2<f32>, gen_multivector2, magnitude);
bench_unop!(multivector2_conjugate_f32, f32, EuclideanMultivector2<f32>, gen_multivector2, conjugate);
bench_unop!(multivector2_involute_f32, f32, EuclideanMultivector2<f32>, gen_multivector2, involute);
bench_unop!(multivector2_dual_f32, f32, EuclideanMultivector2<f32>, gen_multivector2, dual);
bench_unop!(multivector2_reverse_f32, f32, EuclideanMultivector2<f32>, gen_multivector2, reverse);
bench_unop!(multivector2_inverse_f32, f32, EuclideanMultivector2<f32>, gen_multivector2, inverse);


criterion_group!(
    e2ga_benchmarks,
    multivector2_add_multivector2_f32,
    multivector2_sub_multivector2_f32,
    multivector2_mul_multivector2_f32,
    multivector2_outer_product_multivector2_f32,
    multivector2_scalar_product_multivector2_f32,
    multivector2_left_contract_multivector2_f32,
    multivector2_right_contract_multivector2_f32,
    multivector2_commutator_multivector2_f32,
    multivector2_anticommutator_multivector2_f32,
    multivector2_magnitude_f32,
    multivector2_conjugate_f32,
    multivector2_involute_f32,
    multivector2_dual_f32,
    multivector2_reverse_f32,
    multivector2_inverse_f32,
);
criterion_main!(e2ga_benchmarks);

