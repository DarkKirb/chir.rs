//! Lexicographic base64 alphabet

use base64::{
    alphabet::Alphabet,
    engine::{general_purpose::NO_PAD, GeneralPurpose},
    Engine,
};
use eyre::Result;

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

/// Decodes bytes using the lexicographic base64 alphabet
///
/// # Errors
/// This function returns an error if the message is not valid base64
pub fn decode(bytes: &str) -> Result<Vec<u8>> {
    Ok(GeneralPurpose::new(&LEXICOGRAPHIC, NO_PAD).decode(bytes)?)
}
