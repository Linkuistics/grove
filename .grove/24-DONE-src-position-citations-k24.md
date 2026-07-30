# src-position-citations-k24

**Kind:** impl

## Goal

Two `src/` comments still cite dead `.grove/` positions in prose, which
`stale-module-headers-k14` reported as swept to zero. Fix both, and fix the
brief's claim that the class is clear.

## Context

Raised by `driving-original-scheme-example-k22`, which ran the same class of grep
one directory over. Both sites verified by reading, not asserted:

- `src/leaf_id.rs:143` — "*Public so the grow verbs (030) can parse a bare
  `<parent-id>`/`<target-id>` positional*". `(030)` is a dead leaf position.
- `src/provision.rs:7` — "*this replaces the old fetch-tarball +
  materialise-per-harness + `VERSION.md` model (whose deletion is leaf 090)*".

ADR *task-tree-scheme* §5 binds prose to `<slug>-k<key>` and forbids the
position; a source comment is prose. Both referents are doubly dead — that
grove's `.grove/` was deleted at its finish cycle, **and** it predates the key
scheme, so like `k22`'s referents there is no handle to convert to. `k14`'s own
settled answer applies: where the citation is load-bearing it becomes an **ADR
slug or a module path**, otherwise it goes. `provision.rs`'s line already cites
`self-extension-core-and-methodology` / `task-tree-scheme` two lines above, so
the parenthetical may simply be redundant.

**Why `k14` missed them, which is the reusable part.** Its Done-when swept
`src/` **module headers** (`//!`) against an **enumerated pattern list**
(`11.x`, `070/040`, `060/020`, `D<n>`). Both narrowings leaked: `leaf_id.rs:143`
is a *function* doc comment (`///`), and `provision.rs:7` is a module header
whose spelling (`leaf 090`) is on none of the four listed patterns. Same family
as `k11`'s "grep the claim, not the file list" — here, **grep the claim, not an
enumerated list of its spellings**.

`k22` also measured what is *not* an instance, so this leaf need not re-derive
it: the `D<n>` class is genuinely at zero; every surviving dotted-decimal in
`src/` is a version (`0.145.0`), a timing (`0.1–0.4s`), or the v1 grammar **as
subject** (`leaf_id.rs:34,140`, `tree_grow.rs:10`, `tree_migrate.rs:62,1170`);
and every three-digit token in `tree_migrate.rs` is original-scheme **fixture
data**, where the position is the input under test.

## Done when

- Neither site cites a `.grove/` position; any load-bearing referent became an
  ADR slug or a module path.
- The claim is re-verified by grep over all of `src/`, not by this leaf's
  two-item list — and the grep is checked for having actually run (`k17`/`k20`/
  `k21` each recorded a flag trap that manufactures a false clean).
- `cargo test` passes.

(The root brief's `stale-module-headers-k14` paragraph is **already corrected** —
`k22` falsified the claim, so `k22` recorded it, quoted-and-refuted per
`docs-reconcile-k6`'s pattern. Nothing owed here.)

## Notes

CHANGELOG-free unless it changes behaviour, per the rule `retire-help-node-path-k20`
settled and `k21`/`k22` inherited: stale prose corrected against already-shipped
behaviour earns no entry.
