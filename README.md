# groovemesh-plr

**PLR group algebra for collaborative counterpoint — you can never play a wrong note.**

## The Problem

In music theory, the space of chord progressions is vast. Most of it sounds bad. For centuries, composers have navigated this space by ear, by rules of thumb ("avoid parallel fifths"), and by studying counterpoint. But there's a mathematical structure underlying "good" voice leading, and it has a name: the **PLR group**.

If you're building generative music systems, collaborative music tools, or AI composition assistants, you need to *know* that every chord transition you produce is musically valid. You don't want to filter bad progressions — you want a space where bad progressions *can't exist*.

## The Key Insight

The PLR group (Parallel / Leading-tone exchange / Relative) is a group of 3 involutions that acts transitively on the 24 major and minor triads. It's isomorphic to the **dihedral group D₁₂** — the symmetry group of a regular 12-gon. The profound fact is this:

> **Every PLR transformation produces a chord that shares at least one common tone with the previous chord.**

This means voice leading is automatically smooth. You physically cannot "leap" to a distant chord — every step moves at most a few semitones per voice. The PLR lattice is a graph where every edge is a *valid, smooth voice leading*.

This crate implements the full PLR algebra:
- The 3 generators: P, L, R
- The 24-element group action on triads (root + quality pairs)
- The PLR lattice (Cayley graph) with BFS shortest paths
- Voice-leading distance computation
- Species counterpoint rules checking
- **Nearest-triad finding** — give it arbitrary pitches, get the closest valid triad

## Architecture

```
                          ┌──────────────┐
                          │   Triad      │
                          │ (root+quality)│
                          └──────┬───────┘
                                 │
                 ┌───────────────┼───────────────┐
                 │               │               │
          ┌──────▼──────┐ ┌─────▼──────┐ ┌──────▼──────┐
          │   P(allel)  │ │  L(eading) │ │  R(elative) │
          │  flip quality│ │  mediant   │ │ rel. major/ │
          │  same root  │ │  exchange  │ │   minor     │
          └──────┬──────┘ └─────┬──────┘ └──────┬──────┘
                 │               │               │
                 └───────────────┼───────────────┘
                                 │
                    ┌────────────▼────────────┐
                    │      PLR Word           │
                    │ (sequence of P, L, R)   │
                    │ with compose, inverse,  │
                    │ reduce                  │
                    └────────────┬────────────┘
                                 │
                    ┌────────────▼────────────┐
                    │     PLR Lattice         │
                    │ (24 nodes, 3 edges each)│
                    │ BFS shortest path,      │
                    │ distance, at_distance   │
                    └────────────┬────────────┘
                                 │
              ┌──────────────────┼──────────────────┐
              │                  │                  │
     ┌────────▼────────┐ ┌──────▼───────┐ ┌───────▼────────┐
     │  Voice Leading  │ │ Counterpoint │ │   Nearest      │
     │  (minimal       │ │ Rules        │ │   Triad        │
     │   semitone      │ │ (no parallel │ │ (find closest  │
     │   distance)     │ │  5ths/octs)  │ │  valid triad)  │
     └─────────────────┘ └──────────────┘ └────────────────┘
```

### Module Overview

| Module | Purpose |
|--------|---------|
| `chord` | Triad (root + quality), pitch classes, note names |
| `transform` | P, L, R transformations, PLR words (sequences) |
| `group` | Orbit computation, Cayley table, word finding, axiom verification |
| `lattice` | PLR lattice graph with BFS shortest path |
| `voice` | Voice-leading distance computation |
| `counterpoint` | Species counterpoint rules (no parallel 5ths/octaves) |
| `nearest` | Find the closest valid triad to arbitrary pitches |

## The Math: PLR Transformations

Given a triad T = (root, quality), the three transformations are:

### P — Parallel
Same root, flip quality. P(C) = Cm, P(Am) = A.
```
C  major → C  minor    (root stays, third drops by semitone)
G  minor → G  major    (root stays, third rises by semitone)
```

### L — Leading-tone Exchange
Major: the third becomes the new root of a minor triad. L(C) = Em.
Minor: the fifth moves up a semitone to become the root of a major triad. L(Cm) = A♭.
```
L(C)  = Em    (E = third of C = root + 4)
L(Cm) = Ab    (Ab = fifth of Cm + 1 semitone)
```

### R — Relative
Relative major/minor, moving by a minor third.
```
R(C)  = Am    (down a minor third)
R(Am) = C     (up a minor third)
```

### Key Properties

- **Involution**: P² = L² = R² = identity (each is its own inverse)
- **Transitivity**: starting from any triad, you can reach all 24 triads
- **Isomorphism**: the PLR group ≅ D₁₂ (dihedral group of order 24)
- **Common tones**: every PLR edge shares at least one pitch class

## Quick Start

```rust
use groovemesh_plr::{Triad, Quality, apply_p, apply_l, apply_r, PLR, apply};

// Create triads
let c_major = Triad::new_unchecked(0, Quality::Major);
let a_minor = Triad::new_unchecked(9, Quality::Minor);

// Apply transformations
assert_eq!(apply_p(c_major), Triad::new_unchecked(0, Quality::Minor)); // C → Cm
assert_eq!(apply_l(c_major), Triad::new_unchecked(4, Quality::Minor)); // C → Em
assert_eq!(apply_r(c_major), Triad::new_unchecked(9, Quality::Minor)); // C → Am

// P² = identity (involution)
assert_eq!(apply(PLR::P, apply(PLR::P, c_major)), c_major);
```

## Finding Paths Between Chords

```rust
use groovemesh_plr::{Triad, Quality, Lattice, find_word};

let c = Triad::new_unchecked(0, Quality::Major);
let fs = Triad::from_name("F#m").unwrap();

// Find the shortest PLR path
let lattice = Lattice::build();
let path = lattice.shortest_path(c, fs).unwrap();
println!("C → F#m: {} steps: {:?}", path.len(), path);

// Or find the PLR word (sequence of operations)
let word = find_word(c, fs).unwrap();
println!("Word: {} (applied: {})", word, word.apply(c));
```

## Voice Leading

```rust
use groovemesh_plr::{Triad, Quality, minimal_voice_leading, voice_leading_distance};

let c = Triad::new_unchecked(0, Quality::Major);
let cm = Triad::new_unchecked(0, Quality::Minor);

let vl = minimal_voice_leading(c, cm);
println!("C → Cm: distance = {} semitones", vl.distance);
println!("Movements: {:?}", vl.movements);
```

## Counterpoint Rules

```rust
use groovemesh_plr::{Triad, Quality, CounterpointRules};

let rules = CounterpointRules::new();
let c = Triad::new_unchecked(0, Quality::Major);
let cm = Triad::new_unchecked(0, Quality::Minor);

// Check if a transition is legal
assert!(rules.check(c, cm).is_ok()); // P transition: always legal

// Find a legal PLR path from C to Am
let am = Triad::new_unchecked(9, Quality::Minor);
let path = rules.legal_path(c, am, 10);
println!("Legal path C → Am: {} steps", path.len());
```

## Nearest Triad from Arbitrary Pitches

```rust
use groovemesh_plr::{nearest_triad, nearest_plr_triad, Triad, Quality};

// Someone plays C, E, G → that's C major
let triad = nearest_triad(&[0, 4, 7]).unwrap();
assert_eq!(triad, Triad::new_unchecked(0, Quality::Major));

// Someone plays C, F# — find the closest triad
let ambiguous = nearest_triad(&[0, 6]).unwrap();
println!("Closest triad to C+F#: {}", ambiguous);

// From a current triad, take one PLR step toward a pitch set
let c = Triad::new_unchecked(0, Quality::Major);
let next = nearest_plr_triad(c, &[9, 0, 4]); // Am pitches
assert_eq!(next, Triad::new_unchecked(9, Quality::Minor)); // R(C) = Am
```

## Group Theory

```rust
use groovemesh_plr::{orbit, verify_group_axioms, cayley_table, Triad, Quality};

// The orbit of any triad is all 24 triads (transitivity)
let c = Triad::new_unchecked(0, Quality::Major);
let all = orbit(c);
assert_eq!(all.len(), 24);

// Verify group axioms
assert!(verify_group_axioms());

// Full Cayley table
let table = cayley_table();
println!("Cayley table has {} entries", table.len());
```

## Performance

- **Triad operations**: O(1) — arithmetic on pitch classes
- **Voice leading**: O(1) — tries 6 permutations of 3 voices
- **BFS shortest path**: O(24) — the lattice has only 24 nodes
- **Nearest triad**: O(24) — scores all triads
- **Group operations**: O(24) for orbits, O(24) for Cayley table

The entire PLR lattice fits in constant space. There are only 24 triads and 72 edges (3 per triad). This means every operation is effectively O(1).

## Comparison

| Feature | groovemesh-plr | Music21 (Python) | Traditional theory |
|---------|---------------|-------------------|-------------------|
| PLR group | ✅ Full implementation | Partial | Manual |
| Voice leading | ✅ Minimal distance | Yes | Approximate |
| Counterpoint rules | ✅ Configurable | Yes | Manual |
| Nearest triad | ✅ From arbitrary pitches | No | N/A |
| Lattice graph | ✅ BFS shortest path | No | N/A |
| Group verification | ✅ Automated | No | N/A |
| Language | Rust (zero-cost) | Python | N/A |

## SuperInstance Ecosystem

`groovemesh-plr` provides voice-leading algebra for:
- `spreadsheet-engine` — MIDI cells can use PLR for smooth harmonic transitions
- `lotka-beats` — species compete within PLR-legal harmonic spaces
- `tropical-synth` — timbre morphing along PLR lattice edges

## License

MIT
