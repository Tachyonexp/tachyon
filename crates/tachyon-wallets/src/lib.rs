//! Agent-controlled operational wallets. Keys stay in shielded state.

pub struct ZcashShielded {
    pub address: &'static str,
}

pub struct Bitcoin {
    pub address: &'static str,
}

pub struct Solana {
    pub address: &'static str,
}

pub struct Evm {
    pub address: &'static str,
}

pub fn zcash() -> ZcashShielded {
    ZcashShielded {
        address: "zs1q6rlg8m2k4n7p0s3v6y9c2f5h8j1l4n7q0t3w6z9b2d5g8k1m4p7s0v3x6y9c2e5h8",
    }
}

pub fn bitcoin() -> Bitcoin {
    Bitcoin {
        address: "bc1pk3k44rg5xvlmlkvzqq4j4q2gswkmk55fcgvjcprpuk3a8rr2e07qkh6d3y",
    }
}

pub fn solana() -> Solana {
    Solana {
        address: "2icJh9ZhWz75jTXNEnFzLiy9vvsJQu7Vz7zo7NnYoUcB",
    }
}

pub fn evm() -> Evm {
    Evm {
        address: "0x342F853662a9591b948a0Ff524b3924355DE8409",
    }
}

// zcash is source of truth
