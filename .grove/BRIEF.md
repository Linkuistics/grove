# grove.using-codebase-memory — brief

## Goal

Ship one portable skill, `using-codebase-memory`, that lets an agent in any of
the four harnesses (Claude Code, Codex, Gemini, Pi) query the codebase knowledge
graph from the shell, and compose multi-step graph queries in bash rather than
as chained tool calls.

## Done when

- `plugins/linkuistics/skills/using-codebase-memory/SKILL.md` exists and every
  command it documents has been executed against a live indexed graph.
- The `linkuistics` plugin manifest names the skill.
- `./install.sh` places the skill in `~/.codex/skills`, `~/.gemini/skills` and
  `~/.pi/agent/skills`, verified by resolving one symlink and reading its
  frontmatter. **Done, but under an isolated `HOME` — the real `$HOME` was
  deliberately not written** (see Notes § Distribution).

## Decomposition

Requirements, design and planning were **done and committed before this grove
started** — see Pointers. `scope-k1` absorbed them, checked them independently,
and cut the leaves below. It diverged from the plan in two ways.

| Leaf | Kind | Covers |
|---|---|---|
| `skill-k2` | `impl` | the whole `SKILL.md` — plan Tasks 1 **and** 2 |
| `skill-review-k3` | `review-impl` | disprove every claim in it |
| `skill-integrate-k4` | `integrate-review-impl` | apply the findings |
| `distribution-k5` | `impl` | plan Task 3, unchanged |
| `docs-reconcile-k6` | `impl` | reconcile the spec and plan to what shipped |

## Scope — this grove also carries grove-methodology work

The leaves below are **not** about `using-codebase-memory`. They belong to the
**grove** bounded context (`CONTEXT.md`, not `plugins/CONTEXT.md`) and were raised
during this grove's sessions. `chain-as-node-k7` argued in its own Notes that it
was likely a grove of its own; the human was asked at the start of that session and
**chose to keep it here** rather than prune it and start a new grove. The
Done-when above is therefore no longer the whole of what this grove will produce.

| Leaf | Kind | Covers |
|---|---|---|
| `install-workspace-guard-k8` | `impl` | **done** — `install.sh` hijacking a skill install from a secondary workspace; found by `distribution-k5`, and the reason this grove's third done-when is only half-verified |
| `chain-as-node-k7` | `design` | **decided**: a review chain / vendor pair becomes a node directory |
| `chain-node-k9` … `-integrate-k11` | `impl` chain | implement that decision |
| `retire-confirmation-k12` | `design` | **decided**: the Retire cascade asks nothing; it checks, promotes and reports |
| `changelog-unreleased-k13` | `impl` | **done** — `## Unreleased` ratified; this grove's shipped work logged under it |
| `stale-module-headers-k14` | `impl` | five `src/` module headers still isolate themselves from a deleted v1 verb path — raised by `k11` |
| `confirmation-prose-k15` … `-k17` | `impl` chain | **done** — reconcile `content/`, `docs/` and `src/` prose to that decision |
| `changelog-release-rename-k18` | `impl` | **done** — `cargo release` now renames `## Unreleased` itself, replacing the prose note `k13` left — raised by `k13` |
| `walkthrough-harness-routing-k19` | `impl` | **done** — the walkthrough's obsolete one-stamped-harness claim, in *both* pages that carried it; raised by `k16` |
| `retire-help-node-path-k20` | `impl` | **done** — `grove retire --help`'s original-scheme node-path example replaced, and the grammar pinned by a test; raised by `k16` |
| `retire-no-launch-help-k21` | `impl` | **done** — `grove retire --no-launch` described, and its dry run made a checked claim; raised by `k20` |
| `driving-original-scheme-example-k22` | `impl` | **done** — `content/driving.md`'s worked research-leaf example cited original-scheme, keyless work items in *provisioned* content; raised by `k20` |
| `retire-harness-stamp-claim-k23` | `impl` | **done** — the doc was wrong, not the behaviour: only the verb that drives the grove writes the stamp; raised by `k21` |
| `src-position-citations-k24` | `impl` | **done** — two `src/` comments citing dead `.grove/` positions; one of them was a *false claim*, not just a stale citation; raised by `k22` |
| `nonsrc-position-citations-k25` | `impl` | the same class in `tests/` (4, including a live `D<n>`) and `codex-bridge/src/` (1, cross-repo); raised by `k24` |

**What `chain-as-node-k7` decided**, since the tree below builds on it and
`.grove/` dies at the finish cycle. A chain gets its own **node directory** —
reversing a decision the ADR, the spec and the glossary each recorded. The three
arguments that had rejected it lapsed: `leaf-add-chain` made node creation
proactive, the node is **brief-less by rule** so it buys no unearned `BRIEF.md`,
and a brief-less node's close has no close-time **work** — no `Done when` rollup to
check, no brief to promote — where a decomposition node's has both. (That third
argument was originally *"a brief-less node is never asked the cascade's
confirmation"*; `retire-confirmation-k12` has since removed that confirmation for
**every** node species, so the objection is doubly dead and the `BRIEF.md`
discriminator now selects close-time work rather than who gets asked. See the
`retire-confirmation-k12` summary below.) The
decisive new argument *for* it is that a directory makes the group **structural**
in every tree viewer (`yazi`, Finder, `ls -R`), not just in the one grove controls.
Children keep the stem (`skill-review-k4`, not `review-k4`) because `resolve`
matches bare slugs exactly and commit messages outlive `.grove/`. Existing flat
chains are **not** migrated — detecting one needs the suffix parsing the design
forbids, and a flat chain is a valid tree. Recorded in
`docs/specs/task-kind-taxonomy.md`, `docs/adr/task-tree-scheme.md`,
`docs/adr/cli-binary-split.md` and `CONTEXT.md`; nothing here is the durable
record.

**What `chain-node-k9` shipped, and the three things it found.** The two verbs
now write `NN-<stem>-chain-k<key>/` (or `-pair-`) holding their steps at `01`–`03`
— four keys, four paths on stdout, node first. The three properties the spec marks
as worth falsifying by mutation are each pinned by a test: the kind derivation, the
absence of `BRIEF.md`, and a mid-write failure leaving **no directory**. Verified
end-to-end against a real fixture that `pick`, `brief-chain`, `resolve`,
`leaf-retire`, `kind --with-harness` and the tree viewer all cope unchanged.

1. **`leaf-add` was *not* untouched, and the leaf asked for this to be surfaced.**
   Its parent guard required a `BRIEF.md` at `<parent>`, so it refused a chain
   node — breaking the one affordance the node shape exists to buy
   (`leaf-add <chain-node> <stem>-late-step`). The guard now reads the directory's
   **name**, which is what ADR *task-tree-scheme* already said node-ness was; the
   charter distinguishes the two *species*, not node from non-node. Recorded in
   that ADR's *Comparator and verbs*. The other seven verbs needed nothing.
2. **`docs/adr/task-kind-taxonomy.md` still carried the reversed reasoning** —
   `chain-as-node-k7` reconciled the spec and two other ADRs but missed this one,
   which was still arguing "a chain — deliberately not a node — already closes with
   none" and that the cascade cost is *created* by giving a chain a directory.
   Reworked in place (never appended to), so the ADR set is coherent again. Worth a
   sceptical pass in `chain-node-review-k10`: if one ADR was missed, check the rest.
3. **`CHANGELOG.md` got an `## Unreleased` heading, and that is
   `changelog-unreleased-k13`'s decision to ratify or undo.** This change had to be
   logged somewhere and `## v16.2.0` is closed history — editing it would falsify a
   tagged release. So the heading exists now with one entry under it; k13 still owns
   the question and now decides it against a live instance rather than in the
   abstract. It should add the other four rows of its table, not re-litigate this
   one entry's placement unless it is discarding the heading entirely.

**What `chain-node-integrate-k11` found on triage.** All three of `k10`'s findings
were real — each reproduced or grepped before being touched, none accepted on
assertion — and all three are applied. Two things are worth carrying forward.

1. **The High finding was a contract violation, not an arithmetic slip, and the
   fix says where.** `add_run` created the node directory and *then* derived each
   child's key as unchecked `node_key + 1 + i`, which smuggled one resolution step
   past the mutation boundary — so the only failure it could express there was a
   partial tree. Reproduced exactly as `k10` reported (a live two-step node left
   behind); now the whole four-key run is allocated before the first write, beside
   slug validation and the destination check. `next_key` became fallible and
   `next_keys` joined it, so key exhaustion is a modelled fact in one place rather
   than a panic in three. The release half was worse than the debug half: wrapping
   gives the last step `k0`, which both breaks the consecutive-keys contract and
   *lowers the visible max*, so the next `leaf-add` re-issues a live key.
2. **Grepping for the claim found three more stale surfaces than reading found**,
   and two were `grove-llm --help` — the only surface a human at a terminal reads,
   still documenting three contiguous leaves and three printed paths. `k10`'s own
   item 4 said to grep rather than trust `k9`'s file list, and then found its three
   by reading. A file list is written before the work and goes stale; the claim
   cannot, because it *is* what went stale. The spec now records that as the
   lesson, with its own normative staleness as the second half of it.

`k10`'s **rejected candidate** (concurrent composite calls racing for the same
keys) is upheld as rejected: ADR *task-tree-scheme* defines a grove tree as
single-worktree, single-writer, and the one-snapshot logic is correct under it.
The `u32` ceiling is now a *refusal* under that same assumption, not a lock.

**What `retire-confirmation-k12` decided.** The Retire cascade's per-node
confirmation is **gone, for every node species**. Its replacement is a
verify-and-report obligation the session discharges itself: check the node's brief
`Done when` against what the subtree delivered, `leaf-add` the missing work if the
check fails and the gap can be named, escalate if it cannot, promote what survives
upward, and name the closed node by its handle in the commit message. Leaf
retirement (never confirmed), pruning (still HITL) and the finish cycle's single
gate are all unchanged — and that gate is now the loop's *only* routine human one.
The generating rule is two ordered tests: does the answer change what is written,
and if so is the fact the session's to establish or the human's to decide. A node
close fails the first — a node is never marked, so the question gated an inference
and a node closed in error is reopened by one `leaf-add`. Recorded in the **new**
ADR `docs/adr/confirmation-boundary.md`, with `pruning`, `task-tree-scheme`,
`task-kind-taxonomy`, `in-session-finish-cycle`, `docs/specs/task-kind-taxonomy.md`
and `CONTEXT.md` reworked in place to cite it; nothing here is the durable record.
Two things worth carrying: the `BRIEF.md` discriminator **survives with its job
changed** (it now selects whether a closing node has close-time work, not whether
to ask), so `chain-as-node-k7` is untouched; and the confirmation was attached to a
*loop step* while the HITL/AFK mark is a property of a *kind*, which is why it
stalled AFK leaves by construction.

**What `changelog-unreleased-k13` decided, and the one entry it deliberately did
not write.** The `## Unreleased` heading is **ratified**, and the argument is not
"a release has to go somewhere" — it is that the preamble's existing rule already
presupposes the heading ("logged in the section of the grove release it *lands
before*"), so without a standing one that rule is obeyable only retroactively, by
whoever cuts the release and no longer has the context. That is exactly the hole
`k8` fell into. Recorded in `CHANGELOG.md`'s preamble, which is where the
convention lives once `.grove/` is gone.

All four rows of `k13`'s table are logged: the skill and its manifest
registration as **one** `### Added` entry (a manifest that lists no skills is not
a separate shipped change — what it needed was a *description*, which is all
marketplace discovery has), `install.sh`'s guard under `### Fixed`, and the
chain-node work already logged by `k9`.

**`retire-confirmation-k12` gets no entry yet, on purpose, and
`confirmation-prose-k15`…`-k17` owns the one it will get.** `k12` touched
`docs/adr/`, `docs/specs/` and `CONTEXT.md` and **no `content/`** — so the
methodology the binary actually carries still describes the old cascade, and
nothing a user runs has changed. The generalisation is now a preamble rule: *a
decision earns its entry when its behaviour lands, not when it is recorded.* It
also retro-explains why `chain-as-node-k7` has no entry of its own while `k9`
does. The prose chain should therefore write **one** `### Changed` entry covering
both the decision and its enactment, citing `docs/adr/confirmation-boundary.md`.

**The heading's release-time cost was real and is now paid** (`k18`). `k13` left
the rename as a *manual* step in `release.toml`'s usage comment beside the tag
push, and externalized automating it as `changelog-release-rename-k18` rather than
absorbing it: proving a replacement works needs a `cargo release` dry run, which
`release.toml`'s own preamble says the harness classifier refuses as opaque.

**What `changelog-release-rename-k18` shipped, and the two premises it
falsified.** `release.toml` now carries one `pre-release-replacements` entry:
`search = "^## Unreleased$"`, `replace = "## Unreleased\n\n## v{{version}}"`,
`exactly = 1`. One replacement does both halves — the match does not consume the
blank lines around the heading, so writing the heading back *plus* the version
heading below it leaves the accumulated entries where they are, now under the
version. cargo-release's documented `<!-- next-header -->` idiom was **not** used:
it needs two replacements and plants an anchor comment in the file, and the heading
is already its own anchor. `CHANGELOG.md`'s preamble no longer says the cut leaves
this file alone, and states the one constraint an editor is now under (never start
a prose line with a bare `## Unreleased`).

Verified by *executing* a real cut, not only configuring one — in a throwaway
two-member workspace fixture holding byte-identical copies of `release.toml` and
`CHANGELOG.md`. **That rig is the carry-forward**: it needs no `.git`, which
matters because this working tree is a jj-native secondary workspace where
`cargo release` cannot run at all. A plain `cargo release patch` there prints a
unified diff of every replacement; `--execute` leaves the real bytes
(`## Unreleased` standing and empty, `## v16.2.1` below it). The empty-section case
from this leaf's Notes holds — the anchor is the heading, never the content beneath
it — and `exactly = 1` was watched failing loudly rather than corrupting.

1. **`consolidate-commits` has nothing to do with it.** This leaf's Notes inherited
   from `release.toml` the premise that `consolidate-commits = false` "means
   replacements run per crate". Measured both ways: replacements run once per
   *released crate* **regardless**, and that setting affects only how commits are
   grouped. What actually bounds it is cargo-release's **default crate selection**
   (a workspace root that is itself a package releases that package alone; the
   member is reached only by `--workspace`/`-p`), plus `file` resolving against each
   released crate's *own* manifest dir — so the root `CHANGELOG.md` is unreachable
   from any other member and can never be renamed twice.
2. **`release.toml` credited the exclusion to a crate that does not exist.** It said
   "`harness-pane` is `release = false`". There is no `harness-pane` in this
   workspace; the one member is `codex-bridge`, and it carries `publish = false` and
   **no** `[package.metadata.release] release = false`. Both the crate and the
   opt-out were fictional, and the true reason is the invocation, not the manifest.
   Corrected in place with the mechanism named.

**Do not cite a match count, and this is the third generation of the same lesson.**
The obvious way to document `exactly = 1` is "the string occurs 3 times, only one is
a heading" — which was true when this session started and false by the time it
finished, because the changelog entry *about* the anchoring adds more occurrences.
It went 3 → 9 mid-session and the first draft of both surfaces asserted a stale
number. Same class as `k11`'s "a file list goes stale, the claim cannot" and `k17`'s
"a finding against a heading must sweep the summary layer": **a count of a string in
a file that documents that string is self-invalidating.** Both surfaces now state
the structural fact (only the heading is ever a whole line) and say explicitly not
to replace it with a count.

**One thing observed but deliberately not acted on.** `cargo release patch
--execute --no-confirm` ran **permitted** in the fixture, where `release.toml`'s
preamble says the classifier refuses `cargo release … --execute` as opaque. That is
*not* evidence against the claim — a throwaway repo in the scratchpad is a different
target from the real cut in the colocated workspace — so the claim is untouched and
no leaf was cut for it. What did change: `k18`'s own new comment was rewritten to
stop *re-asserting* it, pointing at the fixture rig instead. Whoever next runs a real
cut is the one positioned to settle it.

**A live-binary observation, not a defect.** `grove-llm leaf-add-chain` here wrote
a **flat** three-leaf chain (`k15`–`k17`), not the `-chain/` node directory
`chain-node-k9` implemented — the Homebrew binary on `PATH` is the tagged
`v16.2.0`, which predates that work, while `Cargo.toml` also reads `16.2.0`. A flat
chain is a valid tree by design (the ADR says flat chains are never migrated), so
nothing is broken and the leaves stand as cut. But the working tree and the tagged
release now claim one version with different behaviour, which is
`changelog-unreleased-k13`'s neighbourhood — worth deciding there rather than
inventing a leaf for it.

Externalized rather than absorbed: `stale-module-headers-k14`, five `src/` module
headers still declaring themselves isolated from a v1 verb path this repo deleted.
Same failure class, different generation — noticed while editing those very
headers for the chain-node claim.

**What `stale-module-headers-k14` swept, and the rule it settled.** All five
"Built **isolated**" headers plus `leaf_id`'s D9 variant now describe their module
as it is today; `leaf.rs` and `lib.rs` carried two more of the same class and were
folded in (`lib.rs` also claimed `leaf` survives *solely* as a migration input,
which is false — `Kind` is live everywhere). **`src/` module headers no longer cite
`.grove/` positions** — every `11.x`, `070/040`, `060/020` and `D<n>` is
gone. (As written this claimed *"`src/` comments no longer cite `.grove/`
positions **at all**"*; `k22` falsified that — two citations survived outside the
swept comment species and pattern list. `src-position-citations-k24` has since
fixed both, so the claim is **now true of `src/`** — but only of `src/`: `k24`
found the same class live in `tests/` and `codex-bridge/src/`, which is
`nonsrc-position-citations-k25`. Kept quoted-and-refuted rather than deleted.)
That is not a style preference: ADR *task-tree-scheme* §5 binds "commit
messages and **prose**" to `<slug>-k<key>` and forbids the position, and a source
comment is prose; the referent is deleted twice over, since `.grove/` dies at the
finish cycle. Where a citation was load-bearing it became an **ADR slug** or a
module path — `lib.rs`'s existing "(task-tree-scheme, the install-and-reflip-v2
leaf)" was already the model. `docs/adr/pruning.md` has no `D6`/`D7` sections at
all, so `tree_lifecycle`'s three citations to them were pointing at a dead brief's
running log while *looking* like ADR references. **`confirmation-prose-k17` should
not re-open these headers**; it inherits the rule rather than re-deciding it.

Two things it confirmed rather than assumed. `tree_migrate` **is** wired live
(`cli.rs:147`, `launch.rs:38`) — the header claiming it was not had outlived the
re-flip, so the sweep replaced a stale negative with a checked positive rather than
just deleting the sentence. And `leaf_id`'s only consumer is `tree_migrate`, which
touches `parse` and the `LeafId` shape alone: `filename`, `is_live_leaf`,
`sort_key` and `next_key` are exercised **only by that module's own tests**, and
`parse_position` / `validate_slug` live solely as `parse`'s helpers. Per this
leaf's own Notes that is surfaced, **not trimmed** — the header now argues the
positive case for keeping it whole (a frozen grammar whose tests pin it against
`tree_migrate`'s fixtures), so no follow-up leaf is owed.

**A tooling trap worth carrying, now recorded in both headers.** `query_graph`
reports `tree_grow` / `tree_read` / `tree_lifecycle` calling
`leaf_id::validate_slug` / `next_key` / `parse` / `sort_key`. They do not — those
modules import all four names from **`tree_id`**, which exports an identical
function set for the other grammar, and the indexer resolved the `CALLS` edges by
name. Grep is decisive here where the graph is heuristic (a Rust cross-module call
must appear textually as a path or a `use`), which is worth knowing given this
repo's session protocol reaches for the graph first. `tree_id` and `leaf_id` now
each warn about the other.

**What `confirmation-prose-k15` swept, and the two calls `k16` should attack
first.** Eleven files, zero code. `content/SKILL.md` § Retire is the substantial
rewrite — the close asks nothing for either species, and four verify-and-report
steps replace the question, teaching the *first test* (a node is never marked, so
any answer left the tree byte-identical) rather than only the outcome. The finish
cycle now states it is the loop's only routine human gate, the counterpart fact.
The `BRIEF.md` discriminator's four justifications-by-confirmation were
**rewritten, not cut** — its job is now to select whether a close has *work*, not
whether it gets *asked* (`BRIEF-FORMAT.md`, `TASK-FORMAT.md`, `driving.md`,
`docs/grove.md`, plus `llm_cli.rs` and `tree_grow.rs`). `tree_grow.rs`'s
brief-absence assertion is kept and only its justification rewritten, per the
leaf; the test was **renamed** to what it now pins
(`…_so_its_close_has_nothing_to_do`).

1. **`docs/research/` was included, annotated not rewritten — the widest call
   here.** Four places read *"task-master validates grove's human-confirmed
   roll-up"* as a live endorsement. The rule applied: a survey **finding** is dated
   evidence and immutable, while its **mapping onto our design** is normative and
   must track the decision — so only the mappings moved, framed as "at survey
   time". What survives is the *integrity* half (grove's done-ness cannot drift
   from its children), which is the very premise the ADR used to **remove** the
   gate; the survey was not wrong, its conclusion was drawn one step early. If
   `k16` judges research docs out of scope for a prose sweep, this is the finding to
   raise — the alternative was five scattered annotations or a whole-document
   status banner (the `docs-reconcile-k6` pattern).
2. **`docs/workflows/multi-step.md` was rebuilt, not edited, and grew an
   iteration.** Its scripted beat was *"the user said not yet"* — an interaction
   that can no longer happen. Now check → `leaf-add`, which shows the ADR's own
   repair (the node goes live again with nothing un-marked, which is *why* the close
   needs no gate). That pushed the node's real close into a new iteration 4, moved
   `grove retire` beside the promotion it demonstrates, and made the walkthrough
   five sessions. Worth a consistency read: the illustrative commit hashes and the
   iteration-3 log had to stay reconcilable, and one earlier paragraph ("the node is
   implicitly done") now sets up a check that *fails*.

Also checked and deliberately left alone, so `k16` need not re-derive them:
`src/cli.rs`'s `grove retire` doc comment (promote-and-close, never implied
asking), `content/prompts/retire.md` (one line, delegates to the skill's flow),
and the `leaf-prune` HITL comments in `llm_cli.rs` / `tree_lifecycle.rs` —
pruning stays HITL on test 2. `CHANGELOG.md` got the **one** `### Changed` entry
this chain owns, plus a fix to the *Unreleased* chain-node entry, which still said
the confirmation "is now asked of brief-carrying nodes only"; the identical
sentence under `## v16.2.0` is tagged history and was left. Every `grove-llm` /
`grove` `--help` long-text surface was greped clean of the old claim.

**What `confirmation-prose-review-k16` found.** Two Medium parallel-guidance
defects, both reproduced by the repository-wide grep and left for
`confirmation-prose-integrate-k17`: this brief's `chain-as-node-k7` carry-forward
still makes brief-lessness the reason a node is not asked, and
`docs/research/skill-repo-prior-art.md`'s G5 heading still says grove *asks* even
though its rewritten body says the gate was dropped. The generated chain help,
the six discriminator surfaces, the cold Retire procedure and the five-session
walkthrough otherwise agree with the decision; `cargo test` passed in full,
including the chain-node brief-absence guard.

Two unrelated stale surfaces found only by executing help and reading the whole
walkthrough were externalized, not absorbed: `walkthrough-harness-routing-k19`
owns the last paragraph's obsolete claim that one bootstrap-chosen/stamped
harness runs every task, and `retire-help-node-path-k20` owns `grove retire
--help`'s removed original-scheme example `003-session-store`.

**What `confirmation-prose-integrate-k17` applied, closing the chain.** Both
findings reproduced by grep before being touched, both real, both applied; nothing
upheld as rejected, and **no finding against the ADR**, so
`docs/adr/confirmation-boundary.md` is untouched. The carry-forward above is fixed
in place: a brief-less node's close has no close-time **work**, which is the
discriminator's job since `k12` — the falsified sentence kept *quoted-and-refuted*
rather than deleted, per `docs-reconcile-k6`'s pattern.

1. **The post-fix grep found a third instance `k16` did not**, in the file
   `k16` was already reviewing: `skill-repo-prior-art.md`'s own **Takeaways**
   roll-up still called G5 a live convergence, one structural level above the
   heading `k16` flagged. Applied in the same pass — it is the flagged finding's
   extent, not new work. The generalisable form, and the reason it recurs: `k16`
   read the *section*; a roll-up summarises sections and is not reached by
   correcting one. **When a finding is against a heading, sweep the document's
   summary layer too.**
2. **Two grep traps that manufacture a false clean**, both hit this session and
   both worth knowing given this grove re-runs claim greps *as evidence*: `rg -E`
   is `--encoding`, **not** GNU grep's extended-regex flag, so with `2>/dev/null`
   a flag error is indistinguishable from a clean sweep; and `.grove/` is a
   **dotdir** `rg` skips without `--hidden` — the live mandate sits outside the
   default search space, which is why `k16` specified a "hidden-path" grep.
   `k18`–`k20` each re-verify a claim by grep; use `--hidden`, and check the
   command ran.

`cargo test` green in full. The chain touched no code, so `confirmation-boundary`
is now fully enacted in prose and `CHANGELOG.md` carries the one `### Changed`
entry `k15` wrote for it.

**What `walkthrough-harness-routing-k19` fixed, and the second copy it found.**
`k16` flagged `docs/workflows/multi-step.md`; the repo-wide grep found the *same*
obsolete model one page over, in `start.md`, and there it was worse — stated as an
**invariant** ("a single grove never spans harnesses mid-flight") that per-kind
routing exists to break, and met *first* by anyone reading the set in order. Fixed
in the same pass as the flagged finding's extent. Both passages now name the full
`leaf → kind → family → stamp` precedence and cast the stamp as the **fallback**;
neither wanted deleting, because the error was one of **scope, not of fact** — the
stamp is still written and still read, it just stopped being the whole answer.

Checked and left alone, so `k20` need not re-derive them: `docs/grove.md` (`:64`,
`:79` describe the stamp as a binding and `:77` already documents per-kind
resolution), `docs/workflows/finish.md:137` (stamp *cleanup*, no routing claim),
`src/cli.rs`'s `--harness` help plus the `MODEL_ENV_HELP` attached to the same
commands via `after_long_help` (already carries the four-level order), and
`src/loop_driver.rs`'s doc comments (`:845`). **No `CHANGELOG.md` entry**, and
`k20` faces the same question: the behaviour shipped and was logged when it landed,
so this is stale prose corrected against it — the `stale-module-headers-k14` /
`confirmation-prose-integrate-k17` class, both verified CHANGELOG-free by
`jj diff --stat`. `cargo test`: 622 passed, 0 failed.

**What `retire-help-node-path-k20` fixed, and the property it turned out to be
pinning.** `grove retire --help`'s `<PATH>` now offers a current node path
(`04-session-store-k7`), the nested case, and the stable `<slug>-k<key>` handle —
the handle being legitimate *because* `launch::retire` substitutes the argument
into the prompt verbatim and parses nothing, so what it accepts is whatever the
**session** can resolve, and `resolve` handles node directories by key, bare slug
and full handle alike. The removed example was **unusable, not merely stale**:
measured against a planted node fixture, all three current forms resolve and
`003-session-store` matches nothing.

1. **The three-digit width was never the discriminator**, which the leaf's own
   Context implied it was. `tree_id::parse_position` is lenient on padding, so
   `003` parses as position 3 — the missing terminal `-k<digits>` is what rejects
   the old scheme. A test written against the digit width would have pinned the
   wrong property and passed on any keyless two-digit name. The test that shipped
   runs each offered name through `tree_id::parse` instead, so it pins the
   **grammar** and not the example's spelling; falsified by mutation both ways.
2. **`grove retire --help` was the only stale help surface, established by
   executing all of them** — both binaries, every subcommand, dumped and scanned.
   That is `k11`'s "grep the claim, not the file list" applied *before* a miss
   rather than after one, and the same scan is what found `k21`.
3. **A fourth-generation grep trap, worse than `k17`'s.** `rg -r` is `--replace`,
   not a recursive flag, so `rg -rn '<pattern>' .` **succeeds** and prints every
   match with the pattern substituted away — a plausible-looking hit list with
   fabricated contents, where `k17`'s `rg -E` at least errored. Check the output
   resembles what you asked for, not merely that it is non-empty.

**`k20` also settled the CHANGELOG question `k19` left open, and on a stronger
basis than inheritance.** Every existing `--help` mention in `CHANGELOG.md` is
*incidental* — help named as a surface some shipped change was documented on, never
as a change in itself — so there is no precedent for a help-text-only entry, and
creating one would make the "stale prose corrected against already-shipped
behaviour" rule depend on which prose surface carried the staleness. `k21` and
`k22` inherit that answer; both are CHANGELOG-free unless they change behaviour.

**What `retire-no-launch-help-k21` decided, and the seam that nearly caught it
out.** The asymmetry is **fixed, not stated**: *model-per-task-kind* records
"`--no-launch` resolves the launch it declines to perform … it runs the identical
code path rather than a parallel config check" as a property of the *flag*, with
no verb carve-out, so a dry run that resolved only the harness was an unchecked
readiness claim. Retire's dry run now runs the launch's own path up to the exec —
`load_prompt` + substitute, then the invocation assembly that fires the codex
sandbox pre-flight and derives the VCS-store grants. The residual asymmetry (the
report names no leaf, kind or model) is a **fact about the verb** — retire peeks
no leaf and passes no model — and is stated in three places rather than fixed.

1. **The prompt is the finding, and it is unique to this verb.** `grove retire`
   **never provisions** (`provision_all` has one caller, `do_grove`), so
   `load_prompt` reads a global skill dir some *earlier* `grove do` had to have
   written — the one launch dependency a user cannot see, and the one the old dry
   run returned directly on top of. Verified end-to-end in a scratch tree, both
   ways; and against a **real codex**, where an untrusted tree now hits the
   read-only refusal `codex-grant-refused-k35` added and retire's dry run had
   never reached.
2. **The two verbs exec through different bin seams, so the symmetric-looking
   helper is the wrong one.** `loop_driver::harness_bin` honours
   `GROVE_HARNESS_BIN[_<NAME>]`; `exec_harness` has no seam at all. Reusing
   `preflight_check` would have checked a *different binary* than retire runs —
   this leaf's own defect class, reached by reaching for the obvious helper — and
   would additionally fail the dry run on `GROVE_<KIND>_HARNESS` overrides retire
   routes on none of. The check is `harness.exec_bin`, and it is the one step the
   dry run cannot share (the launch finds out by trying).
3. **Fifth-generation grep trap, with a new edge: `--help` text is the wrong
   instrument entirely.** Re-running `k20`'s scan by awk over rendered output
   emitted `awk: newline in string` *and* two false positives (`-h`, `-V`, both
   described) in one run. Worse than `k17`'s `rg -E` or `k20`'s `rg -r`, because a
   scraper must also reproduce clap's **two layouts** — a multi-paragraph
   description switches the *whole command* into long-help form, so this leaf's
   own fix changed how every sibling row renders. The shipped guard
   (`tests/help_surfaces.rs`) walks `clap::Command` for both binaries, where the
   question is a fact rather than a rendering; falsified by mutation, it names
   `grove retire :: argument \`no_launch\`` — the original defect exactly.
   **`k22` should not scrape help text.**

**`k21` is the first of the `k19`–`k22` run to earn a `CHANGELOG.md` entry**, and
it confirms `k20`'s rule rather than bending it: those leaves are CHANGELOG-free
*because they corrected stale prose against already-shipped behaviour*, and this
one changed behaviour. `k22` edits `content/` only, so it stays free.

Externalized rather than absorbed: `retire-harness-stamp-claim-k23`.
`RetireArgs::harness`'s doc is a verbatim copy of `StartArgs::harness`'s and
claims it writes a stamp; `maybe_stamp` has exactly one call site and it is in
`do_grove`. Left standing deliberately — it needs a *decision* (is the doc wrong,
or should `retire --harness` stamp?), not a reword — and it is why `k21`'s
`--no-launch` description omits the "It writes no stamp" clause `do`'s carries.

**What `driving-original-scheme-example-k22` decided, and the precedent that
decided it.** The leaf offered three options and said not to default. Option 1
(*recover the handles from history*) is **impossible, not merely laborious**:
`capture-issues-for-later-groves` ran entirely pre-key, so its own commit
messages name items `050`, `060/010`, `090/030` — the surviving record is keyless
too, and §5's canonical form cannot be constructed for those referents at all.
Option 3 (*replace the example with a live one*) is **rejected on a stronger
ground than freshness**: `content/` is provisioned into groves with no
relationship to this repo, and *any* `.grove/` is private to one working tree and
deleted at its finish cycle — so citing a live tree regenerates the defect on a
timer. Option 2 was taken, but its form came from evidence rather than
invention.

1. **The house pattern already existed one file over, and that is the finding.**
   `docs/driving-a-grove.md` — this doc's contributor-facing sibling — had already
   solved the identical problem for a *different* dead workstream
   (`refactor-to-archon`): it names leaves by **bare slug**
   (`loop-substrate-spike`), anchors every pointer on **durable artifacts** ("the
   research doc, the ADRs"), and carries a *predates-the-scheme* note. So
   `content/driving.md` was not merely stale — it was the copy that never received
   a convention its sibling already ran, and the fix is to apply that convention.
   Bare slugs are right *because* these leaves predate keys: the slug is the
   position-free part of a handle whose key never existed.
2. **The Context under-counted the class by six sites, and the miss is the
   familiar one.** It enumerated `content/driving.md:55-61`; the grep found **13
   lines** — six more `050`/`060` references scattered through *How to write a
   research leaf brief* and three grilling sections, plus the preamble's claim
   that "*paths point at artifacts in that workstream so a reader can trace a real
   chain end-to-end*", which is the summary layer over the very defect and is
   flatly false once `.grove/` is gone. `k17`'s rule earned another generation:
   **a finding against a section does not reach the document's summary layer.**
3. **One blockquote put two established rules in direct conflict.** The sibling's
   rule (and `k15`'s, for `docs/research/`) is that quotations keep their original
   wording because dated evidence is immutable — but the quoted sentence's entire
   content *was* `060, 070, 080, 090`. Resolved by what the quote is **for**: it
   illustrates a *move*, not a historical fact, and **a quotation in a teaching
   document teaches its form** — a reader copies the shape, positional names
   included. So it is now openly re-set in subject names and says so, rather than
   passing off an edited quote as verbatim. The four leaves are named by subject
   (sync semantics + inbox shape, the `grove meta` rename, the LLM/CLI boundary
   audit, the TUI), read off the surviving survey's own `## Synthesis` headings
   and corroborated in the commit log — not from memory, and strictly more
   informative than the positions ever were.

**The class fix is one paragraph in ADR *task-tree-scheme* §5, and it does not
follow from §5's existing rule.** §5 as written was satisfiable by writing a
*correct* `<slug>-k<key>` handle into provisioned content — which an outside
reader equally cannot follow, since a handle resolves only inside the tree that
issued it. The addition draws the line where it actually falls: a **synthetic**
name illustrating the grammar is fine (the position is then the subject), a
reference to a **real** work item is not, whatever its spelling; anchor worked
examples on durable artifacts. `CONTEXT.md`'s [[Work-item handle]] carries the
matching `_Avoid_`. Verified the rule holds over the repo rather than being
shipped already-violated: every `-k<key>` in `content/` is synthetic
(`sync-design`, `sync-survey`, `extract`, `plan`, `spec`, `build`, `mid`), and
`content/TASK-FORMAT.md`/`SKILL.md`'s filename-grammar illustrations are the
legitimate case the rule carves out.

CHANGELOG-free, and it confirms `k20`'s rule rather than bending it: no
behaviour changed, and an ADR paragraph is a *recorded* convention — "*a decision
earns its entry when its behaviour lands, not when it is recorded*" (`k13`).
`cargo test`: 627 passed, 0 failed.

**What `retire-harness-stamp-claim-k23` decided, and the four surfaces the
Context did not know about.** The doc was wrong; the behaviour stands. The rule
is *a lasting binding is written by the action that asks for one* — which is not
a new principle but the one already in the codebase as `do`'s `maybe_stamp`
sitting **below** its `--no-launch` return ("a documented dry run must never
permanently rebind the grove"). `grove retire --harness` is the same shape: a
one-off by-hand verb, outside the routing lattice entirely (it peeks no leaf and
passes no model), is a strange place to write that lattice's fallback. Symmetry
across the two verbs was the obvious alternative and is rejected on **cost
asymmetry** — an unpersisted `--harness` surprises once and immediately, while a
retire that rebound the grove redirects every later `grove do` and is found
sessions later. Recorded as a Consequences bullet in ADR *model-per-task-kind*
(the record that already held the `--no-launch` half, so the set stays minimal —
it is one rule with two instances, not a new decision), with a *what would reopen
it* trigger; plus both `--harness` doc comments, `maybe_stamp`'s doc, `CONTEXT.md`,
`README.md`, `docs/grove.md` and `docs/workflows/start.md`.

1. **The claim rode five surfaces, and four were verb-generalised prose the leaf
   never named** — `README.md`'s "Session launchers stamp … whenever `--harness`
   is passed explicitly", `docs/grove.md`'s "The CLI writes a one-line stamp"
   (twice, one of them directly after the paragraph that already lists the two
   verbs' *other* asymmetries), and `docs/workflows/start.md`'s "Passing
   `--harness` explicitly **always** writes the stamp". `k11`'s rule again, and
   the fix needed no invention: `.gitignore:15` already said "Written by any
   `grove do --harness <name>`" — the one surface that had scoped it to the verb,
   which is `k22`'s *the house pattern already existed one file over*.
2. **The leaf's "only other clause is shared and correct" is falsified.** The
   default is the **stamp, then** auto-detection; "(default: auto-detected from
   the repo's harness directories)" only read as true in the company of the
   stamping sentence that followed it. So deleting the false clause from
   `RetireArgs` would have promoted an imprecise one into being the whole account
   of the default. Both structs now state it in full. **A clause rescued by its
   neighbour is not correct — it is load-bearing on the sentence you are deleting.**
3. **The guard is a *real* launch, and that is the whole point of it.** The cheap
   version — assert no stamp after `retire --no-launch` — passes on the exact
   regression it exists to catch, because a retire that grew a `maybe_stamp` would
   put it where `do`'s is: below the no-launch return. `retire_never_stamps_the_grove`
   spawns a fake harness through PATH (`exec_harness` has no bin seam) with an
   explicit `--harness`, the one trigger that always stamps on `do`. Falsified by
   mutation. Verified by hand too, both halves against one fixture: `retire
   --harness` left no stamp, `do --harness` wrote `claude`, and `retire --harness
   pi` against that stamped grove launched pi while leaving the stamp reading
   `claude`.

CHANGELOG-free, the third leaf running to confirm `k20`'s rule: no behaviour
changed — stale prose corrected against shipped behaviour, an ADR paragraph
(*recorded*, not landed — `k13`), and a test. Verified by `jj diff --stat`.
`cargo test`: 628 passed, 0 failed; clippy and `cargo fmt --check` clean.

Externalized rather than absorbed: `src-position-citations-k24`. Two `src/`
comments still cite dead `.grove/` positions — `leaf_id.rs:143` ("the grow verbs
(030)") and `provision.rs:7` ("whose deletion is leaf 090") — which makes
`stale-module-headers-k14`'s claim below **false as written**. Both narrowings in
`k14`'s Done-when leaked: it swept module headers (`//!`) against an *enumerated
pattern list*, and these are a function doc comment and an unlisted spelling.
`k22` measured the negative space too, so `k24` inherits it: `D<n>` is genuinely
at zero, and every remaining dotted-decimal or three-digit token in `src/` is a
version, a timing, the v1 grammar *as subject*, or migration fixture data.

**What `src-position-citations-k24` fixed, and the one site that was a false
claim rather than a stale citation.** `provision.rs:7` was the easy half — the
parenthetical was redundant beside the two ADR slugs four words earlier, so the
position went and "now deleted" carries what survived. `leaf_id.rs:143` was not:
*"Public so the grow verbs (030) can parse a bare `<parent-id>`/`<target-id>`
positional"* names **a consumer that does not exist**. `leaf_id::parse_position`
has exactly one call site — `parse`, in its own module — and the grow verbs parse
their positional through `tree_id::parse_position`, a different function under the
same name returning one `u32` instead of a `Vec<u32>`. Its **second** clause ("*and
the position prefix of a `# …` header*") is false too: `tree_migrate` rewrites v1
headers by token substitution (`rewrite_header_line`), building the old token from
an already-parsed id rather than parsing a prefix. So swapping `(030)` for an
ADR slug would have **laundered a false sentence into a well-cited one**; this is
`retire-harness-stamp-claim-k23`'s lesson inverted — there a true clause was
load-bearing on a false neighbour, here a false clause was hiding behind a
plausible citation. The residue was turned into the warning `k14` already wanted
(`tree_id.rs:36` and `leaf_id.rs:15` each warn about the same-named pair), so the
fix names a **module path**, which is `k14`'s own settled rule.

1. **The method was enumerate-then-classify, and that is the reusable part.**
   `k14` swept an *enumerated pattern list* and leaked; re-running a longer list
   would only move the leak. Instead: extract **every** numeric-bearing token from
   every comment in `src/` (95 distinct), then classify all 95. That is complete by
   construction rather than complete-as-far-as-the-list, and it is what surfaced
   `k25`.
2. **The discriminator is dereference-vs-subject, and it is semantic.** A position
   is an instance when the reader must *look the work item up* to use the comment;
   it is fine when the position **is** the subject — `tree_migrate`'s golden v1
   fixture ("node 11 — brief + DONE children + live leaves", whose literal strings
   sit three lines below), a byte count, a synthetic grammar illustration. Nine such
   sites survive in `src/` and are correct. **This is why no guard test shipped**:
   any regex either flags those nine or narrows to a pattern list, re-committing
   `k14`'s defect while wearing a test's authority.
3. **Sixth-generation grep lesson, and it is a new move rather than another trap.**
   `k17`/`k20`/`k21` each learned to check *that the command ran*. That is not
   sufficient — a correct-but-blind instrument also reads clean. So the clean result
   was falsified two ways: a **positive control** (the same `rg` must find
   `task-tree-scheme` in `src/`, and does) and a **cross-tree control** (the same
   dereference patterns must find the class *somewhere*, and do — in `tests/` and
   `codex-bridge/`). A flag error or a bad pattern reads clean **everywhere**;
   clean-here-plus-dirty-there cannot be produced by a broken instrument. `D<n>` in
   `src/` is therefore re-derived at zero, not inherited from `k22`.

Externalized rather than absorbed: `nonsrc-position-citations-k25` — four sites in
`tests/` (including `harness_stamp.rs:85`'s `D5:`, the class `k14` and `k22` both
reported at zero, because neither claim ever reached `tests/`) and one in
`codex-bridge/src/main.rs`, which cites a **foreign repo's** `.grove/` path plus
that dead brief's `D2/D3/D6` and so needs a *decision* (promote the load-bearing
reason, or drop the pointer), not a reword. **The scope narrowing is the third
generation of `k14`'s defect**: `k14` narrowed by comment species and pattern
list, `k24` inherited a narrowing by *directory* — and `codex-bridge/src/main.rs`
**is** a `src/` path, just not the root crate's, so even the word "`src/`" did not
mean what a reader of the claim would take it to mean. **A claim's scope is part
of the claim, and a scope stated as a path goes stale exactly as a file list
does.**

CHANGELOG-free, the fourth leaf running to confirm `k20`'s rule: no behaviour
changed — two comments corrected against shipped behaviour. `cargo test`: 628
passed, 0 failed; clippy and `cargo fmt --check` clean.

**Divergence 1 — Tasks 1 and 2 merged.** They write the *same file*, and the
plan says to run it under `superpowers:subagent-driven-development`, where
splitting is cheap because subagents share the parent's context. Grove leaves are
cold-start sessions: a second leaf would re-read the file and re-establish the
whole CLI contract just to append two sections. A ~120-line `SKILL.md` fits one
session, so the split bought nothing and cost a bootstrap.

**Divergence 2 — a review chain was added.** The plan argues per-task
verification suffices. The evidence says otherwise: the plan's own Global
Constraints assert a "verified" fact that is false (see the `min_degree` note
below). The commands had been run and the projection checked *did* match — the
prose was still wrong. That is a **reading** failure, not an execution failure,
and only a fresh context re-deriving the claim catches it. The BRIEF's own
done-when is a verification claim, so it deserves an independent certifier.
Confirmed with the human at scoping time, at a cost of two extra sessions.

**Not changed:** `distribution-k5` stays separate — different file, and its
verification writes symlinks into `$HOME`, a side effect outside the repo.

## Pointers

- Spec: `docs/superpowers/specs/2026-07-29-portable-codebase-memory-skill-design.md`
- Plan: `docs/superpowers/plans/2026-07-29-using-codebase-memory-skill.md` —
  three tasks, each with verified commands and expected output.
- Prior art in-repo: `plugins/linkuistics/skills/using-jujutsu/` (naming and
  house style), `install.sh` (distribution — globs the skills directory, needs
  no change).

## Notes

**Why a shell path at all.** Pi refuses MCP by design ("*No MCP.* Build CLI
tools with READMEs"), so a capability shipped as an MCP server strands one
harness and needs three config dialects for the other three. `SKILL.md` plus a
CLI reaches all four. `codebase-memory-mcp` exposes the same fourteen tools both
ways.

**Verified contract** (against `codebase-memory-mcp` 0.8.1, re-derived
end-to-end by `skill-k2`; the shipped `SKILL.md` is now the authoritative
statement of all of it):

1. `project` is required; it is **not** inferred from the working directory.
   `list_projects` and `index_repository` are the exceptions.
2. On **success** JSON goes to stdout and logs to stderr, so `| jq` is clean.
   On **failure** stdout is *empty* and the error goes to stderr — so `| jq`
   shows nothing and the pipe masks exit `1` as `jq`'s exit `0`.
3. Malformed JSON is **discarded, not reported**: arguments become empty and
   you are told "project not found", the same message an unindexed project
   gives. Build interpolated arguments with `jq -n`.
4. `search_graph` truncates at `limit`, default **200**, flagged only by
   `has_more`/`total`. `trace_path` caps callers at 100 with no flag at all.
5. `min_degree` gates on **total** degree (in + out); `relationship` and
   `direction` do not make it directional. The "high fan-in" recipe in
   `~/.claude/skills/codebase-memory/SKILL.md` is wrong for this reason.
6. Exit status is honest (0/1), observable **without** a pipeline, and
   defeated by `--json`, which also duplicates its envelope to both streams.

**Corrections `skill-k2` made to the plan, the spec, and this brief.** Running
every documented command falsified several claims all three carried as verified:

- The `| jq -r '.error'` idiom cannot work — errors never reach stdout. (It is
  the **plan**'s, at Task 1 Step 4, not the spec's as this brief first said; the
  spec's version of the same error was an unqualified "`| jq` is therefore clean
  with no redirection needed".)
- "produces byte-identical results" (plan) and "results[] identical" (this
  brief, at scoping) are both false. `relationship`/`direction` drop 2 rows of
  2460; the earlier check saw agreement only because `limit:5` hid it. The
  durable, reproducible claim is item 5 plus: exactly **half** the rows that
  filter returns have `in_degree: 0`.
- The plan's flagship composition recipe passes `limit:200` — the default — and
  sorts client-side, so its "top 20 fan-in" ranks an arbitrary 8% of 2460
  matches. Aggregation and global top-N belong in `query_graph`'s Cypher.
- `trace_path` on a bare `function_name` shared by several symbols resolves to
  **none** of them: 8 `wait_for_socket` symbols, 0 callers, exit 0. The
  `qualified_name` returns 67.

**`docs/` is reconciled** (`docs-reconcile-k6`). Both documents now open with a
status banner naming the shipped `SKILL.md` as the authority for every CLI
behaviour claim; the plan's Global Constraints and both of its embedded
`SKILL.md` drafts are **deleted** rather than annotated, on the grounds that a
second copy of a file that already exists is the defect, not the wording. What
survives in each is the design reasoning and the record of what ran. The
falsified claims are kept only quoted-and-refuted, so the correction is legible
without the assertion surviving anywhere.

**Distribution — `install.sh` cannot be run from this working tree.**
`distribution-k5` verified the install path against an **isolated `HOME`**, not
the real one, and left `$HOME` untouched. The script derives its link source from
`${BASH_SOURCE[0]}` and unconditionally re-links, so running it here — a
*secondary* jj workspace, while the default is `/Users/antony/Development/grove`
— would have re-pointed all 15 already-installed linkuistics skills at a tree
that dies with this grove, silently. The evidence it did produce is stronger for
the actual claim: `48 = 16 skills × 3 harnesses`, all three targets exercised,
including `~/.gemini`, which does not exist on this machine and would otherwise
have printed `skip`. The manifest edit and the glob-pickup are therefore fully
verified; **only the real-machine install is outstanding**, and it is not
grove's to do — `install.sh` should be run from the default workspace once this
grove's work is integrated.

**The underlying defect is now fixed** (`install-workspace-guard-k8`).
`install.sh` probes whether the tree it lives in is the repo's main checkout —
jj-first, mirroring the binary's `repo::vcs_of`, which is load-bearing rather
than merely consistent because a secondary jj workspace of a colocated repo is
not a git worktree and a git-first probe would miss it entirely. It **refuses**
rather than warns, since the damage is silent and delayed; `--force` opts in for
the one legitimate case, testing an unmerged skill live. `install.test.sh` covers
nine tree shapes against an isolated `HOME`. `docs/adr/symmetric-vcs-rule.md` now
names three enforcers, not two. **This does not change the outstanding item
above** — the real-machine install still has to be run from the default
workspace, and the guard is precisely what now makes running it from here fail
loudly instead of silently.

**Authoring authority.** The plan cites `superpowers:writing-skills`. This repo
ships `linkuistics:authoring-conventions`, a **house delta that overrides
upstream's description rule** — house is capability + "Use when …", upstream is
when-only, and upstream's version is injected every session and will tempt an
implementer to strip the capability clause. Read both; house wins.

**Test fixture:** `Users-antony-Development-herdr` — a *live, drifting* index
(23,641/97,504 at plan time, 23,681/97,906 at scoping). Treat every exact figure
as a re-check, and keep counts out of the skill.

**This repo is indexed now**, where scoping recorded it as not —
`Users-antony-Development-grove.using-codebase-memory` appears in
`list_projects` as of `k13`. A leaf working in `src/` can query the graph rather
than only grep.
