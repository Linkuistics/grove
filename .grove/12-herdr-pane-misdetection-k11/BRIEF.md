# herdr-pane-misdetection-k11 — brief

## Goal

Stop herdr labelling a `grove do` pane with the wrong agent, so its screen
manifests evaluate the harness grove actually launched. The route is **settled**:
grove sets herdr's documented **`HERDR_AGENT=<harness>`** environment hint on the
harness child it spawns. No fork hunk, no process-group surgery.

## Done when

- Every harness grove launches carries `HERDR_AGENT=<harness name>`, at **both**
  launch sites (`loop_driver::launch_session` for `grove do`, `launch::exec_harness`
  for `grove retire`).
- Observed live: a `grove do` pane reads the launched harness — not `codex` —
  where detection is what the pane shows.
- ADR *herdr-optional-ui*'s "what to do about that is undecided and out of this
  ADR's scope" paragraph, and `CONTEXT.md`'s **Pane mis-detection** entry, are
  reworked in place to record the answer.

## Decomposition

Two leaves, named off a shared stem (*Review chain* / vendor-pair convention,
`compose-task-chains-k29`). Not a review chain: the change is one line at two
call sites, which the guidance says to subagent, not to chain.

- **agent-hint-k33** — set the hint. Small, and the only thing that must land
  before anything can be observed.
- **agent-hint-observe-k34** — the measurement, plus the durable record. Separate
  because the change lives in the **driver**, and a session is the driver's
  grandchild: it cannot watch its own replacement (the same reason
  `ship-release-k25` preceded `observe-live-surface-k26`). Unlike those, this one
  needs **no release** — the observer drives a throwaway pane from
  `./target/debug/grove`.

## Why this route, and what was rejected

The leaf that became this brief listed three candidates and pre-selected none.
Measurement found a **fourth**, and it dominates all three.

**`HERDR_AGENT=<agent>` is a documented, public herdr extension point**, added for
precisely this shape — herdr's own `agents.mdx`: *"a host-visible wrapper can hide
the real agent process from Herdr. Set `HERDR_AGENT=<agent>` on the wrapper command
to tell Herdr which existing agent screen manifest to use."* `grove do` **is** a
host-visible wrapper. Details in Notes.

- **Restructure the process group** (`setpgid` the harness into its own group and
  `tcsetpgrp` it to the foreground) — the obvious move, and it would work:
  detection reads the pane shell's `e_tpgid`, so the harness would become the
  leader and win outright. **Rejected**: it buys the same outcome by rewriting the
  driver's job-control topology — SIGINT delivery (the driver's `SIG_IGN` survival
  is load-bearing, *self-driving-loop*), SIGTERM/SIGHUP release
  (*herdr-optional-ui*'s table), `tcsetpgrp` restore on child exit, and `SIGTTOU`
  around the call. That is a signal-contract change to buy what one environment
  variable already buys. Reopen only if the hint is ever withdrawn upstream.
- **A third hunk on the fork's `identify_agent_in_job` fallback.** **Rejected**:
  the carry is permanent and held to one principle (*herdr-optional-ui*), so every
  hunk is a rebase obligation forever — and a fix that needs no fork beats one that
  does, outright. It would also fix only *our* build.
- **Accept the mis-label as known noise.** **Rejected**: legitimate only while the
  fix was expensive. It is one line.

The principle the answer encodes, and the sentence for the ADR: **grove reports
what it *is* (`grove`); it hints what it *launched* (`claude` / `codex` / `pi`).**
The two are different fields serving different jobs — the report carries semantic
state grove alone knows, the hint selects which screen manifest herdr parses the
TUI with — and keeping them distinct is what lets both be honest at once.

## Pointers

- ADR *herdr-optional-ui* — its "the `fallback_state` underneath is **not reliable
  for grove panes**" passage is the paragraph this node closes.
- ADR *self-driving-loop* — why the rejected process-group route is expensive.
- `docs/specs/herdr-fork-maintenance.md` — untouched by this node, deliberately:
  no fork change means no rebase cost added.
- Glossary: **Pane mis-detection**, **herdr integration** (`CONTEXT.md`).
- herdr source at `~/Development/herdr`; detection is `src/detect/mod.rs` +
  `src/pane.rs`, the hint reader is `platform::parse_agent_env_hint`.

## Notes

Measured 2026-07-28 against the fork at `ui-layout` = `d17e0f42` — which is the
revision the **installed** `0.7.5-linkuistics.1` was built from (Homebrew's
`INSTALL_RECEIPT.json`), so these hold for the herdr actually running.

- **The hint is upstream, documented, and released.** `HERDR_AGENT=<agent>` landed
  for Linux in 0.7.1 and for macOS in `947328fa` (2026-07-16, tagged `v0.7.5`,
  upstream discussion #679). Read from a process's **exec-time** environment —
  `kern_procargs2` on macOS, `/proc/<pid>/environ` on Linux — and parsed by
  `platform::parse_agent_env_hint`.
- **grove's three harness names are herdr's three canonical labels**, exactly:
  `claude`, `codex`, `pi` all appear in `detect::lookup_agent`. No translation
  table, and a harness herdr does not know simply parses to nothing and degrades
  to today's behaviour.
- **The hint must go on the child, and that is better than the wrapper.** herdr's
  doc says to set it "on the wrapper command", i.e. on `grove` — but grove cannot
  change its own exec-time environment, and it does not need to:
  `pane::probe_foreground_process_from_jobs` consults
  `agent_hint_for_non_leader_foreground_job_members` — every non-leader member of
  the foreground process group — **before** falling through to the group scoring
  that today elects `codex mcp-server`. Setting it per-launch is also *more*
  accurate than a wrapper-level export, because grove's harness varies per leaf
  (*Kind routing*).
- **The leader preference is now doubly reinforced**, wider than the leaf recorded:
  `probe_foreground_process` first builds a **leader-only** job and tries hint then
  name on it, and only then reads the full group — where the order is leader-hint,
  leader-name, **non-leader hint**, group scoring. grove is the leader and is
  unidentifiable, so both leader steps miss and the non-leader hint is the first
  step that can fire.
- **The route adds no risk to the reports already landing.** A screen-detected
  blocker can override a hook report only when
  `parse_agent_label(authority.agent_label) == detected_agent`
  (`state.rs::visible_blocker_overrides_hook`). grove's label is `grove`, which
  parses to `None`, so the equality can hold only if `detected_agent` is `None` —
  and a `None` agent evaluates no manifest, so it can never raise the visible
  blocker the override needs. Changing `detected_agent` from `Some(Codex)` to
  `Some(Claude)` moves nothing in that gate.
- **`strings`/`grep` over a binary cannot refute this feature's presence.**
  `b"HERDR_AGENT="` is a compile-time `strip_prefix` operand, which LLVM lowers to
  immediate byte compares — the literal is absent from the installed binary that
  provably contains the code. The install receipt's pinned revision is the sound
  check; the `strings` habit that worked for grove's own `report-turn` does not
  transfer.
- **Where the mis-label actually bites, unchanged:** only where grove is not
  holding authority — before its first report, and after it releases at
  `complete --done`. A landed report takes precedence over detection, which is why
  `herdr pane get` on a live grove pane shows `agent: "grove"` and hides the bug.
