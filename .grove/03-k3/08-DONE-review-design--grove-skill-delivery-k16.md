# grove-skill-delivery-k16

**Reviews:** grove-skill-delivery-k8

## Goal

Adversarially check Grove's corrected skill-delivery design before implementation
planning treats it as a settled contract.

## Context

Read the producer's commit by its stable handle and the current requirements.
The human corrected the design. Grove names a skill. The user supplies it
natively or through Ravel directory injection, and the agent finds and reads it.
Ravel does not resolve a requested skill. Consume ravel-skill-directories-k15
and any integrated review of it, including its owning-repository commits.

The artifact set is Grove's delivery and caller specs, related ADRs, glossary
notes, design evidence and caller diagrams. Reconcile the producer's historical
diff with any deliberate upstream correction. Do not review the discarded
catalog/Ravel-only proposal as if it were still the artifact.

## Done when

- Check that native installation and injected directories are user choices that
  the design supports. Check that no skill presence, location, catalog or kind
  registry is in Grove's public contract.
- Challenge fresh bootstrap, missing-skill reporting, relative and cross-pack
  resources, conformance boundaries and native-hook coexistence against the
  corrected Ravel contract. A readable directory must not stand in for actual
  agent discovery or usable native integrations.
- Challenge old-reader preservation, explicit cleanup ownership, dual-major
  session compatibility, versioned policy examples and publication/activation
  order. Check that original live ending commands remain usable.
- Assess whether the source-consumer/documentation map and acceptance seams let
  common-skill-loading-k9 cut working increments without inventing behaviour.
  Keep implementation, harness measurements and documentation validation distinct.
- Evaluate the ADR set for necessary, nonduplicated decisions. Produce findings
  only, classifying genuine contradictions separately from visible trade-offs.

## Notes

Read-only review. Cut an integrate-review-design leaf only for findings worth
acting on, immediately before the first later live sibling. This review spends
no in-session reviewer. Implementation remains the existing planner's work.

## Findings

Each finding names its location, the failure it allows, the smallest useful
correction and a severity. High means the contract is false or has no owner as
written. Medium means a reader can act on two readings, or a real failure has
no remedy. Low means a gap that the acceptance table cannot see.

No finding reached high. Visible costs follow the findings in their own
section. Findings 1 to 4 concern interfaces or state and close with a
codebase-design record.

### 1. The rollout order and the unversioned formula dependency contradict each other (medium)

Location: `docs/specs/codex-skill-provisioning.md`, "Packaging and rollout",
paragraph 2, and `scripts/templates/grove.rb.tmpl`, the `depends_on` line and
the test block.

The spec says to publish Ravel's feature release before a Grove release that
advertises it. It also says to avoid an undirected shared upgrade. Grove's
formula depends on `linkuistics/taps/ravel` with no version bound. Its test
asserts a `ravel 1.` version. Shipped Grove refuses major 2, and its remedy is
`brew upgrade ravel`.

Once the tap's `ravel` formula moves to 2.0, `brew upgrade` and a fresh
`brew install grove` pair Grove 22.8 with Ravel 2. Every launch then refuses.
The remedy repeats the upgrade that caused it. The spec records the hazard as
advice, and the "Installed distribution" row checks no dependency bound.

Consequence: a Homebrew user who upgrades between the two releases loses every
launch. That window is the order the spec prescribes.

Smallest correction: state the formula obligation in the rollout section. Two
orders satisfy it. Grove admits both majors before the tap's unversioned
`ravel` formula moves to 2.x. Or the tap publishes 2.x under a versioned
formula name until Grove admits it. Add that check to the "Installed
distribution" row. Replace the resolver's upgrade remedy with one that names
compatible versions.

### 2. A session that cannot load its mandated skill has no named ending (medium)

Location: `docs/specs/codex-skill-provisioning.md`, "Installation and
bootstrap", paragraph 4, and "Test seams", the "Offline and failures" row.
Source: `crates/grove-loop/src/prompt.rs`, `SIGNALLING_CONTRACT`.

The spec says the agent reports the missing guidance and leaves the task live,
and the user reruns. The mandate is the only Grove text such a session holds,
because the skill that states the ending is the missing one. The mandate names
`grove-continue` after retirement and commit, and `grove-force-stop` for
stopping without completion. The spec names neither.

Under one reading the agent sends `grove-continue`. The loop then picks the
same leaf and relaunches it, and an unattended run repeats until a human
intervenes. Under the other reading the agent sends `grove-force-stop` or ends
its turn, and the loop stops with the leaf live. The open-kind decision assumed
a human present to read the report. User-managed delivery makes the
missing-skill case ordinary for every kind, including unattended ones.

Consequence: the acceptance row observes the report but not the loop outcome,
so a relaunch loop would pass it.

Smallest correction: state that a session that cannot load its mandated skill
ends with the force-stop command after its report. Put that sentence in the
mandate, because the absent skill cannot supply it and the too-late test admits
it. Add to the "Offline and failures" row that the loop stops with the leaf
live and does not relaunch.

### 3. The native-only acceptance row cannot pass for the confined standalone caller (medium)

Location: `docs/specs/codex-skill-provisioning.md`, "Installation and
bootstrap", paragraph 2, and "Test seams", the "Native-only bootstrap" row.
Source: Ravel `docs/book/src/09-jobs-signals-and-the-terminal.md`, "Apply
filesystem confinement", and Grove `docs/specs/standalone-invocations.md`.

The row launches both callers with complete native skills and injection
disabled. The spec calls the two routes alternatives.

Ravel's confinement table
grants a confined process no reads outside cwd, the run directory, named
runtime files and system resources. HOME's presence grants nothing under it. A
`--runtime-read` grant names one regular file. Native skill installations live
under HOME as directories, so a confined `grove run` session cannot read them.
Only Ravel 2's snapshot grants reach a confined process, and that is the
directory route.

Consequence: on the native route, a `grove run` session has no skills at all.
The row is unsatisfiable for that caller as written, and the "routes are
alternatives" claim holds for the lifecycle caller only.

Smallest correction: state that confinement denies native installations to the
standalone caller. Limit the native-only row to the lifecycle caller, or state
that native-only standalone runs without skills. Name directory injection as
the only standalone route to skills.

### 4. The mandate's namespaced spelling assumes every kind skill ships in the `grove` plugin (medium)

Location: `docs/specs/codex-skill-provisioning.md`, "Launch and
compatibility", paragraph 2, and "Resources and conformance", paragraph 1.
Source: `crates/grove-loop/src/prompt.rs`, `PLUGIN` and `LOAD_INSTRUCTION`,
`plugins/grove/README.md`, "The token a prompt names", and
`crates/grove-loop/tests/prompt.rs`.

The spec endorses Grove's plugin-qualified spelling for Claude Code as a name
request. The mandate renders that spelling as `grove:grove-<kind>` from one
constant, and the test pins it against the shipped `grove` plugin. The root
brief moves `grove-design`, `grove-impl`, the research and editorial kinds into
other repositories, each a plugin with its own name. After that move a native
Claude Code install registers `grove-design` under another namespace, and the
mandate's spelling names a skill that does not exist.

The spec's cross-pack paragraph says the agent locates a skill through the
active delivery route. It does not say that the namespaced spelling derives
from the plugin that ships the kind skill, which the split changes. No
acceptance row launches a native Claude Code install after the split. This
binds repository-moves-k5 before common-skill-loading-k9, but both planners
read this spec as the contract.

Consequence: the one-imperative-one-target property the wording test measured
breaks for every moved kind on the native route, and the failure is by wording
alone.

Smallest correction: state in the delivery spec that the namespaced spelling
derives from the shipping plugin and that the source split revisits it. Add a
native Claude Code case after the split to the acceptance matrix, or assign it
to repository-moves explicitly.

### 5. The new decision record's slug names the discarded framing (low)

Location: `docs/adr/skill-sources-are-explicit.md`, title and filename, with
citations in `CONTEXT-MAP.md`, the delivery spec and the evidence document.

The record's title is "Skill delivery is the user's choice". Its slug is
`skill-sources-are-explicit`. The slug is the record's identity under
`ADR-FORMAT.md`. "Sources are explicit" describes the pre-correction decision,
in which fresh setup had to configure a Ravel source. The human replaced that
decision in the producing session. A reader who cites the slug cites the
rejected framing.

Smallest correction: rename the file to match the title and reconcile the
three citations.

### 6. The record set restates one decision three times and leaves another in the evidence table (low)

Location: the three modified records, each at the paragraph that mentions
directory injection, and `docs/grove-skill-delivery-design-evidence.md`,
"Interface and alternatives", the last row.

Each modified record restates that Ravel injects directory paths and that
Grove resolves no skill. The delivery record already holds that decision.
`ADR-FORMAT.md` asks a record to cite, not restate, so three copies can drift.

The evidence table's last row records dual-major admission, its rejected
Ravel-2-only alternative and its reversal condition. No record holds that
alternative or condition. The supervision record admits 2.x but lists no
single-major option. The upstream review raised the same shape, and its
integration moved rows into records.

Smallest correction: reduce each restatement to a citation of the delivery
record. Add the single-major alternative and its reversal condition to the
supervision record's options. Reduce the evidence row to a pointer.

### 7. The caller views still describe the upstream correction as pending (low)

Location: `docs/design/harness-selection-and-execution/README.md`, paragraph
2, `diagrams.json`, the `intro` and `discussion` fields, and
`skill-transition.mmd`, step 1.

The README says Ravel's earlier catalog design needs correction before
implementation. The manifest's intro and discussion say the same, with the
status "detailed design pending review". The transition diagram's first step is
"Correct Ravel design to directory injection". The correction landed in Ravel
commit `b93bb927`. The two later Grove sessions updated the specs and evidence,
but not the views. A reader of the views and the spec receives two states of
the same contract.

Smallest correction: update the README paragraph, the manifest intro and
discussion, and the diagram's first step to cite the integrated commit.

### 8. Explicit cleanup has obligations but no owner (low)

Location: `docs/specs/codex-skill-provisioning.md`, "Existing installations
and active readers", paragraph 2, and `docs/adr/skill-sources-are-explicit.md`,
paragraph 4.

The spec requires cleanup to inspect each link and target, account for aliases
and recorded provenance, and preserve uncertain entries. It then says that it
introduces no automatic migration or cleanup command. One reading is a manual
procedure the user performs with documentation. The other is an owner-invoked
verb that is not automatic. The provenance the inspection relies on is the
provisioner's pointer and snapshot layout under HOME. The acceptance table has
no cleanup row.

Smallest correction: name the owner. Either state that cleanup is a documented
manual procedure, and name where it lives. Or state that an owner-invoked verb
is later work, and name its leaf.

### Codebase-design record

| Finding | Evidence | Caller obligation | Entangled concerns | Consequence |
| --- | --- | --- | --- | --- |
| 1 | Unversioned formula dependency and a 1.x test | A Homebrew user must pin or time an upgrade | Package publication and caller admission | Launches refuse in the prescribed window |
| 2 | A mandate with two endings and a spec with none | A skill-less agent must choose an ending | Delivery failure and loop control | Relaunch loop or stop, by agent choice |
| 3 | Confinement grants no reads under HOME | A standalone user must enable injection for skills | Confinement grants and delivery route | Native-only standalone has no skills |
| 4 | One plugin constant and a test against one plugin | A native user must find the real namespace | Mandate wording and corpus layout | Wrong target for every moved kind |

## Visible costs

These are trade-offs that the artifact states and accepts. They are not
defects, and the integration should leave them unless the owner decides
otherwise.

- A fresh installation needs an explicit skill setup step on either route. The
  delivery record states the cost and its reopen condition.
- The spec keeps the `codex-skill-provisioning` path and so names a retired
  mechanism. The producer chose stable citations over a current slug.
- Stale native copies and native hooks coexist with injected directories. The
  spec requires isolated native roots for proof and records no suppression.
- Mixed-major use on one machine needs HOME isolation, because Ravel keeps one
  settings document. Ravel's preparation record holds that trade-off.
- Dual-major admission costs two policy example sets and two fixture lanes. The
  spec and the evidence state both.
- The spec describes a target, labelled pending, that diverges from shipped
  22.8 behaviour until implementation. Both documents label the divergence.
- Aggregate conformance tooling, internal test wiring and delegate measurements
  stay open implementation questions. The planner must route each unclosed
  decision to a design leaf.

## What held under challenge

- No skill presence, location, catalog or kind registry enters Grove's
  contract. The delivery spec, caller spec, four records, glossary entries and
  context map say so, and Grove's launch supplies no parameter.
- The mandate obligations from the upstream integration appear consistently.
  The Grove spec, caller spec, evidence and Ravel's spec, record and glossary
  all qualify "first" and treat a name with "load" as a name.
- Old readers survive. The provisioner's owned roots are `current/plugins` and
  `snapshots` under HOME, and the spec and record forbid startup and upgrade
  from touching them.
- Original live ending commands stay usable. The caller spec, supervision
  record, legacy transition and signalling contract preserve them, and Ravel's
  `end-session` reads no settings.
- Grove's source refuses major 2 as the evidence states. The resolver strips a
  `ravel 1.` prefix and admits minor 1 and above.
- The evidence's source table matches the files read directly: the
  provisioning call site, the version predicate, the policy sample's schema 2
  and the formula's caveats.
- Directory-route confinement is Ravel 2's grant set, and Grove adds no
  bypass. The standalone spec's file-only `--runtime-read` keeps its meaning.
- The consumer map separates design validation from product acceptance and
  names the real-harness trials as separate evidence.
- The producer's historical diff and the two later Grove edits agree. The
  catalog proposal survives only as labelled historical input.

## Decisions (running log)

- The review read the artifact at Grove commit `d0e0a2ee` against the root and
  node briefs, the requirements leaf and Ravel commit `b93bb927`. It spent no
  in-session reviewer.
- The review checked source claims against the Grove and Ravel files read
  directly, not against graph coverage.
- Findings 1 to 4 require edits to the delivery spec and its acceptance matrix,
  and findings 5 to 8 touch the records and views. They earn an integration
  step. The step sits before `common-skill-loading-k9`, the first live sibling
  entry after this review, by `leaf-insert`. Its body names this review by
  handle and restates no finding.
- This session converted the task file to controlled English as a preparatory
  commit, under the technical-communication rule for existing documents.
