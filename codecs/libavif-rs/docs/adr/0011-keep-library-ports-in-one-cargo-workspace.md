# Keep library ports in one Cargo workspace

Develop the independent library crates together with libavif-rs in this
repository, using a shared Cargo workspace. This keeps cross-library changes,
native reference builds, and integration evidence reviewable together while
preserving separate crate boundaries.

Separate repositories would make independent ownership easier but would add
coordination and version synchronization before the porting contracts are
proven. Exact package names, publication policy, and workspace implementation
remain to be planned; this decision does not restructure the existing crate.

## Monorepo amendment, 2026-11-10

[Ferrisma ADR-0026](../../../../docs/adr/0026-codecs-in-one-repository-and-workspace.md)
supersedes the former repository-local workspace, profile and development
toolchain placement. This package uses the root workspace and lockfile. The
original compiler compatibility promises and independent reference comparisons
remain; handwritten ports explicitly use stable Clippy and their MSRV jobs.
Shared profiles retain the numeric and unwinding requirements of each port.
Active workflows now live at the Ferrisma repository root.
