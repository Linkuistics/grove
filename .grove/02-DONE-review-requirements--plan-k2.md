# plan-k2

**Reviews:** plan-k1

## Goal

An adversarial read of the requirements `plan-k1` settled, before a design is
built on them: the root brief's *Settled requirements*, *Done when* and *Out of
scope*, against `plan-k1`'s decision log and the code and documents they
describe.

## Context

- **Report only what would change what gets built or how it is tested.** The
  human's rule for this grove is simplicity, and it covers this review: no
  wording polish, and no finding whose only remedy is more words in the brief.
- **Every decision in the log is the human's**, several against the producing
  session's recommendation and several deliberately at the level of a solution.
  A finding does not overrule one, and *a solution stated as a requirement* is
  not a defect here. What a finding can show: two requirements that cannot both
  hold, a fact about today's system stated wrongly, a consequence that changes
  the work and that nobody recorded, or a decision the brief dropped or changed
  on its way out of the log.
- **Where an early log entry and a later one differ, the later binds.**
  `plan-k1`'s *Notes* names the three that were refined.
- **The doubts the producing session could not close for itself:**
  1. *Requirement 5 removes the catalog, and the shipped review rule leans on
     one.* The review examples check a creator's origin against the catalog's
     provider values and name a reviewer by candidate ID. Does the rule still
     work when the only tool-level fact is the label in the run record?
  2. *Requirement 10 while dispatch still `exec`s.* Dispatch reads the policy,
     runs its worker and commits a run record before the handoff, outside the
     sandbox. Is anything left that a confined harness then cannot reach, and
     can a caller-neutral capability carry both Seatbelt and bubblewrap?
  3. *Requirement 11's parity.* Is it well defined for every kind in the pinned
     file, including the standalone `release-notes` route and arrangements that
     compose several profiles?
  4. *Statements of present fact.* The log and the brief say what today's
     system does in many places. Check the ones a requirement rests on.

## Done when

- Each doubt above carries a ruling.
- Findings are recorded as this kind's skill directs, each naming the
  requirement or log entry it is against and what it rests on.
- If a finding is worth acting on, an `integrate-review-requirements` leaf is
  cut where the walk reaches it before the design leaf.

## Notes

`plan-k1` was launched directly, not through harness-dispatch, so it had no run
to name and this leaf carries no `**Creator:**` line.

## Findings

No actionable findings against `plan-k1`'s committed requirements
(`78455f41aa23334adcea3520fabe6e10726c32d3`). The rulings below concern their
consistency and the work they require; they do not establish that the future
implementation meets them.

### 1. Catalog removal and the review rule — compatible

Against requirement 5 and the log entry *A successful `select` returns the
command itself*. Today's helper obtains the creator's origin from the recorded
run, checks membership in its own catalog's origin set, resolves a reviewer by
ID, then compares origins (`worker/examples/review.ts:116`, `:215`, `:263`,
`:273`, under `crates/harness-dispatch/`). The executable need not export a
catalog for an owner's function to retain its own allowed origins and command
options and compare the selected command's provider with the creator's recorded
provider. Requirement 5 expressly retains that recorded label. Removing IDs and
the tool's catalog validation therefore requires the example rewrite already
accepted in the log, not a new requirement or a Grove-specific core rule.
The pinned sample's static producer/reviewer pairing is also compatible with
this separate opt-in example; parity does not require making that sample
follow a recorded creator instead of its chosen arrangement.

### 2. Confinement with an exec handoff — compatible

Against requirements 10 and 14. Today's front reaps selection, commits the run,
then execs (`crates/harness-dispatch/src/run.rs:61`). Its record-store path is
exported as data; the harness need not open the store to execute its command.
Today's runner builds a sandbox command around the resolved program and
read grants, using Seatbelt or bubblewrap
(`crates/keyed-launch/src/confinement.rs:52`, `:105`, `:157`). Neither backend
requires policy evaluation inside the sandbox. A caller-neutral handoff can
therefore apply this boundary after selection without retaining a second
supervisor. The completion helper and channel are already staged inside the
invocation's private root (`crates/grove/src/standalone.rs:64`).

This is not permission to confine the dispatch front itself or grant the
personal policy/store to the harness. Selection's environment grants and the
harness's runtime read grants remain different boundaries: today's standalone
scrub keeps only a small environment (`standalone.rs:312`). Requirements 7, 8
and 10 require the former to work before the latter applies. Their design is
already assigned to `direct-dispatch-k3`.

### 3. Sample parity — well defined

Against requirement 11 and *The shipped sample converts the human's own
`config.kdl`*. The pinned file contains the active `routes` + `claude-led` +
`codex-sol` selection, four arrangements, two modifiers and `release-notes`.
The resolver's ordered profile application and route-over-shared specificity
define their outcomes (`docs/adr/complete-session-configuration.md`). Compare
the program and argument boundaries after supplying the same runtime values,
before adding any dispatch or sandbox wrapper. That includes the parameter
inside `model_reasoning_effort=…`, Claude's session name, and the repository
argument. The release-notes command is `/bin/bash codex-headless.sh …`, not
the interactive Codex command. Standalone values are `standalone:KIND` and
the staged working directory for both roots, with no task
(`crates/grove/src/standalone.rs:142`; its existing slot test at
`crates/grove/tests/internal/standalone.rs:107`).

Capturing resolver outcomes before deletion is already explicit in the root
brief. Requirement 11 calls for every arrangement and modifier the pinned
file defines, not just the active selection; the sample's offered combinations
can be checked against that same source. No new configuration language is
needed to make the comparison meaningful.

### 4. Present facts and decisions carried into the brief — no defect found

Requirements 5–7 reverse real current limits: the request omits the prompt
(`crates/harness-dispatch/src/choice.rs:460`), and selection is currently
30 seconds by default with a 120-second ceiling (`src/cli.rs:240`). All seven
lifecycle expansion values in requirement 6 exist
(`crates/grove-loop/src/session_config.rs:278`). The current configuration
admission and pre-authoring checks are stated in the configuration ADRs and
the dispatch spec; requirement 9 intentionally removes them. The brief keeps
the later command-return and explicit-install decisions, the final narrowing
of measured reads and surviving owner settings, the four agreed test seams,
and the major-release decision. No dropped decision or conflicting acceptance
condition was found in this bounded read.

## Review evidence

Inspected the producer commit, decision log, root brief, pinned configuration,
the cited ADRs and relevant dispatch, standalone and runner source. Used Tier 2
graph discovery and snippets, generation `2026-10-02T02:27:48Z`, with exact-path
coverage checks. The fast index excludes `docs/` and
`crates/harness-dispatch/worker/examples/`; those materials were read directly.
Some graph call edges around platform-specific confinement were unhelpful, so
the backend source is the evidence for that ruling. Coverage is best effort,
not proof of repository completeness. No tests, builds, lint or format commands
were run, as this review kind requires. The producer's recorded passing checks
are not evidence that the proposed behavior is implemented.

## Decisions (running log)

**The four doubts do not earn an integration leaf (2026-10-02).** Each is
compatible with the human's final decisions and assigned design scope. Record
the rulings here, leave the requirements artifact unchanged, and continue to
`direct-dispatch-k3`.
