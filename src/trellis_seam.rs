//! The grove→trellis framework seam (ADR-0020).
//!
//! `trellis` is grove's **hard fork** of zellij, vendored at `crates/trellis/`
//! as a self-contained cargo workspace (see `crates/trellis/PROVENANCE.md`).
//! This module is the single, explicit place where grove declares its dependency
//! *on* the framework.
//!
//! **The seam is one-way (ADR-0020 §4):**
//! - grove **may** depend on trellis — this `use` is that edge.
//! - trellis **must never** depend on grove. That is enforced *structurally*, not
//!   by lint: `crates/trellis/` is a separate, workspace-`exclude`d cargo
//!   workspace with its own `Cargo.lock`; it builds standalone and has no path
//!   back-edge to the grove crate. A `grove = { path = "../.." }` added to any
//!   trellis crate would not resolve (grove is not laid out as a dependency of
//!   the trellis workspace) and would introduce a build cycle. Keeping trellis
//!   independently buildable is what keeps grove-isms out of the framework and
//!   makes the later clean extraction (ADR-0020 §7) possible.
//!
//! Gated behind the `trellis-seam` feature so a default build stays fast while
//! the native host API is built out (leaf 110); 110 makes the link non-optional.
//!
//! The anonymous `use ... as _` references the crate so the grove→trellis edge is
//! a compiled fact, not just a manifest entry. The real host-API surface (render
//! a native pane in-process, drive tabs/panes/focus) is built in leaf 110.

#[cfg(feature = "trellis-seam")]
#[allow(unused_imports)]
use trellis as _;
