# minimalism-k1

## Goal

Settle the requirements for the five-module decomposition in the brief: what each
module's interface is, what moves, what is deleted, and what the open questions
cost. The measurements below were taken on 2026-08-28 against `main` at v19.3.0
and should not be re-gathered.

## Context

### Sizes today (non-test lines, `src/` plus the library)

| group | lines |
|---|---|
| tree data + lifecycle verbs (`task_tree`, `task_grow`, `tree_lifecycle`, `tree_access`, `tree_format`, `task_name`, `leaf`) | 7,337 |
| finish teardown (`finish_transaction`, `finish_cleanup/*`, `repo/finish_commit`) | 10,366 |
| migration (`tree_migrate`, `tree_migration_transaction`, `repo/migration_commit`) | 3,373 |
| driver runtime (`driver_lease`, `complete`, `provision`, `session_config`) | 2,986 |
| loop + CLI surfaces | 2,668 |
| VCS resolution (`repo`) | 535 |
| `ordinal-fs-tree` library | 5,586 |

### The store already satisfies the name rule; grove does not

`EntryName` (`crates/ordinal-fs-tree/src/name.rs:389`) is the sole seam:
`parse(name, found)`, `compose(ordinal, key, parts)`, `Display`, `view()`,
`distinguished()`. The library never learns the name's *layout* — the only line
inspecting a rendered name is `not_one_component` (`name.rs:686`), which checks
one path component. It states the round-trip law: `format(parse(f)) == f`, and
`conformance::check` holds a consumer to it.

Grove honours this for filenames and **not** for handles. The handle grammar
`<slug>-k<key>` has six implementations, none behind a type:

- produce: `task_tree.rs:513`, `tree_lifecycle.rs:220`, `finish_cleanup.rs:121`,
  `task_grow.rs:475`, `tree_lifecycle.rs:1174`
- parse: `task_tree.rs:952` (`handle_key`), whose own comment concedes it
  *"mirrors the filename grammar"* — the same peel as `task_name.rs:609`
  (`split_shape`), written twice

The handle is the value that crosses every module boundary: the store produces
it, the loop puts it in the prompt, the skills pass it back to verbs.

### What the store is missing

Present: `fs::read` / `fs::write` guards, `append`, `append_many`, `insert`,
`promote`, `rewrite`, `Snapshot`/`Walk`/`Entry`, `conformance`.
Absent: `exists?`, `initialize`, `delete`, and a *search-found-nothing* answer.
`Refusal`'s twelve variants are all refusals to **mutate**; the no-work signal is
grove's own `Option<SelectedLeaf>` (`task_tree.rs:584`), whose predicate is grove
vocabulary and cannot move as-is.

### Adding `exists?` collapses grove's second lock layer

`tree_lifecycle.rs:127` records why grove holds its own `flock` on top of the
library's: *"Classification needs grove's guard, since the tree it classifies may
be absent, legacy or mid transaction and the library can read none of those"* —
and the two deadlock, *"two open file descriptions on one directory do not share
a lock"*. All three reasons are removed by `exists?`, dropping migration, and
`initialize`/`delete` owning their own state. `tree_access.rs` (315), the
two-phase `Classification`/`settle` dance, and `write_scaffold` go with them.

### The VCS already does the finish transaction (measured)

Spike on jj 0.44.0, colocated:

- `rm -rf .grove/` with **no jj command run**, then `jj restore .grove` — all
  five files returned.
- **Partial** deletion (2 of 5 files), then `jj undo` — *"Added 2 files"*,
  exactly the missing ones.
- `jj op log` carries `snapshot working copy` / `restore into commit` / `undo`.

That is what `finish_transaction.rs` hand-builds: a durable pre-operation record
and a proven rollback. The colocated git-index backup
(`finish_cleanup/auxiliary*`, 2,217 lines) exists only because grove drives
*git* plumbing on a colocated repo; jj has no index.

Roughly 25 auto-repair functions become messages under principle 2:
`recover_pending`, `rollback`, `recover_finish`, `recover_plain_git_finish`,
`recover_jj_finish`, `recover_jj_index_backup`, `recover_auxiliary`,
`recover_partial_root_init_unlocked`, `reap_orphaned`, `restore_git_index`, and
the rest.

`TaskNameError` (`task_name.rs:314`) is already the shape principle 2 wants —
every variant carries what is on disk *and* what it should be.

### The runner is nearly domain-free already

`session_config.rs` (633) contains 81 grove-vocabulary occurrences: **57 are
`kind`** — the lookup key, which a generic runner needs as an opaque string —
and the rest is 2 `leaf` and 2 `grove`. Everything else is KDL parse, validation
with source locations, shell-word splitting without a shell, whole-word
substitution, diagnostics.

`driver_lease.rs` (1,737) splits three ways: signal channel + epoch + nonce
(~400) to the runner; the one-driver-per-worktree lease (~900) to the loop; and
`ensure_supported_workspace_layout` + `control_directory_device` (~200, plus
`repo::measured_device`'s 8 call sites) **die**, because they exist only to
guarantee the quarantine rename is same-device.

`repo::`'s whole external surface is 12 functions, splitting cleanly:
workspace resolution (`workspace_control` 11, `measured_device` 8,
`main_repo_of` 6, `vcs_of` 4, `toplevel` 2, `path_is_tracked` 2) to the loop;
the commit boundary (`prepare_finish` 14, `recover_finish` 4,
`abort_preparing_finish` 2, `verify_lost_finish_result` 2, `git_index_path` 1)
to the VCS seam.

### What the loop physically cannot delegate

Four things, each for a different reason:

1. **relaunch** — a session cannot restart itself with fresh context
   (`complete.rs`: *"the 'external exit' an interactive `claude` cannot perform
   on itself"*)
2. **kill** — codex's Seatbelt denies a same-sandbox process signalling its own
   session
3. **kind before launch** — the kind selects the **vendor**. The live config maps
   19 kinds to five templates across two vendors, with reviews deliberately on a
   different vendor from the work they read. A session cannot re-route itself
   after starting.
4. **the bootstrap prompt** — a skill cannot tell you to load it
   (`prompt.rs`, the too-late test)

Everything else is already skill-driven: every tree mutation is a verb a session
invokes, and **`finish-commit` — the whole teardown — is already invoked by the
finish session's skill**, not by the driver.

### Task type as label

The driver interprets a kind in exactly one place: `prompt.rs:reference_file`, an
exhaustive `match` mapping 19 kinds to 10 files — which `prompt.rs` notes is
*"the same grouping the skill's own routing table prints"*. Move it into the
skill and nothing in the driver interprets a kind; it becomes a config key.
Cost: a typo'd kind fails at launch rather than at parse. Reopens
`complete-session-configuration` (whose completeness rule is "all nineteen") and
the closed-set argument in `leaf.rs`.

### Skill delivery

Two mechanisms already live in this repo: the methodology is embedded
(`include_dir!`) and extracted per installed harness against a content-hash
stamp, staged and atomically renamed; the linkuistics skills are a git checkout
plus a symlink farm, where **the skill declares its own harness eligibility**
(`harnesses:` frontmatter) rather than a registry deciding for it.

Comparable tooling: husky/pre-commit writing into `.git/hooks/` is the same
shape done less carefully; Nix/Home Manager makes conflict detection the
product; MCP `instructions` is the zero-delivery option, binding at connect time
with no filesystem at all. Grove **already built and deleted** a serve-on-demand
verb (`methodology`, served the embed by unit id) — it went with the mandate
machinery, not because the delivery model failed. Serving `reference(kind)` on
demand would delete `provision.rs` (485), the harness registry, the stamp, the
staging dance, `one-build-owns-a-session`, and the shared-namespace question, at
the cost of requiring MCP.

## Done when

- Each module's interface is written down as a contract, not a file list.
- Every open question below is either answered or cut with a reason.
- The deletion list is explicit and checked against callers, not assumed.

## Notes

**Open questions.**

- Does `ordinal-fs-tree` leave this repository? `CONTEXT-MAP.md` says its docs
  move *"only if it is extracted to a repository of its own"*.
- Where do the `grove-llm` verbs live? If they ship with the store, the loop
  never touches the tree at all.
- Does `Kind` become an opaque label, and is a launch-time failure acceptable in
  exchange?
- Where does `provision.rs` go — the skills module, or an MCP surface that
  removes it?

**Blocked, needs the human.** Extending the harness registry to a DeepSeek
harness needs two facts that cannot be guessed, because
`harness.rs` gates on marker presence and *"an absent root is skipped, never
created"* — a wrong guess fails silently:

- its home-relative install-marker directory (`project_dir`)
- its home-relative global skills directory (`skills_dir`)

Separately, *"a TUI plugin we could invoke"* does not fit the registry as it
stands — `harness.rs` is explicit that *"a row is a place to write files, never a
thing to run."* Invoking a harness plugin is a `config.kdl` template today, so if
more is meant it is a new capability, not a new row.

## Decisions (running log)

**The git lane is dropped; grove is jj-only.** Principle 1's warrant — the VCS
snapshots before every command and its operation log is the transaction record —
is true of jj and false of plain git, and `repo.rs:29` carries both lanes
(`Vcs::Git` is "a plain git working tree, not jj-enabled"). Narrowing the
principle to "where the VCS can" would have kept the finish transaction alive on
one lane and left the VCS seam the largest of the five modules. Instead grove
refuses a non-jj working tree with a message naming `jj git init --colocate`
(principle 2: the advice is part of the error). This is what makes the whole
10,366-line finish group deletable rather than lane-conditional, and it takes the
2,217-line colocated-index backup with it — that code exists only because grove
drives *git* plumbing on a colocated repo, and jj has no index. `vcs_of` becomes
a precondition gate rather than a dispatch.

**`ordinal-fs-tree` stays, and every module joins it under `crates/`.** One
workspace, one release process, for now. This answers more than the question
asked: a *module* in the brief's sense is a **workspace crate**, so the
done-when "testable through its own interface without the other four" stops
being a discipline held by review and becomes a fact the compiler enforces — a
crate can reach only what its siblings export. The root package stops owning
`src/`; the two binaries become thin crates over the five.

Two consequences follow and are settled with it. `ordinal-fs-tree`'s
`[package.metadata.release] release = false` exclusion is **removed**: its
manifest note (`crates/ordinal-fs-tree/Cargo.toml:104`) says the exclusion exists
to stop "is this crate ever published on its own" being answered *by accident* at
grove's next cut, and a single release process answers it deliberately — one
version, one tag, one changelog, extraction to a separate repository deferred
with the docs left at `docs/ordinal-fs-tree/`. And the vocabulary boundary
`CONTEXT-MAP.md` records as "held by hand" becomes partly mechanical for the
store, though only partly: crate boundaries stop a *name* leaking, not a
*meaning*, so the collision table still binds.

**The `grove-llm` verbs live in the loop crate; `grove-llm` is a thin bin over
it.** The open question's framing — "if they ship with the store, the loop never
touches the tree at all" — is not available. Eleven of the thirteen verbs touch
the tree and every one is stated in *grove's* vocabulary: `brief-chain` walks
`BRIEF.md` charters, `kind` reads one of nineteen, `leaf-add-pair` mints a
research pair, `leaf-retire`/`leaf-prune` write `DONE`/`ABANDONED`, `pick` orders
finish-last. `CONTEXT-MAP.md`'s collision table shows the library has no word for
*outcome*, *kind*, *brief*, *handle* or *finish*, so shipping them with the store
would import grove's vocabulary into the library — the one thing that boundary
exists to prevent. The single exception proves it: `leaf-decompose` *is* the
library's `promote` under a grove name, and it is the only verb that would
survive translation.

The brief already assigns the vocabulary to the loop, and the verbs are that
vocabulary applied to the store, so co-locating gives the handle grammar one
owner (principle 3) and the nineteen kinds one definition, reachable by driver
and verbs alike. The two non-tree verbs reach outward: `finish-commit` to the
VCS seam, `complete` to the runner.

**The driver keeps its two tree mutations, and the loop's contract states the
rule that licenses them.** After migration is deleted, `loop_driver.rs:130-133`
leaves exactly three tree touches: one read (`select`) and two writes — root
scaffolding when bare `grove` meets a rootless working tree, and
`materialize_finish` when `select` returns nothing. The loop's interface is
therefore *reads the tree once, and mutates it only where no session exists to
delegate to*, which turns principle 4 from a judgement into something checkable:
one write happens before the first session, the other between the last one and
the finish session. The `root-init` verb already exists, so the capability is
delegated and only the rootless-bootstrap path is not. The finish sentinel earns
its place separately by being robust to session death — a session that retires
the last leaf and then dies still meets a finish leaf on the next pick, which a
session-side mint would not survive.

**The methodology owns the definition of the kinds, including conformance;
grove reads it and interprets nothing.** `leaf::Kind`'s closed enum dies. In its
place the methodology ships a **kind manifest** — one declaration per kind
carrying the properties grove needs to act ambiently — and every site that today
matches on a kind becomes a lookup in it.

The task file undercounted those sites: there are five, not one, and the manifest
answers all five.

| site | today | under the manifest |
|---|---|---|
| `prompt.rs:reference_file` | exhaustive `match`, 19 → 10 files | the kind's declared reference path |
| `prompt.rs:ending_file` | exhaustive `match`, 19 → 2 signal files | the kind's declared ending |
| `task_tree.rs:593,607` | `Kind::Finish` sorts last in `pick` | a declared ordering property |
| `tree_lifecycle.rs` | `Finish` reserved from the grow verbs; `Requirements` is root-init's default | declared reservation and bootstrap-default properties |
| `session_config.rs:16` | completeness derived from `Kind::ALL` | completeness against the manifest's set |

That last row is the one the task file expected to lose, and it survives.
`complete-session-configuration`'s load-bearing claim — the personal config
declares every kind exactly once, validated in full before every tree mutation
and every launch, which is what makes a partial second source safe — is
preserved with its quantifier restated from *all nineteen* to *every kind the
methodology declares*. The ADR is amended rather than reopened.

**The driver still names the reference file by path in `${prompt}`, and this is
not negotiable.** `prompt.rs` records the measurement: naming the skill *and*
the kind's reference file scored 10/10 against a control that pointed only at
`SKILL.md` and scored 0/10, where sessions "reached its kind's procedure only
after starting work, in every session of both arms." Grove consults the manifest
to fill that slot; consulting a definition is not owning one.

**Cost accepted:** an unknown kind is caught against the manifest rather than by
the Rust type. That is not the launch-time failure the task file priced, because
the config is validated before every tree mutation as well as before every
launch — a typo'd kind fails at `leaf-add`, one step later than a `match` arm and
well before a session exists.

**The provisioning sweep is retired; the methodology ships as a plugin, and the
compatibility check inverts.** Grove stops writing skill directories. The
methodology is installed the way `linkuistics` and `testanyware` already are —
the Claude Code marketplace (`.claude-plugin/marketplace.json` gains a third row)
and `plugins/install.sh`'s symlink farm elsewhere, where a skill declares its own
harness eligibility in `harnesses:` frontmatter rather than a registry deciding
for it.

The check that survives runs the other way. Today the binary owns the skill
directory: it stamps each one with the embed's content hash and restores its own
copy whenever another build took it (`one-build-owns-a-session`). Under this
design the **machinery publishes a version/signature and the skill checks it** —
grove states what it is, the methodology decides whether that is good enough and
what to do when it is not. The skills rule.

Deleted with the sweep: `provision.rs` (485), `harness.rs`'s registry, the
`.grove-content-hash` stamp, the staging-and-atomic-rename dance, the foreign
skill-directory warning, and the shared-namespace precedence question
`CONTEXT-MAP.md` records as open. `content/` leaves both binaries, taking
`include_dir!`, `build.rs`'s per-file `rerun-if-changed` walk, and
`methodology::identity()`'s content hash — which is replaced by the workspace's
single release version (decision 2), a value that means something to a human and
orders correctly, which a content hash never did.

Reopened and to be rewritten, not merely amended: `skill-delivers-the-methodology`
(the delivery path it names no longer exists) and `one-build-owns-a-session`
(there is no build pairing once no build writes a skill directory).

**The cost accepted is the one that record existed to prevent:** grove no longer
guarantees the methodology is present, so a session can be launched pointing at a
skill that is not installed. Under principle 2 that is a message, not machinery —
grove states the version it is and names the install command, and stops.

**`${locations}` is dropped, and the gap is recorded as untested.** The prompt's
third part today lists the provisioned skill directories by absolute path — *"what
makes the instruction actionable by plain file read, the one capability every
harness has"* — and grove cannot know those paths once it stops writing them.
Preserving it would have meant keeping a harness registry, which is most of what
retiring the sweep exists to delete, or asking the operator for a path that goes
stale wrong rather than absent.

The evidence does **not** contradict the drop: `wording-micro-test.md`'s ablation
removed only the two *unestablished* elements, so the absolute paths were never
individually tested — they are established *together with* the imperative and the
ordering clause, not apart from them. Recorded the way that document states its
own gaps: a harness with a skill-loading affordance is unaffected, one without
loses its fallback, and the reopen condition is a session that cannot reach the
methodology by the affordance alone.

**The kind is a skill name, not a file path.** `${prompt}` becomes *"Load the
`grove-<kind>` skill"* plus the handle and the stated VCS. Grove substitutes a
string it does not interpret — the same whole-word substitution the runner
already performs on command templates — and routing moves onto the harness's own
native skill mechanism instead of grove reimplementing it in prose.

This reproduces the element `wording-micro-test.md` measured as load-bearing —
*one imperative naming the target, so the session performs no selection and has
nothing to defer* — while removing every reason grove had to know anything about
a kind. No manifest, no methodology location, no reference-path convention, no
`match`, no registry. `prompt.rs` (324) collapses to a format string.

It also settles what a **kind** is, with no machinery: **a kind exists iff a
skill of that name exists.** The closed set of nineteen leaves the binary and
becomes a property of what is installed. A typo'd kind is a skill that fails to
load, reported by the session that met it — principle 2 at the only layer that
can see it. The methodology plugin ships nineteen skills over a shared `grove`
spine (the seven constraints, `TASK-FORMAT`, `ADR-FORMAT`, `SPEC-FORMAT`,
`CONTEXT-FORMAT`, `grilling`), which each of the nineteen directs its session to
load — a *directed* second load, never a selected one.

**This cuts one of the two items the task file listed as blocked on the human.**
Extending the harness registry to a DeepSeek harness needed `project_dir` and
`skills_dir` because `harness.rs` gates on marker presence and a wrong guess
fails silently. There is no registry left to extend: grove writes no skill
directory, so no row exists and the two facts are not needed. The question is
answered by deletion. The second item stands unchanged and was never a registry
question anyway — *"a TUI plugin we could invoke"* is a `config.kdl` command
template, and if more is meant it is a new runner capability.

**The prompt keeps a session-ending part, and grove authors it.** Not the
methodology's signal file — that leaves with `content/` — but one driver-authored
sentence stating grove's own mechanism: *this session ends by running `grove-llm
complete` as its last action; your skill says when, and with which flag.*

The line between the two halves is what licenses it. `grove-llm complete` writes
`GROVE_SIGNAL_FILE`, the driver watches that path, and the kill escalation
follows — all of it machinery, and the loop's own contract with the session it
spawned. *When* to signal, and whether a `finish` session passes `--done`,
withholds the signal, or signals bare, is methodology and stays in
`grove-finish`. So the prompt covers the loop's highest-consequence failure —
`CONTEXT.md` records that the cancelled mandate experiment degraded most visibly
into "sessions that finish their work and fail to signal", and a session that
ends without signalling stops the loop rather than advancing it — while grove
still makes no methodology claim and needs no per-kind branch.

`prompt.rs`'s dependency on `crate::methodology` goes with this: three parts, all
driver-authored, none of them embedded content.

**Test seams, agreed.** Four, replacing a suite that today has effectively one:
41 integration files and 27,719 lines almost all driving the binaries, which is
why they are that size.

1. **Each crate's public interface** — the primary seam, one per module,
   exercised without the other four. This is the brief's done-when made
   mechanical, and the pattern already exists at `crates/ordinal-fs-tree/tests/`.
2. **One composed-loop seam** — the loop driving a fake harness binary end to
   end. Today's `loop_driver.rs`, `complete.rs` and `driver_lease.rs`, much
   shrunk.
3. **Conformance kits as the cross-crate seam** — the store already ships
   `conformance::check` to hold a consumer to the round-trip law; the runner
   ships the equivalent for a config. This is what keeps "reusable outside grove"
   true without a second repository, and it is why extraction (decision 2) can
   stay deferred without weakening the claim.
4. **The methodology's delivery assertion, in the plugin.**
   `behavioural-coverage-asserts-delivery`'s rule survives and its **instrument
   moves**. The walk it specifies covers "`src/prompt.rs`'s guaranteed core,
   `content/SKILL.md`, `reference_file(kind)`, and the transitive closure of the
   corpus files those name by path" — two of those four no longer exist in the
   binary, so the assertion cannot run in `cargo test` at all. The plugin ships
   its own check that every behavioural rule is present and reachable on the
   composed loaded path of every kind it binds, which is now `grove-<kind>` plus
   the shared spine. Roughly 6,300 lines leave the Rust suite with it
   (`loaded_path_budgets` 1,841, `session_kind_guidance` 1,076, `methodology`
   1,254, `composition_guidance` 798, `reference_navigation` 676,
   `rule_ownership` 656). Cost accepted: the plugin needs a test runner it does
   not have today.

## Module contracts

Five crates in one workspace (decision 2). Each is stated as what it owes and
what it may not know — not as a file list, which goes stale against the thing it
describes.

### 1. Tree store — `ordinal-fs-tree`

**Owes.** The on-filesystem ordered tree: read, mutate (`append`, `append_many`,
`insert`, `promote`, `rewrite`), and the four operations it lacks today —
`exists?`, `initialize`, `delete`, and a no-outcome answer to a search. Snapshot,
Walk, Entry. A conformance kit that holds a consumer to its laws.

**Guarantees.** Names round-trip: `format(parse(f)) == f`. A fresh key is
`max(key) + 1` over the whole tree, so the names on disk *are* the counter, and a
key is never reused. Entries are marked, never removed. Every refusal names what
is wrong *and* how to fix it. **It holds the only lock over the tree** — the
second layer grove keeps today exists because the library cannot answer "is there
a tree here?", and `exists?` removes all three of the reasons
`tree_lifecycle.rs:127` records for it, deadlock included.

**May not know.** *Kind*, *outcome*, *brief*, *handle*, *finish*, *session*. Its
vocabulary is entries, ordinals, keys and the distinguished child, and
`CONTEXT-MAP.md`'s collision table stays the arbiter.

**Tested through.** Its own public API, plus `conformance::check` — with
`reference.rs`'s domain as the second consumer that makes domain-freeness a
demonstrated property rather than a claim.

### 2. Runner

**Owes.** A config of `key → complete command template`; whole-word expansion;
direct argv spawn; session supervision; the out-of-band completion signal; kill
escalation from grace through SIGTERM and kill-grace to SIGKILL.

**Guarantees.** A key resolves to exactly one **complete** template, read whole
out of one file — nothing is merged within a key, so no rule decides which words
of a launch come from where. No shell: the template is split into words without
one, and the argv is spawned directly. The child's environment is the caller's,
minus the scrubbed control values, plus a fresh signal path.

**May not know.** What a key *means*. `kind` enters as an opaque string and
leaves as an opaque string; the 57 occurrences in `session_config.rs` today are
already this shape.

**Tested through.** Its API plus a config conformance kit, driven against a fake
child binary.

### 3. Loop

**Owes.** The composition — `exists? → create or find next → determine the
command → run → finalise` — grove's vocabulary (kind, handle, outcome, the brief
chain, prompt composition), the one-driver-per-workspace lease, and the thirteen
`grove-llm` verbs (decision 3).

**Guarantees.** It **reads** the tree once per iteration and **mutates** it only
where no session exists to delegate to: root scaffolding before the first
session, the finish sentinel between the last one and the finish session
(decision 4). The handle's grammar has exactly one owner, and nothing else spells
or peels one — six hand-rolled implementations today, none behind a type.
`${prompt}` is three driver-authored parts and carries no methodology: load
`grove-<kind>`, the handle and stated VCS, and grove's own signalling contract.

**May not know.** What a kind means. It substitutes the parsed string into a
skill name and a config key and interprets neither.

**Tested through.** Its crate API, plus one composed-loop seam over a fake
harness binary.

### 4. Skills — the methodology

**Owes.** The methodology, the routing from task type to procedure, **the
definition of the kinds**, and conformance over itself.

**Guarantees.** A kind exists iff a skill of that name exists. Every behavioural
rule is present and reachable on the composed loaded path of every kind it binds.
It reads the machinery's published version and decides what to do when it does
not match — the check runs skill-side, and the skills rule.

**Ships as.** A plugin: the Claude Code marketplace plus `plugins/install.sh`'s
symlink farm, one `grove-<kind>` skill per kind over a shared `grove` spine, each
declaring its own harness eligibility in `harnesses:` frontmatter.

**Tested through.** Its own conformance runner, not `cargo test`.

### 5. VCS seam

**Owes.** Taking a commit, and the guidance printed when grove declines to
proceed.

**Guarantees.** jj only (decision 1): a non-jj working tree is refused before any
mutation, with the command that fixes it. Grove **takes** commits and implements
no transaction — no witness protocol, no manifest, no rollback proof, no index
backup. Every refusal carries its remedy, because an error that only reports
detection is unfinished.

**Tested through.** Its crate API against a temporary jj repository.

## Deletion list

Sizes are the task file's own measured non-test groups. Caller counts were taken
against `src/` on 2026-08-28 and exclude a module's own subtree.

### Contained — no caller survives the deletion

| deleted | lines | callers outside the deleted set |
|---|---|---|
| `finish_transaction`, `finish_cleanup/*`, `repo/finish_commit` | 10,366 | **none** — 5 `finish_transaction::` sites are in `tree_lifecycle` and `finish_cleanup/reaper`; all 53 `finish_cleanup::` sites are inside the finish family; `repo/finish_commit` has 1 caller, `repo.rs` |
| `tree_migrate`, `tree_migration_transaction`, `repo/migration_commit` | 3,373 | **none** — 4 + 4 sites, all in `tree_lifecycle` and each other; `migration_commit` has 1 caller, `repo.rs` |
| `harness.rs` | 48 | **none** — 6 sites, all in `provision.rs`, which dies with it |
| `driver_lease`'s `ensure_supported_workspace_layout` + `control_directory_device`, and `repo::measured_device` | ~200 | they exist only to guarantee the quarantine rename is same-device, and the quarantine goes with the finish transaction |

Roughly 25 auto-repair functions go with these under principle 2 —
`recover_pending`, `rollback`, `recover_finish`, `recover_plain_git_finish`,
`recover_jj_finish`, `recover_jj_index_backup`, `recover_auxiliary`,
`recover_partial_root_init_unlocked`, `reap_orphaned`, `restore_git_index` and
the rest — each becoming a message that names what is wrong and how to fix it.
`TaskNameError` (`task_name.rs:314`) is already that shape and is the model.

### Reconciled — every surviving call site named

| deleted | lines | surviving call sites, and what replaces each |
|---|---|---|
| `provision.rs` | 485 | 5. `launch.rs:14` `provision_installed` → gone. `loop_driver.rs:125-126` `reverify_installed` / `report_absent_skill_destination` → gone. `llm_cli.rs:453` `warn_on_foreign_skill_dirs` → gone (no foreign directories once grove writes none). `loop_driver.rs:238` `installed_skill_dirs` → gone with `${locations}`. |
| `methodology.rs` | 160 | 3. `prompt.rs:225` `embed()` → grove's own signalling sentence. `llm_cli.rs:442` `identity()` (`--content-hash`) → the workspace release version. `loop_driver.rs:551` `identity()` (build pairing) → gone; no build writes a skill directory, so there is no pairing. |
| `tree_access.rs` | 315 | 37 total, of which 6 are in modules already dying (`tree_migration_transaction` 4, `repo/migration_commit` 2) and 24 are in `tree_lifecycle`, itself heavily reduced. **7 sites in surviving code need rework**: `llm_cli` 4, `task_name` 2, `task_tree` 1. The two-phase `Classification`/`settle` dance and `write_scaffold` go with it. |
| `prompt.rs`'s content dependency | ~270 of 324 | `reference_file`'s 19→10 `match` and `ending_file`'s 19→2 `match` both go; three driver-authored parts remain. |
| `repo.rs`'s git lane | part of 535 | `git_index_path`, the empty-internal-hooks-path rule, and every plain-git branch; `vcs_of` survives as a precondition gate rather than a dispatch. |

### Total

About **15,200 non-test lines**, against grove's own non-test, non-library
source of roughly 27,300 — **~56%** — and that is the floor, because
`tree_lifecycle`'s and `repo.rs`'s shrinkage is real but not counted here.

On the test side, roughly 6,300 lines of corpus assertion move to the plugin
(`loaded_path_budgets` 1,841, `methodology` 1,254, `session_kind_guidance`
1,076, `composition_guidance` 798, `reference_navigation` 676, `rule_ownership`
656), and the suites for deleted machinery go outright — `finish_lifecycle`
4,144, `lifecycle_cutover` 1,946, `workspace_layout` 721, `migration_commit`
663, plus `migration_transition`, `provision` and `harness`.

### Not deleted, and worth saying so

`complete.rs`, `session_config.rs`, `driver_lease`'s lease and signal halves,
`task_tree`, `task_grow`, `task_name`, `tree_format` and the whole of
`ordinal-fs-tree` survive. `leaf.rs` (583) loses its closed `Kind` enum but not
its parsing role — the name parser still yields a kind, it simply no longer
validates it against a compiled set.

## Open questions — all four resolved

1. **Does `ordinal-fs-tree` leave this repository?** No. It stays, and every
   other module joins it under `crates/` in one workspace with one release
   process (decision 2).
2. **Where do the `grove-llm` verbs live?** The loop crate. The premise that
   they could ship with the store does not survive the vocabulary boundary
   (decision 3).
3. **Does `Kind` become an opaque label?** It becomes an opaque **skill name**,
   which is stronger: grove holds no set at all, and a kind exists iff its skill
   does (decisions 5 and 7).
4. **Where does `provision.rs` go?** Nowhere — it is deleted. The methodology
   ships as a plugin and the compatibility check inverts to run skill-side
   (decision 6). MCP was cut: it appears nowhere in this repo, it would not
   remove provisioning but change what is provisioned, and `harness.rs` is
   explicit that a row is a place to write files, never a thing to run.

**Blocked on the human:** one of the two dissolved. The DeepSeek harness row
needed `project_dir` and `skills_dir`; there is no registry left to hold a row.
The second stands and is restated: *"a TUI plugin we could invoke"* is a
`config.kdl` command template today, so if more is meant it is a **new runner
capability**, and it belongs to the runner's contract rather than to any registry.

## ADR reconciliation — deferred deliberately, and to whom

No ADR is rewritten by this leaf. `docs/adr/` is a minimum coherent set
describing the design's **current state**, and rewriting a record to describe a
design that is not built would make the set lie to a reader of the code. Each
decision above names the record it obliges; the impl leaf that lands a change
reworks its record in place, and the design leaf that writes the spec carries the
target design in the meantime.

The set that moves: `task-tree-transactions-fail-closed` (superseded outright —
its own reopen condition, *"or a durable finish receipt is introduced"*, is not
the one taken; the VCS simply owns the transaction), `skill-delivers-the-methodology`
and `one-build-owns-a-session` (retired — the delivery path and the build pairing
both cease to exist), `supported-workspace-layouts` (dies with the quarantine),
`complete-session-configuration` (amended: the completeness quantifier moves from
*all nineteen* to *every kind the methodology declares*),
`behavioural-coverage-asserts-delivery` (rule kept, instrument moves to the
plugin), `grove-does-not-stage-its-own-renames` and `bulk-marks-are-not-atomic`
(to be re-checked against a store that owns `initialize` and `delete`).
`entries-are-never-removed` and `entry-name-is-the-only-seam` are untouched and
become more load-bearing, not less.
