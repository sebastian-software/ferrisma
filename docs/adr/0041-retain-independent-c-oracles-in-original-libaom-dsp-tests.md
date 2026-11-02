# Retain independent C oracles in original libaom DSP tests

Separate reviewed native C oracle bindings from Rust subject bindings in the
original parameterized DSP tests. Relinking every scalar symbol to Rust can
otherwise turn an original C-versus-C comparison into Rust-versus-Rust while
preserving a passing assertion.

Rename every defined codec global and its relocations in a separate complete
original C archive, preserving its preprocessing, instructions, data, and
archive members. Audit that no original codec name remains inside this oracle
archive. Redirect only explicit, source-hash-checked oracle call sites in
disposable compilation copies of the original tests. Keep subject parameters,
assertions, inputs, case names, disabled flags, and the original baseline
executable unchanged. Verify both implementations' ownership in the link map.

Start with six convolution subjects and their 243 existing cases. Review other
DSP roles separately; this boundary does not establish complete correspondence
or enable any upstream-disabled registrations.
