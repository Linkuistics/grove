# grove-communication-reword-k15

## Goal

Reword grove's "Human-facing communication" section so that it sits under the
skill's precedence order and does not pull against it. The section asks for a
guided, conversational, explanatory style; the skill rules govern the wording
inside that style.

## Context

- The section is in `plugins/grove/skills/grove/SKILL.md`. The spec
  `docs/specs/technical-communication.md`, section *Precedence in grove's own
  text*, states what it must say, and two smaller edits: the entry for
  `linkuistics:technical-communication` in `references/grove.md`, and one
  sentence in `references/commit.md` that allows a conversion commit before the
  task's commit.
- Keep what house style owns: what to explain, leading with the outcome,
  explaining before asking for a decision, and where evidence goes. Change only
  what asks for wording that the skill's rules restrict, such as long or
  idiomatic sentences.
- The skill text reaches users through the Claude marketplace, Grove's embedded
  snapshot for Codex and `plugins/install.sh`. Find any other copy of the
  section, such as an embedded or generated one, and update it.
- `plugins/grove/conformance.sh` checks the grove skills.

## Done when

- The section, the hub entry and the commit sentence meet the spec's
  requirements, and every copy of each is updated.
- The edited text passes the document checker, apart from justified departures.
- `bash scripts/check.sh` is green, including conformance.

## Decisions (running log)

- **Scope of the conversion.** The spec's lazy-conversion rule would convert
  `SKILL.md`, `references/grove.md` and `references/commit.md` whole before
  any edit. This leaf converts only the text it edits: the section, the hub
  entry and the commit sentence. The three files hold much older prose, and a
  whole-document conversion is a separate piece of work for the human to ask for.
- **Section reworded** to the spec: "conversational" and "rather than terseness"
  are gone, the length sentence says what sets the length, and one closing
  paragraph names the order. It restates no rule of the skill. The checker
  reports 0 hard findings on it; its first run found a 7-sentence paragraph,
  which proved the instrument, and the paragraph was split.
- **Hub entry** added to `references/grove.md` with the other plugin citations.
  The opening sentence no longer counts the bodies of guidance, so the next
  citation does not stale it.
- **Commit sentence** added to the first paragraph of `references/commit.md`'s
  "What one commit contains" section, 21 words, so the section's bounds
  (`commit_guidance.rs`) are unchanged.
- **No other copy exists.** Source search for the section's wording found only
  these three files. The Rust tests include them by path, and no generated or
  embedded duplicate exists in the tree.
- **The new citation needed audit rows.** `crates/grove/tests/plugin_fallback.rs`
  sweeps every `linkuistics:` citation. It failed until `CITATIONS` gained one
  row for the section and one for the hub entry, and its count control moved
  from eight citations in six files to ten in seven.
- **`scripts/check.sh` is green except `Ravel caller assets and provenance`.**
  That check stops at `scripts/ravel-policy-typecheck.sh`, which pins Bun 1.4.2.
  The machine has Bun 1.4.3. Nothing in this leaf touches it. The finish cycle
  will meet it again.
