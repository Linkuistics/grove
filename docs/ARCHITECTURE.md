# Grove Architecture

Grove is a small Rust launcher around a filesystem task tree and an embedded
agent methodology. Durable work stays in ordinary repository files and VCS;
Grove adds only enough coordination to select one task, launch one fresh agent
session, and continue until the tree is complete.

## Documentation ownership

| Subject | Canonical source |
|---|---|
| Project description and installation | [`README.md`](../README.md) |
| Human workflow and commands | [`USAGE.md`](USAGE.md) |
| Harness, routing, models, and local state | [`CONFIGURATION.md`](CONFIGURATION.md) |
| Runtime and repository design | this document |
| Grove vocabulary | [`CONTEXT.md`](../CONTEXT.md) |
| Relationship between Grove and the skill plugins | [`CONTEXT-MAP.md`](../CONTEXT-MAP.md) |
| Methodology executed by agents | [`content/SKILL.md`](../content/SKILL.md) and its adjacent format guides |
| Skill-plugin operation | [`plugins/README.md`](../plugins/README.md) |
| Herdr plugin operation and maintenance | [`herdr-plugin/README.md`](../herdr-plugin/README.md) and [`herdr-plugin/MAINTENANCE.md`](../herdr-plugin/MAINTENANCE.md) |

The three files under `docs/` are the maintained project guides. This is not a
ban on durable artifacts produced by future Grove work: when a real decision,
specification, or research result earns a repository record, the methodology
may create focused files under `docs/adr/`, `docs/specs/`, or `docs/research/`.
Those sets describe current state and should be merged or deleted when they no
longer do; VCS holds their history.

The former decision-record slugs remain explicit HTML anchors in this document
(for example, `task-tree-scheme` and `symmetric-vcs-rule`). Source comments and
tests use those stable slugs as compact design references; changing a section
title does not change the anchor.

<a id="skills-monorepo"></a>
## Repository products

The repository contains three independently installed products:

| Product | Source | Delivery |
|---|---|---|
| Grove CLI and methodology | `src/`, `content/`, `build.rs` | Homebrew installs `grove` and `grove-llm`; `grove do` provisions the embedded methodology. |
| Agent skill plugins | `plugins/linkuistics/`, `plugins/testanyware/` | Claude marketplace or `plugins/install.sh` for portable Linkuistics skills. |
| Optional Herdr tree viewer | `herdr-plugin/` | Herdr plugin installation. |

Grove and the skill plugins share a repository because their documented
interfaces evolve together, but they do not install one another. The Herdr
viewer is optional and versions independently.

## Runtime flow

```text
human: grove do
        │
        ├─ locate nearest jj or Git working-tree root
        ├─ resolve primary harness and provision embedded content/
        ├─ migrate an old .grove/ layout if necessary
        └─ foreground loop
             │
             ├─ pick first live leaf (or bootstrap requirements)
             ├─ resolve leaf kind → harness → model
             ├─ preflight executable, prompt, Codex trust, and VCS access
             ├─ launch one harness session with its brief chain
             └─ read completion signal
                    ├─ relaunch → next fresh session
                    ├─ done     → stop
                    └─ absent   → stop safely; next `grove do` resumes
```

The driver stays in the foreground and owns its child process. Completion
signals are temporary control messages, not durable workflow state. Each
iteration re-checks the installed `grove-llm` version so a mixed binary install
cannot silently mutate a tree with a different format contract.

<a id="cli-binary-split"></a>
<a id="command-surfaces"></a>
## Command surfaces

The `grove` binary is for humans and deliberately has only `do`, `migrate`, and
`retire`. `grove-llm` is for deterministic operations invoked by the embedded
methodology: initialize, pick, resolve, add/insert/decompose, retire/prune,
signal completion, and report turns. This split keeps a discoverable human API
without forcing the agent to reproduce filesystem mutations from prose.

`src/main.rs` and `src/bin/grove-llm.rs` are thin entry points. `src/cli.rs`
owns the human grammar; `src/llm_cli.rs` owns the agent grammar.

<a id="task-tree-scheme"></a>
## Task-tree data model

The task tree is the state:

```text
.grove/
  BRIEF.md
  NN-[DONE-|ABANDONED-]<slug>-k<key>.md
  NN-<slug>-k<key>/
    BRIEF.md                 # optional for composition-only nodes
    NN-...                   # children use the same grammar
```

`NN` is a gapless, per-directory position and may change when inserting work.
`k<key>` is a permanent, globally unique identity and survives moves,
decomposition, and completion. A node is a directory; a leaf is Markdown.
`DONE` and `ABANDONED` are terminal filename infixes, so picking and rendering
need not parse file contents.

`tree_id` parses identities, `tree_read` walks and resolves them, `tree_grow`
creates leaves and composition shapes, `tree_rename` performs VCS-safe moves,
and `tree_lifecycle` applies terminal outcomes. `tree_migrate` is a bounded
adapter for prior layouts; it is not another live storage model.

Picking is a stateless depth-first pre-order walk over numeric sibling
positions. It returns the first live leaf and skips terminal entries. Grove
does not encode dependencies, priorities, or a scheduler outside this order.

<a id="task-kind-taxonomy"></a>
## Task kinds and composition

The closed kind set gives each session a discipline and gives the launcher a
routing key.

| Producer | Purpose | Review | Integration |
|---|---|---|---|
| `requirements` (HITL) | Establish what should be built through human dialogue. | `review-requirements` | `integrate-review-requirements` |
| `design` | Establish how; produce current-state specs or decisions. | `review-design` | `integrate-review-design` |
| `planning` | Decompose the design into vertical agent-sized leaves. | `review-planning` | `integrate-review-planning` |
| `prototype` (HITL) | Build a cheap artifact to provoke human reaction, not to ship. | `review-prototype` | `integrate-review-prototype` |
| `impl` | Produce shippable code, docs, or tests. | `review-impl` | `integrate-review-impl` |
| `research` | Produce a primary-source survey. | a second independent `research` | `combine-research` |

Reviews are fresh-context adversarial reads that produce findings rather than
fixes. Integrations verify each finding, then fix the contract, fix the
artifact, accept a visible trade-off, or reject noise. `requirements` and
`prototype` are human-in-the-loop because human words or reactions are their
essential input; any other kind may still stop and ask.

Two documented composition shapes are created as brief-less node directories:

- Review chain: `X → review-X → integrate-review-X`
- Research pair: `research@vendor-A → research@vendor-B → combine-research`

The directory provides containment and ordering. Grove does not validate a
cross-leaf grammar. The research pair is the reason a leaf may declare its own
harness; a kind-level rule cannot route two leaves of the same kind to two
vendors.

Once a session has run Bootstrap and adopted its own `pick`, a plain producer
may spend one in-session fresh-context reviewer across the whole leaf. A second
review need promotes that producer into a review chain. Producers already in a
chain, `review-*`, and research-pair leaves spend none; an
`integrate-review-*` leaf may spend one narrow reviewer and externalises
substantial redesign as a new producer review chain inside the owning node.
Sessions outside that procedural predicate retain standalone doubt behavior.

Generated chains carry stable task relationships independently of their names:
the review declares `**Reviews:** <producer-handle>` and integration declares
`**Integrates:** <review-handle>`. Names and positions remain presentation and
walk order, never relationship grammar.

### Tree access lock and promotion transaction

Every steady-state task-tree reader holds a shared **Tree access lock** on the
open `.grove/` directory; every mutator holds it exclusively through validation,
rollback, or success output. `leaf-promote-chain` uses that seam to move the
currently picked plain producer into a new brief-less review-chain node without
changing its stable handle or bytes. It derives the two remaining kinds and
relationships, allocates fresh keys, and emits paths only after the whole shape
lands.

The operation builds beneath a reserved `PROMOTING-<final-node-name>/` witness
and lands the directory with one same-parent rename. Every other task-tree
command refuses while a witness exists and names `leaf-promote-chain` recovery.
Jujutsu uses filesystem renames; tracked plain Git prepares the producer's final
index path while the witness still blocks readers. The contract is
process-interruption consistency, not power-loss durability. See [Promotion
transactions fail closed](adr/promotion-transactions-fail-closed.md).

<a id="self-driving-loop"></a>
<a id="do-is-sole-lifecycle-verb"></a>
<a id="fresh-grove-start-contract"></a>
## Lifecycle and resumption

`grove do` is the sole start/continue/finish entry. A missing `.grove/` is a
fresh workstream, so the first session is routed as `requirements` and creates
both the root brief and its first leaf. An existing tree resumes by picking its
first live leaf. An empty tree enters the finish cycle.

The loop launches one foreground session at a time. The session commits its
artifact and terminal task-tree mutation before signalling `relaunch` or
`done`. If it exits without a signal, the driver stops instead of guessing;
the filesystem and VCS already say what completed, and a later `grove do`
continues from there.

<a id="pruning"></a>
<a id="confirmation-boundary"></a>
<a id="in-session-finish-cycle"></a>
## Human authority and completion

Grove guides rather than gates. Any session may ask for clarification, but
the CLI has two explicit authority boundaries:

- Abandoning a planned leaf or subtree is human judgment and requires explicit
  confirmation before the agent marks it `ABANDONED`.
- Deleting the completed `.grove/` tree is the one routine finish confirmation.

Finishing happens inside the final session: promote durable information,
delete `.grove/` in one focused commit, then signal `done`. Grove deliberately
does not merge branches/bookmarks or remove working trees.

<a id="user-owned-worktrees"></a>
<a id="symmetric-vcs-rule"></a>
<a id="version-control-seam"></a>
## Version-control seam

Grove walks upward from the current directory and lets the closest repository
marker decide. `.jj/` wins over a colocated `.git`; otherwise `.git` selects
Git. Jujutsu working copies are mutated with ordinary filesystem renames and
committed with Jujutsu. Git working copies use `git mv` for tracked moves and
Git for commits. This preserves Jujutsu's operation log and avoids mutating the
Git index behind a colocated repository.

The user owns topology. Grove reads no branch or bookmark, creates no working
tree, and performs no integration or teardown. The working-tree basename is
the grove name.

<a id="codex-gitdir-grant"></a>
## Codex sandbox access

A Codex session may need to write outside the visible working tree: a Git
worktree's shared gitdir or a Jujutsu workspace's backing store. Before launch,
Grove resolves the relevant VCS storage and appends narrowly scoped Codex
`--add-dir` grants. It also rejects an untrusted read-only workspace before
starting a session that could not commit. Other harnesses receive no Codex
arguments.

Loop-control environment variables are scrubbed from every auxiliary harness
spawn and granted only to the foreground loop child. This prevents a nested or
one-off session from completing an unrelated outer loop.

<a id="model-per-task-kind"></a>
## Harness and model routing

The launcher performs one structured peek returning the next leaf's path,
stable handle, kind, and optional harness line. It retains that one result for
readiness, the launch line, routing, and producer-target handoff, then applies
the policy described in [CONFIGURATION.md](CONFIGURATION.md): leaf, kind,
family, primary binding for harness selection; harness-scoped kind,
harness-scoped family, unscoped kind, unscoped family for models. Missing or
invalid routing fails before spawn rather than launching the wrong agent.

Routing uses each harness's native model/profile flag. There is no proxy,
router service, or persisted model state.

The foreground producer session receives its effective target as internal
launch context. When `leaf-retire` finds one sibling review whose `Reviews`
relationship names that producer, it applies `DONE` first and then atomically
replaces the review's best-effort `**Producer launch:**` receipt. Worktree,
routed-handle, and factual-pick mismatches make the receipt uncheckable but never
reverse retirement.

At `review-*` launch the driver compares that historical receipt with the
review's newly resolved target. It warns unless both harness and exact model
selector differ, emitting one advisory notice to stderr and the session prompt
without blocking launch. Missing, malformed, or mismatched stable
relationships are uncheckable rather than inferred from positions. See [Review
target receipts](adr/review-target-receipts.md).

<a id="self-extension-core-and-methodology"></a>
## Embedded methodology

`build.rs` embeds `content/` into the binary. On every `grove do`, `provision`
writes the launching harness's personal `grove` skill and any other installed
harness targets. A content hash makes this idempotent while still updating the
skill when the binary changes.

The binary refuses to overwrite an unstamped foreign directory and replaces an
old symlink as a link rather than following it. `content/` is the canonical
source; repository-local or hand-edited copies are not supported.

<a id="herdr-optional-ui"></a>
<a id="herdr-turn-boundary-hooks"></a>
<a id="optional-herdr-integration"></a>
## Optional Herdr integration

The Python plugin reads `.grove/` from disk and renders it. It does not call
Grove, open a socket, or write state, so removing it cannot affect task
execution.

Separately, the Rust driver reports best-effort pane states when Herdr's pane
environment is present. The driver sees session boundaries; Claude-only hooks
report prompt, stop, dialog, and tool boundaries that occur inside a session.
Those hooks are injected for a launched session and persist no harness
configuration. Reporting failures are ignored, preserving the rule that Herdr
is observability rather than authority.

The maintained fork and release procedure is component-specific and lives in
[`herdr-plugin/MAINTENANCE.md`](../herdr-plugin/MAINTENANCE.md).

## Main module seams

| Module | Responsibility |
|---|---|
| `launch` | Repository/harness resolution, provisioning, preflight, and top-level commands. |
| `loop_driver` | Foreground iteration, task routing, child lifecycle, and completion signals. |
| `harness`, `harness_stamp` | Harness registry, detection, and persistent local binding. |
| `repo`, `tree_rename` | Git/Jujutsu detection and mutation seam. |
| `tree_id`, `tree_read`, `tree_grow`, `tree_lifecycle`, `tree_migrate` | Filesystem task-tree model. |
| `leaf`, `llm_cli`, `complete` | Task formats and deterministic agent command surface. |
| `provision` | Embedded methodology installation. |
| `herdr` | Optional pane-state reporting. |

The modules are intentionally file-sized rather than wrapped in another
service layer. The task tree, subprocess boundary, and VCS adapter are the
important seams and are tested through public behavior.

## Verification

The principal checks are:

```sh
cargo fmt --check
cargo test --locked
bash plugins/install.test.sh
```

Integration tests use temporary Git and Jujutsu repositories, isolated home
directories, fake harness executables, and the real `grove-llm` binary. The
Herdr renderer remains dependency-free Python and can print one frame without a
TTY for inspection or piping.
