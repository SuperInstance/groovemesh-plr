//! # groovemesh-plr
//!
//! PLR group algebra engine for collaborative counterpoint.
//!
//! The PLR group {P, L, R} generates 24 transformations acting on the 24 major/minor triads,
//! isomorphic to the dihedral group D₁₂.
//!
//! - **P** = Parallel (relative major/minor, same root)
//! - **L** = Leading-tone exchange (mediant transformation)
//! - **R** = Relative (relative major/minor)
//!
//! Every PLR transformation produces a chord that shares at least one common tone
//! with the previous chord. You can never play a wrong note.
//!
//! # Quick Start
//!
//! ```
//! use groovemesh_plr::{Triad, Quality, apply_p, apply_l, apply_r, PLR};
//!
//! let c = Triad::new_unchecked(0, Quality::Major);
//!
//! // P: parallel — same root, flip quality
//! assert_eq!(apply_p(c), Triad::new_unchecked(0, Quality::Minor));
//!
//! // L: leading-tone exchange — C → Em
//! assert_eq!(apply_l(c), Triad::new_unchecked(4, Quality::Minor));
//!
//! // R: relative — C → Am
//! assert_eq!(apply_r(c), Triad::new_unchecked(9, Quality::Minor));
//!
//! // P² = identity (involution property)
//! assert_eq!(groovemesh_plr::apply(PLR::P, apply_p(c)), c);
//! ```

pub mod chord;
pub mod counterpoint;
pub mod error;
pub mod group;
pub mod lattice;
pub mod nearest;
pub mod transform;
pub mod voice;

pub use chord::{PitchClass, Quality, Triad};
pub use counterpoint::CounterpointRules;
pub use error::PlrError;
pub use group::{find_word, orbit, suborbit, verify_group_axioms};
pub use lattice::Lattice;
pub use nearest::nearest_triad;
pub use transform::{apply, apply_l, apply_p, apply_r, PLR, PLRWord};
pub use voice::{VoiceLeading, minimal_voice_leading};
