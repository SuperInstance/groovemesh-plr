//! Tutorial: navigate the PLR lattice, find paths, and check counterpoint.
//!
//! Run with: cargo run --example tutorial

use groovemesh_plr::{
    Triad, Quality, Lattice, PLR, apply,
    find_word, orbit, verify_group_axioms,
    minimal_voice_leading, CounterpointRules,
    nearest_triad,
};
use groovemesh_plr::nearest::nearest_plr_triad;

fn main() {
    println!("=== PLR Group Algebra Tutorial ===\n");

    // Step 1: The 24 triads
    println!("Step 1: The 24 triads (12 roots × 2 qualities)");
    let all = Triad::all();
    println!("  Total: {} triads", all.len());
    println!("  First 5: {}", all[..5].iter().map(|t| t.to_string()).collect::<Vec<_>>().join(", "));

    // Step 2: Build the lattice
    println!("\nStep 2: Build the PLR lattice");
    let lattice = Lattice::build();
    println!("  Nodes: {}", lattice.node_count());
    let c = Triad::new_unchecked(0, Quality::Major);
    let neighbors = lattice.neighbors(c);
    println!("  Neighbors of C: {:?}", neighbors.iter().map(|e| format!("{}→{}", e.op, e.target)).collect::<Vec<_>>());

    // Step 3: Find paths
    println!("\nStep 3: Find shortest paths");
    let targets = [
        Triad::from_name("Am").unwrap(),
        Triad::from_name("F#m").unwrap(),
        Triad::from_name("Bb").unwrap(),
    ];
    for target in &targets {
        let path = lattice.shortest_path(c, *target).unwrap();
        let ops: String = path.iter().map(|p| format!("{}", p)).collect();
        let dist = lattice.distance(c, *target);
        println!("  C → {}: {} steps ({})", target, dist, ops);
    }

    // Step 4: Voice leading
    println!("\nStep 4: Voice leading distances");
    let pairs = [
        (Triad::new_unchecked(0, Quality::Major), Triad::new_unchecked(0, Quality::Minor)),
        (Triad::new_unchecked(0, Quality::Major), Triad::new_unchecked(9, Quality::Minor)),
        (Triad::new_unchecked(0, Quality::Major), Triad::new_unchecked(6, Quality::Major)),
    ];
    for (from, to) in &pairs {
        let vl = minimal_voice_leading(*from, *to);
        println!("  {} → {}: {} semitones", from, to, vl.distance);
    }

    // Step 5: Counterpoint rules
    println!("\nStep 5: Counterpoint rules");
    let rules = CounterpointRules::new();
    let am = Triad::new_unchecked(9, Quality::Minor);
    let path = rules.legal_path(c, am, 10);
    println!("  Legal path C → Am:");
    for (op, triad) in &path {
        println!("    {} → {} ({})", op, triad, triad);
    }

    // Step 6: Nearest triad
    println!("\nStep 6: Nearest triad from arbitrary pitches");
    let cases: Vec<(Vec<u8>, &str)> = vec![
        (vec![0, 4, 7], "C,E,G"),
        (vec![9, 0, 4], "A,C,E"),
        (vec![0, 7], "C,G"),
        (vec![1, 5, 8], "C#,F,G#"),
    ];
    for (pitches, label) in &cases {
        let triad = nearest_triad(pitches).unwrap();
        println!("  [{}] → {}", label, triad);
    }

    // Step 7: PLR step toward a target
    println!("\nStep 7: PLR step toward target");
    let next = nearest_plr_triad(c, &[9, 0, 4]);
    println!("  From C, toward Am pitches: {}", next);

    // Step 8: Group verification
    println!("\nStep 8: Group axioms");
    println!("  Orbit size: {}", orbit(c).len());
    println!("  Axioms verified: {}", verify_group_axioms());
}
