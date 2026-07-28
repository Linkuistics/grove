# grove

Hierarchical, self-extending workstream tool for AI agents. See [`docs/grove.md`](docs/grove.md) for what grove is and why, [`content/SKILL.md`](content/SKILL.md) for the methodology agents read at runtime; this README covers the CLI.

## What's in this repo

Two components, shipped by two paths, plus one optional extra. The two live together because they change in lockstep — see [`docs/adr/skills-monorepo.md`](docs/adr/skills-monorepo.md); the skill plugins were formerly the separate `Linkuistics/skills` repo, grafted in with its history.

| | What it is | How you install it |
|---|---|---|
| **grove** — [`src/`](src/), [`content/`](content/) | the CLI plus the workstream methodology it embeds and provisions to your agent | `brew install grove` — see [Install grove](#install-grove) below |
| **skill plugins** — [`plugins/`](plugins/) | `linkuistics` (coding standards) and `testanyware` (GUI testing in VMs) | Claude Code marketplace, or [`install.sh`](install.sh) for codex/gemini — see [`plugins/README.md`](plugins/README.md) |
| **herdr plugin** — [`herdr-plugin/`](herdr-plugin/) | optional: renders a live `.grove/` tree in a [herdr](https://herdr.dev) pane | `herdr plugin install Linkuistics/grove/herdr-plugin` — see [`herdr-plugin/README.md`](herdr-plugin/README.md) |

**Which do you want?** If you are here to drive long multi-session workstreams, you want grove — keep reading. If you are here for the coding-style, design, ADR, jj or GUI-testing skills, you want the plugins: go to [`plugins/README.md`](plugins/README.md), which lists every skill and what triggers it. Installing one does not install the other; the `grove` binary provisions grove's own methodology and nothing else.

The herdr plugin is a viewer, never a dependency: it reads `.grove/` off disk and grove pushes it nothing, so a grove with no herdr behaves identically (`docs/adr/herdr-optional-ui.md`).

The repo's two bounded contexts and how they relate are mapped in [`CONTEXT-MAP.md`](CONTEXT-MAP.md), over [`CONTEXT.md`](CONTEXT.md) (grove) and [`plugins/CONTEXT.md`](plugins/CONTEXT.md) (skills).

## Install grove

(For the skill plugins instead, see [`plugins/README.md`](plugins/README.md).)

```
brew tap Linkuistics/taps
brew install grove
```

That is the whole installation of grove itself. The `grove` binary embeds its full methodology and provisions it to your **personal** skill dir (`~/.claude/skills/grove/`) on the first `grove do` — idempotent against a content-hash stamp, re-provisioned whenever the binary changes (self-extension-core-and-methodology / task-tree-scheme). There is no per-repo install step and nothing to keep in sync; `grove --version` reports the binary's version.

## Prerequisite

grove's methodology defers two bodies of guidance to the **`linkuistics` plugin**: decision-record philosophy, format and the when-to-write test to `linkuistics:decision-records`, and test-seam judgement to `linkuistics:codebase-design`. Grove's bundled `ADR-FORMAT.md` and `SPEC-FORMAT.md` keep only grove's own placement and recording conventions, so a session raising an ADR or sketching a spec's seams consults those skills.

Install the plugin alongside grove. It is hosted in this repo under [`plugins/linkuistics/`](plugins/linkuistics/) but is **still a separate install** — the `grove` binary provisions grove's methodology and nothing else. See [`plugins/README.md`](plugins/README.md) for the two commands. The dependency is documentation-level (grove does not enforce it at install time); everything else grove needs is embedded in the binary.

## Use

**Drive a grove workstream:**

```
grove do                   # the sole lifecycle entry verb, run from inside your working tree
grove retire <node-path>   # promote a finished node's brief upward (its leaves stay marked done in place)
```

`grove do` is **argument-less** and run from inside a working tree you provide — git or jj-enabled — and grove never creates, integrates, or tears down working trees, branches, or bookmarks, and reads no branch and no bookmark anywhere (VCS topology is entirely yours: plain git/gh or jj, or a dedicated worktree manager such as [worktrunk](https://github.com/max-sixty/worktrunk)). It is the **sole lifecycle entry verb**: it inspects the grove's state on disk and dispatches — no `.grove/` yet → open a bootstrap session; a live tree → continue. The former `grove start` and `grove continue` are removed (`do` already covered both). The former `grove finish` is also removed: a grove is now finished **in-session** — when it has no live leaves left, the running loop proposes the complete finish cycle (promote durable content out of the briefs, delete `.grove/` in one commit, signal the loop to stop). See the methodology's *Finish* step.

Each verb takes optional `--harness <name>` and respects auto-detection from the repo's harness directories (`.claude/`, `.codex/`, `.pi/`). Session launchers stamp `.grove-stamps/<name>` (keyed by the working tree's basename) whenever `--harness` is passed explicitly, and also when needed for disambiguation in multi-harness repos — a single-harness repo relying on auto-detection stays stamp-free. The grove's name is the working tree's own directory basename; the task tree itself is `.grove/` inside it.

See `grove --help` for flag details. For end-to-end walkthroughs of each verb in context, see [`docs/workflows/`](docs/workflows/).

## Configuration

Before each task launch, `grove do` reads the **kind** of the leaf it's about to run and resolves two things from the environment: **which harness** runs it, and **which model** that harness loads. Selection uses each harness's own native launch flags — no router, no proxy.

Every leaf declares one of **seventeen** kinds: five producers — `requirements`, `design`, `planning`, `prototype`, `impl` — each with its own `review-<producer>` and `integrate-review-<producer>` step, plus `research` and `combine-research`. Two **families** group them for configuration, so one line covers five kinds: `REVIEW` (every `review-*`) and `INTEGRATE_REVIEW` (every `integrate-review-*`). Variable names uppercase the label and map `-` to `_` — `review-impl` → `REVIEW_IMPL`. See [`docs/specs/task-kind-taxonomy.md`](docs/specs/task-kind-taxonomy.md) for each kind's discipline.

### Which harness

Most precise wins — **leaf, then kind, then family, then the stamp**:

| Source | Applies to |
|---|---|
| a `**Harness:** <name>` line on the task file | that one leaf. Rare — it exists for the research **vendor pair**, two `research` leaves differing only by vendor, the one shape a per-kind policy cannot express. Read *strictly*: an unknown name refuses to launch rather than degrading. |
| `GROVE_<KIND>_HARNESS` | one kind, e.g. `GROVE_REVIEW_IMPL_HARNESS=codex` |
| `GROVE_<FAMILY>_HARNESS` | a whole family, e.g. `GROVE_REVIEW_HARNESS=codex` |
| *unset* | the harness this grove is **stamped** to (`.grove-stamps/<name>`) — an explicit on-disk binding, not a default |

### Which model

Four keys, **harness-major** — `<HARNESS>` is `CLAUDE`, `CODEX` or `PI`:

| Key, most specific first | Example |
|---|---|
| 1. `GROVE_<HARNESS>_<KIND>_MODEL` | `GROVE_CODEX_REVIEW_IMPL_MODEL` |
| 2. `GROVE_<HARNESS>_<FAMILY>_MODEL` | `GROVE_CODEX_REVIEW_MODEL` |
| 3. `GROVE_<KIND>_MODEL` | `GROVE_IMPL_MODEL` |
| 4. `GROVE_<FAMILY>_MODEL` | `GROVE_INTEGRATE_REVIEW_MODEL` |

The harness axis outranks the kind axis because the two are different kinds of constraint: crossing the harness axis can yield a value that is **invalid** rather than merely less specific (a codex profile name is garbage to pi), while a family's model is less specific yet still your own choice. A **rerouted** launch — the leaf's harness is not the one the grove is stamped to — consults keys 1–2 only, since an unscoped var was written with some *other* harness in mind.

The value is passed via the harness's own flag: `--model` for claude and pi (pi accepts `provider/id` patterns), `--profile` for codex (a codex profile binds model *and* reasoning effort, which a bare model flag cannot express — define profiles as `$CODEX_HOME/<name>.config.toml` files, not a `[profiles.<name>]` table in `config.toml`).

Two things to know:

- **A model is required — an unset variable is an error, not a fallback.** If the picked leaf's kind resolves none of the four keys, grove fails loudly and names every variable that would satisfy it. There is no fall-through to the harness's own default, because that is still grove choosing which model runs a `review-impl` leaf — it just chooses invisibly, and it makes a half-configured setup indistinguishable from a complete one. Three exemptions, each an *absence of the question* rather than a default: no live leaf (the finish-cycle iteration has no task to require a variable for); a harness that takes no model flag at all; and an unset `GROVE_<KIND>_HARNESS`, which means the stamp.
- **The launch model is a default, not a lock.** An in-session `/model` switch outranks the flag for that one session. It does not survive into the next task — every launch passes a flag again.
- **A brand-new grove needs `GROVE_REQUIREMENTS_MODEL`** (or its harness-scoped spelling, `GROVE_<HARNESS>_REQUIREMENTS_MODEL`). Every other launch is routed by peeking the picked leaf's kind, but on a fresh working tree there is no `.grove/` to peek — the agent creates it *inside* that first session — so `grove do` routes the bootstrap by construction, as `requirements` ([`docs/adr/fresh-grove-start-contract.md`](docs/adr/fresh-grove-start-contract.md)). Leave it unset and the very first `grove do` fails.
- **A codex grove needs the working tree *trusted*.** codex's sandbox defaults to `read-only` for any project you have not trusted, and trust does not inherit from parent directories — so a brand-new working tree is untrusted even inside a trusted one. Under `read-only` codex refuses the VCS-store grants a grove session commits through ([`docs/adr/codex-gitdir-grant.md`](docs/adr/codex-gitdir-grant.md)), so grove pre-flights every codex launch and refuses before spawning rather than starting a session that cannot commit. Run `codex` once in the tree and accept the prompt, or set `trust_level = "trusted"` for it in `$CODEX_HOME/config.toml`. Only codex is affected; the pre-flight is skipped for every other harness.

### A worked configuration

claude leads, codex reviews, claude integrates the review. The stamp absorbs every kind that is *not* rerouted, so the whole policy layer is two lines and one family variable does the work of five kinds:

```
# reviews go to codex (one line, all five review-* kinds), on a codex profile;
# rerouted, so it needs the CODEX-scoped spelling
export GROVE_REVIEW_HARNESS=codex
export GROVE_CODEX_REVIEW_MODEL=sol-xhigh

# everything else falls through to the stamped harness (claude)
export GROVE_REQUIREMENTS_MODEL=opus
export GROVE_DESIGN_MODEL=opus
export GROVE_PLANNING_MODEL=opus
export GROVE_PROTOTYPE_MODEL=sonnet
export GROVE_IMPL_MODEL=sonnet
export GROVE_RESEARCH_MODEL=opus
export GROVE_COMBINE_RESEARCH_MODEL=opus
export GROVE_INTEGRATE_REVIEW_MODEL=opus   # family: all five integrate-review-*
```

Ten variables against a ceiling of 95 (17 kinds × 5, plus 2 families × 5). The ceiling is not the burden — the stamp absorbs everything not rerouted. **Use the harness-scoped spellings instead of the unscoped ones if you drive groves stamped to more than one harness**, since an unscoped value would follow a kind onto a harness it was never written for.

See [`docs/adr/model-per-task-kind.md`](docs/adr/model-per-task-kind.md) for the rationale (why native launch flags rather than a router/proxy, and why a missing variable errors).
