# Use one compatible sanitizer runtime for C and Rust parity tests

The host AppleClang runtime uses an Apple-specific ABI guard, while Rust's
instrumentation uses the standard ASan v8 guard. Compile native parity consumers
with LLVM 16 and link both implementations to the selected Rust nightly's v8
runtime; retain version guards and memory instrumentation rather than mixing
incompatible runtimes or suppressing findings.

Exclude only C2Rust's immutable loader function-pointer arrays from global
redzone padding, because Mach-O requires an exact array of initializer pointers.
Initializer functions and library memory operations remain instrumented. Match
upstream sanitized CI's `allocator_may_return_null=1` setting for tests that
deliberately request impossible allocations. These settings affect the sanitizer
harness; ordinary builds retain the original execution configuration.
