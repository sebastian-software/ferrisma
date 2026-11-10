# Map libaom's nonlocal errors to Rust unwinding

Replace the selected original `setjmp` recovery scopes and `longjmp` error
transfer mechanically with typed Rust unwinding keyed by the original jump
buffer address, retaining the original recovery statements and state changes.
Use the C-unwind ABI throughout translated libaom frames and callbacks and
`panic=unwind` for workspace profiles: Rust has no supported returns-twice
foreign call contract, so directly retaining libc setjmp is not acceptable.
Unrelated payloads and targets propagate unchanged; the original error transfer
bypasses the panic hook with `resume_unwind`.

The typed runtime allocates an unwind payload on a nonlocal error. Fault tests
must distinguish original codec allocation failures from Rust runtime
allocation exhaustion; the latter is not an established parity claim. Original
C/C++ consumers remain validation infrastructure, not a C++ exception API.
