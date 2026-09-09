# pass-series-discipline-k221

## Goal

Ship the pass-series construct into the installed methodology, so a session can
actually run one. `docs/specs/pass-series.md` is the design and
`docs/adr/iteration-is-a-node-of-nodes.md` the recorded decision; **neither is
read by a session**, because a session reads `plugins/grove/skills/`.

## Context

Cut by `loop-construct-k7`, whose deliverable was the design and not its shipped
form. The precedent is exact and one node over: `pipeline-kinds-k27` settled
which editorial kinds exist and `pipeline-skills-k28` shipped them as skills over
a new family reference file.

**Wait for the review chain.** `loop-construct-k220` reviews the design and may
cut an integration after it; both sit ahead of this leaf in the walk by
construction. Do not ship a design a reviewer is still contesting — if this leaf
is picked while either is live, something has gone wrong with the ordering and
that is worth saying rather than working around.

**The hard question this leaf owns, which the design deliberately left open: does
this need a new kind, a new reference file, or neither?** Three shapes, and the
evidence points at the third:

- **A new session kind.** `docs/adr/a-kind-is-an-open-token.md` makes one cheap
  to author — but a kind is a *discipline a session runs under*, and a pass
  series has no session of its own. Its passes run `impl`, `draft`, whatever the
  series declares. Adding a kind here would be a kind nothing launches.
- **A family reference file**, as `references/editorial.md` is for the four
  editorial kinds. A family file is loaded because *a member's skill directed you
  here by name* (`SKILL.md`, *The family files*). A pass series has no member
  kinds, so no skill would name it, so nothing would load it.
- **A condition in the spine's register, plus the format files it points at.**
  `SKILL.md`'s loop section is a register of conditions, each naming the file
  whose procedure answers it — and the two conditions here are already
  decomposition-shaped: *when this work is to be done again, on a declared
  sequence with a declared bound* and *when a repetition was not declared in
  advance*. `references/decompose.md` already owns *choosing a composition shape*
  and holds the chain and the pair; a series is the third shape.

Whichever shape wins, four things must reach a session, and they are the four
the series brief declares: the step sequence, the exit condition, the cap, and
that the cap escalates. `BRIEF-FORMAT.md` is where a brief's shape is stated and
is a candidate host for them.

**Two rules must not be restated, only cited** — the ADR set and the spec hold
them, and `docs/adr/restatement-declares-its-class.md` governs how a skill may
repeat a rule at all: the marker-token rejection, and the reason the exit
condition may not be a yield curve.

## Done when

- A session that needs to run work again can find out how, by reading the skill
  files alone — with `docs/specs/` and `docs/adr/` unopened.
- The chosen shape is justified against the three above, in the commit or in an
  amendment to the ADR, and the two rejected ones are recorded as rejected.
- `CONTEXT.md`'s **Pass series / pass** entry and the shipped text agree; where
  the shipped text is the one a session reads, the glossary points at it rather
  than duplicating it.
- `scripts/check.sh` is green.

## Notes

**This is a methodology change, not a code change.** Nothing in `grove-llm`,
`grove-loop` or `ordinal-fs-tree` moves — that is the design's own claim and the
strongest evidence for it. A leaf that finds itself editing Rust to ship this has
found a defect in the design and should say so rather than proceed.

The installed skills reach a session as soon as the install route resolves to
this checkout, while the binaries do not until rebuilt and installed
(`references/grove.md`, *When the grove's subject is grove itself*). So verify by
reading the files, never by expecting the next session in this loop to behave
differently.
