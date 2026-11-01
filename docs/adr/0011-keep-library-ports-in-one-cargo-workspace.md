# Keep library ports in one Cargo workspace

Develop the independent library crates together with libavif-rs in this
repository, using a shared Cargo workspace. This keeps cross-library changes,
native reference builds, and integration evidence reviewable together while
preserving separate crate boundaries.

Separate repositories would make independent ownership easier but would add
coordination and version synchronization before the porting contracts are
proven. Exact package names, publication policy, and workspace implementation
remain to be planned; this decision does not restructure the existing crate.
