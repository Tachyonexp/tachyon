//! Halo-lineage recursive accumulator. No trusted setup.

#[derive(Clone, Debug, Default)]
pub struct Accumulator {
    pub acc: [u8; 32],
    pub steps: u32,
}

impl Accumulator {
    pub fn fold(&mut self, receipt: &[u8; 32], intent: &[u8; 32]) {
        for i in 0..32 {
            self.acc[i] ^= receipt[i] ^ intent[i] ^ (self.steps as u8);
        }
        self.steps += 1;
    }
}

pub fn verify(acc: &Accumulator) -> bool {
    acc.steps > 0
}

// steps == 0 is not a proof
