# harness-selection-and-execution-k7

**Integrates:** harness-selection-and-execution-k5

## Goal

Triage the design review's findings against the accepted requirements and
current Grove contracts, and apply the real ones to the harness-dispatch design
before planning cuts implementation increments.

## Context

Read the findings from the commit of `harness-selection-and-execution-k5`, not
from this body. The reviewed artifact is the committed work of
`harness-selection-and-execution-k3`: `docs/specs/harness-selection-and-execution.md`,
`docs/adr/harness-selection-is-owned-by-policy.md`,
`docs/adr/policy-evaluation-precedes-process-replacement.md`, the glossary entries
it added to `CONTEXT.md`, and `docs/design/harness-selection-and-execution/`.
The root brief and the k1/k4 running logs remain the requirements authority.

## Done when

- Each finding is classified — contract stated unclearly, real issue, visible
  trade-off, or noise — with the reason recorded in this leaf's running log.
- Accepted findings are applied to the spec, the ADR set (reworked in place, with
  citations reconciled) and the design evidence; choices only the human can make
  are put to the human with a recommendation and evidence.
- Settled human requirements stay settled; design choices the review questions
  are re-decided on evidence, not reopened as requirements.
- `harness-selection-and-execution-k6` can plan against a design that needs no
  further review, or a substantial redesign is externalised as its own producer
  and review chain ahead of planning.

## Notes

Several findings turn on trade-offs the human has not accepted rather than on
edits alone; stopping to ask is expected for this kind. New runtime claims made
while integrating need their own probes and positive controls rather than
restated documentation.

## Decisions (running log)

Evidence: findings read from review commit `9b4941b42281`
(`harness-selection-and-execution-k5`); the reviewed artifacts are unchanged
since design commit `843d0dee84e8`, so the review's spec line numbers still
apply. Checked against the root brief, the k1/k4 logs, the cited ADRs, Rust std
source for the 1.93.1 (default) and 1.98.1 toolchains, the grove-loop tree
reader, Bun 1.4.2 documentation, and a new native Bun 1.4.2 probe (recorded in
the runtime evidence document).

F3 — real issue. Recognising reviews by configured kind lets a task that
declares `**Reviews:**` under an unlisted kind take a static route. The Grove
example will read the supplied task on every invocation and refuse a review
declaration under a kind that is not a configured review entry. It therefore
requires the task file and identity.

F4 — real issue. Exact free-text inequality passes a misspelt declaration or a
relabelled origin. The supplied policy will refuse a creator provider that is not
an exact member of the current catalog's provider-origin set, and name the
correction remedy. Comparison stays exact and case-sensitive, with no
normalisation. The seam gains label-drift and misspelt-declaration cases.

F5 — unnecessary obligation. Resolving the declared handle to a task header
needs a directory listing and a root input the SDK lacks, and it adds almost no
protection: a mistyped handle that names a real task resolves anyway. Reading
ancestor briefs has the same listing problem. The adapter will read only the
supplied task file, and lookup by (scope, handle) refuses when nothing is known.

F6 — real contract gap. Chosen: the worker registers each documented package
specifier (SDK, Grove adapter, examples) as a runtime virtual module. The probe
showed a registered specifier beating a `node_modules` shadow beside the policy,
including beside an admitted entry, while the unregistered build loaded that
shadow. Owner policy imports the running worker's own SDK, so there is no version
skew across upgrades and no install path to name. Reserving the whole prefix
through `onResolve` did not intercept an unregistered name, so every documented
specifier must be registered. Types ship as declarations for owner checking.

F8 — real evidence incoherence. The ADR overstated which ambient classes the
design leaf had shown to be dangerous. The claim is now restated at its measured
scope, with a firing configuration for each class taken from the new probe.
tsconfig `paths` beside the entry fire only in a build with tsconfig autoload
enabled; a cwd tsconfig did not fire in either build. A `node_modules` shadow
beside the entry fires without the virtual registration. The private empty worker
cwd alone kept cwd dotenv/bunfig inert in a default build. A `~/.bunfig.toml`
preload fired in neither build, so no control is claimed for it.

F9 — partly real. Noise: Rust std's `exec` does not reset the signal mask on
either toolchain; it inherits the mask and resets only SIGPIPE, before any
`pre_exec` hook runs. Real: the SIGPIPE disposition and "default" versus
"inherited" dispositions, cancellation observed after the handoff commit, and
the undefined pre-handoff deadline. The contract becomes a transparent handoff
of the inherited mask and exec-surviving dispositions. A cancellation check after
the commit appends a best-effort not-executed detail. The lock wait gets its own
bound, outside the selection bound.

F10 — real issue, low. `inspect` will accept an omitted prompt, rendering a
marked placeholder in the argv, since selection never sees the prompt. A refused
delegated `run` prints the equivalent `inspect` invocation.

F11 — handled item by item:

- Association wording: real. A run associates with its task identity, a reviewed
  artifact, both, or neither.
- Static routes with `--choice`: contract stated unclearly. A static table
  accepts any configured candidate named by an explicit choice, including for an
  unrouted kind, and cannot refuse one; constraints need `select`.
- Exactly one of `routes`/`select`: visible trade-off, kept for protocol
  simplicity. The composed example reports the static entry it applied in its
  reason, and inspection shows that.
- Glossary "Grove's adapter supplies": real; Grove supplies the scope.
- Which refusals are recorded: real. Pre-handoff refusals are diagnostics with no
  run; only an exec error or a post-commit cancellation appends to an attempt.
- `--policy-env` and completion variables: contract stated unclearly. The generic
  core cannot know a caller's variable names. Granting one is an explicit owner
  act, and the documentation says so.
- Policy ADR duplicating contract text: real. The ADR is trimmed to the decision
  and points at the spec.
- Adapter placement for extraction: visible trade-off. It ships with the package
  for SDK version coherence, depends only on the public SDK and Grove's documented
  task conventions, and can move to Grove's side at extraction.

F1, F2 and F7 turn on trade-offs that belong to the human. They were put to the
human with recommendations before being applied.

Human resolution of F7: adopt the recommendation. The Linux floor has three
dimensions. glibc 2.17 is executed in an old-userland container and the baseline
CPU under emulation on each Linux target. The worker's kernel floor is Bun's
documented range, labelled documented-not-executed and rechecked at each Bun
upgrade. This is an accepted, visible trade-off: RHEL 7-era kernels are claimed,
not tested.

F1 and F2 — real issues whose remedy the human reopened. Asked first whether to
derive the creator from dispatch's own records, and where the dispatch scope
should live. The human questioned why creators need registering at all. They
proposed recording the provider/model actually used on the task, perhaps as the
last step before closing it or from `grove-llm complete`, and asked what an
"evidence namespace for task handles" is for.

Explanation given: handles restart at `k1` in every grove, and the design needed
the scope only because creator records sit outside the tree in a per-user store
looked up by handle. A concrete follow-up shape was put to the human:
dispatch exports the chosen candidate to the harness environment,
`grove-llm leaf-retire` writes a `**Produced-by:**` line, and the review policy
verifies its run ID. That shape reverses two binding methodology rules
(`body-carries-no-launch-metadata`, `retirement-is-filename-only`) and the brief's
"outside task bodies / namespace handles" wording. `complete` was ruled out as the
writer because it runs after the task's commit.

The human did not adopt that shape. They suggested instead: the grove can be
identified from the task file passed to the dispatcher; for a review, use the
reviewed task's handle, find that task through `grove-llm`, and read which
provider/model produced it. Alternatively, the Grove runner could be responsible
for telling the dispatcher the reviewed artifact's provider/model. The human
judged the issue tricky and in need of more thought.

Human resolution of placement: the provenance rework is a substantial redesign,
so it is externalised as a new `design` leaf followed by a `review-design` leaf,
both ahead of planning. F3, F4, F5, F11's association wording and the glossary's
dispatch-scope entry sit in the area being redesigned. They are left for that
design, which must satisfy their dispositions above or record why they no
longer apply. This session applies the independent repairs (F6–F11 otherwise,
and F7).

Further human input on F1: the artifact's creator is normally the session that
cuts the review task, so it could write its own provider/model into that review
task. The dispatcher receives the review task file anyway, so no cross-task or
cross-grove lookup would be needed. Recorded as a leading option for the
redesign, alongside the human's earlier suggestions. It still amends
`body-carries-no-launch-metadata`, and an eagerly cut review not written by the
creator needs its own remedy.

For the same reason, this session cuts only the `design` leaf, ahead of planning.
That design leaf must cut its own `review-design` leaf with `leaf-insert` at the
planning leaf as its last act. This keeps the lazy producer-cuts-review chain and
satisfies the human's choice that the redesign is reviewed before planning.

## Verification

Applied: F6, F8, F9, F10 and the independent F11 items to the spec, both ADRs,
the runtime evidence and the design README, and the human's F7 choice to the
spec and root brief. The provenance area carries a redesign note in the spec
header, and design `harness-selection-and-execution-k8` was inserted ahead of
planning `harness-selection-and-execution-k6`, whose body now points at it. k8
must cut its own `review-design` before planning.

The integration probe's runtime claims each have a positive control that was
seen to fire, recorded in the runtime evidence. The F9 claims rest on Rust std
source for both toolchains in use, not on a runtime probe; the spec leaves
their observation to the command and PTY seams. Every spec, evidence and README
anchor the edits cite resolved, checked against an absent-anchor control that
came back clean.

`task check` exited 0: all eleven principal checks passed, including the full
Rust suite (1449 tests, 0 failed) and final validation of all six walkthrough
books. The jj working-copy commit stayed `5e169242c22d` before and after the run.
The log was kept in this session's scratch space. This note and the retirement
rename follow the run and change no checked contract. Nothing is claimed as
implemented.

The design leaf is live, so there is no ancestor close to cascade.
