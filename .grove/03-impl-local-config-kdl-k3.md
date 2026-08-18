# local-config-kdl-k3

## Goal

Implement `.grove.kdl` resolution and its fail-closed diagnostics behind
`SessionConfig`, cover it at that seam, and bring `docs/CONFIGURATION.md` and
`docs/ARCHITECTURE.md` up to what the code then does.

## Context

The brief carries the six requirements and the file pointers. What is specific to
this leaf is the seam and the two hazards below.

**The seam is the signature.** `SessionConfig::load` takes only `home` today, and
that is exactly why "the personal file is the entirety of configuration" was
cheap to guarantee — the type could not express anything else. Giving it the
worktree is the design made checkable, and it is the change to make deliberately
rather than by threading a second path in through the back. Both existing load
points are in `src/loop_driver.rs`, the pre-mutation one and the pre-launch one,
and **both must resolve the delta** — otherwise a delta that is invalid only
becomes visible after the tree has already been mutated, which is the case
`task-tree-transactions-fail-closed` exists to prevent.

**Hazard one — the failure diagnostic currently names the wrong file.** When a
configured command fails to spawn, grove reports the session kind, word zero, and
*the personal config path*, unconditionally. Once a kind can resolve from a
delta, that report can name a file which never contained the failing template.
Correcting it to name the file the failing kind actually resolved from is part of
this leaf: it is a defect this change introduces, not scope beyond it. The same
applies to the aggregate validation report, which must attribute each diagnostic
to the file and location it came from.

**Hazard two — `${repo}` must not be re-derived here.** Requirement 6's second
search location is the main repository root, and grove already knows how to
compute it (`src/repo.rs`, and the `${repo}` substitution). Use that one
derivation. A second, subtly different notion of "the repository root" is exactly
the kind of drift that makes the search order behave differently from what
`${repo}` expands to in the very template it selects.

## Done when

- `.grove.kdl` is looked for at the worktree root, then at the main repository
  root, and **the first file found is the delta** — the other is not read and the
  two are never merged. Absence of both resolves exactly as today.
- Each kind the delta declares overrides the personal file's entry outright;
  every kind it does not declare falls through. The personal file is still
  required to declare all nineteen exactly once and is still fully validated.
- An invalid delta fails closed at **both** load points, with aggregate — not
  first-error — diagnostics carrying the delta's own path, line and column.
  Invalid covers: unreadable, unparseable KDL, an unknown kind name, a duplicate
  kind, a node with properties or children or the wrong argument count, and a
  template failing any existing rule (executable in word zero, one `${prompt}`,
  no repeated optional substitution, the unquoted-`#` rule).
- A spawn failure and every validation diagnostic name the file the affected kind
  actually resolved from.
- Covered at `SessionConfig::load`, extending `tests/session_config.rs` in the
  style already there (temp `$HOME`, temp worktree, no process spawn): absence
  resolves as today; a delta wins per kind while unnamed kinds fall through;
  worktree shadows repository root and the losing file is not read; each invalid
  form fails closed with its own path and location in the report.
  **No end-to-end driver test and no inspection verb** — requirement 5, taken
  deliberately with its cost stated. Do not quietly add either; if you come to
  believe the boundary is wrong, say so to the human rather than widen it.
- `docs/CONFIGURATION.md` documents the delta: where it is looked for and in what
  order, that it is per-kind and never merged, that the personal file stays
  complete, what an invalid delta does, and the `.gitignore` line the reader must
  add. It must also stop asserting that the personal file is *the entirety* of
  user configuration, and stop saying repository-local stamps neither override
  nor supplement it.
- `docs/ARCHITECTURE.md` agrees — its session-configuration section and the
  `session_config` row of its module table.
- Grove still creates or edits **no** configuration file and writes **no** ignore
  rule.
- `cargo test` green; `cargo fmt` and `cargo clippy` clean.
- The last act of this session is to decide whether a `review-impl` leaf is
  warranted and, if so, to cut it with the specific doubt written into its body.
  Requirement 5's stated gap — nothing asserts end-to-end that an overridden kind
  reaches a different program — is a reasonable doubt for a reviewer to be
  pointed at deliberately.

## Notes

Read `docs/adr/complete-session-configuration.md` as reworked by
`config-resolution-k2`, not as it stands today; that rework is this leaf's
contract.

Ordering within the session: seam first, then resolution, then diagnostics, then
documentation. Writing the documentation last is what keeps it a description of
what the code does rather than of what it was going to do.
