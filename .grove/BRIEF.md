# grove.refactor-for-minimalism — brief

## Goal

Decompose grove into **five modules with independent lifetimes**, three of which
are reusable outside grove, and shrink what remains of grove itself to the loop
that composes them. The target is a launcher that owns a loop and a vocabulary,
and nothing else.

| module | owns | domain-free |
|---|---|---|
| **tree store** | the on-filesystem ordered tree: read, mutate, `exists?`, `initialize`, `delete`, and a no-outcome answer to a search | yes |
| **runner** | a config file of `key → complete command template`, whole-word expansion, direct argv spawn, session supervision, the out-of-band completion signal, kill escalation | yes |
| **loop** | compose the two: `exists? → create or find next → determine the command → run → finalise`. Holds grove's vocabulary — kind, handle, prompt composition — and the one-driver-per-workspace lease | no |
| **skills** | the methodology, and the routing from task type to procedure | no |
| **VCS seam** | taking a commit, and the guidance printed when grove declines to proceed | partly |

## Done when

- Each module is testable through its own interface without the other four.
- The tree store is the only thing that touches the task tree.
- The runner is the only thing that spawns or supervises a process.
- Adding a task type is authoring content, not editing and rebuilding a binary.
- No module reimplements a guarantee the VCS already provides.

## Decomposition

Ordering is by what unblocks what. `docs/specs/module-decomposition.md` is the
input to every leaf below and is **not to be redesigned**; each leaf implements
the decisions it names.

### The ordered run

| # | leaf | what it lands |
|---|---|---|
| 1 | `delete-migration-k6` | migration and the legacy tree format, deleted |
| 2 | `drop-git-lane-k7` | jj is the only lane; git-index machinery deleted |
| 3 | `delete-finish-transaction-k8` | the hand-built finish transaction, deleted |
| 4 | `extract-jj-workspace-k9` | the VCS seam as a crate |
| 5 | `keyed-launch-templates-k10` | the runner's template half + per-kind configuration |
| 6 | `keyed-launch-run-k11` | the runner's channel, spawn and escalation |
| 7 | `store-operations-k12` (node) | `sought-k24`, `open-shape-k25`, `root-delete-k26` |
| 8 | `collapse-tree-access-k13` | grove's second lock layer, deleted |
| 9 | `name-ownership-k14` | one type owns the name; the handle is a rendering |
| 10 | `grammar-separator-k15` | the `--` grammar, the tree rename, the reinstall |
| 11 | `plugin-spine-k16` | the plugin, the shared spine, the conformance runner |
| 12 | `plugin-kind-skills-k17` | one `grove-<kind>` skill per kind |
| 13 | `prompt-names-the-kind-k18` | the prompt names one skill and publishes a version |
| 14 | `delete-provisioning-k19` | provisioning, the harness registry, the embed |
| 15 | `open-kind-k20` | `Kind` opens; the kind list add; `--kind` required |
| 16 | `loop-crate-verbs-k21` | the tree layer and the twelve verbs as a crate |
| 17 | `loop-crate-driver-k22` | the driver, the lease, the loop; thin binaries |
| 18 | `spec-to-current-state-k23` | the spec rewritten; the whole set closed out |

### Where each spec decision lands

Written down so coverage can be checked without re-reading both documents.

| spec decision | leaf |
|---|---|
| 1 — four library crates, two binaries, one plugin | k9, k10, k16, k21, k22 |
| 2 — the tree store's four new operations | the `store-operations-k12` node |
| 3 — the filename grammar gains a separator | k15 |
| 4 — one type owns the name; the handle renders through it | k14 |
| 5 — grove names a kind only where grove writes the leaf | k20, with the two interpreted sites dying at k18 |
| 6 — configuration completeness becomes per-kind and just-in-time | k10 |
| 7 — the runner | k10 (templates), k11 (launch) |
| 8 — the VCS seam | k7, k8, k9 |
| 9 — the loop | k21 (verbs), k22 (driver) |
| 10 — grove publishes its version in the prompt | k18 |
| 11 — the methodology ships as a plugin; skill fatness | k16, k17, with the old path deleted at k19 |
| requirement: one reading per filename | k15 |
| requirement: grove names only the kinds it writes | k20 |
| requirement: an overlay overrides and never supplies | k10 |
| requirement: no module implements a VCS guarantee | k8 |
| test seam 1 — each crate's public interface | k9, k10, k11, k21, k22, and each store leaf |
| test seam 2 — one composed loop over a fake harness | k22 |
| test seam 3 — conformance kits as the cross-crate seam | k10 (the store's already exists) |
| test seam 4 — the methodology's delivery assertion | k16, k17 |
| out of scope: migration deleted | k6 |
| out of scope: the plain-git lane dropped | k7 |
| out of scope: the harness-registry row answered by deletion | k19 |
| out of scope: the release manifest exclusion removed | k23 |
| out of scope: extracting the tree store to its own repository | **deferred, no leaf** — its documents stay put; only the manifest exclusion moves, at k23 |
| out of scope: serving the methodology over MCP | **rejected, no work** — recorded at k19, which states the rejection where provisioning dies |
| out of scope: invoking a harness plugin | **rejected, no work** — a command template expresses it today; anything more is a runner capability, and k10 owns the runner's contract |

`minimalism-k1`'s `## Deletion list` — roughly 15,200 non-test lines — is spread
across k6, k7, k8, k13, k18 and k19; its one awkward row, `tree_access`'s seven
surviving call sites, is k13's alone. Its two *Reconciled* rows for the delivery
path split: k18 takes `src/prompt.rs`'s content dependency, and `src/methodology.rs`
goes at **k19** with the provisioning that calls it (see the forced ordering below).

### The orderings that are forced

Everything else in the run above is convenience. These are not.

| constraint | why |
|---|---|
| k6, k8 and the store node all precede k13 | grove's second lock layer has three recorded reasons — *absent*, *legacy*, *mid transaction* — and they dissolve **at once**, not one at a time |
| k14 precedes k15 precedes k20 | the handle needs one owner before the grammar moves; the grammar must be unambiguous before the kind opens |
| k15's rename and its reinstall are **one leaf** | this is a meta-grove: the tree cannot wear a grammar the installed binary does not parse, and a session that renames and stops has wedged the loop |
| k6's `FORMAT` deletion and its reinstall are **one leaf** | the installed binary requires the witness and reads its absence as a legacy tree (`src/tree_format.rs:7-29`); worse, the old driver's per-iteration transition reaches `tree_migrate::plan_current` (`src/tree_lifecycle.rs:150`, `src/tree_migration_transaction.rs:145-160`) and would attempt on this live tree the very migration k6 deletes |
| k17 precedes k18 | the prompt may not name a skill that does not exist |
| k18 precedes k19, and does **not** delete `src/methodology.rs` | `provision::reverify_installed` calls `methodology::identity()` on every iteration (`src/provision.rs:53-77`, `src/loop_driver.rs:116-128`), so the module cannot go until provisioning does; the spec assigns both delivery retirements to *the leaf that deletes provisioning* (`docs/specs/module-decomposition.md:776-777`) and no ADR is rewritten ahead of the code |
| k16 and k17 precede k19 | deleting provisioning before the plugin is installed leaves the next session with no methodology, and the failure is silent |
| k7 precedes k8 and k9 | the seam cannot claim *fully domain-free*, or state its own precondition as a refusal, while a git lane sits behind it |
| k10 and k11 precede k22; k9 precedes k22 | the driver consumes all three |

Three starting points depend on nothing and could run in any order: the runner
(k10), the VCS seam once the git lane is dropped (k9), and the store's new
operations (the k12 node).

### Every leaf lands green

There is no stage in this run that cannot land with the suite passing. That is
not an accident of the work: expand → migrate → contract is applied **per crate
boundary** rather than per symbol, and a crate is small enough that all three
stages fit one session — except where the leaf bodies say otherwise (k10/k11 and
k21/k22 split one crate in two, k16/k17/k19 are the plugin's expand and contract).

## Standing notes for every leaf below

**This repo is a meta-grove.** A session here runs against the **installed**
binaries, which on this machine are Homebrew's at `/opt/homebrew/bin/`. So a leaf
that makes a **tree-visible** change the installed build cannot read must see the
new build deployed before it makes that change.

**Two corrections, both from `delete-migration-k6`, which was the first leaf to
attempt this and found the protocol wrong.**

*First: deployment is a release, never a local overwrite.* This section used to
say *overwrite the two Cellar files, or re-point the two `/opt/homebrew/bin`
symlinks*. Do neither. Overwriting `/opt/homebrew/Cellar/grove/<version>/bin/`
makes `grove --version` lie and desyncs Homebrew's manifest from the bytes on
disk. The route is **cut a minor release, publish, update through Homebrew** —
and before publishing, check the new build against every other live grove on the
machine, because they all resolve the same binaries. There were three during k6
(`gh-issue-12`, `code-walkthrough-for-ordinal-fs-tree`, and the default
workspace), each with a driver that re-provisions **its own** embedded methodology
every iteration; a swap under a live loop is undone by the old process and fought
over by every other one.

*Second: most of the five leaves may not need deployment at all.* The list was
`delete-migration-k6`, `grammar-separator-k15`, `prompt-names-the-kind-k18`,
`delete-provisioning-k19` and `open-kind-k20`. **k6 is off it**: the tree-visible
half of that leaf was deleting `.grove/FORMAT`, and a stray `FORMAT` is a foreign
entry every reader ignores, so *not deleting it* avoided the whole problem at no
cost (`08-impl-delete-migration-k6.md`, `## Why this leaf does not install
anything`, carries the measured four-cell matrix).

So each remaining leaf must **re-derive** whether it is a cutover leaf rather than
inherit the label, and the test is the matrix k6 ran: is there a cell where the
*installed* build meets the tree this leaf leaves and fails? If the tree-visible
change can simply be deferred, defer it and do not deploy.

`grammar-separator-k15` is the one that cannot defer — the rename onto the new
grammar **is** its deliverable, and 19.3.0 cannot parse the result. It therefore
owns the release, and `.grove/FORMAT`'s deletion from this tree rides with it:
one published release, one tree-visible cutover, both together.

### The cutover sequence

**There is no build-pairing guard, and no leaf may rely on one.**
`report_build_pairing` returns `()` and prints diagnostics
(`src/loop_driver.rs:550-576`); `docs/USAGE.md:164-177` says so explicitly —
*"it reports rather than refuses"*. It also runs **after**
`provision::reverify_installed` (`src/loop_driver.rs:116-128`), which restores the
*running* build's embedded methodology over anything a session just installed. So
a reinstall under a live loop halts nothing and can be undone by the old process
between iterations. `module-split-k4` planned three cutovers on the opposite
premise; this section replaces it.

**The stop that does exist is the session's own.** The driver breaks its loop when
a session ends **without** a completion signal (`src/loop_driver.rs:49`, and
`LoopOutcome`'s non-signalled arm). That is mechanical, observable in the driver's
own stderr line — *"session ended without a completion signal … loop stopped"* —
and entirely under the session's control. It is the handoff boundary.

A leaf that has re-derived that it genuinely is a cutover leaf runs these steps,
in this order:

1. Land the source change with the tree still in the shape the **old installed**
   binaries accept. `cargo test` and `cargo clippy --all-targets` clean.
2. **Cut and publish a minor release, and update through Homebrew** — not a local
   overwrite. Before publishing, run the new build read-only against every other
   live grove's tree on the machine and confirm it picks what their drivers are
   already on; a release strands them otherwise. `command -v grove-llm` resolves
   `/opt/homebrew/bin/grove-llm` ahead of `~/.cargo/bin`, so `cargo install
   --path .` installs a build no session reaches and is not a route either.
3. **Prove the installed build.** A published release moves the version, so
   `grove --version` is a real witness again — it was not while this section
   still said *overwrite the Cellar files*, which is why it used to forbid the
   check. Take it **and** a behavioural probe, because a version is a claim about
   provenance and not about behaviour: one probe the *old* build fails and the new
   one passes, named by the leaf itself. `readlink -f "$(command -v grove-llm)"`
   still confirms which file `PATH` resolves.
4. Make the tree-visible cutover (delete `FORMAT`, rename onto the grammar,
   install the plugin) **after** step 3, using the newly installed `grove-llm`,
   never the old one.
5. Retire and commit with the new `grove-llm`, then **end the session without
   running `grove-llm complete`.** That stops the loop, so the old driver process
   cannot run another iteration against a tree or a delivery it no longer
   understands. Say so in the commit message: the human restarts `grove`, and the
   new build drives from there. This is the only sanctioned non-signalling exit in
   the tree, and it is a deliverable of a cutover leaf rather than a fault.

**Why a cutover needs step 5, not just the tree-shape ones.** The running
driver is the old build in memory: it composes the old prompt, classifies the tree
with old code, holds the old kind set, and re-provisions its own embedded
methodology over the plugin every iteration. Nothing a session installs reaches
it. Restarting is the only acquisition.

**No ADR is rewritten ahead of the code that makes it true.** `docs/adr/` describes
the design's current state; the leaf that lands a change reworks its record **in
place, in the same commit**. The spec's `## ADR reconciliation` table names who
owns each one, and `spec-to-current-state-k23` walks the whole table as a
checklist at the end.

## Principles

These are settled, and a session that finds itself arguing against one should
raise it rather than quietly design around it.

1. **The version control system owns safety, history and transactionality.**
   It is not this project's concern. Where grove currently hand-builds a
   durable record of a pre-operation state, a proven rollback, or a crash-atomic
   multi-path mutation, the answer is the VCS's own — jj snapshots the working
   copy before every command, and its operation log is the transaction record.
   Grove takes commits; it does not implement transactions.

2. **Anomalies stop with a good message; they are not repaired in code.**
   A misnamed file, a torn tree, an unrecognised state: name what is wrong, name
   how to fix it, and stop. Recovery machinery is not written where a sentence
   and a human will do. An error that only reports detection is unfinished — the
   advice is part of the error.

3. **One type owns a name, end to end.** A name's parsing and unparsing live in
   exactly one type, and nothing else spells or peels one. This holds for the
   *handle* as well as the filename: the handle is a projection of the name and
   belongs to the same owner.

4. **The skills drive; grove is ambient.** Anything a session can do, a session
   does — through verbs it invokes. Grove keeps only what a session physically
   cannot do for itself: relaunch itself with fresh context, kill itself under a
   sandbox, choose its own vendor before it exists, and tell itself to load the
   methodology.

5. **Entries are marked, never removed.** Inherited from the store, whose key
   allocation derives from the names on disk. Deleting the *root* is a different
   operation from removing an *entry*; only the first is on the table.

## Pointers

- `docs/adr/task-tree-transactions-fail-closed.md` — the witness protocol
  principle 1 supersedes. **Retired at `delete-finish-transaction-k8`**, along
  with `supported-workspace-layouts`, `success-is-proved-by-the-ticket-not-the-tree`
  and `finish-keeps-a-cleanup-layer-it-has-not-proved-forced`, whose subjects all
  went with the transaction.
- `docs/adr/complete-session-configuration.md` — the completeness rule that a
  task-type-as-label change reopens.
- `docs/adr/entries-are-never-removed.md` — principle 5, argued from key
  allocation.
- `CONTEXT-MAP.md` — the vocabulary-boundary discipline the first extraction
  used, and the model for the next ones.

## Notes

Migration is **out of scope and to be deleted**, not preserved: no legacy tree
needs it.
