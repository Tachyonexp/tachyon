use tachyon_fold::{verify, Accumulator};

#[test]
fn fold_then_verify() {
    let mut a = Accumulator::default();
    a.fold(&[1u8; 32], &[2u8; 32]);
    assert!(verify(&a));
}
