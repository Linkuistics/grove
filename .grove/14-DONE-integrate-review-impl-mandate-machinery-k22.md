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

## Triage

All three findings **verified and fixed**; none rejected. Each of the two high
findings was the same species — a check whose predicate is weaker than its own
docstring, and weaker in the direction it exists to catch — so each fix is the
widened predicate plus a control that drives it past the line.

1. **Confirmed, fixed.** `.map(|(path, _)| path)` really did discard the bytes.
   The comparison now runs over `BTreeMap<path, contents>` on both sides, still
   gathered by two independent traversals (`fs::read_dir` against
   `include_dir`'s). Disagreements are reported per path by a `disagreements`
   helper rather than by `assert_eq!` on the maps, because an equality failure
   over a tens-of-KiB corpus would print all of it twice to say one file moved.
   Three controls: a same-path content mismatch (the case a path set could not
   state), and the two path-shaped cases the old equality did catch, kept.
2. **Confirmed, fixed.** The exhaustive `match` forces a new variant into *some*
   arm and nothing forces it into the right one; a hand-written membership list
   never sees it. Membership is now derived by `routing_group`, reading each
   kind's **own label** (`integrate-review-` before `review-`, since every
   integration label contains a review one; `combine-research` deliberately its
   own group). Two claims over the derived grouping — every group names one file,
   and no two groups name the same — which together state a bijection between the
   ten routing groups and the ten files. The control misfiles a synthetic
   `review-security` to `references/impl.md`: it resolves, it keeps the distinct
   count at ten, and the family check now names it.
3. **Confirmed, fixed.** `docs/ARCHITECTURE.md` now names `tests/methodology.rs`.
   `tests/provision.rs` still exists and its other three references are correct,
   so only the agent-grammar sentence moved.

**Records reworked in place, because the deletion left them stating what no
longer holds** (they are current-state sets, so nothing is appended):

- `docs/specs/skill-delivered-methodology.md` claimed the family hazard was
  closed by "an exhaustive match plus a test that every path exists" — which is
  exactly the pair finding 2 shows does not close it. The paragraph now says what
  does, and the family scenario is restated non-enumeratively with the misfile
  case beside it.
- `docs/ARCHITECTURE.md` now says the embed comparison is on contents, since a
  missed edit moves no filename.

**One residue found while reading, outside the findings and fixed with them.**
`Cargo.toml`'s `[build-dependencies]` still described `build.rs` as
`#[path]`-including the unit-marker parser and the `Kind` set — machinery
`mandate-machinery-k10` deleted. `build.rs` is now `std`-only, so the `anyhow`
build-dependency was dead along with the comment justifying it; both are gone.
`cargo check --locked --all-targets` passes, so `Cargo.lock` is untouched
(`anyhow` remains an ordinary dependency).

**Verification.** Full `cargo test` green — every one of the suite's 41 result
lines `ok`, no failures —
`cargo clippy --all-targets` clean at the manifest's deny-all baseline,
`cargo fmt --all` applied, `cargo check --locked --all-targets` passes.

## Notes

The instructed-verb scan still reads every embedded Markdown file, pins all
eleven instructed verbs, detects the removed `methodology` verb, and retains its
flat-surface premise. The ending drift pin now targets the same two files' bytes,
the closed-fact locator fails loudly when an opener disappears, no live Markdown
link targets the deleted spec, and the marker-removal corpus diff showed no
additional joined-prose boundary beyond the six already recorded by the
producer.
