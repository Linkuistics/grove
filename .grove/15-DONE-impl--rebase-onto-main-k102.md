# rebase-onto-main-k102

## Goal

Rebase this jj workspace's grove changes onto the current `main` line so the
completed walkthrough skill and the crates newly added on `main` coexist before
the finish sentinel runs again.

## Context

- The human explicitly requested the rebase before Grove teardown.
- The stated VCS is jj; use jj for every mutation and preserve the focused task
  history.
- The completed skill is
  `plugins/linkuistics/skills/writing-code-walkthroughs/SKILL.md`.
- Linkuistics plugins do not require a Grove release: Claude Code versions them
  by repository commit SHA, while Codex/Gemini/Pi use symlinks created by
  `plugins/install.sh` from the main checkout.

## Done when

- The workspace is rebased onto the current `main` destination using jj, with
  conflicts resolved without losing either line's changes.
- The crates introduced on `main` and the completed
  `writing-code-walkthroughs` skill are both present after the rebase.
- Relevant repository checks pass in proportion to the resolved conflicts.
- The handoff reports that Codex users should update the main checkout and rerun
  `./plugins/install.sh` once because this is a newly added skill; no Grove
  binary release or plugin semver bump is required.

## Notes

Do not run the installer from this secondary jj workspace unless the human
explicitly chooses the documented `--force` testing path; doing so would repoint
all installed skill symlinks at a temporary workspace. After this leaf retires,
the live `finish-k101` sentinel should be proposed again in a fresh finish
session.
