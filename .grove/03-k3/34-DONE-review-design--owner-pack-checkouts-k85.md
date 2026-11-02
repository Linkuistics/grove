# owner-pack-checkouts-k85

**Reviews:** owner-pack-checkouts-k84

## Goal

Read the repaired switch recovery and the source identity adversarially
before skill-loading-replan-k80 consumes them. Produce findings and no edits.

## Context

The artifact is the Grove commit of owner-pack-checkouts-k84. Read its diff
against its parent `e3070a5cc489`. k84 integrated the review
owner-pack-checkouts-k83. One in-session reviewer then found eleven more
defects, and k84 repaired them too.

The subject is these parts of `docs/specs/owner-skill-delivery.md`:

- The term "switch set" and the identity paragraphs of "Pack checkouts": the
  tree ID, the `resourceDigest` and the accepted digests.
- The section "The switch inside finish": steps 1 to 14, the continuation
  after a stop and the owner's run.
- The sections "Resuming the switch" and "Rollback", with the native probe.
- The order table and the test seams.

The subject also includes step 5 and the recovery table of
`docs/specs/coordinated-release.md`. The evidence is
`docs/owner-skill-delivery-evidence.md`, with its section "Checkout
configuration and the tree ID". k84's decision log records each disposition
and the verification.

The human's three decisions of 2026-11-02 stay closed. mise's
`[bootstrap.repos]` owns the owner's checkouts. Each checkout stays at a
published commit. No tool-neutral contract names mise.

Compare capture and resolution with Ravel's local-source contract
`7a894c782037` in the jj workspace `Ravel.skill-directory-delivery`. Compare
the finish order with `CLAUDE.md` and
`docs/adr/finish-promotes-after-the-grove-release.md`.

## Doubts to test

1. **Comparison by key.** Test whether the earlier and switched values of each
   plugin key are well defined in Claude Code's `settings.json` and Codex's
   `config.toml`. Test whether a comparison before each rename can still lose
   a concurrent write. Test whether step 1 can name every writer.
2. **The drifted route.** After the owner classifies a drifted item, the
   manifest and the dry run are prepared again without a new rehearsal. Test
   whether that route can lose the original recovery material, or reach a
   switch that nobody rehearsed.
3. **The state table.** Test whether each layout that an interruption or an
   outside change can produce has exactly one state. Test whether each state's
   next steps use only evidence that exists at that moment.
4. **The accepted digests.** Test where each accepted digest comes from: the
   staging trials, rollout's published archives, or a step-4 rerun. Test
   whether the disposable-HOME resolution of step 6 measures what the real
   HOME's launches capture.
5. **The native probe.** Test whether its four observations prove a working
   native route on both harnesses. Test what `ravel skills resolve` reports
   when no settings file exists.

## Done when

- Each finding names its file and line, its evidence and a severity.
- The review cuts an `integrate-review-design` leaf only if a finding needs
  action.

## Notes

k84 did not change `docs/evaluations/owner-skill-delivery/checkouts.py`. No
executable test covers the repaired switch. Its rehearsals and controls are
obligations of later leaves.

## Decisions (running log)

The review inspected Grove `80ce74974797fc9c67c4e548a1a4adf821d3be24`
against `e3070a5cc489`, including k84's dispositions. It compared the source
identity and disabled result with Ravel `7a894c782037` in
`Ravel.skill-directory-delivery`. The three human decisions remain premises.
This is a findings-only review. No test, build, lint or format command was run.

Four findings need triage before skill-loading-replan-k80: the interval
between comparing and replacing a shared file, completion inferred solely
from a receipt, location-only identification of moved entries, and a changed
switch plan reaching the real HOME without a new rehearsal. The digest and
missing-settings questions support no additional finding. The integration
belongs immediately before skill-loading-replan-k80.

## Findings

Locations refer to Grove `80ce74974797`. These are defects in the specified
procedure, not observed failures of an implemented switch. P1 identifies a
possible loss of unrelated settings; P2 identifies a correction needed before
dependent planning.

### 1. P1 — Comparing before a rename does not exclude another writer

**Location:** `docs/specs/owner-skill-delivery.md:609`, with the process check
at lines 560–566 and the inverse at lines 734–738.

The check and the rename are separate operations. A permitted interleaving
is: the switch prepares its new settings from the backup, compares the live
file successfully, a tool writes a new hook or other setting, and the switch
renames its prepared file over that update. The rename is atomic, but its
decision to replace the file is not conditional on the comparison still
holding. The key-only state comparison at lines 706–707 then ignores the
lost unrelated setting. The inverse has the same problem when it constructs
and renames a file while preserving other keys.

The process list is a point-in-time observation. It specifies no exclusion
that lasts through the write interval, cannot find a scheduled writer that
starts later, and expressly excludes finish's descendants. The document
itself identifies independent writers at lines 361–362 and 562–564. Naming
those writers does not prevent their next write. The rehearsal control at
lines 878–880 covers an update before the comparison, but misses an update
between the successful comparison and the rename.

**Smallest correction:** establish an enforceable interval without competing
writes for the forward and inverse operations, including automatic writers
and the executing session's descendants, or use an update mechanism whose
concurrency guarantee includes the comparison and publication. State what
stops the operation if that guarantee cannot be obtained. Add a rehearsal
interleaving at the exact comparison-to-rename boundary for both directions;
the unrelated update must survive or the operation must refuse. Another
unprotected comparison alone does not close the interval.

### 2. P2 — A receipt can mark a native or drifted layout complete

**Location:** `docs/specs/owner-skill-delivery.md:718`, with the other state
rows at lines 719–722 and the permitted later inverse at lines 751–755.

The Complete predicate asks only whether the latest complete backup contains
a receipt. It does not require the current switch set to be switched. It can
therefore hold at the same time as Native, Partial or Drifted. No precedence
or invalidation rule resolves that overlap.

For example, probes pass and step 13 publishes the receipt, but copying it
into the manual fails. The owner then uses the expressly supported inverse,
which restores native delivery without removing or invalidating that
receipt. A retry reaches the Complete row and copies the old activation
receipt into the manual without activating anything. Even without an
inverse, an outside change after step 13 can produce the same false
completion. The coordinated recovery row at
`docs/specs/coordinated-release.md:210` relies on this state table, so it
inherits the ambiguity.

**Smallest correction:** make completion depend on current layout evidence
as well as the receipt, and define what a rollback or subsequent drift does
to a receipt's applicability. Make the five predicates disjoint or state an
unambiguous precedence that cannot certify a reverted layout. Rehearse a
stop between steps 13 and 14 followed by an inverse, and a receipt followed
by outside drift. Neither case may copy stale activation evidence as the
current result.

### 3. P2 — A moved entry's location does not establish its earlier identity

**Location:** `docs/specs/owner-skill-delivery.md:703`, with the exclusion
from baseline comparison at lines 567–574.

After a complete backup exists, step 3 excludes switch-set entries from the
comparison with the prepared manifest. The replacement predicate for a moved
entry checks only which of its two paths exists. It does not compare the
entry's type, link target or content with the earlier or switched evidence.

Consider an interruption after step 8, followed by a change to a superseded
skill tree or a retargeted skill link at its original path. Only that path
exists, so the table still calls the item earlier and the set Native. Its
new baseline identity is excluded from comparison. Reading a Grove kind in
the native probe need not exercise this other changed skill. The procedure
can therefore create a new backup and move the changed entry without the
classification that step 3 requires for a changed entry on the first run.
A replacement at a retained path is similarly indistinguishable from the
original retained bytes. The current both-paths control does not cover
either case.

**Smallest correction:** combine the path conditions with the relevant
recorded change identity for every moved entry, including its form, link
target and resource content. A mismatch must enter the drifted route before
an inverse or a new backup adopts it. Preserve the existing rule that
unrelated keys of shared settings files do not determine state. Add
same-path content-change and link-retargeting controls on both sides of a
move.

### 4. P2 — Reclassification can replace the rehearsed switch plan

**Location:** `docs/specs/owner-skill-delivery.md:724`, through line 727,
with the rehearsal obligation at line 823.

The drifted route requires the owner to prepare the manifest and dry run
again, then proceeds to a new backup and the real switch. It neither limits
the new plan to the one already rehearsed nor requires another rehearsal
when classification changes an affected entry or its inverse. The native
probe checks that native discovery works; it does not exercise the new
forward operation, an interrupted inverse or receipt recovery. A dry run
also does not establish those restoration properties.

Consequently, a plan whose changed entries have never passed the disposable
layout controls can reach step 9. This conflicts with the prepared and
rehearsed operation required at lines 557–558, the order-6 acceptance at
line 823, and `docs/specs/coordinated-release.md:144`–146. The finish ADR
also requires preparation and rehearsal in ordinary leaves before finish,
whose teardown has already removed the task tree.

**Smallest correction:** bind rehearsal evidence to the switch manifest and
inverse it exercised. Define which classifications can reuse that evidence
and require a disposable rehearsal before applying a materially changed
plan. Preserve every earlier backup and its inverse while preparing the
replacement. Give the stopped finish and the owner's later run the same
rule; a native probe alone must not admit a changed plan.

## Inspection evidence and limits

The producer diff, k84's decision log and the targeted specification sections
were read against their committed parent. The publication order agrees with
`CLAUDE.md` and `docs/adr/finish-promotes-after-the-grove-release.md`:
Grove publication and installed verification precede Ravel promotion, which
precedes the owner switch. The ADR owns that ordering decision and delegates
the two procedures to their specifications. The findings require no new
Ravel interface, checkout owner or decision record.

A read-only inspection of the current owner settings found Claude Code
enablement booleans under `enabledPlugins`, and Codex booleans under
`plugins.<plugin-id>.enabled`. Their values can be recorded and compared by
key. They remain parts of shared files, so this observation does not supply
the missing concurrency guarantee. No setting or native installation was
changed.

Ravel's local-source contract was read directly at `7a894c782037`. Its
resource manifest contains relative paths, entry types, normalized executable
bits, file-content digests and link targets; it contains no checkout-root
path. The current `snapshot.rs` manifest function and digest calculation
agree. Thus step 6 can compare the real checkouts from a disposable HOME:
the settings explicitly name their absolute real paths. This checks source
identity before the switch; step 10 separately checks real-HOME resolution
and launches. The accepted staging digests, the published-archive
verification, and step 4's replacement core digest have named owners. The
review supports no additional identity finding.

The missing-settings native probe is also consistent with Ravel. Its contract
at `docs/specs/skill-discovery.md:206` and `:538`, current
`crates/ravel/src/skills/mod.rs:105` and
`crates/ravel/src/skills/command.rs:46`, and the source of
`disabled_delivery_prints_nothing_and_creates_no_cache` agree that absent
settings produce `{"schemaVersion":1,"status":"disabled"}`. No
configuration script or harness is evaluated by that resolution. The other
three native observations require actual harness reads and listings; this
review did not execute them and does not claim the owner route already
works. The probe is a recovery check, not a replacement for staging's wider
capability trials.

Graph verification used Tier 2 in
`Users-antony-Development-AgenticOrchestration-grove.gh-issue-25`, generation
`2026-11-02T10:47:07Z`. Coverage reported matching metadata and no recorded
gap for the relied-on Markdown files and Taskfile. The TSV was not tracked
by the index and its relevant entries were read directly. Ravel's project
was ready at generation `2026-11-02T05:30:19Z`; its `docs/` subtree is
excluded. Its resolver, resolution test and snapshot source had changed
metadata, so the relevant implementations were read directly after graph
discovery and traces. `command.rs` had matching metadata. The exact-name
capture search and material traces returned complete pages; the broad
ranked search was discovery only and was not treated as an exhaustive set.
Coverage is a best-effort signal, not proof of repository-wide completeness.

k84's scratch-clone results and its document and diagram checks remain the
producer's recorded evidence. They were not rerun. No executable switch yet
exists for the repaired procedure, and no tests, builds, linters or
formatters were run in this inspection. Integration owns dispositions,
fixes and post-fix verification.
