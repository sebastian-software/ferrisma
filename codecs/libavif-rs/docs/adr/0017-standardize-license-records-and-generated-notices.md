# Standardize license records and generated notices

Use a common license-file layout, provenance records, and generated per-crate
license summaries across the workspace. This reduces duplicated manual
maintenance while retaining each source's original terms, attributions,
notices, and patent material as required by
[ADR 0014](0014-retain-upstream-license-terms-for-ported-crates.md).

Derive crate metadata and distribution notices from the files actually
packaged. Standardizing this process does not replace upstream licenses or
create an alternative licensing choice. A broader relicensing proposal remains
separate from the mechanical porting work.
