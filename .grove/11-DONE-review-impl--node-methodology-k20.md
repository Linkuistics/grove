# node-methodology-k20

**Reviews:** node-methodology-k8


## Goal

Review the shipped methodology's node-file grammar and mandatory brief chain
against the implemented CLI, before documentation and cutover depend on it.



## Context

- Read the producer's commit by handle. Its artifact is the shipped skill
  corpus, conformance registry and checker controls; installed 20.2.0 skills
  remain untouched until cutover.
- The naming ADR and the root brief define `NN-k<key>/_<slug>.md`, root
  `_BRIEF.md`, filename-derived titles and the unchanged leaf grammar.
- Specific doubt: phrase-based checks establish reachability and known wording,
  not whether bootstrap, decomposition, planning, retirement and artifact
  guidance teach one coherent path. Check role/body terminology, malformed-level
  refusal, scaffold claims, handles, heading examples and format-file links.
- Inspect checker classification and mutation controls without treating them
  as a full filename parser or a semantic proof. The graph CLI was unavailable
  during production; use current evidence rather than assuming an index exists.

## Done when

- Findings are tied to the producer's artifact and contract, with actionable
  evidence and accepted trade-offs distinguished from defects.
- Any required integration runs before `node-documentation-k9`.

## Notes

No installed-plugin refresh, release, or conversion of this live tree belongs
to this review. Those operations remain owned by `node-cutover-k10`.

## Findings

Reviewed: commit `krmytlzt` (`2d8ae008`), *node-methodology-k8: teach mandatory
named node files in shipped skills*, read against the working tree at that
commit (the working copy was empty on top of it). The artifact is the shipped
skill corpus under `plugins/grove/skills/`, the rule registry and the
conformance checker with its controls; coordinates are `path:line` in that
tree. The implemented CLI was read from the built `target/debug/grove-llm`
help text and from source, never run against a tree. Inspection only: no
check, test or conformance run was executed, and the producer's recorded
verification was read, not reproduced. The graph CLI was not used. Ordered by
severity.

### F1 — The refusal path is understated, and nothing says who resolves it (actionable, low–medium)

`plugins/grove/skills/grove/references/bootstrap.md:17-18` is the whole of what
a session is taught about a malformed level: *It refuses malformed levels
instead of skipping gaps; follow its named refusal before continuing
bootstrap.* `BRIEF-FORMAT.md:39-41` says the same thing from the grammar's
side: *Tree reads and mutations refuse that level by name.* Two things are
understated. First, the refusal is not of a level but of the whole read:
`brief-chain --help` says *Missing or misplaced node files refuse the whole
tree read*, `docs/ARCHITECTURE.md:318-321` says the reader validates every
reachable level before any selection or mutation *so an early live leaf cannot
hide a missing or second node file elsewhere*, and `pick`, `resolve`, `kind`
and every grow verb refuse the same way — a session cannot route around a bad
level by asking a different verb. Second, *follow its named refusal* does not
say whether the session or the operator acts on it. Everywhere else the corpus
is explicit that this is the operator's: `driver.md:88-89` (*An operator must
resolve the refusal before restarting the loop*), the naming ADR (*The
operator pays for strictness when hand-editing*), the glossary's *No
migration* entry (*Operators rename or restore entries*), and the parallel
rules for the other tree-shaped stops — `bootstrap.md:6-9` (a stale launch:
*say which of the two you got, and stop*) and `retire.md:28` (*An agent never
prunes on its own*). Read by an AFK session, *follow* is a licence to rename
`.grove/` entries by hand and carry on, which is the one mutation grove itself
refuses to perform. Correction: at `bootstrap.md:17-18` say that a refusal
from `brief-chain` — or any verb — is whole-tree, names the level and the
canonical form, and ends the session the way a stale launch does: report the
refusal verbatim and stop, the operator repairs. At `BRIEF-FORMAT.md:40` say
*refuse the whole read, naming that level* rather than *refuse that level*.

### F2 — `BRIEF-FORMAT.md` tells the session to create the node file, which no session does (actionable, low)

`BRIEF-FORMAT.md:42`: *Create the node file with its node; add brief sections
only when they earn their place.* Two paragraphs earlier (`:29-30`) the file
already says `leaf-decompose` moves the body in and `root-init` scaffolds the
root's; `driver.md:69` says a session *never scaffolds the tree itself*; and
`docs/ARCHITECTURE.md:329-331` says *No writer creates a charter outside that
plan.* A hand-created node file is precisely the operator recovery path, and
the missing-node-file diagnostic the reader emits ends *do not allocate a
fresh key or manufacture a brief* (`crates/grove-loop/src/task_name.rs:660`).
The imperative invites the one act the diagnostic forbids. Correction: state
it as a fact rather than an instruction — the node file arrives with its
node, from `leaf-decompose` or from the root scaffold, and a session writes
only its body.

### F3 — The CLI's own vocabulary still calls the node file a brief (observation; outside this artifact)

The skills now follow the glossary's *Node file* entry — *Avoid: using
"brief" for the filename's role — the brief is the body* (`CONTEXT.md:901-902`)
— but the binary sessions are told to consult does not: `pick --help` skips
*briefs*, `leaf-retire --help` *Refuses a brief*, `leaf-prune --help` likewise,
`leaf-decompose --help` *Prints the brief's absolute path*, `root-init --help`
writes *the root `_BRIEF.md` charter* and prints *the charter's path*; the
refusal messages at `crates/grove-loop/src/tree_lifecycle.rs:612`, `:748` and
`:852` say *cannot decompose/retire/prune a brief*. `decompose.md:35-37` sends
a session to `--help` rather than a transcription, so the two vocabularies meet
in every session that grows the tree. This is not the producer's: `cli.rs` and
`tree_lifecycle.rs` are declared book roots, so the change carries fragments
and prose and belongs to a code leaf — `node-cutover-k10` is the last one — or
to a `leaf-add`. The integration should route it, not fix it; accepting the
CLI's looser word is also a legitimate answer, provided it is recorded.

### F4 — Checker classification and controls (trivia; holds)

The classification order is right: the `_BRIEF.md` case arm precedes the
titled-node-file regex (`conformance.sh:213-219`), `BRIEF-FORMAT.md` cannot
match `^_` so it stays in the checked skill class, and the
`missing-brief-format` control proves that negative. The widened leaf regex
(`:221`) has its positive control. Both deletion controls
(`conformance.test.sh:109-121`) remove the one line carrying each pinned
phrase (`BRIEF-FORMAT.md:39`, `bootstrap.md:17`) and fail loud if the phrase
ever moves off that line. Two trivia: the class comment at
`conformance.sh:188-189` still lists *a brief* among tree artifacts, where the
word now means a body; and a path-shaped token such as `01-k7/_extract.md`
classifies as `repo` (the `*/*` arm at `:208` precedes the tree arms) and is
silently accepted — harmless for assertion 3, worth a comment rather than a
change.

### What was checked and holds

- **Grammar statements against the ADR and the reader.** `BRIEF-FORMAT.md:5-20`
  and `TASK-FORMAT.md:19-28` state the grammar the naming ADR fixes: slugless
  `NN-k<key>/`, `_<slug>.md` in a positioned node, `_BRIEF.md` only at the
  root, `BRIEF` reserved, positions of at least two digits without excess
  zeros, positive keys without a leading zero, the next key above the highest
  in the tree (`crates/ordinal-fs-tree/src/name.rs:64`). *Malformed names stop
  tree operations, naming the path and the canonical form* is what
  `task_name.rs:654-655` emits; the level error names the required form and
  the offending names (`:658-660`).
- **Heading conventions.** The root heading is `# <worktree basename> — brief`
  (`tree_lifecycle.rs:1033-1050`), matching `BRIEF-FORMAT.md:72`; a decomposed
  node gains ` — brief` idempotently with custom headings preserved
  (`:1064-1073`, test at `:1104`), matching `TASK-FORMAT.md:129-130`. Titles
  are read from names only, as `BRIEF-FORMAT.md:10` says.
- **Handles.** `BRIEF-FORMAT.md:8-9` and `TASK-FORMAT.md:25-27` match
  `docs/specs/module-decomposition.md` decision 4 and `resolve --help`: a
  node's handle is its file's slug plus its directory's key, the root has none.
- **Scaffold claims.** `driver.md:63-67` matches `root-init --help`,
  `docs/ARCHITECTURE.md:965-972` and the witnessless-root ADR: root, `_BRIEF.md`
  and first leaf in one plan, a taskless root refused and never a finish
  signal. `fresh-grove-start-contract` resolves to `docs/ARCHITECTURE.md:1004`.
  No witness-creation teaching remains: a sweep for *witness*, *skipped
  silently*, *lazy artifact*, *charter has not been written* and `/BRIEF` over
  `plugins/grove/skills/` is empty, with the cross-tree control dirty at
  `docs/USAGE.md:206`, `:369` and `CONTEXT.md:990`.
- **No `BRIEF.md` spelling survives under `plugins/`**; the same pattern is
  dirty in `docs/USAGE.md`, which `node-documentation-k9` owns.
- **Registry and reachability.** The renamed rows `brief-chain-refuses-gaps`
  and `tree-grammar-is-required`, and the re-pinned `every-node-carries-a-brief`
  and `structure-brief-is-three-citable-things`, each bind through a file on
  the loaded path (`bootstrap.md` and `driver.md` via the spine `SKILL.md`,
  `BRIEF-FORMAT.md` via `decompose.md:38`); no removed paraphrase returns.
  Rule ids keeping the word *brief* is stable naming, not drift.
- **Role/body terminology.** Every touched site uses *node file* for the name
  and *brief* for the body: `grove-draft/SKILL.md:41`,
  `grove-planning/SKILL.md:40`, `retire.md:80-81`, `grove.md:16,24`,
  `SPEC-FORMAT.md:15,30,119`, `decompose.md:126,215,227,277`,
  `editorial.md:41`. The *charter never written* paragraph is correctly gone
  from `retire.md`, since the reader now makes that state unreachable.
- **The walk.** `driver.md:5-8` matches `pick --help`: positioned children in
  numeric order, node files and terminal leaves skipped, foreign names ignored.
- **One path.** Bootstrap consumes `brief-chain` paths; decomposition makes
  nodes only through verbs and bodies through `BRIEF-FORMAT.md`; retirement
  closes a node from its brief's `Done when` and promotes upward; `grove.md`
  places what outlives the tree. The seam is F1, and it is a wording seam, not
  a design one.

## Decisions (running log)

**1. Four findings; one integration leaf.** F1 and F2 are actionable against
the producer's own Done when (*bootstrap … guidance name … accurately … with
malformed levels refused*) and against the coherence the mandate doubted; F3
is outside the artifact and is recorded for routing; F4 is trivia over
controls that hold. Documentation at `node-documentation-k9` will teach from
these files, so the integration goes ahead of it: `leaf-insert` targeting
`node-documentation-k9`, the first sibling entry after this review whose
subtree holds live work, with the bare stem `node-methodology`. Its
body names this review's handle and not the findings.

**2. Inspection only, no in-session reviewer.** Nothing was built, tested or
run against a tree; every claim above is read off the tree at `krmytlzt` and
the built binary's help text. Bookkeeping uses the installed 20.2.0 binary,
because this live tree is still on the grammar that binary reads.
