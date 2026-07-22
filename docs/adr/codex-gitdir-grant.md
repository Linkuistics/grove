# codex sessions launch with the gitdir granted via `--add-dir`

codex's `workspace-write` sandbox carves the repository gitdir out read-only —
so `git commit`, and with it grove's mandatory Commit and Retire steps, fail
inside a codex session. grove's codex launch path therefore derives
`git rev-parse --git-common-dir` (absolutized) for the working tree and appends
`--add-dir <path>` to the codex command line. One path suffices in both repo
shapes: a linked worktree's gitdir (`<common>/.git/worktrees/<name>`) is a
subpath of the common dir, and in a plain checkout `--git-common-dir` *is*
`.git`. The flag is passed unconditionally for codex launches (harmless when
the sandbox is off) and no other harness is touched.

Three facts make this the mechanism (verified against codex-cli 0.144.5 source
at `rust-v0.144.5` and live `codex exec` probes in a scratch linked worktree,
2026-07-22):

- **Grants are additive.** `--add-dir` / `sandbox_workspace_write.writable_roots`
  *add to* the default writable roots (cwd, `/tmp`, `$TMPDIR`) — the policy
  builder (`get_writable_roots_with_cwd`) always appends the defaults. The
  loop's own signal write to `$TMPDIR` stays safe alongside the grant.
- **The `.git` carve-out is per-rule, not a global deny.** Each writable root's
  seatbelt allow-rule excludes its own `.git` via `require-not` — codex even
  resolves a worktree's `.git` pointer file and carves out the real gitdir. But
  seatbelt allow-rules compose: a separate explicit root gets its own clean
  allow, which is why the grant reopens the gitdir without touching the
  defaults.
- **Probe trap: the `codex sandbox` subcommand does not model the session
  sandbox.** Since at least 0.144.5 it requires `--permission-profile` and
  exercises codex's new named-profile machinery, not the legacy
  `sandbox_workspace_write` policy the TUI/exec path uses. Probing it gave a
  false replace-not-add reading that this decision's earlier drafts nearly
  encoded. Settle sandbox questions against `codex exec` (same policy builder
  as the TUI), never the subcommand.

## Considered options

- **A user-owned `~/.codex/config.toml` `writable_roots` grant** — rejected:
  the path is per-working-tree, so a static global config must be re-derived by
  hand for every new grove, and it widens *every* codex session in that
  checkout, not just grove's. Reopened if codex removes `--add-dir` (present
  on both the TUI and `codex exec` at 0.144.5).
- **A launch pre-flight probing gitdir writability** — rejected: with the flag
  structural, the failure mode it would detect no longer exists; a codex old
  enough to reject the flag fails loudly at spawn, which is diagnostic enough.
  Reopened if unexplained codex launch failures surface in the field.

## Consequences

- The sandboxed agent gains write access to the entire common gitdir —
  **including `.git/hooks/`**, which is precisely what codex's carve-out
  defends (a hook written in-sandbox later fires outside it, e.g. on the
  user's own commit in the main checkout). Accepted deliberately: letting an
  agent commit *is* extending that trust, and it is the same trust already
  extended to the unsandboxed harnesses (pi, claude's default posture).
- grove sessions on codex are committable hands-free; the codex half of the
  harness trial runs the full work → commit → retire → signal cycle without
  manual sandbox configuration.
