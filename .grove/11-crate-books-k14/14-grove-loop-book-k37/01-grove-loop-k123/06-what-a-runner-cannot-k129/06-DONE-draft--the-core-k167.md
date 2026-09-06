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

1. **The slice is valid at the stated figures.** `book-check --repo . --book
   docs/walkthroughs/grove-loop --through too-late-to-say-later --check all`
   reports *valid: 13 files, 9918 resolved lines, 615 deferred lines,
   final=false*. The `the-prompt-core` ownership row reads `resolved` and the
   chapter-1 cast row for `compose`, `Mandate` reads `explained`. `README.md`'s
   placeholder line was consumed rather than added beside, chapter 18 gained its
   `Next` navigation, and thirty-six concept entries were appended after chapter
   18's run.

2. **`scripts/check.sh` is red on `book-check` alone — 1 of 8**, and the four
   `book-check` diagnostics are all the unwritten chapters 20 and 21: two `M101`
   missing pages, the `M103` contents and navigation that name them, and the
   `F003`/`F009` pair over `loop-driver`'s still-deferred block. Every other book
   is green and `final=true`.

3. **The block owes no early-use row, established by enumeration rather than by
   reading the manifest.** All thirty-six distinct backticked tokens in lines
   1–245 were classified. None names or exercises a symbol chapter 20 owns:
   `GROVE_SIGNAL_FILE` is reached by chapters 15 and 16, `grove-llm complete` is
   chapter 15's, `Handle` and `Kind` are chapter 3's, and `Workspace` is another
   book's. Two tokens resolve to nothing and both are adjudicated on the page.

4. **The chapter takes *do not restate* and the *supply the claim* obligation is
   vacant here, stated rather than left silent.** The block has no
   `#[cfg(test)]` module and no `#[test]` function, so there is no reproduced
   test for that obligation to attach to; chapter 19 is not on the structure
   brief's list for it. There are also no refusal arms and no failure path at all
   — no `Result`, `?`, `unwrap`, `expect`, `panic!` or `bail!` in 245 lines — so
   no mutation was run, and the page says why rather than leaving a coverage
   section a reader would read as omitted.

5. **Two defects found, both address-only, both adjudicated on the page and both
   cut as leaves.** `prompt.rs:234` cites the methodology rule as
   `skill-stated-vcs-is-definitive`; the inventory
   `plugins/grove/conformance/rules.tsv` carries it as `stated-vcs-is-definitive`
   and no id there begins with `skill-` — `prompt-rule-id-prefix-k174`, inserted
   ahead of `architecture-residue-k75`. And `tests/prompt.rs:289` says `--done`
   would reach *the eighteen kinds it is not an ending for* where the shipped set
   is twenty-three, so twenty-two — `stop-flag-kind-count-k173`, which also
   carries `crates/grove-llm/tests/kind.rs:2`'s *closed nineteen kinds*. Line
   28's unresolved intra-doc link was already held by
   `unresolved-doc-links-k151` and is adjudicated beside them.

6. **The book-wide mutation control was one too high, and both briefs are
   corrected.** Ten of `tests/prompt.rs`'s sixteen tests reach `compose` and die
   in the `workspace()` fixture in a non-jj copy; the eleventh failure every
   earlier child recorded was
   `the_namespace_is_the_shipped_plugin_entrys_declared_name` failing on
   `.claude-plugin/marketplace.json`, which the promoted copy recipe omits. Added
   the directory to the recipe in `grove-loop-k123`'s brief and restated the
   baseline as ten there and in this node's brief, because a control one too high
   hides an observer of `PLUGIN`.

7. **The instrument forecast was taken as a prediction and re-measured.**
   `which-files-k166` recorded that chapters 19 and 20 were not the exception
   `session_config.rs` was. `prompt.rs` has zero plain `//` and no `#[cfg(test)]`
   module, so `cargo doc` reaches all 171 of its comment lines — confirmed by a
   control planting a broken link in a `///` docblock, taking the crate from
   thirty warnings to thirty-one. The forecast was argued about `loop_driver.rs`
   only and still stands for chapter 20.

8. **Cut nothing as a last act.** This is child 6 of 7 under
   `what-a-runner-cannot-k129`; `the-loop-k168` already stands as a live later
   sibling, and this node's brief assigns the `copy-edit` leaf to
   `what-could-not-move-k130`. Under `references/editorial.md` a stage that finds
   its successor already standing cuts nothing.

9. **A third defect, on a finished page rather than in the source.**
   `18-which-files.md` calls `session_config.rs` *the smallest root in Part V* at
   two sites; Part V's roots are 1,383 / 615 / 358 / **245**, so `prompt.rs` is.
   This is `lease-size-ranking-k171`'s class recurring, so
   `chapter-eighteen-size-claim-k175` was inserted **ahead of `the-loop-k168`**,
   matching k171's placement, and the node brief's decomposition table now
   carries three correction leaves rather than two. `smallest` was enumerated
   across the book — thirteen hits, ten unrelated — and the structure brief
   carries the word nowhere, so no brief edit is owed.

10. **Three of this session's own claims failed their own enumeration and were
    corrected before the page shipped**, which is the discipline `which-files-k166`
    promoted applied to this draft: *the longest doc comment in the file* (part 3's
    is 34 lines against part 1's 32), *the only string in the file that is not part
    of the prompt's own text* (`PLUGIN` **is** part of it — the true claim is *the
    only `const` that is not one of the three parts*), and *the only source-owning
    chapter whose block can refuse nothing*, which was narrowed to the enumerated
    form: `prompt.rs` is the only one of the crate's twelve Rust roots with no
    fallible construct, the next lowest being three.

11. **The leaf's one in-session reviewer was spent on the page's factual claims,
    and it paid.** The four-step pass of `references/execute.md`: the claim was
    *every count, ranking, uniqueness and citation statement on the page is true
    of the artifacts as they stand today*; the reviewer was given the page and
    its contract with the conclusions stripped and an adversarial *find what is
    wrong* prompt. It returned seventeen findings. Classified: **twelve valid and
    actionable**, all corrected below; **four already fixed** by this session's
    own enumeration before the reviewer reported (the *if* count, the eight/two
    split, the five-versus-six never-composing tests, and the *only string*
    claim); **one rejected**; **none noise**, and **none** a contract stated
    unclearly. The rejected one is worth recording: the reviewer called *the
    fifth structure-brief count correction* wrong on an enumeration of four
    correction leaves, but that list omits `pick-test-count-k147`, which
    corrected the brief's *fifteen tests* to nineteen — so five is right, and
    this node's own brief already says so. Every finding was re-derived before
    being accepted, and the corrected text follows the re-derivation rather than
    the report.

12. **The correction that matters most is that this session's headline finding
    was not a finding.** The page claimed to have discovered that the mutation
    baseline's eleven failures are two causes rather than one. **Chapter 11
    recorded exactly that split** (*ten … with `NotAWorkspace`, and one … on a
    manifest the copy does not carry*), and chapter 10 reported ten under a wider
    626-test copy. What was actually wrong was `grove-loop-k123`'s brief, whose
    promoted summary flattened the split to *all eleven … because the copy is not
    a jj repository* — and that summary is what chapters 12 to 17 read. Both
    briefs now carry the split and the reason; the page credits chapters 10 and
    11 rather than claiming the distinction.

13. **Ten further claims corrected against re-derivation.** The Part 1 doc
    comment is the *third* longest argument in the file, not the second — the
    module header's forty lines beat it and part 3's thirty-four.
    `every_kind_names_a_skill_the_plugin_ships` **composes nothing**: it walks the
    skill directories, asserts every one but the spine is a `grove-<kind>`, and
    checks `skill_name`'s rendering against the listing. `label()` is reached
    **six** times in `loop_driver.rs`, not five — the sixth is
    `Kind::finish().label()`. The composed prompt is **1,843–1,921 bytes** over all
    twenty-three shipped kinds at a stated workspace root, with a **78-byte**
    spread that is path-independent while the absolute figure is not; the earlier
    1,885–1,960 came from sampling five kinds at one path. The instrument
    forecast belongs to the **node brief's** promoted finding, not to chapter 18's
    page, which says only *the next two chapters go back to the general case*.
    The percentage disagreement is **chapter 16's** record, not chapter 15's,
    which gave counts. Chapter 18 also calls itself *the chapter the outcome's
    third question was written for*, so this page no longer claims that status
    exclusively. Chapter 17 found a **structure-brief** error rather than a
    citation, so *the two chapters before this one each found a citation that did
    not hold* became chapter 18's two addresses and chapter 16's unresolvable
    name. The word *exception* named two different defects two paragraphs apart
    and now names one. And the citation census's own criterion admitted two grove
    task keys it excluded, so the page states eighteen document-and-record
    citations and accounts for the task keys and the intra-doc link separately.
