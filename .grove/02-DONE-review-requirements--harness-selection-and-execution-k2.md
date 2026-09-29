# harness-selection-and-execution-k2

**Reviews:** harness-selection-and-execution-k1

## Goal

Independently review the agreed first-release requirements for missing
obligations, internal consistency, falsifiable acceptance conditions and
unintended design constraints.

## Context

Read the committed artifact from `harness-selection-and-execution-k1`, including
its running decisions and verification evidence. The durable boundary is
`docs/adr/harness-selection-is-owned-by-policy.md`; the starting research is
`docs/research/grove-model-effort-routing.md`. The research's sample flags and
types are proposals, not existing APIs.

## Done when

- Findings identify material gaps or contradictions with concrete evidence and
  affected requirements, or explain why no material findings remain.
- The read covers independent use, Grove integration, static and computed
  policy, explicit configuration trust, incomplete-map refusal, simple producer
  provenance, process behavior, records and initial delivery with Grove.
- Each acceptance condition is assessable through the agreed external test
  seams. Settled human choices are distinguished from open design decisions.
- Any required integration is scheduled before `harness-selection-and-execution-k3`
  according to the review procedure. Do not create integration work when there
  are no findings to integrate.

## Notes

The human confirmed this contract and the test seams. The first increment uses
only the original creator's provider for review separation, with the rule owned
by configuration. It stops on incomplete mappings. A working local LLM selector,
its calibration pilot and extraction to a separate repository are follow-up
work. Earlier stronger identity rules in the running log are explicitly
superseded; do not restore them as requirements.

This is an inspection-only read of the committed artifact and recorded evidence.
Produce findings, not fixes, and do not rerun builds, tests, lint or formatting.

## Decisions (running log)

Material findings remain, so integration is required. It was cut as
`harness-selection-and-execution-k4` by `leaf-insert` at the design leaf, the first
live sibling after this review, so it runs before
`harness-selection-and-execution-k3`.

## Findings

Read: the k1 commit `f0f26e56` — `.grove/_BRIEF.md`, the k1 running log and
verification, `docs/adr/harness-selection-is-owned-by-policy.md` and the k1
edits to `docs/research/grove-model-effort-routing.md` — against the Grove
contracts the brief names and current source. Inspection only; the only commands
run were read-only `grove-llm resolve` / `brief-chain` and `jj` queries. Brief
line numbers refer to `.grove/_BRIEF.md` at that commit.

Severity: **high** — every written acceptance case could pass while the defect
ships; **medium** — an acceptance condition cannot be assessed as written, or
scope is ambiguous; **low** — wording or consistency.

### F1 (high) — Missing provenance has no remedy, and retained direct templates guarantee it

Affected: brief 23-27, 44-47, 87-91, 94.

Provenance exists only for sessions launched through the new executable.
Artifacts produced by a direct-harness template (kept valid by brief 94), before
adoption, or under any mixed configuration have none, so with the supplied
policy every review of them is an incomplete mapping and stops. The requirements
demand an *actionable* diagnostic but name no action short of re-producing the
artifact through the executable. Concrete case: this grove's k1 and k3 run before
the executable exists; any later review of them routed through the supplied
policy stops. The scope of "every review" (brief 23) is also unstated where review
kinds stay on direct templates, to which no provenance rule can apply.

Decision needed (human): what remedies missing provenance — (a) accept refusal
and document an adoption order (route producers before their reviewers); (b) admit
an explicit, caller-declared original-creator provider that policy sees and
records mark as *declared* rather than *observed*; (c) other. Recommend (b): the
declaration is explicit, inspectable and recorded, so the human's rejection of
*silent* permission and automatic fallback stands, and the diagnostic gains an
action. Also state that "every review" means every review selected through the
supplied policy.

### F2 (high) — The Grove seam carries no stable identity for association

Affected: brief 17-18, 52, 87-91, 92-96, 100-104; ADR 19-22.

The requirements pass Grove's *task file* (a path) and forbid the executable from
parsing Grove filenames. Leaf paths are not stable: retirement renames
`NN-<kind>--<slug>-k<key>.md` to `NN-DONE-…`, and `leaf-insert` shifts later
siblings — this session's own insertion moves k3 from `03-` to `04-`. Only the
`<slug>-k<key>` handle is permanent. A review looks up its producer's provenance
*after* the producer retired, and outcome records associate later acceptance and
repair with a run, so path-keyed association fails in the ordinary case. The
requirements neither require association to survive these renames nor say who
supplies a stable key (the Grove seam or a Grove-specific loader). The minimum
scope over which provenance must stay discoverable is also unstated: `finish-commit`
deletes `.grove/` (`docs/adr/untracked-configuration-delta.md:69`) and a
machine-local store is invisible from another checkout. The task body is no home
either: Grove's task format says a leaf carries "no record of how any past session
ran" (grove skill, `TASK-FORMAT.md`).

Recommend: add an acceptance condition that provenance and outcome association
survive retirement and reordering of the producing leaf; state that Grove supplies
its stable task identity (the handle) as caller data; state the minimum
discoverability scope (for the first release, at least the life of the grove in
which producer and review both run). Representation stays with design.

### F3 (high) — Required execution evidence conflicts with the wrapper-must-exec contract

Affected: brief 57-58, 61-65, 96-104.

Grove's contract requires a wrapper to `exec` the harness it fronts so Grove keeps
direct ownership of the foreground child (`docs/CONFIGURATION.md:460`), and the
research assumes `exec` (`docs/research/grove-model-effort-routing.md:201-202`).
After `exec` the executable is gone: it cannot observe exit status, duration,
usage, or a mid-session model change inside the harness. Yet the requirements ask
for "actual execution evidence", evaluation of latency and total usage, and a
joint choice that "stays fixed". Either the first release supervises the harness —
re-owning the terminal handover, process group and signal behaviour that
`docs/adr/the-launched-child-is-a-job.md` gives Grove, and revising the
configuration contract — or that evidence comes from elsewhere or is recorded as
unknown. Nothing says which evidence the first release must capture, by whom,
whether a writer for outcome records ships or only their format, or whether a
record that cannot be written stops the launch.

Recommend: state the minimum evidence captured at launch (selected candidate and
argv, launch success or failure, time); allow exit status, duration and usage to
be recorded unknown unless a later step supplies them; say recorded provenance is
the *launched choice*, not an observed model identity; state record-write-failure
behaviour and whether an outcome writer is in scope. Let design choose exec versus
supervision against that.

### F4 (high) — Delegation silently weakens Grove's refuse-before-mutation guarantee

Affected: brief 44-47, 52-53, 92-96; `docs/adr/complete-session-configuration.md:42`,
`docs/adr/a-kind-is-an-open-token.md:28`.

Grove refuses to write a leaf of kind K unless K resolves, asking before the tree
is mutated (`crates/grove-llm/src/cli.rs:20-23`). Once a route points at the new
executable, resolution proves only the wrapper's argv. A policy with no mapping
for K is discovered at launch — possibly hours into an unattended run — and stops
the loop on a leaf written long before. The requirements neither preserve the
authoring-time check (for example by asking the executable whether K is mapped)
nor accept its loss.

Decision needed: keep an authoring-time completeness check for statically mapped
kinds, or accept launch-time refusal as the documented cost. Recommend stating the
choice as a requirement. Computed policy cannot be checked in advance, so at
minimum the usage docs and configure-grove must say the guarantee now covers only
the wrapper.

### F5 (high) — The trust boundary is narrower than the hazard it guards

Affected: brief 31-33, 84-86; ADR 27-30.

(a) "Explicitly selected through personal configuration or a `--config`
argument" admits a cwd-relative or pattern path. A personal Grove command with a
literal `--config .policy/select.ts`, or a personal policy that loads "the
repository's policy file", executes whatever each checkout ships there — including
a repository cloned only to read. That is the hazard the untracked-delta ADR
rejected for `.grove.kdl` (`docs/adr/untracked-configuration-delta.md:26-29`,
`:92`). The acceptance case (a repository containing policy causes no execution)
passes under default configuration and never tests this.

(b) The requirement names repository *policy*. A TypeScript host commonly
discovers runtime configuration and module resolution from its working directory
(project configuration, preload hooks, environment files, `node_modules` for bare
specifiers), and the working directory is the repository. Loading trusted
*personal* policy there can execute repository-controlled code with no repository
policy selected.

Recommend: state the boundary as *no repository-controlled code or runtime
configuration is loaded unless the owner named that exact file*; decide whether
relative or pattern admission is allowed (recommend not in the first release);
extend the trust acceptance with fixtures for a relative `--config`, and for
repository-local runtime configuration and a shadowing module beside trusted
personal policy.

### F6 (medium) — "A selector cannot complete the task" is not attainable for trusted code

Affected: brief 57-58, 96-98; ADR 47-49.

The ADR says trusted TypeScript is not a sandbox. The completion channel is a file
in the workspace control directory (`crates/grove-loop/src/loop_driver.rs:44`),
reachable by any process of the same user, so trusted policy can always find and
write it. "Cannot complete" is therefore false or unfalsifiable. The assessable
property is that selection helpers are *not granted* completion authority.

Recommend: restate it that way. The observable: a fake policy that records its
environment finds no loop-control variable, while the fake harness receives the
channel.

### F7 (medium) — Explicit choice inputs have no defined source, shape or enforcer

Affected: brief 23-24, 59-60, 82-83; ADR 18-19.

The acceptance case says explicit choices are visible to policy and never silently
become another candidate, but the ADR gives "explicit choice handling" to policy.
If policy owns it, a user policy may substitute and an executable-level test has
no expected result; if the executable enforces it, it must compare result with
request. Also undefined: the shape (whole candidate, or a partial model/effort
request); and the source for Grove sessions, since Grove admits no per-invocation
flag or environment override (`docs/CONFIGURATION.md:41-43`) and rejected launch
policy in task leaves (`docs/adr/untracked-configuration-delta.md:127`). "Retries"
(brief 24) is undefined when neither Grove nor the executable retries
automatically; each relaunch is an ordinary new selection.

Recommend: define an explicit choice's shape, which callers can supply one (for
Grove, none in the first release unless the human wants a channel), and whether
the executable checks the returned candidate against it; define "retry" as any
re-invocation, or drop the word.

### F8 (medium) — The review rule's deliverable and scope are unidentified

Affected: brief 28-30, 55-56, 87-91; ADR 18-25.

"The supplied configuration chooses another provider" does not say what is
supplied: an example policy shipped with the package, the human's personal policy
applied through configure-grove, or a test fixture. Only the first two deliver the
rule; a fixture satisfies the command seam while delivering nothing. Also open: how
policy recognises a review among open kind tokens (for Grove, the `review-*`
kinds and the leaf named by `**Reviews:**`), and whether provenance-based review
selection must work for non-Grove callers in the first release. k1 removed the
research sentence letting other callers declare a review relationship (former
research lines 268-270) and no requirement replaces it.

Recommend: name the delivered artifact that carries the rule and require the
command seam to exercise that artifact; state whether non-Grove review selection
is in first-release scope.

### F9 (medium) — "Provider" and "original creator" are undefined

Affected: brief 23-27, 54-56, 87-91; the ADR throughout.

The rule's truth depends on what a provider is: model vendor, gateway (one
vendor's model through a third-party cloud), or local open-weights model. The
research makes provider an explicit per-candidate declaration that gateway changes
cannot alter (`docs/research/grove-model-effort-routing.md:267-268`), but no
requirement or glossary entry says so — and Grove rejected vendor-diversity
checks because opaque commands expose no reliable identity
(`docs/adr/complete-session-configuration.md:76-80`). The brief also drifts
between "producer identity" (54), "original creator", and the ADR's "previous
producer" (22).

Recommend: define provider as an owner-declared attribute of each candidate,
never inferred from harness or argv; use "original creator" consistently; record
both terms in the owning context's glossary.

### F10 (medium) — Pre-launch process behaviour is unspecified

Affected: brief 57-58, 96-99.

The process requirements cover only the selected harness. Unspecified: Ctrl-C, or
the driver's SIGTERM/SIGKILL escalation, while policy is still evaluating (the
executable is then Grove's foreground job); whether that launches nothing and
records a failure; whether a refusal's exit status is distinguishable from a
harness failure; whether policy may read stdin or write to the terminal before the
harness owns it; and whether selection has a latency bound — a hung computed
policy blocks the session indefinitely, and with fallback rejected a timeout can
only refuse.

Recommend: add to the Grove launch-boundary seam that an interrupt during
selection launches nothing, and that refusal exits with a documented non-zero
status.

### F11 (low) — "Bounded" and "sufficient" context have no observable

Affected: brief 18, 54, 71-74, 79-81.

Design owns the context mechanism (brief 142), but no acceptance case can fail on
"bounded", "enough" or "sufficient". The refusal list also omits a policy that
requires context the caller did not supply, and a context loader that fails —
both stop-with-diagnostic in the research (`grove-model-effort-routing.md:246-248`).

Recommend: add those two refusal cases; make "bounded" an observable the design
defines (for example, inspection reports what context was delivered and its size),
or drop the word from the requirements.

### F12 (low) — `grove run` scope is unstated

Affected: brief 52-53, 92-96.

Standalone routes launch under a mandatory filesystem sandbox: writes only beneath
the invocation directory, explicit read grants for files outside runtime
locations (`docs/CONFIGURATION.md:66-80`). The research keeps standalone templates
unchanged (`grove-model-effort-routing.md:209-212`), but the requirements do not
say whether the executable must work behind a `grove run` route. If it must,
policy and runtime reads and record writes need stated behaviour under
confinement.

Recommend: state `grove run` routes out of first-release scope, or add the
confinement acceptance.

### F13 (medium) — Delivery, documentation and independence are not assessable through the agreed seams

Affected: brief 12-14, 71-74, 105-108.

The agreed seams cover command behaviour and Grove's launch boundary. Delivery "on
its supported targets" has no check: the targets are `aarch64-apple-darwin` and two
Linux targets cross-built against a glibc 2.17 floor
(`scripts/release-common.sh:21-25`, `scripts/release-build.sh:24-25`), and release
verification installs and runs on the release host only (`docs/RELEASING.md`
§4). Nor does anything say whether the installed command must run TypeScript
policy with no separately installed runtime — a human installation choice, not
only a design one. Independence ("a caller without Grove") and the extraction
boundary have no falsifiable observable.

Recommend: run the command seam with no Grove binaries or Grove configuration
present; state whether a separately installed TypeScript runtime is an acceptable
prerequisite; name a per-target delivery observable; say documentation acceptance
is by review.

### F14 (low) — The research record contradicts the k1 notes

`docs/research/grove-model-effort-routing.md:4-5` says personal configuration has
not been changed; the k1 notes (k1 task file lines 29-30) say the preceding
session applied personal model/effort defaults. One is stale, and design will
cite the research. Recommend correcting the dated statement.

## Settled choices and open design decisions

Settled by the human; integration must not reopen them: a separate package in this
repository, shipped with Grove, extractable later; an explicit open kind token, no
Grove filename parsing and no Grove dependency for ordinary use; the unchanged
prompt, with the task file optional; static mapping plus computed TypeScript
policy choosing harness, model and effort jointly; review separation by *provider
only* against the original creator, owned by configuration, with no
model-inequality check and no multi-author exclusion set (earlier stronger rules
superseded); missing provenance stops; personal policy by default, repository
TypeScript only by explicit selection; no automatic fallback; first-release scope
excluding a local selector, pilot, calibration and extraction; the two agreed test
seams.

Delegated to design (k3) by brief 139-145 and the running log: name, language,
TypeScript runtime and hosting, package boundary and delivery mechanics; request
and selection protocol; configuration precedence between personal policy and
`--config`; the context mechanism; provenance representation and record format.
The design must also say which execution counts as the original creator when a
producing leaf was launched more than once — under computed policy a relaunch may
pick another provider. The human delegated that representation, so it is a design
obligation, not a requirement change.

Findings F1, F4, F5(a) and the runtime-prerequisite part of F13 need a human
choice; the rest are clarifications the integration can make against settled
intent.

## Observation outside the artifact

k1's `bash scripts/check.sh` run ended with SIGTERM (exit 143) during book checks,
cause not established. It does not affect the requirements, but the finish
sequence in `CLAUDE.md` depends on that script's exit status for the integrated
result. No action for the integration leaf.
