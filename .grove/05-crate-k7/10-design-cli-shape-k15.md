# cli-shape-k15

## Goal

Decide what the CLI *is*, now that there is a library for it to expose. The
deliverable is a design — a section of `ARCHITECTURE.md`, or a document beside
it, plus an ADR if the choice earns one — not code. `11-impl-cli-k16.md` builds
whatever this leaf settles.

## Context

Beyond the brief chain:

- The root brief: *a CLI in `grove-llm`'s shape* that *drives any conforming
  tree*. Those two phrases pull in different directions and reconciling them is
  the first question below.
- `linkuistics:cli-tool-design` — the skill covering LLM-facing CLI design:
  structured output, help text with examples, actionable errors, exit codes,
  consistent flags. Read it before deciding anything about output.
- `src/llm_cli.rs`, `src/cli.rs` and `src/tree_read.rs` for what `grove-llm`'s
  shape actually is, including its audience split — `grove --help` shows a human
  none of the LLM verbs. Prior art, and the closest thing to a requirement the
  root brief gives.
- `docs/adr/entry-name-is-the-only-seam.md`. Whatever the CLI turns out to be,
  it must not become a second seam.

## The questions

- **What is generic and what is concrete?** A CLI that drives *any* conforming
  tree needs the domain at compile time, because parsing is the domain's. So the
  candidates are roughly: a command factory and dispatcher generic over
  `N: EntryName` that any consumer instantiates to get a CLI for free, with the
  crate shipping a thin binary on the reference domain; or a binary on the
  reference domain alone, which demonstrates the library and drives nothing
  else. Name the rejected one and why.
- **Which verbs?** The library's operations are not automatically the CLI's. A
  verb per operation is one answer; another is the shape `grove-llm` actually
  has, where verbs are named for what an operator wants rather than for what the
  algebra offers.
- **What does a verb print, and to which stream?** `grove-llm` prints the new
  path on stdout and its renumber summary on stderr, which is a decision worth
  either inheriting deliberately or departing from deliberately.
- **How does a refusal reach the operator?** Every refusal is a stated outcome
  carrying the domain's own error, and reserved-name refusal carries recovery
  advice, not just detection. The CLI is where that advice either reaches a human
  or is thrown away.
- **What is out.** Removal does not exist and migration is out of the increment.
  Say so where an operator would otherwise go looking.

## Done when

- Each question above has an answer, with the rejected alternative named.
- The answers are written where the implementing leaf will find them, and the
  glossary gains any term the CLI introduces (`CONTEXT-FORMAT.md`).
- An ADR exists **only** if a decision passes all three clauses of
  `ADR-FORMAT.md`'s test. The generic-versus-concrete choice is the only
  plausible candidate; being hard to reverse is the clause to check honestly.

## Notes

**Do not cut implementation leaves.** A `design` session that finds itself
cutting `impl` leaves has drifted into planning's job. `11-impl-cli-k16.md`
already exists for the build; if this design turns out to need *more* than one
implementation session, decompose that leaf rather than growing this one.

**Reaching for a formalism here is optional and probably wrong.** The routing
table's own advice applies: count the states a property mentions, and check
first whether the type system already forbids what you were about to model. A
CLI's shape is mostly neither. If you do model something, the findings entry is
owed; if you do not, no entry is owed from this leaf.
