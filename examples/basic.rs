use groovemesh_plr::{Triad, Quality, Lattice, apply, PLR, PLRWord, find_word, CounterpointRules, nearest_triad, VoiceLeading};

fn main() {
    // The PLR group acts on the 24 major/minor triads via three involutions:
    //   P = Parallel (same root, flip quality): C → Cm
    //   L = Leading-tone exchange: C → Em
    //   R = Relative: C → Am
    //
    // These generate the dihedral group D₁₂, acting transitively on all 24 triads.

    let c_major = Triad::new(0, Quality::Major).unwrap();
    let a_minor = Triad::new(9, Quality::Minor).unwrap();

    println!("=== PLR Group Voice Leading ===\n");
    println!("Start: {}", c_major);
    println!("Target: {}", a_minor);

    // Apply individual operations
    println!("\nP(C) = {}", apply(PLR::P, c_major));
    println!("L(C) = {}", apply(PLR::L, c_major));
    println!("R(C) = {}", apply(PLR::R, c_major));

    // Build the full PLR lattice and find shortest path
    let lattice = Lattice::build();
    let path = lattice.shortest_path(c_major, a_minor).unwrap();
    println!("\nShortest path C → Am: {:?}", path.iter().map(|op| format!("{}", op)).collect::<Vec<_>>());
    println!("Distance: {}", lattice.distance(c_major, a_minor));

    // Find the PLR word transforming C into Am
    let word = find_word(c_major, a_minor).unwrap();
    println!("PLR word: {}", word);
    println!("Apply word to C: {}", word.apply(c_major));

    // Compose and reduce
    let w = PLRWord::from_ops(&[PLR::P, PLR::L, PLR::P, PLR::P]);
    println!("\nWord PLPP reduced: {} → {}", w, w.reduce());

    // Counterpoint-aware path finding
    let rules = CounterpointRules::new();
    let legal = rules.legal_path(c_major, a_minor, 10);
    println!("\nCounterpoint-legal path C → Am:");
    for (op, triad) in &legal {
        println!("  {} → {}", op, triad);
    }

    // Nearest triad by pitch classes
    let nearest = nearest_triad(&[0, 4, 7]).unwrap();
    println!("\nNearest triad to C-E-G: {}", nearest);

    // Voice leading between C major and F# major
    let fs_major = Triad::new(6, Quality::Major).unwrap();
    let vl = VoiceLeading::minimal_voice_leading(c_major, fs_major);
    println!("\nVoice leading C → F#: distance = {} semitones", vl.distance);
}
