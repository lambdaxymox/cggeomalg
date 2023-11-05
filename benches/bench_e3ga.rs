extern crate cggeomalg;
extern crate criterion;
extern crate rand;
extern crate rand_isaac;


use cggeomalg::e3ga::EuclideanMultivector3;
use core::ops::{
    Add,
    BitOr,
    BitXor,
    Mul,
    Shl,
    Shr,
    Sub,
};

use rand::{
    distributions::Standard,
    prelude::Distribution,
    Rng,
};

use rand_isaac::IsaacRng;

use criterion::{
    criterion_group,
    criterion_main,
};


fn gen_multivector3<S>() -> EuclideanMultivector3<S>
where
    Standard: Distribution<S>,
{
    use rand::SeedableRng;
    let mut rng = IsaacRng::seed_from_u64(0);

    EuclideanMultivector3::new(
        rng.gen(),
        rng.gen(),
        rng.gen(),
        rng.gen(),
        rng.gen(),
        rng.gen(),
        rng.gen(),
        rng.gen(),
    )
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
    multivector3_add_multivector3_f32,
    f32,
    EuclideanMultivector3<f32>,
    EuclideanMultivector3<f32>,
    gen_multivector3,
    gen_multivector3,
    add
);
bench_binop!(
    multivector3_sub_multivector3_f32,
    f32,
    EuclideanMultivector3<f32>,
    EuclideanMultivector3<f32>,
    gen_multivector3,
    gen_multivector3,
    sub
);
bench_binop!(
    multivector3_mul_multivector3_f32,
    f32,
    EuclideanMultivector3<f32>,
    EuclideanMultivector3<f32>,
    gen_multivector3,
    gen_multivector3,
    mul
);
bench_binop!(
    multivector3_outer_product_multivector3_f32,
    f32,
    EuclideanMultivector3<f32>,
    EuclideanMultivector3<f32>,
    gen_multivector3,
    gen_multivector3,
    bitxor
);
bench_binop!(
    multivector3_scalar_product_multivector3_f32,
    f32,
    EuclideanMultivector3<f32>,
    EuclideanMultivector3<f32>,
    gen_multivector3,
    gen_multivector3,
    bitor
);
bench_binop!(
    multivector3_left_contract_multivector3_f32,
    f32,
    EuclideanMultivector3<f32>,
    EuclideanMultivector3<f32>,
    gen_multivector3,
    gen_multivector3,
    shl
);
bench_binop!(
    multivector3_right_contract_multivector3_f32,
    f32,
    EuclideanMultivector3<f32>,
    EuclideanMultivector3<f32>,
    gen_multivector3,
    gen_multivector3,
    shr
);

bench_binop_ref!(
    multivector3_commutator_multivector3_f32,
    f32,
    EuclideanMultivector3<f32>,
    EuclideanMultivector3<f32>,
    gen_multivector3,
    gen_multivector3,
    commutator
);
bench_binop_ref!(
    multivector3_anticommutator_multivector3_f32,
    f32,
    EuclideanMultivector3<f32>,
    EuclideanMultivector3<f32>,
    gen_multivector3,
    gen_multivector3,
    anticommutator
);

bench_unop!(
    multivector3_magnitude_f32,
    f32,
    EuclideanMultivector3<f32>,
    gen_multivector3,
    magnitude
);
bench_unop!(
    multivector3_conjugate_f32,
    f32,
    EuclideanMultivector3<f32>,
    gen_multivector3,
    conjugate
);
bench_unop!(
    multivector3_involute_f32,
    f32,
    EuclideanMultivector3<f32>,
    gen_multivector3,
    involute
);
bench_unop!(multivector3_dual_f32, f32, EuclideanMultivector3<f32>, gen_multivector3, dual);
bench_unop!(multivector3_reverse_f32, f32, EuclideanMultivector3<f32>, gen_multivector3, reverse);
bench_unop!(multivector3_inverse_f32, f32, EuclideanMultivector3<f32>, gen_multivector3, inverse);


criterion_group!(
    e3ga_benchmarks,
    multivector3_add_multivector3_f32,
    multivector3_sub_multivector3_f32,
    multivector3_mul_multivector3_f32,
    multivector3_outer_product_multivector3_f32,
    multivector3_scalar_product_multivector3_f32,
    multivector3_left_contract_multivector3_f32,
    multivector3_right_contract_multivector3_f32,
    multivector3_commutator_multivector3_f32,
    multivector3_anticommutator_multivector3_f32,
    multivector3_magnitude_f32,
    multivector3_conjugate_f32,
    multivector3_involute_f32,
    multivector3_dual_f32,
    multivector3_reverse_f32,
    multivector3_inverse_f32,
);
criterion_main!(e3ga_benchmarks);
