use crate::chord::Triad;
use crate::lattice::Lattice;
use crate::transform::apply;

/// Voice leading between two triads
#[derive(Debug, Clone)]
pub struct VoiceLeading {
    pub from: Triad,
    pub to: Triad,
    /// Pitch movement for each voice (4 voices: soprano, alto, tenor, bass)
    pub movements: [i8; 4],
    /// Total voice-leading distance (sum of absolute semitone movements)
    pub distance: u32,
}

/// Compute the minimal voice leading between two triads.
/// Places triads in close position and computes minimal movements.
pub fn minimal_voice_leading(from: Triad, to: Triad) -> VoiceLeading {
    let from_pcs = from.pitch_classes();
    let to_pcs = to.pitch_classes();

    // Try different octave placements for minimal total movement
    let mut best_movements = [0i8; 4];
    let mut best_distance = u32::MAX;

    // Bass voice: root in a low octave, find closest root
    let from_root = from.root as i8;
    let to_root = to.root as i8;
    let bass_move = minimal_semitone_move(from_root, to_root);

    // For the three upper voices, find the minimal matching
    // We try all 6 permutations of the three upper voices
    let perms = [
        [0, 1, 2],
        [0, 2, 1],
        [1, 0, 2],
        [1, 2, 0],
        [2, 0, 1],
        [2, 1, 0],
    ];

    for perm in perms {
        let mut movements = [0i8; 4];
        movements[3] = bass_move;

        let mut total = bass_move.unsigned_abs() as u32;

        for (i, &j) in perm.iter().enumerate() {
            let from_pc = from_pcs[i] as i8;
            let to_pc = to_pcs[j] as i8;
            let m = minimal_semitone_move(from_pc, to_pc);
            movements[i] = m;
            total += m.unsigned_abs() as u32;
        }

        if total < best_distance {
            best_distance = total;
            best_movements = movements;
        }
    }

    VoiceLeading {
        from,
        to,
        movements: best_movements,
        distance: best_distance,
    }
}

/// Minimal semitone movement between two pitch classes (taking the shorter path around the circle).
fn minimal_semitone_move(from: i8, to: i8) -> i8 {
    let diff = to - from;
    // Choose the shorter direction around the 12-tone circle
    if diff > 6 {
        diff - 12
    } else if diff < -6 {
        diff + 12
    } else {
        diff
    }
}

/// Voice-leading distance between two triads.
pub fn voice_leading_distance(from: Triad, to: Triad) -> u32 {
    minimal_voice_leading(from, to).distance
}

/// Find the triad with minimal voice-leading distance from `from` among the candidates.
pub fn nearest_by_voice_leading(from: Triad, candidates: &[Triad]) -> Option<Triad> {
    candidates
        .iter()
        .min_by_key(|&&t| voice_leading_distance(from, t))
        .copied()
}

/// Find the nearest triad (by voice leading) reachable by a single PLR step.
pub fn nearest_plr_neighbor(from: Triad, target: Triad) -> (crate::transform::PLR, Triad, u32) {
    use crate::transform::PLR;
    let _lattice = Lattice::build();
    let mut best_op = PLR::P;
    let mut best_triad = apply(PLR::P, from);
    let mut best_dist = voice_leading_distance(apply(PLR::P, from), target);

    for &op in &[PLR::P, PLR::L, PLR::R] {
        let neighbor = apply(op, from);
        let dist = voice_leading_distance(neighbor, target);
        if dist < best_dist {
            best_op = op;
            best_triad = neighbor;
            best_dist = dist;
        }
    }

    (best_op, best_triad, best_dist)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Quality;

    #[test]
    fn test_voice_leading_self() {
        let c = Triad::new_unchecked(0, Quality::Major);
        let vl = minimal_voice_leading(c, c);
        assert_eq!(vl.distance, 0);
    }

    #[test]
    fn test_voice_leading_p() {
        let c = Triad::new_unchecked(0, Quality::Major);
        let cm = Triad::new_unchecked(0, Quality::Minor);
        let vl = minimal_voice_leading(c, cm);
        // C major → C minor: root stays, third moves down 1 semitone, fifth stays
        assert!(vl.distance <= 2); // At most 1 semitone for the third
    }

    #[test]
    fn test_voice_leading_distance_symmetric() {
        let c = Triad::new_unchecked(0, Quality::Major);
        let fs = Triad::new_unchecked(6, Quality::Major);
        assert_eq!(
            voice_leading_distance(c, fs),
            voice_leading_distance(fs, c)
        );
    }

    #[test]
    fn test_minimal_semitone_move() {
        assert_eq!(minimal_semitone_move(0, 1), 1); // C → C#
        assert_eq!(minimal_semitone_move(0, 11), -1); // C → B (shorter going down)
        assert_eq!(minimal_semitone_move(0, 6), 6); // C → F# (either direction is 6)
        assert_eq!(minimal_semitone_move(0, 7), -5); // C → G (shorter going up: 5 vs 7)
    }
}
