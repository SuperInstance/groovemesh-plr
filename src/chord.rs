use serde::{Deserialize, Serialize};

use crate::error::PlrError;

/// 12 pitch classes (C=0, C#=1, ... B=11)
pub type PitchClass = u8;

/// Note name letters
const NOTE_NAMES: [&str; 12] = [
    "C", "C#", "D", "D#", "E", "F", "F#", "G", "G#", "A", "A#", "B",
];

/// Triad quality
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Quality {
    Major,
    Minor,
}

impl std::fmt::Display for Quality {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Quality::Major => write!(f, "major"),
            Quality::Minor => write!(f, "minor"),
        }
    }
}

/// A triad (root + quality) — one of 24 possible triads
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Triad {
    pub root: PitchClass,
    pub quality: Quality,
}

impl Triad {
    /// Create a new triad, validating the root is in 0..=11.
    pub fn new(root: PitchClass, quality: Quality) -> Result<Self, PlrError> {
        if root > 11 {
            return Err(PlrError::InvalidPitchClass(root));
        }
        Ok(Self { root, quality })
    }

    /// Create a triad without validation (for internal use where root is known valid).
    pub const fn new_unchecked(root: PitchClass, quality: Quality) -> Self {
        Self { root, quality }
    }

    /// Parse from chord name: "C" = C major, "Am" = A minor.
    pub fn from_name(name: &str) -> Result<Self, PlrError> {
        let name = name.trim();
        if name.is_empty() {
            return Err(PlrError::InvalidChordName(String::new()));
        }

        // Determine quality and extract note portion
        let (note_part, quality) = if name.len() > 1 && name.ends_with('m') {
            // "Am", "C#m", etc. — but not just "m" alone
            let candidate = &name[..name.len() - 1];
            // Make sure it's not something like "Bbm" where 'm' really means minor
            // Also handle "maj" suffix
            if NOTE_NAMES.contains(&candidate) {
                (candidate, Quality::Minor)
            } else {
                // Could be a note name without 'm', treat as major
                (name, Quality::Major)
            }
        } else if let Some(stripped) = name.strip_suffix("maj") {
            (stripped, Quality::Major)
        } else {
            (name, Quality::Major)
        };

        let root = NOTE_NAMES
            .iter()
            .position(|&n| n == note_part)
            .ok_or_else(|| PlrError::InvalidChordName(name.to_string()))?;

        Ok(Self {
            root: root as PitchClass,
            quality,
        })
    }

    /// All 24 major/minor triads.
    pub fn all() -> Vec<Triad> {
        let mut triads = Vec::with_capacity(24);
        for root in 0..12u8 {
            triads.push(Self::new_unchecked(root, Quality::Major));
            triads.push(Self::new_unchecked(root, Quality::Minor));
        }
        triads
    }

    /// The three pitch classes of this triad.
    /// Major triad: root, root+4, root+7
    /// Minor triad: root, root+3, root+7
    pub fn pitch_classes(&self) -> [PitchClass; 3] {
        let third = match self.quality {
            Quality::Major => (self.root + 4) % 12,
            Quality::Minor => (self.root + 3) % 12,
        };
        let fifth = (self.root + 7) % 12;
        [self.root, third, fifth]
    }

    /// The root note name.
    pub fn root_name(&self) -> &'static str {
        NOTE_NAMES[self.root as usize]
    }

    /// Check if this triad contains a given pitch class.
    pub fn contains(&self, pc: PitchClass) -> bool {
        self.pitch_classes().contains(&pc)
    }

    /// Number of common tones with another triad.
    pub fn common_tones(&self, other: &Triad) -> usize {
        let a = self.pitch_classes();
        let b = other.pitch_classes();
        a.iter().filter(|x| b.contains(x)).count()
    }
}

impl std::fmt::Display for Triad {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let suffix = match self.quality {
            Quality::Major => "",
            Quality::Minor => "m",
        };
        write!(f, "{}{}", NOTE_NAMES[self.root as usize], suffix)
    }
}
