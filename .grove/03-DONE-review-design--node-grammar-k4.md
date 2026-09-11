# node-grammar-k4

## Goal

Adversarially review the node grammar design as a contract for both Grove and
`ordinal-fs-tree`, before `node-grammar-k3` cuts implementation work.

**Reviews:** node-grammar-k2

## Context

- Requirements: `plan-k1` and the root brief. The interview is complete; do not
  re-interview. The design's commit is found by its stable handle.
- The artifact is the producer's diff: the canonical naming and seam ADRs,
  iteration and root-lifecycle reasoning, both architectures, the design specs,
  both glossaries and context map, the structural and operational models and
  the Quint runner. `docs/formalism-findings.md` entry 049 records the modelling
  evidence and limits.
- No file under `crates/` is part of this design change. Review the proposed
  contract, using source to test whether the handoff accounts for its consumers.

## Review questions

- Can a consumer implement the name-and-bytes input and
  `validate_distinguished(root-or-node, names)` through one name seam, with
  useful errors for missing, competing and misplaced files? Does the library's
  independent at-most-one check preserve its obligation even for a permissive
  consumer?
- Is every way to create or change a level covered before effects, including
  append, batches, insert, initialization entries, promotion's optional first
  child and rewritten node parts? Are whole-tree snapshot validation, interrupted
  promotion and rollback consequences stated consistently?
- Does comparing canonical renderings for distinguished-name identity preserve
  the name laws without teaching the library labels? Can Grove form a node
  handle solely from the actual parsed directory and node-file names, without
  duplicating the slug into node parts or reading content?
- Do the models exercise acceptance as well as refusal, distinguish different
  distinguished filenames, and avoid passing claims only by assuming their
  conclusions? Check the runner's complete claim inventory and the stated limits
  of random simulation and of the root-versus-node policy instances.
- Is the ADR/spec set coherent and current, with no naming history, stale handle
  substring claim, optional Grove node file, changed glossary anchor or design
  requirement that planning cannot consume?

## Done when

- Findings are inspection-only, tied to the producer's commit and current
  artifact coordinates, with their contract and practical consequence stated.
- If actionable findings exist, insert an `integrate-review-design` leaf using
  the bare slug `node-grammar` ahead of `node-grammar-k3`. Its body points to this
  review's handle, rather than adopting the findings as its charter. If there
  are no actionable findings, create no integration leaf.
- Planning still follows the review and any integration it earns.

## Findings

Reviewed: commit `usotvzzt` (`b795f2e3`), *node-grammar-k2: define node files
and level validation*, read against the working tree at that commit (the
working copy was clean on top of it). Coordinates are `path:line` in that tree.
Inspection only: no runner, build or check was executed; every model claim
below is read off the model text and the runners' claim tables. Ordered by
severity; each names the contract it fails and the practical consequence.

### F1 — Three Alloy checks and one Quint invariant are true by construction, and are cited as evidence (actionable, medium)

`docs/ordinal-fs-tree/models/structure.als:463-470` defines `ReaderAccepts`
with three conjuncts over every descended level: `lone distinguishedAt[d]`,
`d in required implies one distinguishedAt[d]`, and
`distinguishedAt[d].nm in allowed[d]`. The three new checks —
`DistinguishedIsUniquePerNode` (472-476),
`RequiredLevelsHaveExactlyOneDistinguishedChild` (478-482) and
`AcceptedDistinguishedNameFitsItsLevel` (484-488) — each have the form
`CorrectedTraitLaws and ReaderAccepts implies X` where `X` is, verbatim, one of
those three conjuncts. They cannot find a counterexample under any trait laws
and any policy: they pass for no reason, with satisfiable laws.

`docs/ordinal-fs-tree/models/operations.qnt:998-1001` `inv_invalidLevelIsAtomic`
asserts `fs.objects == op.before` at rest after `HaltedInvalidLevel` or
`RefusedInvalidDistinguished`. Both are `Refuse` decisions, and `beginOp`
(688-701) sets `fs' = fs` and `before = fs.objects` for every `Refuse`; no
action moves `fs` while `op` keeps that outcome. Unlike `inv_atomicity`, which
checks a modelled unwind, this one checks the transition relation's own shape.

Contract: the models' stated discipline — `structure.als:11-27` (a `check`
"must find NO counterexample" and is a property of the design) and
`docs/ordinal-fs-tree/ARCHITECTURE.md:1116-1130`, whose vacuity guards exist
because "a `check` of the form *laws imply property* passes for no reason if
the laws are unsatisfiable". Q4 of this review asks exactly this.
`docs/ordinal-fs-tree/ARCHITECTURE.md:1076-1080` cites the two Alloy checks and
`inv_invalidLevelIsAtomic` as stating the cardinality-and-placement obligation;
`docs/formalism-findings.md:10535` cites `inv_invalidLevelIsAtomic` as covering
"snapshot and preflight refusals"; the commit message and `node-grammar-k2`
decision 4 count "all 28 Alloy commands" as verification.

Consequence: the structural evidence that exists is the witnesses —
`witness_two_distinguished_children` (598-603), the two refusal witnesses
(605-619) and the positive control `witness_per_node_names_are_accepted`
(623-634). Three of the 28 Alloy commands and one of the fifteen always-on Quint
invariants check nothing, and the prose overstates what was checked. Suggested
shape: state in the model that `ReaderAccepts` is the reader's *definition*;
drop the three checks, or turn each into a `run` that shows the definition is
satisfiable with that conjunct doing work; drop `inv_invalidLevelIsAtomic` or
mark it definitional; correct the three citations.

### F2 — The `required` instance's standing claim inventory does not witness most of the constructors the prose says it covers (actionable, medium)

`docs/ordinal-fs-tree/ARCHITECTURE.md:346-352` claims projected-level validation
"covers every level created or changed: a node appended without content, nodes
in an append batch, an inserted node, a promoted node and its optional first
child, an initialized root and its initial entries, and a rewritten node".
`docs/formalism-findings.md:10531-10541` says Quint "carries this under
`inv_successHasValidLevels`, with `wit_refusedBareNode` and explicit
supplied-name witnesses".

`docs/ordinal-fs-tree/models/run-quint.sh:73-83` (`required`) claims six
witnesses: initialize succeeded and with-child, promote uses first and second
name, `wit_refusedBareNode`, `wit_refusedInvalidDistinguishedOnInitialize`.
`wit_refusedBareNode` (`operations.qnt:1034-1036`) matches `TagAppend` only. So
in the one instance whose policy requires a file and distinguishes root from
node:

- no witness that `insert` or `append_many` with node parts is refused (the
  `append_many` refusal exists only as the one-off seeded control recorded in
  entry 049, not as a standing claim);
- no witness that `promote` with a misplaced name (`Dist(0)`, the root marker,
  at a node) or with a positioned name supplied as `dist` is refused —
  `wit_refusedInvalidDistinguished` is claimed only in `no_distinguished`
  (run-quint.sh:120), where every name is refused and the case cannot be told
  from *no policy at all*;
- no success witness for `append`, `append_many`, `insert`, `rewrite` or
  promote-with-child (`wit_succeeded`, `wit_appendManySucceeded` and
  `wit_promoteWithChild` are claimed only in `pristine` and `failures`).
  `inv_successHasValidLevels` (995-996) is `not(succeeded) or …`, so it says
  nothing about a constructor never observed succeeding in that instance.

Contract: the runner's own header (run-quint.sh:9-13) — "the table is the
model's own account of what each instance shows" — and Q4's "check the runner's
complete claim inventory". Consequence: *every constructor is covered* rests on
one seeded control and an invariant that is vacuous for the constructors not
witnessed. Suggested: add to `required` the standing success witnesses, a
refusal witness per constructor (`insert` and `append_many` with node parts),
and a promote-side misplaced-or-wrong-species witness (for example `TagPromote`
with `dist == Dist(0)` refused).

### F3 — The design is silent on the ` — brief` header rewrite that `leaf-decompose` performs outside the store (actionable, medium)

`docs/specs/module-decomposition.md:241-246`: "`leaf-decompose` passes … `_<slug>.md`
… to `promote`. … its bytes move verbatim to that file. … All of these writes
use the library's exclusive guard and plan; none writes a node file outside
that operation."

Today `crates/grove-loop/src/tree_lifecycle.rs:587` calls
`append_brief_suffix_in_file` after `promote` returns, rewriting the moved
file's first line to `# <slug>-k<key> — brief` (1045-1063). That suffix is what
the spine's `TASK-FORMAT.md` specifies for a decomposed leaf, it is the shape
this tree's own root file carries, and it is the test the book-structure spec
names (`docs/specs/grove-loop-book-structure.md:444`,
`decompose_seeds_brief_from_leaf_body_and_appends_brief_suffix`). The design
neither keeps the rewrite (then §4's last sentence is false as written: the
store moves bytes verbatim and the rewrite is a Grove-side write to a node file
after the operation) nor drops it (then the format rule, `CONTEXT.md:964`
"Headings repeat names for readers", the book-structure spec and the migration
script's expectations all change, and planning has to cut that work).

Contract: SPEC-FORMAT — a spec is what planning decomposes; Q5's "design
requirement that planning cannot consume". Consequence: the verbs leaf and the
books leaf will each guess, and the guesses can differ. Suggested: one sentence
in §4 saying which; if the rewrite stays, restate the claim as "no writer
*creates* a node file outside that operation".

### F4 — The interrupted-promotion signature is now refused as a malformed level, and that refusal's recovery advice is the wrong repair (actionable, medium)

`docs/ARCHITECTURE.md:590-598`: an interrupted promotion leaves `NN-k<key>/`
with no file beside `NN-<kind>--<slug>-k<key>.md`; "The next snapshot refuses
that node as a malformed level before handle lookup can inspect the duplicate
key. The error preserves the directory path and required node-file form".
`docs/ordinal-fs-tree/ARCHITECTURE.md:776-789` still gives the recovery as
"either half can be removed", and 1072-1076 says "failed rollback or process
death can leave a level the next reader refuses".

Today `crates/grove-loop/src/task_tree.rs:446-460` and `497-508` recognise
exactly this shape and print the library's own recovery, because "the process
that meets it is never the process that caused it" and the generic twin advice,
*give one of them a fresh key*, "is actively wrong here". Under the design that
recognition is unreachable: the level refusal fires at open, before any verb
runs, and `validate_distinguished(node: Option<&Self>, children: &[Self])`
cannot see the parent level's same-key sibling. The message an operator now
meets says only that `NN-k<key>/` needs `_<slug>.md`. Following it creates the
file by hand, which makes the twin permanent, and the next command's
`addressable_key` then gives the fresh-key advice the code comment calls wrong.

Contract: `docs/adr/task-names-are-canonical.md:37-38` — "a refusal must
therefore provide the spelling needed to recover" — and Q2's "interrupted
promotion and rollback consequences stated consistently". Consequence: a
diagnostic regression on the one path by which the library damages a tree, not
recorded as a trade-off. Suggested: either say the loss is accepted and why, or
specify a Grove-side wrapping of the store's level error — the same
listing-under-the-guard move `docs/ARCHITECTURE.md:970-971` already uses for
foreign names — that, for a node with no file, checks its parent listing for a
same-key sibling leaf and appends the interrupted-decompose advice.

### F5 — Two of the three root classifications in `a-witnessless-root-refuses-what-it-cannot-account-for` are no longer reachable as written (actionable, low)

`docs/adr/a-witnessless-root-refuses-what-it-cannot-account-for.md:6` —
Taskless: "nothing but `_BRIEF.md`, **or an empty directory**"; `:11` —
Unrecognised: "foreign names and no Grove work". `CONTEXT.md:1050` repeats "or
nothing at all". But the design requires `_BRIEF.md` at every root and
validates every reachable level at open
(`docs/ordinal-fs-tree/ARCHITECTURE.md:672-677`: "An empty directory is a tree
… a domain requiring a distinguished child refuses it";
`docs/specs/module-decomposition.md:193-198`), and classification runs on an
already-opened guard (`crates/grove-loop/src/tree_lifecycle.rs:444-457`:
"Classify a root grove has already opened"; `docs/ARCHITECTURE.md:970-971`: the
listing is read "under the store's guard"). So an empty root, and a root holding
only foreign names, are refused by the reader as *missing `_BRIEF.md`* before
either classification can be made; only a root that already has `_BRIEF.md`
reaches Taskless or Unrecognised. The ADR's own lines 23-24 cover "only
`_BRIEF.md`" and "work and no `_BRIEF.md`" and are silent on "no work and no
`_BRIEF.md`", which is exactly the two shapes steps 1 and 3 claim.

Consequence: the ADR describes an ordered test the design cannot perform for two
of its named inputs, and lifecycle fixtures written from it will expect a
Taskless or Unrecognised message and get a level refusal. Suggested: drop "or an
empty directory" from step 1, condition step 3 on `_BRIEF.md` being present, and
add "a root with neither work nor `_BRIEF.md` is refused by the reader as a
malformed level before classification"; mirror in `CONTEXT.md:1048-1057`.

### F6 — The grammar block is restated byte-for-byte in the ADR and the spec (actionable, low; a pre-existing shape this change extended)

`docs/adr/task-names-are-canonical.md:8-11` and
`docs/specs/module-decomposition.md:183-186` carry the identical four-line
grammar block, and the paragraphs after each (ADR 13-18, spec 188-200) both
state the exactly-one-node-file rule and the root-versus-positioned placement
rule. Before this commit both files carried the same two-line block, so the
duplication is not new, but this commit doubled it. SPEC-FORMAT's grain rule: "A
spec cites the ADRs in its area and never restates them — restate one and the
two sets will disagree, after which neither binds." Suggested: §3 keeps the
sentence about what the grammar separates and the reader's consequence, and
cites the ADR for the block.

### Checked and found sound

Recorded so the integrator knows the review's scope, not as findings.

- **Naming history.** No old spelling or rename narrative in the ADR set, both
  glossaries, the collision table, the library architecture, the models or the
  specs (ripgrep; positive control on `_BRIEF.md`; cross-tree control still
  finds `BRIEF.md` in the plugin, the books and `.grove/`). `docs/USAGE.md`
  still spells the old grammar and is outside this leaf's charter.
- **Anchors.** Every `<a id>` present before the commit is present after it
  (`CONTEXT.md` 48→49, `docs/ARCHITECTURE.md` 24→24; a control mutated to an
  absent id returns nothing).
- **Citations.** No dangling ADR or spec link in `CONTEXT*.md`, `docs/adr`,
  `docs/specs`, `docs/ARCHITECTURE.md` or the library docs. Two pre-existing
  broken relative links in `docs/walkthroughs/grove-loop/13-outcomes.md` are the
  book's and untouched.
- **Q1.** Name-and-bytes for initialization, a supplied name for promotion and
  `validate_distinguished(root-or-node, complete set)` are all on `EntryName`;
  missing, competing and misplaced are each decidable from the inputs the method
  receives; the reader attaches the level path; the library's unconditional
  at-most-one check is stated in both models (`levelValid`'s `ds.size() <= 1`,
  `ReaderAccepts`'s `lone`) and both architectures.
- **Q2.** Whole-tree validation before selection is stated consistently in the
  library architecture, the spec, grove's architecture, the glossary and
  `levelsValid` over `visited`.
- **Q3.** Distinguished identity by canonical rendering follows from canonicity
  (one filename, one parse) and teaches the library no label; the handle is
  composed from the directory key and the node-file slug by two explicit
  constructors, the tree module supplying the pairing; nothing reads content.
- **Q4.** `wit_promoteUsesFirstName` / `SecondName` and
  `witness_per_node_names_are_accepted` exercise acceptance of two different
  node-file names; entry 049 states the random-sampling limit and the
  root-versus-node policy limit.
- **Q5.** The handle-substring claim is corrected in the spec (§4) and in
  `docs/ARCHITECTURE.md:482-483`; no design surface still calls the node file
  optional; the spec gained a requirement with three scenarios and a test-seams
  paragraph planning can cut leaves from; four ADRs reworked in place, none
  superseded, none dangling.

## Decisions (running log)

**1. Six findings, five of them actionable enough to earn integration.**
F1–F5 change what the models claim or what planning would cut; F6 is a format
defect. `node-grammar-k5` (`integrate-review-design`) is inserted ahead of
`node-grammar-k3`, targeting the planning leaf as the first later sibling with
live work; its body names this review's handle and not its findings.

**2. Inspection only, no in-session reviewer.** A review spends none; nothing
was run, and every model claim above is read off the text.
