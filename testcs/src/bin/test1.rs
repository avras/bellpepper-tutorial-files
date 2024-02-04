use bellpepper_core::{test_cs::TestConstraintSystem, ConstraintSystem};
use pasta_curves::pallas::Scalar as Fq;

fn main() {
    let mut cs = TestConstraintSystem::<Fq>::new();
    let a = cs.alloc(|| "a var", || Ok(Fq::from(10u64))).unwrap();
    let b = cs.alloc(|| "b var", || Ok(Fq::from(4u64))).unwrap();
    let c = cs.alloc(|| "product", || Ok(Fq::from(40u64))).unwrap();

    cs.enforce(|| "mult", |lc| lc + a, |lc| lc + b, |lc| lc + c);
    assert!(cs.is_satisfied());
    assert_eq!(cs.num_constraints(), 1);
}
