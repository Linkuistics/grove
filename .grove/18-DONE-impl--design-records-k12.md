# design-records-k12

## Goal

Rework the design records that state the reversed decision, so that they
describe the shipped arrangement. The code and the books are final by now.

## Context

- The root brief: *Records to rework*.
- The shipped code: the new module of `grove-loop` and its interface, from the
  running logs of `ravel-job-k7`, `grove-run-job-k8` and `lifecycle-job-k9`.
- `docs/adr/the-launched-child-is-a-job.md` and
  `docs/adr/dispatch-supervises-the-harness.md` are already reworked. Read them
  so that no record contradicts them.

## Done when

- `docs/specs/module-decomposition.md`: decision 1, with its table and prose,
  describes the crates that remain. Decision 7 records the shipped interface.
- `docs/adr/one-live-driver-per-working-tree.md`: the passage on orphaned
  processes credits the guarantee of Ravel with the reap of the group. The
  "two generic runner events" are the two notices of the new module.
- The entry *Run ending* in `CONTEXT.md`, and the rows and paragraphs of
  `CONTEXT-MAP.md` on the crate, including "no surviving group".
- `docs/ARCHITECTURE.md` and `docs/grove-delivery-contraction.md` describe the
  arrangement without the crate.
- The current-discussion panel is removed from
  `docs/design/harness-selection-and-execution/diagrams.json`. The file parses.
- A search of the records that this leaf owns finds no statement of the
  reversed decision: `CONTEXT.md`, `CONTEXT-MAP.md`, `docs/ARCHITECTURE.md`,
  `docs/grove-delivery-contraction.md`, `module-decomposition.md`,
  `one-live-driver-per-working-tree.md` and `diagrams.json`. The pattern covers
  `keyed-launch`, `keyed_launch`, `survivor`, `surviving`, "group is gone" and
  "runner". Enumerate the matches, and classify each one. Show that the
  search finds a seeded match.
- Records that `user-records-k13` and `book-specs-k14` still own may keep their
  matches. List them by file in the running log.
- Every edited record passes the technical-communication checker. Convert each
  record as its own change before the edit. A large document converts only the
  sections that the edit touches.
- `bash scripts/check.sh` passes.

## Notes

- Describe the current arrangement on its own terms. Do not carry the old
  scheme as contrast. The ADRs and the changelog hold the history.
- Follow `docs/adr` rules for a changed decision: edit in place, and chase the
  citations. `ADR-FORMAT.md` of the `grove` skill states them.

## Decisions (running log)

- Each record landed as two changes: a conversion of the touched sections to
  controlled English, then the edit. Five of the six records were large, so each
  converted only the sections that the edit touches and carries `pending` markers
  for the rest. `grove-delivery-contraction.md` was small. It converted in full,
  and its dated k49 and k51 evidence sits in an `off` region.
- `module-decomposition.md`: decision 1 lists four library crates, and two of the
  modules are domain-free now. Decision 7 is "The job function". It records the
  shipped interface: `Argv` (in `session_command`), `Shape`, `LaunchEvent`,
  `JobEnd`, `JobError`, `launch`, `CONTROL_ENV`, `scrub`, `take_interrupt` and
  `reraise`. The explicit anchor `7--the-runner` stays above the renamed heading,
  because two released `CHANGELOG.md` entries link to it and a released entry
  never changes.
- `one-live-driver-per-working-tree.md`: Ravel's guarantee, not Grove's, ends and
  reaps the managed group. The orphan cases are a process outside that group and a
  harness that an uncatchable death of `ravel` orphaned. The "two generic runner
  events" are the `Started` and `Reaped` notices of `ravel_job`.
- `CONTEXT.md` *Run ending*, `CONTEXT-MAP.md`, `docs/ARCHITECTURE.md` and
  `grove-delivery-contraction.md` lost the crate, its row, its paragraph and the
  "surviving group" step. The ending precedence in `ARCHITECTURE.md` has four
  steps now. `diagrams.json` lost its `discussion` panel and parses. The viewer
  treats that key as optional.
- A record outside the brief states the reversed decision:
  `docs/adr/harness-selection-is-owned-by-policy.md` said "Grove keeps its own
  local runner". No leaf owned it, so this leaf converted and edited it.
- The first run of the search read clean for the wrong reason. The file list sat
  in an unsplit zsh variable, so grep opened one missing file. The seeded control
  returned 0 instead of 6, which exposed it. The array version returned 6.
- Search of `CONTEXT.md`, `CONTEXT-MAP.md`, `docs/ARCHITECTURE.md`,
  `docs/grove-delivery-contraction.md`, `module-decomposition.md`,
  `one-live-driver-per-working-tree.md` and `diagrams.json` for `keyed-launch`,
  `keyed_launch`, `survivor`, `surviving`, "group is gone" and "runner". The
  seeded file `docs/zz-seed-control.md` returned all 6 of its lines, then I removed
  it. The same pattern found matches in each of these records at the start of the
  session, the crate name among them. Each remaining match, classified:
  - The plugin's conformance runner, which is the skills module and not the
    process runner: `CONTEXT-MAP.md:259`, `CONTEXT.md:250` and `:259`,
    `module-decomposition.md` lines 93 and 842.
  - A retired launch configuration: `CONTEXT.md:845` names "the runner's
    *catalog*, *templates*, *slot* and *overlay*", terms that went with it.
  - Ravel's own runner: `grove-delivery-contraction.md:48`, `ARCHITECTURE.md`
    lines 345 and 474, `diagrams.json` lines 8 and 32.
  - Plain English "surviving" and "survivors" unrelated to process groups:
    `ARCHITECTURE.md` lines 53, 257 (a test of killed drivers with living
    children), 263, 308, 311, 683 and 800.
  - The kept anchor `7--the-runner` at `module-decomposition.md:328`.
- Records that `user-records-k13` and `book-specs-k14` still own keep their
  matches. `user-records-k13`: `docs/RELEASING.md`, `docs/USAGE.md` (lines 949
  and others) and `plugins/grove/skills/grove/references/driver.md` (lines 68 and
  70).
  `book-specs-k14`: `docs/specs/grove-loop-book-structure.md`,
  `overview-book-structure.md`, `grove-llm-book-structure.md` and
  `user-guide-coverage.md`, with the `grove-loop` book chapters 01, 16, 18, 19 and
  20 and its README and indexes.
- Left as correct: `docs/specs/standalone-invocations.md:81` and the prompt text
  `crates/grove-loop/src/prompt.rs:151` say that the group is gone before a result
  returns. Ravel gives that guarantee. The prompt is code, and its sentence stays
  true, so no leaf changes it.
