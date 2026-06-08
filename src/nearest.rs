use crate::chord::{PitchClass, Triad};
use crate::error::PlrError;
use crate::lattice::Lattice;

/// Given an arbitrary set of pitch classes, find the closest triad.
/// "Closest" = minimal voice-leading distance to a triad that contains
/// as many of the given pitch classes as possible.
pub fn nearest_triad(pitches: &[PitchClass]) -> Result<Triad, PlrError> {
    if pitches.is_empty() {
        return Err(PlrError::NoValidTriad);
    }

    // Score each triad:
    // 1. Number of matching pitch classes (more is better)
    // 2. Voice-leading distance to the closest matching triad (less is better)
    let mut best: Option<(Triad, usize, u32)> = None;

    for t in Triad::all() {
        let tpcs = t.pitch_classes();
        let matches = pitches.iter().filter(|&&p| tpcs.contains(&p)).count();

        // Compute minimal distance: for each input pitch, distance to nearest triad pitch
        let mut total_dist: u32 = 0;
        for &p in pitches {
            let min_d = tpcs
                .iter()
                .map(|&tp| {
                    let diff = (p as i32 - tp as i32).abs();
                    std::cmp::min(diff, 12 - diff) as u32
                })
                .min()
                .unwrap_or(6);
            total_dist += min_d;
        }

        match &best {
            None => best = Some((t, matches, total_dist)),
            Some((_, best_matches, best_dist)) => {
                // Prefer more matches, then less distance
                if matches > *best_matches
                    || (matches == *best_matches && total_dist < *best_dist)
                {
                    best = Some((t, matches, total_dist));
                }
            }
        }
    }

    best.map(|(t, _, _)| t).ok_or(PlrError::NoValidTriad)
}

/// Given a current triad and a set of pitch classes, find the nearest legal triad
/// that is reachable through the PLR lattice.
pub fn nearest_plr_triad(current: Triad, pitches: &[PitchClass]) -> Triad {
    let ideal = nearest_triad(pitches);
    match ideal {
        Ok(target) => {
            // Find the closest PLR neighbor to current that's close to target
            let lattice = Lattice::build();
            let path = lattice.shortest_path(current, target);
            if let Some(steps) = path {
                if steps.is_empty() {
                    return current;
                }
                // Take one step toward the target
                return crate::transform::apply(steps[0], current);
            }
            current
        }
        Err(_) => current,
    }
}

/// Find the nearest triad (by voice leading) to the given pitch set,
/// considering all 24 triads.
pub fn nearest_by_pitches(pitches: &[PitchClass]) -> Result<Triad, PlrError> {
    nearest_triad(pitches)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Quality;

    #[test]
    fn test_nearest_triad_c_major() {
        // C, E, G → C major
        let result = nearest_triad(&[0, 4, 7]).unwrap();
        assert_eq!(result, Triad::new_unchecked(0, Quality::Major));
    }

    #[test]
    fn test_nearest_triad_a_minor() {
        // A, C, E → A minor
        let result = nearest_triad(&[9, 0, 4]).unwrap();
        assert_eq!(result, Triad::new_unchecked(9, Quality::Minor));
    }

    #[test]
    fn test_nearest_triad_partial() {
        // Just C, G → should find C major (closest match)
        let result = nearest_triad(&[0, 7]).unwrap();
        assert_eq!(result, Triad::new_unchecked(0, Quality::Major));
    }

    #[test]
    fn test_nearest_triad_single_pitch() {
        let result = nearest_triad(&[0]).unwrap();
        // Should be a C triad (major or minor)
        assert!(result.root == 0);
    }

    #[test]
    fn test_nearest_empty() {
        assert!(nearest_triad(&[]).is_err());
    }

    #[test]
    fn test_nearest_plr_triad() {
        let c = Triad::new_unchecked(0, Quality::Major);
        // Pitches of Am: should move toward Am
        let result = nearest_plr_triad(c, &[9, 0, 4]);
        // R(C) = Am
        assert_eq!(result, Triad::new_unchecked(9, Quality::Minor));
    }
}
