# Exercise the original static fault fuzzer

Retain the original `dav1d_fuzzer.c`, `main.c`, and `alloc_fail.c` bodies and
their five allocation/thread failure hooks. Apply upstream's symbol rewriting
to the 51 C decoder objects and all Rust decoder codegen objects; link the
original Rust archive only for runtime dependencies and reject any unwrapped
decoder member in the link map. The observer counts calls and failures without
changing the original fuzzer's exit/crash oracle. Header variants of four
unchanged frame payloads provide bounded failure coverage; this is not a
continuous fuzz campaign or exhaustive failure-position enumeration.

LLVM can fold the selected source's `malloc` plus zeroing into `calloc` in
Release. Restore that operation through a test-only zeroing adapter so the
original malloc hook still reaches it; production objects remain unchanged.
Reject explicit source `calloc` calls until their hook contract is reviewed.
Require observed calls and failures in all five classes in each implementation;
concurrent random-hook event ordering is recorded without an equality claim.
