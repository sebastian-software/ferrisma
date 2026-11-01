# Preserve the C implementation mechanically before redesigning it

Port the pinned libavif source into Rust with its C ABI, names, data layout,
control flow, numeric behavior, and allocation ownership intact. This deliberately
retains raw pointers and unsafe C-shaped code so source correspondence and the
unchanged upstream suite can establish parity before any idiomatic Rust API,
ownership redesign, or optimization is considered.
