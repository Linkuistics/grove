# coordinated-ravel-release-k39

**Reviews:** coordinated-ravel-release-k36

## Goal

Challenge the coordinated release contract before dependent design,
implementation or publication relies on it.

## Context

Read the producer's committed result, decision log and owner acknowledgement.
The artifacts are Grove's `docs/specs/coordinated-release.md` and evidence
document, its release views, and Ravel's `docs/specs/release-channels.md` at the
owning-repository commit `80e382223129cc659783b78e6e93bd6d3e391739`, on local
bookmark `coordinated-release-design` in `Ravel.coordinated-release-design`.
Its parent is exact Ravel main `24a545236185b0910ba6117fe8f9e8b2e2c8a748`.
Reconcile later source or release movement explicitly. The owner confirmed
that all other workstreams remain paused until this grove finishes. Release
prerequisites must be owned here, without editing the paused task tree.

## Done when

- Test the design against the root's agreed release, installation and reader
  requirements. Inspect actual release procedures and the observed public state.
- Challenge the single-owner handoff, exact Rhai inputs, release/schema
  identities, and every intermediate fresh-install and upgrade state. Challenge
  formula channel propagation, partial-cut recovery, old-driver lifetime,
  settings isolation, and assignment of promotion to Grove finish.
- Distinguish a valid proposed interface from an existing command. Determine
  whether ordinary planning owns all tooling needed before finish, and whether
  final confirmation and ending contracts remain intact.
- Report supported findings with consequence and smallest useful correction.
  Follow the review skill for any integration step. Do not implement repairs.

## Notes

No release, tap change, owner activation or other-workstream tree edits.
The producer names its finishing provider evidence only when it retires.

## Findings

Each finding names its location, the failure it allows, the smallest useful
correction and a severity. High means the contract is false or has no owner as
written. Medium means a reader can act on two readings, or a real failure has
no remedy. Low means a gap that the acceptance table cannot see.

No finding reached high. Visible costs follow the findings in their own
section. Findings 1, 3, 4 and 6 concern interfaces or state and close with a
codebase-design record.

### 1. Promotion converts every default-channel configuration, and no gate names it (medium)

Location: `docs/specs/coordinated-release.md`, "Compatible tap states", row D
and the "Pinned Grove" paragraph. Source: Ravel main
`crates/ravel/src/authority.rs`, `PERSONAL_RHAI_DEFAULT` and `retired_format`.

Ravel 1.1.0 reads `~/.config/ravel/policy.ts`. Ravel main reads
`~/.config/ravel/config.rhai`. When that file is absent and `policy.ts` exists,
main refuses with `config_format_retired` and exit 3. Grove's published formula
tells every user to copy a `policy.ts` sample. The owner's configuration
directory holds only `policy.ts`. The paused Rhai workstream removed its
migration guide and chose a one-off manual conversion.

Row D says fresh installs get a compatible pair. After promotion, every user
whose upgrade moves unversioned `ravel` to 2.0.0 holds a compatible binary pair
that refuses at the first launch. The design defines compatibility to include
settings. The table and the legacy-route paragraph cover only packaging
failures: pinned Grove, `HOMEBREW_NO_INSTALLED_DEPENDENTS_CHECK`, failed upgrades
and kept archives.

Consequence: promotion breaks every default-channel installation that has not
written `config.rhai`. The rehearsal cannot see it, because its subjects are
formulae.

Smallest correction: add to row D and to the promotion prerequisites that
promotion converts every default-channel configuration from `policy.ts` to
`config.rhai`. Before promotion, publish the conversion guidance: Ravel 2.0.0's
changelog entry and book chapter 4, and the compatible Grove's caveats with a
`config.rhai` sample. Name `config_format_retired` as the observable failure
and its remedy. Add a `policy.ts`-only HOME to the A to D and C to D upgrade
rehearsals.

### 2. Two load-bearing decisions pass the record test and have no record (medium)

Location: `docs/specs/coordinated-release.md`, "Release boundary", paragraph 3,
and "Publication ownership and order", step 7. Ravel
`docs/specs/release-channels.md`, "One release, two formula channels".

The first decision publishes a caller transition through a keg-only versioned
formula and promotes later. Its rejected alternatives are direct unversioned
publication, which Grove 22.8 refuses through `check_version` and its
unversioned dependency, and an earlier Grove compatibility release. The second
decision assigns Ravel promotion and assigned activation to Grove's
driver-created finish as an extension before its signal. Its rejected
alternative is an ordinary leaf, which cannot follow Grove's release because
that release belongs to finish.

Both are hard to reverse once `ravel@2.rb` is public and the root charter is
amended. Both surprise a reader without the Grove 22.8 admission context. Both
record their rejected alternative only in spec prose and in the root brief,
which dies with `.grove/`. Ravel's spec gives the reason as the state of a
Grove workstream, which is process context. The producer wrote no ADR.

Consequence: a future Ravel maintainer who meets `ravel@2.rb` or
`release:promote` finds no durable reason. The spec restates trade-offs that
the grain rule assigns to records, and the two can drift. The prior review's
finding 4 had this shape.

Smallest correction: write one ADR in Ravel for the versioned-channel-first
transition and one in Grove for the finish-owned promotion. Give each its
rejected alternatives and a reopen condition. Reduce the spec paragraphs to
citations.

### 3. The channel rule after promotion is undefined (medium)

Location: Ravel `docs/specs/release-channels.md`, "Release interface",
paragraph 2, and "Promotion without another release", paragraph 2. Grove spec,
row D, "Keep `ravel@2`".

The Ravel spec says later 2.x cuts take `ravel@2` as their baseline formula
while unversioned remains 1.1.0. Both specs say to keep `ravel@2` after
promotion. Neither says which channel a 2.0.1 cut publishes after state D.
Under one reading every 2.x cut publishes `ravel@2` and needs a further
promotion. Under the other, unversioned becomes the default again and `ravel@2`
stays at 2.0.0 while the caller design tells directory users to select it.

Consequence: the first post-promotion release either leaves `ravel@2` silently
stale or doubles every release. The release tool cannot refuse a channel it
has no rule for.

Smallest correction: state the post-promotion default channel and the fate of
`ravel@2`, either frozen at 2.0.0 as a transition formula or tracked by every
2.x cut. State the baseline rule for each channel after D.

### 4. Finish records teardown through the newly installed helper, unconstrained (low)

Location: Grove spec, "Activation and running readers", paragraph 4. Source:
`crates/grove-llm/src/cli.rs`, `cmd_record_teardown`, and
`crates/grove-loop/src/launch_directory.rs`.

The design preserves the Ravel 1.1.0 keg and its ending command. After
`task release:minor`, the same finish session runs `grove-llm record-teardown`
from the new Grove keg under the 22.8.0 driver. That verb requires
session-epoch admission through `GROVE_LAUNCH_DIR` and writes a regular
`teardown` marker that the driver reads. `docs/RELEASING.md` already says a
session-signal release must preserve the old helper. The design neither
classifies the compatible release against that rule nor constrains the
launch-directory contract.

Consequence: a compatible release that changes admission leaves finish unable
to record teardown after `.grove/` is already gone, and the loop stops.

Smallest correction: state that the compatible Grove release keeps the 22.8.0
launch-directory and epoch admission contract unchanged. Otherwise name the
helper-preservation step from the signal-contract cutover as a rollout
deliverable.

### 5. Grove's release gate consumes pinned Ravel archives, and no row names it (low)

Location: Grove spec, "Publication ownership and order", steps 5 and 6.
Source: `docs/ravel-target-inputs.md`, `scripts/release-smoke.sh` and
`scripts/grove-installed-smoke.sh`.

Grove's release smoke admits independent Ravel archives by pinned SHA-256 and
hard-codes 1.1.0 in the archive names and the installed version assertion. The
compatible Grove release must admit the 2.0.0 archives to exercise its second
route, and keep 1.1.0 for the first. The spec orders Ravel publication before
Grove's release and says both routes must pass. It names source-pack pins, not
these release pins, as what rollout measures against published archives.

Consequence: a rollout planner can schedule Grove's release before the pins
exist, and Grove's own task then refuses at its smoke step.

Smallest correction: add Grove's pinned independent Ravel inputs to the
ownership table as a consumer of the 2.0.0 publication receipt, retained
beside 1.1.0.

### 6. Cleanup suppression covers only the mutations that set it (low)

Location: Grove spec, "Activation and running readers", paragraph 1. Ravel
spec, "Recovery and existing processes", paragraph 3. Source: the Homebrew
manual, `HOMEBREW_NO_INSTALL_CLEANUP`, `HOMEBREW_CLEANUP_PERIODIC_FULL_DAYS`
and `HOMEBREW_NO_CLEANUP_FORMULAE`.

Both specs name `HOMEBREW_NO_INSTALL_CLEANUP`. The manual says any
`brew install`, `brew upgrade` or `brew reinstall` without it runs a full
cleanup of all packages every 30 days. After promotion the 1.1.0 keg is an old
version, so an unrelated install in another shell can remove it while the
driver still needs it. Grove's legacy transition tool already adds
`HOMEBREW_NO_CLEANUP_FORMULAE` for this reason.

Consequence: the ending path can vanish through a mutation the finish never
ran.

Smallest correction: name `HOMEBREW_NO_CLEANUP_FORMULAE`, with `ravel` added to
existing exclusions, for the window from promotion to the driver's reap, in
both specs.

### 7. The schema-4 reservation states no reason and no cost (low)

Location: Grove spec, "Public identities", rows 2 and 3, and the paragraph that
follows the table.

The table reserves script/request schema 4 and leaves compatibility with
schema-3 scripts to the directory design. It gives no reason. The reason that
justified the earlier bump is in `docs/grove-skill-delivery-design-evidence.md`:
the prepared prompt changes what `request.prompt` means to the script. That
reason also decides the open question, because a script with delivery disabled
sees an unchanged prompt. The cost is unrecorded: every example, fixture and
book occurrence of `SCHEMA_VERSION = 3` on Ravel main, and the paused
workstream's conversion target, move to 4.

Consequence: k34 receives a number without the test that produced it and can
decide compatibility against the wrong criterion.

Smallest correction: one sentence stating the reason and one naming the cost.

### 8. The paused Rhai workstream's release leaves receive no written handoff (low)

Location: `docs/coordinated-release-evidence.md`, "Task ownership and handoff",
row 2. Source: `Ravel.gh-issue-2`, `21-impl--release-ravel-2-0-0-k37.md`, and
Ravel `scripts/release-prepare.sh`.

The paused leaf says its workstream makes no release before it, and tells its
session to run `task release:major` after one question. Ravel's preparation
cuts a new major whenever the baseline release is complete and main has
changes. A resumed k37 after this grove finishes would therefore cut 3.0.0
unless the human remembers. The evidence assigns reconciliation to that
workstream's maintainer on resumption and forbids tree edits. It names no
deliverable that carries the receipts there. The root brief already requires
brief-update text for other paused groves.

Consequence: the single-owner handoff protects the release only while the
human's memory does.

Smallest correction: add the Rhai workstream to the brief-update deliverable,
owned by rollout. Carry the 2.0.0 receipts and the instruction to prune or
retarget k37, k38 and k39.

### 9. The return to state C has no owner after teardown (low)

Location: Grove spec, "Compatible tap states", last paragraph, the sentence
"If the supported upgrade cannot preserve the pair, stay in C and repair
packaging."

The rehearsal that can send the route back to C runs before finish, as
"Publication ownership and order" says. The sentence sits in the tap-state
section without that timing. Inside finish, `.grove/` is gone after
`finish-commit`, so no leaf can carry a repair. Only the recovery table's
resume rows apply there.

Smallest correction: state that the stay-in-C decision is rollout's, before
`finish-commit`, and that inside finish only the resume rows apply.

### Codebase-design record

| Finding | Evidence | Caller obligation | Entangled concerns | Consequence |
| --- | --- | --- | --- | --- |
| 1 | Main refuses `policy.ts` with exit 3 | Every default-channel user must write `config.rhai` before promotion reaches them | Formula channel and configuration format | Compatible binaries refuse at first launch |
| 3 | Two specs keep `ravel@2` and name no later channel | A release operator must choose a channel the tool has no rule for | Transition channel and steady-state channel | Stale formula or double publication |
| 4 | Teardown needs epoch admission through the new helper | Finish must trust an unconstrained contract | Grove release content and driver lifetime | Loop stops with the tree gone |
| 6 | Periodic cleanup runs from any mutation | The owner must guard every shell for the window | Keg retention and unrelated installs | Ending path removed |

## Visible costs

These are trade-offs the artifact states and accepts. They are not defects, and
the integration should leave them unless the owner decides otherwise.

- Combining Rhai and directory delivery makes Rhai publication wait for
  directory acceptance and the unfinished book work. The spec states this.
- Keg-only `ravel@2` never becomes the default command, so a directory user must
  put its prefix first on PATH for every launch. The caller design owns the
  examples.
- The shared `settings.json` means a Rhai environment that needs any settings
  key must run under a separate HOME while a 1.1 launch remains. Source confirms
  both majors refuse unknown keys and rename `policyEnv` to `configEnv`. The
  owner has no settings file today.
- Grove's formula keeps its unversioned dependency through C, so a
  directory-only user still installs Ravel 1.1.0.
- The finish session grows by one rehearsed operation. The design rules out
  designing it there.

## What held under challenge

- The identity table matches source at main `24a54523`. Script schema 3 is in
  `rhai_worker/engine.rs` and worker protocol 1 in `rhai_worker/mod.rs`. Context
  schema 1 is in `context.rs`, and refusal and selection notices carry schema 2
  in `refusal.rs` and `run.rs`. The manifest says 1.1.0.
- The public state is unchanged since the producer's observation. Ravel main is
  `24a54523` locally and remotely. The only tags are v0.1.0, v1.0.0 and v1.1.0,
  with three GitHub releases. The tap's main is `77e034ce` and holds no
  `ravel@2`. Installed Grove 22.8.0 and Ravel 1.1.0 are unpinned.
- Grove 22.8 refuses any version outside stable 1.x from 1.1.0, so direct
  unversioned 2.0.0 would break every fresh Grove install. The versioned-first
  route is necessary without an earlier compatibility release.
- Ravel's release task hard-codes the unversioned channel exactly as the
  evidence says: `Formula/ravel.rb` in the baseline, `ravel.rb` from a template
  with class `Ravel`, `brew upgrade ravel` and `brew test ravel`. It reads the
  version at the shared prefix.
- The first-cut baseline rule matches `release-prepare.sh`: local and remote
  v1.1.0, a complete release, the tap version line and changes since the tag.
- The recovery table agrees with Ravel's chapter 11 rows and Grove's
  `docs/RELEASING.md`. The release lock is per repository, because the task
  requires the default workspace.
- Configuration scripts coexist under one HOME. With `config.rhai` present,
  main ignores `policy.ts`.
- The Homebrew claims match the manual: outdated dependents upgrade unless
  disabled, and cleanup runs unless suppressed.
- The finish contract is intact. One confirmation covers the whole sequence,
  teardown precedes the signal, the extension sits before the signal, and
  force-stop semantics are unchanged. The 1.1.0 ending path survives through the
  retained keg.
- `docs/RELEASING.md`, the Ravel spec and the evidence document mark every
  proposed interface as proposed. `CHANNEL` and `release:promote` exist in no
  Taskfile.
- The two diagrams match the ordering and the tap states. The producer records
  that it inspected desktop light rendering only.

## Decisions (running log)

- The review read the artifacts at Grove commit `89d124f0` and Ravel commit
  `80e38222` in `Ravel.coordinated-release-design`, against the root and node
  briefs. It spent no in-session reviewer and edited no Ravel file or paused
  tree.
- The review compared source claims with files read directly at Ravel main
  `24a54523`, Grove's working tree, the tap clone and the Homebrew manual. It
  used no graph coverage.
- Findings 1 to 3 require edits to both specifications and a record set. They
  earn an integration step. The step sits before
  `owner-ravel-skill-delivery-k37`, the first live sibling entry after this
  review, by `leaf-insert`. Its body names this review by handle and restates
  no finding.
- This launch has no `GROVE_PROVIDER_EVIDENCE` and is not a legacy dispatch
  launch. This leaf is a review, not a producer, so it writes no Creator line.
- This session converted the task file to controlled English as a preparatory
  commit, under the technical-communication rule for existing documents.
