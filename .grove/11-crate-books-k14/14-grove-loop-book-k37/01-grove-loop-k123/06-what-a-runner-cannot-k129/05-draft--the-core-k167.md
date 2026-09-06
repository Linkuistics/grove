# the-core-k167

## Goal

Draft chapter 19 of the `grove-loop` book, *The guaranteed core* —
`crates/grove-loop/src/prompt.rs` lines 1–245, one whole root — to a valid slice
through `too-late-to-say-later`.

## Context

- **The rule: a sentence rides `${prompt}` only if its failure mode is one the
  skill cannot repair — because by the time the skill could speak, the moment has
  passed.** The three driver-authored parts and no methodology; the too-late test
  and its closure on the word *fact*: a driver fact is a launch-varying value the
  methodology cannot know at authoring time, and its static meaning stays in the
  skill.
- At 69% comment prose the charter is **do not restate**. This is one of the most
  argued-in-situ files in the crate; the page's work is the connection between
  the arguments and the measurements no comment can make about itself.
- Pinned by `the_prompt_is_three_parts_in_the_sessions_own_timeline_order`,
  `the_runtime_facts_restate_no_rule_the_skill_owns`,
  `the_signalling_contract_states_the_mechanism_and_defers_the_ending` and
  `every_kind_names_a_skill_the_plugin_ships`.
- **`prompt.rs` has one unresolved intra-doc link**, at line 28, to
  `crate::methodology`. The prose around it is true and only the link is broken;
  adjudicate on the page. `unresolved-doc-links-k151` holds the fix, deferred
  behind the book. Run `cargo doc --no-deps --document-private-items` over this
  block rather than reading the links — and check which comment marker the file
  uses before trusting a clean run, since the tool sees only `///` and `//!`.
- **`crates/grove-loop/tests/prompt.rs` fails ten or eleven times in any
  workspace copy that is not a jj repository**, which is the environmental
  baseline every mutation reading here must be diffed against; and a mutant
  naming a `prompt.rs` test must be re-run, because this file wedges under load.
- Declares glossary anchors `guaranteed-core` and `stated-vcs`; keeps
  `one-live-driver-per-working-tree` where the stated VCS reaches it, named in
  prose and never linked.
- Carries three `docs/ARCHITECTURE.md` residue markers — *the core's three
  parts*, *what the core reads*, *the stated VCS in `${prompt}`* — by subject and
  not by line number.

## Done when

- `book-check --repo . --book docs/walkthroughs/grove-loop --through
  too-late-to-say-later --check all` is valid at 9,918 resolved lines, 615
  deferred, `final=false`.
- The `the-prompt-core` ownership row reads `resolved`, the chapter-1 cast row
  owned by `too-late-to-say-later` reads `explained`; contents and navigation
  updated; `scripts/check.sh` red on `book-check` alone, and this file says so.

## Decisions (running log)
