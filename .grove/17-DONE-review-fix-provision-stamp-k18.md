# review-fix-provision-stamp-k18

**Kind:** work

## Goal
Fix the write-ordering and blast-radius defects in stamping and provisioning —
the ones that can leave a user wedged, permanently mis-bound, or (B10) with a
stray `kill -TERM -1`.

## Context
Findings B3, B4, B8, B9, B10, D5 in `.grove/14-DONE-branch-review-k14.md`.

Three of these are the same mistake — committing a durable effect before the
thing that could invalidate it has run:

- **B3** `maybe_stamp` (src/launch.rs:25) runs before the `no_launch` return
  (`:47`), so a documented dry run permanently rebinds the grove.
- **B4** the stamp is written before `provision_all` (`:30`), which can bail —
  leaving a permanent bad binding with no verb to clear it, and no message
  naming the stamp path.
- **B8** `sync_to_stamp` writes files (src/provision.rs:133) before the stamp
  (`:134`), so an interrupted extract leaves a state indistinguishable from a
  foreign dir and wedges every later `grove do`. This is a regression — the
  pre-change path self-healed. Extract to a sibling temp dir and `rename`.

Plus:

- **B9** `provision_all` propagates with `?` (src/provision.rs:51), so a
  foreign dir under *any* harness aborts the sweep and blocks the launch of an
  unrelated primary — after already rewriting earlier targets, unreported.
- **B10** `src/complete.rs:98-101` accepts a negative PID that reaches
  `kill -TERM {pid}` (`:163-165`). One-line guard: `.filter(|p| *p > 0)`.
- **D5** `.grove-stamps/` is not in `.gitignore`, and runbook step 6 mandates
  creating it in every migrated repo.

## Done when
- B3, B4, B8, B9, B10, D5 fixed; each has a test that fails without the fix
  (B8's is the interrupt case — simulate by planting a non-empty unstamped dir
  grove itself would have produced).
- A user who hits a bad stamp has a stated recovery path (a message naming the
  file is enough; a verb is a judgement call — raise it, don't assume it).
- `cargo test`, `fmt`, `clippy` clean.
- One focused commit.

## Notes
- B9 needs a decision, not just a fix: provision the primary first and warn on
  non-primary failures, or keep hard-bail everywhere? The contract requires
  bailing on a foreign dir but says nothing about one harness blocking
  another. Record the choice.
- Related items parked under "decision" in k14 that touch this same code —
  raise if cheap: the primary creating its own harness root against the
  "never create" comment; `$CODEX_HOME`/`$CLAUDE_CONFIG_DIR` ignored; the
  user's symlink farm replaced without logging the target; `HOME=""` yielding
  relative paths; non-atomic stamp write; bare io error on an unreadable stamp.
- The user's live `~/.codex/skills/grove` and `~/.pi/agent/skills/grove` are
  currently symlinks to the claude copy. The first `grove do` after release
  replaces both with independent snapshots — intended, but verify it happens
  in the right order and says so out loud.
