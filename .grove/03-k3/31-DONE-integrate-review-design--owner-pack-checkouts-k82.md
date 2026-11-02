# owner-pack-checkouts-k82

**Integrates:** owner-pack-checkouts-k81

## Goal

Triage the committed design review and apply the valid findings before
skill-loading-replan-k80 consumes the owner-delivery design.

## Context

Read the findings in owner-pack-checkouts-k81's commit and the producer
owner-pack-checkouts-k79. The review identifies the exact Grove and Ravel
contract revisions and its verification limits. Preserve the human's settled
checkout-owner, published-pin and tool-neutral product decisions.

## Done when

Each finding has an evidence-based disposition. Valid corrections agree
across their owning design, evidence, views and handoffs, and the relevant
verification is recorded. The replanning leaf receives a coherent contract.

## Notes

Follow the integration family's triage procedure. The review's findings are
inputs to assess, not a predetermined implementation checklist.

## Decisions (running log)

**Base.** On 2026-11-02 this checkout was at `03794cdd5645`, the commit of
owner-pack-checkouts-k81. The producer is `15a4aecdfc2a`. Ravel's
local-source contract is `7a894c782037` in `Ravel.skill-directory-delivery`.

**Triage.** Each of the six findings reproduces against the cited text.

1. P1, a real issue. The rule "no Ravel launch runs" contradicts the switch,
   which runs inside finish's own launch. Ravel's rule is about capture: a
   source stays quiet during a capture, and `ravel skills resolve` also
   captures. A Ravel 1.1 launch captures nothing. Settings without a `skills`
   key capture nothing. The fix restates the rule as an interval without a
   capture.
2. P2, a real issue. Read-only `git config` queries showed each route. An
   inherited `XDG_CONFIG_HOME`, `GIT_CONFIG_GLOBAL` or `GIT_CONFIG_COUNT`
   exposes a helper under the recipe of the specification. The owner's global
   configuration is on the XDG path and names Git Credential Manager. The
   fixture's allowlist exposes none. The fix copies the allowlist into the
   specification and requires a decoy-helper control.
3. P2, a real issue. A failed `apply` inside finish had no continuation. The
   fix defines a stop before the native change for every switch step that
   precedes it. Finish keeps the edited pin, writes a pending-activation
   record and retries. If the session must end first, it ends without the
   success signal. The owner resumes the same switch from its first step.
   That run releases, promotes and tears down nothing.
4. P2, a real issue. The fix separates the accepted `grove-core` candidate
   from the released pin. Staging records the Git tree ID of the source
   directory. The switch compares it before any change, and reruns the trials
   that read `grove-core` when the IDs differ. A probe and a staging trial now
   read a Grove kind skill. The v22.8.0 release commit changed only
   `CHANGELOG.md`, `Cargo.lock` and `Cargo.toml`, so equal IDs are the
   expected case.
5. P2, a real issue. The isolation trial launches sessions with all ten sources
   configured. The fix puts the credential control and the clones before it.
6. P2, a contract stated unclearly. k79's log said that this grove's briefs
   name mise. Its review charter said that the root text names no tool. The
   human's decision covers only the tool-neutral contracts, so either reading
   keeps that decision. The review charter wins, because the owner
   specification already owns the tool. A brief that restated it would be a
   second owner. The fixed root-brief text is below, and k80 reads it
   from here.

No finding needs a redesign. Each fix keeps the human's checkout owner,
pin policy and tool-neutral contracts.

**In-session reviewer.** One fresh reviewer read the fixed finish sequence,
the advance rule, the credential control and the cutover order. It received
the artifacts and their contracts, without these conclusions. It reported six
P2 and three P3 findings. Each one reproduces against the text:

- R1, valid. Later leaves of this workstream can change `grove-core` after
  order 2, so an unequal tree ID was likely. The candidate is now the last
  core change. Rollout verifies the final ID before finish, and a committed
  evidence document holds it. The staging HOME stays until the receipt. A
  failed core trial after the release leads to a Grove patch release.
- R2, valid. A failure in the native change, or an inverse after a failed
  probe, had no continuation. Both now share the continuation of a stop.
- R3, valid. `GROVE_LAUNCH_DIR` is in this session's environment. A probe
  that ran `grove-llm record-teardown` could record finish's teardown. Probes
  and core trials now drop Grove's loop variables and run outside the
  workspace. The probe reads a kind skill other than `grove-finish`.
- R4, valid. Nested launches, independent resolutions, a release task and a
  synced pin can also capture or move a checkout. Every `apply` and every
  complete bootstrap now counts as an advance, behind a process check.
- R5, valid. The control's base environment was undefined, so it could call
  the real helpers. Each case now uses the allowlist of a fixture HOME as its base
  and clones through mise. `GIT_CONFIG_PARAMETERS` and `GIT_CONFIG_SYSTEM`
  join the routes.
- R6, valid. Order 4 could not pass before the clones of order 5. Its check
  moves into order 5.
- R7, valid at P3. One Claude Code session takes finish's place when the
  owner runs the switch.
- R8, valid at P3. Step 6 now compares each pin with the accepted pin.
- R9, a contract stated unclearly. The trials accept the feature build, and
  rollout repeats the pin check against the published archives. Order 5 now
  says so. The two undefined failures at P3 now count as failed probes. The
  filter exception to the tree ID becomes a layout rule.

The fixes for R1 to R5 are substantive, and no executable test covers them.
That is a second need for review. This session cut the focused review
owner-pack-checkouts-k83 before skill-loading-replan-k80. Its body holds the
doubts that remain. No provider evidence is in this session's environment, so
the review has no Creator line.

**Verification.** On 2026-11-02, `task owner:checkouts` passed all 33 cases
with mise 2026.10.25. The `--mutate` run failed exactly the offline advance to
a held commit. The script digest stayed
`349e2e233c3967f370c39174034a019cf84f2980758f4098c2827bc819cd3244`, and no
temporary directory remained.

The credential-route queries are in the evidence document. The prose checker reports no hard finding in each changed
document, and `diagrams.json` parses. The new anchors resolve to their
headings. No walkthrough book, test or check script reads the changed
documents, so `bash scripts/check.sh` would measure nothing of this change.
The finish integration runs it on the integrated result.

After the reviewer's fixes, two more `git config` queries showed that
`GIT_CONFIG_PARAMETERS` exposes a decoy helper through the allowlist, and that
`GIT_CONFIG_NOSYSTEM=1` blocks `GIT_CONFIG_SYSTEM`. The evidence table records
both. Each changed document again has no hard prose finding, and
`diagrams.json` parses.

## Handoff to skill-loading-replan-k80

This text replaces the root-brief replacement in the k79 handoff. The k79
replacement for the brief of external-research-pack-k4 stays as it is.

Find the Ravel paragraph of "Ownership boundaries" in the root brief. Replace
its sentence whose first words are "A design leaf after the pinned node" with
the quoted sentences below. Keep the sentence whose first words are "This
paragraph supersedes" while another brief holds a pinned-Git clause. Delete it
as the last edit of the reconciliation:

> owner-pack-checkouts-k79 settled the mechanism, and owner-pack-checkouts-k82
> integrated its review. Ravel and Grove require no checkout tool. A tool
> outside Ravel holds each of the owner's pack checkouts at a published commit.
> `docs/specs/owner-skill-delivery.md` names the owner's tool. It also states
> the commands, the missing-checkout behaviour and the behaviour without a
> network.

The integration added these obligations to the specifications. The leaves
that owner-ravel-skill-inventory-k38 and discovery-rollout-k12 grow consume
them:

- Order 2 of the cutover contract publishes the `grove-core` candidate commit
  of this workstream's line.
- The staging trials run the decoy-helper control, then the clones, then the
  isolation trial. The evidence records the tree ID of the `grove-core`
  candidate.
- Step 5 of the coordinated release design records that tree ID for finish.
  Its recovery table has a row for a switch that stopped before its native
  change.
- The `grove-core` candidate is the last core change of this workstream's
  line. Rollout shows before finish that the final line has the same tree ID.
  The staging HOME and the checkout HOME stay until the switch receipt.
- The finish switch compares the tree IDs before any change. A stop, or an
  inverse after a failed probe, writes a pending-activation record and runs
  the switch again. Rollout names this continuation in the finish handoff.
- Probes and core reruns run outside every Grove workspace, without Grove's
  loop control variables.
- Every `apply` and every complete `mise bootstrap` is an advance, behind a
  process check. The machine contract and `mac-bootstrap` guidance follow
  this rule.
- The coordinated recovery table has a row for a failed core trial after the
  release. A Grove patch release follows the fix on `main`.
