//! Advanced: compose PLR words, Cayley tables, and chord progressions.
//!
//! Run with: cargo run --example advanced

use groovemesh_plr::{
    Triad, Quality, PLR, PLRWord, apply, find_word,
    Lattice, orbit, suborbit,
    minimal_voice_leading, nearest_triad,
    CounterpointRules,
};
use groovemesh_plr::group::cayley_table;
use groovemesh_plr::voice::voice_leading_distance;

fn main() {
    println!("=== Advanced PLR Group Algebra ===\n");

    // ── PLR Word composition ──
    println!("1. PLR Word Composition");
    let w1 = PLRWord::from_ops(&[PLR::P, PLR::L]);
    let w2 = PLRWord::from_ops(&[PLR::R]);
    let composed = w1.compose(&w2);
    println!("   P·L composed with R = {:?}", composed.ops);

    let c = Triad::new_unchecked(0, Quality::Major);
    let result = composed.apply(c);
    println!("   PLR(C) = {}", result);

    // Inverse
    let inv = composed.inverse();
    println!("   Inverse: {:?}", inv.ops);
    let back = inv.apply(result);
    println!("   Apply inverse: {} (should be C)", back);

    // Reduction: PP cancels
    let word = PLRWord::from_ops(&[PLR::P, PLR::L, PLR::P, PLR::P]);
    let reduced = word.reduce();
    println!("   Reduce PLPP → {:?}", reduced.ops);

    // ── Cayley table ──
    println!("\n2. Cayley Table");
    let table = cayley_table();
    let c = Triad::new_unchecked(0, Quality::Major);
    let row = table.get(&c).unwrap();
    println!("   C major row:");
    for (op, triad) in row {
        println!("     {}(C) = {}", op, triad);
    }

    // ── Suborbits ──
    println!("\n3. Suborbits (subgroups)");
    let p_only = suborbit(c, &[PLR::P]);
    println!("   <P> orbit: {} triads", p_only.len());
    // Should be just {C, Cm}
    for t in &p_only {
        println!("     {}", t);
    }

    let pl_orbit = suborbit(c, &[PLR::P, PLR::L]);
    println!("   <P,L> orbit: {} triads", pl_orbit.len());

    // ── Chord progression via legal PLR steps ──
    println!("\n4. Chord Progression (C → F#m → Bb → Cm)");
    let targets = [
        Triad::from_name("F#m").unwrap(),
        Triad::from_name("Bb").unwrap(),
        Triad::from_name("Cm").unwrap(),
    ];
    let rules = CounterpointRules::new();
    let mut current = Triad::new_unchecked(0, Quality::Major);
    let mut progression = vec![current];

    for target in &targets {
        let path = rules.legal_path(current, *target, 10);
        for (_, triad) in &path {
            progression.push(*triad);
        }
        if let Some(&last) = path.last() {
            current = last.1;
        }
    }

    println!("   Full progression:");
    for (i, triad) in progression.iter().enumerate() {
        let vl = if i > 0 {
            let d = voice_leading_distance(progression[i-1], *triad);
            format!(" (distance: {})", d)
        } else {
            String::new()
        };
        println!("     {}: {}{}", i, triad, vl);
    }

    // ── Nearest triad analysis ──
    println!("\n5. Nearest Triad for common chord voicings");
    let voicings: Vec<(&str, Vec<u8>)> = vec![
        ("C open", vec![0, 4, 7]),
        ("Am open", vec![9, 0, 4]),
        ("G barre", vec![7, 11, 2]),
        ("Add9", vec![0, 4, 7, 2]),      // 4 notes → best triad match
        ("Sus2", vec![0, 2, 7]),          // not a major/minor triad
        ("Tritone", vec![0, 6]),           // very ambiguous
    ];
    for (name, pitches) in &voicings {
        let triad = nearest_triad(pitches).unwrap();
        let common = triad.pitch_classes();
        let matches: usize = pitches.iter().filter(|&&p| common.contains(&p)).count();
        println!("   {}: {:?} → {} ({}/{} match)", name, pitches, triad, matches, pitches.len());
    }

    // ── Lattice distances ──
    println!("\n6. Lattice Diameter");
    let lattice = Lattice::build();
    let mut max_dist = 0;
    let mut max_pair = (Triad::new_unchecked(0, Quality::Major), Triad::new_unchecked(0, Quality::Major));
    for a in Triad::all() {
        for b in Triad::all() {
            let d = lattice.distance(a, b);
            if d > max_dist {
                max_dist = d;
                max_pair = (a, b);
            }
        }
    }
    println!("   Maximum distance: {} ({})", max_dist, format!("{} ↔ {}", max_pair.0, max_pair.1));
}
