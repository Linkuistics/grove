# 010-remove-redundant-verbs

**Kind:** work

## Goal
Remove `grove start`, `grove continue`, and `grove finish` as public CLI verbs so
that `grove do` is the sole lifecycle entry verb. The internal `start()` and
`continue_grove()` helpers stay (they back `do`'s dispatch); only the public
`Command` variants and their arg structs go where unused.

## Context
- `src/cli.rs:20-35` — `Command` enum: remove the `Start`, `Continue`, `Finish`
  variants; check `StartArgs` (cli.rs:251) usage — `do` calls `start()` with a
  constructed `StartArgs` (launch.rs:56), so the struct stays but its `clap`
  derive may no longer need to be a subcommand. `NameArgs` is shared by several
  verbs — keep it.
- `src/cli.rs:283-301` — `run()` match arms for `Start`/`Continue`/`Finish`.
- `src/launch.rs:34-70` — `continue_grove`, `do_grove`, `finish`. Keep `start()`
  and `continue_grove()` as internal helpers; remove `finish()` (the finish flow
  moves in-session — see 020) OR leave `finish()` dead-coded for 020 to repurpose;
  decide during this task.
- **Decision pending for this task:** does `do` need to expose `--start-point`?
  Today only `grove start` has it (cli.rs:254-255); `do`'s new-grove path
  hard-codes `start_point: None` (launch.rs:58). Removing `start` removes the only
  way to set a non-default branch point. Recommend adding `--start-point` to
  `NameArgs`/`do` to preserve the capability.
- **Doc/skill/prompt sweep** — `grove start`/`grove continue`/`grove finish` are
  referenced widely. Find every occurrence and update to `grove do` (or to the
  in-session finish flow). Known sites: `CONTEXT.md` ("Drain" entry says
  "performed at every `grove start` and `grove continue`"; "cli version" entry
  mentions removed verbs), `.claude/skills/grove/SKILL.md` (the loop intro names
  `grove start`/`grove continue`; the Finish step), the launcher prompts in
  `.claude/skills/grove/prompts/` (the `finish.md` prompt is obsoleted by 020 —
  coordinate), `README.md` CLI reference, `docs/` (workflows, grove.md, ADRs).
  Use `git grep` to enumerate exhaustively.

## Done when
- `grove start`, `grove continue`, `grove finish` are gone from `--help` and the
  `Command` enum; `grove do` covers every entry path; `--start-point` decision
  implemented.
- `cargo build` + `cargo test` pass; `grove do <name>` exercises new/continue/
  orphaned paths.
- No stale `grove start`/`grove continue`/`grove finish` references remain in
  docs/skill/prompts/CONTEXT/ADRs (verified by `git grep`).
- An ADR is raised recording "`do` is the sole lifecycle verb" (the decision is
  settled — see root BRIEF running log; cite the v4.0.0 list/version removal
  precedent, ADR-0007).

## Notes
- Sibling-coupled with 020: both touch the launcher prompts and the SKILL.md loop
  text. 010 runs first and removes references; 020 layers the finish behaviour on
  the simplified surface. If `finish.md`'s fate is ambiguous, leave it for 020.
- This is a breaking CLI change — see the root BRIEF "Release" note.
