//! Lexicographic base64 alphabet

use base64::{
    alphabet::Alphabet,
    engine::{general_purpose::NO_PAD, GeneralPurpose},
    Engine,
};

/// A lexicographic URL-safe alphabet.
pub const LEXICOGRAPHIC: Alphabet = const {
    if let Ok(v) = Alphabet::new("-0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ_abcdefghijklmnopqrstuvwxyz")
    {
        v
    } else {
        unreachable!()
    }
};

/// Encodes bytes using the lexicographic base64 alphabet
pub fn encode(bytes: impl AsRef<[u8]>) -> String {
    GeneralPurpose::new(&LEXICOGRAPHIC, NO_PAD).encode(bytes)
}
