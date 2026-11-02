# Exercise libaom inline subjects through optional Rust exports

Expose reviewed original header-inline test subjects through generated wrappers
behind the nondefault `port-test-bridges` feature. Each wrapper calls an existing
translated inline function in the same module and retains its original ABI.
Keep the production algorithm body unchanged and inventory every added export.

Recompile the original C++ test inputs with forced includes that redirect only
the reviewed subjects to those exports. Preserve the original test bodies,
case registrations, parameters, assertions, and native reference executable.
Keep native oracle and fixture helpers bound to their original implementations.
This faithful boundary follows ADR 0016 without reimplementing GoogleTest or
rewriting the original assertions in Rust. Remaining inline roles, indirect
calls, and scalar self-comparisons still require explicit review.
