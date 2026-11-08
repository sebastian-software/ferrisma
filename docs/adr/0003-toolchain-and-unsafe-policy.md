# Build on stable Rust with unsafe code denied by default

- Status: accepted
- Created: 2026-11-06
- Updated: 2026-11-08

## Context

Ferrixel's core is stable Rust without `unsafe` and accepts codec `unsafe` only
in isolated, audited places. libwebp-rs shows that a mechanical decoder port
can stay in safe Rust except for its exported C ABI, which this port does not
have: it serves Rust programs, and C compatibility is not a goal.

## Decision

- `rust-version = "1.95"`, edition 2024, stable toolchain
  (`rust-toolchain.toml`). Raise the minimum deliberately, in its own change.
- `src/lib.rs` declares `#![deny(unsafe_code)]`. A module that needs `unsafe`
  gets an ADR that names the module, the operations and the local safety
  argument, and an item-level `#[allow(unsafe_code, reason = ...)]`; inside it,
  one unsafe operation per block with a `// SAFETY:` comment. One such ADR is
  expected, for the SIMD kernels
  ([ADR 0004](0004-simd-after-the-portable-port.md)).
- The library exports no C ABI. Should a test harness ever need a C
  interface, it lives in a separate crate of the workspace that the library's
  consumers never link, with its own ADR.
- `src/lib.rs` declares `#![warn(clippy::all, clippy::pedantic)]`; CI runs
  Clippy with `-D warnings`. In translated code, keep the source's form and add
  an item-scoped `#[expect(clippy::<lint>, reason = "...")]` naming the C
  construct. Use `expect`, not `allow`, so a stale exception fails the check.
- No runtime dependencies. Development dependencies need an amendment of this
  record.

## Consequences

The decoder builds with a plain `cargo build` on stable Rust, and the crate
that consumers link contains no `unsafe` until the SIMD packages add theirs.
Without a C ABI, no C application's `longjmp` ever crosses Rust frames.

## Review triggers

Revisit when a unit cannot be expressed in safe Rust, before the SIMD
packages start, or when the minimum Rust version is raised.
