use serde::{Deserialize, Serialize};

use crate::chord::{Quality, Triad};

/// PLR operation
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum PLR {
    P,
    L,
    R,
}

impl std::fmt::Display for PLR {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PLR::P => write!(f, "P"),
            PLR::L => write!(f, "L"),
            PLR::R => write!(f, "R"),
        }
    }
}

/// Apply the Parallel transformation: same root, flip quality.
/// P(C) = Cm, P(Am) = A
pub fn apply_p(t: Triad) -> Triad {
    Triad::new_unchecked(
        t.root,
        match t.quality {
            Quality::Major => Quality::Minor,
            Quality::Minor => Quality::Major,
        },
    )
}

/// Apply the Leading-tone exchange transformation.
/// L(C) = Em — the third of C major becomes the root of E minor
/// L(Cm) = Ab — the fifth of C minor moves up by semitone
pub fn apply_l(t: Triad) -> Triad {
    match t.quality {
        Quality::Major => {
            // Major: new root = old third (root+4), quality = Minor
            // L(C) = Em: root of Em = E = 0+4 = 4
            Triad::new_unchecked((t.root + 4) % 12, Quality::Minor)
        }
        Quality::Minor => {
            // Minor: new root = old fifth + 1 (root+8), quality = Major
            // L(Cm) = Ab: root of Ab = 0+8 = 8
            Triad::new_unchecked((t.root + 8) % 12, Quality::Major)
        }
    }
}

/// Apply the Relative transformation.
/// R(C) = Am — relative minor of major / relative major of minor
pub fn apply_r(t: Triad) -> Triad {
    match t.quality {
        Quality::Major => {
            // Down a minor third
            Triad::new_unchecked((t.root + 9) % 12, Quality::Minor)
        }
        Quality::Minor => {
            // Up a minor third
            Triad::new_unchecked((t.root + 3) % 12, Quality::Major)
        }
    }
}

/// Apply a single PLR operation to a triad.
pub fn apply(op: PLR, t: Triad) -> Triad {
    match op {
        PLR::P => apply_p(t),
        PLR::L => apply_l(t),
        PLR::R => apply_r(t),
    }
}

/// A word in the PLR group (sequence of P, L, R operations).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PLRWord {
    pub ops: Vec<PLR>,
}

impl PLRWord {
    /// Create a new empty PLR word (identity).
    pub fn new() -> Self {
        Self { ops: Vec::new() }
    }

    /// Create from a slice of operations.
    pub fn from_ops(ops: &[PLR]) -> Self {
        Self {
            ops: ops.to_vec(),
        }
    }

    /// Append an operation.
    pub fn push(&mut self, op: PLR) {
        self.ops.push(op);
    }

    /// Apply this word to a triad.
    pub fn apply(&self, t: Triad) -> Triad {
        self.ops.iter().fold(t, |acc, &op| apply(op, acc))
    }

    /// Compose two words: self followed by other.
    pub fn compose(&self, other: &PLRWord) -> PLRWord {
        let mut ops = self.ops.clone();
        ops.extend_from_slice(&other.ops);
        PLRWord { ops }
    }

    /// The inverse word (reverse order, each op is self-inverse since P²=L²=R²=identity).
    pub fn inverse(&self) -> PLRWord {
        PLRWord {
            ops: self.ops.iter().rev().copied().collect(),
        }
    }

    /// Reduce the word by canceling adjacent inverses.
    /// Since each PLR op is its own inverse, cancel adjacent identical ops.
    pub fn reduce(&self) -> PLRWord {
        let mut reduced: Vec<PLR> = Vec::new();
        for &op in &self.ops {
            if reduced.last() == Some(&op) {
                reduced.pop();
            } else {
                reduced.push(op);
            }
        }
        PLRWord { ops: reduced }
    }

    /// Length of the word.
    pub fn len(&self) -> usize {
        self.ops.len()
    }

    /// Whether the word is empty (identity).
    pub fn is_empty(&self) -> bool {
        self.ops.is_empty()
    }
}

impl Default for PLRWord {
    fn default() -> Self {
        Self::new()
    }
}

impl std::fmt::Display for PLRWord {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.ops.is_empty() {
            write!(f, "ε")?;
        } else {
            for op in &self.ops {
                write!(f, "{}", op)?;
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_p_parallel() {
        let c = Triad::new_unchecked(0, Quality::Major);
        let cm = Triad::new_unchecked(0, Quality::Minor);
        assert_eq!(apply_p(c), cm);
        assert_eq!(apply_p(cm), c);
    }

    #[test]
    fn test_l_leading_tone() {
        let c = Triad::new_unchecked(0, Quality::Major);
        let em = Triad::new_unchecked(4, Quality::Minor);
        assert_eq!(apply_l(c), em);

        let cm = Triad::new_unchecked(0, Quality::Minor);
        // L(Cm) = Ab major
        let ab = Triad::new_unchecked(8, Quality::Major);
        assert_eq!(apply_l(cm), ab);
    }

    #[test]
    fn test_r_relative() {
        let c = Triad::new_unchecked(0, Quality::Major);
        let am = Triad::new_unchecked(9, Quality::Minor);
        assert_eq!(apply_r(c), am);

        let cm = Triad::new_unchecked(0, Quality::Minor);
        let eb = Triad::new_unchecked(3, Quality::Major);
        assert_eq!(apply_r(cm), eb);
    }

    #[test]
    fn test_involution() {
        // P² = L² = R² = identity
        for t in Triad::all() {
            assert_eq!(apply_p(apply_p(t)), t, "P² != identity for {}", t);
            assert_eq!(apply_l(apply_l(t)), t, "L² != identity for {}", t);
            assert_eq!(apply_r(apply_r(t)), t, "R² != identity for {}", t);
        }
    }

    #[test]
    fn test_plr_word_apply() {
        let c = Triad::new_unchecked(0, Quality::Major);
        let word = PLRWord::from_ops(&[PLR::R]);
        assert_eq!(word.apply(c), Triad::new_unchecked(9, Quality::Minor));
    }

    #[test]
    fn test_plr_word_compose() {
        let w1 = PLRWord::from_ops(&[PLR::P]);
        let w2 = PLRWord::from_ops(&[PLR::L]);
        let composed = w1.compose(&w2);
        assert_eq!(composed.ops, vec![PLR::P, PLR::L]);
    }

    #[test]
    fn test_plr_word_inverse() {
        let word = PLRWord::from_ops(&[PLR::P, PLR::L, PLR::R]);
        let inv = word.inverse();
        assert_eq!(inv.ops, vec![PLR::R, PLR::L, PLR::P]);
    }

    #[test]
    fn test_plr_word_reduce() {
        let word = PLRWord::from_ops(&[PLR::P, PLR::L, PLR::P, PLR::P]);
        let reduced = word.reduce();
        // PP cancels, leaving PL
        assert_eq!(reduced.ops, vec![PLR::P, PLR::L]);
    }

    #[test]
    fn test_pp_is_identity() {
        let c = Triad::new_unchecked(0, Quality::Major);
        let pp = PLRWord::from_ops(&[PLR::P, PLR::P]);
        assert_eq!(pp.apply(c), c);
    }

    #[test]
    fn test_lr_on_c() {
        // L(R(C)) = L(Am)
        let c = Triad::new_unchecked(0, Quality::Major);
        let am = apply_r(c);
        let result = apply_l(am);
        // L(Am) = C major (0+8=8... wait: L(Am): root=9, 9+8=17%12=5=F)
        // Actually L(Am) = F major
        assert_eq!(result, Triad::new_unchecked(5, Quality::Major));
    }
}
