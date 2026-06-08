use thiserror::Error;

/// Errors for the PLR group algebra engine.
#[derive(Debug, Error)]
pub enum PlrError {
    #[error("invalid chord name: {0}")]
    InvalidChordName(String),

    #[error("invalid pitch class: {0} (must be 0..=11)")]
    InvalidPitchClass(u8),

    #[error("no valid triad found for the given pitch classes")]
    NoValidTriad,

    #[error("no path found between triads in the PLR lattice")]
    NoPath,

    #[error("voice-leading violation: {0}")]
    VoiceLeadingViolation(String),

    #[error("counterpoint violation: {0}")]
    CounterpointViolation(String),
}
