use std::collections::{HashMap, HashSet, VecDeque};

use crate::chord::Triad;
use crate::transform::{apply, PLR};

/// Edge in the PLR lattice
#[derive(Debug, Clone)]
pub struct LatticeEdge {
    pub op: PLR,
    pub target: Triad,
}

/// The PLR lattice as a graph
#[derive(Debug, Clone)]
pub struct Lattice {
    /// Adjacency list: triad -> list of (operation, neighbor)
    adj: HashMap<Triad, Vec<LatticeEdge>>,
}

impl Lattice {
    /// Build the full PLR lattice (24 nodes, each with 3 edges).
    pub fn build() -> Self {
        let mut adj = HashMap::new();
        for t in Triad::all() {
            let edges: Vec<LatticeEdge> = [PLR::P, PLR::L, PLR::R]
                .iter()
                .map(|&op| LatticeEdge {
                    op,
                    target: apply(op, t),
                })
                .collect();
            adj.insert(t, edges);
        }
        Self { adj }
    }

    /// Get the neighbors of a triad.
    pub fn neighbors(&self, t: Triad) -> &[LatticeEdge] {
        self.adj.get(&t).map(|v| v.as_slice()).unwrap_or(&[])
    }

    /// Find the nearest neighbor of `t` in the lattice direction `op`.
    pub fn step(&self, t: Triad, op: PLR) -> Triad {
        apply(op, t)
    }

    /// BFS shortest path between two triads, returning the sequence of PLR operations.
    pub fn shortest_path(&self, from: Triad, to: Triad) -> Option<Vec<PLR>> {
        if from == to {
            return Some(vec![]);
        }
        let mut visited = HashSet::new();
        let mut queue = VecDeque::new();
        visited.insert(from);
        queue.push_back((from, Vec::new()));

        while let Some((current, path)) = queue.pop_front() {
            for &op in &[PLR::P, PLR::L, PLR::R] {
                let next = apply(op, current);
                let mut new_path = path.clone();
                new_path.push(op);
                if next == to {
                    return Some(new_path);
                }
                if visited.insert(next) {
                    queue.push_back((next, new_path));
                }
            }
        }
        None
    }

    /// Distance between two triads in the PLR lattice.
    pub fn distance(&self, from: Triad, to: Triad) -> usize {
        self.shortest_path(from, to).map(|p| p.len()).unwrap_or(usize::MAX)
    }

    /// All triads at a given distance from `start`.
    pub fn at_distance(&self, start: Triad, dist: usize) -> Vec<Triad> {
        let mut result = Vec::new();
        for t in Triad::all() {
            if self.distance(start, t) == dist {
                result.push(t);
            }
        }
        result
    }

    /// Number of nodes in the lattice (always 24).
    pub fn node_count(&self) -> usize {
        self.adj.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::chord::Quality;

    #[test]
    fn test_lattice_has_24_nodes() {
        let lattice = Lattice::build();
        assert_eq!(lattice.node_count(), 24);
    }

    #[test]
    fn test_lattice_neighbors_count() {
        let lattice = Lattice::build();
        let c = Triad::new_unchecked(0, Quality::Major);
        assert_eq!(lattice.neighbors(c).len(), 3);
    }

    #[test]
    fn test_lattice_shortest_path_self() {
        let lattice = Lattice::build();
        let c = Triad::new_unchecked(0, Quality::Major);
        let path = lattice.shortest_path(c, c).unwrap();
        assert!(path.is_empty());
    }

    #[test]
    fn test_lattice_shortest_path_p() {
        let lattice = Lattice::build();
        let c = Triad::new_unchecked(0, Quality::Major);
        let cm = Triad::new_unchecked(0, Quality::Minor);
        let path = lattice.shortest_path(c, cm).unwrap();
        assert_eq!(path.len(), 1);
        assert_eq!(path[0], PLR::P);
    }

    #[test]
    fn test_lattice_distance_symmetric() {
        let lattice = Lattice::build();
        let c = Triad::new_unchecked(0, Quality::Major);
        let fs = Triad::new_unchecked(6, Quality::Minor);
        assert_eq!(lattice.distance(c, fs), lattice.distance(fs, c));
    }

    #[test]
    fn test_lattice_at_distance_1() {
        let lattice = Lattice::build();
        let c = Triad::new_unchecked(0, Quality::Major);
        let d1 = lattice.at_distance(c, 1);
        assert_eq!(d1.len(), 3); // P(Cm), L(Em), R(Am)
    }

    #[test]
    fn test_lattice_max_distance() {
        let lattice = Lattice::build();
        // The diameter of the PLR lattice (D12 Cayley graph) should be small
        let mut max_dist = 0;
        for a in Triad::all() {
            for b in Triad::all() {
                let d = lattice.distance(a, b);
                if d > max_dist {
                    max_dist = d;
                }
            }
        }
        // D12 Cayley graph diameter with 3 generators is at most 6
        assert!(max_dist <= 6);
    }
}
