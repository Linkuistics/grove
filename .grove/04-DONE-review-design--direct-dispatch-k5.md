# direct-dispatch-k5

**Reviews:** direct-dispatch-k3

## Goal

An adversarial read of the design `direct-dispatch-k3` delivered, before
`direct-dispatch-k4` cuts it into increments: the reworked
`docs/specs/harness-selection-and-execution.md` and
`docs/specs/standalone-invocations.md`, the reworked
`docs/adr/harness-selection-is-owned-by-policy.md` and the smaller edits to
three other records, against the root brief's settled requirements and the code
the design changes.

## Context

- **Report only what would change what gets built or how it is tested.** The
  human's rule for this grove is simplicity, and it covers this review: no
  wording polish, and no finding whose remedy is more mechanism than a
  requirement asks for. A finding that the design builds *too much* is in scope.
- **The requirements are the human's and are not under review.** What is under
  review is whether the design meets each one, and whether two of its answers
  can both hold.
- `direct-dispatch-k3`'s decision log says what each call was made against.
- **The doubts the producing session could not close for itself:**
  1. *Confined `grove run` through `inspect`.* Requirements 1 and 10 say every
     standalone invocation is launched by Grove running harness-dispatch,
     through a caller-neutral capability. The design selects with
     `inspect --json` and has Grove's runner launch the reported command, so
     dispatch selects and does not launch, and no run is recorded. Is that an
     honest reading of both requirements, and of requirement 14? Does the
     reported `command` give the runner everything its confinement needs, given
     that the runner resolves the program again for its read grant
     (`crates/keyed-launch/src/confinement.rs`)?
  2. *Owner settings are one global file.* Requirement 8 says what an owner sets
     per invocation today through flags in a command definition stays settable.
     A command definition could differ by kind; the settings file cannot. Does
     that lose anything an owner needs, a longer bound for one kind included?
  3. *The records left for the implementation to delete.* The two configuration
     ADRs and the modular-configuration spec stay until the machinery goes, so
     the ADR set briefly holds a record the reworked one contradicts. Is the
     deferral sound, and is `direct-dispatch-k4`'s list of what goes with them
     complete?
  4. *The launch document stays version 1 with nulls.* A new run writes `null`
     for the candidate ID, the selection form and the explicit choice. Does any
     reader of a run (`record show`, `record observe`, run lookup, the review
     example) break on that, or on a 21.13.0 record?
  5. *What the tool no longer checks.* The command validates the returned
     command's shape and nothing else, and inspection without a prompt evaluates
     with a marker. Is anything a requirement relies on now unchecked?
  6. *Parts of the specification this session did not rewrite.* Its runtime
     discovery, bounded context and delivery sections were edited only where a
     sentence named the catalog, the choice or the withheld prompt. Does any
     sentence left standing contradict the new contract?
  7. *The sample's selections.* Four arrangements, two modifiers, a default, and
     a choice file that replaces the default whole. Is parity with the pinned
     configuration well defined for every selection the capture enumerates,
     `codex-sol`'s route patches under an arrangement whose reviews are not
     Codex's included?

## Done when

- Each doubt above carries a ruling.
- Findings are recorded as this kind's skill directs, each naming the
  requirement or the design statement it is against and what it rests on.
- If a finding is worth acting on, an `integrate-review-design` leaf is cut
  where the walk reaches it before the planning leaf.

## Notes

`direct-dispatch-k3` was launched directly, not through harness-dispatch, so it
had no run to name and this leaf carries no `**Creator:**` line.

## Findings

Review of `direct-dispatch-k3`'s committed design,
`4679d99f2aa9f4388ae76c26c31b9d508e5690a6`. Three findings need integration
before planning. The existing code has not yet been migrated; it is evidence
about the seams the proposal must accommodate.

### F1 — Dispatch selects standalone work but does not launch it [P1]

**Against:** root brief requirements 1, 10 and 14; dispatch spec
`docs/specs/harness-selection-and-execution.md:1015`, also
`docs/specs/standalone-invocations.md:13`.

The proposed `grove run` invokes `inspect`, takes its command as data and has
Grove's runner launch the harness. `inspect` expressly launches nothing
(dispatch spec, lines 69–72). This is selection through dispatch, but requirement
1 says every standalone invocation is **launched** by Grove running dispatch.
The distinction is confirmed by `plan-k1`'s decision *Standalone `grove run`
launches through dispatch too* and the previous review's ruling 2: selection
and confinement must be separated through a caller-neutral handoff, with only
the harness confined. Neither accepted an inspection-only exception.

The producer rejected an exec-prefix because confinement needs the resolved
program and policy grants must survive selection. Those are real constraints,
but do not establish that dispatch cannot launch after resolving and before
applying confinement. Inspection can remain a general capability; it cannot be
the sole standalone launch integration under the settled requirement.

**Smallest useful correction:** specify a caller-neutral launch handoff that
lets dispatch launch the selected command through the confinement boundary,
with selection outside it and no duplicate policy evaluation. Keep the exec
handoff and existing domain-free runner; do not take on the later supervisor
design. Reconcile the standalone spec, ADR's inspection choice, diagram and
planning notes. Test the real front/worker and confined harness on this path,
including owner grants, refusal and cancellation. Whether that handoff records
a run follows its chosen contract; absence of a standalone record is not
independently reported as a requirement violation.

### F2 — Global settings discard per-invocation owner control [P2]

**Against:** root brief requirement 8; dispatch spec
`docs/specs/harness-selection-and-execution.md:275` and Grove integration at
`:1006`.

An owner can currently put different dispatch flags in different Grove command
definitions and route kinds to them. The proposed settings file has only four
global values, and Grove passes no setting flags. It cannot express a 30-second
bound for a table-only kind and a 300-second bound for an agent-selected kind.
Raising the global bound lets the latter run, but loses the former's short
failure bound. Different policy environment grants, context budgets or record
directories have the same problem. Re-exporting an entry from `policy.ts`
cannot change grants or the bound covering its imports, as the design itself
explains at lines 299–301.

**Smallest useful correction:** retain owner control based on invocation data
before worker start, with a global fallback and explicit flag precedence. A
literal kind-specific override is sufficient for the counterexample; no Grove
routes, profiles or configuration compiler are required. State how record
commands find a nondefault store. Add a no-flags test with two kinds receiving
different effective bounds and grants, so preserving a global default alone
cannot pass requirement 8's acceptance.

### F3 — The confined handoff leaves a second executable lookup unspecified [P2]

**Against:** root brief requirements 6 and 10; dispatch spec
`docs/specs/harness-selection-and-execution.md:97`, `:185` and `:1023`; the
statement at `:1043` that the runner's behavior is unchanged.

Inspection reports both the returned `program` and resolved `executable`, but
the design does not specify which the runner executes and grants. Today's
`keyed-launch::confinement::command` resolves `argv.program()` again, uses the
resolved path for its read grant, and passes that path to the sandbox command
(`crates/keyed-launch/src/confinement.rs:52`). Its `executable` helper takes the
first regular file on PATH without checking executable permission and resolves
relative paths against Grove's process cwd (`:88`). Dispatch instead skips
unexecutable PATH entries and resolves relative paths against the explicit
caller cwd (`crates/harness-dispatch/src/program.rs:83`). For standalone
selection that cwd is the staged working directory; Grove need not have changed
its own cwd.

An unexecutable `tool` in the first PATH directory and an executable `tool` in
the second produces a successful inspection but a failed confined launch if
the raw program is forwarded. A selected `./tool` staged in the invocation can
instead resolve against the parent project, failing or running a different
file. Merely exposing `executable` does not settle its consumption, and
changing only the Argv constructor does not fix it.

**Smallest useful correction:** make the confined handoff consume the resolved
executable for both launch and its read grant, without repeating a name lookup;
state the treatment of the returned program as argv[0]. Keep that value-based
interface domain-free. At the real confinement seam, verify a staged relative
program, relative/empty PATH entries and a nonexecutable PATH shadow. This
remains necessary when F1's launch path is corrected.

## Rulings on the seven doubts

1. **Confined launch through inspection — actionable, F1 and F3.** Selection
   outside the sandbox is right; inspection alone drops the launch obligation,
   and the runner needs an explicit resolved-command contract. Absence of a
   record is a consequence of that path, not a separate demand for bookkeeping.
   Requirement 14 does not require a supervisor here.
2. **One global settings file — actionable, F2.** Personal authority and
   pre-worker loading are supported. Global-only values do not preserve what
   per-kind command flags can express.
3. **Deferred configuration-record deletion — accepted migration tradeoff.**
   The code still implements the old configuration. Keeping its records until
   the deletion increment avoids dangling code/book citations; the design log
   and planning notes clearly delimit that interval. The reconciliation list
   covers the known consumers in this bounded read. It is not proof of an
   exhaustive citation sweep: root requirement 2 and the planning task still
   require enumerating all consumers when deletion lands. No extra ADR or
   producer leaf is warranted merely to remove the records early.
4. **Version-1 records with nulls — supported by the specified rewrite.**
   `record::readable` checks object/version and launch-failure cause
   (`crates/harness-dispatch/src/record.rs:161`); text export renders null as
   `none` (`:347`). Observation import validates its run through the store's
   shared reader (`src/observation.rs:146`, `src/store.rs:536`). Current lookup
   requires a string candidate ID (`record.rs:185`), but flattening its answer
   and requiring only kind, task identity and labels is explicitly part of the
   design and planning handoff. The SDK and review example currently use
   `lookup.candidate.provider`; both are assigned rewrites. Preserve stored
   labels in their existing location and project the new lookup from them.
   Exercise new null-bearing runs as well as a 21.13.0 creator in the specified
   round-trip cases. No launch-version bump or destructive store migration is
   necessary. This is backward reading by the new release, not a promise that
   the old lookup reads new null IDs.
5. **Removed checks and marker inspection — accepted contract change.**
   Requirement 5 deliberately removes catalog validation and prompt-slot
   enforcement. Prompt omission and marker evaluation are visibly reported;
   they are not evidence of a prompt-dependent policy's real selection. The
   deciding-agent guidance and scripted test retain the agreed trust boundary
   without rebuilding the catalog's safeguards. No further mechanism is
   justified by this doubt.
6. **Unrewritten discovery/context/delivery sections — no further behavioral
   finding supported.** Runtime authority, worker cancellation, measured reads
   and installed-layout controls can survive the command-return contract.
   The spec's assertion at lines 164–165 that context cannot become arguments
   cannot be credited as an enforced guarantee: trusted `select` can construct
   arguments from its context. The explicit shape-only contract governs. That
   leftover explanation warrants no additional validation mechanism and is
   not elevated to a wording-only finding. F2 and F3 are the material interface
   gaps found in this bounded read.
7. **Sample selection parity — well defined.** The pinned resolver applies
   ordered profiles with route-specific values winning over shared values
   (`docs/adr/complete-session-configuration.md:16`). Thus `codex-sol`'s three
   effort patches also apply when those reviews use Claude. The sample must
   reproduce that result, not repair the modifier's historical name or comment.
   Planning already requires capture before deletion for each arrangement
   alone and with each modifier combination, default `claude-led` + `codex-sol`,
   with `routes` supplied for the two arrangements that omit it
   (`direct-dispatch-k4`, lines 71–80). A choice replaces the default whole;
   omitting `codex-sol` therefore removes its patches. Include the distinct
   `/bin/bash codex-headless.sh …` release-notes command in the same capture.

## Review evidence and limits

Inspected the producer commit, root requirements, producer decision log,
previous requirements review, planning handoff, pinned configuration, both
specifications, cited ADRs, launch diagram and relevant dispatch/runner source.
Applied `codebase-design`: policy/front responsibilities, explicit state
ownership and the four existing test seams are supported; caller obligations
and executable-resolution ownership have the unresolved F1–F3 gaps; the
exec/unknown-outcome contract, label trust and deferred deletion are accepted
tradeoffs. The correction should reuse the resolved command and existing runner
rather than add a second resolver, supervisor or catalog.

Tier 2 graph project
`Users-antony-Development-grove.use-harness-dispatch-directly-for-execution`,
generation `2026-10-02T02:27:48Z`. Used symbol discovery, bounded call traces and
exact source snippets, checking coverage for every evidence path. Relevant
Rust source metadata matched with no recorded gap. The fast index excludes
`docs/` and worker examples; those were read directly. New task files were
untracked in the index and root/glossary metadata changed, so their current
bytes were read directly too. Some trace edges were unrelated name matches;
the cited source, rather than those edges, supports the conclusions. Coverage
is best effort, not proof of repository completeness.

No tests, builds, lint or format commands were run, as the review skill requires.
These rulings establish design compatibility where stated, not passing
implementation behavior. Integration owns fixes and subsequent verification.
No production, test, spec or ADR fixes were made here.

## Decisions (running log)

**Inspection alone does not discharge the standalone launch requirement
(2026-10-02).** Root brief requirements 1 and 10 bind together: dispatch must
launch, selection must precede confinement, and only the harness is confined.
The proposed inspection-only path satisfies the second boundary but drops the
first. Record F1; leave the capability's design to integration.

**The settings contract must retain per-kind control (2026-10-02).** Requirement
8 preserves what flags in separate command definitions can set today. One
global default cannot represent a short bound for a table-only kind and a
longer bound for an agent-selected kind, or different grants and stores. Record
F2 without prescribing a configuration composition system.

**Executable resolution needs one owner across confinement (2026-10-02).**
Dispatch's resolution and the runner's existing lookup are different algorithms
and use different cwd inputs. The design reports the resolved executable but
does not say how the runner consumes it. Record F3 with concrete counterexamples
and a test obligation, rather than requesting a new resolver.

**Integration runs before planning (2026-10-02).** F1–F3 change what is built
or tested and earn `direct-dispatch-k6`. Insert it immediately after this review,
ahead of `direct-dispatch-k4`; its charter references this review without copying
the findings, so integration can reject a finding on the evidence.
