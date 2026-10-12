use tachyon_notes::{spend, NoteCommitment, ShieldedNote};

#[test]
fn spend_changes_nullifier() {
    let n = ShieldedNote { cm: NoteCommitment([3u8; 32]), enc: vec![] };
    let nf = spend(&n);
    assert_ne!(nf.0, n.cm.0);
}
