//! Shielded notes: intent fragments, state commitments, wallet handles.

#[derive(Clone, Debug)]
pub struct NoteCommitment(pub [u8; 32]);

#[derive(Clone, Debug)]
pub struct Nullifier(pub [u8; 32]);

#[derive(Clone, Debug)]
pub struct ShieldedNote {
    pub cm: NoteCommitment,
    pub enc: Vec<u8>,
}

pub fn spend(note: &ShieldedNote) -> Nullifier {
    let mut n = [0u8; 32];
    n.copy_from_slice(&note.cm.0);
    n[0] ^= 0xff;
    Nullifier(n)
}
