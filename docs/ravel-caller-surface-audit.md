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

## Final caller acceptance — k45

Executable acceptance passes against independent installed Ravel 0.1.0 and
checkout-local Bun 1.4.2. The parent contract remains open: two current
documentation passages contradict the tested boundary. This matrix checks each
k5/k29 Done when item, including its inherited clauses. A passing suite and
unchanged source hashes do not establish correct prose.

| Contract item | Evidence and disposition |
|---|---|
| k5: independent resolver, version, confinement and ending | `dispatch/ravel.rs` selects/canonicalizes the first executable in absolute PATH and checks stable 0.1.x once. Both callers reuse it through `session_command`; runtime-read grants resolve before staging. Resolver and installed command tests pass with install/upgrade remedies. Only public CLI/stdout is consumed. |
| k5: caller assets, SDK, examples and provenance | Readable `assets/ravel-policy/` passes installed-SDK types and real-worker fake-agent cases. Creator lifecycle tests cover retirement, closing-child attribution, retries/remapping, refusal/declaration resume and invalid receipts. Ravel embeds no Grove adapter/store. k6 owns readable-asset packaging and compiler-seam relocation. |
| k5: independent fixtures and exact result readers | `ravel_activation` exercises both actual callers with different prefixes/versions, sibling/cwd/older/malformed decoys, remedies and missing/unsupported results. Copied installed reader/command fixtures use isolated PATH without host Bun/Node. Both runners accept exactly `grove-continue` after reap, separated from managed noise. All pass. |
| k5: lifecycle invariants and complete surface cutover | Continuation, completion, refusal/resume, cancellation/group cleanup, PTY recovery, confinement, epochs and stale sessions pass in workspace nextest. Four authority lists agree on ten old/new controls; actual boundary probes pass. Prompt/finish/Creator guidance retains the explicit v22 exception. **Incomplete:** policy ADR and usage recovery still describe retired behavior. |
| k5: runtime/caller docs, local runner, books, gate and release helpers | Public supervision/creator/worker ADRs and standalone spec describe the CLI boundary; Cargo metadata has no external local dependency path into Ravel. The local runner/guards remain. Grove-only release-note build argv, real confined fake-agent preparation and retained archive checks pass. All six books validate final/all. **Incomplete:** current policy ADR and refusal recovery require repair. |
| k29: complete classification and named exceptions | All 446 inventory rows were checked; non-coordination source hashes match. Only k44/k45 task notes changed after that snapshot. Six observed positive/cross-tree/mutation controls pass. **Incomplete:** file classification missed contradictory active paragraphs; they are not accepted historical/compatibility exceptions. |
| k29: remove staged allowances/locator and reconcile ADR/prose/skills | Both actual consumers use the independent resolver/builders/reader; version probes scrub authority. Finish/Creator and legacy instructions are tested. **Incomplete:** policy ADR and usage recovery remain unreconciled. |
| k29: release-note/prepare assumptions and retained archives | `release-notes.sh` builds Grove binaries without `dispatch:build`. `release-prepare.test.sh` asserts the exact argv/ending and actual confined Ravel invocation. The gate's release tests retain pre-k6 archive coverage. All pass. |
| k29: source corpora and repository gate | `task check` passes all 14 principal checks, 1,723 workspace nextest cases, nine doctests, strict types, release/archive checks, 21 conformance mutations and all six final/all books with zero deferred lines. Explicit installed acceptance also passes. |
| k29: complete parent delivery and closure | Runtime acceptance passes; current prose prevents closure. `ravel-caller-prose-repair-k46` owns the exact repair and fresh final acceptance before k29/k5 can close. k6/k12 retain their existing delivery/broader-prose work. |

The policy ADR's confinement paragraph (108–116 at this measurement) still
names `run --confine`, a run ID and a run record. Its considered options
(141–162) retain an installation subcommand, record-directory/store costs,
executable-supplied provenance and a shared repository/release route, conflicting
with its opening and the settled extraction. `docs/USAGE.md` 435–440 directs
the operator to run an `inspect:` line until it reports a command. The adjacent
example contains no such line; Ravel has no inspection verb. Exact passages and
source hashes are saved in `target/k45/acceptance-gaps.json`. These are active
caller instructions/decisions, not broader ownership prose excused until k12.
k46 reconciles them in place and refreshes the audit/inventory.

Complete logs are `target/k45/check.log` and `installed-acceptance.log`. The
latter runs `task ravel:fixtures ravel:commands ravel:review test:authority
RAVEL=/opt/homebrew/bin/ravel` with explicit `GROVE_TEST_RAVEL` and the pinned
Bun directory first on PATH. It passes 116 case executions, including all six
ignored installed resolver/reader/command cases and nine public-policy cases.
Across both logs, 1,738 unique nextest identities pass; the 101 shared identities
have individually equal outcomes in `per-case-comparison.json`. The workspace
run reports its 31 skipped cases; the named installed fixtures are explicitly
executed by the additional tasks.

`before.json`, `after.json` and `digest-comparison.json` compare all **17,062**
individually named subjects without additions, removals or changed hashes:
tracked source/config/task notes and complete book sources/indexes/manifests,
ignored worker dependencies, locked Cargo dependency source trees, measured
compiler/runner tools, retained generated worker/SDK files, complete original
22.1.0 Grove and independent 0.1.0 Ravel kegs, old/new owner configuration and
tracked adjacent Grove/Ravel/tap checkouts. Symlink targets also match. Four
generated hostile-probe subjects were frozen after build and before workspace
runtime, then compared unchanged. Adjacent checkout state records match too:
Ravel/tap remain clean successors of `3916a4a1`/`bd5aa37b`, and default Grove
remains unchanged at `89f947d5`. Evidence/handoff notes and the repair leaf
follow this measurement.

The inventory comparison, observed controls, authority-list comparison,
acceptance-gap pins, Cargo metadata and measurement instrument are saved under
`target/k45/`. Graph coverage was checked for all 1,970 tracked paths in 20
complete batches at generation `2026-10-13T09:21:18Z`; `coverage-summary.json`
retains limitations. Graph traces corroborate both resolver callers and result
reader consumers. `.cargo` is excluded, shell/Mermaid files have parse ranges,
and some non-code/binary artifacts have unavailable graph freshness. Material
source, including `.cargo` and reported shell/Mermaid ranges, was read directly.
No conclusion relies on an empty graph result or binary semantics.

No runtime change, personal-policy edit, installation, integration, publication,
new driver or teardown occurred. The original-v22 admission/ending contract
remains intact. This is executable acceptance with a named remaining repair;
it does not close the caller contract.
