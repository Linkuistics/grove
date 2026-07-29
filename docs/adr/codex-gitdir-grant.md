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

The flags are passed unconditionally for codex launches and no other harness is
touched — but **not blindly**: a launch pre-flight asks codex what sandbox this
launch would get and **refuses** rather than passing grants codex would reject.
The three modes divide two ways, not three: `workspace-write` and
`danger-full-access` accept `--add-dir`, and `read-only` refuses it *fatally*
(codex exits 1 before drawing any TUI). "Off" is not a fourth mode — the grants
are inert under `danger-full-access`, which is what "harmless when the sandbox
is off" meant and all it ever covered.

`read-only` is not an exotic configuration, it is the **default for any project
the user has not trusted**, and trust is per-directory in
`$CODEX_HOME/config.toml` with **no inheritance from parent directories** — so a
brand-new working tree, which is exactly what `grove do` bootstraps into, is
untrusted by construction even inside a trusted parent. The pre-flight therefore
guards the common case, not a corner.

**Refuse, rather than elevate or degrade.** grove could pass `--sandbox
workspace-write` and make every launch succeed, but the sandbox posture is the
user's: codex's trust prompt exists so a human answers it once, and a process
tool has no mandate to route around it on every launch. (Contrast the `.git`
grant itself, which sits *inside* the trust a user extends by letting an agent
commit at all — a project they have never trusted extends no such trust.)
Degrading is worse still: `-c sandbox_workspace_write.writable_roots=[…]` is
silently *ignored* under `read-only` rather than fatal, so switching to that flag
form would buy a session that comes up and then cannot commit, and grove's Commit
and Retire steps are mandatory. Refusing costs the human one action, once per
working tree, and the diagnostic names both remedies — trust the project, or set
`sandbox_mode = "workspace-write"` — because codex's own one-liner names the two
modes it would accept and says nothing about the trust that set the mode.

**The verdict is codex's own, not a reimplementation.** The probe runs `codex
exec` with the same model flags and the same grants the launch will pass, and
reads the one `sandbox:` line of the header — the same policy builder the TUI
uses, which is why `codex sandbox` is the trap recorded below. It also passes
`--skip-git-repo-check`, which is what makes the probe *be* the TUI rather than a
stricter cousin of it: `codex exec` carries a git gate the TUI does not, and the
gate closes in exactly the quadrant the pre-flight exists for (below). Reimplementing the
trust rules could not be made safe anyway, since an explicit `sandbox_mode` in
the config, in a `--profile` layer, or on the command line overrides the trust
default. So there are no false refusals: if it says `read-only`, the session
genuinely could not have committed. Anything else — codex unspawnable, no header
inside the timeout, a mode this build does not know — proceeds, because grove
guides and does not gate (constraint 5) and a probe that cannot answer must never
be what stops a loop.

The probe runs **per launch, not once per `grove do`**: codex models are routed
through `--profile`, which is a whole config layer that can itself set
`sandbox_mode`, so the answer is a property of *(tree, model)* and is unknowable
before a leaf is picked. It costs ~0.15s, is killed the instant the header
arrives — before codex issues any request — and so spends no tokens, writes no
trust entry and leaves no rollout; its one residue is an empty lock dir in
`$CODEX_HOME/tmp/arg0/`, which codex sweeps — the directory held four entries and
0 bytes both before and after a dozen probes, and one of the four was replaced in
between.

**The spawn carries no loop-control environment.** It is a harness spawn that is
not the session, so `GROVE_SIGNAL_FILE` and the retired PID handles are scrubbed
from it (*self-driving-loop*). Not a detail of this decision so much as the first
place its general rule was needed: the probe originally inherited the driver's
signal file, and in a meta-grove — where the test suite runs as a descendant of a
live session — that killed the terminal `cargo test` was typed into.

Verified facts (codex-cli 0.145.0 source at `rust-v0.145.0`, plus live
`codex exec --sandbox workspace-write` probes in scratch repos of every jj
shape, 2026-07-23; the git facts first verified against 0.144.5, 2026-07-22; the
read-only facts against the installed 0.145.0, 2026-07-28):

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
- **`read-only` is the untrusted default, and it does not inherit.** A scratch
  repo at `~/.cache/…` — inside the trusted `[projects."/Users/antony"]` — still
  reports `sandbox: read-only`. Under it, `codex --add-dir <dir>` exits **1**
  with `Error adding directories: Ignoring --add-dir (…) because the effective
  permissions do not allow additional writable roots.` — fatal despite reading
  like a warning, and before any TUI is drawn, which is what made the loop's stop
  look like a mute non-signal exit.
- **`codex exec` still answers under `read-only`, grants and all.** Given the
  same `--add-dir` flags that kill the TUI, `codex exec` prints its full header
  (`sandbox: read-only`) and carries on. That asymmetry is what makes the
  pre-flight possible at all: the probe gets its verdict in exactly the case the
  launch would die in. The header goes to **stderr**, and arrives in 0.1–0.4s,
  before hooks or MCP servers spin up.
- **…but `codex exec` gates on a git repo where the TUI does not, so the probe
  must pass `--skip-git-repo-check`.** Outside a git repo *and* outside a trusted
  directory, `codex exec` prints `Not inside a trusted directory and
  --skip-git-repo-check was not specified.` and exits **before** the header.
  Untrusted is precisely the condition that makes the sandbox `read-only`, so in
  a **jj-native** working tree — no `.git` for the gate to find, and a secondary
  workspace of a *colocated* main has none of its own either — the two conditions
  arrive together and an unflagged probe goes mute in exactly the case it exists
  for: the verdict degrades to `Unknown`, the launch proceeds, and codex dies on
  `--add-dir`. The flag clears the gate and moves no policy — an untrusted
  jj-native tree reports the same `sandbox: read-only` that the same tree reports
  with a `.git` beside it (measured, 0.145.0, 2026-07-29). This is why the
  guard's blind spot was anti-correlated with its purpose rather than merely
  incomplete, and why the case is pinned by a fake codex that enforces the gate.
- **Probe trap: the `codex sandbox` subcommand does not model the session
  sandbox.** Since at least 0.144.5 it requires `--permission-profile` and
  exercises codex's named-profile machinery, not the legacy
  `sandbox_workspace_write` policy the TUI/exec path uses. Probing it gave a
  false replace-not-add reading that this decision's earlier drafts nearly
  encoded. Settle sandbox questions against `codex exec` (same policy builder
  as the TUI), never the subcommand. (Relatedly: the hard not-a-git-repo refusal
  is `codex exec`-only — the TUI grove launches has no git gate, which is both
  why jj-native trees launch fine and why the probe must clear it, above.)

## Considered options

- **A user-owned `~/.codex/config.toml` `writable_roots` grant** — rejected:
  the path is per-working-tree, so a static global config must be re-derived by
  hand for every new grove, and it widens *every* codex session in that
  checkout, not just grove's. Reopened if codex removes `--add-dir` (present
  on both the TUI and `codex exec` at 0.145.0).
- **A launch pre-flight probing store *writability*** — rejected, and it stays
  rejected: what the pre-flight above asks is which **mode** codex resolved, not
  whether a particular path is writable. The distinction is the whole reason it
  works — a mode is one line codex prints about itself, whereas writability is a
  property this probe would have to establish by *doing* something in the user's
  tree, per grant, per repo shape.

  (This is where the decision moved. The pre-flight *idea* was rejected on
  "with the flags structural, the failure mode it would detect no longer
  exists", reopened by its own condition when an unexplained codex launch
  failure surfaced in the field, and reinstated in the mode-probing form above.
  The premise that failed was not "pre-flights are unnecessary" but the
  unconditional-grants sentence it rested on: there was a third mode, it was
  fatal, and it was the default.)
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
  secondary jj workspaces — **once the project is trusted**. Trusting it is a
  one-off human action per working tree, and it is the one thing about a codex
  grove that is not hands-free; grove asks for it by name rather than working
  around it.
- Every codex launch spawns codex twice: once as the probe, once as the session.
  ~0.15s and no tokens, but it means a fake codex in a test — or any wrapper on
  the `GROVE_HARNESS_BIN*` seam — sees two invocations and must tell them apart.
  `exec` as the first argument is the discriminator: grove's codex launch argv
  never carries a subcommand.
