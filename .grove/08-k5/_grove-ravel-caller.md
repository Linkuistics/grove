# grove-ravel-caller-k5 — brief

## Goal

Make Grove's lifecycle and standalone invocations consume installed Ravel only
through start-session/end-session, with caller-owned prompts, result meanings,
policy adapter and provenance.

## Context

- Requires ravel-publication-k11's independently installed release and k3's
  documented rename/report/layout contract.
- Actual lifecycle and standalone callers now use installed Ravel 0.1.0 through
  `dispatch::ravel::locate`, shared public session_command builders and the held
  SessionResult reader. The resolver trusts executable absolute PATH entries,
  canonicalizes/version-checks once and scrubs version-probe authority. The old
  sibling locator and JSON ending readers are gone. Launch and prompt ending
  commands use that exact selected path; standalone public confinement reads
  resolve before staging cwd. The caller ADR records the PATH trust costs.
- Preserve the current adapter, Grove examples/sample, fixtures and command
  tests from the old dispatch package. The adapter uses the generic SDK and
  public task conventions; deliver it through Grove-owned readable assets and
  ordinary documented policy imports, never Ravel's embedded module registry.

## Done when

- Lifecycle and standalone paths resolve the installed `ravel` independently
  of Grove's real executable directory/version, with an actionable missing-tool
  remedy. Resolve once and confine that executable and its documented runtime
  layout. Grove consumes the public result, never private worker protocol. It
  supplies instructions to call ravel end-session with its continue value.
  Check the documented Ravel CLI version against the minimum delivered contract
  before launching; unknown/older versions refuse with an upgrade remedy.
  Homebrew depends_on ensures presence, not this runtime contract check.
- Caller-owned adapter/sample/examples and their typecheck/runtime fixtures work
  with Ravel's generic routing/context SDK. Grove owns any Creator/Reviews and
  provider provenance it still needs; remove reliance on Ravel's retired host.run,
  record commands or persistent store. Supply needed data as caller context.
  Ravel contains no Grove module registration or prompt injection.
- Extend existing lifecycle/standalone fixtures to execute the real installed
  Ravel and worker, with fake agents and temporary policies. Put Grove and Ravel
  in separate prefixes with different versions; a decoy old sibling must not
  win. Missing Ravel reports its install remedy. No host Bun/Node is required.
  Replace test support's old sibling symlink seam with isolated PATH/prefix
  fixtures; exercise a decoy older Ravel, malformed version output and a cwd
  decoy. Freeze Grove's continue result fixture and verify both actual readers accept
  that value and reject unsupported/missing results. Confirm session output
  does not contaminate the returned result. k8 extends caller-owned meanings.
- Existing continuation, completion, refusal/resume, cancellation, process-group
  cleanup, PTY recovery, confinement, epoch admission and stale-session checks
  pass. This leaf retains ordinary continuation using Grove's explicit result value;
  its prompts now instruct the session to invoke ravel end-session with it.
  Flip every current Grove command, environment, SDK/import and report-source
  spelling together: prompt/signalling text, tree-lifecycle/standalone prose,
  grove-llm CLI, finish skill and Creator instructions, assertions, and the
  loop/standalone/testing-support/cargo scrub lists. Scrub both old and new
  control sets. New provenance instructions use caller-owned data, with explicit
  legacy handling for sessions still launched by the original driver. End-session
  discovers process context; a session ID is not an ending-command argument.
  Enumerate/classify the complete Grove surfaces, including dotfiles, with the
  spine's observed positive/cross-tree controls; keep only named compatibility
  and historical exceptions. Source changes do not change the live prompt.
- Grove's runtime and caller documentation describe the executable boundary.
  Preserve its local keyed-launch implementation and authority guards; no Rust
  dependency points into Ravel. Run the repository `task check` after migration.
  Update source-pinned walkthrough blocks/manifests/indexes in this same leaf,
  wherever its source changed (overview, grove-loop and any other affected
  corpus). Update release-notes.sh's old dispatch:build prerequisite and
  release-prepare.test.sh's command/build assertions with the caller migration.
  Keep old package archive checks working until k6; k12 owns broader prose.

## Notes

Keep the old bundled package/build recipe temporarily where needed to leave
current archive checks green; grove-ravel-delivery-k6 removes that delivery.
Do not make new source prompts or SDK names alter this session's original
installed driver/exit contract. Record the Ravel version used for integration.

## Execution slices

This migration exceeds a focused session. Sequence expand → migrate → contract:

- grove-ravel-resolution-k26 adds the independently testable PATH/version resolver
  beside the old locator and documents its trust decision. Existing launches stay
  green while the new contract is tested against installed Ravel 0.1.0.
- grove-ravel-runtime-k27 switches both actual launch/read paths, confinement,
  prompts and end-session commands as one atomic consumer migration; extends the
  installed-pair and lifecycle fixtures and updates every changed source corpus.
- grove-ravel-policy-k28 transfers Grove-owned adapter/sample/examples and
  caller-context provenance to the generic SDK, with typecheck/runtime fixtures.
- grove-ravel-contract-k29 removes temporary migration scaffolding and audits
  every current spelling/control surface, including dotfiles, source-pinned
  books and release helper assumptions; runs the final repository task check.

Each child must leave its deliverable independently verifiable. The parent
Done when remains binding; k6 owns removal of bundled package delivery and k12
owns the broader prose reconciliation. Do not change the live v22 ending channel.

## Actual consumer handoff — runtime switch subtree closed

k39 delivers the atomic caller migration, completing grove-ravel-atomic-switch-k37,
grove-ravel-migrate-k35, grove-ravel-switch-k33, grove-ravel-consumers-k31 and
grove-ravel-runtime-k27. Its terminal handoff records exact logs, controls and
frozen input comparisons. Both actual consumers now accept only grove-continue
from held stdout after reap and their ordered authority checks. Runtime nextest
passes 827 unique cases, plus installed and authority checks; all three changed
source books, release helper/asset fixtures and skill conformance pass. All 2,022
measured subjects match, and installed original v22/Ravel bytes are preserved.

Future siblings consume the active public boundary, not staged commands or
private worker/store reports. Current prompts and finish guidance name the exact
canonical end-session command; legacy v22 sessions retain their original dispatch
exit/admission/teardown. k28 retains ownership of generic adapter/sample/examples
and caller-owned provenance; current Ravel has no run lookup, identity or store.
The caller fixtures preserve review retirement/reordering/cascade and explicit
creator refusal/declaration resume without embedded Ravel adapters. k29 owns the
final complete surface audit and repository task check (with explicit
GROVE_TEST_RAVEL=/opt/homebrew/bin/ravel and the pinned Bun directory for retained
package checks); k6 owns bundled delivery removal and k12 broader prose. This
node remains live for k28/k29. No installation or new driver was launched.

## Policy/provenance handoff — grove-ravel-policy-k28 closed

k40/k41/k42 deliver the complete readable `assets/ravel-policy/` set: static
sample/example, optional `withProvenance` producer wrapper, measured Grove task
adapter, generic review rule and composed creator-aware Grove review example.
The composed policy wraps its final selection once. Imports are ordinary local
files plus Ravel's public SDK/static module, never an embedded Grove registry.
`docs/grove-provider-evidence.md` owns the exact receipt/context contract and
`assets/ravel-policy/README.md` its import/composition instructions.

The finishing session's recorded receipt, or the owner's explicit declaration,
becomes measured `facts.creator`; `facts.reviewedArtifact` names the producer.
Raw inline records require canonical bytes; parsed generic JSON records validate
exact fields/values independent of transport key order. Exact own-key origin
entries select another provider. Missing/invalid/duplicate/unknown/legacy origins,
same-provider gateways and conflicting caller review facts refuse with remedies.
Current routing never supplies past creator identity; node closes can carry a
closing child's task/kind. Actual producer retirement/retry/cascade and review
consumption run together against independent installed Ravel 0.1.0 with fake
agents. Review retries preserve the finishing evidence after producer remapping.

k42's terminal handoff pins core assets and gives complete saved logs/digests.
`task ravel:review RAVEL=/opt/homebrew/bin/ravel` passes 88 unique nextest cases,
public/retained SDK types and shipped conformance; formatting and workspace
all-target clippy pass. All 3,046 measured subjects retain their individual
hashes, including owner policy, complete original v22 keg and independent Ravel.
The scoped reviewer found no issues; same-origin and receipt/provider-agreement
mutation controls fired. No changed source belongs to any source-book corpus.
Ravel/tap remain clean successors of the published `3916a4a1`/`bd5aa37b` states.

This closes k28 only. k29 must consume the complete new asset tests in its
repository acceptance and classify remaining whole-repository spellings.
k6 must ship the readable directory structure and relocate the policy
compiler/tsconfig/typecheck seam out of the retained dispatch crate before
removing that package; normal gates must keep installed-Ravel policy/provenance
checks while dropping legacy compatibility checks. k12 retains broader prose
reconciliation. Those are already named live tasks, not unresolved k28 work.
No personal choices, arguments, bounds or grants were edited. The original
installed-v22 driver's admission/teardown and `harness-dispatch exit` remain live.
