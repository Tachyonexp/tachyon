//! Thin adapters: private intent → signed action + public receipt.

#[derive(Clone, Debug)]
pub struct Receipt {
    pub chain: &'static str,
    pub digest: [u8; 32],
}

pub fn bitcoin_step(intent: &[u8; 32]) -> Receipt {
    Receipt { chain: "bitcoin", digest: *intent }
}

pub fn solana_step(intent: &[u8; 32]) -> Receipt {
    Receipt { chain: "solana", digest: *intent }
}

pub fn evm_step(intent: &[u8; 32]) -> Receipt {
    Receipt { chain: "evm", digest: *intent }
}

pub fn zcash_step(intent: &[u8; 32]) -> Receipt {
    Receipt { chain: "zcash", digest: *intent }
}

// no calldata dump
// digest only
