# Contributing to groovemesh-plr

Thank you for your interest in PLR group voice-leading algebra!

## Getting Started

```bash
git clone https://github.com/SuperInstance/groovemesh-plr.git
cd groovemesh-plr
cargo test
```

## Architecture Decisions

### Why only major and minor triads?

The PLR group is defined on the 24 major/minor triads (12 roots × 2 qualities). This is the setting where the group has the richest structure — it's isomorphic to D₁₂, acts transitively, and every transformation preserves common tones. Seventh chords, suspended chords, etc. would require a different algebraic framework.

### Why is the lattice always 24 nodes?

There are exactly 24 major/minor triads in 12-tone equal temperament. The PLR group acts transitively on all of them, so the orbit of any single triad is the entire set. The lattice has 24 nodes with 3 edges each (one per generator P, L, R), giving 72 directed edges.

### Why BFS for shortest paths?

The lattice has only 24 nodes. BFS is optimal for unweighted shortest path and completes in microseconds. There's no benefit to more sophisticated algorithms.

### Why involutions?

P, L, and R are each self-inverse: P² = L² = R² = identity. This means:
- The inverse of a PLR word is just the reverse sequence
- Adjacent identical ops cancel (reduction)
- The group has a very clean algebraic structure

## How to Add New Features

### Adding a new transformation
1. Define the function in `transform.rs`
2. Add it as a variant in `PLR` enum (or create a new enum for extended transformations)
3. Update `apply()`, `Display`, and all match arms
4. Add tests — verify involution, transitivity, common tones

### Adding a new counterpoint rule
1. Add the rule as a field in `CounterpointRules`
2. Implement the check in `CounterpointRules::check()`
3. Add a test with known good/bad progressions

### Adding a new voice-leading metric
1. Add the function to `voice.rs`
2. Consider how it interacts with `minimal_voice_leading`
3. Add tests comparing with known voice-leading distances

## Testing

```bash
cargo test                      # All tests
cargo test test_plr             # PLR-specific tests
cargo test test_lattice         # Lattice tests
cargo test test_counterpoint    # Counterpoint tests
```

The test suite verifies:
- All 24 triads are reachable (transitivity)
- P² = L² = R² = identity (involution)
- Voice-leading distances are symmetric
- Counterpoint rules catch parallel 5ths and octaves
- Nearest-triad finds correct results
- BFS shortest paths are correct

## Code Style

- `cargo fmt` — no debate
- `cargo clippy` — warnings are errors in CI
- Doc comments on all `pub` items

## Commit Messages

Follow [Conventional Commits](https://www.conventionalcommits.org/):
- `feat:` new features
- `fix:` bug fixes
- `docs:` documentation changes

## License

By contributing, you agree that your contributions will be licensed under the MIT License.
