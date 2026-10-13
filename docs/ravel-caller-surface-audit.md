# Grove's Ravel caller surface audit

This is k44's bounded caller handoff. Active lifecycle and standalone calls use
independent Ravel's public two-verb CLI, canonical ending instructions and held
stdout result. Grove owns result meanings, task conventions, teardown and inline
provider receipts. The complete repository still retains delivery work for k6,
broader ownership prose for k12 and the original live-driver contract for k13.
Those are named work boundaries, not a claim that old vocabulary has disappeared.

## Inventory and evidence

[The per-file inventory](ravel-caller-surface-inventory.tsv) names every candidate
file, classification, exact candidate lines, namespace/control tokens and source
SHA-256. It includes binary fixtures separately. `jj file list` supplies the whole
tracked set, including root manifests, dotfiles and symlinked instructions.
Generated audit products have explicit evidence rows and are excluded from
self-enumeration; frozen verification includes their bytes.

The instrument first extracts every lexical token from every tracked text file,
then selects process/router namespaces, Grove controls, CLI flags and review
markers for classification. It retains a complete lexicon plus exact line/token
records in `target/k44/lexicon-final.json` and `inventory-final.json`.
Other CLI/library/plugin vocabulary is classified separately, rather than
silently dropped by a preselected directory or file list. This is a source-text
audit, not a claim about identifiers inside binary fixture payloads.

The graph was verified at generation `2026-10-13T06:40:55Z`. The bounded finish
trace identifies `new_finish_leaf` as the caller of `finish_body`; its actual
materialization and lifecycle tests supply the consumer evidence. Coverage is
best-effort. The graph excludes `.cargo`; Mermaid views and two shell files have
reported parse ranges. The inventory reads complete tracked source directly,
including those ranges. No negative conclusion relies on graph absence.

Observed controls in `target/k44/observed-controls.json` exercise the same
`rg --hidden -n` instrument. The repaired caller yields its known Ravel ending;
an impossible mutation fails the positive expectation. The removed inspection
command remains discoverable in the retained old package and an intentionally
wrong scratch subject, then disappears when that subject is corrected. A clean
current skill therefore has both a dirty cross-tree control and an observed
mutation, rather than an untested empty search.

## Current caller corrections

- The finish sentinel directs the session to its launch prompt's exact ending
  command, with current Ravel and explicit original-v22 examples. A regression
  run failed on the old hardcoded instruction before the repair.
- The configure-grove skill uses personal Ravel policy/settings, `.ravel-choice`,
  readable Grove assets and fake-agent verification. It teaches no inspection or
  installation verb. A single reference-testing context failed the old workflow
  and passed the repaired workflow without executing a model.
- The caller ADR set and standalone specification describe public stdout,
  first-publication semantics, independent PATH/version trust and caller-owned
  provenance. Private runtime decisions/evidence link to Ravel's pinned v0.1.0
  documents. Grove keeps its local runner without a shared Rust dependency.
- Current fixture comments, names and assertions describe the independent
  installation. The policy-worker environment assertion also rejects Ravel
  controls. Retirement reminders check the current result command as well as
  preserving the explicit legacy example.
- `dispatch:check` now uses nextest for the retained binary package's runtime
  suite; it has no library doctest target. The root gate retains workspace
  doctests. `grove-loop` keeps `serde_json` only as a dev
  dependency for public installed-session fixtures.
- Source-book literal fragments, manifest roots/ranges, ownership and fragment
  indexes, rollups, explanatory examples and concept indexes are reconciled.
  The local runner's appearance-only Channel examples use inert `TEST_CHANNEL`;
  they make no claim to implement Ravel's private result transport.

## Individually named current exceptions

The inventory names every retained delivery, historical and broader-prose file.
The following mixed current surfaces require line-level interpretation:

| Surface | Retained meaning |
|---|---|
| `.cargo/config.toml` | Force-clear original/new admission, ending, PID and provider-evidence authority; explicitly preserve the original launch-prompt ending. |
| `testing/support.rs` | Both control sets are scrubbed. `harness_dispatch()` is the actual retained old front helper for old-package/removed-surface fixtures; installed Ravel fixtures use their independent helper. |
| `crates/grove/src/standalone.rs`, `crates/grove-loop/src/loop_driver.rs` | Legacy environment names remain in the ten-control scrub, alongside `RAVEL_EXIT_FILE` and `GROVE_PROVIDER_EVIDENCE`. |
| `crates/grove/src/dispatch/ravel.rs` | An old-tool version string is deliberately rejected by the version parser. Its version probe uses the caller scrub list. |
| `crates/grove-loop/src/prompt.rs`, `crates/grove-loop/src/tree_lifecycle.rs`, `crates/grove-llm/src/cli.rs` | Original installed-v22 ending guidance is explicitly conditional on that live launch contract. |
| `crates/grove/tests/env_hygiene.rs`, `crates/grove-loop/tests/session_result_installed.rs` | Old/new authority values are hostile inherited inputs whose removal is observed. |
| `crates/grove/tests/loop_driver.rs` | Retired run/store controls and private `--ending-file` are rejected. Old JSON ending observations are unsupported-result fixtures, not accepted reports. |
| `crates/grove/tests/ravel_activation.rs`, `crates/grove/tests/standalone.rs` | Old sibling executables and a legacy store are decoys/unreadable-file probes. No active lookup or store access is implied. |
| `crates/grove/tests/lifecycle_cutover.rs` | A pre-launch control-directory failure must not contain old/new launch diagnostics. |
| `crates/grove-llm/tests/removed_surface.rs` | The old front provides the retained metadata-diversity fixture, not current launch routing. |
| `crates/grove-llm/tests/composition_guidance.rs`, `crates/grove-loop/tests/prompt.rs`, `crates/grove-llm/tests/leaf_ops.rs` | Assert explicit legacy guidance or reminder absence/data separation. |
| `crates/grove/tests/support/session-probe.c` | Names the old package probe as the historical reason to use C; the current witness is Ravel-based. |
| `plugins/grove/skills/grove-finish/SKILL.md`, `grove/references/driver.md`, `grove/references/retire.md` | Paths under `plugins/grove/skills/`; preserve only the original live-v22 ending and Creator-run compatibility. Current Creator records use caller receipts/declarations. |
| `docs/USAGE.md`, `docs/grove-provider-evidence.md`, `docs/adr/a-review-carries-its-creator-reference.md` | Explicit original-v22 exceptions and rejected legacy provenance forms. |
| `docs/walkthroughs/overview/05-what-the-call-reaches.md` | Actual old workspace-package row is labelled pre-k6 archive/build compatibility; active callers resolve Ravel independently. |
| `docs/walkthroughs/overview/06-standalone-invocations.md`, `grove-loop/18-the-core.md`, `grove-loop/19-the-loop.md`, `grove-loop/14-finishing.md`, `grove-llm/05-ending-work.md` | Paths under `docs/walkthroughs/`; exact source literals retain scrubs, rejection fixtures and conditional live-v22 guidance. |
| `Taskfile.yml`, `scripts/check.sh` | Retained old worker/probe/type/archive and policy-compatibility recipes remain executable until k6. Their presence does not select the caller front. |
| `AGENTS.md`, `CLAUDE.md` | Original live-driver/release instructions; no running session authority is rewritten. |

All four authority lists agree on the same ten names: both retired signal paths,
Grove launch admission, legacy exit/run/store controls, Ravel exit context,
provider evidence and both retired PID handles. The resolver scrubs through the
standalone list; direct fixtures use the shared support list; Cargo clears its
own boundary independently. Final authority checks remain part of k45 acceptance.

## k45 and later handoff

`scripts/release-notes.sh` builds only the Grove caller binaries and resolves
installed Ravel through `grove run`. `release-prepare.test.sh` asserts that exact
build argv, exercises a real confined Ravel session with a fake agent and checks
the public ending command. No dispatch build prerequisite was reintroduced.
Old archive recipes are unchanged; k43's full gate retains their evidence and
k45 reruns the final gate. This leaf performs no build publication or release.

k45 must freeze complete subjects and run `task check`, `task ravel:review`,
`ravel:fixtures`, `ravel:commands` and authority acceptance against explicit
installed Ravel 0.1.0, then reconcile every parent k5 criterion before closing
k29/k5. The per-file `ownership-prose-k12` rows identify the exact broader
README/architecture/release/glossary/spec/design work still assigned to k12;
the `retained-delivery-k6` rows identify the old package/archive/formula work.
Neither set is mislabeled current Ravel behavior or completed here.

Complete k44 test logs, red/green outcomes, source classifications, measurement
subjects and before/after comparisons live in `target/k44/`; the terminal task
handoff names the exact runs. The original 22.1.0 Grove keg, independent Ravel
0.1.0 keg, personal Ravel policy and tap formula are included in frozen digests.
