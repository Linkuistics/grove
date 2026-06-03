# grove

The grove CLI and methodology. This glossary holds terms specific to this codebase; general programming concepts are excluded by design.

## Language

**Install scope**:
The bounded set of paths a single `grove install` or `grove update` invocation modifies — the union of `harness.install_path(repo)` over the harnesses targeted in that invocation. grove's auto-commit stages and commits exactly this set, leaving unrelated dirty state alone.

**Path-scoped commit**:
A commit constructed via explicit `git add <paths>` and `git commit -- <paths>` rather than via whatever happens to be in the index. The technique grove's `install`/`update` use so the auto-commit cannot sweep up unrelated work-in-progress.

**Lifecycle walkthrough**:
A per-verb prose document under `docs/workflows/` that shows the command(s) for one grove flow (install, update, start, multi-step, finish), explains what happens, and shows what changed in the repo or worktree at each step. Distinct from the CLI reference in `README.md` (which lists flags) and from the methodology doc `docs/grove.md` (which is "what grove is and why"). Avoid the aliases *tutorial*, *guide*, and *runbook* in this codebase.

**Inbox**:
A directory accumulating observations addressed to a named grove, living at `inboxes/<name>/` on the [[grove-meta branch]]. Each observation is a single markdown file at `inboxes/<name>/<UTC-iso8601-seconds>Z-<slug>-<content-hash-8>.md`; the directory's existence (preserved by a `.gitkeep`) is the "this grove is known" signal even when no observations are pending. Observations are written by the LLM working in any grove that notices something belonging elsewhere, via `grove-llm inbox-add` (never direct file edits; see ADR-0006 for the LLM-binary split). The inbox is addressed to the workstream, not to a person. A running grove [[drain]]s its own inbox at every `grove do` (the sole lifecycle entry verb). An inbox whose addressed grove does not currently exist as a worktree is called a [[seed]]. The shape moved from "one file per grove" to "one directory per grove, one file per observation" in ADR-0004 to escape the merge-conflict pathology of single-file multi-writer (see `docs/research/in-repo-issue-tracker-postmortems.md`).

**Seed**:
An [[inbox]] whose addressed grove does not *yet* exist as a worktree — a workstream noticed but not started. Same artifact and same path as a running grove's inbox; only the lifecycle state differs. A *finished* grove is **not** a seed: the [[Complete finish cycle]] removes its inbox (`grove-llm inbox-remove`, ADR-0012) as a teardown step, so a retired grove leaves no `inboxes/<name>/` behind to be mistaken for a not-yet-started one. A still-orphaned inbox (worktree gone but the grove never properly finished) therefore signals an *incomplete* finish, not a dead grove. The seed's name is the working title of the addressed grove (e.g. `racket-bugs-discovered-while-implementing-chez`); it may later be renamed, narrowed, or folded into a catchall seed. When `grove do <name>` first runs against that name, the seed simply becomes the new running grove's inbox — there is no separate consumption step. A seed is distinct from a brief note (in-scope context for the current node, retired with it), from an ADR (a decision, not an observation), and from a TODO comment in code (local to one file, not a workstream candidate). Avoid the alias *issue* — it collides with GitHub Issues and with bug-tracker connotations.

**grove-meta branch**:
A dedicated branch in each repo, materialised as a sibling worktree at `<repo>/.grove-meta/`, holding shared cross-grove coordination data and repo-level metadata that does not belong on the default branch. The primary occupants today are [[inbox]] directories at `inboxes/<name>/`; the branch is reserved more broadly as the home for any cross-grove coordination artifact or repo-level metadata (grove-related or otherwise) that future work surfaces. All reads and writes go through `grove` CLI subcommands rather than direct git/filesystem operations, so the LLM does not interact with the branch's git plumbing. Cross-repo inbox writes follow the same rule against another repo's `.grove-meta/` worktree, and require that worktree to be present locally. The branch is created (and the worktree attached) by `grove meta init` — invoked explicitly for repos that pre-date the feature or to repair a missing worktree, and implicitly during `grove install` / `grove update`. Remote-sync is **opt-in**: by default the branch has no upstream and lives local-only. Multi-machine users add a remote with `grove meta remote add <url>` (which sets upstream tracking); thereafter `grove do` fetches-before-drain and `grove-llm inbox-add` / `grove-llm inbox-drain` push-after-commit, both best-effort with one auto-retry on non-ff (ADR-0005). The previously-considered name `grove-inboxes` was rejected because it narrowed the branch's scope to its first occupant.

**Drain**:
The session-bootstrap triage of a running grove's [[inbox]], performed at every `grove do` (the sole lifecycle entry verb). If the `grove-meta` branch has a remote configured, drain fetches it first (soft-on-failure — warn-and-continue if offline; refuse-and-instruct on non-ff). For each pending observation the LLM proposes incorporating it into the current task, deferring it to a later leaf, or rejecting it as out-of-scope (and possibly seeding a different grove). After triage the triaged observation files are deleted in one session-commit; drained observations live in git history (`git log inboxes/<name>/`). The `.gitkeep` stays so the directory's existence remains the "known grove" signal even when no observations are pending.

**cli version** / **repo version** / **worktree version**:
The three locations at which a grove-methodology version stamp can live, and which any pair can drift between. **cli version** is the installed `grove` binary's own version (`env!("CARGO_PKG_VERSION")`, the methodology bundled inside the binary). **repo version** is the version stamped in `<repo>/.<harness>/skills/grove/VERSION.md` — one per installed harness, written by `grove install` / `grove update`. **worktree version** is the version stamped in `<repo>/.grove-worktrees/<name>/.<harness>/skills/grove/VERSION.md` — one per worktree per harness, written when `grove do` first materialises the skill into a new worktree. The three are independent because `grove install` only touches the repo's install path and `grove do` (on the new-grove path) only touches the new worktree's install path; the binary the user runs `grove status` with is a third axis entirely. Surfacing all three (and drift between them) is the job of `grove status` and the TUI — the canonical visibility surface (ADR-0007). The former `grove list` and `grove version`, whose output `grove status` subsumes, were removed in v4.0.0; `grove --version` still answers the CLI-only question. **Drift rule:** plain raw string equality (`==`), both stamps shown verbatim on mismatch, no semver interpretation and no `v`-prefix normalisation. From v4 onward `grove install` stores the **canonical** stamp (the incoming git tag with its leading `v` stripped — the canonicalisation lives on the install/write side, see the install leaf), and the **cli version** (`CARGO_PKG_VERSION`) is already canonical, so every stamp in play is canonical. Any stamp still bearing a `v` is therefore necessarily a pre-v4 release — genuinely older than the running code — and `!=` flagging it as drift is the correct verdict, not a false positive (the binary only ships under a new version number, so there is no scenario where a canonical and a `v`-prefixed stamp name the *same* release). A missing/unreadable worktree `VERSION.md` renders `(unknown)` and is never drift; an orphan worktree (harness no longer installed in the repo) shows `repo=(none)` informationally without a warning. `status::worktree_version` is the shared per-worktree-per-harness reader; the comparison is plain `==` so every surface (incl. the TUI) applies it directly.

**Complete finish cycle**:
The terminal, whole-grove sequence that retires a grove once it has no live leaves left: (1) promote durable artifacts from the briefs (ADRs, docs, glossary); (2) delete `.grove/` in a focused commit; (3) merge the branch into the default branch (`git -C <repo> merge <name>` — fast-forwards or makes a merge commit); (4) clean up the inbox — drain any pending observations then `grove-llm inbox-remove --for=<name>` to delete `inboxes/<name>/`, so the finished grove stops showing as a [[Seed]] (ADR-0012; refuses while observations are pending, no-op when never seeded); (5) remove the worktree; (6) delete the branch. Steps 3, 5 and 6 run against the main repo with `git -C <repo>`; step 4 writes to the `grove-meta` worktree and runs before step 5 while cwd is still valid; worktree-remove precedes branch-delete (git refuses `branch -d` on a branch checked out in a live worktree). Driven by the in-session LLM, not Rust automation (ADR-0010); proposed and executed only on explicit human confirmation, so a headless run with no human reports the plan and stops; resumed from git/filesystem state with no progress marker (constraint 1). Triggered whenever `grove-llm pick` reports no live leaves — or errors because `.grove/` is already gone, the partial-finish resume case. Distinct from [[Drain]] (a per-session bootstrap step) and from the per-leaf Retire step (which archives one leaf into `done/`); the finish cycle is what retiring the *last* leaf leads into. The former `grove finish` verb that launched this was removed (ADR-0009): finishing is a step of the loop, not a launched verb.

**root-init** / **fresh-grove start**:
The bootstrap of a brand-new grove (worktree + branch exist, but no `.grove/` tree yet), enacted by the `grove-llm root-init [<slug>]` verb. It creates `.grove/`, the root `BRIEF.md` stub, and a first **planning** leaf `010-<slug>.md` (default slug `plan`) — a working-tree change with no commit, refusing to clobber an existing `.grove/`. It is the one tree verb that sits *below* the floor the others stand on (`leaf-add`/`leaf-insert`/`leaf-decompose`/`leaf-retire` all require `.grove/` to already exist), so it is what makes a fresh grove enter the steady-state loop. Creating the first leaf — not just the root brief — is load-bearing: `grove-llm pick` skips every `BRIEF.md`, so a brief-only `.grove/` reports "no live leaves; this grove is done" and would mis-trigger the [[Complete finish cycle]] (ADR-0011). The first session's commit folds the scaffold in. Distinct from [[Bootstrap]]-the-loop-step (reading context at the start of every session); this is the one-time creation of the tree that the loop then reads.

**Bootstrap**:
The per-session context-loading step of the grove loop: read the glossary, the ancestor `BRIEF.md` chain, the cited ADRs, and the task file, then [[Drain]] the inbox. Read-only — no script must succeed before work begins. Not to be confused with [[root-init]] (the one-time scaffolding of a *new* grove's tree); bootstrap reads an existing tree, fresh-grove start creates one.

## TUI / v2 presentation

**Dashboard**:
*(Mechanism updated by the [[trellis framework]] fork — ADR-0020: both surfaces
below now render **natively in-process** — the nav is a native surface (not a WASM
plugin) and the detail is native (not a dumb proxy). The two-surface **split** and
where each lives are unchanged.)*
grove's TUI chrome — historically the master/detail navigation surface (`grove list`-style grove list, task tree, briefs, inbox triage). **As of ADR-0019 (the "A′" model) the master/detail split is dissolved into two surfaces:** the **master list** is the [[nav plugin]], which *is* the **"home" [[workspace]]** (full-height grove list, the leader-focusable command surface); the **detail** (one grove's task tree / inbox / capture) is a per-grove [[detail proxy]] rendered into *that grove's own* [[workspace]] tab beside its harness. Both still sit *above* the [[presentation boundary]] and render from the presentation-agnostic `RepoView`/`MultiRepoView` core inside the [[controlling process]]; the detail proxy is a [[dashboard proxy]] in the ADR-0016 sense (dumb, controller-rendered), the nav is a smart self-rendering plugin. **Switching *between* groves is done in the [[nav plugin]]** (which opens/switches tabs itself, ADR-0019) and via `GoToTab` keybinds — never in a home dashboard, which no longer carries a grove list (ADR-0019 superseded both the earlier "dashboard is the switch surface" model and the ADR-0018 "home dashboard lists groves" framing).

**Controlling process**:
*(Dissolved by the [[trellis framework]] fork — ADR-0020. With grove compiled in
natively there is no separate controller launching zellij as a child; all the
functionality below now runs **in-process** inside the forked framework, and the
proxy seam disappears. This entry describes the superseded out-of-process model.)*
The single, persistent process that launches and owns the [[zellij substrate]] (runs zellij as a child; does *not* `exec` and vanish) and owns **all** grove functionality: the `RepoView`/`MultiRepoView` data layer, fs-watch/`notify`, shell-out-to-`grove` writes, the decisions about which [[harness pane]]s to open/focus (driving zellij via `zellij action`), **and** the ratatui rendering of every [[dashboard proxy]] (ADR-0016). It keeps a per-proxy render target sized to that proxy and ships rendered diffs down the [[seam frame]] channel (a unix socket); input flows up, display flows down. Its non-interactive `grove-llm` writes run in-process with no tty (the v1 alt-screen `suspended()` dance is gone — that only existed because the dashboard process used to own the tty); only the interactive `$EDITOR` drop needs a tty, and routes to the proxy's own via a `RunEditor` seam frame (ADR-0017). The realisation of the [[head binary]] role as an always-on owner rather than a launcher. The single-source-of-truth that all presentations (1b native-pane proxy, future 1a plugin, future web client) proxy to.

**Dashboard proxy**:
*(Superseded by the [[trellis framework]] fork — ADR-0020: rendering is in-process,
so there is no dumb-terminal proxy and no socket seam. This entry describes the
superseded mechanism; the per-grove detail *surface* it carried survives natively —
see [[detail proxy]].)*
A **dumb-terminal proxy** for a [[dashboard]] surface (working name `grove __dash-proxy`): a thin client running in a zellij pane that only (1) reports its terminal size to the [[controlling process]] (and on SIGWINCH), (2) blits the controller's down-going output to its stdout, and (3) forwards its stdin (key/mouse) up to the controller (ADR-0016). It holds **no** grove state, business logic, or ratatui — all of that is in the controlling process. The seam is one unix-domain socket carrying tag+length [[seam frame]]s both ways; the one place the dumb proxy meets an interactive child is the `$EDITOR` drop, which the controller routes to the proxy's *own* tty via a `RunEditor` control frame (ADR-0017). Implemented as a single-threaded `poll(2)` loop over (socket, stdin) so that while an editor child owns the tty the proxy is not racing it for stdin. "component(s)" is plural: multiple dashboard surfaces are each an independent proxy the controller renders separately. Distinct from a [[harness pane]] (a *native* zellij pane running someone else's program, `grove do <name>`, which zellij emulates — not a grove-rendered surface).

**Seam frame** (controller↔proxy wire):
*(Superseded by the [[trellis framework]] fork — ADR-0020: with in-process
rendering there is no controller↔proxy wire. Describes the superseded socket
codec.)*
The message unit on the [[dashboard proxy]] seam — one unix-domain socket, both directions tag+length framed (ADR-0016, ADR-0017). **Down** (controller→proxy): `Output` carries a rendered draw's raw ANSI (the hot path, one frame per draw via a `FrameWriter`; the proxy blits the payload), and `RunEditor { path }` is the rare control verb to run `$EDITOR` on the proxy's tty. **Up** (proxy→controller): `Resize` (size on connect + SIGWINCH), `Input` (forwarded raw stdin, which the controller decodes into `crossterm` `KeyCode`/`KeyModifiers` itself — its `event::read()` cannot point at a socket), and `EditorDone { ok }` (the `RunEditor` reply). The codec is hand-rolled (no `serde` for a handful of variants) and `ratatui`/`RepoView`-free transport. 010 left the down direction unframed ("proposed; refine in build"); ADR-0017 framed it to carry `RunEditor`. The `$EDITOR` tempfile is shared-filesystem (local-proxy assumption); a future remote/web proxy uses a non-`RunEditor` edit affordance.

**Harness pane**:
*(Mechanism updated by the [[trellis framework]] fork — ADR-0020: the harness is a
native pane the **forked framework** emulates and grove opens by a **direct
in-process call**, not via external `zellij action`. The role below is unchanged.)*
The TUI region showing one grove's **live, interactive harness session** — the `grove do <name>` process (claude code / codex / other terminal tooling) — displayed within that grove's [[workspace]] tab as one pane of its [[working set]] (ADR-0018) so the user can watch and interact with it. Named "pane" because the working model is a single window split into the dashboard surface plus the active harness, not separate full-screen windows. **The realisation mechanism is a native [[zellij substrate]] pane** (decision D2 — first settled as in-process pty by the 050 spike/ADR-0014, then *superseded* by ADR-0015 after the 060/020 spike evaluated zellij on its own terms): the harness runs as a native zellij terminal pane that zellij emulates, opened/focused by grove via `zellij action`. grove does **not** emulate the harness itself. (The superseded in-process-pty path — the [[harness-pane crate]] — is the recoverable fallback, not the live mechanism.) Distinct from the [[dashboard]] (grove's own chrome); the harness pane shows someone else's program (the harness).

**trellis framework** (provisional name; the v2 substrate as of ADR-0020):
grove's v2 TUI/harness substrate after the fork: a **deep hard fork of zellij**
turned into a reusable **TUI application framework whose headline concern is
seamlessly embedding *other* TUI apps** (vim, lazygit, yazi, a shell, claude) as
first-class, fully-emulated panes of a host app — using zellij's emulation under
the hood, not a hand-rolled one. grove's TUI logic is compiled in **natively,
in-process** (not a WASM plugin, not external `zellij action` driving), which
**dissolves** the [[controlling process]]/[[dashboard proxy]] seam, the WASM
[[nav plugin]], and the `cli_pipe_output` back-channel — each of which was a
workaround for grove being *outside* zellij. **Hard fork, not a tracking fork:**
owned outright, **no rebase**; upstream watched **only as a CVE feed**, security
fixes hand-patched. **Two products, grove-first / extract-later:** `trellis` itself
(publishable — the non-grove parts behind a compiler-enforced **one-way crate
seam**, framework must not depend on grove) **+ grove as its first consumer**.
Native consumers *link* it and get the **full system model as typed Rust values +
calls** ("you don't serialize what you can link"); a wire format (and there,
**GraphQL** over protobuf RPC) matters **only if/when** an out-of-process/network
surface is later added — not in the core. **Observing + programmatically driving
embedded tools** is a first-class framework facility (grove's harness-watching,
generalised). `trellis` is a **provisional working name**; the public brand is
decided at the extraction leaf. **Supersedes [[Owned zellij substrate]]** (ADR-0015,
unmodified installed zellij) and the *mechanism* of ADR-0016/0018/0019 (the UX they
settled survives). Distinct from the shelved [[harness-pane crate]] — trellis
fulfils that crate's embedding *mission* via zellij's mature emulator instead of
`tui-term`/`vt100`. **Vendored** at `crates/trellis/` (a self-contained,
workspace-`exclude`d cargo workspace; forked from zellij `v0.44.3`, see
`crates/trellis/PROVENANCE.md`); the grove→trellis edge is the optional `trellis`
dependency behind the `trellis-seam` feature until the native host API (leaf 110)
makes it always-on.

**trellis hosting API** (library-you-link; resolved by ADR-0021):
the shape by which grove hosts the [[trellis framework]] — **grove owns `main`**,
*links* the `zellij-client`/`zellij-server`/`zellij-utils` crates, and **dispatches
the client/server roles itself** (no `--server` arg → client UI path; `--server
<socket>` → the trellis server, augmented with grove's surfaces). Chosen over
*runtime-you-plug-into* (trellis owns `main`, grove registers an app) because
**zellij's server is a re-exec'd separate process** (`spawn_server` runs
`current_exe() --server`, double-forks, talks over a unix socket): grove's panes
render **server-side**, so grove's code must be compiled in and re-dispatched on
the `--server` path regardless — a runtime wrapper can only mean a **WASM plugin**
(the rejected ADR-0018 premise) or a native callback that can't survive the
re-exec. This is the shape zellij's own `main.rs` already implements. **The seam
110 builds on:** (1) grove's `main` dispatches roles; (2) grove substitutes/extends
the injectable `ClientOsApi`/`ServerOsApi` trait objects; (3) grove adds a **native
`Pane` impl** — a *third pane kind* beside `Terminal`/`Plugin` — rendering ratatui
as `CharacterChunk`s server-side, subsuming the [[dashboard proxy]] seam and the
`zellij action` driving. Note: ADR-0020's "in-process rendering" means *inside the
server daemon* (same binary), not the thin foreground client that blits ANSI.

**host surface / host driver / host tick** (the realised trellis host-API; leaf
`110/030`): the consumer-facing seam a host app (grove) drives the [[trellis
framework]] through — the realisation of the [[trellis hosting API]]'s seam point
(3). A **host surface** (`trait HostSurface`) is what a host draws into an
off-screen ratatui `Buffer`: `draw` / `handle_key` (crossterm keys) / `resize` /
`set_focused` / `cursor`, plus the lifecycle hooks `set_driver` and `tick`. A
**host driver** (`HostDriver`, handed to the surface by `set_driver`) is the
clonable, `Send` handle the surface drives layout through — `new_command_tab` /
`focus_tab` / `close_tab` (open / focus / close a *named* harness tab),
`request_tick`, `quit` — each a fire-and-forget post onto the server's
`ScreenInstruction` channel (the surface runs *on* the screen thread, so it cannot
block waiting for that same thread to reply; that is why driving is by message,
not return value). A **host tick** (`ScreenInstruction::HostSurfaceTick(pane_id)`)
is how a host's *background* thread (grove's fs-watch) wakes a refresh+redraw with
no keypress: it posts the tick, the screen thread calls the surface's `tick()` and
re-renders if it returns `true`. grove's v1 [[dashboard]] is the first consumer —
it now renders as a host surface, retiring the [[dashboard proxy]] seam and the
`zellij action` driving. Contract doc: `crates/trellis/HOST_API.md`.

**Owned zellij substrate** (zellij substrate):
*(Superseded by the [[trellis framework]] fork — ADR-0020 — for the v2 path; this
describes the unmodified-installed-zellij model it replaced. The substrate is now a
deep hard fork, not an unmodified dependency.)* grove's v2 presentation/harness substrate (ADR-0015): a **grove-owned zellij multiplexer**. A [[head binary]] launches zellij with grove's *bundled config + bars-free KDL layout* so it presents as a single binary; the [[dashboard]] is a native zellij pane running grove's own ratatui (Strategy **1b**), [[harness pane]]s are native zellij panes, and grove drives the layout from outside via `zellij action` (each grove a tab: `new-tab` / `go-to-tab-by-id` / `close-tab-by-id`, existence checked via `list-tabs` — ADR-0018) — its shell-out write idiom. zellij runs in `default_mode "locked"` so every key passes through to the focused app (the modal design that avoids tmux's prefix-collision tax); the only intercepted key is the remappable unlock. Wins native+free: copy mode, scrollback, search, floating panes, session persistence, web client. **Strategy 1a** (a WASM plugin via `zellij_widgets` + zellij *pipe* IPC to the head binary) was a recorded future refinement; ADR-0018 **pulled it forward for the [[nav plugin]]** (not the whole dashboard) after the CLI-driven `focus-pane-id` switch model hit a locked-mode dead-end. Workspace switching also moved from `focus-pane-id` to per-grove [[workspace]] tabs (`GoToTab`). Distinct from the retired tmux-owner plan (ADR-0014: zellij ≠ tmux) and from the superseded in-process-pty embed (the [[harness-pane crate]] fallback).

**Head binary**:
The grove process (`grove`/`grove tui`) that launches and owns the [[zellij substrate]] — starting zellij (as a **child**, persisting alongside it — ADR-0016) with grove's bundled config and layout, and driving panes via `zellij action`. Its purpose is to make a multi-process substrate present as a single walk-away binary: "one binary vs many" is a packaging detail, the UX/DX is what matters (user steer, 060/020). It *is* the [[controlling process]]: **all** functionality (data layer, fs-watch, writes, pane decisions, and dashboard rendering) lives here; every dashboard surface is a [[dashboard proxy]]. (Earlier phrasing had the 1b dashboard pane running a standalone `grove tui`; ADR-0016 corrected that — the pane is a dumb proxy, the logic+rendering is here.)

**harness-pane crate**:
*(Fallback as of ADR-0015 — superseded by the [[zellij substrate]] for the v2 path; retained in-repo as recoverable evidence the embed works, not built on further.)* The extractable in-repo workspace crate (working name `crates/harness-pane`) that implements the in-process-pty embed: `tui-term` + `portable-pty` + `vt100 0.15.2`, plus the consumer wiring `tui-term` leaves open — crossterm `KeyEvent`→bytes encoding, SGR mouse, bracketed paste, resize, native-cursor handling (hide tui-term's drawn cursor, position the hardware cursor), [[dynamic mouse capture]], and [[pane-local copy mode]]. grove consumes it; it sits *below* the [[presentation boundary]] (returns a renderable screen + plain data, takes input events). Identified during the 050 spike as a reusable component in its own right (the gap `tui-term`'s render-only WIP leaves); publishable later if its API stabilises, no published-crate overhead now (ADR-0014). The `vt100 0.15.2` pin is load-bearing — `tui-term 0.2` re-exports its own vt100 and only impls its `Screen` trait for that version. Its public types are [[TerminalEmulator]] (the renderable core) and [[PtySession]] (the byte source); a [[harness pane]] is a layout region that pairs them — "pane" stays a *layout* word, not the name of the pty-backed thing inside it.

**TerminalEmulator**:
*(Part of the [[harness-pane crate]] fallback — superseded by the [[zellij substrate]], ADR-0015.)* The [[harness-pane crate]]'s renderable core: owns the `vt100 0.15.2` parser/`Screen`, is **fed bytes** via `process(&[u8])`, and renders through `tui-term`'s `PseudoTerminal` widget. Deliberately **source-agnostic** — a [[PtySession]], synthetic ANSI, or a recording can all feed it — which makes it unit-testable with no child process (mirrors the 050 spike's `--dump`). It also exposes cursor position/visibility and `Screen::title()`, and `wants_mouse()` (`mouse_protocol_mode() != None`) for [[dynamic mouse capture]]; it hides tui-term's drawn cursor so the host positions the native hardware cursor. Named to keep "pane" meaning a *layout region*, distinct from the pty-backed emulator inside it (user steer, 060). Lands in 060 leaf `010-embed-pane`.

**PtySession**:
*(Part of the [[harness-pane crate]] fallback — superseded by the [[zellij substrate]], ADR-0015.)* The [[harness-pane crate]]'s byte *source*: a `portable-pty 0.9` master + spawned child + the reader thread that pumps child output into an `mpsc` channel (drained via `try_recv` between event-loop ticks — the 050 sync pump). Accepts input (`write_input`) and resize (updates `master.resize` and the [[TerminalEmulator]] together), and reports child status (`try_wait`). The counterpart to `TerminalEmulator` (which is source-agnostic): `PtySession` is the one source that happens to be a live process. Lands in 060 leaf `010-embed-pane`.

**dynamic mouse capture**:
*(Embed-specific — applies to the [[harness-pane crate]] fallback only; the live [[zellij substrate]] handles mouse natively, ADR-0015.)* The rule that the embed enables terminal mouse capture (`EnableMouseCapture`) **only while the focused harness requests mouse reporting** (vt100 `mouse_protocol_mode != None`, e.g. vim), and releases it otherwise (e.g. claude, which requests none). Unconditional capture is worst-of-both: the app gets no mouse *and* the host terminal's native text-selection/copy is disabled. Re-evaluated on every focus change, not just at startup. A small data-up/command-down coupling across the [[presentation boundary]] (emulator state below the seam drives a backend command above it) that a tmux backend would not have (ADR-0014).

**pane-local copy mode**:
*(Moot for the v2 path as of ADR-0015 — the [[zellij substrate]] provides per-pane copy mode, scrollback, and search natively/free, which is why leaf 030-scrollback-copy was retired. This entry describes the obligation the superseded [[harness-pane crate]] embed would have owed.)* A scrollback-and-selection model the [[harness-pane crate]] must build over vt100 scrollback so the user can scroll back through *one* pane's history and select multiple lines *within that pane* (not the whole outer terminal grid, which the host's native selection spans) and copy that pane's text (clipboard / OSC-52). Scrollback navigation is required **both key- and mouse-driven** (mouse wheel intercepted by the pane only when the focused app isn't itself requesting mouse), mapped to vt100 `set_scrollback`; the selection/extraction logic is ours to build (vt100 stores the lines, not a selection). The feature tmux gives free via its per-pane copy-mode; the in-process-pty embed owes it (decision D2 / ADR-0014). Lands in 060 leaf `020-scrollback-copy`.

**Presentation boundary** (core↔presentation seam):
The architectural seam (ADR-0013) separating Ratatui rendering (*above*) from grove's core logic — the `RepoView`/`MultiRepoView` data layer, harness driving, and shell-out writes (*below*). Code below the seam never imports `ratatui` types. Its purpose is optionality: a future web front-end becomes a *second presentation over the same core*, not a rewrite. Enforced by module placement and review, deliberately **not** by a `GroveBackend` trait (no second consumer exists yet to shape one). The harness-embedding mechanism (tmux vs in-process pty, decision D2) lives *below* this boundary either way.

**Workspace** (one grove's swappable working set):
*(Reshaped by ADR-0022 — a workspace is **no longer a zellij tab**. The nav is
**constant** (always on screen) and a **content region** swaps the selected grove's
[[working set]] in; non-selected harness ptys stay alive off-screen. The term
*workspace* is kept for "one grove's working set, switched as a unit," but
"workspace = tab / `GoToTab` switching" is superseded. The one-grove-at-a-time
property survives. This entry describes the superseded tab model.)*
A single zellij **tab** holding one grove's [[working set]], with a "home" tab for the [[dashboard]] (ADR-0018). One grove's working set is visible at a time; zellij keeps every tab's panes alive across switches. Switched via native `GoToTab`/`GoToNextTab` keybinds (the hot path) and the [[nav plugin]] (rich/fuzzy selection). Replaces the superseded model of multiple [[harness pane]]s beside the [[dashboard]] switched by `focus-pane-id`. **Why tabs went (ADR-0022):** pinning a *tiled* nav pane across tabs is not native (floating-only), and a pty stays alive/captures scrollback while not displayed (per-pane reader thread) — so tabs were never load-bearing for keeping background harnesses alive, and they fought the constant-nav requirement.

**Working set**:
*(Mechanism updated by the [[trellis framework]] fork — ADR-0020: opened by a
**direct in-process call** into the framework (not the WASM nav's
`new_tabs_with_layout` over a `zellij pipe`), aux tools embedded via trellis's
TUI-embedding. **Then ADR-0022:** the working set is no longer a tab's contents — it
fills the **content region** beside the constant nav when its grove is selected,
and is **parked alive off-screen** (not closed) when another grove is selected. The
pane composition + toggle/responsive **UX** below is unchanged. Live leaf:
`150-working-set`.)*
The set of panes shown for one grove inside its [[workspace]] tab: the [[harness pane]] + a per-grove [[detail proxy]] (task tree / inbox / capture) + a plain terminal + yazi (files) + lazygit/lazyjj (vcs) (ADR-0018, ADR-0019). Panes are individually toggleable and the layout is responsive — pack on a large display, degrade gracefully to a laptop screen. **First-time creation is the [[nav plugin]]'s job** (ADR-0019: the nav opens the tab itself via `new_tabs_with_layout`, fed the grove's `grove do` command + cwd over the forward `zellij pipe`) — *not* the controller via `zellij action`, since the `cli_pipe_output` back-channel that would have let the nav ask the controller to open is unworkable. Toggling/focusing within the set is also driven from the nav.

**Nav plugin** (`grove-nav`):
*(Realisation superseded by the [[trellis framework]] fork — ADR-0020: the nav is
now a **native in-process surface**, not a WASM plugin — a keybind reaches it by a
direct focus call, with no `LaunchOrFocusPlugin`/permission/`cli_pipe_output`
apparatus. The nav's **role/UX** below survives; only "it is a WASM plugin" is
superseded. Live leaf: `120-native-nav`.)*
grove's [[leader]]-focused command surface, realised as a **zellij WASM plugin** (Strategy 1a, pulled forward by ADR-0018 — the one surface a keybind can focus *by name* and that receives keys while focused; locked-mode key delivery to it is confirmed live). The [[leader]] focuses it via `LaunchOrFocusPlugin`. As of ADR-0019 it is also the **"home" [[workspace]]** — a full-height grove list. It switches [[workspace]]s, toggles [[working set]] panes, jumps home, and is the mode/key discoverability surface (subsuming the former `050-mode-discoverability` concern). It holds **no** grove state — the [[controlling process]] pipes it the live grove list *with each grove's `grove do` command + cwd* (forward `zellij pipe`). **It opens and switches workspaces *itself*** (ADR-0019: `switch_tab_to` for an open grove, `new_tabs_with_layout` to first-open a closed one) — *not* by "signalling intent back to the controller," which ADR-0018 proposed but ADR-0019 found unworkable (`cli_pipe_output` is reply-only — it cannot push to a stored channel from a keypress). Amends ADR-0016: unlike a [[dashboard proxy]] (dumb, controller-rendered), the nav is a *smart*, self-rendering surface.

**Detail proxy** (per-grove):
*(Mechanism superseded by the [[trellis framework]] fork — ADR-0020: per-grove
detail is rendered **natively in-process** in each grove tab, not as a dumb
terminal proxy over a seam. The per-grove-detail **UX** survives; "it is a dumb
proxy / N socket clients" is superseded. Live leaf: `130-native-detail`.)*
A grove-scoped [[dashboard proxy]] (`grove __dash-proxy --grove <name>`) that lives **inside that grove's [[workspace]] tab**, beside its [[harness pane]], and shows *only that grove's* detail — task tree, inbox triage, capture (ADR-0019). The [[controlling process]] renders **one per open grove** (N proxies, the latent ADR-0016 "supports N proxies" now load-bearing): each proxy is fixed to its grove for its whole life, set when the [[nav plugin]] opens the tab, so **no nav→controller selection signalling is needed** (which is what makes A′ buildable given `cli_pipe_output` is reply-only). The dumb-proxy mechanics (size up, blit down, input up, `RunEditor` for `$EDITOR`) are unchanged from [[dashboard proxy]]; "detail" just means the chrome it renders is the detail view, not a grove list (the list is the [[nav plugin]]).

**Whichkey bar**:
*(Realisation superseded by the [[trellis framework]] fork — ADR-0020: a **native
bottom region** the in-process host draws, not a WASM plugin (no build.rs embed, no
layout-pin). The single-hint-owner **UX** survives. Live leaf:
`140-native-whichkey`.)*
A grove-owned, **full-width bottom-bar** zellij plugin spanning every [[workspace]] tab, rendering the context-sensitive key hints (sigils, e.g. `⏎`/`⎋`) for whatever is focused (ADR-0019). It is the *single owner* of the bottom hint line: the [[dashboard]]/[[detail proxy]] and the harness stop drawing their own. Pinned in the bundled layout like the [[nav plugin]] sidebar; a sibling plugin to `grove-nav`, not the same instance.

**Leader** (control-seam key):
*(Binding updated by the [[trellis framework]] fork — ADR-0020: `Ctrl-o` now
triggers a **native in-process focus** of the nav surface, not
`LaunchOrFocusPlugin`. The key and its role are unchanged.)*
The single zellij locked-mode keybind that reaches grove's control — `Ctrl-o` (unchanged since ADR-0016), but bound to `LaunchOrFocusPlugin "grove-nav"` rather than `SwitchToMode "Normal"` (ADR-0018). It *must* be a zellij keybind: in `default_mode "locked"` only zellij intercepts keys while another app is focused; everything *after* the leader is handled by the [[nav plugin]]. The earlier `Ctrl-o`→`Normal` binding was a dead-end — grove's config (leaf 030) stripped all pane-navigation bindings, so unlocked `Normal` had no way to move focus.

## Flagged ambiguities

**"grove"** is overloaded across this codebase. It can mean:
1. The **CLI tool** / Rust crate published as `grove` — the verbs `install`, `do`, `status`, `retire`, etc.
2. The **methodology** bundled in `content/SKILL.md` and materialised into harnesses.
3. A single **workstream** — one named task tree under `.grove-worktrees/<name>/.grove/`. `grove do` operates on this sense.

When usage is ambiguous, qualify: "grove CLI", "grove methodology", "this grove".
