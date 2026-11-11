# ravel-book-inside-k330

## Goal

Draft chapter 8 of the Ravel book for 2.0.0, “Follow a session through the
code”. A reader follows a successful, a refused and a cancelled session
through the code that exists now.

## Context

- Read the node brief `_ravel-book-inside.md` and its pointers. The structure
  brief is Ravel `docs/book/contract.md`, sections 1 and 2.
- Chapter 7, as drafted by `ravel-book-inside-k329`, sets the words for the
  parts. Restate them in chapter 8 and use the same concrete example.
- The draft of 2026-11-11 found these stale claims in chapter 8:
  - “Evaluate and check the configuration script” names `evaluate_and_resolve`,
    `choice::judge` and `worker::evaluate`. The Rhai path in
    `crates/ravel/src/choice.rs` is `choose_prepared`, then
    `evaluate_and_resolve_rhai`, then `rhai_worker::front::evaluate`. Its
    checks are the closures `judge` (`rhai_context`) and `reply_judge`
    (`config::judge_result`).
  - The exchange table names `crates/ravel/worker/src/main.ts`, which does not
    exist. The worker is `crates/ravel/src/rhai_worker/role.rs`, chosen by
    `arg0` in `main`. The frames are in the module comment of
    `crates/ravel/src/rhai_worker/mod.rs`: `hello`, `evaluate`, `host`,
    `reply`, `context`, `delivered`, `selection` and `failure`.
  - The front, not the worker, reads the entry. The source travels in the
    `evaluate` frame. Host calls are answered on a Tokio current-thread
    runtime in `rhai_worker/serve.rs`.
  - “The worker supplies `undefined`” is TypeScript. Check what `select`
    receives without a context in Rhai.
  - The refusal section and its failure tables name `choice::judge`,
    `Validator::validate`, `choice::deliver` and `worker::expired`. Check each
    one against the source.
- Every excerpt is whole lines of a tracked file. Read `docs/book/excerpts.md`.
  The 12 current excerpts pass, but passing proves only that the lines exist.
  Check that each excerpt is still on the path that the text describes.

## Done when

- Chapter 8 follows the Rhai path from `main` to the result write, then the
  refusal and cancellation paths. Each claim is checked against the source.
- `task docs:excerpts`, `task docs:examples`, `task docs:ste` and
  `task docs:breaks` pass for chapter 8.
- The chapter follows each Read 1 pattern of contract section 2.
- The leaf lists figure needs for the art step in the parent node brief's
  `## Handed forward`, with `session-code-path` and `selection-cancellation`
  checked against the new text.

## Notes

Work in `Ravel.skill-directory-delivery` on `skill-directory-delivery`. If a
heading changes, reconcile every inbound link in the book (L65). Chapter 9
links to “Finish the managed job” and “Choose the final outcome”.

## Decisions (running log)

- 2026-11-12: The structure brief is `docs/book/contract.md` section 1. It
  states the reader (“The reader”, “What the reader can do afterwards”), the
  ordered section plan (“The section plan”, chapter 8's row and its note that
  chapter 8 is an orientation) and the emphasis and exclusions.
- 2026-11-12: Baseline before any edit: `task docs:excerpts` passed 10
  excerpts in chapter 8. Every stale claim in the leaf's context was real. Two
  more were stale: `main` calls `Cli::command().try_get_matches_from` and
  `Cli::from_arg_matches`, not `Cli::try_parse_from`, and `config::selected`
  is now `config::judge_result`. The alt text of `selection-cancellation` in
  the chapter said that the worker sends INT. In the figure, the fake harness
  sends it.
- 2026-11-12: Answers to the leaf's questions, from the source on
  `skill-directory-delivery` at Ravel commit `56dac027f917`:
  - The Rhai path is `choose_prepared`, `evaluate_and_resolve_rhai`,
    `rhai_worker::front::evaluate`, then `worker::supervise` with
    `front::converse` and `serve::serve`. The checks are the closures `judge`
    (`rhai_context`) and `reply_judge` (`config::judge_result`).
    `judge_result` runs twice: inside `converse`, before teardown, and again in
    `evaluate_and_resolve_rhai` after it.
  - Without a context, `request_map` stores JSON null as `request.context`,
    and `evaluate_with` passes it to `select`. Rhai shows it as `()`. A probe
    printed `type_of(context)` as `()`, and the refusal example now claims it.
  - `Validator::validate` and `choice::deliver` do not exist. The context
    checks are `config::loader_refused` and `context::deliver`.
    `worker::expired` exists as the private `expired` in `worker.rs`.
- 2026-11-12: The chapter follows chapter 7's `ravel-parts` example. It adds
  two complete examples that change one file each: `ravel-parts-refusal` (the
  script refuses after the harness run, status 3) and `ravel-parts-cancel`
  (the agent sends `finished`, then INT, status 130). Selection cancellation
  stays with chapter 5's `cancel-selection`, which the figure draws. A probe of
  it in the `ravel-parts` shape showed `selection_cancelled` with
  `worker_failed` under “original failure:”, then `session_cancelled`. The
  text states this from the source and that run, not from a claimed line.
- 2026-11-12: Headings with inbound links stay: “Finish the managed job”,
  “Complete cleanup and write the result”, “Follow cancellation”, “Choose the
  final outcome” and “Find where a change belongs”. A subsection of “Answer the
  script's requests” became a section, because the contract adds a heading
  level only for two subsections or more.
- 2026-11-12: The result channel now has three consumers: the runner, `run`
  and `harness::execute`. “Find where a change belongs” names all three and
  adds a table of first files for frequent changes.
- 2026-11-12: The leaf's one in-session reviewer read the evaluation path
  against the source. The claim was that the text about the worker start, the
  host bridge, the refusal and the selection cancellation is true of the code.
  All 11 findings were valid and actionable, and each fix follows the cited
  source lines. Three statements were false: `authority::resolve` does not
  read the entry, an unreadable entry refuses after the worker starts, and
  skill preparation can give status 5. The other eight named the wrong actor
  or were incomplete: the socket watch, the deadline before the `hello`, the
  descriptor moves in the child, the worker start failures, the harness
  cleanup code, the context failures, the runner's thread and the label of a
  refusal after teardown failure. No second review was needed, because each fix
  is a correction against a named source line.
- 2026-11-12: Checks on the final chapter 8. `task docs:examples` passed its 2
  examples in five consecutive runs. A control copy that claimed `map` for the
  context and the worker as the agent's parent failed both. `task docs:ste
  --strict` and `task docs:breaks --strict` found 0 problems.
  `task docs:excerpts` passed 20 excerpts. A link check of every page found no
  broken file or anchor, and a planted bad anchor failed it.
  `task docs:book:build` passed. The chapter 8 page has 18 headings with 18 ids
  and no raw Markdown.
- 2026-11-12: Lessons L76 and L77 are in `docs/documentation-lessons.md`. The
  figure needs are in the parent node brief's `## Handed forward`.
- 2026-11-12: Ravel commit `f6ee31358634` on `skill-directory-delivery` holds
  chapter 8 and the lessons. The bookmark is local, as for the earlier leaves.
  The copy-edit leaf `ravel-book-inside-k309` already stands, so this leaf
  cuts no copy-edit leaf.
