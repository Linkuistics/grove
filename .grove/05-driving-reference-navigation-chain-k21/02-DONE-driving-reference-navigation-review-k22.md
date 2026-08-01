# driving-reference-navigation-review-k22

**Kind:** review-impl
**Reviews:** driving-reference-navigation-k19

## Goal

Adversarially review `driving-reference-navigation-k19` and record concrete findings for its integration step.

## Context

Review the artifact produced by `driving-reference-navigation-k19` — the
`## In this guide` block in `content/driving.md` and the navigation lint added to
`tests/composition_guidance.rs` — against that leaf's two `Done when` clauses and
against `content/SKILL.md`'s own description of what `driving.md` contains.

A lint is part of the deliverable, so it is reviewed as a contract, not as test
scaffolding: what end states does it now forbid, and what drift does it still let
through.

## Done when

- Both `Done when` clauses of `driving-reference-navigation-k19` are traced to a
  falsifiable behavior or recorded as holding.
- The lint is probed in both directions — false positives (valid navigation it
  rejects) and false negatives (broken navigation it accepts).
- Findings are severity-ranked and recorded in this leaf for
  `driving-reference-navigation-integrate-k23`; do not fix the artifact.

## Notes

Assume the author validated the twelve links they wrote and did not ask what the
lint does to the thirteenth.

## Findings

Artifacts reviewed: `content/driving.md:17-32` (the navigation block) and
`tests/composition_guidance.rs:32-125` (the anchor model, the unit test, and the
navigation lint), against `driving-reference-navigation-k19`'s `Done when` and
`content/SKILL.md:488`. Every claim marked **[measured]** was produced by editing
`content/driving.md`, running `cargo test --test composition_guidance
driving_reference_navigation`, and restoring the file; the working copy is
unchanged.

### Done-when trace

| Clause (from `…-navigation-k19`) | Verdict | Where |
|---|---|---|
| compact ToC reflecting the useful conceptual sections | **partially met — one section `SKILL.md` advertises by name is unreachable** | R2 |
| without becoming a second outline of the methodology | **met, then over-enforced into a ban on ever completing it** | R1 |
| anchor links render correctly | **holds — verified against an independent implementation of GitHub's rule** | counterexamples |
| a focused test or lint protects them from obvious drift | **holds for renames, fails open for cross-level anchor collisions** | R3 |

### R1 — the lint hard-fails a *complete* table of contents (severity: high)

`tests/composition_guidance.rs:108-111` asserts:

```rust
!navigation_anchors.is_empty() && navigation_anchors.len() < section_anchors.len()
```

`section_anchors` excludes only `In this guide` (`:60`), so `content/driving.md`
has 17 countable sections and the navigation names 12. The intent — "navigation
must be selective rather than repeat the section outline" — is a statement about
*shape*. What is checked is a strict inequality on *counts*, and the two are not
the same property: listing 16 of 17 passes, listing 17 fails.

**[measured]** Adding the five omitted sections to the navigation block — the
natural fix for R2, and a change that leaves the block one level deep, in
document order, with every anchor resolving — fails:

```text
panicked at tests/composition_guidance.rs:108:5:
content/driving.md navigation must be selective rather than repeat the section outline
```

So the lint does not merely permit R2's incompleteness, it *requires* it, and a
future session that tries to close the gap will read a failure message asserting
that closing it is wrong. This is the finding that makes the others expensive:
R2 and R4 both have obvious fixes that this assertion blocks.

*What would satisfy this:* drop the upper bound and keep `!is_empty()`, or
replace the count comparison with the property actually wanted — that the
navigation groups sections rather than restating each heading verbatim, which if
it cannot be expressed as an assertion should be prose in the file, not a test.

### R2 — five sections, ~28% of the guide, are unreachable from the navigation (severity: high)

**[measured]** Resolving all headings under an independent implementation of
GitHub's anchor rule and subtracting the 12 linked anchors leaves:

| Line | Section | Lines |
|---|---|---|
| 75 | How to write a research leaf brief | 47 |
| 276 | Reworking ADRs and briefs as understanding shifts | 32 |
| 340 | Verifying a claim about the repo itself | 66 |
| 561 | What a good child leaf looks like | 26 |
| 587 | Recording fog without pre-slicing it | 15 |

186 of 664 lines. Selectivity was requested by the leaf, so a shorter list is not
by itself a defect — but two of these omissions are not judgement calls:

- **`content/SKILL.md:488` advertises the omitted section by name.** The
  reference-file entry says `driving.md` covers "when to commission prior-art
  research, **how to write a research-leaf brief**, grilling moves…". A session
  that follows that pointer, opens `driving.md`, and reads the navigation block
  finds `commissioning prior art` and no research-leaf brief entry. The two
  canonical surfaces now describe different files.
- **"Verifying a claim about the repo itself" is the single longest section in
  the guide** (66 lines) and is one of the `impl`-session habits the file's own
  intro (`content/driving.md:8-11`) says the later sections exist to teach.

The remaining three are defensible omissions.

*What would satisfy this:* add entries for `#how-to-write-a-research-leaf-brief`
and `#verifying-a-claim-about-the-repo-itself` — which requires R1 fixed first,
since 14 of 17 still passes but the fix and R1's bound now sit two entries
apart — or, if the list is to stay at 12, correct `content/SKILL.md:488` so the
two surfaces agree on what the guide is indexed for.

### R3 — the uniqueness check is scoped to `##`, but GitHub's anchor namespace is document-wide (severity: medium)

`tests/composition_guidance.rs:101-107` guards against ambiguous anchors by
asserting the `## ` anchors are unique. GitHub deduplicates across **every**
heading level in the document, appending `-1`, `-2` in document order — so a
`###` heading that collides with a later `##` heading silently steals the plain
anchor and pushes the section the navigation names to `-1`.

**[measured]** Inserting `### Anti-patterns` at `content/driving.md:520`:

```text
line 520  ### 'Anti-patterns' -> #anti-patterns
line 635  ##  'Anti-patterns' -> #anti-patterns-1
test result: ok. 1 passed; 0 failed
```

The lint passes while `[anti-patterns](#anti-patterns)` now lands 115 lines above
the section it names. This is exactly the "obvious drift" the clause commissioned
the lint to catch, and it is the failure mode a reader cannot see from the
navigation block — a broken link 404s visibly, a *redirected* one does not.

Adding subsections is ordinary editing: the guide already carries four `###`
headings (`:185-236`), and their titles are short imperative phrases of the same
shape as the `##` titles.

*What would satisfy this:* compute the anchor namespace over all heading levels
with GitHub's `-N` deduplication, and assert that each navigation anchor resolves
to a `## ` heading — which subsumes the current uniqueness assertion.

### R4 — the lint rejects valid links to `###` subsections (severity: medium)

`tests/composition_guidance.rs:113-124` resolves each navigation anchor against
`##` headings only. A link to any `###` subsection is a valid GitHub anchor and a
lint failure.

**[measured]** Adding `[asking WDYT](#ask-the-llm-wdyt-before-committing)` — a
correct link to `content/driving.md:185`:

```text
panicked at tests/composition_guidance.rs:119:17:
content/driving.md navigation target "ask-the-llm-wdyt-before-committing"
must name a later top-level section
```

The cost is concrete rather than theoretical: `content/SKILL.md:488` advertises
`driving.md` for "grilling moves (**WDYT, pushback**, running log, citation
discipline)", and all four are `###` subsections under
`#when-to-invoke-a-design-discussion-grilling`. The named moves a reader is sent
here for can never be linked, only reached by opening grilling and scrolling.

This overlaps R3: both come from the lint modelling the anchor namespace as
"top-level sections" when GitHub's is "all headings". One fix closes both.

*What would satisfy this:* resolve against all heading levels (R3's fix) and, if
one-level-deep navigation is still wanted, enforce that on the **list structure**
where it belongs rather than on the link targets.

### R5 — the structure assertion is a line-prefix whitelist with a message that misdescribes it (severity: low)

`tests/composition_guidance.rs:88-91` accepts a line only if it starts with
`- `, `  [`, or `  and [`, and reports a failure as "navigation must stay one
level deep". The whitelist does prevent a nested `  - [` sub-list, but it is
carrying that meaning by accident — what it actually pins is where the author
happened to break lines.

**[measured]** Changing the final entry's continuation from `  and [the shortest
version]…` to `  or [the shortest version]…`, which is one level deep, fails at
`:88` with the one-level-deep message.

The result is a lint that appears to guard a documented convention and in
practice objects to rewrapping a sentence, with a diagnostic that sends the next
author looking for nesting that is not there.

*What would satisfy this:* reject only a leading `-`/`*`/digit after
indentation — the actual nesting signal — and word the message for what is
checked.

### R6 — the lint is filed in a test module about a different subject (severity: low)

`tests/composition_guidance.rs` holds five tests about the doubt/Grove
composition contract — the subject of this grove — plus two constants
(`DRIVING`, and now `HashSet`) pulled in for a markdown navigation lint that
shares nothing with them. The repository otherwise runs one concern per test
file across 24 files (`tests/pick.rs`, `tests/resolve.rs`, `tests/brief_chain.rs`,
…). `driving-reference-navigation-k19`'s own `Context` states the work "is
unrelated to [`composition-guidance-k17`'s] review contract" — and then lands in
its test file.

`.grove/` dies at the finish cycle, so the misfiling outlives the only context
that explains it.

*What would satisfy this:* move `github_heading_anchor`,
`top_level_heading_anchors`, and both tests to `tests/reference_navigation.rs`.

### Counterexamples attempted and not found

Recorded so `driving-reference-navigation-integrate-k23` does not re-derive them:

- **The twelve anchors being wrong.** **[measured]** Resolving every heading in
  `content/driving.md` under an independent implementation of GitHub's rule
  (lowercase, strip non-`[\w\- ]`, spaces to hyphens, `-N` dedup) matches all 12
  navigation targets, including the three non-obvious ones —
  `#when-to-retire-research-into-adrs-versus-leave-it-in-docsresearch` (backticks
  and slashes dropped from `` `docs/research/` ``),
  `#the-review-chain--when-doubt-earns-its-own-leaves`, and
  `#prune-reorder-or-file-an-issue--the-triage-a-status-word-would-hide` (em dash
  dropped, its two flanking spaces surviving as `--`). `Done when` clause 1 holds.
- **The lint failing to catch a renamed section.** **[measured]** Renaming
  `## Externalizing surfaced work` panics at `:119` naming the now-dangling
  anchor. The core drift protection works; R3 is a gap beside it, not instead of it.
- **The attribution comment being orphaned.** Inserting the navigation block
  above the `<!-- adapted … -->` comment (`content/driving.md:34-37`) left it
  still immediately above `## When not to start a grove`, the section whose
  no-fog early exit it credits. Placement is correct.
- **The fence heuristic mis-parsing this file.** `line.starts_with("```")`
  (`:51`) toggles on any fence, so an indented or `~~~` fence would be missed —
  but `content/driving.md` has exactly two fenced blocks (`:131-133`, `:456-458`),
  neither indented and neither containing a `## ` line. Sound for this file today;
  the `driving_navigation_lint_models_github_anchor_edges` unit test (`:66-74`)
  pins the case that matters.
- **Thematic grouping conflicting with the document-order assertion.** The
  navigation's four bold group labels are thematic, and `:113-124` requires
  document order — but the 12 targets are strictly increasing today
  (39, 50, 122, 173, 255, 308, 406, 445, 520, 602, 631, 652), so the two do not
  currently collide. Worth knowing before any section is moved.
- **The navigation duplicating the intro.** `content/driving.md:3-15` sketches
  the same arc in prose. The overlap is real but the two serve different
  needs — one orients, one jumps — and the guide is long enough to carry both.
