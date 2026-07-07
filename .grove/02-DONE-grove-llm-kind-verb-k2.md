# grove-llm-kind-verb-k2

**Kind:** work

## Goal

Add a `grove-llm kind [<leaf>]` verb that prints the task kind (`planning` or
`work`) of a leaf. This is the primitive the loop driver uses to key model
selection on the picked leaf's kind (ADR *model-per-task-kind*).

## Context

- ADR: `docs/adr/model-per-task-kind.md` (Q5 "kind plumbing").
- `src/leaf.rs` — `Kind` enum + `Kind::parse` (`"work"`/`"planning"`). Reuse it;
  do **not** add a second parser (single source of truth).
- `src/llm_cli.rs` — the `grove-llm` CLI surface. `cmd_brief_chain` is the shape
  to mirror: an **optional** leaf-path arg that, when absent, defaults to
  `tree_read::pick`'s next live leaf; a "no live leaves" diagnostic on stderr.
- `src/tree_read.rs` — `pick`, `brief_chain`. The task file's `**Kind:**` line
  is what to read/parse (see `content/TASK-FORMAT.md`: `**Kind:** work`).
- `src/cli.rs` — the `grove-llm` subcommand enum (add a `Kind` variant).

## Done when

- `grove-llm kind <leaf-path>` prints `planning` or `work` for that leaf.
- `grove-llm kind` (no arg) resolves `pick`'s next live leaf and prints its kind;
  on an empty grove it prints the standard "no live leaves" diagnostic on stderr
  (mirroring `brief-chain`) and exits 0.
- Parsing goes through `leaf::Kind` — no duplicate `**Kind:**` parser. If a task
  file is missing/garbled its `**Kind:**` line, the error is actionable (names
  the file), not a panic.
- Tests cover: work leaf, planning leaf, no-arg-defaults-to-pick, empty-grove,
  and a malformed/missing `**Kind:**` line.
- `cargo build` + `cargo test` green; `grove-llm kind --help` reads cleanly
  (follow `linkuistics:cli-tool-design`).

## Notes

- Keep `pick`'s existing stdout contract **unchanged** — the launched agent
  parses `pick` as a bare path. `kind` is a *separate* verb, not a `pick` flag.
- Output is a single lowercase token (`planning`/`work`) + newline — easy for the
  driver (leaf 03-driver-model-selection) to consume with a trivial match.
