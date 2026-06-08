# groovemesh-plr

[![crates.io](https://img.shields.io/crates/v/groovemesh-plr.svg)](https://crates.io/crates/groovemesh-plr)
[![docs.rs](https://docs.rs/groovemesh-plr/badge.svg)](https://docs.rs/groovemesh-plr)
[![license: MIT](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

**PLR group algebra engine for collaborative counterpoint.**

The PLR group — **{P, L, R}** — generates 24 transformations acting on the 24
major and minor triads, isomorphic to the **dihedral group D₁₂**:

- **P** (Parallel) — switches between relative major and minor (same root)
- **L** (Leading-tone exchange) — mediant transformation
- **R** (Relative) — switches between relative major and minor keys

Every PLR sequence produces a valid chord progression. You can never play a wrong
note — the algebra guarantees smooth voice leading by construction.

## Features

- **Triad algebra** — `PitchClass`, `Quality` (Major/Minor), and `Triad`
  representations for all 24 major/minor triads
- **PLR transformations** — `apply_p()`, `apply_l()`, `apply_r()` and arbitrary
  `PLRWord` composition (`PLR::PLR`, `PLR::LRP`, etc.)
- **Group-theoretic tools** — `orbit()`, `suborbit()`, `find_word()`, and
  `verify_group_axioms()` for exploring the D₁₂ structure
- **Chicken-wire lattice** — `Lattice` for visualizing and navigating the
  Tonnetz (chord neighborhood graph)
- **Counterpoint validation** — `CounterpointRules` checks for parallel fifths,
  octaves, and other classical voice-leading constraints
- **Voice leading** — `VoiceLeading` computes optimal part assignments between
  successive chords
- **Nearest-triad lookup** — find the closest triad in PLR-space from an
  arbitrary pitch-class set

## Quick Start

```rust
use groovemesh_plr::{Triad, PitchClass, Quality, apply_r, apply_l, apply_p};

// Start with C major
let c_major = Triad::new(PitchClass::C, Quality::Major);

// Apply R (Relative): C major → A minor
let a_minor = apply_r(&c_major);

// Apply L (Leading-tone): A minor → F major
let f_major = apply_l(&a_minor);

// Apply P (Parallel): F major → F minor
let f_minor = apply_p(&f_major);

println!("C major → {} → {} → {}", a_minor, f_major, f_minor);
```

## Exploring the Group

```rust
use groovemesh_plr::{Triad, PitchClass, Quality, orbit, find_word, PLR};

let start = Triad::new(PitchClass::C, Quality::Major);

// Full orbit under all PLR words (24 triads)
let all = orbit(&start);
assert_eq!(all.len(), 24);

// Find the PLR word connecting two triads
let target = Triad::new(PitchClass::E, Quality::Minor);
let word = find_word(&start, &target).unwrap();
```

## Module Overview

| Module | Description |
|---|---|
| `chord` | `Triad`, `PitchClass`, `Quality` — triad representation |
| `transform` | `apply_p/l/r`, `PLRWord`, `PLR` — transformation engine |
| `group` | `orbit`, `suborbit`, `find_word`, `verify_group_axioms` |
| `lattice` | `Lattice` — Tonnetz / chicken-wire graph |
| `voice` | `VoiceLeading` — optimal part assignment |
| `counterpoint` | `CounterpointRules` — classical constraint checking |
| `nearest` | `nearest_triad` — nearest-triad lookup |
| `error` | Error types |

## Links

- [Documentation](https://docs.rs/groovemesh-plr)
- [Repository](https://github.com/nightshift-crates/groovemesh-plr)
- [Crates.io](https://crates.io/crates/groovemesh-plr)

## License

MIT
