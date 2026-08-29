# decomposition-k2

## Goal

Turn `minimalism-k1`'s five module contracts into interfaces a session can build
against, and land the agreement point: a spec under `docs/specs/`, signed off by
the human, before decomposition cuts impl leaves.

## Context

`minimalism-k1` is the whole input and it is unusually complete — read its
`## Decisions (running log)`, `## Module contracts` and `## Deletion list` before
anything else. Nine decisions were settled with the human and none of them is
open for re-litigation; `SPEC-FORMAT.md`'s *synthesise, never re-interview* rule
binds hard here.

The four that most constrain this session:

- **jj only.** The git lane is dropped, which is what makes the 10,366-line
  finish group deletable rather than lane-conditional.
- **Five workspace crates under `crates/`**, one release process. A module *is* a
  crate, so "testable through its own interface without the other four" is
  compiler-enforced.
- **The kind is a skill name.** `${prompt}` says *load `grove-<kind>`*; grove
  holds no set of kinds, no reference-file mapping and no `match`. A kind exists
  iff its skill does.
- **The methodology ships as a plugin and the version check inverts** — grove
  publishes a version, the skill checks it and decides.

Also read `docs/research/wording-micro-test.md` before touching prompt
composition: the 0/10-vs-10/10 result is the reason the prompt names one target
and asks the session to select nothing, and any redesign of `${prompt}` has to
preserve that property or restate the evidence against it.

## Done when

- Each of the five crates has a named public interface — types and signatures,
  not prose — that a reader can implement against.
- The spec is written under `docs/specs/`, cites the ADRs in its area rather than
  restating them, and carries the four agreed test seams.
- The human has agreed the shape.
- The next leaf is cut. `references/decompose.md`'s **expand → migrate →
  contract** rule almost certainly applies: this is a wide refactor whose blast
  radius makes any single vertical slice unable to land green.

## Notes

**Seven things `minimalism-k1` deliberately left to this session.**

1. **Crate names.** Five of them, and `ordinal-fs-tree` is already taken and
   staying.
2. **The store's four new operations.** `exists?`, `initialize`, `delete` and a
   no-outcome answer to a search. The last is the sharp one: grove's current
   no-work signal is `Option<SelectedLeaf>` (`task_tree.rs:584`), whose predicate
   is grove vocabulary and **cannot move as-is**. The store needs a way to say
   *found nothing* that is domain-free, alongside `Refusal`'s twelve variants,
   all of which are refusals to *mutate*.
3. **What the name parser yields once `leaf::Kind` loses its closed enum.** It
   still parses a kind out of a filename; it simply no longer validates one
   against a compiled set.
4. **The handle type.** Principle 3 says one type owns a name end to end, and the
   handle is a projection of the same name — six hand-rolled implementations
   today (`task_tree.rs:513`, `tree_lifecycle.rs:220`, `finish_cleanup.rs:121`,
   `task_grow.rs:475`, `tree_lifecycle.rs:1174` produce; `task_tree.rs:952` and
   `task_name.rs:609` peel). `EntryName` in the store is the model to follow.
5. **What grove publishes as its version/signature, and how a skill reads it.**
   One workspace, one release version (decision 2), so the value exists; the
   surface does not.
6. **How fat the nineteen skills are.** The kind is the skill name, and whether
   each is a thin pointer over a shared `grove` spine or a full standalone skill
   was left open on purpose.
7. **The plugin's conformance runner.** `behavioural-coverage-asserts-delivery`'s
   rule survives and its instrument moves out of `cargo test`; the plugin has no
   test runner today.

**On ADRs.** `minimalism-k1` rewrote none, deliberately — `docs/adr/` describes
the design's *current state*, and rewriting a record to describe unbuilt code
would make the set lie. Its `## ADR reconciliation` section names which records
each decision obliges and who reworks them. The spec is where the target design
lives until then; cite the records, do not restate them.

## Decisions (running log)

**Four Rust crates, two thin binary crates, one plugin — not five crates.**
Decision 2 of `minimalism-k1` says a module *is* a workspace crate, and the task
file reads that as five crates. Four of the five modules are code and become
crates; the **skills** module's artifact is markdown that ships by an entirely
different path (decision 6), so it has no crate and gains nothing from one. Its
half of the compiler-enforced done-when is met the way its contract already says
— *"tested through its own conformance runner, not `cargo test`"*.

| module | crate / package | domain-free |
|---|---|---|
| tree store | `ordinal-fs-tree` (unchanged) | yes |
| runner | `keyed-launch` | yes |
| VCS seam | `jj-workspace` | yes |
| loop | `grove-loop` | no |
| skills | plugin at `plugins/grove/` | no |
| — | `grove` (bin), `grove-llm` (bin) | — |

The two binaries are **separate crates**, not `[[bin]]` targets inside
`grove-loop`, and that is the same argument decision 2 makes for the modules: a
binary target inside a library crate can reach that library's private items, so
*"the binary is thin"* stops being compiler-enforced the moment it is a target
rather than a crate.

`jj-workspace` comes out **fully** domain-free rather than the brief's *partly*.
Its whole surface is *resolve a jj workspace, refuse a tree that is not one, take
a path-scoped commit*, and principle 2's remedy for the refusal is `jj git init
--colocate` — jj's advice, not grove's. The brief's hedge was priced against a
seam that still owned the finish transaction, which decision 1 deleted. So the
brief's *three reusable outside grove* holds with a cleaner third member.

`keyed-launch` is named for its interface rather than its behaviour: the key is
what a consumer names, supervision is what sits behind it. Its vocabulary —
*key*, *template*, *launch*, *child*, *signal*, *escalation* — deliberately
avoids **session**, which would be a fourth entry in `CONTEXT-MAP.md`'s collision
table against grove's own sense of the word.

**Workspace resolution belongs to the VCS seam, not the loop.** `minimalism-k1`'s
`## Context` splits `repo::`'s twelve functions with workspace resolution
(`workspace_control`, `main_repo_of`, `vcs_of`, `toplevel`, `path_is_tracked`)
going to the loop and only the commit boundary going to the seam. That split was
measured *before* decision 1 landed, and decision 1 makes it wrong: the seam's
own guarantee is now *"a non-jj working tree is refused before any mutation, with
the command that fixes it"*, and that refusal **is** `vcs_of`. Splitting them
would leave the loop shelling out to `jj` and the seam unable to state its own
precondition. The module contracts written after the decisions already imply
this; the earlier sentence is superseded, not re-litigated.

**The store answers `exists?` by the shape of what opening returns, not by a
predicate.** A separate `exists(root) -> bool` is a check-then-act split, and
check-then-act over a tree under a lock is exactly the disease grove's two-phase
`Classification`/`settle` dance exists to paper over. Instead the two entry
points keep their names and widen their result: opening for read yields a tree or
a vacancy, opening for write yields a tree guard or a **vacancy guard**, and
`initialize` is a method on the vacancy while `delete` is a method on the tree
guard. One lock acquisition, and the answer hands you the only operation valid
for it — a caller cannot initialize over an existing tree or delete a vacancy,
because the types do not offer it. Something at the root that is neither a tree
nor nothing is an `Error`, not a third variant.

**The fourth new operation is `Sought<T>`, and it replaces `Option` across the
store's whole search surface.** `Refusal`'s twelve variants are refusals to
*mutate*; a search that completed and matched nothing is neither a refusal nor an
error, and a store whose only word for it is `None` forces every consumer to
invent the answer in its own vocabulary — which is precisely what grove's
`Option<SelectedLeaf>` is. Two variants (`Match` / `Nothing`), one word for one
concept: `find` becomes `seek` and `by_key` returns it too.

**`entries-are-never-removed` needs one clause, and this is a correction to
`minimalism-k1`'s ADR reconciliation.** That leaf lists the record as *untouched
and more load-bearing*, but its opening sentence is *"`ordinal-fs-tree` offers no
removal operation"*, and the store is about to offer `delete`. The record's
argument is untouched — it is about **entry** removal lowering the key maximum —
so the fix is one clause distinguishing removing an *entry* from deleting the
*root*, which brief principle 5 already states and the record does not. The impl
leaf that lands `delete` amends it.

**`.grove/FORMAT` is deleted with migration.** Its only job is the
legacy/current discriminator, migration is deleted, and the brief records that no
legacy tree needs it. It costs the store nothing today — `TaskName::parse` reads
it as `Foreign` and the walk skips it — so it is not load-bearing on anything
that survives. A genuinely legacy tree now fails on its **names**, through
`TaskNameError`, which already carries what is on disk and what it should be.
That is principle 2's message in the place a reader will actually meet it.

**The filename grammar gains a `--` between kind and slug, and this is the one
thing decision 7 breaks that `minimalism-k1` did not see.** Once `Kind` is an
open token, `02-design--decomposition-k2.md` no longer has one parse: kind
`design` + slug `decomposition`, and kind `design-decomposition` + an empty slug,
are both readable, and with a three-word kind like
`integrate-review-design` the ambiguity is four ways deep. Today it is resolved
only by matching against the closed set — the very thing decision 7 removes. Two
filenames naming one entry is the hazard the store's canonicality obligation
exists to forbid, and here it is one filename naming two entries, which is worse:
the **handle** is what differs between the readings, and the handle is the
identity crossing every module boundary.

The grammar becomes:

    NN-[DONE-|ABANDONED-]<kind>--<slug>-k<key>.md      leaf
    NN-<slug>-k<key>                                    node directory

Split the middle at the **first** `--`; neither kind nor slug may contain one.
`format(parse(f)) == f` holds, the key stays terminal, node names are untouched,
and the kind token stays byte-identical to the skill suffix — `review-design`
names `grove-review-design` with no mapping, which is the property decision 7 is
for.

Considered and rejected: **spelling multi-word kinds with an inner `_`**
(`review_design`), because the filename token and the skill name would then
differ by a rule, reintroducing exactly the second source decision 7 deletes;
**moving the kind after the key** (`NN-<slug>-k<key>-<kind>.md`), which reads
worse, unseats the terminal-key rule `resolve` and the glossary both lean on, and
only relocates the delimiter problem; and **forbidding hyphens in slugs**, which
renames just as much for a worse read.

Cost: every live leaf filename changes by one character. This grove has a handful,
and the brief already licenses breaking legacy trees. **Sequencing constraint for
the planning leaf:** this repo is a meta-grove, so the tree can only be renamed
once the *installed* binary parses the new grammar — the rename and the reinstall
are one step, and no session may run between them.

**`Kind` is an open validated token; the parser still yields one.** It keeps its
place in `Parts::Leaf` and its `Display`, and loses only the closed set: `new`
validates the token's *shape* — non-empty, lowercase ASCII letters, digits and
single hyphens, no `--`, not a reserved word — and nothing else. `Kind::ALL`,
`Kind::parse`'s nineteen-arm match and `TaskNameError::UnknownKind` all go;
what replaces the last is a shape refusal that names the character it refused.

**`Handle` is a type, and `TaskName`'s `Display` renders through it.** Principle
3 asks that one type own a name end to end and that the handle be a projection of
the same name. A `Handle { slug, key }` with `parse`, `Display`, and
`Handle::of(&TaskName)` closes the six hand-rolled implementations — but the
strong form is structural rather than disciplinary: the leaf and node renderings
both **end** in the handle's own rendering, so there is exactly one place the
`<slug>-k<key>` grammar is spelled and drift between the two is not expressible.
That is also why the `--` grammar is worth its rename: it leaves the handle a
contiguous terminal substring of every name that has one.

**Grove names a kind only where grove writes the leaf — two tokens, and no
manifest.** `minimalism-k1`'s decision 5 answered five kind-matching sites with a
kind manifest; its decision 7 then removed the manifest outright (*"No manifest,
no methodology location, no reference-path convention, no `match`, no
registry"*). The later decision governs, and the five sites resolve without one:

| site | under decision 7 |
|---|---|
| `reference_file`, 19 → 10 | dies; the prompt names `grove-<kind>` |
| `ending_file`, 19 → 2 | dies; one driver-authored signalling sentence for every kind |
| `finish` sorts last in `pick` | grove's own token — grove mints that leaf |
| `finish` reserved from the grow verbs; `requirements` is root-init's default | grove's own two tokens, same warrant |
| config completeness derived from `Kind::ALL` | becomes per-kind, just-in-time |

The rule that licenses the two tokens is decision 4's loop contract restated:
**the loop mutates the tree only where no session exists to delegate to, and the
two leaves it writes there are the only kinds it may name.** Root scaffolding
mints a `requirements` leaf; the finish sentinel mints a `finish` leaf. Every
other kind is an opaque string. `root-init --kind` overrides the first, so it is
a default rather than a constant.

**Configuration completeness becomes per-kind and just-in-time, and this is a
second correction to `minimalism-k1`.** That leaf predicted
`complete-session-configuration`'s quantifier would move from *all nineteen* to
*every kind the methodology declares* — but that restatement needs the manifest
decision 7 deleted, and grove has no way to enumerate the methodology's set: it
writes no skill directory and holds no registry, so it cannot list what is
installed. The honest restatement is **the kind being used, at the moment it is
used**: before writing a leaf of kind K, and before launching kind K, K must
resolve to exactly one complete template read whole out of one file.

The record's load-bearing property survives intact — nothing is merged within a
kind, one author per launch, and a delta that omits a kind cannot supply it.
What is lost is only the *early* warning: a stale personal config now fails at
the `leaf-add` that first writes that kind rather than at the next tree mutation
of any kind. That is the cost decision 7 already priced (*"a typo'd kind fails at
`leaf-add`, one step later than a `match` arm and well before a session
exists"*), and it buys back a real thing: adding a kind no longer wedges every
operation in every stale config until each user edits their file.

**Grove publishes its version in the prompt, not through a verb.** Decision 6
inverts the compatibility check — grove states what it is, the skill decides —
and leaves the surface open. The prompt already carries the handle and the stated
VCS as bare values, and one more value line costs nothing, needs no command to
succeed, and cannot fail. A verb would need `grove-llm` on `PATH` and would fire
only if the session thought to run it, which is the deferred read the wording
micro-test measured. The value is the workspace's single release version
(decision 2), which orders and means something to a human where the content hash
did neither. `grove-llm --version` remains as clap's free fallback, not as the
mechanism. `methodology::identity()`, `--content-hash` and build pairing all go.

**A kind's skill carries its own family's procedure inline; the shared spine is
one directed second load.** Decision 7 leaves the prompt naming one skill, so
whatever is *not* in `grove-<kind>` is reached by a hop the wording micro-test
never measured. The fatness rule that follows:

- **inline** in `grove-<kind>/SKILL.md`: every rule owned by that kind or its
  family — its goal, deliverable, HITL/AFK mark, review allowance, and whether it
  passes `--done`;
- **in the shared `grove` spine**: every rule shared across families — the seven
  constraints, bootstrap, execute, decompose, retire, commit, and the four
  `*-FORMAT` files;
- **nowhere twice.** `corpus-rules-have-one-owner` and
  `restatement-declares-its-class` bind unchanged and are what make this
  checkable.

Where a rule belongs to a *family* rather than one kind — the five reviews, the
two researches — the family's text is one file in the spine, and each member's
skill directs a load of it **by name** in its opening imperative. A directed load
is not a selection, which is the property `wording-micro-test.md` found
load-bearing.

**The gap is recorded rather than claimed closed.** The micro-test measured one
hop, from a prompt naming two targets. Nothing measures the second hop, from
`grove-<kind>` to the spine. Recorded the way that document records its own
gaps: what is inline is unaffected, what is in the spine loses its guarantee, and
the reopen condition is a session observed acting without a spine rule.

**The plugin's conformance runner is a dependency-free bash script beside
`plugins/install.test.sh`.** The instrument has to leave `cargo test` —
`behavioural-coverage-asserts-delivery`'s walk names `src/prompt.rs`'s guaranteed
core and `content/SKILL.md`, and neither exists after this design — and the
plugins context already tests itself exactly this way, isolated `HOME` and all.
It asserts three things over the shipped skill set: every behavioural rule is
present on the composed loaded path of every kind that binds it; no rule has two
owners; and every file a skill names by path exists. It asserts nothing about
*how many* kinds there are, because under decision 7 a kind exists iff its skill
does.

**The human agreed the shape**, taking the recommendation on all four calls put
to them: the `--` separator, document-eager/kind-just-in-time configuration
validation, family-inline skills over a directed spine, and
`keyed-launch` / `jj-workspace` / `grove-loop`.

One refinement came out of the configuration answer and is folded into the spec:
**eager document validation survives in full.** The whole configuration is still
validated before every tree mutation and again before every launch — syntax,
duplicates, node shape, every template rule — so a malformed entry for a kind
this iteration will not reach still fails before anything is spawned. Only
*presence* becomes just-in-time. That is a strictly smaller amendment to
`complete-session-configuration` than the one first drafted, and it keeps most of
the early warning the record exists for: what is lost is the warning for a kind
**not yet reached**, and nothing else.

**The next leaf is a `planning` leaf, not a stack of impl leaves.** A `design`
session that cuts implementation leaves has drifted into planning's job. The
sequencing this design already knows, and the planning leaf's charter, are in
that leaf's body: the wide-refactor **expand → migrate → contract** rule applies,
and two orderings are forced rather than chosen — the store's new operations
before the second lock layer can go, and the grammar rename inseparable from the
reinstall that parses it.

**A `review-design` leaf is cut ahead of it.** The artifact is load-bearing by
`references/decompose.md`'s test — a landed spec a chain of leaves builds against
for months — and this session had to correct its own input three times and invent
a grammar change the requirements did not foresee. Human sign-off agreed the
*shape*; it is not the adversarial read that asks whether the interfaces are
implementable and whether the decisions contradict each other.
