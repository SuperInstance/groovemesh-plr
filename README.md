# groovemesh-plr

[![crates.io](https://img.shields.io/crates/v/groovemesh-plr.svg)](https://crates.io/crates/groovemesh-plr)
[![docs.rs](https://docs.rs/groovemesh-plr/badge.svg)](https://docs.rs/groovemesh-plr)
[![license: MIT](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

## The Problem

Harmony is a combinatorial explosion. 12 pitch classes × 2 qualities (major/minor) = 24 triads. From any triad, you can move to any of the other 23. Most of those moves sound terrible. How do you navigate?

Music theory gives rules ("avoid parallel fifths", "resolve the leading tone") but these are negative constraints — they tell you what *not* to do. They don't give you a positive generative system.

## The Insight

There are exactly three transformations that act on major and minor triads and preserve smooth voice leading:

| Transform | Name | What it does | Voice leading |
|---|---|---|---|
| **P** | Parallel | Swap major ↔ minor on the same root | 1 semitone moves |
| **L** | Leading-tone exchange | Swap the third ↔ root between relative triads | 1 semitone moves |
| **R** | Relative | Move to the relative major/minor | 1 semitone moves |

These three operations generate a group — the **PLR group** — isomorphic to the dihedral group D₁₂ (the symmetries of a regular 12-gon). Every element of D₁₂ acts on triads, and every word in {P, L, R} produces a valid chord change with minimal voice leading.

This means: **any sequence of P, L, R operations produces a musically valid chord progression.** You literally cannot play a wrong note. The algebra guarantees it.

## The Math

The 24 triads (12 major + 12 minor) form a set that D₁₂ acts on transitively. Starting from any triad, you can reach any other triad via a PLR word. The group has order 24, matching the 24 triads — this isn't coincidence, it's because the action is *simply transitive*. Every triad has a unique PLR address.

The **Tonnetz** (chicken-wire lattice) visualizes this: a 2D grid where horizontal neighbors are connected by P, vertical neighbors by L, and diagonal neighbors by R. Navigation on the lattice IS harmonic progression.

## Using The Library

```rust
use groovemesh_plr::{Triad, PitchClass, Quality, apply_p, apply_l, apply_r};

// Start with C major
let c_major = Triad::new(PitchClass::C, Quality::Major);

// RLP: a classic ii-V-I disguised as group theory
let a_min = apply_r(&c_major);    // C major → A minor (relative)
let f_major = apply_l(&a_min);    // A minor → F major (leading tone)
let f_minor = apply_p(&f_major);  // F major → F minor (parallel)

println!("C → {} → {} → {}", a_min, f_major, f_minor);
// Output: C → Am → F → Fm
```

### Finding paths between any two triads

```rust
use groovemesh_plr::{Triad, PitchClass, Quality, find_word};

let start = Triad::new(PitchClass::C, Quality::Major);
let target = Triad::new(PitchClass::G, Quality::Minor);

// find_word returns the shortest PLR sequence connecting them
let word = find_word(&start, &target).unwrap();
println!("Path: {:?}", word); // e.g., PLR
```

### The full orbit

```rust
use groovemesh_plr::{Triad, PitchClass, Quality, orbit};

let start = Triad::new(PitchClass::C, Quality::Major);
let all_24 = orbit(&start);
assert_eq!(all_24.len(), 24); // reaches every triad exactly once
```

## Counterpoint Validation

The PLR group guarantees smooth voice leading, but classical counterpoint has additional constraints (no parallel fifths, no parallel octaves, etc.). The `CounterpointRules` module checks these:

```rust
use groovemesh_plr::{CounterpointRules, Triad, PitchClass, Quality};

let rules = CounterpointRules::strict();
let c = Triad::new(PitchClass::C, Quality::Major);
let g = Triad::new(PitchClass::G, Quality::Major);
let ok = rules.check_progression(&c, &g); // checks for parallels
```

## Voice Leading Computation

Given two triads, `VoiceLeading` finds the minimal-distance part assignment:

```rust
use groovemesh_plr::VoiceLeading;

let c_major = Triad::new(PitchClass::C, Quality::Major);
let a_minor = Triad::new(PitchClass::A, Quality::Minor);
let vl = VoiceLeading::between(&c_major, &a_minor);
println!("Total semitone distance: {}", vl.total_distance());
```

## The Tonnetz Lattice

```rust
use groovemesh_plr::Lattice;

let lattice = Lattice::standard();
let neighbors = lattice.neighbors(&Triad::new(PitchClass::C, Quality::Major));
// Returns all triads reachable by a single P, L, or R move
```

## Module Map

| Module | What it does |
|---|---|
| `chord` | `Triad`, `PitchClass`, `Quality` — the 24 triads |
| `transform` | `apply_p/l/r`, `PLRWord`, `PLR` — the D₁₂ generators |
| `group` | `orbit`, `suborbit`, `find_word`, `verify_group_axioms` — group-theoretic tools |
| `lattice` | `Lattice` — the Tonnetz (chicken-wire graph) |
| `voice` | `VoiceLeading` — minimal-distance part assignment |
| `counterpoint` | `CounterpointRules` — classical constraint checking |
| `nearest` | `nearest_triad` — find closest triad from arbitrary pitch classes |

## Design Decisions

- **Why only major/minor?** The PLR group is specifically about the 24 major/minor triads. Seventh chords and extended harmonies require larger groups (the UTT group, contextual transformations) — that's a future crate.
- **Why D₁₂ and not Tₙ/I?** Tₙ/I (transposition/inversion) is the standard music-theory group, but it doesn't preserve smooth voice leading. PLR does. The two groups are isomorphic, but PLR is the one that matters for harmonic navigation.
- **Why integer semitones?** Voice leading distance is measured in semitones because that's the smallest unit that distinguishes triads in 12-TET. If you want microtonal PLR, you'd change the PitchClass representation.

## Links

- [Documentation](https://docs.rs/groovemesh-plr)
- [Repository](https://github.com/SuperInstance/groovemesh-plr)
- [crates.io](https://crates.io/crates/groovemesh-plr)
- [Callender, Quinn, and Tymoczko (2008)](https://doi.org/10.1090/S0273-0979-08-01194-7) — the foundational paper on geometric music theory

## License

MIT
