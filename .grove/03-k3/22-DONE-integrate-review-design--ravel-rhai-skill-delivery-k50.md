# ravel-rhai-skill-delivery-k50

**Integrates:** ravel-rhai-skill-delivery-k49

## Goal

Triage the Rhai directory-contract review before the Grove adoption design and
local planning consume the contract. Apply the supported corrections in their
owning artifacts and record the revised handoff.

## Context

Read the committed findings of `ravel-rhai-skill-delivery-k49` and the
producer's `ravel-rhai-skill-delivery-k34` decision log. The reviewed Ravel
artifact is commit `fc87cb41a422bf2623ce362ef689c38ddf98a506`, above the channel
contract `2a77ffbd` and main `24a54523`. It is on the local bookmark
`skill-directory-delivery` in `Ravel.skill-directory-delivery`.

Read the review's evidence and the cited source before classifying each
finding. Follow Ravel's repository instructions. Grove's coordinated-release
and owner-delivery specifications remain the consumer constraints. The local,
pinned and adoption task handles remain stable after this insertion.

## Done when

- Classify each finding as an unclear contract, a real issue, a visible
  trade-off or noise, with evidence for the classification.
- Apply accepted corrections to the owning Ravel design, evidence and views.
  Reconcile their summaries and affected consumer handoffs.
<!-- tc: depart synonym-rotation because "checks" names the document check runs, not a second verb for verify -->
- Verify the changed design and its relevant observations. Distinguish
  document checks from product behaviour that implementation and actual
  harness acceptance must still establish.
- Record the exact Ravel output commit and give the adoption and local/pinned
  consumers that revised input before they run. Keep the feature line off
  main and unpublished through acceptance.
- Externalise substantial redesign as its own producer and review chain.

## Notes

The findings are claims to assess, not an imposed correction list. This leaf
owns design integration, not feature implementation, owner activation, native
removal, a release or a tap change. Preserve the live driver's canonical ending
command. Edit neither the `Ravel.gh-issue-2` task tree nor another workstream's
working-copy change, and send no other-workstream signal.

## Decisions (running log)

**Inputs.** The findings come from the review commit `90ef6c54`. The reviewed
artifact is Ravel `fc87cb41` in the clean `Ravel.skill-directory-delivery`
workspace. `jj diff` from main `24a54523` shows no change under
`crates/ravel/src` or in the smoke scripts. No fetch ran. The local
`main@origin` is still `24a54523`.

**Finding 1 is a real issue, applied.** `LAUNCH_CONTROLS` holds six names.
The smoke function `sandboxed` removes four credential names and two Ravel
names. The sandbox recipe controlled `CODEX_HOME` only. The Claude Code
environment reference and authentication page, read on 2026-11-02, agree
with the review. `CLAUDE_CONFIG_DIR` moves settings, history, plugins, the sign-in
file and the macOS Keychain entry.

The verification also found three documented plugin-location variables with
the same effect: `CLAUDE_CODE_PLUGIN_CACHE_DIR`, `CLAUDE_CODE_PLUGIN_DIRS` and
`CLAUDE_CODE_PLUGIN_SEED_DIR`. A Claude Code run now refuses an inherited value
of any of the four. It does not remove the value, because the linked default
sign-in can then belong to another account. It does not set a sandbox value,
because that value moves the sign-in away from the link.

The run records the effective configuration directory of each harness.
Fake-harness controls in each sandbox wrapper prove the four refusals and the
`CODEX_HOME` refusal. Account-synced skills arrive through the reused sign-in.
The recipe records them as platform-managed residue, as the owner-delivery
design classifies them. The macOS fallback sign-in file is in the evidence
only, because the sign-in check fails visibly.

**Finding 2 is a real issue, applied.** The bound adds a 1 MiB envelope
allowance and a 1 MiB descriptor allowance. The two prompts and the descriptor
cannot all fill their allowances. `read_frame` already takes its bound as an
argument, and its test refuses at a reduced bound. Three tests now replace the
one-byte control: a public-bound fit, a unit test at a reduced limit and a
long-kind refusal.

`--kind` and `--task-file` are arguments, and the platform bounds the working
directory. A platform with small argument limits can admit no request over the
bound. The local stage therefore establishes on which release platforms the
long-kind test can run.

The production formula holds. `context::deliver` bounds the compact encoding
of the caller's document before the evaluate frame, also with a
`load_context`. A caller document cannot hold `measured`, so that encoding
covers `request.context`.

**The other challenged decisions stay.** The review accepted the schema-3
refusal, the harness-run environment, the unchanged protocol and document
numbers and the strict duplicate-member rule as visible costs. This
integration found no new evidence against them.

**Consumers.** Grove's `docs/specs/owner-skill-delivery.md` staging HOME gains
the same four-variable control and its proof in step 3. Its evidence gains the
documented observation. The subtree brief and the k35, local, pinned and
acceptance files name the reviewed commit. The k35 consumer handoff did not
change. Commits `12e47b6e` and `e2a9f8b1` first converted the acceptance
brief and this task to controlled English.

**Output.** The reviewed Ravel contract is commit
`822c0ed2f67d9a446b74476fdad7d05922552b87` on the local bookmark
`skill-directory-delivery`. Its parent is `fc87cb41`. The bookmark has no
remote copy, so nobody pushed it. The views show neither changed rule, so they
did not change.

**Checks.** The technical-communication checker reports no hard finding in
any changed document. The 91 local links and anchors of the changed Ravel
documents resolve, and a planted control fails. The view manifest parses.

These are document checks only. No product test, build, smoke task or harness
trial ran. The refusals, the frame check, the platform limits and the installed
Claude Code paths remain for the local stage and actual acceptance.

**No new chain.** Both corrections repair the reviewed contract, and neither
is a redesign. No review names this leaf, so no Creator line applies. No
in-session reviewer ran.
