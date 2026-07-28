# codex-grant-refused-k35

**Kind:** impl

## Goal

A `grove do` pane launching **codex** dies at startup because codex refuses the
`--add-dir` VCS-store grants grove passes it, and the loop stops. Make a codex
launch work — or fail diagnostically — when the effective permission profile is
not `workspace-write`.

## The report

Observed by the human 2026-07-28 in the `UIAnyware` grove (a **colocated jj**
tree — both `.jj` and `.git` were granted, matching *codex-gitdir-grant*'s rule):

```
grove: launching codex (model: sol-xhigh)
Error adding directories: Ignoring --add-dir (/Users/antony/Development/UIAnyware/.jj,
  /Users/antony/Development/UIAnyware/.git) because the effective permissions do not
  allow additional writable roots. Switch to workspace-write or danger-full-access
  to allow them.
grove: session ended without a completion signal — loop stopped.
       Re-run `grove do` from this working tree to resume (restart ≡ continuation).
```

## Context

This **fires an existing reopen condition verbatim.** ADR *codex-gitdir-grant*
rejected a launch pre-flight probing store writability, "Reopened if unexplained
codex launch failures surface in the field." One has.

The ADR's load-bearing assumption is the sentence to attack first: *"The flags are
passed unconditionally for codex launches (harmless when the sandbox is off)."*
That was verified against 0.145.0 for the sandbox-**off** case and the
`workspace-write` case. This report is a third case neither covered — a profile
that is neither off nor `workspace-write` (codex's message names the two it would
accept, so the effective one is something else, most likely `read-only` or a named
`--permission-profile`). Establish what the effective profile actually was before
theorising: it may come from `~/.codex/config.toml`, a profile grove's model
string (`sol-xhigh`) selects, or a codex version newer than the ADR's 0.145.0.

Two things are worth separating, because they may have different causes:

1. **Is the `--add-dir` message fatal, or a warning grove's loop mistook for
   death?** The text says "Ignoring", which reads like a warning — yet the session
   ended with no completion signal. If codex exited non-zero on it, that is one
   bug; if codex carried on and the session died for an unrelated reason, the
   grant message is a red herring and the real fault is elsewhere. `grove do
   --no-launch` plus a hand-run of the exact argv is the cheapest way to tell.
2. **Even granted correctly, would the session have been committable?** Under a
   read-only profile it would not — grove's Commit and Retire steps are mandatory,
   so a silently-degraded launch is worse than a refused one.

## Done when

- The effective codex permission profile in the failing case is identified, and
  whether the `--add-dir` refusal is fatal or incidental is settled by
  measurement, not inference.
- A codex `grove do` launch in that configuration either succeeds, or fails
  **before** spawning with a message naming what to change. The loop must not
  stop on a mute non-signal exit.
- ADR *codex-gitdir-grant* is reworked **in place**: its "harmless when the
  sandbox is off" claim is corrected to whatever the third case turns out to
  require, and the pre-flight-probe option's disposition is updated now that its
  reopen condition has fired. Never append a superseding record
  (`linkuistics:decision-records`).

## Notes

Reproduce against the real tree if the human still has it, but **do not assume
their config is the general case** — the fix has to hold for anyone whose codex
defaults differ. `docs/adr/codex-gitdir-grant.md` records the probe method that
worked last time, including its trap: settle sandbox questions against
`codex exec`, never the `codex sandbox` subcommand, which models a different
policy path.

The grant is built by `launch::append_codex_vcs_store_grant`; both launch sites
call it. Check the installed codex version against the ADR's 0.145.0 first — a
behaviour change upstream is the cheapest explanation and the easiest to confirm.

## Work already in the tree, and one defect in it (found by guard-loop-signal-k37)

**A previous session got most of the way and was killed before committing.** Its
work survives uncommitted in the working copy (jj snapshots it): `src/launch.rs`
(+222), `src/loop_driver.rs` (+33), `tests/launch.rs`. Read it before restarting
from scratch — it already establishes the answer the Goal asks for, and states it
well:

> codex's effective sandbox is `read-only` for any project the user has not
> **trusted**, and trust is per-directory with **no inheritance from parent
> directories** — so a brand-new working tree, which is exactly what `grove do`
> bootstraps into, is untrusted by construction. Under `read-only`, `--add-dir`
> is refused **fatally**: codex exits 1 in ~0ms, before drawing any TUI.

So the ADR's third case is the *default* case, not an exotic one, and the "mute
non-signal exit" in the report is that 0ms exit. It adds
`launch::check_codex_sandbox_accepts_grants`, a pre-flight called from both the
loop and `readiness()`, which refuses rather than elevating or degrading.

**It is not finished, and it has one defect that must be fixed before it lands:**

- **8 loop_driver tests fail** — all codex-launch tests
  (`codex_launches_with_no_name_flag_and_a_model_flag`,
  `review_leaf_reroutes_to_the_review_harness`, the four reroute tests,
  `a_leaf_declaration_beats_the_per_kind_policy`,
  `a_leaf_declared_harness_launches_there_whatever_the_stamp`). The pre-flight
  invokes the harness *binary*, which under test is a fake shell script, so every
  codex launch test now spawns it twice with the probe's argv.
- **The pre-flight spawns a harness process without scrubbing the loop's control
  environment** (`probe_codex_sandbox`, `src/launch.rs:334-358`). `launch_session`
  is the *only* site that scopes `GROVE_SIGNAL_FILE`, deliberately — it sets it on
  the session child and nothing else. The probe inherits the driver's own value,
  so in this repo's suite the fake harness's `: > "$GROVE_SIGNAL_FILE"` wrote the
  **live** session's signal and the real driver killed the developer's terminal.
  That is what made `cargo test` start killing sessions, and it was pinned to
  these exact five tests by measurement (k37).

  In production the leak is latent — a real `codex exec` never writes that path —
  but the rule it breaks is the general one, and `launch_session` already scrubs
  `GROVE_HARNESS_PID` / `GROVE_CLAUDE_PID` on exactly this reasoning ("must not
  leak a stale, unrelated PID into the new harness session",
  `src/loop_driver.rs:347-353`). **Any harness spawn that is not the session
  itself must scrub the loop-control env**; consider giving that rule one helper
  rather than open-coding it at each site, since this leaf adds the second site.

k37 fixed the *suite* so this can no longer kill a session
(`.cargo/config.toml` force-override + `tests/support` scrub list +
`tests/env_hygiene.rs`), which is why the 8 failures are now safe to iterate on.
It deliberately did **not** touch `src/launch.rs` — that is this leaf's code and
this leaf's commit.
