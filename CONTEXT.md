# grove

The grove CLI and methodology. This glossary holds terms specific to this codebase; general programming concepts are excluded by design.

## Language

**Install scope**:
The bounded set of paths a single `grove install` invocation modifies — the union of `harness.install_path(repo)` over the harnesses targeted in that invocation. grove's auto-commit stages and commits exactly this set, leaving unrelated dirty state alone.

**Path-scoped commit**:
A commit constructed via explicit `git add <paths>` and `git commit -- <paths>` rather than via whatever happens to be in the index. The technique grove's `install` uses so the auto-commit cannot sweep up unrelated work-in-progress.

**Lifecycle walkthrough**:
A per-verb prose document under `docs/workflows/` that shows the command(s) for one grove flow (install, start, multi-step, finish), explains what happens, and shows what changed in the repo or worktree at each step. Distinct from the CLI reference in `README.md` (which lists flags) and from the methodology doc `docs/grove.md` (which is "what grove is and why"). Avoid the aliases *tutorial*, *guide*, and *runbook* in this codebase.

**Inbox**:
A directory accumulating observations addressed to a named grove, living at `inboxes/<name>/` on the [[grove-meta branch]]. Each observation is a single markdown file at `inboxes/<name>/<UTC-iso8601-seconds>Z-<slug>-<content-hash-8>.md`; the directory's existence (preserved by a `.gitkeep`) is the "this grove is known" signal even when no observations are pending. Observations are written by the LLM working in any grove that notices something belonging elsewhere, via `grove-llm inbox-add` (never direct file edits; see ADR-0006 for the LLM-binary split). The inbox is addressed to the workstream, not to a person. A running grove [[drain]]s its own inbox at every `grove do` (the sole lifecycle entry verb). An inbox whose addressed grove does not currently exist as a worktree is called a [[seed]]. The shape moved from "one file per grove" to "one directory per grove, one file per observation" in ADR-0004 to escape the merge-conflict pathology of single-file multi-writer (see `docs/research/in-repo-issue-tracker-postmortems.md`).

**Seed**:
An [[inbox]] whose addressed grove does not *yet* exist as a worktree — a workstream noticed but not started. Same artifact and same path as a running grove's inbox; only the lifecycle state differs. A *finished* grove is **not** a seed: the [[Complete finish cycle]] removes its inbox (`grove-llm inbox-remove`, ADR-0012) as a teardown step, so a retired grove leaves no `inboxes/<name>/` behind to be mistaken for a not-yet-started one. A still-orphaned inbox (worktree gone but the grove never properly finished) therefore signals an *incomplete* finish, not a dead grove. The seed's name is the working title of the addressed grove (e.g. `racket-bugs-discovered-while-implementing-chez`); it may later be renamed, narrowed, or folded into a catchall seed. When `grove do <name>` first runs against that name, the seed simply becomes the new running grove's inbox — there is no separate consumption step. A seed is distinct from a brief note (in-scope context for the current node, retired with it), from an ADR (a decision, not an observation), and from a TODO comment in code (local to one file, not a workstream candidate). Avoid the alias *issue* — it collides with GitHub Issues and with bug-tracker connotations.

**grove-meta branch**:
A dedicated branch in each repo, materialised as a sibling worktree at `<repo>/.grove-meta/`, holding shared cross-grove coordination data and repo-level metadata that does not belong on the default branch. The primary occupants today are [[inbox]] directories at `inboxes/<name>/`; the branch is reserved more broadly as the home for any cross-grove coordination artifact or repo-level metadata (grove-related or otherwise) that future work surfaces. All reads and writes go through `grove` CLI subcommands rather than direct git/filesystem operations, so the LLM does not interact with the branch's git plumbing. Cross-repo inbox writes follow the same rule against another repo's `.grove-meta/` worktree, and require that worktree to be present locally. The branch is created (and the worktree attached) by `grove meta init` — invoked explicitly for repos that pre-date the feature or to repair a missing worktree, and implicitly during `grove install`. Remote-sync is **opt-in**: by default the branch has no upstream and lives local-only. Multi-machine users add a remote with `grove meta remote add <url>` (which sets upstream tracking); thereafter `grove do` fetches-before-drain and `grove-llm inbox-add` / `grove-llm inbox-drain` push-after-commit, both best-effort with one auto-retry on non-ff (ADR-0005). The previously-considered name `grove-inboxes` was rejected because it narrowed the branch's scope to its first occupant.

**Drain**:
The session-bootstrap triage of a running grove's [[inbox]], performed at every `grove do` (the sole lifecycle entry verb). If the `grove-meta` branch has a remote configured, drain fetches it first (soft-on-failure — warn-and-continue if offline; refuse-and-instruct on non-ff). For each pending observation the LLM proposes incorporating it into the current task, deferring it to a later leaf, or rejecting it as out-of-scope (and possibly seeding a different grove). After triage the triaged observation files are deleted in one session-commit; drained observations live in git history (`git log inboxes/<name>/`). The `.gitkeep` stays so the directory's existence remains the "known grove" signal even when no observations are pending.

**cli version** / **repo version** / **worktree version**:
The three locations at which a grove-methodology version stamp can live, and which any pair can drift between. **cli version** is the installed `grove` binary's own version (`env!("CARGO_PKG_VERSION")`, the methodology bundled inside the binary). **repo version** is the version stamped in `<repo>/.<harness>/skills/grove/VERSION.md` — one per installed harness, written by `grove install`. **worktree version** is the version stamped in `<repo>/.grove-worktrees/<name>/.<harness>/skills/grove/VERSION.md` — one per worktree per harness, written when `grove do` first materialises the skill into a new worktree. The three are independent because `grove install` only touches the repo's install path and `grove do` (on the new-grove path) only touches the new worktree's install path; the binary the user runs `grove status` with is a third axis entirely. Surfacing all three (and drift between them) is the job of `grove status` and the TUI — the canonical visibility surface (ADR-0007). The former `grove list` and `grove version`, whose output `grove status` subsumes, were removed in v4.0.0; `grove --version` still answers the CLI-only question. **Drift rule:** plain raw string equality (`==`), both stamps shown verbatim on mismatch, no semver interpretation and no `v`-prefix normalisation. From v4 onward `grove install` stores the **canonical** stamp (the incoming git tag with its leading `v` stripped — the canonicalisation lives on the install/write side, see the install leaf), and the **cli version** (`CARGO_PKG_VERSION`) is already canonical, so every stamp in play is canonical. Any stamp still bearing a `v` is therefore necessarily a pre-v4 release — genuinely older than the running code — and `!=` flagging it as drift is the correct verdict, not a false positive (the binary only ships under a new version number, so there is no scenario where a canonical and a `v`-prefixed stamp name the *same* release). A missing/unreadable worktree `VERSION.md` renders `(unknown)` and is never drift; an orphan worktree (harness no longer installed in the repo) shows `repo=(none)` informationally without a warning. `status::worktree_version` is the shared per-worktree-per-harness reader; the comparison is plain `==` so every surface (incl. the TUI) applies it directly.

**Global skill provisioning** / **skill precedence**:
The `grove` binary embeds `content/` and extracts it to the **personal** global skill dir `~/.claude/skills/grove/` on every `grove do` — idempotent against a content-hash stamp (ADR-0034). `content/` stays canonical; the binary is the only writer of the global skill. Claude Code skill precedence is **enterprise > personal > project** (Claude Code docs): a **personal** skill *overrides* a same-named **project** skill (`<repo>/.claude/skills/grove/`), so the binary-provisioned global copy always wins. A leftover project mirror is therefore **dead, shadowed code** — not an active override — and is removed (070/050) so it cannot mislead a contributor into editing it; no new project mirror should be re-added (edit `content/` and rebuild instead). This supersedes the old fetch+materialise model and its [[cli version]] / [[repo version]] / [[worktree version]] `VERSION.md` drift (whose code 090 deletes). Note: the 070/050 leaf's prose had the shadow direction backwards (it read "project shadows personal"); the docs are unambiguous that personal wins.

**Complete finish cycle**:
The terminal, whole-grove sequence that retires a grove once it has no live leaves left: (1) promote durable artifacts from the briefs (ADRs, docs, glossary); (2) delete `.grove/` in a focused commit; (3) merge the branch into the default branch (`git -C <repo> merge <name>` — fast-forwards or makes a merge commit); (4) clean up the inbox — drain any pending observations then `grove-llm inbox-remove --for=<name>` to delete `inboxes/<name>/`, so the finished grove stops showing as a [[Seed]] (ADR-0012; refuses while observations are pending, no-op when never seeded); (5) remove the worktree; (6) delete the branch. Steps 3, 5 and 6 run against the main repo with `git -C <repo>`; step 4 writes to the `grove-meta` worktree and runs before step 5 while cwd is still valid; worktree-remove precedes branch-delete (git refuses `branch -d` on a branch checked out in a live worktree). Driven by the in-session LLM, not Rust automation (ADR-0010); proposed and executed only on explicit human confirmation, so a headless run with no human reports the plan and stops; resumed from git/filesystem state with no progress marker (constraint 1). Triggered whenever `grove-llm pick` reports no live leaves — or errors because `.grove/` is already gone, the partial-finish resume case. Distinct from [[Drain]] (a per-session bootstrap step) and from the per-leaf Retire step (which archives one leaf into `done/`); the finish cycle is what retiring the *last* leaf leads into. The former `grove finish` verb that launched this was removed (ADR-0009): finishing is a step of the loop, not a launched verb.

**root-init** / **fresh-grove start**:
The bootstrap of a brand-new grove (worktree + branch exist, but no `.grove/` tree yet), enacted by the `grove-llm root-init [<slug>]` verb. It creates `.grove/`, the root `BRIEF.md` stub, and a first **planning** leaf `010-<slug>.md` (default slug `plan`) — a working-tree change with no commit, refusing to clobber an existing `.grove/`. It is the one tree verb that sits *below* the floor the others stand on (`leaf-add`/`leaf-insert`/`leaf-decompose`/`leaf-retire` all require `.grove/` to already exist), so it is what makes a fresh grove enter the steady-state loop. Creating the first leaf — not just the root brief — is load-bearing: `grove-llm pick` skips every `BRIEF.md`, so a brief-only `.grove/` reports "no live leaves; this grove is done" and would mis-trigger the [[Complete finish cycle]] (ADR-0011). The first session's commit folds the scaffold in. Distinct from [[Bootstrap]]-the-loop-step (reading context at the start of every session); this is the one-time creation of the tree that the loop then reads.

**Bootstrap**:
The per-session context-loading step of the grove loop: read the glossary, the ancestor `BRIEF.md` chain, the cited ADRs, and the task file, then [[Drain]] the inbox. Read-only — no script must succeed before work begins. Not to be confused with [[root-init]] (the one-time scaffolding of a *new* grove's tree); bootstrap reads an existing tree, fresh-grove start creates one.

## TUI (rmux substrate)

Under **ADR-0028** the TUI substrate is **rmux**: grove is a ratatui app that **owns
its own draw loop** and embeds *foreign* terminal programs (the harness; the aux
term/yazi/vcs panes) as rmux **panes**, while grove's own surfaces (nav, detail,
capture modal, whichkey) are **ratatui widgets grove draws itself**. A separate rmux
daemon owns the ptys. This **inverts** the trellis era, in which grove compiled
*into* a zellij fork. The trellis/zellij-era terms that inversion dissolved —
*dashboard*, *dashboard proxy*, *seam frame*, *controlling process*, *head binary*,
*host surface / host driver / host tick*, *trellis framework*, *trellis hosting API*,
*owned zellij substrate*, *harness-pane crate* / *TerminalEmulator* / *PtySession* /
*dynamic mouse capture* / *pane-local copy mode*, and the WASM *nav plugin* — are
superseded; see ADR-0028 and git history. The live terms follow.

**Fleet** (multi-repo view):
The grove workstreams across **N repos** surfaced in one TUI process — concern 1
of the v2 work. The data layer is **`MultiRepoView`**, a collection of per-repo
`RepoView`s scanned concurrently below the [[presentation boundary]] (no
`ratatui`); a repo whose scan fails is **silently skipped**, never blocking the
others. **Single-repo is the N=1 "fleet of one"** — the nav renders one path, not
a single-vs-multi fork. The repos to span are resolved **purely from config**: a
**manifest + scan-roots hybrid** (ADR-0025: explicit `repos` always included,
`scan_roots` walked to discover more) plus additive `--repo` flags. **No cwd git
root is detected or auto-included** — ADR-0027 removed the cwd anchor entirely
(amending ADR-0025 §3), so `grove tui` runs from *any* directory and is driven
only by the config; an empty resolved fleet launches the TUI with an in-nav
empty-state, never a git-repo error. The fleet TUI is a **singleton rmux
session** (`grove-fleet`, persistent across `grove tui` restarts via `CreateOrReuse`
— ADR-0027/0030), since with no cwd anchor the session name is a constant.
The nav has two shapes: **grouped** (collapsible repo section headers → their
groves; the lone header auto-hides at N=1) when idle, and a **flat ranked list**
(no headers, repo shown as a per-row `<repo>/` prefix) whenever any filter
dimension is engaged — a fuzzy needle over `<repo>/<grove>`, an inbox-pending
toggle, a lifecycle cycle, or a sort toggle, **all ephemeral per session**
(constraint 1). fs-watch at fleet scale uses **one multi-root watcher** with a
`/.git/`-path filter and a prefix-matched **per-repo re-scan** (only the repo
owning the changed path is re-scanned). Selecting any grove opens its [[working
set]] in its *owning* repo (the harness path carries the repo explicitly), with
workspace keys **repo-qualified** so two repos' same-named groves don't
collide. Avoid the alias *workspace* for this — a [[workspace]] is one grove's
swappable working set, an orthogonal axis.

**Harness pane**:
The region showing one grove's **live, interactive harness session** — the
`grove do <name>` process (claude code / codex / other terminal tooling). Under the
inversion (ADR-0028) it is a **foreign rmux pane**: a program the rmux daemon runs in
a detached window, embedded into a grove-chosen `Rect` via `ratatui-rmux`'s
`PaneWidget` over a `PaneSnapshot`, driven/observed through `rmux-sdk`. grove does
**not** emulate it. Addressed by a stable `PaneId` via the app's `name → PaneDriver`
map (ADR-0028 E3). The aux term/yazi/vcs panes of the [[working set]] are embedded the
same way. Distinct from grove's own drawn surfaces ([[nav surface]],
[[detail widget]], capture modal, [[whichkey footer]]); the harness pane shows
someone else's program.

**Presentation boundary** (core↔presentation seam):
The architectural seam (ADR-0013) separating ratatui rendering (*above*) from grove's core logic — the `RepoView`/`MultiRepoView` data layer, harness driving, and shell-out writes (*below*). Code below the seam never imports `ratatui` types. Its purpose is optionality: a future web front-end becomes a *second presentation over the same core*, not a rewrite. **Under rmux (ADR-0028)** the boundary is a **literal directory wall** (E2): code under `src/tui/` may import `ratatui`/`ratatui_rmux`/`rmux_sdk`/`tokio`, code outside it may not — a review-time guard, not a `GroveBackend` trait (no second consumer yet). Async is confined to `src/tui/` (E1: the draw loop is `tokio::select!`, the sync core is called directly with `spawn_blocking` for slow writes), so the seam doubles as a runtime firewall.

**Workspace** (one grove's swappable working set):
One grove's [[working set]], switched as a unit — the term kept for "the set of
panes + widgets shown for one grove," an axis orthogonal to the [[fleet]]. Under the
inversion (ADR-0028) a workspace is **not** a zellij tab and **not** a content-region
swap: it is simply the layout grove draws for the [[focus]]ed grove (its
[[harness pane]] + [[detail widget]] + the visible aux panes). The earlier zellij-tab
(ADR-0018) and constant-nav + content-region (ADR-0022) models are superseded; the
one-grove-at-a-time property survives.

**Working set**:
The layout grove draws for one grove under the inversion (ADR-0028,
050-plan-rebuild/050-working-set): the [[harness pane]] (dominant left) + a **side
column** stacking the *visible* members vertically + the [[whichkey footer]] row. The
side column is a **heterogeneous stack** — the per-grove [[detail widget]]
(always-present top member, grove-drawn) + zero-or-more **aux panes** (plain term /
yazi / lazygit-lazyjj vcs) below it in fixed order. Aux panes are ordinary foreign
rmux panes embedded exactly like the [[harness pane]] — one **detached rmux window**
each, keyed by composite **(grove, role)**, rendered into a grove-chosen `Rect` via
`PaneWidget` (no rmux splits — grove owns the layout). **Park-alive is native and
free** (the ADR-0023/0024 machinery evaporates): a pane in a detached window stays
alive in the daemon whether or not grove draws it, so "off-screen" is just "not drawn
this frame" — no suppress/restore, no `replace_pane`. Aux panes **lazy-spawn on first
toggle** (leader `t`/`y`/`v`, cwd = worktree), **hide-not-close** on toggle-off, and
close only at TUI exit; **visibility is per-grove**, ephemeral per session.
**Responsive tiers collapse to geometry:** a single ~220-col breakpoint sets the
side-column width + per-member min-height — ADR-0022's "tier picks which panes mount"
rule dissolves (membership is now user-toggle + always-on detail). Built in
050-working-set/{010-pane-keying, 020-side-column-layout, 030-aux-panes}.

**Detail widget** (per-grove):
Per-grove **detail** as a **ratatui widget grove draws itself** from `RepoView`
(`src/tui/detail.rs`) — task tree + brief chain + inbox view — a focusable panel that
**coexists** beside the [[harness pane]] in a grove's [[working set]], reached via the
leader-dispatch gate (`leader → d`, see [[focus]]). Pure snapshot → `Buffer`,
headless-tested. Inbox interaction is **grooming** only (reject + move/re-route via
`grove-llm` below the [[presentation boundary]]); incorporate / defer-to-leaf stay at
the bootstrap [[Drain]]. Built in 050-plan-rebuild/{030-detail-widget,
040-detail-triage}. The trellis-era *dumb proxy* (`grove __dash-proxy`, the controller
socket seam, the `RunEditor` frame, N socket clients) is dissolved by ADR-0028 — see
git history.

**Whichkey footer**:
Not a surface and not a pane — one **footer line the `App` draws** (ADR-0028,
050-plan-rebuild/010-surfaces): the live **leader menu** when [[focus]] is
`LeaderPending` (`g nav · d detail · c capture · e editor · q quit · ⎋ cancel`), the
focused surface's hint line otherwise. ADR-0019's *single hint-owner* rule holds **by
construction** — one draw loop, one footer. Collapsed too far to earn its own leaf;
folded into 020-leader-dispatch. The trellis-era injected `grove-whichkey` host pane +
publish/subscribe `HostDriver` seam are dissolved by ADR-0028 — see git history.

**Leader** (control-seam key):
The single **grove-owned crossterm key** — **`Alt-g`** (configurable via `leader` in
`~/.config/grove/tui.toml`) — that reaches grove's control (ADR-0028 E4,
030-engine/020). grove owns the draw loop, so there is no zellij locked-mode: grove
sees every key first and arbitrates by [[focus]]. While a [[harness pane]] is focused
grove forwards *every* key to the pane except the leader, which opens the
leader-dispatch gate (`LeaderPending`; the next key → `g` nav / `d` detail / `c`
capture / `e` editor / `q` quit / `t`·`y`·`v` aux / ⎋ cancel). The earlier
`Ctrl-o`/zellij-locked-mode keybind model is superseded (ADR-0028).

**Focus** (rmux-substrate, ADR-0028 E4 + 050-plan-rebuild/010-surfaces):
grove's leader-gated input-arbitration state — `Pane | Detail | Nav | Modal(kind)`
plus a transient `LeaderPending` (`src/tui/focus.rs`). **`Pane`** is any focused
foreign rmux pane (the [[harness pane]], or the aux term/yazi/vcs panes — `self.focused`
says which, so aux panes need no new variant); **`Detail`** is the grove-drawn
[[detail widget]] peer (`leader → d`; `j`/`k` scroll); **`Nav`** is the
[[nav surface]]; **`Modal(kind)`** is a focus overlay that captures all keys and
restores the prior focus on cancel/submit — the `kind` selects the overlay:
`Capture` (a text buffer), `MovePicker` (the inbox move-target grove list), or
`Confirm` (the seed-start y/n prompt, `Enter` on a [[Seed]] row → `grove do
<name>`). The [[leader]] opens
`LeaderPending`, and the next key dispatches to a surface; the [[whichkey footer]]
renders the live menu while pending. Pane addressing is the app's `name → PaneDriver`
map + `focused` key (ADR-0028 E3), not the focus state. The pure transition table
`arbitrate(Focus, Leader, Event) → (Focus, Action)` and the crossterm→tmux key-map
(`src/tui/input.rs`) are headless-tested — the testability win of the rmux migration.

**Nav surface** (rmux-substrate, 030-engine leaf 030):
grove's minimal grove-list command surface (`src/tui/nav.rs`) — what makes
`grove tui` usable for more than one grove. A flat, selectable list of **live**
groves projected from the presentation-agnostic core
([[Fleet]]/`MultiRepoView`/`RepoView`): labels are bare `<grove>` at a single-repo
fleet and `<repo>/<grove>` when N>1. The [[leader]] flips [[Focus]] to it;
↑/↓ (or `j`/`k`) move, ⏎ opens the selected grove's [[harness pane]]
(`grove do <name>` in a new detached rmux window) or focuses the already-open one
(no duplicate pane — the `grove-name → PaneId` map, E3), Esc returns to the
harness. fs-watch ticks rebuild it (groves appearing/retiring), preserving the
selection by name. `Nav::render` is a **pure** snapshot → `Buffer` function
(headless-tested). Deliberately minimal: grouped/collapsible repo headers, fuzzy
ranked filtering, and inbox/lifecycle toggles are **050**; park/close of
non-selected harnesses is **050** (here a deselected pane just stays open in the
background). Distinct from the superseded zellij nav *plugin* (a WASM plugin,
ADR-0028) — this is a native ratatui surface grove draws itself.

**Filter mode** (rmux-tui-polish, 010-plan):
the [[nav surface]]'s discrete sort/filter sub-mode — entered with `/` from Nav,
exited with Enter (accept) or Esc (clear). Inline and live: the list re-ranks as
the fuzzy needle is typed and the toggles flip (Ctrl-i inbox-pending, Ctrl-l
lifecycle cycle, Ctrl-s sort cycle). While any dimension is **engaged** the nav
shows a flat ranked list plus a one-line criteria summary; Esc in normal Nav
layers — first press clears the engaged filter, second returns to the pane. All
state is ephemeral per session (constraint 1). Sort cycle: name → recency
(last-commit timestamp on the grove branch) → inbox-pending count. Avoid the
alias *sub-modal* (the old-codebase precedent it replaces — the discreteness
survives, the separate panel does not).

**Live preview** (rmux-tui-polish, 010-plan):
the inspect-without-harness affordance: while Nav has focus, the [[detail
widget]] re-points to the *highlighted* grove as the cursor moves — reading a
grove's task tree / briefs / inbox never spawns its harness. Tab (or `l` on a
grove row) moves focus into detail; Esc there returns to Nav (detail remembers
it was entered from Nav, unlike the pane-entered path). A previewed **seed**
shows its pending observations (it has no task tree); Enter on a seed starts
the grove behind a y/n confirm. Avoid the alias *peek key* (the rejected
explicit-keystroke variant).

**Rendered-history capture** (rmux-substrate, 040; ADR-0029):
the clean, emulator-**rendered** scrollback of a harness pane (history rows + visible
screen), as opposed to raw `line_stream` ANSI bytes. The source of truth is the rmux
*daemon's* rendered grid (`rmux-core` keeps `history` rows in the grid and renders them via
`capture_transcript`); grove obtains it by **shelling out to the stock `rmux capture-pane`
CLI** (`-p -S - -J` for full history, plain text, soft-wrap joined) against the daemon its
TUI already started — the same shell-out-below-the-seam idiom as the capture write (ADR-0028
E1). **No fork:** published rmux 0.5.0 already ships this end-to-end (emulator → proto
`CapturePaneRequest` → CLI); the 040 scoping spike retired the earlier plan to fork rmux (D7)
and the still-earlier grove-side "rendered-scrollback ring" (D2). Note the asymmetry: the
`rmux-sdk` `Pane` helpers (`snapshot`/`capture_region`/`screenshot`) are **visible-screen
only** — history capture lives in the proto/CLI, not the SDK sugar. First consumer is
[[open-in-editor]] (040); the deferred real copy-mode/scrollback/search groves build on the
same capability. Avoid the alias *scrollback ring* (the abandoned D2 grove-side
reconstruction).

**open-in-editor** (rmux-substrate, 040):
the interim stand-in (010-plan D1) for real copy-mode/scrollback: **leader → `e`** dumps the
focused [[harness pane]]'s [[rendered-history capture]] to a temp file and opens `$EDITOR`,
restoring the TUI on exit. Trivial because grove owns the draw loop (ADR-0028) — suspend raw
mode + alt-screen (and pause the crossterm input-reader thread so the editor child owns
stdin), run `$EDITOR`, restore. Deliberately *not* a real standalone copy-mode/scrollback/
search surface — those are explicitly out of scope, deferred to follow-up groves.

## Flagged ambiguities

**"grove"** is overloaded across this codebase. It can mean:
1. The **CLI tool** / Rust crate published as `grove` — the verbs `install`, `do`, `status`, `retire`, etc.
2. The **methodology** bundled in `content/SKILL.md` and materialised into harnesses.
3. A single **workstream** — one named task tree under `.grove-worktrees/<name>/.grove/`. `grove do` operates on this sense.

When usage is ambiguous, qualify: "grove CLI", "grove methodology", "this grove".
