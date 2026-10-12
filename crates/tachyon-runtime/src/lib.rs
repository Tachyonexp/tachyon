//! Agent loop: commit note, fold steps, settle through own wallets.

use tachyon_fold::Accumulator;
use tachyon_notes::{spend, NoteCommitment, ShieldedNote};

pub struct Plan {
    pub intent: [u8; 32],
}

pub fn run(plan: Plan) -> Accumulator {
    let note = ShieldedNote {
        cm: NoteCommitment(plan.intent),
        enc: vec![],
    };
    let _nf = spend(&note);
    let mut acc = Accumulator::default();
    let r = tachyon_adapters::zcash_step(&plan.intent);
    acc.fold(&r.digest, &plan.intent);
    acc
}
