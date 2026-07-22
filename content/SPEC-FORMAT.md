<!-- grove reference file — the spec shape -->

# SPEC-FORMAT — the spec

A **spec** is the human-facing, team-shareable design of an area of the system,
written by a planning task *when the increment is a genuine agreement point*. The
flow there is: grill → spec (review & agree) → decompose → execute. Specs live in
`docs/specs/`, one file per area, **slug-named**: `docs/specs/<slug>.md`. The slug
is the spec's identity; create the directory lazily, only when the first spec is
needed.

Most planning increments write no spec at all (constraint 4). The one that does
is the one where the human signs off on the design's shape before decomposition
turns it into work tasks.

## The set is current-state

`docs/specs/` is a **minimum coherent set describing the design's current
state** — the same rule as `docs/adr/`, one grain coarser. Edit a spec in place
as understanding shifts; merge two whose designs converged; split one that turned
out to cover two areas; delete one that no longer describes anything. Never date
or number the filename, and never append a superseding spec: the artifacts hold
the present, git holds the past (constraint 1).

**The membership test.** *Would a session on an unrelated future grove need to
read this?*

- **Yes** → it is a spec. It outlives the grove that wrote it.
- **No** → it is a `BRIEF.md`. Work-orders, keep/delete tables, and "the input
  for the next three leaves" are node briefs, and they die with `.grove/`.

**The grain rule.** An ADR records *one decision and its trade-off*; a spec
describes *how an area works*. A spec **cites** the ADRs in its area and never
restates them — restate one and the two sets will disagree, after which neither
binds.

## Suggested shape

A guide, not a schema (constraint 3). Include a section only when it earns its
place.

```markdown
# <slug>

## Problem
What is wrong, and why now — in the reader's terms.

## Solution
The shape of the design — in the reader's terms.

## Decisions
The settled calls: the modules built or modified, their interfaces, technical
clarifications, architectural decisions, schema changes, API contracts.

## Test seams
The seams the work will be tested through, agreed with the human.

## Out of scope
The non-goals, and the deliberate non-actions with the reason each was rejected.
```

Optional: user stories (`As an <actor>, I want <feature>, so that <benefit>`)
where the area has distinct actors whose needs would otherwise go unstated.

<!-- The requirements language below is adapted in grove from OpenSpec
     (Fission-AI/OpenSpec, <https://openspec.dev/>) — the requirement/scenario
     spec language only, none of its delta headers, validation, or
     change-folder machinery — MIT licensed; see LICENSES/openspec.LICENSE. -->

Optional: a `## Requirements` section, where the area has a behavioural
surface whose acceptance criteria would otherwise stay vague (like every
section: include it when it earns its place, not because the shape names it):

```markdown
## Requirements
### Requirement: <title>
The <system> SHALL <specific behavior>.

#### Scenario: <short description>
- **WHEN** <action or condition>
- **THEN** <expected outcome>
```

One `### Requirement:` per behaviour, its SHALL statement specific enough to
test; each `#### Scenario:` is one acceptance case. The pairing rule: scenarios
say *what must pass*; `## Test seams` says *where it is tested*.

## Three rules

**Synthesise; never re-interview.** The grilling *is* the interview and it has
already happened. A spec synthesises the planning task's running decision log and
the codebase understanding it built. A session that writes a spec by re-asking
the questions is running grilling twice.

**Behavioural, not procedural.** Describe interfaces, types, and behavioural
contracts. No file paths, no line numbers, no code — they go stale faster than
the decisions they illustrate. *One exception:* a prototype produced a snippet
that encodes a decision more precisely than prose can — a state machine, a
reducer, a schema, a type shape. Inline it in the relevant decision, trimmed to
the decision-rich part, and note that it came from a prototype. Not a working
demo.

**Speak the project's language.** Use `CONTEXT.md`'s vocabulary throughout, and
respect the ADRs in the area you are touching.

## Test seams

Sketch the seams the feature will be tested through, and **check with the human
that they match expectations** — that check is a grilling move, made before the
spec is written, not after. Prefer existing seams to new ones; propose any new
seam at the highest point you can; the fewer seams across the codebase the
better, and the ideal number is one. For what a seam *is* and how to judge one,
use the `linkuistics:codebase-design` skill — this note only says where the
agreement gets recorded.

When the increment covers code that will be tested but writes **no** spec — the
common case — record the agreed seams in the node's `BRIEF.md` instead. The brief
chain is how a node's settled design reaches its child work leaves, and it binds
them without a new artifact. That the brief dies with `.grove/` is correct: once
the tests exist at the seam, the tests are the record.
