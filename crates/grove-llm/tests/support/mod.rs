// The shared test-only helpers, which live once at the workspace root.
//
// `loop-crate-verbs-k21` split this repository's tests across two packages —
// `grove` at the root and `grove-llm` beside it — and the helpers they share
// (env isolation, the readiness seam, the jj fixtures, the kind-label walk) are
// about the *repository*, not about either package. Copying them would have
// produced two that drift; this is the one, included by path.
//
// Cargo's `tests/*.rs` auto-discovery only scans direct children of `tests/`, so
// this file is not itself a test binary — each consumer pulls it in with
// `mod support;`, exactly as the root package's do.
#[path = "../../../../tests/support/mod.rs"]
mod shared;

pub use shared::*;
