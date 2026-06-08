# Contributing to groovemesh-plr

Thank you for your interest in contributing!

## Building

```bash
cargo build
```

## Testing

```bash
cargo test
```

## Running Examples

```bash
cargo run --example basic
```

## Code Quality

Before submitting a PR:

```bash
cargo fmt -- --check
cargo clippy -- -D warnings
cargo test
```

## Submitting Changes

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/my-feature`)
3. Make your changes with clear commit messages
4. Ensure CI passes (fmt, clippy, test)
5. Open a pull request against `main`

## Music Theory Notes

The PLR group is isomorphic to the dihedral group D₁₂. Each generator (P, L, R) is an involution:
- **P** (Parallel): flips quality at the same root — C major ↔ C minor
- **L** (Leading-tone): mediates between relative triads — C major ↔ E minor
- **R** (Relative): maps to relative major/minor — C major ↔ A minor

The group acts transitively on the 24 major/minor triads, meaning any triad can reach any other through a sequence of P, L, R operations.
