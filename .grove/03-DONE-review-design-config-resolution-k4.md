# config-resolution-k4

**Reviews:** config-resolution-k2

## Goal

Read the reworked `docs/adr/complete-session-configuration.md` adversarially,
before `local-config-kdl-k3` writes code against it. The producer could not
judge its own prose against the one failure mode this rework was warned about,
and the minimum-coherent-set calls it made were made without a second reader.
Findings, not fixes.

## Context

`config-resolution-k2` rewrote the binding statement wholesale and took the
*Considered options* from six entries to ten. Everything it changed is prose, so
`cargo test` proves only that the corpus still parses, the rules still have one
owner, and the loaded paths still fit — nothing about whether the record is
coherent, honest or minimal. The producer's own `Decisions (running log)`
carries its reasoning; read that after forming a view, not before.

**Four specific doubts**, in the order they are most likely to be real:

1. **Does the record defeat the skimming reader?** The producer was told a
   reader who skims will assume the precedence lattice came back, and answered
   with "search order is not merge order" in the binding statement plus a
   revised *Keep a primary harness and layer kind/family overrides over it*
   entry. Whether that lands is a reader's judgement and the producer is the
   worst judge of it. Read the record cold and report what you think resolution
   does before you check.
2. **Is the set still minimum and coherent, or did it bloat?** Four options
   are new — *Let a second file replace the personal one*, *Track the delta*,
   *Put the delta inside `.grove/`*, *Warn and fall back*.
   `linkuistics:decision-records` admits a rejection only when it is non-obvious
   *and* someone will otherwise re-propose it, and each needs a reopen trigger
   that is a gate rather than a tombstone. Say which of the four fail that bar.
   The three untouched entries — shell execution, research-target comparison,
   inline methodology — should be checked for whether the rework left them
   still true.
3. **Should the ADR have been split?** The producer argued against, on the
   grounds that opacity and two-file resolution are one claim seen from two
   sides. The contrary reading is available: a record that must state a *search
   order* and a *placement rule* and a *trackedness rule* alongside *Grove
   infers nothing* may be two decisions sharing a slug. If it should split, name
   the two records and which citations move.
4. **Does the record claim behaviour that does not exist yet?** The rework was
   deliberately allowed to state the *decision* ahead of the code, while
   `docs/CONFIGURATION.md`, `docs/ARCHITECTURE.md` and `CONTEXT.md` wait for
   `local-config-kdl-k3`. Check the line held: an ADR describing what the binary
   *does* today, in the present tense, about a `.grove.kdl` nothing reads yet,
   is over it.

Two smaller ones. The producer also edited `content/references/driver.md` — it
deleted the *no repository stamp* clause it was told to, and additionally
weakened "launched by `~/.config/grove/config.kdl`" to "launched from personal
configuration" and changed "no default, family or **fallback**" to
"…or **inheritance**". Neither extra edit was asked for; judge whether each is
a correct reading of *a clause rescued by its neighbour* or scope the leaf did
not have. And the producer edited `.grove/BRIEF.md` to hand two unowned
reconciliations (`CONTEXT.md`'s **Grove configuration** entry,
`content/references/decompose.md`'s "two entries") forward to `k3`; judge
whether the brief was the right instrument or a question for the human.

## Done when

- Each of the four doubts above has an explicit verdict, and each verdict is
  argued from the record's own text rather than from the producer's log.
- Any finding carries the artifact, the location, and what specifically is
  wrong — enough that an integrating session need not re-derive the intent.
- The record is read against its neighbours.
  `docs/adr/supported-workspace-layouts.md` it now cites twice.
  `docs/adr/task-tree-transactions-fail-closed.md` it does **not** cite, yet it
  describes that record's teardown evacuation and finish-commit pathspec as the
  reason the delta sits beside `.grove/` — decide whether that needs a citation
  or whether one would be the cross-reference chain the format warns against.
  `docs/adr/one-build-owns-a-session.md` cites it twice and was left untouched;
  confirm both citations still hold.
- A review that finds nothing worth acting on **creates nothing** and simply
  retires. Cut `integrate-review-design` only if there are findings to act on,
  and insert it where `pick` reaches it next rather than appending it after
  `local-config-kdl-k3`.

## Notes

No code, and no edits to the ADR. This is the adversarial read; the fixes belong
to the integration step.

The producer spent no in-session reviewer, so nothing here has been pre-checked
by a second context.

## Review result

### Cold read

Before reading the producer's log, I read the record to mean this: Grove first
searches for one delta, at the worktree root and then the main repository root;
it never combines those two candidates. If a delta exists, each kind named in it
uses that file's whole template and every omitted kind uses the personal file's
whole template. No word, flag, or fragment is merged. The opening paragraph at
`docs/adr/complete-session-configuration.md:3`-`16` therefore defeats the
precedence-lattice misread; **doubt 1 passes**.

### Findings

**F1 — `docs/adr/complete-session-configuration.md:37`-`42` claims an
unenforced security property (high).** Calling `.grove.kdl` "untracked" does
not make it untrackable. A repository can commit that exact path, and ignore
rules do not apply to an already tracked file. The planned implementation in
`local-config-kdl-k3`, §Done when (the resolution and documentation bullets),
searches the two paths and tells the user to add an ignore line, but contains no
trackedness check; the existing
`SessionConfig::load` seam at `src/session_config.rs:75`-`95` has no VCS input
from which it could establish one. Such a checkout can therefore ship a
`.grove.kdl` that selects an executable, contrary to both requirement 1 and the
record's statement that this is "the whole security story." Integration must
either add a design and testable seam that refuses tracked candidates in every
supported VCS layout, or weaken the statement to an operator convention and ask
the human to accept the resulting repository-code-execution risk; documentation
and `.gitignore` guidance alone cannot establish the current claim.

**F2 — `docs/adr/complete-session-configuration.md:3`-`59` contains two
independently reversible decisions, so the record is not the minimum coherent
set (medium).** Whole-template opacity and no harness inference are one decision.
The untracked delta's lookup order, placement, lifetime, validation posture, and
security boundary are another: any of those source-policy choices can change
without allowing partial command templates, and opacity can change without
moving `.grove.kdl`. The second decision independently clears the ADR admission
test and carries four of the new alternatives. Split the set into
`complete-session-configuration` (opaque, complete per-kind value; no hidden
launch inference) and, for example, `worktree-local-configuration-delta`
(candidate search, per-kind selection, personal-file completeness interaction,
trackedness/security, fail-closed behavior, and placement). The two citations in
`docs/adr/one-build-owns-a-session.md:74`-`75` and `:152`-`154` rely only on
opacity and remain on `complete-session-configuration`; the delta record should
own the replace-vs-override, tracked-delta, inside-`.grove/`, and warn/fallback
options. One explicit cross-citation for the completeness invariant is coherent;
keeping both reversible axes in one 144-line record is not.

**F3 — `docs/adr/complete-session-configuration.md:136`-`144` describes a
retired delivery design (medium).** `${prompt}` does not carry a
"kind-selected slice set". `docs/adr/skill-delivers-the-methodology.md:3`-`9`
says the whole methodology is provisioned as a skill and `${prompt}` carries a
short guaranteed core pointing to it. That decision also owns the surviving
reason complete inline delivery is rejected; the configuration ADR's
"specificity rather than size" rationale predates the reversal. Delete this
option from the configuration record or reduce it to a current, accurate
citation if it remains a genuine alternative to launch configuration.

**F4 — `content/references/driver.md:23`-`24` is not a bridge sentence that
survives the implementation (medium).** "Personal configuration — it lives at
`~/.config/grove/config.kdl`" still locates all personal configuration in the
home file, while the settled design makes the worktree/repository delta personal
launch policy too and lets it supply the selected kind. The sentence is true of
the current binary only under the narrow reading the producer was trying to
replace, and becomes false or ambiguous when `local-config-kdl-k3` lands. Keep
this procedure current until the code lands, then make the implementation leaf
reconcile the whole paragraph; changing "fallback" to "inheritance" at
`content/references/driver.md:32`-`33` is correct and needs no reversal.

**F5 — the placement rationale cites the wrong neighbour and omits the one it
restates (low).** `docs/adr/complete-session-configuration.md:43`-`46` repeats
the finish pathspec and evacuation mechanics owned by
`docs/adr/task-tree-transactions-fail-closed.md:16`-`22` and `:33`-`56` without
citing that record. A direct citation is not a cross-reference chain; it is the
source of the non-obvious reason `.grove.kdl` cannot live inside `.grove/`.
Conversely, the link at
`docs/adr/complete-session-configuration.md:50`-`52` says
`supported-workspace-layouts` distinguishes the worktree and main-repository
configuration roots, but that record distinguishes working-tree roots from VCS
control directories for teardown (`docs/adr/supported-workspace-layouts.md:19`-`36`),
not the `${worktree}`/`${repo}` substitution semantics. Cite the direct owner of
those root meanings or let the configuration-delta record define them without
that attribution. The second `supported-workspace-layouts` citation, at
`docs/adr/complete-session-configuration.md:117`-`121`, does support the shared
"no advisory channel" argument and remains valid.

### Explicit verdicts on the remaining doubts

- **Doubt 2 — the four new options:** all four earn a place, but in the proposed
  delta record. Replacement-vs-override is an explicit branch from the original
  issue; tracked policy is the obvious convenience that violates the executable
  trust boundary; inside-`.grove/` is tempting until the transaction mechanics
  are known; warn-and-fallback is a plausible availability choice. Each has a
  concrete reopen gate. The unchanged shell-execution and research-target
  comparison entries remain true. The unchanged inline-methodology entry does
  not, as F3 records.
- **Doubt 3 — split:** yes, for F2's independent reversibility and grain, not
  merely because the record is long.
- **Doubt 4 — decision ahead of behavior:** no finding. The ADR is the binding
  design the sequenced implementation will satisfy, so present-tense decision
  language may lead the binary. Current-behavior artifacts correctly remain
  unchanged until code lands. The source confirms the gap rather than turning
  it into an ADR defect: both current load points call the personal-only
  `SessionConfig::load` at `src/loop_driver.rs:119` and `:127`.
- **Brief handoff:** correct. The root brief is the subtree contract every
  remaining leaf bootstraps, and the human had already settled the
  reconcile-every-current-claim requirement. Adding the two missed artifacts to
  its `Done when` and pointers repaired ownership without rewriting a sibling
  task or inventing a new decision.
- **Existing inbound citations:** both citations from
  `one-build-owns-a-session` still hold because each depends on opaque complete
  launch policy, not the number of source files. `CONTEXT-MAP.md`'s ownership
  citation also remains valid for the surviving slug.

The producer commit records no command output as verification evidence. This
inspection-only review did not rerun tests; the absence does not change the
design findings above.
