//! Print the detected and chosen kernel levels; CI runs this so that every
//! job log states which variants its differential tests exercised.

fn main() {
    let best = ferrixel_kernels::Kernels::best();
    println!("machine level: {}", ferrixel_kernels::machine_level());
    println!("chosen level: {} ({:?})", best.level(), best.reason());
    let available: Vec<String> = ferrixel_kernels::Kernels::available()
        .iter()
        .map(|kernels| kernels.level().to_string())
        .collect();
    println!("levels exercised by tests: {}", available.join(", "));
}
