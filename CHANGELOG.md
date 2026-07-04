# Changelog

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
