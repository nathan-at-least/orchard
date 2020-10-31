//! Monetary values within the Pollard shielded pool.
//!
//! Values are represented in two places within Pollard:
//! - The value of an individual note, which is unsigned.
//! - The sum of note values within a Pollard [`Action`] or [`Bundle`], which is signed.
//!
//! We give these separate types within this crate. Users should map these types to their
//! own general "amount" type as appropriate.
//!
//! Inside the circuit, values are constrained to be 64-bit integers.
//! - TODO: Should this be constrained further to 53 bits? To Zcash's MAX_MONEY?
//!
//! [`Action`]: crate::Action
//! [`Bundle`]: crate::Bundle

/// The value of an individual Pollard note.
pub struct NoteValue(u64);

/// A sum of Pollard note values.
pub struct ValueSum(i64);

/// A commitment to a [`ValueSum`].
pub struct ValueCommitment;

impl ValueCommitment {
    fn new(value: ValueSum, blinding_factor: ()) -> Self {
        ValueCommitment
    }
}
