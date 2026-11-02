# owner-pack-checkouts-k81

**Reviews:** owner-pack-checkouts-k79

## Goal

Read the revised owner-delivery design adversarially before
skill-loading-replan-k80 consumes it. Produce findings and no edits.

## Context

The artifact is the Grove commit of owner-pack-checkouts-k79. Read its diff
against its parent `a20454a854b7`. The main subject is the section "Pack
checkouts" of `docs/specs/owner-skill-delivery.md`. The subject also includes
that file's staging, final-session, switch, rollback and cutover sections.

The evidence is `docs/owner-skill-delivery-evidence.md` and
`docs/evaluations/owner-skill-delivery/checkouts.py`. Run
`task owner:checkouts`, and run the script with `--mutate`. The script writes
only into a temporary directory.

The human fixed three decisions on 2026-11-02, and the review does not reopen
them:

- On the owner's machine, the `[bootstrap.repos]` part of mise owns the pack
  checkouts.
- Each checkout stays at a published commit, the pin, which the owner advances
  by an edit.
- No tool-neutral contract requires mise. Ravel's specification, Grove's
  delivery specification and the external-pack text name no tool.

Compare the design with Ravel's local-source contract `7a894c782037`, in the jj
workspace `Ravel.skill-directory-delivery`. Compare it with mise's own help and
reference, not with memory. Do not run a mise command that changes the real
HOME.

## Doubts to test

An in-session reviewer found 22 findings, and k79 fixed all of them. Three
fixes change the design, and no executable test covers them:

1. **The `grove-core` pin advance inside finish.** Switch step 4 advances the
   pin to the commit of finish's Grove release and runs `apply`. Test the
   order against the coordinated release design and the finish rules in
   `CLAUDE.md`. Test also the rollback and the receipt, and the case where
   `apply` fails inside finish.
2. **The separate checkout HOME for staging trials.** The checkouts now lie
   outside the staging HOME. Test whether the staging settings, the isolation
   trial and Ravel's sandbox recipe still hold. Test also whether the
   credential rule prevents every read of the real keychain.
3. **An advance only while no Ravel launch runs.** Test whether the owner can
   satisfy the rule while a Grove loop runs. Test whether the switch and the
   preparation steps satisfy it.

Also test the handoff text in k79 for external-research-pack-k4 and the root
brief. The text must name no tool. Its order must keep each pinned-Git clause
superseded until k80 removes it.

## Done when

- Each finding names its file and line, its evidence and a severity.
- The review cuts an `integrate-review-design` leaf only if a finding needs
  action.

## Findings

Reviewed Grove commit `15a4aecdfc2a` against `a20454a854b7`, with Ravel's
local-source contract at `7a894c782037`. Locations below name the producer's
committed files. P1 blocks the designed execution; P2 requires a correction
before the dependent plan consumes the design.

### 1. P1 — The update rule excludes the session assigned to perform the update

**Location:** `docs/specs/owner-skill-delivery.md:212` (also switch step 4 at
line 466 and `docs/design/harness-selection-and-execution/owner-pack-checkouts.mmd:6`).

The rule permits an advance only while no Ravel launch runs. Finish must
advance `grove-core` while its enclosing Ravel 1.1 launch runs, and switch
step 1 explicitly excludes that live driver chain from its process gate.
An owner following both instructions cannot complete the switch. An update
performed by an ordinary Grove session has the same contradiction. The rule
also omits standalone `ravel skills resolve`, which can capture a source
without launching a session.

**Evidence:** the owner design's lines 435–446 and 456–467; Ravel
`docs/specs/skill-discovery.md:326–342` at `7a894c782037`. Ravel requires the
source to be quiet during capture. An existing session reads an immutable
snapshot, so its whole managed lifetime does not require a quiet checkout.
Ravel 1.1 supplies no directory capture in the first place.

**Smallest correction:** define a workable interval that prevents captures
of the affected checkouts through either public resolution boundary while
the update runs, including new captures. State how preparation and finish
establish it. Permit already-launched snapshot readers and this native-only
1.1 finish session, and reconcile the diagram with that rule. This changes
neither the human's pin policy nor Ravel's capture contract.

### 2. P2 — The staging clone environment can still select a real credential helper

**Location:** `docs/specs/owner-skill-delivery.md:374`.

An empty checkout HOME, `GIT_CONFIG_NOSYSTEM=1` and
`GIT_TERMINAL_PROMPT=0` do not establish the promised absence of a real
keychain read. An inherited `GIT_CONFIG_GLOBAL` can name the owner's Git
configuration. An inherited `XDG_CONFIG_HOME` can expose its Git config, and
`GIT_CONFIG_COUNT` can inject `credential.helper=osxkeychain` directly.
`GIT_ASKPASS` and `SSH_ASKPASS` are additional credential-program routes.
Disabling a terminal prompt does not disable those routes. A remote asking
for authentication can therefore reach them before failing.

**Evidence:** Git's [configuration environment and file rules](https://git-scm.com/docs/git-config#_environment)
and [credential helper and askpass rules](https://git-scm.com/docs/gitcredentials#_requesting_credentials).
The committed fixture is stronger than the staging recipe:
`docs/evaluations/owner-skill-delivery/checkouts.py:37–45` constructs a fresh
environment, isolates the XDG and mise paths and sets `GIT_CONFIG_GLOBAL`
to the null device. Its recorded result does not validate the weaker recipe.

**Smallest correction:** specify the clone environment as an explicit set
of allowed variables, with isolated mise/XDG paths and no inherited Git
configuration or credential programs. Require a disposable decoy-helper
control before real-remote trials. The control must observe that the helper
was not invoked; it must never use the real keychain.

### 3. P2 — A failed finish-time apply has no defined continuation after teardown

**Location:** `docs/specs/owner-skill-delivery.md:466`.

The new step edits the tracked intended pin and then performs a fetch that
can fail. The next step stops on drift, before the backup and receipt are
written. By this point finish has removed `.grove/`, published Grove and
promoted Ravel. The native switch has not happened, while the intended pin
and checkout HEAD can disagree. The design specifies neither an in-session
retry nor the persisted evidence and entry point for resuming activation if
that session ends.

**Evidence:** the recorded offline-advance case at
`docs/owner-skill-delivery-evidence.md:250–251` leaves the checkout at the old
commit after an intended-pin edit. Switch lines 474–485 write the backup
and receipt later, and rollback at lines 508–517 deliberately retains the
checkouts and mise entries. `CLAUDE.md:16–24` places teardown before release.
`docs/specs/coordinated-release.md:90–93` excludes returning to ordinary
leaves after teardown; its recovery table at lines 191–202 has release and
promotion retries but no owner-activation retry after successful promotion.

**Smallest correction:** define the apply-failure branch before native
mutation. State whether the intended pin is retained or restored, record the
published release identities and actual checkout state, and give a concrete
retry/resume operation that completes activation and its receipt without
another release, promotion or teardown. Preserve the original ending command
and withhold the successful finish signal until activation succeeds.

### 4. P2 — The final core pin is selected after the step that verifies final pins

**Location:** `docs/specs/coordinated-release.md:117` (with
`docs/specs/owner-skill-delivery.md:466`).

Rollout must verify the final source-pack pins before Grove's release, but
the new switch step selects a different `grove-core` pin after that release.
Finish also rebases onto current main between rollout and publication, so
the released core skill bytes can differ from the staged candidate. No step
requires evidence that the new pin contains the accepted core bytes or a
repeat of the affected directory-delivery trial when they differ. The switch
probes cover mandatory guidance, an unspecified craft skill and TestAnyware;
they need not load a core kind from the newly selected pin.

**Evidence:** coordinated release lines 117–132 and `CLAUDE.md:18–24`;
the new owner switch step at lines 466–467 and its probe list at 487–494.
The pin's release identity proves which commit was published, but does not
carry the earlier skill-read evidence across an intervening main integration.

**Smallest correction:** distinguish the pre-release candidate pins from
the final released core pin. Require a source-content identity comparison
that carries the accepted core evidence to the released commit, or rerun the
affected staging proof on that commit before real activation. Reconcile
rollout's handoff and finish's assigned work so that final pin verification
has one executable order.

### 5. P2 — The first isolation launch precedes installation of its required sources

**Location:** `docs/specs/owner-skill-delivery.md:589`.

The cutover says that the isolation trial runs first and that the staging
trials then clone the checkouts. The isolation trial itself launches one
session on each harness at lines 414–419. Its settings already contain all
ten required sources at lines 368–382. With the clones still absent, Ravel
refuses before the harness launches, so the isolation trial cannot produce
its required observations. Moving the clones to a separate HOME does not
remove this dependency.

**Evidence:** Ravel's required-source rule in
`docs/specs/skill-discovery.md` at `7a894c782037` refuses a missing local
directory before configuration evaluation or worker launch. The owner design
also states this outcome at lines 224–227.

**Smallest correction:** explicitly place the isolated checkout creation
and staging-path substitution before the isolation trial's first launch.
Keep the isolation trial first among actual harness trials, and keep both
temporary homes distinct from the real HOME.

### 6. P2 — The root-brief replacement violates the tool-free handoff condition

**Location:** `.grove/03-k3/29-DONE-design--owner-pack-checkouts-k79.md:101`.

The root-brief replacement explicitly names mise and `[bootstrap.repos]`.
This review's handoff condition requires the replacement text for both the
external-pack brief and the root brief to name no tool. The external-pack
replacement meets that condition, but copying the root replacement as given
does not. Its disclaimer that other users need no particular tool does not
remove the named tool from the proposed text.

**Evidence:** k79's quoted replacement at lines 101–105 and this review's
"Also test the handoff text" condition. The general Ravel boundary at
`docs/specs/codex-skill-provisioning.md:94–103` already delegates the owner's
instance to the owner-delivery specification without naming it.

**Smallest correction:** keep the root handoff at the level of a checkout
mechanism outside Ravel, a published commit and a reference to the owner
specification. Keep the owner's mise choice in that owning specification.
Preserve the existing supersession sentence until the final pinned-Git
clause has been reconciled; k79's proposed reconciliation order already
preserves that requirement.

## Inspection evidence and limits

The human's three settled choices remain the review's premises. The review
does not propose a different checkout owner, revision policy or general
Ravel/Grove interface. Keeping checkouts after rollback is consistent with
their location outside native discovery; finding 3 concerns the missing
failure continuation, not their retention.

Read the producer diff, its specification and recorded fixture evidence,
the complete fixture source, the coordinated release contract and the finish
rules. Read Ravel's local-source and sandbox-home contracts directly from
`7a894c782037` in `Ravel.skill-directory-delivery`. That workspace is absent
from the available graph projects, so those document reads used jj directly.

Read installed mise 2026.10.25's `bootstrap repos apply --help` and
`bootstrap repos status --help`, and the
[official repository reference](https://mise.jdx.dev/bootstrap/repos.html).
The fixture SHA-256 is
`349e2e233c3967f370c39174034a019cf84f2980758f4098c2827bc819cd3244`,
which matches the producer's evidence. The reported 33 passing comparisons
and failing mutation remain the producer's recorded results, not new runs.

The loaded `grove-review-design` skill directs `references/review.md`, which
says, "A review does not run test, build, lint, or format commands."
Consequently this inspection did not run `task owner:checkouts` or its
mutation variant despite the task's suggested method. Integration owns
post-fix verification. No production, test or design artifact was edited.

Graph work used Tier 2 verification in project
`Users-antony-Development-AgenticOrchestration-grove.gh-issue-25`, generation
`2026-11-02T09:25:33Z`. The fixture search was complete for its query
(`has_more: false`), followed by a trace, an exact snippet and full source
inspection. Coverage checks reported no recorded gaps for the fixture,
specifications, task evidence and release rules. The two partial Mermaid
files reported ranges 1–33 and 1–37; both files were read in full. These are
bounded review observations, not a claim of repository-wide completeness.
