# docs-pi-harness-mentions-k20

**Kind:** work

## Goal
Decide whether the "multi-harness repo" docs need a third mention for `pi`,
and fix them if so — surfaced while fixing review-fix-docs-k19's D1-D4, out
of scope for that leaf.

## Context
Three docs describe multi-harness detection as exactly `.claude/` +
`.codex/`, with no mention of `.pi/`:
- `README.md:29` — "auto-detection from the repo's `.claude/` and `.codex/`
  directories"
- `docs/grove.md:60` — "a multi-harness repo (both `.claude/` and `.codex/`)"
- `docs/workflows/start.md:88` — "has both `.claude/` and `.codex/`
  directories — i.e. both harnesses have been used in this repo"

But `harness::detect_in_repo` (`src/harness.rs`) scans all three registry
rows, `pi` included — so a repo with a `.pi/` dir present *would* be treated
as multi-harness by the code, contradicting these docs' "only two" framing.

However, `src/harness.rs:71-74`'s own comment on the `pi` row cautions this
may not matter in practice: "Opt-in detection marker; pi does not create
repo-local `.pi/` dirs, so explicit `--harness pi` + the stamp is the normal
binding route." If that's right, `.pi/` auto-detection is a code path that
essentially never fires, and the docs' two-harness framing is a reasonable
simplification, not a bug — unlike D1-D4, which were unconditionally false.

## Done when
- Judgement call made on whether this is worth fixing: either (a) the three
  spots are corrected to mention `pi`/`.pi/` accurately, or (b) this leaf is
  retired as abandoned (with the reasoning above sufficing as the record,
  per the grove methodology's pruning guidance — no ADR needed for a
  no-op-in-practice code path).
- If fixed: `cargo test`, `fmt`, `clippy` clean; one focused commit.

## Notes
Not one of branch-review-k14's D1-D4 findings; this is a new concern noticed
while executing review-fix-docs-k19, externalized per the grove
methodology's Decompose step rather than folded into that leaf.
