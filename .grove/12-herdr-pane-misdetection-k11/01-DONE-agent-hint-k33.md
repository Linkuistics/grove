# agent-hint-k33

**Kind:** impl

## Goal

Set herdr's documented `HERDR_AGENT=<harness name>` environment hint on every
harness process grove spawns, so herdr detects the harness rather than scoring the
process group and electing a `codex mcp-server` helper.

## Context

The brief carries the route and why the three alternatives lost; do not re-open
them. What this leaf needs beyond the brief:

- **Two launch sites, both real.** `loop_driver::launch_session` (the `grove do`
  loop) and `launch::exec_harness` (reached by `grove retire`). Unlike the turn
  hooks — which are deliberately driver-only, because a `retire` session sets no
  signal file and every turn end would read `blocked` — the hint has **no such
  asymmetry**: a `grove retire` pane is mis-detected exactly as a `grove do` pane
  is, and the hint carries no discriminator. Put it in both, and say why in a
  comment, since the neighbouring `append_turn_hooks` comment says the opposite
  about itself.
- The natural home in the driver is beside the existing
  `cmd.env("GROVE_SIGNAL_FILE", …)` / `env_remove` block.
- `harness.name` is already exactly herdr's canonical label for all three
  harnesses (`claude`, `codex`, `pi` — `detect::lookup_agent`), so this is
  `harness.name`, not a mapping table.

## Done when

- Both launch sites set `HERDR_AGENT` to the launched harness's name.
- Tests pin it. The launch command is a payload another program reads, so assert
  it rather than trust it — `launch.rs`'s existing tests already do this for the
  injected `--settings`, and are the pattern to follow.
- `cargo test` and `cargo clippy` are clean, and the CHANGELOG's unreleased entry
  gains a line.

## Notes

**One design call this leaf must make and record: gate the hint on
`herdr::in_pane()`, or set it unconditionally?**

The recommendation is **gate it**, for symmetry with the turn hooks — grove's
standing discipline is that outside a herdr pane its launch is byte-identical to a
grove with no herdr integration at all, and herdr's own docs warn against letting
the variable spread further than the process it describes. The counter-argument is
that the variable is inert with no herdr running and the gate is a branch to keep
correct forever. Decide, implement one, and put the reason in the code comment —
this is the kind of small asymmetry a later session will otherwise re-litigate.

Two things **not** to do, both already settled in the brief:

- Do not set the hint on `grove` itself (re-exec, wrapper script, or otherwise).
  grove cannot change its own exec-time environment, and the child path is both
  sufficient and more accurate per leaf.
- Do not touch the fork. The whole point of this route is that it costs no hunk.

Verifying this leaf's *own* work stops at the process environment — that grove
launches the harness with the variable set. Whether herdr then labels the pane
correctly is **agent-hint-observe-k34**, and it needs a driver this session cannot
be running under.
