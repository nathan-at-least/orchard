use crate::{keys::NullifierKey, value::NoteValue};

/// A discrete amount of funds received by an address.
pub struct Note {
    /// The recipient of the funds.
    recipient: (),
    /// The value of this note.
    value: NoteValue,
    /// The randomness used to blind the [`NoteCommitment`].
    ///
    /// TODO: This will end up generated from a common seed.
    blinding_factor: (),
}

impl Note {
    fn commitment(&self) -> NoteCommitment {
        todo!()
    }

    fn nullifier(&self, nk: &NullifierKey) -> Nullifier {
        todo!()
    }
}

pub struct EncryptedNote;

/// A commitment to a note.
pub struct NoteCommitment;

pub struct Nullifier;
