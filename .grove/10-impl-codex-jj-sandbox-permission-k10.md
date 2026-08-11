# codex-jj-sandbox-permission-k10

## Goal

Make unattended Grove Codex sessions able to complete normal `jj describe` and
`jj new` operations in Git-backed jj workspaces.

## Context

Grove launches `review-impl` with `--sandbox workspace-write
--ask-for-approval never --add-dir ${repo}`. Codex deliberately makes Git
metadata read-only in `workspace-write`, including the shared Git object store
used by secondary jj workspaces. In this workspace, `.jj/repo` resolves to
`/Users/antony/Development/grove/.jj/repo`, while commit objects must be written
under `/Users/antony/Development/grove/.git/objects`; the latter is denied by
the active sandbox. Claude Code succeeds because Grove launches it through a
different permission system and does not impose Codex's `workspace-write`
sandbox.

## Done when

- Grove's Codex launch policy intentionally grants the Git/jj metadata writes
  required for its commit-and-retire protocol.
- A Codex-launched probe can run `jj describe` and `jj new` in both the main
  checkout and a secondary jj workspace.
- Unattended runs retain an explicit, reviewed safety boundary.

## Notes

- This is a Codex launcher/sandbox mismatch, not a jj workspace failure.
- Merely adding the repository with `--add-dir` does not override the protected
  `.git` carve-out in `workspace-write`.
