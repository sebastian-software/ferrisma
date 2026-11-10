# Map dav1d atomics and C address-taking mechanically

Preserve each original C11 atomic operation and memory ordering while updating
C2Rust's old intrinsic spelling to the pinned Rust compiler's const arguments.
Retain strong compare/exchange, sequentially consistent defaults, explicit
relaxed/acquire operations, integer widths, and nonvolatile atomic accesses.
No ordering is weakened or synchronization algorithm redesigned.

Use raw address expressions for C address-taking and generated assignment
temporaries, including shared atomic fields and static array fields. C addresses
must not introduce Rust exclusive borrows. Map pthread storage, once initialization,
FILE handles, and variadic argument lists to the target runtime representation;
keep the original pthread operations and initialization sequence. All mappings
are applied by the reproducible importer and retain per-unit correction counts.
