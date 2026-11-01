# ravel-skill-directories-k17

**Reviews:** ravel-skill-directories-k15

## Goal

Adversarially review the corrected Ravel directory-injection design before Grove
adoption review and implementation planning consume it.

## Context

Read the producer's Grove commit by its stable handle and Ravel commit
`4c16f8e09d58c8f649a375029c8e8fa2933de2b4` in
`/Users/antony/Development/AgenticOrchestration/Ravel`. Follow that repository's
instructions. Its skill/session specs, skill ADRs, glossary, design evidence and
three visual views are the artifact. The root/parent briefs and human corrections
in grove-skill-delivery-k8 are the contract. Earlier catalog proposals are
historical input, not requirements to restore.

The user supplies skills natively or through Ravel. Ravel acquires configured
local/exact pinned sources and injects directory locations. The agent finds and
loads guidance. Grove names its required skill without resolving its location.
The corrected design retains immutable source resources, direct mandatory-file
references and a directory-only recovery descriptor. Challenge that design
against the requirements rather than assuming these choices are sufficient.

## Done when

- Check that the complete artifact set has one coherent directory contract.
  Check that no skill-name resolution, metadata admission, duplicate-name
  gating or generated skill catalog reappears through another interface.
- Challenge mandatory file references, agent enumeration, explicit native/path
  requests, ambiguous matches and already injected native guidance. Identify
  any contradiction or missing observation needed to establish implicit skill
  discovery in both real harnesses.
- Challenge local capture, exact Git/offline reuse, descriptor publication,
  source provenance, resource lifetime, compaction recovery and delegated reads.
  Distinguish retained paths from actual permission and instructions from
  removal of inherited ending authority.
- Check disabled/no-source behaviour, source-specific failure remedies and
  original prompt/ending preservation. Check the source-acquisition CLI and
  policy interface for enough precision to implement without introducing a
  caller-side lookup protocol.
- Challenge the Ravel 2/schema-3 migration against the existing caller, SDK and
  worker boundaries. Preserve Grove's separate native-only 1.1 route and require
  explicit 2.x admission, policy examples and acceptance before shared activation.
- Assess whether the ADR set and public acceptance seams carry the real
  trade-offs. Distinguish document validation from unimplemented product
  behaviour. Produce findings only. Classify visible costs separately from
  contract defects.

## Notes

<!-- tc: depart synonym-rotation because Verify names a graph evidence tier -->
The producer used Verify-tier graph evidence with direct source fallback for
excluded Ravel docs. Its evidence file names source coordinates and limitations.
Refresh coverage before relying on them. The producer performed no product or
real-harness acceptance. No current creator receipt was present in the producing
session.

This is the upstream review, immediately before grove-skill-delivery-k16. For
findings worth acting on, insert integrate-review-design immediately before that
next live sibling, using this review's handle and no transcribed finding list.
Spend no in-session reviewer. Implementation remains common-skill-loading-k9's
planning responsibility after both contracts settle.

## Findings

Each finding names its location, the failure it allows, the smallest useful
correction and a severity. High means the contract is false or has no owner as
written. Medium means a reader can act on two readings, or a real failure has
no remedy. Low means a gap that the acceptance table cannot see.

No finding reached high. Visible costs follow the findings in their own
section. Findings 1, 2, 3 and 7 concern interfaces or state and close with a
codebase-design record.

### 1. The explicit-loader rule defines no test and no fallback (medium)

Location: `docs/specs/skill-discovery.md`, "Agent discovery and native
installations", paragraph 2, and "Prompt and policy interface", paragraph 2.

The spec gives one rule for a request without an explicit loading mechanism:
search the supplied directories first, then native discovery. It gives another
rule for an explicitly requested native loader or concrete path: that loader
governs the load. It never says what counts as an explicit loading mechanism.
It calls a plugin-qualified name agent-interpreted text.

Grove's current mandate
says to load the kind skill now and names the plugin-qualified spelling for
Claude Code (`crates/grove-loop/src/prompt.rs`, `LOAD_INSTRUCTION`). On Claude
Code, loading a skill is the Skill tool. An agent can read that sentence as a
name or as a request for a native mechanism.

Under the first reading the agent searches the directories. Under the second
the native loader governs, and the spec defines no fallback when it has no
match. The agent must then report missing guidance before dependent work. With
native copies absent, every Grove session then fails the root brief's
requirement that the same contract works in both real harnesses. The spec hands
this exception to callers in one sentence. Grove's evidence document says no
session-interface assumption needs to change.

Consequence: the explicit-native-loader trial has two defensible outcomes, and
the directory bootstrap trial can fail for Grove by wording alone.

Smallest correction: define an explicit loading mechanism as a concrete native
command, tool invocation or filesystem path. State that a skill name with a verb
such as "load" is a name, plugin-qualified or not. State that an explicit native
mechanism with no native match searches the directories for the same name
before the agent reports. Name the expected trial outcome for each case. The
Grove adoption review then checks the mandate wording and the evidence claim
against that rule.

### 2. The preamble asserts an order and disclaims priority in one section (medium)

Location: "Prompt and policy interface", paragraph 2, and "Directory descriptor
and loading contract", obligation 1.

Obligation 1 tells the agent to read mandatory files before optional discovery
and task actions. Paragraph 2 says a caller instruction to load its own skill
first comes after the mandatory bootstrap. The same paragraph says the
preamble claims no higher instruction priority. The root brief accepts
mandatory-first. Grove's mandate says to load its skill first, before anything
else. The agent therefore holds two "first" instructions, and the spec says the
preamble has no priority to settle them.

Consequence: an implementer writing the preamble cannot satisfy both sentences.
A trial that observes a caller-first read cannot say whether the agent erred.

Smallest correction: state that the preamble asserts exactly one priority, that
mandatory reads precede caller-block actions. Keep the disclaimer for everything
else. Name the caller obligation in the Grove handoff. A mandate that says
"first" must say "after any mandatory guidance".

### 3. Skill settings in the shared settings file defeat the documented side-by-side transition (medium)

Location: "Owner configuration", paragraph 1, and "Compatibility and
transition", paragraph 3. Source: `crates/ravel/src/settings.rs`, the
unknown-key branch.

The `skills` object lives in `HOME/.config/ravel/settings.json`. Ravel 1.1
refuses any unknown key in that file with `settings_invalid` and exit 2. The
transition section tells owners to run Ravel 2 in a separate prefix while
existing callers keep 1.1. Grove is the only known caller, and it refuses Ravel
2 until its adoption lands.

So once an owner adds `skills`, every Grove launch
under that HOME refuses before any policy runs. The spec sees this and
prescribes HOME isolation or waiting. Neither ADR records the placement
decision or a rejected alternative.

A separate owner file, such as `HOME/.config/ravel/skills.json`, is still one
owner configuration and is invisible to 1.1. The side-by-side transition then
needs no HOME isolation.

Consequence: an owner cannot enable directory injection on a machine until its
last 1.x caller migrates. The spec records that cost only as advice.

Smallest correction: move `skills` to a separate owner file that 1.x never
opens. Or record in `skill-preparation-precedes-policy` why the shared file
stays and why the owner accepts the isolation cost.

### 4. The evidence document holds a second decision register (medium)

Location: `docs/skill-discovery-design-evidence.md`, "Directory-injection
correction", the choice table, and `docs/adr/skill-preparation-precedes-policy.md`.

The table has five rows of choice, alternative and reversal evidence. Rows 1
and 4 restate the alternatives that the two ADRs already record. Rows 2 and 3
record decisions that no ADR holds. One is mandatory guidance as file
references rather than resolved names. The other is a retained directory-only
descriptor rather than inline paths. Both set public schemas, through
`mandatory` in owner settings and through `directories.json` with
`RAVEL_SKILL_DIRECTORIES`.

Both
surprise a reader who lacks the human-correction context. Both rejected a real
alternative. They pass the three-part record test. The prior review raised the
same shape, and its integration moved two rows into records. This revision
added a new table.

Consequence: the spec cites the ADRs, the ADRs omit two decisions, and the
evidence table can drift from both.

Smallest correction: add the two decisions as paragraphs to
`skill-preparation-precedes-policy`, which already records the catalog
rejection. Reduce the evidence table to pointers at the records.

### 5. Four schema and failure details lack the precision an implementer needs (low)

Location: "Directory descriptor and loading contract", "Prompt and policy
interface" and "Bounds and failures".

- The code `skill_access_unavailable` appears in the failure list and is
  defined nowhere.
- A `mandatory` entry that names an unknown source ID appears in the acceptance
  table. The text does not say whether it is a settings error with exit 2 or
  `skill_mandatory_invalid` with exit 3.
- The `sources` records in the descriptor and in the `skills` request object
  carry described contents but no field names. The `directories` and
  `mandatory` records name theirs.
- The caller block is "clearly labelled", and `skills resolve` prints the same
  preamble as a launch. The preamble therefore cannot carry the per-preparation
  nonce, and the spec gives no label that marks the block.

Smallest correction: define the code or remove it, and classify the
unknown-source case. Name the source-record fields or assign them to the SDK
declaration, and state the block label.

### 6. The compaction trial must cover loss of the instruction, not only loss of the path (low)

Location: "Directory descriptor and loading contract", obligation 4, and "Test
seams and acceptance", the "Directory recovery" row.

Obligation 4 tells the agent to preserve the descriptor path in compaction
summaries and to recover it through `RAVEL_SKILL_DIRECTORIES`. Harness
compaction rewrites the agent's context. The instruction to consult the
environment lives in the same preamble that compaction can drop. An agent
without the preamble has no standing instruction to recover anything. The
fixture row loses only the path. The real-harness list names compaction without
this distinction.

Smallest correction: add the trial case in which compaction removes the whole
preamble, and observe whether the agent rereads mandatory guidance unprompted.
Record a negative result as the acceptance gap it is. State that Ravel has no
durable instruction channel into a harness.

### 7. Byte offsets expose prompt layout into the policy interface (low)

Location: "Prompt and policy interface", the `skills` field.

`callerOffsetBytes` and `callerLengthBytes` locate the caller block inside
`prompt`. They are UTF-8 byte offsets into a JavaScript string that policies
index by UTF-16 code units. `originalPrompt` already carries the exact bytes. A
policy author who slices `prompt` with these numbers gets wrong text on any
non-ASCII prompt. The acceptance row that uses them is a Rust-side check of the
request JSON.

Smallest correction: remove the two fields from the policy-visible request, or
label them diagnostic data and state their unit.

### Codebase-design record

| Finding | Evidence | Caller obligation | Entangled concerns | Consequence |
| --- | --- | --- | --- | --- |
| 1 | Two rules and no test between them | A caller must guess which reading an agent takes | Skill naming and loader selection | Directory bootstrap fails by wording |
| 2 | One section asserts order and disclaims priority | A caller must reword its own "first" | Prompt ordering and caller intent | Preamble text cannot satisfy both |
| 3 | 1.1 refuses unknown settings keys | An owner must isolate HOME to run two majors | Skill configuration and selection bounds in one file | No side-by-side enablement |
| 7 | Byte offsets into a UTF-16 string | A policy must convert units | Prompt layout and policy data | Wrong slices on non-ASCII text |

## Visible costs

These are trade-offs that the artifact states and accepts. They are not defects,
and the integration should leave them unless the owner decides otherwise.

- Every launch recaptures each local source in full before any digest reuse, so
  launch latency grows with source size. The spec advises narrow sources.
- The agent enumerates every `SKILL.md` under the roots in each session, and
  each delegate repeats the mandatory reads. A pack may supply a hand-written
  index as a mandatory file, which keeps Ravel generic.
- Both prompt fields travel in one 1 MiB worker frame, so an enabled launch
  bounds the caller prompt below half the public limit. The spec states this.
  Raising the frame bound for protocol 2, or composing `prompt` in the worker,
  would remove it.
- Different spellings of one remote are different acquisition keys, so one pack
  can occupy the cache twice.
- An ordinary delegate keeps inherited ending authority. The supervision record
  states the residue and its reopen condition.

## What held under challenge

- No catalog reappears. A sweep for catalog vocabulary across the Ravel tree
  finds only rejection sentences. The same sweep at the previous commit finds
  the retired `entry.md` and `RAVEL_SKILL_CATALOG` contract, so the instrument
  fires.
- The descriptor, the `skills resolve` output and the `skills` request object
  carry sources, directories and mandatory files only. No field names a skill.
  Ravel checks filesystem shape and UTF-8, not metadata.
- Ordering matches the source. `choice::prepare` reads settings and resolves
  the entry, `Confined::prepare` consumes that result, and `choose_prepared`
  evaluates policy after both. The worker freezes the request before any policy
  code runs.
- Grove's resolver refuses `ravel 2.0.0` by test and admits stable 1.x from
  1.1.0, as the spec claims. The SDK declares schema 2 with an unchanged
  prompt, so the design needs schema 3 and a major release.
- Disabled behaviour has one outcome. No file, no key and empty arrays each give
  a byte-identical prompt, a null `skills` field and no cache or network work.
- Original bytes survive. The nonce-delimited block, the recorded range and the
  refusal to parse ending commands leave no reading in which caller bytes
  change.
- Offline reuse, exact-pin extraction, path-identity refusal, LFS pointer
  capture, the configurable resolution deadline and the detached-reader diagram
  carry the prior review's integrated corrections forward.
- The glossary, the spec and the permissions view separate retained paths from
  permission, and instructions from authority.
- The ADR set is two records plus one amended record, each with a trade-off and
  a reopen condition. The cited source coordinates match the files read
  directly.
- The Ravel repository root carries no `CLAUDE.md` or `AGENTS.md`. This review
  read it and changed nothing there.

## Decisions (running log)

- The review read the artifact at Ravel commit
  `4c16f8e09d58c8f649a375029c8e8fa2933de2b4` against the root and node briefs
  and the grove-skill-delivery-k8 corrections. It spent no in-session reviewer.
- The review checked source claims against the Ravel and Grove files read
  directly, not against graph coverage alone.
- Findings 1 to 4 require edits to the Ravel specification and ADR set, and
  findings 1 and 2 touch the Grove handoff. They earn an integration step. The
  step sits before `grove-skill-delivery-k16`, the first live sibling entry
  after this review, by `leaf-insert`. Its body names this review by handle and
  restates no finding.
- This session converted the task file to controlled English as a preparatory
  commit, under the technical-communication rule for existing documents.
