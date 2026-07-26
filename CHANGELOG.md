# Changelog

One file, one entry style, for both of this repo's components (*skills-monorepo*).

**Versioned sections are grove's.** A `## v<N>.<m>.<p>` heading is a release of
the `grove` binary — that is the only artifact this repo tags and ships by
version. Entries under it are grouped `### Added` / `### Changed` / `### Fixed`
where a release has enough of each to be worth grouping, and a flat list
otherwise.

**A skills-only change is logged in the section of the grove release it lands
before**, prefixed with the plugin and skill it touched — e.g. *"`linkuistics`
/ `using-jujutsu`: …"*. It gets no `##` heading of its own: this file is not the
plugins' release ledger, it is the record of what changed and when. A grove
release that happens to contain only skill changes still gets a version, because
the binary is what was cut.

**A skills entry names no version, because the plugins carry none.** Neither
`plugins/<name>/.claude-plugin/plugin.json` declares a `version`: both are
versioned by commit SHA, so every push delivers and there is no bump to record
(*skills-monorepo* has the trade, and adding a `version` would quietly undo it).
The skills reaching codex, gemini and pi by `install.sh` are symlinks, so they
were never in a version's path either.

The section at the foot of this file is the `Linkuistics/skills` changelog as it
stood at the graft — a closed record, not part of the versioned sequence above.

- **The `linkuistics` and `testanyware` plugins now ship from this repo;
  `Linkuistics/skills` is archived** (*skills-monorepo*). That repo's history is
  grafted in here, so `git blame` on `plugins/linkuistics/skills/*` still traces
  past the merge. The two components live together because they change in
  lockstep — most grove changes need a matching skill change, and two repos made
  every such change a cross-repo pair no single commit could carry.

  **If you installed the marketplace before this change, run both of these:**

  ```
  /plugin marketplace remove linkuistics
  /plugin marketplace add Linkuistics/grove
  ```

  Nothing else changes: the marketplace keeps the name `linkuistics` (its
  identity is the `name` field in `marketplace.json`, never the repo URL), so
  `linkuistics@linkuistics`, `testanyware@linkuistics` and every
  `linkuistics:<skill>` reference keep working untouched. **Re-pointing is not
  optional even though nothing will break loudly:** an archived GitHub repo stays
  readable, so `autoUpdate: true` keeps *succeeding* against `Linkuistics/skills`
  — the skills simply freeze at the last commit before the archive, with no error
  surfaced. `install.sh` users re-clone from `Linkuistics/grove` instead.

- **`install.sh` links into Pi as well as codex and gemini.** Pi reads personal
  skills from `~/.pi/agent/skills`, one level deeper than the others; the
  existing "install only if the parent directory exists" guard needs no
  special-casing for it, since `dirname` yields `~/.pi/agent` — present only
  when Pi is set up. Pi had been symlinked by hand against `Linkuistics/skills`
  and so was left behind by the graft, frozen at the pre-archive skill set;
  re-running `install.sh` re-points it here and picks up `using-jujutsu` and
  `git-to-jj-mapping`, which no hand-linked harness had.

- **A `grove do` pane now tells herdr what it is doing** (*herdr-optional-ui*).
  The loop driver reports `working` while a session runs, `blocked` when the loop
  parks needing a human, and `idle` when the grove finishes — over herdr's own
  unix socket, addressed by the `HERDR_*` variables herdr puts in the pane
  environment. This fixes the complaint the feature exists for: a grove that
  stopped overnight used to read as **done**, because herdr derives `done` from
  `idle && !seen` and nothing ever reported `blocked`.

  **Entirely optional, and never load-bearing.** With no herdr present — or with
  stock, unpatched herdr, which drops the reports — every grove behaviour is
  unchanged, minus the status surface. A refused or wedged socket is a no-op
  bounded at 500ms, never a failed launch and never a stalled loop.

  Two details worth knowing. **Release is not "on every exit":** grove hands the
  pane back on `complete --done` and on SIGTERM/SIGHUP, but a loop parked without
  a completion signal keeps its `blocked` report — releasing would return the
  pane to screen detection, which reads a parked grove as `idle`, restoring the
  very bug. And **the driver sees session boundaries, not turn boundaries**: a
  session stalled *mid-turn* on a question reads `working`, not `blocked`. That
  is a strict improvement on `done`, not the whole fix; intra-turn state needs
  per-harness hooks. Uncovered by design, since herdr never expires an authority:
  SIGKILL, panic, OOM and power loss pin the pane at grove's last state until the
  next `grove do` or a `herdr pane release-agent`.

## v15.0.0

grove goes dual-VCS: jj-enabled working trees — native, colocated, and
secondary jj workspaces — drive first-class alongside git, from detection
through the codex sandbox grants to the prose.

### Added

- **jj-enabled working trees drive first-class.** A thin jj-first probe, no
  VCS trait: the nearest ancestor's `.jj/` wins over a `.git` beside it, so a
  colocated repo drives through jj while a git repo nested under a jj tree
  stays git — the repo's state alone picks the interface, and git remains it,
  silently, in not-jj-enabled trees. In a jj-enabled tree the workspace root
  comes straight from the marker walk (no jj binary needed); main-repo
  resolution from a secondary workspace runs `jj workspace root --name
  default --ignore-working-copy` (never snapshotting); renames go plain (jj
  has no index to keep in step — a `git mv` in a colocated tree would stage
  into an index jj ignores); and the adoption migration commits via
  fileset-scoped `jj commit .grove` (jj-authored: change-id, op-undoable).
  Tests cover all three repo shapes.
- **codex launches grant the jj store** (*codex-gitdir-grant*). The
  `--add-dir` grant goes per-VCS: git trees keep the absolutized common dir
  exactly as before; jj-enabled trees grant the main workspace's `.jj` — a
  secondary workspace's every op lands in the *main* workspace's `.jj/repo`,
  outside the sandbox cwd — plus the main `.git` when it exists (colocated),
  where jj's git backend writes commit objects and exported refs into the
  carved-out gitdir. Each grant proven load-bearing by live `codex exec`
  probes across every jj shape. Previously the grant derivation shelled
  `git rev-parse --git-common-dir` unconditionally, so a codex launch in a
  jj-native tree died at spawn.

### Changed

- **The prose speaks dual-VCS, git default** (*user-owned-worktrees*,
  reworked in place to user-owned *working trees*). Conceptual lines go
  VCS-neutral ("the VCS holds the history"); wherever a concrete command
  appears, both interfaces are named (`git init` / `jj git init --colocate`,
  `git rev-parse --show-toplevel` / `jj workspace root`); verb-behaviour
  notes carry the jj plain-rename reality; CLI help drops its git-only
  precondition claim. grove reads no branch — and now, no bookmark.

## v14.0.0

The harness trial's remaining launch-time gaps close: codex sessions can
commit, pi sessions carry names, and a `brew upgrade` mid-loop stops the loop
cleanly at the next session boundary instead of hanging it at the next
completion signal.

### Added

- **Version-skew guard** (*self-driving-loop*). Before each session launch
  the driver confirms that `grove-llm --version` — resolved as the *agent*
  resolves it, through `GROVE_LLM_BIN`/PATH, deliberately not the driver's
  prefer-the-sibling rule (the stale sibling would match the stale driver and
  hide exactly the skew being checked for) — still reports the driver's own
  compiled-in version. A `brew upgrade` mid-loop replaces the binary on disk
  while the running driver keeps executing the text segment it started with,
  silently splitting the signal protocol's two halves: every session hangs at
  its completion signal, nothing relaunches, no diagnostic. On a confirmed
  mismatch the loop now stops before the next session, naming both versions
  and the restart instruction (restart ≡ continuation). An *unreadable*
  version (missing binary, failed or unparseable `--version`) only warns and
  continues — the guard guides, it does not gate. Checked per session, not
  per driver start, because a mid-loop upgrade is precisely what a start-time
  check misses.
- **`## Requirements` in the spec format.** SPEC-FORMAT.md gains an optional
  requirements section — one `### Requirement:` per behaviour with a SHALL
  statement specific enough to test, each `#### Scenario:` a WHEN/THEN
  acceptance case; scenarios say *what must pass*, `## Test seams` says
  *where it is tested*. The requirement/scenario language is adapted from
  OpenSpec (MIT, `LICENSES/openspec.LICENSE`) — the spec language only, none
  of its delta/validation machinery.

### Fixed

- **codex sessions can commit** (*codex-gitdir-grant*). codex's
  `workspace-write` sandbox carves the repository gitdir out read-only, so
  `git commit` — and with it grove's mandatory Commit and Retire steps —
  failed inside every codex session. Every codex launch now appends
  `--add-dir <absolutized git-common-dir>`: one path covers both repo shapes
  (a linked worktree's gitdir is a subpath of the common dir; a plain
  checkout's common dir *is* `.git`), grants are additive so the default
  writable roots stay intact, and the flag is harmless when the sandbox is
  off. No other harness is touched.
- **pi sessions are pre-named at launch.** pi does have a launch-time
  session-name flag — `--name/-n` (verified live on pi 0.80.10) — so pi
  launches now set the standard `<repo>: <name> grove` display name; v12
  shipped without pre-naming on the mistaken reading that no such flag
  existed.

## v13.0.0

The self-driving loop's session-end kill moves from the agent to the loop
driver: an in-agent self-kill cannot be trusted under every harness sandbox.

### Breaking

- **`GROVE_HARNESS_PID` and `GROVE_CLAUDE_PID` are gone.** The loop driver no
  longer exports either — it spawns the harness session directly (no `sh -c
  export…exec` wrapper) and kills its own child itself, so the agent never
  needs its own PID. `grove-llm complete`'s `--pid`, `--grace`, and
  `--kill-grace` flags are gone with them; `GROVE_KILL_GRACE` and
  `GROVE_KILL_GRACE_KILL` are now read by the driver instead of by `complete`.

### Fixed

- **`grove-llm complete` now actually ends a codex session.** codex's
  Seatbelt sandbox denies a same-sandbox process signalling its own session
  (`(allow signal (target same-sandbox))`), so the previous self-spawned
  delayed killer's `kill -TERM`/`kill -KILL` silently failed under codex (the
  `EPERM` was hidden by `2>/dev/null`) — a codex-driven loop never relaunched
  on its own. The loop driver, running outside any harness sandbox and always
  able to signal its own child, now watches for the completion signal itself
  and applies the same grace → SIGTERM → kill-grace → SIGKILL sequence to the
  session it spawned.
- **An out-of-range `GROVE_KILL_GRACE` no longer panics the driver.** The
  watcher clamped negatives but passed non-finite and absurdly large values
  straight to `Duration::from_secs_f64`, which panics on them: `inf` or `1e300`
  took the whole loop down. Non-finite values now fall back to the default and
  finite ones clamp into `[0, 3600]`.
- **`grove do`'s pre-flight now checks every harness a per-kind override could
  route to, not just the stamped one.** Dropping the `sh -c` wrapper (above)
  changed a missing harness binary's failure mode: a genuinely unspawnable
  binary now aborts `grove do` with a loud `ENOENT`, rather than the old
  `sh`-absorbed exit 126 that looked like a friendly "the human exited" stop.
  Loud-over-silent is right, but pre-flight validated only
  `resolve_for_launch`'s stamped harness — so `GROVE_REVIEW_HARNESS=pi`
  against a grove stamped to `codex` passed pre-flight with `pi` not
  installed, ran for however long, and only aborted once a review leaf was
  finally picked. Pre-flight now resolves and checks every configured
  `GROVE_<KIND>_HARNESS` override too (through the same
  `GROVE_HARNESS_BIN`/`GROVE_HARNESS_BIN_<NAME>` resolution the real launch
  uses), naming the offending override var and binary in the diagnostic.

## v12.0.0

grove learns to route: a `pi` harness joins claude and codex, leaves can be
routed per **kind** to a different harness than the grove's own, and model
selection becomes harness-scoped — the shape needed to drive two harnesses
(two subscriptions) concurrently and still send every review to one reviewer.

### Breaking

- **codex launches use `--profile`, not `--model`.** A codex profile binds
  model + reasoning effort, which a bare model flag cannot; model-per-task-kind
  values for codex now name profiles defined as `$CODEX_HOME/<name>.config.toml`
  files (not a `[profiles.<name>]` table in `config.toml`).
- **`GROVE_HARNESS_PID` replaces `GROVE_CLAUDE_PID`.** The loop wrapper still
  co-exports the old name and `grove-llm complete` still reads it as a
  fallback — for this release only.
- **Skill provisioning is multi-harness.** `grove do` extracts the embedded
  methodology into every installed harness's skills dir (`~/.claude/skills/grove`,
  `~/.codex/skills/grove`, `~/.pi/agent/skills/grove`), replacing symlinked
  `grove` entries with real dirs (links are removed as links, never followed).
  A `grove` entry that is neither a symlink nor grove-provisioned is refused.

### Added

- **`pi` harness** (`--harness pi`): launches `pi` with `--model` (pi accepts
  `provider/id` patterns), no session pre-naming (pi has no launch-time name
  flag), skills under `~/.pi/agent/skills`.
- **`GROVE_<KIND>_HARNESS`** — route leaves of one kind to another harness at
  launch (e.g. `GROVE_REVIEW_HARNESS=pi`). Model resolution follows the
  post-override harness. Unknown names fail loudly.
- **`GROVE_<HARNESS>_<KIND>_MODEL`** — harness-scoped model vars
  (e.g. `GROVE_PI_REVIEW_MODEL`) that beat the base `GROVE_<KIND>_MODEL`.

### Fixed

- **Explicit `--harness` now always persists** to `.grove-stamps/<name>`.
  Previously only multi-harness repos stamped, so an explicit choice in a
  repo with a single (different) harness dir silently reverted on the next
  plain `grove do`.

## v11.0.0

grove exits the git-topology business. Through v10.0.3, `grove do <name>` owned a canonical layout — it created `<repo>/.grove-worktrees/<name>/` on a same-named branch, re-attached it if orphaned, and the finish cycle merged that branch to the default, removed the worktree, and deleted the branch. That layout fights any tool that wants to own worktree placement itself (e.g. [worktrunk](https://github.com/max-sixty/worktrunk)): grove and the tool each assume they control creation, naming, and teardown. grove now owns none of it — the workflow's one precondition is *a git working tree*, user-provided, on any branch, anywhere on disk; grove reads no branch, ever. (ADR `user-owned-worktrees`)

### Breaking

- **`grove do` loses its `<name>` argument and `--start-point`.** Run it argument-less, from inside the working tree you've already created (`git init`, `git clone`, a plain checkout, or a linked worktree from your own tooling). It still inspects state on disk and dispatches exactly as before — no `.grove/` yet → bootstrap; a live tree → continue; no live leaves → propose the finish cycle — the state-dispatch logic is unchanged, only where the worktree itself comes from (ADR `do-is-sole-lifecycle-verb`, reworked in place: the creation and orphan-reattach dispatch arms are gone, since there is no longer a canonical location to create or re-attach).
- **The grove's name is now the working tree's own directory basename**, read via `git rev-parse --show-toplevel` — never derived from a branch or a canonical path. It names the root brief, the harness session (`<repo-basename>: <name> grove`), and the harness stamp (`<repo>/.grove-stamps/<name>`), exactly as before; only the source of the name changed.
- **`grove retire` addresses a node as `grove retire <node-path>`, in-worktree** — the old two-part `<name>/<node-path>` addressing is gone with the canonical layout, since `<name>` is no longer a lookup key, just the label the current working tree happens to carry.
- **The complete finish cycle shrinks from six steps to three.** It now ends at *promote → delete `.grove/` in one commit → signal `complete --done`*; the old merge / worktree-remove / branch-delete steps are gone (ADR `in-session-finish-cycle`, reworked in place). Integrating the branch and tearing down the working tree are the user's own git/gh (or worktree tool) from here — grove creates no topology, so symmetrically it merges and deletes none. Resume logic shrinks to match: a half-finished grove resumes at "promote" if `.grove/` still exists, or reports "already finished" if it's already gone — the old merge-base / worktree / branch resume checks no longer apply.
- **No topology convenience verbs.** An earlier design for this release sketched `grove create <name>` / `grove remove <name>` as opt-in utilities outside the main workflow; both were eliminated in the same grilling that settled the inversion; nothing in the CLI surface replaces the worktree/branch handling that's gone. The surface is now exactly `do` / `migrate` / `retire`.

### Migrating an existing grove

Nothing needs to move. A worktree already sitting at `<repo>/.grove-worktrees/<name>/` on branch `<name>` is just an ordinary git working tree now — `cd` into it and run argument-less `grove do`; `.grove-worktrees/` carries no special meaning to grove any more, so leaving it in place or relocating it with plain `git worktree move` are equally fine. A new grove needs a working tree you make yourself before the first `grove do`.

## v10.0.3

Two bug fixes, each filed against a precondition that was false or never examined: grove could not rename a task-tree entry it had not yet committed, and the codex harness declared a command-line flag codex does not have.

### Fixed

- **A task-tree entry grown this session can now be renamed.** grove renames an entry with `git mv` in five places — `leaf-insert`'s sibling renumber, `leaf-decompose`'s leaf→`BRIEF.md` promotion, `leaf-retire`'s `DONE` infix, `leaf-prune`'s `ABANDONED` infix, and the v1→v2 migration. But `git mv`'s job is to move an entry in git's *index*, and grove's grow verbs deliberately create entries **untracked** — working-tree changes only, folded in by the enclosing task's commit. So grove routinely produced files its own rename primitive rejected, and the ordinary rhythm of a planning session — grow a few leaves, then realise one must sequence earlier, or decompose one, or finish one — hit a raw `fatal: not under version control` with no hint at the cause or the fix. That was never git being obstructive; it was grove asking git to move something it had not been told about. The primitive is now fixed in one place (`src/tree_rename.rs`) and dispatches on the entry's state: **tracked ⇒ `git mv`**, so the index entry moves with the file and `git status` still shows a clean staged rename before you commit; **untracked ⇒ a plain filesystem rename**, because there is no index entry to move — the entry was untracked before and is untracked after, at a new name, and the same `git add` that was always going to fold it in still does. Neither branch is a fallback for the other; each is the correct operation for the entry's state. A commit records no rename information either way (git infers renames at diff time, by content similarity), so the two branches commit byte-identical trees. (Issue #3)

  **`leaf-prune` changes behaviour.** It had met this same defect and answered it differently — detecting an untracked leaf and refusing with "run `git add` first". Renaming one now simply works, so that check is gone. Its two-phase validate-before-mutate walk stays, because a leaf whose `ABANDONED` name is already taken still has to fail the whole call cleanly rather than half-mark a subtree.

- **The codex harness no longer passes a flag codex does not have.** `src/harness.rs` gave codex `name_args: &["--name"]`, under a comment claiming it had been verified against `codex --help`. codex has no `--name` — checked against codex-cli 0.144.1, zero matches. Session names do exist in codex, but are assigned *after* start (`codex resume` takes a "session id (UUID) or session name", and the name is set in-session via `/rename`); naming at launch is an open upstream request. A grove session on codex would therefore have died in codex's argument parser before any session began — latent until now only because harness selection runs in single-harness mode and no grove drives codex. Conversely, codex *does* accept `-m, --model <MODEL>`, so its `model_args: &[]` had opted it out of model-per-task-kind (v10.0.0) for no reason. Both are corrected: codex declares no launch-time name flag (the launch paths already skip pre-naming when the template is empty) and takes part in model-per-task-kind on the same terms as claude. A false verification claim is worse than no comment — it tells the next reader not to re-check — so the field's own doc comment, which carried the same claim one level up, is corrected too. (Issue #1)

## v10.0.2

A bug fix: the self-driving loop swallowed the one warning that explains a model downgrade, so an unrecognised task kind quietly launched its leaf on the cheapest model.

### Fixed

- **An unrecognised `**Kind:**` line no longer downgrades a leaf's model in silence.** Reading a kind *degrades* rather than errors — an unrecognised token (a typo, a hand-edited file, or a tree written by a newer grove) warns and is treated as `work` (ADR `task-kind-taxonomy`), so a typo can never jam an unattended relaunch. But the warning rides a **zero exit**: `grove-llm kind` prints it on stderr and exits 0. The loop driver ran that peek through `Command::output()`, which *captures* stderr, and only read it back on the failure branches — so on the degrade path the warning was discarded outright. The leaf then launched on `GROVE_WORK_MODEL`, which in a typical configuration is the *cheapest* model, with nothing on screen to say why. Because the degrade always lands on `work`, the failure was always in the cheap direction, and so never announced itself as a failure. `resolve_kind` now **inherits** the child's stderr instead of capturing it, so every `grove-llm kind` diagnostic — this warning and any future one — reaches the operator; the failure branch drops its now-redundant echo. The degrade itself is unchanged: the kind still resolves to `work` and the leaf still launches, because model selection must never be a reason to stop the loop. (Issue #2's sibling, issue #4)

  **On the symptom that prompted this.** Issue #4 reported research leaves launching on Sonnet. That observation was a **version skew, not a dispatch bug, and it needs no fix**: those leaves lived in the grove that was *building* the five-kind taxonomy, driven by the then-installed v9.1.0 — whose `Kind` was `{Work, Planning}` and which had no degrade-on-read at all, so `grove-llm kind` simply failed on `**Kind:** research`, `resolve_kind` degraded to `None`, no `--model` was passed, and the session inherited the harness's own default. Dispatch on v10 is correct and verified end-to-end. What the investigation *did* surface is the defect fixed above — and the reason it was hard to find in-session is precisely that the diagnostic was being thrown away.

## v10.0.1

A documentation fix. Two files still described grove's task kinds as a binary after v10.0.0 made them a closed set of five. The binary is unchanged apart from the embedded methodology it carries.

### Changed

- **`content/SKILL.md` and ADR `self-extension-core-and-methodology` point at the task-kind taxonomy instead of restating a count.** Both still read "the two task kinds" after v10.0.0 replaced the `planning`/`work` binary with a closed set of five — a stale count in the *embedded methodology*, which `grove do` extracts to `~/.claude/skills/grove/`, so a session was told there were two kinds while `leaf-add --kind` already accepted five. Both now name the taxonomy and cite where it is defined (`content/TASK-FORMAT.md`, ADR `task-kind-taxonomy`) rather than restating a number that a sixth kind would falsify again. The v10.0.0 entry below has also been completed: the five-kind taxonomy shipped in that release but was never written down here.

## v10.0.0

This release expands grove's task kinds from a `planning`/`work` binary to a closed set of five, replaces grove's two planning artifacts with one — the spec absorbs the PRD — and gives grove a representation for abandoned work: a leaf can now be pruned in place (`ABANDONED`) instead of hand-deleted, closing the defect where deletion silently re-issued a retired permanent key.

### Breaking

- **grove has one planning artifact: the spec. `docs/prd/` is gone.** Through v9.1.0 grove named two artifacts a planning increment could produce — a PRD (`docs/prd/`, "human-facing agreement checkpoints; committed, never retired") and a design spec (`docs/specs/*-design.md`, "workstream-level technical design"). No document ever honoured the distinction, in this repo or downstream, because no step ever tested it. They are now one artifact: **a spec at `docs/specs/<slug>.md`**, slug-named, described by a new bundled format guide `content/SPEC-FORMAT.md`. Two rules make the set checkable. **Membership:** *would a session on an unrelated future grove need to read this?* If not it is a `BRIEF.md`, and it dies with `.grove/` — work-orders, keep/delete tables, and "the input for the next three leaves" were always briefs. **Grain:** an ADR records one decision and its trade-off; a spec describes how an area works, and *cites* the ADRs in its area rather than restating them. Like `docs/adr/` since v9.0.0, **`docs/specs/` is now a minimum coherent set describing the current design** — edited, merged and split in place, and deleted once a spec describes nothing (constraint 1: the artifacts hold the present, git holds the past). It does *not* inherit the PRD row's "committed, never retired". Specs also drop the `## Decomposition` section, which is brief material.

  **Migrating a repo that has `docs/prd/`.** grove ships no migration for this — nothing in the binary reads `docs/prd/` or `docs/specs/`, so a stale directory breaks nothing and can be converted whenever a grove next drives the repo. Per file: (1) `git mv docs/prd/<file>.md docs/specs/<slug>.md`, stripping any `NNNN-` or `YYYY-MM-DD-` prefix and any `-design` suffix — the slug is the identity; (2) apply the **membership test** — a document that only ever served one grove's leaves was a brief, so delete it rather than move it, and a document that no longer describes anything is deleted too (git holds both); (3) reshape what survives to `SPEC-FORMAT.md`'s sections, moving any `## Decomposition` into the relevant `BRIEF.md`; (4) remove `docs/prd/` once empty. Existing `docs/specs/*-design.md` files rename to `docs/specs/<slug>.md` by the same rule.

### Added

- **A closed set of five task kinds — `planning`, `research`, `prototype`, `work`, `review` — each with its own session discipline and model bucket.** Through v9.1.0 every leaf was either `planning` or `work`, which forced three genuinely different sessions — a citation-disciplined literature survey, a deliberately throwaway spike, and a fresh-context adversarial read — to share one label, one discipline and one model. grove now names all five, and each earns its place by carrying behaviour beyond a name. The set is **closed**: a sixth kind is a change to grove's code and docs, not a free-text label a leaf may coin — because grove *reasons* about the kind rather than merely reporting it (`leaf-decompose` gives a new node's first child its parent leaf's kind, so a research leaf that proves bigger becomes a research node, and a label grove cannot enumerate is one it cannot give defaults for or key a model bucket on). Enforcement is deliberately **asymmetric — gate on write, degrade on read**: `leaf-add` / `leaf-insert` / `leaf-decompose` reject an unknown `--kind` with an error listing the five (a human is present at authoring time, so catching `reserch` there costs one retry), while an unrecognised `**Kind:**` line — hand-edited, or written by a future grove version — emits a warning and is treated as `work`, because the self-driving loop relaunches unattended and a typo must not jam it (constraint 5: grove guides rather than gates). Only **`planning`** carries methodological force — it remains the loop's sole branch and the only kind that grows the tree; the other four are work-shaped sessions producing an artifact, differing in discipline, not in what the loop does with them. Each kind is marked HITL (`planning`, `prototype`) or AFK (`research`, `work`, `review`) in `content/TASK-FORMAT.md` — documented guidance with no machinery behind it. Existing trees are unaffected: `planning` and `work` keep their labels and meanings, and no leaf changes kind on its own. (ADR `task-kind-taxonomy`)
- **Five model variables, one per kind**, extending v9.1.0's two: `GROVE_PLANNING_MODEL`, `GROVE_RESEARCH_MODEL`, `GROVE_PROTOTYPE_MODEL`, `GROVE_WORK_MODEL`, `GROVE_REVIEW_MODEL`. The v9.1.0 rules are unchanged — an **unset variable means no `--model`** for that kind (the session inherits your own default, so grove stays a no-op until you opt in), and the launch model is a default, not a lock. Documented in `grove do --help`, README (`## Configuration`), and CONTEXT.md. (ADR `model-per-task-kind`)
- **`content/SPEC-FORMAT.md` — the spec's shape, the fifth bundled format guide.** Five sections that earn their place (`## Problem`, `## Solution`, `## Decisions`, `## Test seams`, `## Out of scope`) plus three rules: **synthesise, never re-interview** (the grilling *is* the interview and it already happened — a session that writes a spec by re-asking the questions is running grilling twice); **behavioural, not procedural** (interfaces, types and contracts — no file paths, no line numbers, no code, with one exception for a prototype snippet that encodes a decision more precisely than prose can: a state machine, a schema, a type shape); and **speak the project's language** (`CONTEXT.md`'s vocabulary; respect the ADRs in the area). Upstream `to-spec`'s mandated "LONG, extremely extensive" user-story list is optional here. `docs/adr/self-extension-core-and-methodology` has claimed grove ships a "CONTEXT / ADR / PRD format guide" set since it was written; the guide it named never existed, and that absence is the direct cause of five downstream documents carrying three naming conventions.
- **Test-seam sketching as a grilling move.** `content/grilling.md` gains *Agree the test seams*: when the increment covers code that will be tested, sketch the seams the work will be tested through and put them to the human before the design is committed. Prefer existing seams to new ones, propose new ones at the highest point you can, and drive the count down — the ideal number is one. The agreement is recorded in the spec's `## Test seams`, or, when the increment writes no spec (the common case), in the node's `BRIEF.md` — the brief chain is how a node's settled design reaches its child work leaves. `content/BRIEF-FORMAT.md`'s Pointers gains the corresponding optional item. What a seam *is* and how to judge one is owned by the **`linkuistics:codebase-design`** skill, which grove now names as a second reason the `linkuistics` plugin is a prerequisite (the first being `linkuistics:decision-records` for ADRs). Adapted from `mattpocock/skills@d574778` `to-spec` and `to-tickets`.
- **`grove-llm leaf-prune` — abandon a leaf, marked `ABANDONED` in place**, the pruning counterpart to `leaf-retire`'s `DONE`. Until now the only way to drop planned work that was decided against was a bare `git rm`, which lowers the permanent-key counter (`next_key` is `max key in tree + 1`) and lets a later `leaf-add` re-issue an already-spoken-for key — silently repointing every durable reference to it (issue #2). Marking in place instead keeps the key visible forever, exactly as `DONE` already does. `pick`, `resolve`, and `next_key` treat `ABANDONED` as non-live alongside `DONE`; `resolve` on a pruned entry now reports it as abandoned rather than looking identical to a live leaf. Given a **node**, `leaf-prune` bulk-marks every live leaf in its subtree in one call — an atomic two-phase walk that validates the whole subtree before mutating any of it, so a mid-walk failure is a clean no-op rather than a half-pruned tree — leaves `DONE` leaves untouched, and refuses the grove root (abandoning a whole workstream is a branch-delete, not a tree mark). Pruning is HITL only: the methodology (`content/SKILL.md`, `TASK-FORMAT.md`, `driving.md`) states plainly that an agent never prunes on its own, and that the durable *why* a path was rejected belongs in the ADR set, not in `.grove/`, since the task tree is deleted at the finish cycle. (ADR `pruning`)

### Removed

- **grove's two dead specs.** `docs/specs/2026-07-04-adr-disposition.md` (an executed keep/delete/merge table, self-described as "the executable input for `corpus-rework-k6`" — a brief's job) and `docs/specs/2026-07-04-adr-minimum-coherent-set-design.md` (a design seed whose content is now in `linkuistics:decision-records`, `content/ADR-FORMAT.md`, `content/SKILL.md` and `content/driving.md`). Neither was cited by any file; both are in git. This is the new rule applied to the repo that wrote it.

## v9.1.0

This release adds per-kind model selection to the self-driving loop: `grove do` launches planning and work sessions on different models via Claude Code's native `--model`. Additive and backward compatible — with neither new env var set, grove launches exactly as in v9.0.0 (no `--model`).

### Added

- **Per-kind model selection — planning and work leaves launch on different models.** Before each task launch the self-driving loop peeks the next live leaf's kind (`planning` vs `work`) and starts its `claude` session on the model named by `GROVE_PLANNING_MODEL` or `GROVE_WORK_MODEL` — so a grove can run planning (grilling, design) on a stronger reasoning model and mechanical work (code, docs, tests) on a cheaper/faster one. Selection uses Claude Code's **native `--model` flag** on the same subscription — no router, no proxy (a multi-provider proxy was rejected: it needs an API key and breaks or drains Max billing, whereas native `--model` does Opus↔Sonnet routing on the subscription for free). The kind is re-derived from the filesystem every iteration, so the loop stays stateless and restart ≡ continuation is preserved (`self-driving-loop`). Two load-bearing rules: an **unset variable ⇒ no `--model`** for that kind — the session inherits the user's own `ANTHROPIC_MODEL`/settings default, so grove is a no-op until you opt in, never clobbers a default you already have, and setting only one variable is fine (the other kind still inherits); and **the launch model is a default, not a lock** — an in-session `/model` switch overrides it but does *not* persist across relaunch, since the next task is a fresh session re-keyed on its own kind. The brand-new-grove `start` path is planning by construction (its first leaf is always planning, `fresh-grove-start-contract`), so it uses `GROVE_PLANNING_MODEL` unconditionally. Configured via `grove do --help`, README (`## Configuration`), and CONTEXT.md. (ADR `model-per-task-kind`)
- **`grove-llm kind [<leaf>]` — resolve a leaf's task kind from its task file.** A new verb on the `grove-llm` surface that reads the `**Kind:**` line via `leaf::Kind::parse` and prints a single lowercase token (`planning` or `work`). With no argument it resolves `pick`'s next live leaf; on a done grove it prints the standard "no live leaves" diagnostic on stderr and exits 0 (mirroring `brief-chain`). This is the peek the loop driver uses to select each launch model; `pick`'s bare-path output is unchanged.

## v9.0.0

This release reworks grove's decision-record methodology and prunes grove's own ADR corpus to a minimum coherent set. The CLI surface and task-tree behaviour are unchanged; the entire delta is in the embedded methodology (`content/`, extracted to `~/.claude/skills/grove/`) and grove's own `docs/adr/`.

### Breaking

- **The ADR philosophy is no longer bundled — grove now names the `linkuistics` plugin as a prerequisite.** Through v8 grove shipped a self-contained ADR guide (`content/ADR-FORMAT.md`, bundled from `mattpocock/skills`) carrying the format, the template, and the when-to-write test. That guidance now lives in the **`linkuistics:decision-records`** skill, and `ADR-FORMAT.md` shrinks to a grove-specific **placement** note (where ADRs live — slug-named `docs/adr/<slug>.md` — and multi-context placement under a root `CONTEXT-MAP.md`). `content/SKILL.md` now declares the `linkuistics` plugin a **documented prerequisite** for raising or reworking ADRs. The dependency is documentation-level, not install-enforced, and everything else grove needs stays self-contained (constraint 6) — but the long-standing "ADR guidance travels inside grove" guarantee is deliberately dropped. (`self-extension-core-and-methodology`)

### Changed

- **ADRs are a slug-named minimum coherent set reworked in place — not an append-only numbered log.** Records move from sequential `docs/adr/NNNN-slug.md` to **`docs/adr/<slug>.md`**; the slug *is* the record's identity and is cited by slug/title, never by number. The methodology now treats `docs/adr/` as a **minimum coherent set describing the current design**: when a decision changes, the set is reworked *in place* — merge / split / delete — and the briefs and ADRs that cite it are reconciled, rather than a superseding ADR being appended (git holds the history). Both the planning **Discover** step (grilling) and the **Retire** step of the loop gain an explicit ADR-set-reconciliation moment. (`task-tree-scheme`, deferring to `linkuistics:decision-records`)
- **grove's own `docs/adr/` reduced from 35 numbered records to 7 slug-named survivors.** The obsolete TUI/multiplexer/inbox tower — the `0013`–`0030` trellis/rmux ADRs and the `grove-meta` inbox-model records — described machinery already shed across the v6→v8 refactors and no longer reflected the current design; it is deleted. The surviving coherent set is `cli-binary-split`, `do-is-sole-lifecycle-verb`, `fresh-grove-start-contract`, `in-session-finish-cycle`, `self-driving-loop`, `self-extension-core-and-methodology`, and `task-tree-scheme`.
- **All numeric ADR citations reconciled to stable slugs across code, tests, and content.** References such as `ADR-0032`, `ADR-0011`, and `ADR-0035` in `src/`, `tests/`, and `content/` are rewritten to their survivor slugs (`self-driving-loop`, `fresh-grove-start-contract`, `task-tree-scheme`), so citations remain valid under the number-free slug scheme and survive future reorders.

## v6.2.1

### Fixed

- **`grove tui`'s native nav and whichkey surfaces no longer render blank.** The left `grove-nav` fleet list and the bottom `grove-whichkey` hint bar are `PaneId::Host` panes — trellis's third pane kind, which supplies its cells by drawing into an off-screen ratatui `Buffer` that the server converts to `CharacterChunk`s in `Pane::render` (exactly like a WASM plugin pane; terminal panes use a separate grid path). But both of trellis's pane-render loops gated the call that actually invokes `Pane::render` and feeds its chunks to the client — `render_pane_contents_for_client` — to `PaneId::Plugin` alone. Host panes therefore had their frame drawn but `render` never called, so every host surface composited to nothing. The companion `add_pane_contents` dispatch had already been widened to `Plugin | Host` when the host pane kind was added; the parallel render gate was the one site missed. Both gates (tiled and floating) now match `Plugin | Host`, and a new end-to-end regression test drives a host surface through `inject_host_pane → Tab::render → Output::serialize` and asserts its content reaches the composited output — the guard whose absence let the surfaces ship blank.

## v6.2.0

### Breaking

- **`grove tui` is resolved purely from config — the cwd git-repo anchor is gone.** The dashboard is now driven entirely by `fleet.toml` (`repos` + `scan_roots`) plus additive, repeatable `--repo <path>` flags; no cwd git root is detected or auto-included. `grove tui` runs from **any** directory — including a non-git one — and the pre-launch `not in a git repo (cwd: …)` error is gone (previously the gate fired before the fleet was even built, so a `scan_roots` manifest couldn't be reached from outside a repo). The cost is that the zero-config "stand in a repo → see its groves" convenience is removed: standing in a repo with no manifest now shows the empty-state, not that repo's groves. The deliberate replacements are a one-line manifest (`repos = ["."]` or a `scan_roots` entry) or **`grove tui --repo .`** to pin the current directory. An empty fleet (no manifest, no scan hits, no `--repo`) still launches the TUI, to an in-nav empty-state pointing at `~/.config/grove/fleet.toml` and `--repo` — no precondition branch is reintroduced. The fleet TUI is now a **singleton `grove-fleet` session**: a second `grove tui` re-attaches it rather than spawning one session per launch directory. The `grove tui` argument changes from a single positional `[<repo>]` to repeatable `--repo` flags. (ADR-0027)

## v6.1.0

### Breaking

- **trellis is the only TUI — the legacy in-terminal dashboard and the `--local` flag are gone.** v6.0.0 shipped the native trellis dashboard but left the pre-v6 in-terminal ratatui dashboard behind a hidden `grove tui --local` escape hatch, gated by an *off-by-default* `trellis-seam` build feature. Because nothing in the release path turned that feature on, the **shipped binary actually built without trellis and always fell back to the local, single-repo dashboard that ignores `~/.config/grove/fleet.toml`** — the fleet view never reached users. This release removes the `trellis-seam` feature (the `trellis` crates are now unconditional dependencies, so a default `cargo build` is trellis-capable), deletes the in-terminal `tui::run` event loop, and removes the `--local` flag (`grove tui --local` is now an unknown-flag error). `grove tui` launches the native trellis dashboard, unconditionally. (ADR-0026)

### Changed

- **The trellis TUI no longer reads the user's zellij configuration.** grove embeds a vendored zellij fork (trellis), not zellij, but the trellis client still resolved its config *base* from the user's zellij locations — `$ZELLIJ_CONFIG_DIR`, `$XDG_CONFIG_HOME/zellij`, `~/.config/zellij/config.kdl`, plus the user theme/layout dirs — and merged grove's config on top, so a user's stray `~/.config/zellij` could perturb grove's dashboard (and grove would even `mkdir` that directory). The fork's `find_default_config_dir` — the single chokepoint all of those sources funnel through — now returns `None` unconditionally: grove's config is trellis's built-in defaults plus grove's in-process `GROVE_TUI_CONFIG` only. A user with a populated `~/.config/zellij` now sees identical grove behaviour to one with none, and grove never creates that directory.

### Removed

- **The obsolete `grove-nav` WASM plugin.** The nav became a native in-process surface during the v6 fork (ADR-0020); the WASM plugin's embed site was already gone (nothing did `include_bytes!` on it), leaving the `crates/grove-nav` crate and its `build.rs` compile step as dead weight that also broke `cargo build` inside nested worktrees. Both are deleted; the `"grove-nav"` name now denotes only the native nav *pane* in the layout. As a result, **no `wasm32-wasip1` target is needed to build grove**.

## v6.0.0

This release rebuilds grove's TUI on its own multiplexer. The CLI and methodology surface (`grove`, `grove-llm`, the `.grove/` task tree, durable markdown artifacts) is unchanged; the entire delta is the dashboard substrate and the multi-repo/embedding capabilities it unlocks.

### Breaking

- **grove's TUI is now a native, in-process multiplexer; it no longer drives an installed zellij from outside.** Through v5 the dashboard shelled out to a stock zellij and compensated for living *outside* it with a dumb-terminal proxy seam (ADR-0016), a WASM nav plugin (ADR-0018), and a reply-only back-channel (ADR-0019). v6 hard-forks zellij 0.44.3 into **`trellis`** — grove's own TUI framework, vendored under `crates/trellis` — and compiles grove's dashboard in **natively**: grove owns `main`, links the forked `zellij-*` crates, and starts the trellis client in-process (ADR-0020/0021). All three indirection layers evaporate. Consequences for users: **the TUI is no longer walk-away-able** — deleting grove no longer leaves a stock zellij behind (the CLI/methodology core *remains* walk-away, since durable artifacts are still standard markdown), there is **no dependency on a separately-installed zellij**, and the binary is larger. `trellis` is a hard fork with **no upstream-rebase cadence** — upstream zellij is watched only as a CVE/advisory feed and relevant fixes are hand-patched. zellij's MIT license and copyright are preserved under `content/LICENSES/zellij.LICENSE` and `crates/trellis/`.
- **`grove tui` renders the native dashboard by default; the legacy in-terminal dashboard moves behind `grove tui --local`.** The pre-v6 in-terminal ratatui dashboard (no trellis, no embedding) is retained only as a hidden dev/debug escape hatch (`--local`). The supported path is the native trellis pane. Migration: nothing for normal use — `grove tui` still launches the dashboard; scripts that depended on the old in-terminal rendering should pass `--local`.

### Added

- **Fleet view — one grove process surfacing groves across many repositories.** The dashboard is no longer single-repo. Fleet membership is resolved from a hand-editable XDG manifest at `$XDG_CONFIG_HOME/grove/fleet.toml` (falling back to `~/.config/grove/fleet.toml`) with two keys: `repos` (explicit repo roots, always included) and `scan_roots` (directories grove walks to discover repos containing a `.grove-worktrees/`). `--repo <path>` flags layer additively, the current repo (cwd's git root) is always included, and repos reached by more than one route dedup by canonical path (ADR-0025). With no manifest and no flags the fleet is just `[current repo]`, so existing single-repo behaviour is preserved with zero config. Navigation is grouped two-level (repo → grove) with cross-repo, repo-qualified working-set keys, fleet-scale fs-watch, and interactive filtering.
- **The working set — a grove's harness plus its embedded tools, switched as a unit.** Selecting a grove in the nav swaps that grove's *working set* into a constant content region beside an always-on-screen nav (ADR-0022): the agent harness terminal alongside auxiliary tool panes (a shell, `yazi`, a VCS/lazygit view), each toggleable, with a responsive layout that picks a default-visible set by breakpoint. Non-selected harness ptys keep running and capturing scrollback — switching groves parks the current panes (alive, off-screen) and mounts the selected grove's, never suspending a harness. `trellis`'s headline capability is seamlessly embedding *other* fully-emulated TUI apps as first-class regions, with first-class observability of those wrapped tools.
- **Native navigation, per-grove detail, and whichkey hint bar.** A constant full-height nav surface focused by a `Ctrl-o` leader (no "home tab" to travel to); per-grove detail mounted beside the harness via an in-place content-swap substrate (ADR-0023); a native `$EDITOR` drop driven by embedded-tool exit observability (ADR-0024); and a single grove-owned full-width whichkey hint bar. The pre-v6 tab-per-grove model (`GoToTab`/`Alt-1..9` switching) is retired in favour of nav-driven content swapping.

## v5.1.0

### Added

- **`grove-llm root-init [<slug>]` — scaffold a brand-new grove's tree.** A fresh grove (worktree + branch exist, no `.grove/` yet) had no bootstrap path: `grove-llm pick` errored `grove root not found` and no verb could create the root. `root-init` creates `.grove/`, the root `BRIEF.md` stub, and a first **planning** leaf `010-<slug>.md` (default slug `plan`), so `pick` immediately returns work and the grove drops into the steady-state loop. Working-tree change only, no commit; refuses to clobber an existing `.grove/`. Creating the first leaf — not just the brief — is load-bearing: a brief-only `.grove/` reports "no live leaves; this grove is done" and would mis-trigger the finish cycle, leaving a fresh grove indistinguishable from a finished one (ADR-0011). The `start.md` launcher prompt and `content/SKILL.md` now name `root-init` as the first step of a fresh grove.
- **`grove-llm inbox-remove --for=<name>` — finish-cycle inbox cleanup.** The complete finish cycle tore down a grove's worktree and branch but orphaned its `grove-meta` inbox, so a *finished* grove kept showing up as a **Seed** in `grove status` / the TUI. The finish cycle gains a step that removes `inboxes/<name>/` via this verb. It refuses-and-instructs while observations are still pending (drain first) rather than silently discarding work another grove may have captured since the session's bootstrap drain, and is an idempotent no-op when the inbox is absent — so the state-checked finish resume needs no marker file. `CONTEXT.md`'s **Seed** definition no longer counts a finished grove's inbox; a still-orphaned inbox now signals an *incomplete* finish (ADR-0012).

## v5.0.0

### Breaking

- **`grove start`, `grove continue`, and `grove finish` removed; `grove do` is the sole lifecycle entry verb.** `grove do` already subsumed start/continue (no grove by that name → create the worktree and open a bootstrap session; live worktree → continue; branch present but worktree gone → re-attach and continue), so both were strictly redundant (ADR-0009). `grove finish` is removed too: finishing a grove is now an **in-session** step — when the grove has no live leaves left, the running loop proposes the complete finish cycle (promote durable artifacts → delete `.grove/` in a focused commit → `git -C <repo> merge <name>` → remove the worktree → delete the branch; single confirmation gate, propose-and-wait so headless runs report rather than act, and state-checked resume with no marker file — step-level design in ADR-0010). Migration: replace `grove start <name>` / `grove continue <name>` / `grove finish <name>` with `grove do <name>`. The `--start-point <ref>` flag, formerly on `grove start`, now lives on `grove do` and applies on the new-grove path. Trade-off: there is no longer a way to force-finish a grove that still has live leaves — retire or clear the leaves first.

## v4.0.0

### Breaking

- **`grove list` removed.** Its output (grove names, one per line) is a subset of `grove status`, now the canonical visibility surface (ADR-0007). Migration: parse `grove status` instead of `grove list`.
- **`grove version` removed.** Its output (CLI version + per-harness installed version) is subsumed by `grove status`. Migration: use `grove --version` for the CLI version alone, or `grove status` for the full cli/repo/worktree picture.
- **`grove update` removed; `grove install` is now idempotent** (ADR-0008). One verb converges on the bundled version from any starting state: not installed → install; same version → no-op (no empty commit); different version → update. It always prints a per-harness outcome line — `installed @ X`, `already at X, no change`, or `updated X → Y` — making the result explicit and safe to rely on in CI/setup scripts. There is no `--update` / `--force` flag and no deprecated `grove update` alias. Migration: replace `grove update` with `grove install` (add `--version <tag>` to pin). The default commit subject is still `Install grove v<ver>` for a fresh install and `Update grove to v<ver>` when refreshing an existing one. The stored `VERSION.md` stamp is now canonical (no leading `v`); the git fetch ref is unchanged.

## v2.2.0

- `**Retire.**` doctrine in `content/SKILL.md` is now imperative and procedural: after committing a task, the session mvs the just-finished leaf into `.grove/done/` (mechanical, no ask), then walks the parent chain. If a node has no live leaves left, the session **asks the user** before retiring it — the confirmation gives them a moment to add a follow-up leaf — then promotes any still-relevant brief content upward and `mv`s the node into `.grove/done/`. The cascade recurses through ancestors until a node still has live leaves or the grove root is reached. The inner-loop mermaid graph and the `multi-step.md` walkthrough are updated to match.

## v2.1.0

- `grove install` and `grove update` now produce a single path-scoped git commit covering every targeted harness path (per ADR-0001). `--no-commit` opts out and prints the staging command; `-m`/`--message` overrides the default message. Pre-flight refuses if install-scope paths already have staged hunks; unrelated dirty state elsewhere is left alone. Hook failures leave the materialisation in place and print a follow-up `git commit -- <paths>`. Multi-harness invocations produce one combined commit; no-op materialisations skip the commit.
- New per-flow walkthroughs under `docs/workflows/` (install, update, start, multi-step, finish) with an index; README and `docs/grove.md` cross-link to them.
- Documentation clarifies that `grove continue` is a session launcher and notes that up-arrow history recall surfaces the last continue prompt.

## v2.0.0

Breaking on-disk layout change. Every storage location is now dot-prefixed and the per-grove namespace is gone where it was redundant.

- Task tree: `groves/<name>/` → `.grove/` (inside the grove's worktree). One worktree = one grove, so the name no longer needs to namespace the task tree.
- Worktree: `worktrees/<name>-grove/` on branch `<name>-grove` → `.grove-worktrees/<name>/` on branch `<name>`.
- Harness stamp: `groves/<name>/.harness` → `.grove-stamps/<name>`.
- `grove finish` now explicitly deletes `.grove/` in a focused commit before merging, so the default branch never carries any grove's local state. The history of completed groves lives in git's commit graph, not in retained `done/` directories.
- `grove uninstall`'s "live groves" check is now "any worktree exists in `.grove-worktrees/`" — simpler and authoritative.

Migration: existing groves on v1.x layout need manual relocation (`mv groves/<name> .grove-worktrees/<name>-grove/.grove`, then rebranch, then refresh content with `grove update`). New repos pick up the new layout automatically.

## v1.0.1

- Relicense from MIT to Apache-2.0 (matches sibling Linkuistics projects); add the missing LICENSE file at repo root.
- Add `docs/grove.md` — project-level intro covering the methodology rationale and the CLI's workstream verbs.

## v1.0.0

- Initial public release of the grove CLI.
- Lifecycle verbs: `install`, `update`, `uninstall`, `version`, `status`, `list`.
- Launcher verbs: `start`, `continue`, `takeover`, `retire`, `finish`.
- Multi-harness support with auto-detection of `.claude/` and `.codex/`; `.harness` stamp used as a per-grove disambiguator.
- Release pipeline producing macOS arm64 and Linux x86_64/arm64 binaries.

## The `Linkuistics/skills` changelog, carried in verbatim

The skill plugins under `plugins/` were developed in a separate repo,
`Linkuistics/skills`, until its history was grafted in here (*skills-monorepo*).
That repo's changelog is preserved below exactly as it stood at the graft: one
never-versioned `Unreleased` section, newest entry first. It is a closed record —
nothing new is appended to it.

- Added the `using-jujutsu` and `git-to-jj-mapping` skills. `using-jujutsu`
  auto-fires on version-control work: in a jj-enabled repo (a `.jj/` directory
  exists) it drives everything through Jujutsu's native model
  (working-copy-as-commit, `jj new`/`jj describe`, bookmarks, op-log undo);
  otherwise git remains the interface, silently — the skills never convert a
  repo or offer to. `git-to-jj-mapping` is the on-demand git→jj command and
  concept reference, loaded only when a translation is needed. Reconciled the
  existing skills with the jj design: `guardrail` now also gates `jj abandon`
  and `jj op restore` (hook pattern list, SKILL.md table, and test suite; jj
  0.43 has no force-push flag to gate — pushes are lease-checked by default,
  pinned by a defer test), and `decision-records` generalises its "git holds
  the past/history" phrasing to "the VCS holds …" at all six sites. Updated
  the `linkuistics` manifest description/keywords and the README skills table.
- `authoring-conventions`: added **Negation** (steering by prohibition drags the
  forbidden behaviour into context and makes it more available, not less; state the
  positive target, reserve `never`/`don't` for guardrails that can't be phrased
  positively), the **context load / cognitive load** vocabulary for the user-invoked vs
  model-invoked lever plus the **router skill** cure for cognitive-load pile-up, and a
  **sentence-level no-op hunt** (test each sentence against the no-skill default; delete
  failing sentences outright rather than trim words). All three are drawn from
  `mattpocock/skills`' `writing-great-skills` skill (MIT), which postdates this repo's
  prior-art survey. `codebase-design`: added the concrete parallel-sub-agent "design it
  twice" procedure (divergent per-agent briefs: minimize-interface / maximize-flexibility
  / optimize-common-caller / ports-and-adapters), from the same upstream's
  `DESIGN-IT-TWICE.md`. Refreshed the prior-art survey's mattpocock citations with a dated
  note pointing at `writing-great-skills/{SKILL,GLOSSARY}.md` @ `d574778` as the current
  canonical source.
- Added the `decision-records` skill — ADRs as a **minimum coherent set**
  describing the design's current state (current-state over changelog,
  edit/merge/split/delete in place, identity by slug not number, the
  when-to-write test, a minimal template). The minimal template, the three-part
  when-to-write test, and the qualifying examples are distilled from
  `mattpocock/skills`' ADR-format material (MIT); the coherent-set framing is
  original. Updated the `linkuistics` manifest description/keywords and the
  README skills table.
- grove moved to its own repo (`Linkuistics/grove`) and is now distributed via
  `brew tap Linkuistics/taps && brew install grove`. The `grove/` directory and
  `scripts/materialise-grove.sh` were removed from this repo; `docs/grove.md`
  now points readers to the new repo.
- Added `grove-start` and `grove-next` shell launchers, bundled with the
  grove skill and materialised alongside it. They collapse the per-session
  restart ritual (`/clear` → `/rename` → kickoff prompt) into a single
  command: `grove-start <name>` creates the worktree at
  `<repo>/worktrees/<name>-grove/` on branch `<name>-grove` and launches a
  pre-named bootstrap session; `grove-next <name>` cd's into the worktree
  and launches a pre-named continuation session. Both work from anywhere
  inside the repo (any worktree) via `git rev-parse --git-common-dir`.
  Updated SKILL.md, docs/grove.md, and the materialise test to match.
- Moved the grove skill out of the `linkuistics` plugin tree
  (`plugins/linkuistics/skills/grove/` → `grove/`) so installing the plugin
  no longer ships a global grove. Grove is materialisation-only — a global
  installed copy would conflict with the per-project pinned copy and
  re-introduce the version-drift problem grove exists to prevent. Updated
  `scripts/materialise-grove.sh`, its test, `README.md`, and `docs/grove.md`.
- Renamed the repo to `Linkuistics/skills` and the plugin to `linkuistics`
  (namespace `linkuistics:`); added an Apache-2.0 licence.
- Added the `grove` skill — a methodology for hierarchical, self-extending,
  git-tracked task-tree workstreams. Bundles three convention files from
  `mattpocock/skills` (MIT); upstream licence in `grove/LICENSES/`.
- Added `scripts/materialise-grove.sh` — copies grove into a consuming repo
  and stamps `VERSION.md`.
- Added `docs/grove.md` — problem, solution, install/update guidance, and example
  prompts. Restructured `README.md` to lead with grove as a top-level section
  alongside the coding-style skills.
- Made grove's heritage explicit up front in `docs/grove.md` and the README's
  `## grove` section: bundling of Matt Pocock's `grill-with-docs` conventions
  and DDD's Ubiquitous Language and bounded-context concepts.
- Made grove's single-worktree-per-grove convention explicit in `SKILL.md`
  (loop preamble) and `docs/grove.md` (the "Git worktrees" subsection now
  reads as directive rather than descriptive).
- Made grove name the session: SKILL.md now instructs the LLM to suggest
  `/rename <project>: <grove-name> grove` on the first turn of grove
  activity, once per session.
- Elevated grilling in `SKILL.md`: a planning task now **opens** with a
  grilling session via `grilling.md`, rather than listing "grills" as one of
  several planning-task activities. Matched in `docs/grove.md` section 2 and
  the "Run a planning task explicitly" prompt.
- Initial release. Coding standards packaged as agent skills:
  - `coding-style` — universal principles (auto-loads on any file).
  - `coding-style-{rust,python,elixir,bash,swift,typescript}` — per-language
    style guides, auto-loading by file extension.
  - `cli-tool-design` — LLM-friendly CLI design guidance, with the audit
    checklist and refactoring sequence split into `references/`.
- Claude Code marketplace manifest (`.claude-plugin/marketplace.json`).
- `install.sh` for symlinking skills into Codex / Gemini CLI.
