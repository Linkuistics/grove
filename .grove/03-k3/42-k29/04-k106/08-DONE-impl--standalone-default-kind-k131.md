# standalone-default-kind-k131

## Goal

Give `grove run` a default kind `standalone`, and remove the promise that a
standalone run reports missing guidance. The human decided both during
confined-harness-trials-k128.

## Context

- The record sections "missing-kind-confined in Codex, repaired" and "Where a
  durable repair belongs" of `docs/common-loading-acceptance-record.md`.
- `crates/grove/src/standalone.rs` and `crates/grove/tests/standalone.rs`.
- The sentences "It reports the missing guidance and runs the force-stop
  command" in `docs/USAGE.md` and `docs/specs/standalone-invocations.md`.
- The Confined standalone row and the paragraph on
  `crates/grove/tests/standalone.rs` in `docs/specs/codex-skill-provisioning.md`.

## Done when

- `grove run` accepts no kind. Outside an active grove, it then routes the
  kind `standalone`. An explicit kind keeps its present behaviour. Help text,
  `docs/USAGE.md` and the standalone specification name the default and its
  condition.
- Inside an active grove, a bare `grove run` does not use `standalone`. The
  leaf asks the human two questions before it changes code. How does
  `grove run` detect an active grove? What does a bare `grove run` do inside
  one? The answers go into the running log and the specification.
- The kind `standalone` requires no skill. It exists so that the owner's
  policy can select a harness, a model and an effort for a standalone
  invocation. The prompt of a standalone run gains no load instruction and no
  missing-guidance rule, for any kind.
- No document promises that a standalone agent reports missing guidance. The
  Confined standalone row keeps its sandbox part: native trees stay unreadable
  and Grove adds no grant. The agent part becomes an explicit gap by the
  human's decision, with `missing-kind-confined-20261103T062817Z` as its
  evidence.
- The standalone fixture states what it proves: Grove's interpretation of a
  force-stop. It no longer stands as evidence of agent behaviour.
- The record's triage gives `missing-kind-confined` in both harnesses its
  decision. Grove's full `bash scripts/check.sh` passes.

## Notes

The human's words: "standalone should not *require* a skill. it is primarily
so that the harness/model/effort can be selected for standalone invocations.
The fact that the skill might or might not exist shouldn't matter."
And later: "The use of `standalone` as a default kind should only be used if
there is no active grove."

The owner's Ravel policy needs a route for `standalone` before a bare
`grove run` launches. Say so in the documentation. Do not change the owner's
settings.

## Decisions (running log)

- **Detection of an active grove.** The human chose the lifecycle's tree
  walk. A grove is active when the closest ancestor of the current directory
  that holds `.jj/` also holds `.grove/`. The walk reads the filesystem only
  and starts no jj process. Outside a jj workspace, no grove is active.
- **A bare `grove run` inside an active grove.** The human chose the kind of
  the next live leaf, as `grove-llm pick` finds it. The run gains no mandate
  and no tree authority.
- **Positional arguments.** The human chose the default only when no
  positional argument is given. One positional argument stays the kind, so a
  bare run takes its prompt from `--prompt-file`. Each present command line
  keeps its present parse.
- **Edge cases of the leaf kind.** A grove with no live leaf, or a tree that
  the reader refuses, makes a bare `grove run` refuse before any launch. The
  error names the grove root and tells the caller to give an explicit kind.
  The model decided this, because no leaf kind exists to route.
- **The walk without jj.** `Workspace::resolve` starts jj for a secondary
  workspace, so `jj-workspace` gained `Workspace::find_root`, the walk alone.
  `resolve` is now built on it and keeps its refusal.
- **The caller-surface inventory stays frozen.** 144 of its 441 rows were
  already stale before this leaf, and no check keeps it current. This leaf
  does not refresh it.
- **Verification.** `bash scripts/check.sh` ran under
  `mise exec bun@1.4.3`, because the policy typecheck pins Bun 1.4.3. All 21
  principal checks passed.
