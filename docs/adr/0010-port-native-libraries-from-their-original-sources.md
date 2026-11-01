# Port native libraries from their original sources

For the follow-up program, prefer new mechanical ports of the selected codec
and pixel conversion libraries over adopting independently evolved Rust ports.
Correspondence with the original implementation and a verifiable 1:1 match take
priority over reducing the amount of translation work. Existing Rust projects
can inform comparisons, but are not assumed to satisfy that requirement.
rav1e is already Rust and is not a new C-to-Rust translation target.

The long-term scope is each selected library's complete functionality, provided
that it is cleanly covered by tests. Preserve the original suites and record
uncovered behavior as a gap rather than claiming completion from libavif's
integration tests alone. The precise equivalence criteria and target matrix
remain planning questions.

The delivered codec and pixel conversion implementations should no longer
depend on C/C++ implementations of those algorithms. Normal system facilities,
including libc, and original C/C++ reference implementations and test harnesses
remain allowed. Assembly, transitive algorithm libraries, and compatibility
wrappers still need explicit decisions. This extends the existing
[external-engine boundary](0007-optional-codec-boundaries.md) as a planned
follow-up; it does not claim that the current implementation has replaced those
engines.
