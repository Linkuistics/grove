# plan-k1

**Kind:** planning

## Goal

Design how a grove uses **different LLM models for planning tasks vs work
tasks**, so the self-driving loop can launch each task's `claude` session on a
model matched to the task's kind (`planning` vs `work`) — e.g. a stronger
reasoning model for grilling/design, a cheaper/faster one for mechanical work.
Grill the design tree to shared understanding, capture durable decisions (ADR
+ glossary), and grow the tree into the leaves that implement it.

## Context

Key files this design touches:

- `src/loop_driver.rs` — the self-driving loop. `run_loop` launches a fresh
  foreground `claude` per task via `launch_session`, currently with **no
  `--model` flag**. It launches `start`/`continue` **blindly** — `grove-llm
  pick` runs *inside* the launched session, so the driver does not currently
  know the picked leaf's kind.
- `src/harness.rs` — the `Harness` static struct (`exec_bin`, `name_args`).
  A model flag would parallel `name_args` (per-harness template).
- `src/leaf.rs` — the `Kind` enum (`Work` / `Planning`) + `Kind::parse`.
- `src/llm_cli.rs` — `grove-llm pick` (`cmd_pick`) prints only the leaf path.
- Task kind lives in each task file's `**Kind:**` line (`content/TASK-FORMAT.md`).
- ADR `docs/adr/self-driving-loop.md` — the loop's design record.

## Done when

- The design tree is grilled to shared understanding.
- Durable decisions are captured (an ADR if it clears the when-to-write bar;
  glossary terms inline).
- The tree is grown into concrete implementation leaves.

## Decisions (running log)

**Q1 — mechanism & locus (settled).** Model selection is a **launch-time
default set by the loop driver**, using Claude Code's **native `--model`
flag** (no router, no proxy, no external project).

Evidence (Claude Code model-config docs): selection priority is
`in-session /model` > `--model` flag > `ANTHROPIC_MODEL` env > settings file;
on a **Max subscription** all bill against the subscription (OAuth), no API
key needed. Consequences:

- The driver **peeks**: run `grove-llm pick`, read the leaf's `**Kind:**`, then
  launch `claude --model <planning|work model>`. Stateless (re-derived each
  iteration, constraint 1); matches the driver's existing role (it already sets
  the session name at launch).
- **Mid-session switch is free & native.** In-session `/model` is *highest*
  priority, so it always overrides the launch model. This covers the "a work
  session becomes substantial planning" case with **zero grove code** — the
  driver sets the default; the agent/human overrides in-session at will.
- **No model router.** claude-code-router et al. are for multi-*provider*
  routing, proxy the API, need an API key, and risk breaking/draining Max
  billing. Native `--model` does Opus↔Sonnet routing on the Max sub for free.
- **Skills repo is not the home.** `~/Development/skills` is the general
  coding-style plugin. Any (optional, lazy) mid-session-switch *guidance* line
  belongs in grove's own `content/`, not there — and only if we decide it earns
  its place (open, later question).

**Q2 — config surface (settled).** **Env vars** `GROVE_PLANNING_MODEL` /
`GROVE_WORK_MODEL`, read by the driver at each launch. **Unset ⇒ pass no
`--model`** (inherit the user's own default; never clobber their
`ANTHROPIC_MODEL`/settings — required by the priority order). Walk-away-able
(the shell-loop equivalent expresses it trivially), stateless, re-derived every
`grove do`, opt-in (constraints 1, 4, 6). Rejected: `grove do` CLI flags (don't
survive a loop restart — fights restart≡continuation); config file (adds
non-task state to `.grove/`).

**Q3 — harness generality (settled).** Add a per-harness `model_args` template
to the `Harness` struct (`claude: ["--model"]`), parallel to `name_args`. Wire
claude now; codex best-effort/lazy. Keeps the existing abstraction intact.

**Q4 — granularity (settled).** Two buckets only — model keyed purely on the
leaf's `Kind` (planning / work). Native `/model` covers ad-hoc needs when
present. Per-leaf `**Model:**` override is a clean v2 follow-up if it earns its
place (its only unique value: a `work` leaf needing a strong model while running
*unattended* in the relaunch loop).

**Q5 — kind plumbing (decided detail).** The driver learns the kind via a new
`grove-llm kind [<leaf>]` verb — mirrors `brief-chain`'s "optional leaf arg,
else default to `pick`'s next live leaf" shape; prints `planning` or `work`
using `leaf::Kind::parse` (single source of truth, keeps parsing in Rust). The
default `pick` output is **unchanged** (the launched agent still parses it as a
bare path).

**Start-path rule (forced consequence).** On a brand-new grove the driver
launches `start` while `.grove/` does not yet exist (the agent runs `root-init`
*inside* that session), so there is no leaf to peek. But `root-init`'s first
leaf is **always planning** by construction (fresh-grove-start-contract), so the
`start` path uses `GROVE_PLANNING_MODEL` unconditionally. The `continue` path
peeks the next live leaf's kind. Either way, an unset env var ⇒ no `--model`.

_Appended as each question settles._
