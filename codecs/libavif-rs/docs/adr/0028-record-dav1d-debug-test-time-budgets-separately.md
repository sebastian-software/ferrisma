# Record dav1d Debug test time budgets separately

Keep normal unoptimized Rust Debug code generation and all original test inputs,
commands, assertions, and expected hashes. The initial Debug corpus run passed
1,708 tests and reached the original wall-clock limit in three tests. Allow an
explicit timeout multiplier for subsequent bounded execution, recording both
the original budget and the multiplier for every case. Treat timeouts as failed
executions and retain the original-limit result separately; a longer budget is
behavioral evidence and does not establish performance parity.
