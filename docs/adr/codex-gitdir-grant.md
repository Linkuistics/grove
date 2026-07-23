# codex sessions launch with the VCS store granted via `--add-dir`

codex's `workspace-write` sandbox blocks the VCS store writes grove's mandatory
Commit and Retire steps depend on, so grove's codex launch path appends
`--add-dir <store>` grants to the codex command line — per-VCS, because the two
VCSes fail differently:

- **git tree**: the sandbox carves `.git` out of every writable root, so
  `git commit` fails in-sandbox. The grant is the absolutized
  `git rev-parse --git-common-dir`; one path suffices in both repo shapes (a
  linked worktree's gitdir is a subpath of the common dir, and in a plain
  checkout `--git-common-dir` *is* `.git`).
- **jj-enabled tree**: `.jj` is *not* carved out — but a secondary
  `jj workspace`'s own `.jj/` holds only the working copy, while every op
  lands in the *main* workspace's `.jj/repo`, outside the sandbox cwd
  entirely. The grant is the main workspace's `.jj` (root from
  `jj workspace root --name default`), plus the main workspace's `.git` when
  it exists (colocated): jj's git backend writes commit objects and exported
  refs into the carved-out gitdir. The rule is uniform across jj shapes — in
  a primary workspace the `.jj` grant is redundant but harmless, and stays
  correct should codex ever add a `.jj` carve-out.

The flags are passed unconditionally for codex launches (harmless when the
sandbox is off) and no other harness is touched.

Verified facts (codex-cli 0.145.0 source at `rust-v0.145.0`, plus live
`codex exec --sandbox workspace-write` probes in scratch repos of every jj
shape, 2026-07-23; the git facts first verified against 0.144.5, 2026-07-22):

- **Grants are additive.** `--add-dir` adds to the default writable roots
  (cwd, `/tmp`, `$TMPDIR`) — the policy builder (`get_writable_roots_with_cwd`)
  always appends the defaults, and the exec header confirms
  `workspace-write [workdir, /tmp, $TMPDIR]` alongside grants. The loop's own
  signal write to `$TMPDIR` stays safe, and redundant grants cost nothing.
- **The carve-out is per-rule and name-specific: `.git`, `.agents`, `.codex` —
  never `.jj`.** Each writable root's seatbelt allow-rule excludes its own
  top-level protected names via `require-not`
  (`default_read_only_subpaths_for_writable_root`,
  `codex-rs/protocol/src/permissions.rs:22-27` at `rust-v0.145.0`); codex even
  resolves a worktree's `.git` pointer file and carves out the real gitdir.
  Allow-rules compose, so a separate explicit root gets its own clean allow —
  which is why a grant reopens a carved dir without touching the defaults.
- **The jj probe matrix** (each grant proven load-bearing in at least one
  shape): jj-native primary — `jj describe` succeeds with *no* grant (`.jj`
  writable under cwd). Colocated primary — fails without the gitdir grant,
  succeeds with it (git-object/ref writes into `.git`). Secondary workspace of
  a native main — fails without the main-`.jj` grant (snapshot writes into
  `<main>/.jj/repo/store/git/objects`), succeeds with it. Secondary workspace
  of a colocated main — still fails with only the `.jj` grant, succeeds once
  the main `.git` is granted too.
- **Probe trap: the `codex sandbox` subcommand does not model the session
  sandbox.** Since at least 0.144.5 it requires `--permission-profile` and
  exercises codex's named-profile machinery, not the legacy
  `sandbox_workspace_write` policy the TUI/exec path uses. Probing it gave a
  false replace-not-add reading that this decision's earlier drafts nearly
  encoded. Settle sandbox questions against `codex exec` (same policy builder
  as the TUI), never the subcommand. (Relatedly: `--skip-git-repo-check` and
  the hard not-a-git-repo refusal are `codex exec`-only — the TUI grove
  launches has no git gate, so jj-native trees launch fine.)

## Considered options

- **A user-owned `~/.codex/config.toml` `writable_roots` grant** — rejected:
  the path is per-working-tree, so a static global config must be re-derived by
  hand for every new grove, and it widens *every* codex session in that
  checkout, not just grove's. Reopened if codex removes `--add-dir` (present
  on both the TUI and `codex exec` at 0.145.0).
- **A launch pre-flight probing store writability** — rejected: with the flags
  structural, the failure mode it would detect no longer exists; a codex old
  enough to reject the flag fails loudly at spawn, which is diagnostic enough.
  Reopened if unexplained codex launch failures surface in the field.
- **Deriving the jj git store from `.jj/repo/store/git_target`** — rejected:
  parsing jj's on-disk store format couples grove to it for no gain;
  `<main>/.git` existence is the same signal (colocation is exactly the case
  where `git_target` escapes the repo store), and the probe matrix confirms
  the pair of grants suffices.
- **Granting only what each jj shape strictly needs** — rejected: shape
  dispatch (primary vs secondary, native vs colocated) buys nothing, since
  grants are additive and a redundant grant is a no-op; the uniform rule is
  simpler and future-proof.

## Consequences

- The sandboxed agent gains write access to the entire store it is granted —
  for git, **including `.git/hooks/`**, which is precisely what codex's
  carve-out defends (a hook written in-sandbox later fires outside it, e.g. on
  the user's own commit in the main checkout); in a colocated or secondary-of-
  colocated jj tree the granted main `.git` carries the same exposure. Accepted
  deliberately: letting an agent commit *is* extending that trust, and it is
  the same trust already extended to the unsandboxed harnesses (pi, claude's
  default posture).
- grove sessions on codex are committable hands-free in every repo shape grove
  drives — plain git checkout, linked worktree, jj-native, colocated, and
  secondary jj workspaces — without manual sandbox configuration.
