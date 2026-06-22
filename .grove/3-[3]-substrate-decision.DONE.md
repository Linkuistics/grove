# 3-[3]-substrate-decision

**Kind:** planning

## Goal

Using `020`'s `docs/research/loop-substrate-options.md`, **decide the loop
substrate** for grove-on-a-workflow, then **grow the implementation leaves** for
the refactor.

## Context

Spike landed (`docs/research/loop-substrate-options.md`). Its evidence reversed
the founding premise: **do not make grove an Archon workflow** — Archon's
`interactive:` is an approve/reject gate (fails the make-or-break grilling gate
D), the restart hypothesis is refuted, and it adds an external dependency plus a
DB of load-bearing run state. The recommendation was a thin, stateless,
grove-owned loop over `grove-llm pick`.

Read the root `BRIEF.md`, the retired `010-plan` running log (D1–D8), and the
spike doc first.

## Decisions (running log)

**Inputs from the user (2026-06-20, opening the grilling):**

- **Substrate direction → transparent PTY-wrap (spike candidate 3).** A
  transparent PTY wrapper that detects a completion sentinel (proposed:
  `## GROVE: RESTART ##\n`) and restarts the agent with fresh context. Accepts
  the spike's "re-grows a little shed terminal plumbing" cost. Well-motivated:
  the agent cannot cleanly self-exit an interactive session, so
  detect-and-kill-from-outside is the right shape (the lighter
  respawn-on-clean-exit shell loop is unavailable for that reason). *Open
  refinement:* sentinel detection inside the Claude Code TUI byte-stream is the
  spike's flagged reliability risk — revisit stream-scan vs. sentinel-file vs. a
  `grove-llm` IPC signal.
- **Distribution preference → 100%-skill with no separate Homebrew; OR Homebrew
  as the *sole* install step.** The unacceptable shape is two gestures (a skill
  install *and* a separate brew). Refines D8a. Realization: `grove-llm`'s
  agent-facing verbs can ship in a plugin's `bin/` (on the Bash-tool PATH), so
  the *agent* needs no Homebrew; the human-launched **wrapper** is the only piece
  that forces the install question.

**D1 — Substrate = the self-driving outer loop (PTY wrapper). Settled 2026-06-20.**
Corner ② of the trilemma: automated cranking + fresh context, at the cost of one
external process. The user **explicitly removed the manual crank** — reaffirming
D7 (the engine turns the crank, not the human). Rejected ① (pure `/grove`, manual
`/clear`+`/grove`) and ③ (`/loop /grove`, which sacrifices the fresh-context
invariant). The loop driver is an outer process that spawns a fresh interactive
`claude` per task, passes keystrokes through transparently (so grilling is native
during a task), detects a completion signal, and kills+respawns with fresh context
until `grove-llm pick` is empty. Built in Rust reusing `portable-pty` (already a
grove dependency) for the gate-D-critical transparent passthrough.

**D2 — Distribution = Homebrew as the sole install step. Settled 2026-06-20.**
Refines D8a. `brew install grove` installs one binary (the loop driver + the
tree verbs) **and provisions the global skill** into `~/.claude/skills/grove/`.
No separate skill install, no shell-PATH fiddling, no per-repo
materialise/`VERSION.md` drift. Recommended provisioning mechanism: the binary
embeds the skill content (as today) and idempotently extracts it to the global
skills dir on launch, so the skill always matches the installed binary — which
**dissolves the cli/repo/worktree drift model** (D3's intent) rather than porting
it. (User recalled a skill distributed purely through Homebrew as the model.)
Exact provisioning wiring (formula post-install vs binary-on-launch extract) →
the distribution leaf.

**Fast-following implications (provisional — flagged for confirmation):** fresh
context = kill+respawn a new `claude` process (not in-process `/clear`); the
wrapper is transparent *during* a task so grilling "just works"; `grove do <name>`
(or `grove <name>`) becomes the **whole-loop** launcher, not a per-task one; the
per-grove worktree model (`.grove-worktrees/<name>/`) is unchanged (the loop runs
many sequential fresh sessions in the one worktree).

**D3 — Signal = an out-of-band CLI command, NOT PTY-content pattern-matching.
Settled 2026-06-20.** The user rejected scanning the pty stream for a sentinel
(too fragile inside Claude Code's TUI redraws). Instead the agent, as its last
step (after commit + retire), runs an explicit `grove-llm` signalling verb via
Bash; that command communicates out-of-band to the wrapper (transport TBD —
socket / FIFO / file / signal-to-PID, the wrapper handing the child a handle via
env var). Architecture this yields: the **signal demarcates task-done**, and
**`grove-llm pick` is the loop condition** (wrapper checks after each reset → leaf
respawns, empty stops). The wrapper holds zero grove state and constructs no
prompts — D6's stateless self-locating body, so restart ≡ continuation by
construction. Robustness add: the wrapper should also reset on an unexpected child
*exit* (crash → re-run re-derives from `pick`; the unretired leaf is redone).
*(Mechanism realization superseded by D4 — no PTY wrapper.)*

**Remaining wiring-leaf detail (NOT settled here):** the exact IPC transport, and
the slimmed PoC — transparent `portable-pty` passthrough surviving a TUI session
(resize + Ctrl-C intact) + the env-handle reaching the agent's Bash tool +
clean kill/respawn on signal. The hardest former risk (TUI-stream sentinel
detection) is **dissolved** by D3.

**D4 — Mechanism refined: NO pty wrapper; native foreground `claude` + an
out-of-band kill. Settled 2026-06-20.** Because the completion signal is
out-of-band (D3), nothing needs to sit in the human↔claude I/O path, so PTY
wrapping is unnecessary. Mechanism: a trivial **loop driver** (`while grove-llm
pick has work: launch claude; relaunch`) runs `claude` as a **normal foreground
child** owning the real TTY — grilling / resize / Ctrl-C all 100% native, ZERO
passthrough code. The agent, as its last step, runs a `grove-llm` signal verb; an
**out-of-band kill** then ends the claude session (the external "exit" the agent
can't perform itself); on exit the loop driver consults `grove-llm pick` →
relaunch or stop. **Supersedes D1's PTY-wrapper realization and D3's passthrough
PoC; dissolves the gate-D passthrough risk and the `portable-pty` dependency.**

Two candidate kill-realizations (→ wiring-leaf detail + tiny PoC; both avoid PTY
wrapping):
- **(a) generic file-watch daemon** — a long-lived, grove-agnostic watcher
  (existing `fswatch`/`watchexec`, or a tiny utility) kills claude when the signal
  file appears. Event-driven/prompt; needs a daemon lifecycle + a pid file.
- **(b) self-spawned delayed killer** *(user, 2026-06-20)* — the signal verb
  spawns a **detached** helper that waits a short grace then kills the claude
  session, and returns immediately (the grace lets the verb return before its own
  session dies). No daemon, no persistent process, no file to clean up; targets
  claude via a loop-driver-exported `GROVE_CLAUDE_PID` (not ancestry-guessing);
  short grace + SIGTERM (SIGKILL fallback) rather than a fixed 10 s.
  **Lean: (b)** — fewest moving parts ("least in grove"), and being spawned
  *inside* claude it sidesteps the daemon's pid-coordination.

Other wiring details: terminal reset after kill (`stty sane` / `tput rmcup`); a
signal file (if used) also disambiguates task-done→relaunch vs human-quit→stop.
New, much smaller PoC: foreground claude in a shell loop + the chosen kill + clean
relaunch, grilling intact.

**Consequence for D2 (flagged, pending):** dropping `portable-pty` removes the
binary's strongest justification. D2 (Homebrew + Rust `grove-llm`) still stands —
the tree-verb logic (dotted-decimal renumber/insert) still justifies tested
compiled code, and Homebrew cleanly pulls in any file-watcher dependency +
provisions the global skill — but pure-scripts / no-Homebrew is reopened.

**D5 — Backwards-compat = transitional bridge + one-time migration, then drop
old-format reading. Settled 2026-06-20.** (Option ii.) The new skill + `grove-llm`
read BOTH the old `NNN-slug/` directory format and the new flat dotted-decimal
format *during a transition* — so in-flight groves (including this dogfood tree)
keep working with zero disruption — plus a one-time `grove migrate` (old→new).
Old-format reading is then **dropped** once trees convert. Rationale: grove trees
are ephemeral (each finishes and deletes its `.grove/`), so the old-format
population drains on its own; permanent dual-format would pay a forever-tax for a
transient problem. Refines D8a (which assumed permanent dual-read).

**OPEN ISSUE (recorded per user; resolve in the substrate-wiring leaf) — loop
interrupt / stop semantics.** How does the human interrupt the loop so it does
NOT relaunch on the next stop? The user has hit this failure mode in prior
loop-driven setups (Ctrl-C kills the child but the loop immediately respawns →
trapped). Candidate resolution: make **auto-relaunch conditional on an explicit
task-completion signal** (D3/D4) — relaunch ONLY when the agent ran the completion
verb; ANY other exit (human `/exit` or Ctrl-C, or a crash) **stops** the loop,
resumable later by re-running `grove do <name>` (restart ≡ continuation, D6).
Corollary: the loop driver must itself survive the human's interrupt
(trap/ignore SIGINT) so it reaches the relaunch-vs-stop decision rather than being
killed or blindly looping. Distinguishing human-quit from crash is unnecessary
under this design — both stop and both resume identically.

**D6 — Retain grove's distinctive methodology; "less in grove" = less MACHINERY,
not less wisdom. Settled 2026-06-20.** Walks back D2/D3's "shed methodology →
third-party skills" plank. The prompts/methodology (grilling, the `driving.md`
habits, the loop discipline, the CONTEXT/ADR/PRD format guides) have proven
successful and are superior in many ways to third-party collections (e.g.
addyosmani/agent-skills — which `driving.md` already borrows two sections *from*,
in grove's voice, with attribution). They are grove's differentiating
intellectual core, nearly free to carry (markdown), and must be **RETAINED** —
bundled in the global skill (as today), not deleted for inferior externals. The
shedding the refactor performs is **machinery only** (TUI, inbox/grove-meta,
install/materialise). Generic engineering practices grove never had superior
versions of (TDD/review/debugging) continue to defer to mature third-party skills
(e.g. superpowers) — unchanged, and the only legitimate "shed to third-party."
**Sequencing (user): remove little/none of the prompts _initially_** — any later
methodology pruning/modularization is a separate, careful, deferred question,
explicitly OUT of this refactor's initial scope. Refines ADR-0031's thesis:
extract the self-extension core **and keep the methodology**; shed the machinery.

## Anticipated implementation leaves (grown after the substrate firms)

- dotted-decimal numbering + verb changes (D4/D5)
- global-skill + `grove-llm` distribution with backwards-compat (D8a)
- shed-the-TUI
- shed inbox/grove-meta + install/materialise machinery (D3)
- the substrate wiring (the PTY loop driver itself)
- migration of existing `NNN-slug/` trees
