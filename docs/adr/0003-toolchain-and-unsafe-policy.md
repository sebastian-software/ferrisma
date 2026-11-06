# Build on stable Rust with unsafe code denied by default

- Status: accepted
- Created: 2026-11-06
- Updated: 2026-11-06

## Context

Ferrixel's core is stable Rust without `unsafe` and accepts codec `unsafe` only
in isolated, audited places. libwebp-rs shows that a mechanical decoder port
can stay in safe Rust except for its exported C ABI.

## Decision

- `rust-version = "1.95"`, edition 2024, stable toolchain
  (`rust-toolchain.toml`). Raise the minimum deliberately, in its own change.
- `src/lib.rs` declares `#![deny(unsafe_code)]`. A module that needs `unsafe`
  gets an ADR that names the module, the operations and the local safety
  argument, and an item-level `#[allow(unsafe_code, reason = ...)]`; inside it,
  one unsafe operation per block with a `// SAFETY:` comment. Two such ADRs
  are expected: the exported C ABI (work package 7) and the SIMD kernels
  ([ADR 0004](0004-simd-after-the-portable-port.md)).
- `src/lib.rs` declares `#![warn(clippy::all, clippy::pedantic)]`; CI runs
  Clippy with `-D warnings`. In translated code, keep the source's form and add
  an item-scoped `#[expect(clippy::<lint>, reason = "...")]` naming the C
  construct. Use `expect`, not `allow`, so a stale exception fails the check.
- No runtime dependencies. Development dependencies need an amendment of this
  record.

## Consequences

The decoder builds with a plain `cargo build` on stable Rust. libjpeg's error
exit is a callback that applications usually leave with `longjmp`; crossing
Rust frames that way is undefined behavior, so the C ABI package must decide how
error exits are represented before the ABI is exported (libavif-rs ADR 0036
mapped libaom's nonlocal errors to Rust unwinding).

## Review triggers

Revisit when a unit cannot be expressed in safe Rust, before the C ABI or SIMD
packages start, or when the minimum Rust version is raised.
