# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.1.0] - 2026-06-08

### Added
- PLR group operations (P, L, R) acting on 24 major/minor triads
- `Triad` with parsing from chord names ("C", "Am", "F#m")
- Full PLR lattice as a graph with BFS shortest path
- `PLRWord` for composing, reducing, and inverting PLR sequences
- `CounterpointRules` for species counterpoint validation
- Voice leading distance computation with minimal movement
- Nearest-triad finder from arbitrary pitch class sets
- Group-theoretic utilities: orbit, Cayley table, axiom verification
- Serde support for triads and PLR operations
