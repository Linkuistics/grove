# driver-model-selection-k3

**Kind:** work

## Goal

Make the self-driving loop launch each `claude` session on a model chosen by the
picked leaf's kind, using native `--model` — per ADR *model-per-task-kind*.
Configured by `GROVE_PLANNING_MODEL` / `GROVE_WORK_MODEL`; unset ⇒ no `--model`.

## Context

- ADR: `docs/adr/model-per-task-kind.md` — the full design (mechanism, env
  config, unset⇒no-flag, start-path rule, per-harness template).
- Depends on leaf `grove-llm-kind-verb-k2` (the `grove-llm kind` verb) — the
  `continue` path calls it to resolve the next live leaf's kind.
- `src/harness.rs` — add a `model_args: &'static [&'static str]` field to the
  `Harness` struct, parallel to `name_args`. `claude: ["--model"]`. codex:
  best-effort/lazy (leave `&[]` if its flag/model-names aren't verified — an
  empty template means "skip model selection for this harness").
- `src/loop_driver.rs` — `run_loop` / `launch_session`. This is where the
  model is chosen and appended. `run_loop` already branches
  `verb = if worktree.join(".grove").is_dir() { "continue" } else { "start" }` —
  reuse that branch for the kind:
  - **`start` path** (no `.grove/` yet): kind is planning by construction
    (fresh-grove-start-contract) ⇒ use `GROVE_PLANNING_MODEL`.
  - **`continue` path**: run `grove-llm kind` (against the worktree) ⇒
    planning/work ⇒ use the matching env var.
  - Chosen env var **unset ⇒ pass no `--model`** (the load-bearing rule; never
    clobber the user's own `ANTHROPIC_MODEL`/settings default).
  - When set, append `harness.model_args` + the value to the command (only if
    `model_args` is non-empty).

## Done when

- With `GROVE_PLANNING_MODEL`/`GROVE_WORK_MODEL` set, the launched `claude`
  command includes `--model <value>` matching the leaf's kind; unset ⇒ the
  command has no `--model` (byte-for-byte the current behavior).
- `start` path uses the planning model; `continue` path uses the kind of the
  next live leaf.
- An integration test uses the existing `GROVE_HARNESS_BIN` override (a recorder
  script/binary) to assert the exact args: `--model` present with the right value
  when the env var is set, and **absent** when it is unset. Cover planning-kind,
  work-kind, and unset.
- `cargo build` + `cargo test` green.
- End-to-end smoke: `GROVE_WORK_MODEL=sonnet GROVE_PLANNING_MODEL=opus` with a
  recording `GROVE_HARNESS_BIN` shows the expected `--model` per iteration.

## Notes

- Keep the driver stateless — read env + `grove-llm kind` each iteration; store
  nothing. Restart ≡ continuation must hold (self-driving-loop, constraint 1).
- Resolving kind on the `continue` path is a driver-side `pick` peek; within one
  iteration it returns the same leaf the launched agent will pick (deterministic
  pick, same tree) — note this in a comment so the TOCTOU non-issue is explicit.
- The `src/launch.rs` `exec_harness` path (non-loop `start`/`continue`/`retire`
  verbs) is **out of scope** — model selection is a loop-driver concern only.
  If trivial to thread through, fine, but don't expand scope; the loop is the
  only path that relaunches per task.
