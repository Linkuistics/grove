# owner-pack-checkouts-k83

**Reviews:** owner-pack-checkouts-k82

## Goal

Read the repaired owner-delivery design adversarially before
skill-loading-replan-k80 consumes it. Produce findings and no edits.

## Context

The artifact is the Grove commit of owner-pack-checkouts-k82. Read its diff
against its parent `03794cdd5645`. k82 integrated the review
owner-pack-checkouts-k81 of the design owner-pack-checkouts-k79. One
in-session reviewer then found nine more defects, and k82 repaired them too.

The subject is these parts of `docs/specs/owner-skill-delivery.md`:

- The section "Pack checkouts": the `grove-core` candidate and the advance
  rule.
- The section "Settings isolation and the staging HOME": the clone
  environment and the credential control.
- The section "The switch inside finish": the steps, the continuation after a
  stop or an inverse, the probes and the receipt.
- The order table and the staging trials of "Cutover contract".

The subject also includes step 5 and the recovery table of
`docs/specs/coordinated-release.md`. The evidence is
`docs/owner-skill-delivery-evidence.md`. k82's decision log records each
disposition and the verification.

The human's three decisions of 2026-11-02 stay closed. mise's
`[bootstrap.repos]` owns the owner's checkouts. Each checkout stays at a
published commit. No tool-neutral contract names mise.

Compare the finish order with `CLAUDE.md` and
`docs/adr/finish-promotes-after-the-grove-release.md`. Compare capture with
Ravel's local-source contract `7a894c782037` in the jj workspace
`Ravel.skill-directory-delivery`. Compare Grove's loop variables with the
section "Environment" of `docs/specs/harness-selection-and-execution.md`.

## Doubts to test

1. **The continuation after a stop or an inverse.** Test whether finish can
   always end without the success signal, and whether the owner can always
   complete activation later. Test the case where the inverse itself fails.
2. **The Grove patch release after a failed core trial.** Test it against the
   recovery rules of the coordinated release design and `docs/RELEASING.md`.
   Test whether the tree ID of the final line can still differ at the released
   commit.
3. **The advance rule in daily use.** Test whether the process check is
   enough while harness sessions run. Test `mac-bootstrap` and a pin that
   `mise dot sync` brings from another machine.
4. **The probe environment.** A probe runs outside the workspace without
   Grove's loop variables. Test whether any route remains by which a probe
   could end or finish this driver's loop.
5. **The credential control.** Test whether the routes are complete under Git's
   documented configuration and credential rules, and whether the control can
   fail.

## Done when

- Each finding names its file and line, its evidence and a severity.
- The review cuts an `integrate-review-design` leaf only if a finding needs
  action.

## Notes

k82 did not change `docs/evaluations/owner-skill-delivery/checkouts.py`. Its
recorded run passed 33 cases, and `--mutate` failed the one expected case.

## Decisions (running log)

The review inspected Grove `36f4bcb9` against its parent `03794cdd5645` and
Ravel's local-source contract `7a894c782037`. The human's three decisions
remain premises. No production, test or design artifact changes belong to
this leaf, and no test, build, lint or format command was run.

Three findings need integration: recovery when the inverse fails, completion
when activation passes but the receipt is not written, and the use of Git tree
identity as proof of captured checkout identity. The process interval,
credential allowlist and probe environment do not support an additional
finding within the inspected scope. The integration belongs immediately
before skill-loading-replan-k80.

## Findings

Locations refer to Grove `36f4bcb9`, the committed artifact of
owner-pack-checkouts-k82. P2 means that the design needs a correction before
dependent planning consumes it. These are design findings, not observed
failures of an implemented switch.

### 1. P2 — Recovery assumes that the inverse cannot fail

**Location:** `docs/specs/owner-skill-delivery.md:584`, with the inverse calls
at lines 569–570 and 578–579, and the continuation at lines 590–605.

The new continuation assumes that applying the inverse restores native
delivery and removes the skill configuration. The inverse itself changes
several files and moves retained entries. A failed write or move can leave
some native entries disabled or absent while Ravel settings still enable
directory delivery. Neither the owner specification nor the coordinated
release recovery table defines that state. The latter's row at
`docs/specs/coordinated-release.md:207` covers a stopped switch or one
reversed by its inverse, which is not evidence of successful restoration.

For example, step 9 writes the skill settings and moves some links, a later
move fails, and restoring one registry or link fails too. Restarting at step 1
then reaches step 3, which rejects the deliberately changed or missing native
entries against the original manifest. The normal path cannot reach a fresh
switch, and the claim that no launch captures a checkout at lines 586–588 is
no longer established. The pending record also need not name the original
backup or the operations that restoration did and did not complete; its
required fields cover a failed step or probe only. The receipt that names the
backup has not been written.

**Smallest correction:** distinguish a verified native restoration from an
incomplete inverse. Preserve and identify the original backup and actual
partial state before ending. Give the incomplete inverse a recovery entry
that repairs or resumes restoration, verifies the native route, and only then
permits the ordinary retry. Do not overwrite the original recovery material
with a backup of the partial state. Carry that distinction into the recovery
table and the cutover view, which currently labels the inverse as restoring
the native route unconditionally. This needs no new Ravel or Grove protocol.

### 2. P2 — Activation without its receipt has no completion-only retry

**Location:** `docs/specs/owner-skill-delivery.md:580`, with the blanket retry
at lines 597 and 601–605.

Step 9 can finish, both probes can pass, and then writing the receipt can
fail, or the session can end before that write. Delivery is active, but the
receipt is a required finish outcome. No inverse is specified for this case:
the inverse belongs to a failed switch operation or failed probe. The only
defined continuation repeats the entire switch from step 1.

That continuation is not usable for an already activated layout. Its step 3
compares the current baseline with the pre-switch manifest and must stop on
the entries that step 9 intentionally moved or changed. The same problem
occurs after a receipt was written but before completion was acknowledged:
the procedure has no branch that recognises an already completed activation.
The special rule for a failed `mac-check` at lines 624–627 handles that
command's result, not a missing receipt. The coordinated recovery table has
no row for activation that passed its probes but lacks completion evidence.

**Smallest correction:** identify the activated state from the prepared
manifest, retained backup and probe evidence, and define how an owner resumes
verification and receipt completion from it. A retry must distinguish that
state from a native layout and from an incomplete switch. Preserve enough
evidence before the final receipt write for a later session to make the
distinction. Reconcile the statement at line 768 that only finish runs the
real-HOME switch with the expressly permitted owner continuation. Release,
promotion and teardown must remain outside this completion retry.

### 3. P2 — Equal Git trees and a clean status do not prove equal captured resources

**Location:** `docs/specs/owner-skill-delivery.md:216`, with the added layout
rule at lines 140–142 and switch steps 4–6 at lines 547–560.

The tree comparison carries staging acceptance to the released core when the
tree IDs agree. That identifies the committed representation, but Ravel
captures the materialised checkout. The new prohibition covers transforming
Git attributes. It does not constrain checkout configuration. Staging clones
use the isolated Git environment at lines 420–431; the real-HOME `apply`
uses the owner's normal environment.

Git documents that `core.autocrlf=true` converts line endings on checkout
without a repository attribute, and that `core.symlinks=false` writes a
tracked link as a regular file containing its target. Neither operation
changes the Git tree ID, and Git can regard the result as clean. Thus a
real-HOME checkout can satisfy step 6 while holding different resources from
the accepted staging checkout. The symlink case is admitted by the design's
relative-link layout rule, yet it can break a supporting resource that the
small final kind-read probe does not exercise. Ravel's local-source contract
at `7a894c782037`, lines 276–294 and 324–342, captures the actual files and
links rather than reconstructing the Git tree.

**Evidence:** Git's [checkout conversion and symlink configuration](https://git-scm.com/docs/git-config#Documentation/git-config.txt-coreautocrlf).
A read-only query of this checkout's effective configuration found LFS
filters and `core.filemode=true`, but no setting of `core.autocrlf`,
`core.eol` or `core.symlinks`. This finding concerns the sufficiency of the
designed check; it does not assert that today's owner checkout is converted.

**Smallest correction:** make the materialised source identity the acceptance
condition. Either constrain and verify checkout configuration and the written
file representation in both homes, or compare the captured resource manifest
with the accepted one. Keep the Git tree ID for release provenance. A
difference in the resources must not bypass the affected staging trials just
because the committed tree is equal.

## Inspection evidence and limits

Read the producer diff against `03794cdd5645`, its decision log, the changed
specifications and evidence, and the affected diagram sources and summaries.
Compared the finish order with `CLAUDE.md`, the finish ADR and
`docs/RELEASING.md`. The new patch-release repair is compatible with the
owning procedure when a repair reaches `main` and partial publication resumes
at its unfinished step. Equal core trees are not assumed across finish's
integration of current `main`; the design explicitly compares the released
tree and retains the staging homes for a rerun.

The advance rule now names both capture boundaries and requires other harness
sessions and drivers to end before any complete bootstrap or `apply`. That
also covers a pin received through `mise dot sync`. Its operational cost is
explicit. The review found no additional contradiction in that interval.
Consulted the [official mise repository reference](https://mise.jdx.dev/bootstrap/repos.html)
for pin, drift and local-work behaviour. The 33 fixture results and the failed
mutation remain k82's recorded evidence; this review did not rerun them.

Compared the clone allowlist and decoy control with Git's
[configuration environment](https://git-scm.com/docs/git-config#_environment)
and [credential request rules](https://git-scm.com/docs/gitcredentials#_requesting_credentials).
The fresh environment excludes inherited configuration and askpass programs;
the positive cases remove the blocking entry for their route. A local HTTP
remote that demands credentials can exercise those cases without the real
keychain. No clone or credential control was executed in this inspection.

For the probe environment, inspected the harness specification's complete
control list and followed `cmd_record_teardown` through its graph trace to
the exact teardown implementation. Also inspected Ravel's `end_session` and
its outbound trace. Removing `GROVE_LAUNCH_DIR` removes ordinary teardown
authority, removing `RAVEL_EXIT_FILE` removes the enclosing result channel,
and a nested launch gives its managed child a new context. Running outside
every Grove workspace also avoids ordinary tree discovery. This is evidence
about cooperating probes, not authentication against hostile code of the same
user. No additional probe-authority finding is supported.

Graph verification used Tier 2 in
`Users-antony-Development-AgenticOrchestration-grove.gh-issue-25`, generation
`2026-11-02T10:03:26Z`. Searches and traces returned complete pages. Coverage
reported no recorded gap and matching metadata for the relied-on Markdown
and Rust files. It reported partial Mermaid ranges 1–33 for
`owner-pack-checkouts.mmd` and 1–40 for `owner-cutover.mmd`; both were read in
full. Ravel's project was ready at generation `2026-11-02T05:30:19Z`, with
matching coverage for `crates/ravel/src/completion.rs`. Its `docs/` subtree
is excluded from that index, so the relied-on contract sections and local
source ADR were read directly. The workspace parent is exactly
`7a894c7820372b753775ce1c6778023b26f99fb4`. Coverage is a best-effort signal,
not proof of repository-wide completeness.

The finish ADR continues to own one decision and points to the two
specifications for their separate procedures. No additional ADR or interface
is needed to address these findings. Integration owns the fixes and all
post-fix verification.
