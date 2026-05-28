# 030-writes-and-fs-watch

**Kind:** work

## Goal

Close out v1: add the one write action (`c` → `grove inbox add`) and
the filesystem-watch refresh. After this leaf, the TUI is interactive
(dogfooding capture is one keystroke away) and self-updating (no
manual refresh needed). v1 ships.

## Context

Parent BRIEF settled:

- Write actions: `c` only — shells out to `grove inbox add`, prompting
  for target grove (which may be the current one, another grove in
  the repo, or a fresh seed name). No direct file edits ever.
- Filesystem-watch: `notify` crate, polled from the sync event loop,
  200ms debounce.

Leaves 010 and 020 are prerequisites: the data layer exists and can
re-scan cheaply, and the TUI shell renders off it.

The write action's UX: pressing `c` opens a modal that takes (a) the
target grove name (default: currently-selected grove on detail screen;
default: empty on list screen — let the user type), and (b) the
observation body in a multi-line editor (drop to `$EDITOR` if it's set
and the body is non-trivial; otherwise an inline Ratatui input is
fine). On submit, the TUI suspends terminal raw-mode, runs
`grove inbox add --to=<name> --body-file=<tempfile>` (or stdin),
prints any stderr to a status line on return, and resumes the TUI.

Filesystem-watch: register `notify` watchers on `.grove/` (recursive)
for every grove in scope, plus
`<repo>/.grove-meta/inboxes/<name>/` for the inboxes. On any event,
schedule a re-scan; coalesce events within a 200ms window so a single
git operation doesn't trigger N re-scans. The sync event loop polls
both crossterm events and the notify channel — `crossbeam-channel` or
`mpsc` plus a short poll timeout is the standard pattern; pick
whichever the rest of the codebase already uses.

## Done when

- `c` keybind works on both screens. From the detail screen it
  defaults to the current grove; from the list screen it prompts.
  Pressing it opens the capture modal, accepts a body, shells to
  `grove inbox add`, and on success the inbox count for the target
  grove updates (via fs-watch — verifies the round trip).
- The shell-out cleanly suspends and restores terminal state
  (Ratatui has helpers for this; use them — don't roll bespoke
  alternate-screen toggling).
- Filesystem-watch driven refresh: editing a leaf file in another
  terminal, or running `grove start <name>` against a seed, causes
  the TUI to reflect the change within a second or so.
- Debounce holds: a `git checkout` that touches many files produces
  at most one re-scan, not dozens.
- The TUI exits cleanly on `q` even with watchers active — no
  leaked threads, no zombie file handles.
- A short walkthrough in the commit message demonstrates the round
  trip (capture from TUI → inbox count updates without manual
  refresh) and the debounce behaviour.

## Notes

- **The capture modal is the dogfooding moment** the parent grove was
  built for. Whoever implements this leaf should themselves capture
  *at least one* seed from the TUI during development, to feel the
  loop.
- **Body input:** dropping to `$EDITOR` is the right answer for
  anything beyond a one-liner. Ratatui's inline text input is fine
  for short observations; gate the `$EDITOR` drop on a second
  keybind inside the modal (e.g. Ctrl+E) rather than auto-deciding
  on body length.
- **Watcher scope is per-repo.** The TUI watches everything under
  `<repo>/.grove-worktrees/*/.grove/` and `<repo>/.grove-meta/inboxes/`.
  The future multi-repo grove (per the seed) extends this to a
  watcher set per repo; the v1 layering should make that additive,
  not a refactor — keep the watcher-registration code grouped so it
  can be lifted into a per-repo helper.
- **`notify` backends differ by platform.** macOS `FSEvents`,
  Linux `inotify`, Windows `ReadDirectoryChangesW`. Default
  `RecommendedWatcher` is the right choice; if a platform behaves
  badly under it, that's a follow-up, not a v1 blocker.
- **No retry on capture failure** in v1. If `grove inbox add` exits
  non-zero, surface the stderr and let the user re-press `c`.
  Auto-retry is the future grove's problem.
