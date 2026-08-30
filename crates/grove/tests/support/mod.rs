// The shared test-only helpers, which live once at `testing/support.rs`.
//
// `loop-crate-driver-k22` made the repository root a bare workspace, so the
// helpers three packages share — env isolation, the readiness seam, the jj
// fixtures, the kind-label walk — have no root package to live in. They are
// about the *repository*, not about any one package; copying them would have
// produced three that drift, so this is the one, included by path.
//
// Cargo's `tests/*.rs` auto-discovery only scans direct children of `tests/`, so
// this file is not itself a test binary — each consumer pulls it in with
// `mod support;`.
#[path = "../../../../testing/support.rs"]
mod shared;

pub use shared::*;
