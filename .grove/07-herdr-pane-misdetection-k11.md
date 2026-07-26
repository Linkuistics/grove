# herdr-pane-misdetection-k11

**Kind:** planning

## Goal

Decide how grove stops its own panes being mis-labelled by herdr, now that
upstreaming is off the table (`herdr-upstream-pr-k10` was pruned; see ADR
*herdr-optional-ui*). Every `grove do` pane is currently detected as **codex**
whatever harness it launched, so herdr's screen manifests evaluate the wrong
agent — and that is the state the pane falls back to whenever grove is not
holding hook authority.

## Context

**The mechanism, measured this session** — and it is *not* what the root brief
said. That note claimed "MCP servers inherit the harness's foreground process
group, so a `codex` MCP server running under `claude` makes herdr identify the
pane as codex." True as far as it goes, but it misses why herdr's existing
defence does not fire.

The pane's foreground process group, read live from `wQ:p1`:

```
15007  grove do --harness claude     ← process-group LEADER
33470  └─ claude …
33496     ├─ codebase-memory-mcp
33497     ├─ npm exec @upstash/context7-mcp
33498     ├─ npm exec @playwright/mcp@latest
33505     ├─ codex mcp-server         ← wins detection
33579     ├─ node …/playwright-mcp
33597     └─ node …/context7-mcp
```

`identify_agent_in_job` (`src/detect/mod.rs:210` on upstream HEAD) does two
things in order:

1. If a process's pid equals `job.process_group_id`, and its normalised name
   identifies an agent, **return that** — the leader wins outright.
2. Otherwise fall back to scoring *every* process in the group by
   `process_priority` (`src/detect/mod.rs:581`), first-best-wins.

Step 1 exists because of upstream issue **#161** ("Agent detection can switch
from the launched agent to a child MCP process", filed 2026-05-18, fixed in
v0.5.11) — which is otherwise a near-exact description of our symptom, in the
same claude+codex-MCP shape. It does not save us because **grove is the
leader**, and `identify_agent("grove")` is `None`, so detection drops straight
to the fallback and a `codex mcp-server` in the group outscores or outranks the
real `claude`.

So this is grove-shaped as much as herdr-shaped: *grove being the pane's
foreground process-group leader is exactly what disables the fix that would
otherwise protect it.* Any grove pane running any harness alongside a
`codex`-named MCP server hits this.

**What is already settled and must not be reopened here:**

- The state *reporting* path is done and works (`report-plumbing-k8`), and
  while grove holds hook authority its report takes precedence over detection.
  Mis-detection therefore bites in the gaps: before grove's first report, and
  after grove releases at `complete --done`. It also drives the pane's displayed
  agent label whenever grove is not the authority.
- grove reports as agent **`grove`** by design (ADR *herdr-optional-ui*). This
  leaf is not about the reported label; it is about what herdr *detects*.
- Upstreaming is closed. Whatever this leaf chooses lands on our fork, in grove,
  or nowhere.

## Done when

The route is settled and recorded, and the tree grown to match. Candidate
routes to grill, none pre-selected:

- **grove-side**: change how `grove do` launches the harness so the pane's
  foreground process-group leader is the harness itself rather than `grove`
  (a `setpgid` on the child, or some equivalent), restoring #161's leader
  preference for free. Cheapest if it works, and it fixes every herdr, patched
  or stock — but the driver must stay the parent that survives Ctrl-C and reaches
  the relaunch-vs-stop decision (ADR *self-driving-loop*), so verify that first.
- **fork-side**: a third hunk on the fork's `identify_agent_in_job` fallback —
  e.g. prefer a direct child of the leader over deeper descendants. Note the cost
  this now carries: the fork is a **permanent** carry, so every hunk is a
  recurring rebase forever, and *herdr-optional-ui* deliberately holds the patch
  to a single principle.
- **accept it**: record the mis-label as known noise and stop. Legitimate if the
  reporting path already covers the cases anyone looks at.

If the answer is "do nothing", prune this leaf rather than writing a no-op one.

## Notes

**Re-verify before building on any of this.** The measurements above are from
one herdr instance (`0.7.5-linkuistics.1`, server process older than the
install) and from upstream `state.rs`/`detect/mod.rs` at
`47104169`. herdr moves fast, and the root brief's version of this same finding
was already stale by the time it was read.

**Scope guard**: intra-session turn boundaries are `04-herdr-turn-hooks-k4`.
This leaf is only about which *agent* herdr thinks a grove pane is running.
