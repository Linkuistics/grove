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
| `confirmation-prose-k15` … `-k17` | `impl` chain | `k15` **done** — reconcile `content/`, `docs/` and `src/` prose to that decision |
| `changelog-release-rename-k18` | `impl` | make `cargo release` rename `## Unreleased`, rather than the prose note `k13` left — raised by `k13` |

**What `chain-as-node-k7` decided**, since the tree below builds on it and
`.grove/` dies at the finish cycle. A chain gets its own **node directory** —
reversing a decision the ADR, the spec and the glossary each recorded. The three
arguments that had rejected it lapsed: `leaf-add-chain` made node creation
proactive, the node is **brief-less by rule** so it buys no unearned `BRIEF.md`,
and a brief-less node is never asked the Retire cascade's confirmation. The
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

**The heading's release-time cost is real and only half-paid.** `release.toml`
carries no `pre-release-replacements`, so `cargo release` does not touch
`CHANGELOG.md` — the rename to `## v<next>` is a *manual* step. `k13` recorded it
in `release.toml`'s usage comment beside the tag push, and externalized
automating it as `changelog-release-rename-k18` rather than absorbing it: proving
a replacement works needs a `cargo release` dry run, which `release.toml`'s own
preamble says the harness classifier refuses as opaque.

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
which is false — `Kind` is live everywhere). **`src/` comments no longer cite
`.grove/` positions at all** — every `11.x`, `070/040`, `060/020` and `D<n>` is
gone. That is not a style preference: ADR *task-tree-scheme* §5 binds "commit
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
