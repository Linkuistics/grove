# cli-shape-k15

## Goal

Decide what the CLI *is*, now that there is a library for it to expose. The
deliverable is a design — a section of `ARCHITECTURE.md`, or a document beside
it, plus an ADR if the choice earns one — not code. `cli-k16` builds
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
cutting `impl` leaves has drifted into planning's job. `cli-k16`
already exists for the build; if this design turns out to need *more* than one
implementation session, decompose that leaf rather than growing this one.

**Reaching for a formalism here is optional and probably wrong.** The routing
table's own advice applies: count the states a property mentions, and check
first whether the type system already forbids what you were about to model. A
CLI's shape is mostly neither. If you do model something, the findings entry is
owed; if you do not, no entry is owed from this leaf.

## Decisions (running log)

**The deliverable is `docs/ordinal-fs-tree/CLI.md`, not a section of
`ARCHITECTURE.md`.** The architecture document is the *library's* specification
of record and its claims are the ones two models check; the CLI is a consumer of
that library and nothing in it is modelled. Folding a consumer's stream and
exit-code contract into the document that says what the models cover would make
"the models are the specification, the document the explanation" false of a third
of it. `ARCHITECTURE.md` gains a pointer and one bullet under *What this library
deliberately does not do*; `CLI.md` moves with the crate exactly as the glossary
and the models do.

**Generic-versus-concrete: the crate ships a binary on the reference domain, and
there is no generic CLI in the library.** A generic mutating CLI *is* buildable
without widening the seam — `parse` plus `view()` recovers a `Parts` from a whole
filename — but the ordinal and key in that filename are allocated by the library
and discarded, so every mutating verb would take an argument two thirds of which
is a lie, and its `--help` could carry no example because the library does not
know what a name looks like. The alternative that gives good arguments — a
command factory parameterised by a parts-parser — is a second parameterisation
point and falsifies the headline sentence of
`docs/adr/entry-name-is-the-only-seam.md`. Both are rejected, and the decisive
evidence is that the one consumer we know is coming would use neither: grove's
verbs are `leaf-add` and `leaf-retire`, named for what an operator wants, and a
factory that hands out `append` and `insert` serves consumers who want the
algebra's verbs verbatim.

**No ADR from this leaf, and the clause that fails is hard-to-reverse.** Adding a
generic `cli` module later is additive — the binary stays, no consumer breaks, no
on-disk format changes, no model claim moves. `ADR-FORMAT.md`'s test is an AND
over three clauses and this decision clears two. The rejected alternatives
therefore live in `CLI.md`, which is the durable artifact; `.grove/` is deleted
at the finish, so `records-k5`'s finding applies here too.

**Verb naming: a noun prefix appears exactly where the operator chooses a
species, and nowhere else.** `lesson-add` / `module-add` / `lesson-insert` /
`module-insert` are noun-prefixed because the parts the operator supplies are
what decide leaf-or-node; `promote`, `relabel`, `publish`, `unpublish`, `show`,
`list`, `ancestors` and `overview-chain` are bare because their target is named
by key and its species is read off the tree. That is *the species follows from
the parts* surfaced in the verb grammar, and it is why the rejected alternative —
one `insert` verb with `--as lesson|module` — is worse than it looks: it makes
the operator name a species the design says nobody names.

**`insert` takes an ordinal, not the key of the entry it displaces.** The
key-of-the-displaced-entry spelling (grove's own `leaf-insert`) is friendlier and
was rejected: it would resolve a key into an ordinal the library was going to be
handed anyway, invent an operation the library does not have, and make
`Refusal::NoOccupantAtOrdinal` — the most carefully specified refusal in the
design, carrying the level's occupied span across three distinct messages —
unreachable from the CLI. Taking the ordinal also closes the discovery loop: an
operator who guesses wrong is told the level's least and greatest occupied
ordinals by the refusal itself.

**stdout is `<key>` TAB `<path>`, one record per line; stderr is advisory.** The
key column is there because *every operation names its target by key*, so a
caller that could not recover a key from `list` would have to re-implement the
domain's grammar to drive the next verb — the one thing the library exists to
prevent. Column 1 is the target you would pass to another verb to name what the
line is about: a key, or `.` for the tree root; a distinguished child carries no
key of its own, so its line names the level whose content it is. The parsing rule
is *split on the first tab*, because a caller-supplied root may contain one.

**A mutation prints `Report::created()` when it is non-empty and
`Report::renamed()`'s destinations otherwise** — a mechanical rule, not a
per-verb judgement. Every operation here either creates something or is a pure
rename, and the siblings a shift moves are the price of the subject rather than
the subject. The full landing order, creations and renames labelled, goes to
stderr, which is where `Report::paths()`'s plan order and the highest-first shift
rule stay observable. stdout is written only after the mutation succeeded — a run
that fails is rolled back, so paths printed as it went would describe files that
are no longer there.

**No `--json` and no `--limit`, each with its excuse named.** `cli-tool-design`'s
own applicability clause supplies both: *audience* excuses the structured mode —
this is a demonstration binary whose consumers are contract tests and developers
reading the library, and the one parsing guarantee it actually needs is key
round-tripping, which the second column delivers; *shape* excuses the default
page — the result set is bounded by the tree the operator named, and a silently
truncated tree listing is precisely the failure the library's no-silent-skip rule
exists to prevent. `--under`, `--status` and `--label` narrow instead.

**A refusal reaches the operator as `Display`, verbatim, and the exit code is the
error category.** Seven codes, derived from the library's own outcome taxonomy so
each answers *what should the caller do next*: 0, 1 environment, 2 usage, 3 no
such entry, 4 refused, 5 this tree cannot be read as a syllabus, 6 failed and
rolled back (retryable), 7 failed and the rollback failed (needs a human). 6
against 7 is the single most valuable distinction the library offers and a
generic `1` would throw it away. A read that finds no entry renders
`Refusal::TargetMissing` rather than a second wording of its own —
`docs/formalism-findings.md` entry 017 found that message text is exactly where a
second author's version drifts.

**The binary's source lives outside `src/`, at `bin/syllabus.rs`.** Verified, not
assumed: a probe at `src/bin/probe_tmp.rs` calling `ordinal_fs_tree::fs::read`
failed `the_algebra_cannot_reach_the_filesystem` with
`bin/probe_tmp.rs:5`, and the same file outside `src/` passed. The alternative is
a second exemption in the guard, and every exemption is a hole; keeping the CLI
out of `src/` also says structurally what the generic-versus-concrete decision
says in prose — the CLI is not a module of this library.

**No review chain.** The bar is a load-bearing artifact others build on for
months; this is a specification for one implementation session, for a
demonstration binary, in an increment that ends with it. `cli-k16` is already
chartered to treat awkwardness as a finding about the seam, which makes the
implementing session the adversarial read this artifact actually needs.
