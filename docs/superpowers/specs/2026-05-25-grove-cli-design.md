# Grove CLI — design spec

**Date:** 2026-05-25
**Status:** Draft, pre-implementation
**Author:** brainstormed with Claude Opus 4.7

## Goal

Package the grove skill as a Homebrew-distributed Rust CLI so the methodology
is usable through one command (`grove`) from any shell, in any repo, without
clones-of-clones or per-repo path management. The CLI absorbs today's
`materialise-grove.sh` (install/update plumbing) and today's `grove-start` /
`grove-next` shell helpers (session launchers), and extends to additional
harnesses (initially Claude Code and Codex).

## Non-goals

- **Not** a generic skill installer. This CLI is scoped to one skill (grove).
  If other skills want similar treatment they get their own binary.
- **Not** a grove-content runtime. The CLI launches harness sessions and
  manages on-disk state; it does not run grove itself. The harness (Claude
  Code, Codex) is the runtime.
- **Not** the long-term home of grove's content. Content lives in
  `Linkuistics/grove/content/`; the CLI is its transport.
- **Not** a `grove finish` policy engine. `finish` launches a session and
  lets the harness handle the merge ritual per project convention.

## Architecture overview

```
┌─ User's machine ──────────────────────────────────────┐
│                                                       │
│  brew tap Linkuistics/taps   (→ Linkuistics/homebrew-taps)
│  brew install grove                                   │
│      └→ /usr/local/bin/grove   (Rust static binary)   │
│                                                       │
│  $ grove install ~/projects/Modaliser                 │
│      └→ fetches Linkuistics/grove@vX.Y.Z tarball,     │
│         extracts content/* into                       │
│         <repo>/.<harness>/skills/grove/               │
│                                                       │
│  $ grove start auth-rewrite                           │
│      └→ creates worktree + branch, execs the          │
│         harness with a tailored prompt                │
└───────────────────────────────────────────────────────┘
```

Two repos:

| Repo | Role |
|---|---|
| `Linkuistics/grove` *(new)* | Rust CLI source (`Cargo.toml`, `src/`) + `content/` tree (SKILL.md, format docs, grilling.md, prompts). Tagged `v0.1.0`, `v0.2.0`. GitHub Releases provide source tarballs and prebuilt binaries. |
| `Linkuistics/homebrew-taps` *(exists)* | Adds `Formula/grove.rb` pointing at a `Linkuistics/grove` release. Bumped when the CLI binary changes; **not** bumped for content-only tags (those land via `grove install --version`). |

## `Linkuistics/grove` repo layout

```
Linkuistics/grove/
├── Cargo.toml
├── README.md
├── CHANGELOG.md
├── src/                        # Rust CLI source
│   ├── main.rs
│   ├── cli.rs                  # clap definitions
│   ├── harness.rs              # harness registry
│   ├── install.rs              # install / update / uninstall
│   ├── launch.rs               # start / continue / takeover / retire / finish
│   ├── fetch.rs                # GitHub tarball fetcher
│   └── ...
├── content/                    # The materialised payload
│   ├── SKILL.md
│   ├── BRIEF-FORMAT.md
│   ├── CONTEXT-FORMAT.md
│   ├── TASK-FORMAT.md
│   ├── ADR-FORMAT.md
│   ├── grilling.md
│   ├── LICENSES/
│   └── prompts/                # Launcher prompts (CLI reads at exec time)
│       ├── start.md
│       ├── continue.md
│       ├── takeover.md
│       ├── retire.md
│       └── finish.md
└── .github/workflows/release.yml
```

### Version model

The whole repo is tagged as one unit using semver: `vMAJOR.MINOR.PATCH`.

- **PATCH** — content edits, prompt tweaks, bug fixes.
- **MINOR** — new CLI verbs / flags, backward-compatible.
- **MAJOR** — breaking CLI flag/output changes, content reorganization that
  changes install layout.

The brew formula bumps when the **CLI binary** changes (typically MINOR or
MAJOR). Content-only tags (PATCH-typical) don't require a formula bump:
users `grove install --version vX.Y.Z` from any installed CLI.

## CLI command surface

### Conventions

- Where a verb takes `[<repo>]`, omitting it means "cwd's git root" (resolved
  via `git rev-parse --show-toplevel`). If cwd is not in a git repo and no
  `<repo>` is passed, the verb errors.
- Where a verb takes `--harness <name>`, the flag is **repeatable**:
  `--harness claude --harness codex` operates on both.
- Where a verb resolves a content version (`install`, `update`), `--version
  <tag>` overrides; default is the highest semver-sorted `v*` tag on
  `Linkuistics/grove`'s default branch, queried via GitHub API at exec time.

### File-system verbs (no harness session)

```
grove install   [<repo>] [--harness <name>]... [--version <tag>]
grove update    [<repo>] [--harness <name>]... [--version <tag>]
grove uninstall [<repo>] [--harness <name>]... [--force]
grove init      [<repo>] [--harness <name>]...
grove version
grove status    [<repo>]
grove list      [<repo>]
```

| Verb | Behavior |
|---|---|
| `install` | Create-only. Errors if `<repo>/.<harness>/skills/grove/` already exists. |
| `update` | Update-only. Errors if `<repo>/.<harness>/skills/grove/` doesn't exist. Re-materialises, bumps `VERSION.md`, prints reminder to record the bump as an ADR per `ADR-FORMAT.md`. |
| `uninstall` | Remove `<repo>/.<harness>/skills/grove/`. Refuses if `<repo>/groves/` has live (non-`done/`) leaves unless `--force`. |
| `init` | Composite first-time setup. Runs `install` + creates `CONTEXT.md` (if missing) + `docs/adr/` placeholder + prints "next: `grove start <name>`". |
| `version` | Print CLI version. If cwd is in a grove-installed repo, print installed content version per harness (read from `VERSION.md`). |
| `status` | Per harness present: installed content version, drift flag if multiple harnesses diverge. Per grove in `<repo>/groves/`: live-leaves count, retired count, worktree present, `.harness` stamp if any. |
| `list` | Groves in `<repo>` as one-per-line names — scriptable. |

### Session-launching verbs

Each one creates/finds the right worktree, reads its prompt from
`<repo>/.<harness>/skills/grove/prompts/<verb>.md`, and execs the harness
process with a tailored argv. The prompt is the materialised content, so
flow tweaks ship as content tags (no CLI release).

```
grove start    <name> [--start-point <ref>] [--harness <name>] [--no-launch]
grove continue <name> [--harness <name>] [--no-launch]
grove takeover <name> [--harness <name>] [--no-launch]
grove retire   <name>/<NNN-node> [--harness <name>] [--no-launch]
grove finish   <name> [--harness <name>] [--no-launch]
```

| Verb | What it does |
|---|---|
| `start` | New grove: create worktree at `<repo>/worktrees/<name>-grove/` on a new branch `<name>-grove`, write `groves/<name>/.harness` only if disambiguation was required, launch harness with `prompts/start.md`. |
| `continue` | Resume: cd to worktree, launch harness with `prompts/continue.md`. |
| `takeover` | Orient on an unfamiliar grove: launch harness with `prompts/takeover.md` — read CONTEXT, brief chain, recent git activity; report state without picking a task. |
| `retire` | Run the node-retirement ritual: promote brief upward, `mv` into `done/`, one focused commit. Argument is `<name>/<node-path>` where `<node-path>` is the path of the node directory relative to `groves/<name>/` (e.g. `003-auth/002-session-store`). |
| `finish` | Grove-is-done: launch harness with `prompts/finish.md`. The prompt instructs the harness to follow the project's merge/cleanup convention (PR vs ff-merge vs squash) — CLI doesn't encode merge policy. |

The `--no-launch` flag does all the on-disk setup but skips the harness
exec; useful for testing and for users who prefer to launch the session
themselves.

### Deliberately not CLI verbs

- **"Run a planning task explicitly" / "Run a work task explicitly"**
  (docs/grove.md flows #3, #4) — these are "I know what's coming" overrides.
  `continue` auto-picks. Users wanting the override can launch the harness
  with the prompt themselves. Adding `grove plan` / `grove work` would
  clutter the surface for rare cases.

## Harness model

A harness is a known agent-runtime that grove can be materialised for. The
registry is a small table in Rust:

```rust
struct Harness {
    name: &'static str,       // "claude", "codex"
    project_dir: &'static str // ".claude", ".codex"
}

const HARNESSES: &[Harness] = &[
    Harness { name: "claude", project_dir: ".claude" },
    Harness { name: "codex",  project_dir: ".codex"  },
];
```

Each harness's install path is `<repo>/<project_dir>/skills/grove/`. New
harnesses (gemini, copilot) are additive rows.

The CLI never inspects `PATH` and has no knowledge of harness binaries (no
`which claude`, no version probes). Its world is filesystem directories.

### Autodetection

Single signal: which harness directories exist in `<repo>`.

```
For install / init / update:
  Scan <repo> for harness dirs:
    only .claude/        → claude
    only .codex/         → codex
    both                 → both (--harness narrows)
    neither              → error: "no harness session detected in <repo>;
                           run the harness at least once in this repo or
                           pass --harness explicitly"

For uninstall / status / list / version:
  Enumerate existing <repo>/.<harness>/skills/grove/. No detection needed.

For start / continue / takeover / retire / finish:
  Read groves/<name>/.harness if present.
  Else, repo scan:
    exactly one harness dir → use it
    both                    → error: "ambiguous; pass --harness explicitly"
    neither                 → error
```

**Premise:** by the time `grove install` is run on a repo, the repo has
already had at least one harness session in it. This is a reasonable
constraint — grove has no value in a virgin project.

### Content portability

Today's grove content (SKILL.md, docs) contains some Claude-isms — mentions
of `/clear`, `/rename`, paths like `.claude/skills/grove/`. **For v1 these
are left as-is.** Codex users reading the materialised file see Claude-
flavored examples but the methodology (`groves/` tree, BRIEF chains,
CONTEXT.md, retirement ritual) is harness-agnostic.

**Future enhancement, deliberately out of v1 scope:** placeholder
substitution at extract time. The content would use `{{SKILLS_DIR}}`,
`{{SESSION_RESET}}`, etc. and the CLI would substitute per harness. Add
this when a Codex user actually trips over the Claude-isms.

## Asset fetch flow

`grove install` and `grove update`:

1. Resolve target version. `--version <tag>` overrides; default is the
   latest `v*` tag on `Linkuistics/grove` (queried via GitHub API). The
   resolved tag is printed before the fetch.
2. Download `https://github.com/Linkuistics/grove/archive/refs/tags/<tag>.tar.gz`.
3. Extract `<tarball>/content/*` into `<repo>/.<harness>/skills/grove/`.
4. Stamp `<repo>/.<harness>/skills/grove/VERSION.md` (see schema below).
5. For `install`: error if directory existed beforehand. For `update`:
   error if it didn't.

No local cache for v1. Each install hits GitHub. Caching is a future
enhancement (e.g., `~/.cache/grove/<tag>.tar.gz`).

### `VERSION.md` schema

```markdown
# grove — materialised version

A materialised copy of the grove skill for the `<harness>` harness.

| | |
|---|---|
| version | `vX.Y.Z` |
| materialised on | YYYY-MM-DD |
| materialised into | `.<harness>/skills/grove/` |

## Updating

```
grove update --version <tag>
```
```

The CLI parses `version` back out when computing `grove status` / drift.

## `.harness` stamp

A grove's harness binding lives at `groves/<name>/.harness`. It's written
**only when needed for disambiguation** — a single-harness repo never has
`.harness` files in its groves.

- Format: one line, the harness name (e.g. `claude`).
- Written by: `grove start --harness <name>` when the repo has multiple
  harness dirs. (Single-harness `grove start` does not write it.)
- Read by: every session-launcher when present.
- Committed: yes — it's grove state, like the BRIEF chain.
- If absent and the repo is multi-harness: launcher errors and asks for
  `--harness`; the user's first explicit choice gets stamped.

## Migration plan

### In `Linkuistics/skills` (after `Linkuistics/grove` v0.1.0 ships)

1. Delete `grove/` directory (now lives in `Linkuistics/grove/content/`).
2. Delete `scripts/materialise-grove.sh` and its test.
3. Replace `docs/grove.md` with a one-paragraph pointer to the new repo
   and the brew install command. Keep the file around so existing links
   don't 404.
4. CHANGELOG entry noting the move.

### In existing consuming projects (per-project, as users upgrade)

Existing materialised installs continue to work — they have the old
launchers (`grove-start`/`grove-next`) bundled, so users can keep using
them until they `brew install grove` and `grove update <repo>`, at which
point the new (launcher-less) content replaces the old.

When a consuming project upgrades to the CLI-managed flow, record the
switch as an ADR (grove's standing discipline: record version bumps,
especially methodology-level ones, as ADRs).

## Build & release

GitHub Actions on tag push:

- Build Rust binaries: `darwin-arm64`, `darwin-x86_64`, `linux-x86_64`,
  `linux-arm64`.
- Upload to GitHub Releases for the tag.
- For tags that bump the CLI binary, follow up with a PR to
  `Linkuistics/homebrew-taps` updating `Formula/grove.rb` (version, url,
  sha256). Manual for v1; automated later.

Brew formula skeleton:

```ruby
class Grove < Formula
  desc "Hierarchical, self-extending workstream tool for AI agents"
  homepage "https://github.com/Linkuistics/grove"
  version "0.1.0"

  on_macos do
    on_arm do
      url "https://github.com/Linkuistics/grove/releases/download/v0.1.0/grove-darwin-arm64.tar.gz"
      sha256 "..."
    end
    # ...
  end

  def install
    bin.install "grove"
  end
end
```

## Open questions / future enhancements

- **Caching of fetched tarballs** at `~/.cache/grove/`. Offline `grove
  install --version <tag>` for any version previously fetched.
- **Placeholder substitution** for harness-specific content. Triggered by
  a real Codex pain point, not speculatively.
- **`grove finish` automation**. v1 just launches a session. A future
  version could integrate with the host project's merge policy (PR vs
  ff-merge) when that pattern stabilises.
- **Plugin-style global install option**. If users start wanting grove
  available without per-repo install, a `~/.<harness>/skills/grove/`
  install path could be added — at the cost of per-repo version pinning.
  Explicitly off-spec for v1.

## Out of scope

- Generic multi-skill installer.
- Embedding the harness binary itself (we exec the user's installed
  `claude` / `codex`).
- Hosting alternative content sources (forks of grove).
- Windows support for v1 (macOS + Linux only).
