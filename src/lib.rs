//! # groovemesh-plr
//!
//! PLR group algebra engine for collaborative counterpoint.
//!
//! The PLR group {P, L, R} generates 24 transformations acting on the 24 major/minor triads,
//! isomorphic to the dihedral group D12.
//!
//! - **P** = Parallel (relative major/minor, same root)
//! - **L** = Leading-tone exchange (mediant transformation)
//! - **R** = Relative (relative major/minor)
//!
//! You can never play a wrong note.

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
pub use voice::VoiceLeading;
