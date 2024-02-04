use bellpepper_core::{num::AllocatedNum, test_cs::TestConstraintSystem, ConstraintSystem};
use pasta_curves::pallas::Scalar as Fq;

fn main() {
    let mut cs = TestConstraintSystem::<Fq>::new();

    let a = AllocatedNum::alloc(cs.namespace(|| "a"), || Ok(Fq::from(10u64))).unwrap();
    let b = AllocatedNum::alloc(cs.namespace(|| "b"), || Ok(Fq::from(4u64))).unwrap();
    let c = a.mul(&mut cs, &b).unwrap();

    assert!(cs.is_satisfied());
    assert!(c.get_value().unwrap() == Fq::from(40u64));
}
