use crate::chord::{PitchClass, Triad};
use crate::error::PlrError;
use crate::voice::voice_leading_distance;

/// Species counterpoint rules checker.
#[derive(Debug, Clone)]
pub struct CounterpointRules {
    /// Prohibit parallel fifths
    pub no_parallel_fifths: bool,
    /// Prohibit parallel octaves
    pub no_parallel_octaves: bool,
    /// Prohibit voice crossing
    pub no_voice_crossing: bool,
    /// Maximum allowed voice-leading distance
    pub max_voice_distance: Option<u32>,
}

impl Default for CounterpointRules {
    fn default() -> Self {
        Self {
            no_parallel_fifths: true,
            no_parallel_octaves: true,
            no_voice_crossing: true,
            max_voice_distance: Some(7),
        }
    }
}

impl CounterpointRules {
    pub fn new() -> Self {
        Self::default()
    }

    /// Check whether a voice leading between two triads satisfies counterpoint rules.
    pub fn check(&self, from: Triad, to: Triad) -> Result<(), PlrError> {
        let from_pcs = from.pitch_classes();
        let to_pcs = to.pitch_classes();

        // Check parallel fifths
        if self.no_parallel_fifths && has_parallel_interval(from_pcs, to_pcs, 7) {
            return Err(PlrError::CounterpointViolation(
                "parallel fifths detected".into(),
            ));
        }

        // Check parallel octaves
        if self.no_parallel_octaves && has_parallel_interval(from_pcs, to_pcs, 0) {
            return Err(PlrError::CounterpointViolation(
                "parallel octaves detected".into(),
            ));
        }

        // Check voice-leading distance
        if let Some(max) = self.max_voice_distance {
            let dist = voice_leading_distance(from, to);
            if dist > max {
                return Err(PlrError::CounterpointViolation(format!(
                    "voice-leading distance {} exceeds max {}",
                    dist, max
                )));
            }
        }

        Ok(())
    }

    /// Find the nearest PLR neighbor of `from` that satisfies counterpoint rules
    /// when moving toward `target`.
    pub fn legal_plr_step(
        &self,
        from: Triad,
        target: Triad,
    ) -> Option<(crate::transform::PLR, Triad)> {
        use crate::transform::{apply, PLR};

        let mut candidates: Vec<(PLR, Triad, u32)> = Vec::new();
        for &op in &[PLR::P, PLR::L, PLR::R] {
            let neighbor = apply(op, from);
            if self.check(from, neighbor).is_ok() {
                let dist = voice_leading_distance(neighbor, target);
                candidates.push((op, neighbor, dist));
            }
        }
        candidates.sort_by_key(|c| c.2);
        candidates.into_iter().next().map(|(op, t, _)| (op, t))
    }

    /// Walk from `start` toward `target` using only legal PLR steps.
    /// Returns the sequence of (operation, triad) pairs.
    pub fn legal_path(
        &self,
        start: Triad,
        target: Triad,
        max_steps: usize,
    ) -> Vec<(crate::transform::PLR, Triad)> {
        let mut path = Vec::new();
        let mut current = start;
        for _ in 0..max_steps {
            if current == target {
                break;
            }
            match self.legal_plr_step(current, target) {
                Some((op, next)) => {
                    path.push((op, next));
                    current = next;
                }
                None => break, // No legal step available
            }
        }
        path
    }
}

/// Check for parallel motion at a given interval (0=octave, 7=fifth)
/// between two pairs of pitch classes.
fn has_parallel_interval(from_pcs: [PitchClass; 3], to_pcs: [PitchClass; 3], interval: u8) -> bool {
    // Check all pairs of voices
    for i in 0..3 {
        for j in (i + 1)..3 {
            let from_interval = (from_pcs[i] as i16 - from_pcs[j] as i16).unsigned_abs() as u8 % 12;
            let to_interval = (to_pcs[i] as i16 - to_pcs[j] as i16).unsigned_abs() as u8 % 12;
            if from_interval == interval && to_interval == interval {
                // Both voices moved (didn't stay the same) → parallel
                if from_pcs[i] != to_pcs[i] && from_pcs[j] != to_pcs[j] {
                    return true;
                }
            }
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Quality;

    #[test]
    fn test_self_transition_legal() {
        let rules = CounterpointRules::new();
        let c = Triad::new_unchecked(0, Quality::Major);
        assert!(rules.check(c, c).is_ok());
    }

    #[test]
    fn test_p_transition_legal() {
        let rules = CounterpointRules::new();
        let c = Triad::new_unchecked(0, Quality::Major);
        let cm = Triad::new_unchecked(0, Quality::Minor);
        assert!(rules.check(c, cm).is_ok());
    }

    #[test]
    fn test_legal_plr_step() {
        let rules = CounterpointRules::new();
        let c = Triad::new_unchecked(0, Quality::Major);
        let cm = Triad::new_unchecked(0, Quality::Minor);
        let result = rules.legal_plr_step(c, cm);
        assert!(result.is_some());
    }

    #[test]
    fn test_legal_path() {
        let rules = CounterpointRules::new();
        let c = Triad::new_unchecked(0, Quality::Major);
        let am = Triad::new_unchecked(9, Quality::Minor);
        let path = rules.legal_path(c, am, 10);
        // Should reach Am in a few steps
        if let Some(&(_, last)) = path.last() {
            assert_eq!(last, am);
        }
    }
}
