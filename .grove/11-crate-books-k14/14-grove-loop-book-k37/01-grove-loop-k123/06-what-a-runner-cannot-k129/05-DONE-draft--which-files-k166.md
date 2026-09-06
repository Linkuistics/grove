# which-files-k166

## Goal

Draft chapter 18 of the `grove-loop` book, *Which files take part* —
`crates/grove-loop/src/session_config.rs` lines 1–358, one whole root — to a
valid slice through `whose-file-and-whether`.

## Context

- **The rule: everything a template *is* belongs to `keyed-launch`; what is left
  here is whose file, and whether the second one is admissible.** The personal
  file's path, the two roots the delta is searched at, `DeltaRoots`,
  `TemplateSource`, the four slots grove's templates are written against, and the
  refusal of a **tracked** delta — a question about grove's worktree, answered
  through grove's version-control seam, and the boundary between an untrusted
  repository and arbitrary code execution.
- At 44% comment prose by the structure brief's count the charter is **do not
  restate**: the comments argue and the fragment graph quotes them verbatim.
  Prose connects arguments across items, names the test, and stops. If you take
  your own count, say which rule you counted under — chapter 15 recorded a
  one-point disagreement with the brief's percentages and no leaf was cut for it.
- **This chapter owns the stale sentence and must adjudicate it, not correct it.**
  `session_config.rs` line 89, in `TemplateSource`'s doc comment, says the loop
  re-reads the configuration once per iteration. `loop_driver.rs` calls
  `templates.load(&delta_roots)` **twice** — line 241 before
  `transition_to_current` and line 260 after the leaf is selected. The type's own
  doc comment names both reads in the same sentence that says *once*, so the
  count is stale rather than the design. State it beside the fragment that
  reproduces the bytes; `template-source-read-count-k86` holds the fix and lands
  after this book. Chapter 20 owns the two calls and shows the refutation.
- Pinned by `the_four_slots_are_the_vocabulary_and_prompt_is_the_required_one`,
  `a_snapshotted_jj_delta_is_refused_in_both_jj_shapes`,
  `a_trackedness_probe_that_cannot_be_completed_fails_closed` and
  `a_grove_configuration_conforms_to_the_runners_own_kit` — all in
  `crates/grove-loop/tests/`, which is evidence and not corpus, so **read each
  test's fixture and its helpers** before writing what it pins. A cited test can
  pin a narrower claim than its citation.
- The chapter keeps `complete-session-configuration`, named in prose and never
  linked. It declares no anchor of its own.
- No `docs/ARCHITECTURE.md` residue marker is assigned to this chapter.

## Done when

- `book-check --repo . --book docs/walkthroughs/grove-loop --through
  whose-file-and-whether --check all` is valid at 9,673 resolved lines, 860
  deferred, `final=false`.
- The `whose-file` ownership row reads `resolved`, the chapter-1 cast row owned
  by `whose-file-and-whether` reads `explained`; contents and navigation updated;
  `scripts/check.sh` red on `book-check` alone, and this file says so.

## Notes

**The corpus is frozen.** The stale sentence is adjudicated on the page and not
edited.

## Decisions (running log)

- **The block holds no `#[test]` and no `#[cfg(test)]` module.** 358 lines, all
  production. So the *supply the claim* obligation has nothing in this chapter to
  attach to — chapter 18 is not on the structure brief's list for it, and the
  four tests the brief pins live in `crates/grove-loop/tests/`, which is evidence
  and not corpus. The chapter cites them; it reproduces none of them.
- **Comment prose measured at 156 of 358 lines = 43.6%**, counting lines whose
  first non-space characters are `//`. The brief's *What each chapter's prose
  owes* says 44% for `session_config.rs`; the two agree at the rounding, so
  unlike chapter 15 there is no disagreement to record. The chapter states the
  count and the rule, not the percentage alone.
- **Every one of those 156 lines is `///` or `//!`; the root has zero plain
  `//`.** That is the opposite of `tree_lifecycle.rs` and `task_grow.rs`, and it
  means the crate's running blind-spot caveat does not apply to this root: with
  no `#[cfg(test)]` module either, `cargo doc`'s whole reach covers the block.
- **`cargo doc --no-deps --document-private-items -p grove-loop` reports 30
  warnings over the crate and names `session_config.rs` in none of them —
  confirmed as a measurement by a control.** Planting `[`crate::no_such_item`]`
  in the module header of a scratch copy took the count to 31 and named
  `session_config.rs:16`. So the clean read is evidence about this file rather
  than the instrument failing to look at it.
- **Twenty backticked tokens' worth of names in the block, and every one
  resolves.** Enumerated from the bytes rather than swept against expected
  names, per `the-lease-k164`. `main_repo` and `is_tracked` are
  `crates/jj-workspace/src/lib.rs` 116 and 162; `keyed_launch::run` is
  `crates/keyed-launch/src/run.rs` 367; `Templates`' four methods are
  `templates.rs` 104, 150, 162 and 178; `crate::run` is `loop_driver.rs` 193.
  Unlike chapter 16's block there is no `probe_lease_holder` here — no name in
  this block resolves to nothing.
- **Exactly one later-owned symbol is named, and no floor row is owed for it.**
  Sweeping the block against every item declared in `prompt.rs` (chapter 19) and
  `loop_driver.rs` (chapter 20) returns `crate::run` alone, at lines 86 and 96.
  The chapter-1 cast row `` `run`, `LoopOutcome` `` already states the minimum
  — *the loop itself, and how it ends* — so this is the exception the structure
  brief names (the same one that spared chapter 2 a row for `Handle::render`),
  not a row the manifest omitted. `keyed_launch::run` at line 224 is another
  crate's and owes nothing. **Chapter 18 adds no early-use row**; it flips the
  `` `SessionConfig`, `TemplateSource` `` cast row to `explained`.
- **A third citation defect in the corpus, of the `llm_cli` class:
  `session_config.rs` line 271's *the search precedence requirement 6 fixes*
  cites a record that does not exist.** `requirement 6` is a unique string in
  the whole repository. `docs/specs/module-decomposition.md` numbers its
  **decisions** 1–11 and *names* its four requirements; read as decision 6
  (*Configuration completeness is per-kind and just-in-time*) it still does not
  fix a search precedence — decision 6 moves the quantifier and states
  overrides-never-supplies, at the revision the comment was written
  (`ykxruswv`, `loop-crate-driver-k22`) as today. What does fix the precedence,
  and states the very absence rule the comment is defending, is
  `docs/adr/untracked-configuration-delta.md`, which this same file already
  cites at lines 9, 15 and 331. **The behaviour the comment claims is correct
  and only the address is wrong**, so it is adjudicated on the page in one
  clause under the `paths-k142` precedent rather than cut as a leaf — and
  `template-source-read-count-k86` is already editing this file's comments
  inside the frozen line counts and can carry it. This does not contradict the
  structure brief's *No third stale claim was found*: that sentence is about
  claims the code does not bear out, and this is a misattributed citation.
- **The evidence file holds twenty tests, not four.**
  `crates/grove-loop/tests/session_config.rs` is 713 lines and 20 `#[test]`
  functions in four labelled sections; the structure brief pins four of them for
  this chapter and does not claim to be exhaustive. Counted against the file per
  `structure-brief-test-list-k172`, not against the brief's prose list. The
  chapter reproduces none — `tests/` is evidence, not corpus.
- **The read-count refutation re-derived rather than inherited.**
  `loop_driver.rs` calls `templates.load(&delta_roots)` at 241 and 260, both
  inside the `loop {` body opened at 225 — the first bound to
  `pre_transition_config` and consumed by `require(Kind::finish().label())`, the
  second to `config` and consumed by `source` and `expand`. `TemplateSource::load`
  is `pub(crate)` with exactly three production call sites in the workspace:
  those two and `session_config.rs:195` in `load_for_worktree`. So *once per
  iteration* is refuted inside this book's own corpus, as the brief says.
- **`a_trackedness_probe_that_cannot_be_completed_fails_closed` is carried
  entirely by the `with_context` string, and that string is not unique to the
  path it names.** Mutation in a workspace copy: control 20/20 green; replacing
  the context message at line 310 with one not containing *is tracked* turns
  exactly that test red, and the underlying error it then shows is jj's own
  *Failed to read commit backend type … No such file or directory* — no
  occurrence of *tracked* anywhere in the chain. But `refuse_a_tracked_delta`'s
  `bail!` reads *it is tracked in version control*, which **also** contains the
  asserted substring, so a genuinely tracked delta satisfies the same assertion.
  What the test pins is *the load failed while asking about trackedness rather
  than resolving to the personal file*; it does not pin *the probe was
  unanswerable*. This is `the-epoch-k165`'s shared-context-string class.
- **The refusal's whole remedy is held by nothing, and the third assertion that
  looks like it holds it cannot fail.** In
  `a_snapshotted_jj_delta_is_refused_in_both_jj_shapes`, assertion 3 checks the
  error contains `/.grove.kdl` — but assertion 2 already requires it to contain
  the candidate's full path, which ends in exactly those bytes, so 3 is entailed
  by 2 and adds nothing. Confirmed by mutation: deleting the entire
  `Untrack it (…)` sentence — the command, the `.gitignore` ordering and the
  parenthetical — leaves all 20 tests green. `jj file untrack` occurs nowhere in
  the workspace but this file's own comment and message. The four lines of doc
  comment at 302–306 arguing why the ignore line is named first are argument the
  suite does not reach.
- **That ordering argument's cited evidence re-run, and it holds.** The comment
  quotes `jj file untrack --help` at jj 0.44.0 — *"Paths to untrack. They must
  already be ignored."* The installed jj is **0.45.1** and emits that sentence
  verbatim, so the quotation survives the version bump. The version named is
  older than the one to hand; the claim it supports is still true.
- **A fourth citation defect, and `cargo doc` structurally cannot see it.**
  `session_config.rs` line 331 cites the untracked-configuration-delta record as
  a **Markdown link** to `../docs/adr/untracked-configuration-delta.md`. Every
  other citation in the block — six, at lines 15, 28, 37, 58, 93 and 209 — is a
  backticked path in prose, which is the file's convention and is stable. The
  link resolves from nowhere: rustdoc emits the target verbatim, so from
  `grove_loop/session_config/fn.delta_is_tracked.html` it points inside the
  generated tree where no `docs/adr/` exists, and read relative to the source it
  would be `crates/grove-loop/docs/adr/`, which does not exist either. **No
  warning is emitted**, because rustdoc checks intra-doc links and never an
  explicit URL — a different blind spot from `unresolved-doc-links-k151`'s five,
  which it does report. Same disposition as `requirement 6`: adjudicated on the
  page, address-only, and a candidate for `template-source-read-count-k86`.
- **Uniqueness claim caught in my own prose before it shipped.** I wrote that
  `ExpansionContext` is *the one item in the block with no prose of its own*.
  Enumerated rather than eyeballed, **seven of the block's twenty-five items
  carry no doc comment**: `CONFIG_PATH`, `ExpansionContext`, both `impl` blocks,
  `SessionConfig`, `SessionConfig::path` and `read`. The page now states the
  enumeration. This is the class the campaign keeps meeting — a count is cheaper
  than a correction, and the sentence was wrong by a factor of seven.
- **Slice proved.** `book-check --through whose-file-and-whether --check all` is
  valid at **9,673 resolved, 860 deferred, `final=false`** — the figures the
  brief predicted. The `whose-file` ownership row reads `resolved`, the
  chapter-1 cast row `` `SessionConfig`, `TemplateSource` `` reads `explained`,
  and the only rows left `pending` are chapter 19's and chapter 20's.
  `bash scripts/check.sh` is **red on `book-check` alone, 1 of 8**, every failure
  a `--final` consequence of chapters 19–21 not existing yet.
- **No early-use row added and none owed** — see the enumeration above. The
  chapter's page carries 22 literal fragments under one composite, contiguous
  over 1–358.
