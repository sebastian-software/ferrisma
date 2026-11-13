//! Print the detected and chosen kernel levels; CI runs this so that every
//! job log states which variants its differential tests exercised.

fn main() {
    let best = ferrisma_kernels::Kernels::best();
    println!("machine level: {}", ferrisma_kernels::machine_level());
    println!("chosen level: {} ({:?})", best.level(), best.reason());
    println!(
        "default ceiling: {}; with explicit avx512 ceiling: {}",
        ferrisma_kernels::DEFAULT_CEILING,
        ferrisma_kernels::Kernels::with_ceiling(ferrisma_kernels::IsaLevel::Avx512).level()
    );
    let available: Vec<String> = ferrisma_kernels::Kernels::available()
        .iter()
        .map(|kernels| kernels.level().to_string())
        .collect();
    println!("levels exercised by tests: {}", available.join(", "));
}
