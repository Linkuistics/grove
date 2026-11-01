# ravel-skill-discovery-k13

**Reviews:** ravel-skill-discovery-k7

## Goal

Adversarially review Ravel's generic skill-discovery contract before Grove's
adoption design or implementation planning treats it as settled.

## Context

Read the producer's committed artifacts in
`/Users/antony/Development/AgenticOrchestration/Ravel`, following that repository's
instructions and VCS discipline. The producer task records its owning-repository
commit. The behavioural owners are `docs/specs/skill-discovery.md` and the updated
`docs/specs/session-execution.md`. The companion source/coverage evidence is
`docs/skill-discovery-design-evidence.md`, with editable views under
`docs/design/skill-discovery/`.

Producer's Ravel commit: `b68af6e492316494dfce7ec2c0ee77a1a996e60b`
(jj change `kqtnkxrunqnmuypxwtmsqnmvmvuxxukk`).

The human approved the presented direction, not a claim that the design is
implemented. Use the root and common-skill-loading briefs as requirements.
Product code remains Ravel 1.1, so distinguish intended interface changes from
current source facts.

## Done when

- Read the complete contract against accepted requirements and the real launch,
  policy-worker and confinement seams. Refresh graph coverage and source
  evidence.
- Challenge immutable capture/publication, exact Git identity/offline behaviour,
  concurrent readers, untracked delegate lifetime and manual cleanup. Look for
  failure cases that the acceptance table cannot discriminate.
- Challenge whether delegate read access and removal of enclosing ending
  authority are feasible through real harness adapters. Separate actual Ravel
  guarantees from instructed behaviour and explicitly unsupported nested
  launches.
- Challenge the no-source/bootstrap behaviour and duplicate/native handling.
  Challenge the original/prepared prompt byte contract, request/context
  compatibility and schema/version migration. Check for contradictory ownership
  across the specs.
- Produce findings only. Cut an integrate-review-design step
  immediately before the dependent Grove adoption leaf only if actionable
  findings earn it. Its charter points to this review rather than restating the
  findings.

## Notes

This is the load-bearing contract for every later external pack. The producer
used no additional in-session reviewer. The finishing producer had no
`GROVE_PROVIDER_EVIDENCE` in its environment, so it inferred no Creator line.
If the owner policy requires provider evidence, its owner-declaration remedy
applies. Do not reconstruct a receipt from model names or historical sessions.

## Findings

Each finding names its location, the failure it allows, the smallest useful
correction and a severity. High means the contract is false or has no owner as
written. Medium means a reader can act on two readings, or a real failure has
no remedy. Low means a gap that the acceptance table cannot see. Findings 1, 2,
5 and 8 concern interfaces or state and close with a codebase-design record.

### 1. Delegate ending authority is an obligation with no owner (high)

Location: `docs/specs/skill-discovery.md`, "Read access and delegation", and
`CONTEXT.md`, the Delegate term.

The spec says the delegate launcher must unset `RAVEL_EXIT_FILE` and remove
enclosing completion context. It calls this a harness-adapter obligation.
Neither Claude Code nor Codex exposes such an adapter for an in-process
delegate. A subagent's shell inherits the harness process environment,
including `RAVEL_EXIT_FILE`. Grove's `.cargo/config.toml` guard exists because
that inheritance is real. So on both real harnesses the delegate inherits the authority,
and only the entry document's instruction withholds its use.

Consequence: the brief requires the boundary without delegate ending authority,
and that requirement reads as met. It is not met. The "Delegate
boundary" acceptance case tests a fixture launcher that no production path
uses. The real-harness trial observes only a delegate's reads, so it cannot
tell an obedient delegate from one without authority.

Smallest correction: state the real guarantee. For an in-process delegate,
Ravel removes nothing and the instruction withholds use. The first-writer rule
of `end-session` bounds the damage to an early or wrong result. Record that as
an accepted residue with a reopen condition, as Ravel's ADRs do for same-user
code. Change the glossary's "must not receive" into a description of what Ravel
does. Keep the adapter text as guidance for a harness that offers one.

### 2. Catalog precedence contradicts caller intent for a named native loader (medium)

Location: "Metadata and names" and "Prompt and policy interface".

The first section makes the catalog entry authoritative for any matching name.
It tells the harness to read the catalog path instead of a name-based native
loader. The second says the preamble claims no higher priority and that
configured guidance does not authorise rewriting caller intent. Grove's current
core names the native plugin spelling and says to load it first
(`crates/grove-loop/src/prompt.rs`, `LOAD_INSTRUCTION`). A harness that reads
both holds two instructions for one skill name. The spec settles the order of a
caller's named load, not which loader wins.

Consequence: the stale-native-duplicate trial has no defined expected outcome
when the caller names the native loader. Grove's adoption design would have to
invent the rule, and the Ravel spec would then not own it.

Smallest correction: one sentence that says which binds when the caller block
names a native loader for a catalog name. Either the catalog designation binds
for every name match, or the caller's explicit mechanism wins and the
designation covers unqualified mentions only. Name the trial outcome for each.

### 3. The ADR set omits two decisions that meet the record test (medium)

Location: `docs/skill-discovery-design-evidence.md`, "Alternatives and costs",
and `docs/adr/`.

The evidence document adds no ADR because the spec owns reversible details. Two
decisions are not reversible details. Preparing the prompt before policy
reverses Ravel 1's stated no-injection promise, forces a major release with
schema 3, and rejected two alternatives. Retaining immutable snapshots with no
collector sets an on-disk cache format and a growth cost on every owner's
machine, and rejected leases.

Both are hard to reverse, surprising without context, and the result of a real
trade-off. Ravel's three existing ADRs record
decisions of the same class, each with reopen conditions. The rejected
alternatives now sit in an evidence table that nothing cites as binding.

Consequence: a later session reads `ravel-supervises-the-session`, which says
Ravel keeps no persistent record, beside a spec that adds a persistent cache.
No record reconciles them.

Smallest correction: two slug-named records with the decision, the trade-off,
the rejected alternatives and the reopen condition, moved from the evidence
table. Add one clause to `ravel-supervises-the-session` that names the content
cache as the one persistence surface. The spec then cites the records and stops
restating the alternatives.

### 4. The only known caller refuses Ravel 2 by test (medium)

Location: "Compatibility and transition".

Grove probes `ravel --version` and accepts only `ravel 1.` prefixes
(`crates/grove/src/dispatch/ravel.rs`, `check_version`). Its test refuses
`ravel 2.0.0` by name, and its remedy says to run `brew upgrade ravel`. The
spec's migration list names the SDK, declarations, policies, CLI reference and
release notes. It omits callers that pin the major version. Installing the 2.0
archive or tap formula stops every Grove driver on that machine at its next
launch, before any policy or skill matters.

Consequence: the root brief's rule that Ravel release work precedes Grove's
finish cannot hold on a shared machine without a sequencing rule. Running
sessions keep their launch's executable path and finish, but no new session
launches. The remedy text then tells the owner to upgrade, which is the action
that caused the refusal.

Smallest correction: name the caller coupling in the transition section. State
that a caller pinned to major 1 must adopt before the installed pair changes.
Or state that Ravel 2 runs beside 1.1 in an independent prefix for acceptance,
as the node brief already allows.

### 5. A fixed resolution deadline while a content check runs on every reuse (medium)

Location: "Bounds and failures" and "Exact Git acquisition and offline reuse".

Every cache hit checks manifest and content before reuse. Sources may total
512 MiB. Resolution, including transport and lock waits, has a fixed 120 second
deadline with no setting or flag. A valid but large cache on a slow disk, or a
cold fetch of a large repository, can refuse every launch with
`skill_resolution_timeout`. The only remedy is to shrink sources, which the
owner may not control for a pinned pack.

Consequence: the "Empty and bounds" case exercises limits with mutation
controls, but no case separates a slow valid cache from an unavailable one. The
failure looks like a missing revision to the owner.

Smallest correction: make the deadline an owner setting with the same shape as
`timeoutMs`, or check content fully only on publication and on explicit
request. State which, and add the slow-valid-cache case to the table.

### 6. The session specification names two public verbs and then a third (low)

Location: `docs/specs/session-execution.md`, first paragraph and "Verification
and scope".

The first paragraph says the public boundary is `start-session` and
`end-session`. The last section says Ravel 2 adds the policy-free
`skills resolve` command. The document contradicts itself about the Ravel 2
boundary. `crates/ravel/src/cli.rs` and `main.rs` match exactly two verbs today,
which holds for 1.1. Grove's caller spec also calls Ravel a two-verb CLI.

Smallest correction: say that the Ravel 2 public boundary is three verbs and
that `skills resolve` launches nothing. Leave the Grove side to its adoption
design.

### 7. Exit 3 now carries a failure that is not a policy refusal (low)

Location: "Bounds and failures", and `session-execution.md`, "Managed process
and result".

Skill-source failures use exit 3 with stage `skills`. The session spec defines
exit 3 as policy refusals, and the glossary's Refusal term covers the policy's
file, behaviour or data. A missing skill directory is owner environment, not
policy. Grove's caller spec tells its user to edit the named policy on exit 3.

Smallest correction: add stage `skills` to the exit summary of the session spec
and to the Refusal term, or give source failures their own exit. State which,
so callers do not route source failures to policy remedies.

### 8. No mechanical channel carries the catalog path to a session (low)

Location: "Catalog and loading contract", instructions 4 and 5.

The catalog path reaches the session only inside the prompt. Compaction
summaries and delegate prompts carry it by instruction, and the spec admits
that a dropped path is unrecoverable. The managed-process environment already
carries one Ravel variable. A second, holding the entry path, costs nothing
under confinement, and the policy worker never sees `RAVEL_*` names. Hooks,
in-process delegates and a re-entered session could then recover the path
mechanically. The alternatives table does not consider or reject it.

Consequence: the compaction and delegate obligations rest entirely on model
behaviour while a cheap mechanical fallback exists.

Smallest correction: add the variable to the managed-process environment and
the descriptor, or record in "Compatibility and transition" why the environment
is not used.

### 9. Three failure cases the acceptance table cannot discriminate (low)

- Path collisions on the cache filesystem. macOS APFS is case-insensitive and
  normalisation-insensitive by default. A Git source with two paths that differ
  only by case or Unicode form extracts one over the other. The manifest lists
  both, so the check should refuse, but no text names the case. Add a
  `skill_resource_invalid` reason for a path collision and a fixture with such
  a tree.
- Git LFS pointers. Acquisition runs no LFS filters and extracts object contents
  directly. A resource stored in LFS captures as a pointer file with no
  refusal. Say so, and either refuse or document the pointer.
- Lock waits with no lock. The resolution bound includes lock waits, but
  publication is lock-free by design and no lock appears elsewhere. Name the
  lock or drop the phrase.

### 10. The lifetime diagram shows an ordinary delegate reading after its parent exits (low)

Location: `docs/design/skill-discovery/lifetime.mmd`, the last two messages.

Ravel's cleanup ends the managed process group on exit, so an ordinary
delegate dies with its parent. A delegate that reads after the parent exits is
outside the group, such as a remote or detached worker. The diagram presents
that surviving reader as the normal delegate.

Smallest correction: label the surviving reader as untracked or detached, which
is the case the retention rule exists for.

### Codebase-design record

| Finding | Evidence | Caller obligation | Entangled concerns | Consequence |
| --- | --- | --- | --- | --- |
| 1 | `end-session` reads inherited `RAVEL_EXIT_FILE`, and subagents inherit the harness environment | A harness must build an adapter that nobody ships | Read grants and ending authority travel in one inherited environment | A delegate can end the session although the spec says it must not |
| 2 | Two sections state different precedence | A caller cannot know which loader its named skill reaches | Catalog designation and caller intent | Undefined trial outcome for the duplicate case |
| 5 | A check on every hit plus a fixed deadline | An owner must shrink sources it may not control | Cache integrity checking and launch latency | Valid caches refuse as timeouts |
| 8 | The path exists only in prompt text | The model must copy a path through compaction and delegation | Prompt transport and path recovery | Instructed behaviour where a channel is cheap |

## What held under challenge

- Ordering. `choice::prepare` resolves settings and the owner entry,
  `Confined::prepare` consumes that result, and `choose_prepared` evaluates
  policy afterwards. The design's insertion point sits exactly there, and the
  worker freezes the request before context loading.
- The caller-byte contract. A nonce-delimited block, recorded offset and
  length, and byte-identical prompts when discovery is off leave no reading in
  which caller bytes change.
- No-source and bootstrap behaviour. Disabled, missing and malformed cases each
  have one outcome. A missing configured source refuses with a remedy rather
  than a substitute.
- Duplicate names. Refusal is the simplest rule that keeps unqualified
  references deterministic, and the rejected alternatives are visible.
- Offline reuse. The acquisition key, the no-network cache hit and the ban on
  neighbour substitution match the accepted requirement.
- Publication. A rename of a non-empty staged tree onto an absent name is
  atomic on POSIX. It cannot replace a complete object, so readers see none or
  all.
- Read grants. The macOS profile grants `subpath` and the Linux backend
  `--ro-bind`, so a read-only directory grant for snapshot trees is a small
  extension of `FilesystemGrants`.
- Nested launches. The Seatbelt profile hides the policy directory with ENOENT,
  so a nested launch inside a confined parent refuses as the spec says.
- Local source overlap with protected paths. The existing overlap check extends
  to snapshot origins without a new mechanism.

This review refreshed coverage and source evidence. The Ravel graph project
reports generation `2026-11-01T07:59:38Z`, status ready, no parse-partial or
skipped files, and `docs/` excluded by design. Coverage for the eleven cited
source files reports no recorded issue with matching metadata, a best-effort
signal. The product diff from `2095cbd8` to `HEAD` under `crates`, `scripts`
and the Cargo manifests is empty, so the source read is the 1.1.0 source. The
producer commit is the parent of a clean Ravel working copy. Every claim above
about Ravel code cites a file read directly.

## Decisions (running log)

- The review read the artifact at the producer's commit against the root and
  node briefs. It spent no in-session reviewer, as the review family file
  requires.
- Findings 1 to 5 require edits to Ravel's specification, glossary and ADR set,
  so they earn an integration step. The step sits before
  `grove-skill-delivery-k8`, the first live sibling entry after this review, by
  `leaf-insert`. Its body names this review by handle and restates no finding.
- This session converted the task file to controlled English as a preparatory
  commit, under the technical-communication rule for existing documents.
