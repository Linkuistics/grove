# ravel-book-inside-k329

## Goal

Draft chapter 7 of the Ravel book for 2.0.0, “The parts of Ravel”. After it,
a reader can explain the Ravel CLI and its separate Rhai worker. The reader can
also say why Ravel reaps the worker before the managed launch. The reader can
say how the runner package and the other parts compose.

## Context

- Read the node brief `_ravel-book-inside.md` and its pointers. The structure
  brief is Ravel `docs/book/contract.md`, sections 1 and 2. Section 1 gives
  chapter 7's purpose in the section plan.
- Source of truth: the code on `skill-directory-delivery`,
  `docs/specs/configuration-evaluation.md`, `docs/specs/skill-discovery.md`
  and `CONTEXT.md`.
- Chapter 7 must describe the Rhai worker as Ravel's own binary in an
  internal role. It must also describe the host bridge, the reader thread,
  the harness runs and the skill directories. An LLM call is a harness run. Ravel has no LLM adapter.

## Done when

- Chapter 7 describes 2.0.0, and each claim is checked against the source.
  It names no Bun worker, no SDK and no `libexec` layout, except in a
  statement of what was removed.
- The chapter follows each Read 1 pattern of contract section 2. It opens
  with a concrete case before the general model.
- `task docs:examples`, `task docs:ste`, `task docs:breaks` and
  `task docs:excerpts` pass for chapter 7.
- Figure needs for chapter 7 are in the parent node brief's
  `## Handed forward`.

## Notes

Work in `Ravel.skill-directory-delivery` on `skill-directory-delivery`.

## Decisions (running log)

- 2026-11-11: The structure brief is `docs/book/contract.md` section 1. It
  states the reader (“The reader”, “What the reader can do afterwards”), the
  ordered section plan (“The section plan”) and the emphasis and exclusions.
- 2026-11-11: The draft of chapters 7 to 9 was too large for one session.
  Chapter 8's evaluation section names functions that the Rhai path no longer
  calls and a TypeScript file. The leaf became node `ravel-book-inside-k307`
  with one draft child for each chapter. This leaf drafts chapter 7.
- 2026-11-11: The baseline before any edit: `task docs:excerpts` passed 12
  excerpts, and `task docs:ste -- --strict` found 0 problems in chapters 7 to 9.
- 2026-11-11: Chapter 7 opens with a new complete example, `~/ravel-parts`.
  It extends chapter 3's first session with one harness run, `advisor`. The
  fakes write their parent's arguments, the `RAVEL_CONFIG_RUN` marker and
  their `RAVEL_EXIT_FILE` path to files. A run of build `76ddf6dec652` on
  macOS gave `ravel start-session` as both parents. The marker `1` was only in
  the harness run. The two result paths differed, and `tmp` was empty
  afterwards.
  `ps -o comm=` cannot tell the CLI from the worker, because both are the same
  binary, so the fakes print `ps -o args=`. The output depends on neither the
  prompt nor `RAVEL_SKILL_DIRECTORIES`, so a default skill directory cannot
  change it.
- 2026-11-11: `run.rs` writes the script's diagnostics, then the routing
  notice, in sequence. Worker operation reports come before both. Thus, one
  “Standard error includes” claim can cover the `config stdout:` line and the
  notice (L55).
- 2026-11-11: The reasons in chapter 7 come from their records (L54). The
  worker process comes from `configuration-evaluation-precedes-the-launch`.
  Rhai comes from `configuration-script-uses-rhai`. Skill preparation comes
  from `skill-preparation-precedes-evaluation`. The reader thread and the
  runtime come from the specification section “The bridge between the script
  and the CLI”.
- 2026-11-11: The runner has an interface that the old chapter 7 omitted:
  `run_captured` with `CapturedLaunch` and `StopRequest`, added by
  `rhai-harness-run-k23`. Harness runs use it, and it installs no signal
  handler. `run_noninteractive` has no production caller in Ravel. Only a test
  in `cancellation.rs` calls it. Chapter 9 describes both, so the chapter 9
  draft checks them.
- 2026-11-11: A Homebrew installation writes `share/ravel/installation.json`.
  With no `skills.sources`, an installed Ravel then reads the default skill
  directory, also under `env -i` with an empty `HOME`. If that directory holds
  a source, the prompt gains the preamble. Chapter 3's first session prints the
  prompt and does not say this. The checkout build has no record, so the
  examples check cannot see it. This is outside chapters 7 to 11, so it goes to
  the proof's `## Handed forward` entry.
- 2026-11-11: Chapter 7 links to no decision record. A link from
  `docs/book/src/` to `../adr/` was wrong, and the built site holds no
  record. The chapter names each record and its path in code font, as chapter
  6 names a specification (L74).
- 2026-11-11: Chapter 7 keeps the three headings that other pages link to:
  “The runner owns the local job mechanics”, “The result channel carries the
  value” and “The installed executable supplies both roles”. Its other
  headings have no inbound link.
- 2026-11-12: Checks on the final chapter 7, build `76ddf6dec652`.
  `task docs:examples` passed its 1 example. A scratch copy that claimed
  `RAVEL_CONFIG_RUN is 2` failed. `task docs:ste --strict` and
  `task docs:breaks --strict` found 0 problems. `task docs:excerpts` passed 12
  excerpts. A link check of every page found no broken file or anchor. A
  planted bad anchor in chapter 9 failed it, and the plant was removed.
  `task docs:book:build` passed, and the chapter 7 page has no raw Markdown and
  12 headings with 12 ids.
- 2026-11-12: A walk of the visible steps in `zsh -f`, with the files saved
  without the executable bit, matched each claimed line. A control without
  `chmod` refused as `program_unexecutable`. This shows that the steps run. It
  does not show that a reader understands them.
- 2026-11-12: Lessons L73 to L75 are in `docs/documentation-lessons.md`. The
  figure needs and the chapter 3 case are in the parent node brief's
  `## Handed forward`. The copy-edit leaf `ravel-book-inside-k309` already
  stands, so this leaf cuts no copy-edit leaf.
- 2026-11-12: Ravel commit `56dac027f917` on `skill-directory-delivery` holds
  chapter 7 and the lessons. The bookmark is local, as for the earlier leaves.
