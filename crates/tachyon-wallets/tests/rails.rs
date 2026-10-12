use tachyon_wallets::{bitcoin, evm, solana, zcash};

#[test]
fn rails_are_set() {
    assert!(zcash().address.starts_with("zs1"));
    assert!(bitcoin().address.starts_with("bc1p"));
    assert!(solana().address.len() > 32);
    assert!(evm().address.starts_with("0x"));
}
