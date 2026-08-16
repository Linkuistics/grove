# mandate-machinery-k22

**Integrates:** mandate-machinery-k21

## Goal

Triage and apply the mandate-machinery deletion review findings below. Preserve
the deletion, but restore the checks and documentation claims that the deletion
currently leaves weaker or false.

## Context

The reviewed producer is `mandate-machinery-k10` (commit `b61adcf58a64`). The
review was inspection-only; no test, build, lint, or format command was run.

## Findings

### 1. High — the stale-embed check drops the bytes it claims to compare

`tests/methodology.rs:44` maps every embedded `(path, text)` pair to `path` and
compares only a `BTreeSet<String>` of filenames with the on-disk tree. Editing an
existing Markdown file leaves both sets identical. If `build.rs`'s per-file
`rerun-if-changed` walk misses that edit — the exact regression this test says it
is the sole check for — `cargo test` can run the stale test binary, compare the
same paths, and pass while the binary still embeds old prose. The installed skill
can therefore lag `content/` with the claimed guard green.

Make the independent disk/embed comparison include each file's bytes (or an
equivalent content digest), and retain a control that demonstrates a same-path
content mismatch is rejected.

### 2. High — future family members can be routed to the wrong discipline

`tests/prompt.rs:431` hand-enumerates today's five review kinds, five integration
kinds, and two research kinds. The exhaustive match at `src/prompt.rs:136` forces
a new `Kind` variant into *some* arm, but nothing forces it into the arm its label
and taxonomy imply. For example, a future `ReviewSecurity` variant can be mapped
to `references/impl.md`: every mapped path still exists, the manually frozen
review-family slice never sees the new variant, and the distinct path count can
remain ten, so the replacement checks stay green while review sessions receive
implementation discipline. This is the live taxonomy claim the deleted
family-scope guard used to protect.

Derive family membership independently from the closed kind taxonomy and check
the mapping against it, including a control that deliberately misfiles one
family-shaped kind.

### 3. Low — the architecture names the deleted verification seam

`docs/ARCHITECTURE.md:130` still says the agent grammar is scanned by “the
provisioning test,” but the producer moved that scan to `tests/methodology.rs`.
That sends maintainers to a verification boundary that no longer owns the claim,
undercutting the record reconciliation this deletion is meant to complete.
Name the methodology test instead.

## Done when

- Each finding is verified and either fixed or rejected with concrete evidence.
- The embed/disk guard demonstrably rejects a same-path content mismatch.
- The kind-family routing guard demonstrably rejects a misfiled future-family
  member.
- The affected test, build, lint, and format checks pass after integration.

## Notes

The instructed-verb scan still reads every embedded Markdown file, pins all
eleven instructed verbs, detects the removed `methodology` verb, and retains its
flat-surface premise. The ending drift pin now targets the same two files' bytes,
the closed-fact locator fails loudly when an opener disappears, no live Markdown
link targets the deleted spec, and the marker-removal corpus diff showed no
additional joined-prose boundary beyond the six already recorded by the
producer.
