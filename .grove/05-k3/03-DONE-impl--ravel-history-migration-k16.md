# ravel-history-migration-k16

## Goal

Make runnable Ravel use the owner's preserved configuration. Historical
continuity was explicitly rejected; the existing handle is retained.

## Context

Requires ravel-runtime-k15. Read its exact Ravel commit, current CLI docs and
root/ravel-cli-k3 contracts. Recheck source ~/.config/harness-dispatch and
destination ~/.config/ravel contents; prior inspection is not a lock.

## Done when

- Copy actual personal configuration, preserving source contents/destination
  edits, selection choices, bounds and grants. Relocate necessary SDK/caller
  imports. Remove obsolete destination storage settings; no history access.
- Typecheck the copied policy against Ravel's public SDK and validate applicable
  routing with temporary fake agents, using deterministic tools. Do not invoke
  a real model. The two-verb CLI has no inspect or init command.
- Document the selected configuration and validate that the new tool uses no
  old store. Open/copy/import no personal records; no snapshot or catch-up work.
- Commit project docs through jj in Ravel, record exact commit/validation here,
  retire, check/promote k3's complete contract and seal the coordination change.
  Preserve the old installed front and this session's original ending command.

## Notes

Publication/release preparation remain k4/k11. Source configuration and the
original driver remain intact. Any Grove-specific provenance is caller-owned;
Ravel offers no record lookup or durable store.

## Decisions (running log)

**Copy boundary and plan.** Rechecked source/destination: the source holds only
policy.ts; settings.json and the entire destination are absent. Copy exclusively
into a new destination, preserving the source bytes/mode. Relocate only the SDK
import and update obsolete inspection comments; keep the owner-maintained
version label, every route, argument and secondary-jj-workspace grant. There are
no explicit bounds, policy environment grants, choice imports or storage settings
to migrate. The old/new default bounds match. Validate against generated public
SDK declarations and the real front/worker with a private fake-only PATH, cleared
environment and temporary workspaces. Cover every mapped kind, unknown-kind and
unreadable-jj-store refusals, ordinary/empty/multiline prompts and both workspace
forms. Finish project documentation before measuring; save deterministic logs,
digest every validation input before/after, commit Ravel docs, then retire and
close/promote k3. No history access, model launch, publication or installed-front
replacement belongs to this leaf.

**Evidence.** Tier 2 graph lookup in Ravel found only retired symbols. Its
2026-09-08 generation predates the replacement and excludes docs; current source
reads of the SDK, settings, bounds, inputs, environment, policy/front and existing
installed-smoke instruments supply the fallback. Grove's 2026-10-12T11:58:44Z
coverage matches the source bounds and brief/task paths with no recorded gap.

**Configuration preserved.** The exclusive copy retained file mode 0644 and the
source SHA-256 `59a54bfae00295a887ab87283e7ba0a794c456a437fb142ff19a6a3d87e2275d`.
Destination SHA-256 is
`7ca627d265fb8b3d81a1f7f710aa42298eac7aa6369fec92e126f608efd3196e`.
Everything from the first import through the final policy statement is
byte-identical after the sole SDK import substitution. Header comments replace
the obsolete inspect guidance; owner version `harness-dispatch 2` remains.
The policy has no caller SDK helper to relocate and never consults a choice file.
No ADR is earned by this reversible configuration copy; existing runtime
decisions remain current. The new guide is explicitly classified as caller-owned
configuration documentation in Ravel's caller-reference inventory.

**Validation instrument corrected.** The first routing run passed 80 selected
sessions, then its refusal assertion used the wrong JSON envelope. A direct
unknown-kind reproduction and refusal.rs confirmed the policy code belongs under
error. The corrected final instrument passed every case; its deliberately wrong
argument control fails in the fake agent, and a deliberately old SDK import fails
typechecking. No product source or owner policy behavior changed for this repair.

**Final configuration validation.** Strict copied-policy typechecking against
generated public SDK declarations and `task typecheck` passed. `task test:cli`
passed all 11 command-boundary tests. The final real-pair run passed 81 selected
fake-agent sessions plus 3 refusals: all 24 routes across plain/primary/secondary
jj directories, both providers' empty/flag-like/multiline prompts, the actual
Grove secondary workspace/main-repository grant, directory-store no-grant behavior,
unknown kinds, empty jj references and obsolete stateDir refusal. Exact arguments,
labels, result bytes, default bounds, absent environment grants and temporary
cleanup were asserted. Private fake-only PATH/cleared child environment prevented
model execution and inherited authority. No old personal store was opened;
storage checks used a private scratch sentinel. Both owner directories retain
only policy.ts. The original installed commands remain version 22.1.0, and their
digests and the real exit channel's absence survived the run.

**One narrow reviewer.** The review found no actionable issues in the copy,
configuration guide or validation evidence. It independently compared the policy
diff/modes/digests, per-case records and negative controls, and confirmed the
before/after inventories match. No product or configuration fix was needed.
The old retired bookmark is still remotely retrievable at its named revision;
the Homebrew tap is clean, with ravel-lite.rb intact and no ravel.rb yet. The
pending publication leaf's obsolete record-case wording was reconciled to the
root's two-verb opaque-result contract.

## Validation and handoff

Ravel commit: `51b73cbffaa46882d7f9ebad15058e17dbd7bec4`
(`ravel-history-migration-k16: document and validate preserved owner configuration`).
Local main names this sealed descendant of runtime commit
`b1060a176d5fa7c6c5bc96f41cacc18480e0fb65`; its following working copy is empty.
Only README.md, docs/session-cli.md, docs/caller-references.md and the new
docs/owner-configuration.md changed in Ravel. No push, publication, installation,
model launch or history access occurred. The source configuration and original
installed driver/front remain intact; this session ends with harness-dispatch exit.

Saved evidence under `/Users/antony/Development/Ravel/target/`:

- `k16-copy.{mjs,json}` records the exclusive copy and source/destination modes,
  inventories and digests; `k16-code-equivalence.log` proves implementation bytes
  unchanged after import substitution.
- `k16-policy-tsconfig.json` and `k16-policy-typecheck.log` hold the strict copied
  policy/declaration check. `k16-typecheck.log` and `worker-check.PxEsSx` hold the
  complete `task typecheck` results.
- `k16-cli.log` holds the complete visible nextest run: 11 tests passed.
- `k16-routing.mjs`, `k16-routing.log` and `k16-routing-results.json` retain the
  exact deterministic instrument and each of 81 successful selections/3 refusals.
  `k16-routing-first.log` retains the instrument-envelope failure; the final run
  confirms every item, rather than comparing only totals.
- `k16-routing-control.log` and `k16-typecheck-control.log` retain observed
  wrong-argument and old-SDK-import failures. No personal policy was mutated for
  those controls.
- `k16-measure.mjs` and `k16-measure-{subjects,before,after}.json` identify all
  1,145 subjects and matching individual SHA-256 readings: every tracked project
  source/config/doc, worker development dependency, generated worker/SDK/notice,
  pinned runtime, front/CLI test executable, nextest executable, validation
  script/tsconfig, both owner policies and original installed fronts. No subject
  changed during the final measurements; both inventories and modes also match.
- `k16-doc-links.log`: all 16 local links in the four changed Markdown files
  resolve. The full runtime/install/hostile/terminal/confinement handoff remains
  k23's committed evidence; product code and compiled runtime inputs did not
  change here. k4 owns supported-target/archive/release preparation, k11 publication.

k3's full Done when is met by k14's preserved retired revision, k17/k21/k22/k23's
runtime and installed-layout handoffs, and this completed configuration boundary.
Promote the exact source/configuration handoff to the root for k4 and later callers.
The root remains live for delivery, publication and Grove integration/release work.
