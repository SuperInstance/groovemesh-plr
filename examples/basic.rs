//! Basic groovemesh-plr usage: create triads and apply PLR transformations.
//!
//! Run with: cargo run --example basic

use groovemesh_plr::{Triad, Quality, apply_p, apply_l, apply_r, PLR, apply};

fn main() {
    // Create a C major triad
    let c = Triad::new_unchecked(0, Quality::Major);
    println!("Starting triad: {}", c);
    println!("Pitch classes: {:?}", c.pitch_classes());

    // Apply the three PLR transformations
    let parallel = apply_p(c);     // C → Cm
    let leading = apply_l(c);      // C → Em
    let relative = apply_r(c);     // C → Am

    println!("\nPLR transformations:");
    println!("  P({}) = {}", c, parallel);
    println!("  L({}) = {}", c, leading);
    println!("  R({}) = {}", c, relative);

    // Involution: P² = identity
    println!("\nInvolution check:");
    println!("  P(P(C)) = {} (should be C major)", apply(PLR::P, parallel));
    println!("  L(L(C)) = {} (should be C major)", apply(PLR::L, leading));
    println!("  R(R(C)) = {} (should be C major)", apply(PLR::R, relative));

    // Parse from chord names
    let am = Triad::from_name("Am").unwrap();
    let fs = Triad::from_name("F#").unwrap();
    println!("\nFrom name: Am = {}, F# = {}", am, fs);
}
