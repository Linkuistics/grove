# walkthrough-k24

**Integrates:** walkthrough-k23

## Goal

Apply the `walkthrough-k23` planning-review findings to the committed
decomposition, so the research, design, validation, authoring and skill lanes
start from a tree with no contradicted sequence, no leaf gated on a later
sibling, and no unowned repo-level obligation.

## Context

- The reviewed artifact is the `walkthrough-k2` commit (jj change
  `ttmnnrlrutkv`, commit `62661ec0`): the root `BRIEF.md` decomposition, the
  three node briefs, and the fourteen leaves it created.
- Findings below are stated against file paths as they stood at that commit.
  Root-level positions shifted by one when this leaf was inserted (`04`→`05`
  onward); handles did not move, so resolve every reference by `<slug>-k<key>`.
- This is a task-tree edit. It changes `.grove/` only: no book, validator,
  skill or crate source is written here.
- Findings F1–F3 are structural and must be applied. F4–F11 are ordered by
  severity; each names a concrete repair, and a repair judged wrong should be
  answered in this leaf's running log rather than silently dropped.

## Findings

### F1 — High — the required complete-operation tour is sequenced out of existence

**Location.** `.grove/09-ordinal-fs-tree-book-k10/BRIEF.md`, `## Decomposition`
and its closing sentence; against `.grove/07-design-book-system-k6.md`,
`Done when` bullet 4.

**Violated requirement.** `plan-k1`, settled decision 3: the exposition "starts
from purpose and public behavior, **follows a complete operation through the
system**, and then expands the participating layers and the CLI."

**Defect.** `book-system-k6` is required to map "a complete operation through
CLI, public guard, algebra, plan, interpreter, report, and error/refusal
surfaces **before expanding each layer**." The node's fixed slice sequence does
the opposite: `name-seam-k12`, `reference-domain-k13`, `read-path-k14`,
`mutation-algebra-k15` and `filesystem-interpreter-k16` expand every layer
first, and the only end-to-end trace in the tree is `syllabus-cli-k17`'s ("One
command is traced from clap dispatch through reference parts, guard, snapshot,
algebra, plan, interpreter, report, stdout/stderr, and exit status") — the
seventh of eight slices. The node brief then forbids the escape: `book-system-k6`
"may not weaken this sequence." Whichever way the design leaf resolves this it
violates one of its two instructions, and the requirement loses by default
because the sequence is the enforced half.

**Repair.** Give the tour to the opening slice and keep the resolved trace at
the CLI:
- Add to `orientation-k11`'s `Done when`: one complete operation is followed
  end to end at low resolution — CLI invocation, public guard, snapshot,
  decision, plan, interpreter, report, exit — naming each layer the later
  slices expand and owning no fragments beyond its assigned manifest and
  crate-root scope.
- Amend node-09's `## Decomposition` closing sentence so `book-system-k6` may
  place that tour (as part of `orientation-k11` or as one additional slice
  between `orientation-k11` and `name-seam-k12`), while the prohibition on
  weakening the *layer* order stands.
- Leave `syllabus-cli-k17`'s fully-resolved trace as the closing pass, and say
  in both leaves that the two traces are the same operation at two resolutions.

### F2 — High — two validator leaves are gated on a scaffold a later sibling creates

**Location.** `.grove/08-book-validation-k7/01-impl-fragment-validation-k8.md`,
`Done when` bullet 5 ("The real book scaffold passes the scoped check available
at this stage"); `.grove/08-book-validation-k7/02-impl-markdown-validation-k9.md`,
`Done when` bullet 5 ("The book scaffold passes, tests pass"); against
`.grove/09-ordinal-fs-tree-book-k10/01-impl-orientation-k11.md`, Goal ("Create
the book scaffold…") and `Done when` bullet 1.

**Violated requirement.** `walkthrough-k2`, `Done when` bullet 2 — "Each leaf
fits one focused session and delivers a verifiable vertical slice"; and the
independence test in `references/decompose.md` ("can this leaf's work be demoed
or verified on its own, without waiting on a sibling?").

**Defect.** `book-validation-k7` precedes `ordinal-fs-tree-book-k10` in the pick
walk. At `fragment-validation-k8` and `markdown-validation-k9` there is no book
and no scaffold, because creating it is `orientation-k11`'s stated goal. Both
clauses are therefore unsatisfiable as written, and the likely field resolution
is worse than the defect: a validator session invents a scaffold to satisfy its
own `Done when`, and collides with `orientation-k11`'s ownership of the
index/navigation contract and with the design ledger that has not yet been
applied to real content.

**Repair.**
- In both leaves, replace "the real book scaffold" with the design's committed
  example/fixture book. `fragment-validation-k8` already requires success and
  failure fixtures; make the fixture corpus the sole subject of both leaves'
  passing claims.
- Add to `orientation-k11`'s `Done when` that it is the first leaf to run both
  validators against real book content, and that any validator defect it
  surfaces is externalised as a leaf rather than fixed inline.

### F3 — High — scoped validation is undefined for a source file split across two slices, and two of the fifteen are split

**Location.** `.grove/08-book-validation-k7/01-impl-fragment-validation-k8.md`,
`Done when` bullets 1–3; `.grove/07-design-book-system-k6.md`, `Done when`
bullets 2, 3 and 7; `.grove/09-.../04-impl-read-path-k14.md`, Context;
`.grove/09-.../01-impl-orientation-k11.md` and `.../07-impl-syllabus-cli-k17.md`,
Context.

**Violated requirement.** Root brief — "recursively expanding the root fragments
reproduces every in-scope source file exactly"; and `walkthrough-k2`, `Done
when` bullet 2 (each leaf verifiable on its own).

**Defect.** Two of the fifteen in-scope files are split across slices, verified
against the crate:
- `crates/ordinal-fs-tree/src/fs/mod.rs` (393 lines) — `read-path-k14` takes
  "the read side", `filesystem-interpreter-k16` takes the rest.
- `crates/ordinal-fs-tree/Cargo.toml` (116 lines) — `orientation-k11` takes the
  manifest, `syllabus-cli-k17` takes "the manifest's CLI feature and binary
  declaration". These blocks interleave in the real file: `[[bin]]` sits between
  `[features]` and `[dev-dependencies]`, so the split is genuinely
  non-contiguous, not a tail.

`fragment-validation-k8` requires that "recursive expansion of every declared
source root can be compared byte for byte with its repository source file", and
separately lists "unresolved references" among the failures its tests must
demonstrate. A slice that owns part of a file necessarily leaves a hole, and as
specified that hole is indistinguishable from a genuine unresolved reference.
Nothing in `book-system-k6` or `fragment-validation-k8` says how a
deferred-to-a-later-slice hole is declared, represented, or treated in scoped
mode. `orientation-k11` and `read-path-k14` cannot pass their own scoped checks
until this is settled, and it will be discovered mid-authoring.

**Repair.**
- Add to `book-system-k6`'s `Done when`: the ownership ledger declares, for
  every source file whose fragments are split, each deferred hole and the slice
  that fills it; the fragment grammar carries a distinguishable deferred
  reference.
- Add to `fragment-validation-k8`'s `Done when`: scoped mode accepts a hole
  declared deferred to a named later slice and reports it as deferred rather
  than as a diagnostic; exhaustive mode requires zero deferred holes; and the
  failure fixtures cover a hole deferred to a slice that never fills it.

### F4 — Medium — `Cargo.toml`'s split ownership is asserted twice and declared nowhere

**Location.** `.grove/09-.../01-impl-orientation-k11.md`, Context and `Done when`
bullets 3–4; against `.grove/09-.../07-impl-syllabus-cli-k17.md`, Context.

**Defect.** `orientation-k11` says the manifest is explained "including
feature/dependency boundaries and the library/CLI separation" and that every
fragment it claims "appears once"; `syllabus-cli-k17` claims "the manifest's CLI
feature and binary declaration". Both assert manifest coverage and neither
defers to the other. The asymmetry is visible one leaf away: `read-path-k14`
flags its own split explicitly ("the **read side** of `src/fs/mod.rs`"), so the
manifest split is the one that reads as whole-file ownership. Under the
fail-closed duplicate-source rule this surfaces as a validator failure in
whichever of the two runs second.

**Repair.** Mirror `read-path-k14`'s wording in `orientation-k11`: the manifest
**excluding** the `cli` feature and the `[[bin]]` declaration, which
`syllabus-cli-k17` owns per the ledger. Keep the library/CLI *separation* as
`orientation-k11`'s conceptual subject while the CLI-specific manifest fragments
stay `syllabus-cli-k17`'s.

### F5 — Medium — the editorial review's kind, stem and `**Reviews:**` target are unspecified

**Location.** `.grove/09-ordinal-fs-tree-book-k10/BRIEF.md`, `## Notes`.

**Defect.** The brief is fully specific for the technical read — via
`book-assembly-k18`, kind `review-impl`, bare stem `book-assembly`,
`**Reviews:** book-assembly-k18` — and then degrades to "the editorial reviewer"
for the second read, which is the step where the convention is least obvious:
it is commissioned by an `integrate-review-impl` session rather than by a
producer, and `TASK-FORMAT.md`'s closed set has no editorial kind, so it is
necessarily another `review-impl` distinguished only by its body.
`references/decompose.md` warns about exactly this: a mis-kinded review "is a
perfectly valid invocation" that buys the wrong read, "and nothing downstream
detects the mismatch."

The `**Reviews:**` target is undefined too, and both available answers are wrong
alone: naming `book-assembly-k18` points the editorial reviewer at a diff that
the technical integration has since modified, while naming the integration
handle points it at the fixes only, not at the book.

**Repair.** State in node-09's `## Notes`:
- both reviews are kind `review-impl` and both integrations
  `integrate-review-impl`, all four carrying the bare stem `book-assembly`, so
  each step is referenced by its `<slug>-k<key>` handle and never by stem alone;
- the editorial leaf's `**Reviews:**` names whichever session last wrote the
  book — `book-assembly-k18`, or the technical integration if one ran;
- the editorial body scopes the read to the **whole book** rather than to that
  handle's diff, and says so explicitly, since that is the one place the
  handle-plus-diff default in `references/decompose.md` under-serves the read.

### F6 — Medium — four consecutive authoring slices carry ~6× the first slice's source, with no decomposition relief

**Location.** `.grove/09-ordinal-fs-tree-book-k10/BRIEF.md`, `## Decomposition`;
leaves `01`–`07`.

**Violated requirement.** `walkthrough-k2`, `Done when` bullet 2 — "Each leaf
fits one focused session".

**Defect.** Measured against the crate at this commit:

| slice | source owned | lines |
|---|---|---|
| `orientation-k11` | `Cargo.toml`, `src/lib.rs` | 210 |
| `name-seam-k12` | `src/name.rs` | 700 |
| `reference-domain-k13` | `src/reference.rs`, `src/conformance.rs` | 1,191 |
| `read-path-k14` | `src/snapshot.rs`, `src/fs/read.rs`, part of `src/fs/mod.rs` | ~950 |
| `mutation-algebra-k15` | `src/ops.rs`, `src/plan.rs`, `src/report.rs` | 1,263 |
| `filesystem-interpreter-k16` | rest of `src/fs/mod.rs`, `src/fs/apply.rs`, `src/fs/lock.rs`, `src/error.rs` | ~1,100 |
| `syllabus-cli-k17` | `bin/syllabus.rs` + CLI manifest fragments | ~1,150 |

Reproducing ~1,200 lines as owned fragments **with** the explanatory prose the
root brief demands, plus scoped validation, is at the edge of one focused
session — and four such leaves run consecutively. The plan gives no relief, so
the predictable failure is four runaway sessions rather than four decompositions.

**Repair.** Add to node-09's brief that `reference-domain-k13`,
`mutation-algebra-k15`, `filesystem-interpreter-k16` and `syllabus-cli-k17` are
expected `leaf-decompose` candidates, and that decomposing at a stated
conceptual seam is the intended response to an oversized slice rather than a
longer session. Add to `book-system-k6`'s `Done when` that the ledger publishes
each slice's owned-source line count, so the seam is visible before authoring
starts rather than discovered inside it.

### F7 — Medium — no leaf owns a source-drift policy, and byte-exactness is the book's central claim

**Location.** `.grove/09-ordinal-fs-tree-book-k10/BRIEF.md` (whole); root
`BRIEF.md`, `## Done when` bullet 4.

**Defect.** The book must reconstruct fifteen files byte for byte, and the
slices land across at least eight sessions in a live repository. Nothing in the
tree freezes the in-scope source for the node's duration, says what happens when
an authoring or reviewing session finds a genuine defect in the crate, or
requires earlier slices to re-tangle after an accepted source change. The
failure mode is silent until `book-assembly-k18`'s exhaustive check, where a
one-line source edit made during `read-path-k14` surfaces as cross-slice rework
in the leaf that can least absorb it.

**Repair.** Add to node-09's brief: the fifteen in-scope files are frozen for
the node's duration; a defect found in crate source is externalised as a leaf
rather than fixed inline; and any accepted source change obliges re-running
exhaustive fragment validation and updating every affected slice before the node
closes. Add the corresponding check to `book-assembly-k18`'s `Done when`.

### F8 — Medium — repo-level registration of both new artifacts is unowned

**Location.** `.grove/10-.../09-impl-book-assembly-k18.md` and
`.grove/10-walkthrough-skill-delivery-k19/02-impl-writing-code-walkthroughs-k21.md`,
`Done when` sections.

**Defect.** Two repository-level indexes enumerate by hand what exists, and no
leaf owns updating either:
- `CONTEXT-MAP.md` names the `ordinal-fs-tree` context's artifacts explicitly
  and states which stay under `docs/ordinal-fs-tree/`. The book will be that
  context's largest artifact.
- `plugins/linkuistics/.claude-plugin/plugin.json` carries a hand-maintained
  `description` enumerating every skill's capability, plus a `keywords` list.
  Adding `writing-code-walkthroughs` requires editing both fields, and
  `plugins/linkuistics/skills/authoring-conventions/SKILL.md` — the leaf's named
  house-rules pointer — does not mention the manifest, so nothing else in the
  route will catch it.

**Repair.** Add the `CONTEXT-MAP.md` and `docs/ordinal-fs-tree/CONTEXT.md`
registration of the book to `book-assembly-k18`'s `Done when`, and the
plugin-manifest `description`/`keywords` update to
`writing-code-walkthroughs-k21`'s.

### F9 — Medium-low — the baseline's contamination controls do not name the in-repo evidence that defeats them

**Location.** `.grove/10-walkthrough-skill-delivery-k19/01-impl-skill-baseline-k20.md`,
Context and `Done when` bullet 2.

**Defect.** The leaf excludes "draft skill instructions or book-specific task
files". By the time it runs, the same checkout also holds the completed book,
`walkthrough-method-k5`'s reconciled synthesis, and node-09's brief — each of
which encodes the method the no-skill control is supposed to lack. Nothing
requires any scenario to target a codebase other than `ordinal-fs-tree`, so the
control can read the finished answer to its own task. The whole deployment claim
in `skill-evaluation-k22` rests on this baseline being uncontaminated.

**Repair.** Name the completed book, the research synthesis and the whole
`.grove/` tree as excluded material; require at least one scenario targeting a
codebase outside this repository; and record, per run, what the evaluated
context could actually read.

### F10 — Low — the two evaluation leaves carry multi-run campaigns with no sizing relief

**Location.** `.grove/10-.../01-impl-skill-baseline-k20.md` and
`.../03-impl-skill-evaluation-k22.md`; node-10 brief, `## Notes`.

**Defect.** At least five fresh-context repetitions per behaviour-shaping case,
over the eight behaviours named in `skill-baseline-k20`, run once as baseline
and again enabled, with refinement reruns in `skill-evaluation-k22`. That is
plausibly forty-plus agent runs per leaf plus rubric authoring and analysis —
the same one-session bar F6 raises, in a lane where the runs cannot be shortened
without invalidating the comparison.

**Repair.** Bound the behaviour-shaping case count in node-10's brief, or state
that `skill-baseline-k20` and `skill-evaluation-k22` are expected
`leaf-decompose` candidates split by scenario group, with the rubric committed
once by the first child and shared by the rest.

### F11 — Low — types are explained several chapters before their source is owned, with no stated rule

**Location.** `.grove/07-design-book-system-k6.md`, `Done when` bullet 7;
`.grove/09-.../04-impl-read-path-k14.md`, `Done when` bullet 4.

**Defect.** `src/error.rs` is owned by `filesystem-interpreter-k16`, yet
`orientation-k11`, `name-seam-k12`, `reference-domain-k13` and `read-path-k14`
all describe error or refusal behaviour; `Refusal` lives in `src/plan.rs`, owned
by `mutation-algebra-k15`, while `read-path-k14` covers "error/refusal
distinctions encountered on reads". `book-system-k6` authorises explaining a
file out of file order but says nothing about a *type* used chapters before its
definition appears — which is precisely the split-attention case the root brief's
repetition rule exists to settle, left to seven authors to settle severally.

**Repair.** Add to `book-system-k6`'s `Done when`: for each type first used
before its source is owned, the ledger records the minimum local restatement the
earlier chapter must carry and the later chapter that owns the definition. That
makes the repetition-versus-cross-reference rule checkable at review time
instead of a per-author judgement.

## Checked and clean

Stated explicitly so the integration does not re-derive them:

- **Source corpus is exactly right.** The fifteen files in node-09's brief are
  precisely the production non-test files of `crates/ordinal-fs-tree`, verified
  against the tree. The exclusions are complete and correct: `src/fixtures.rs`
  (declared `#[cfg(test)] mod fixtures;` in `src/lib.rs`, so genuinely
  test-only), `src/{ops,plan,snapshot,fs/apply}/tests.rs`, all of `tests/**`,
  and the Alloy/Quint model source. In-scope total is 6,618 lines, matching
  `plan-k1`'s "exceed six thousand lines".
- **The embedded-`#[cfg(test)]` clause is real and small.** Four two-line
  `mod tests;` declarations, plus genuine `#[cfg(test)]` fault-injection hooks
  inside `src/fs/apply.rs` — which `filesystem-interpreter-k16` already
  anticipates by naming fault-injection checks.
- **The research pair is justified and independent.** The root brief argues the
  two-corpora cost rather than assuming it; `walkthrough-method-k4` forbids
  reading `k3`'s output and biases its corpus away from the obvious sources; and
  the vendor axis `references/decompose.md` says to verify before paying for the
  second leaf is actually configured — the effective `.grove.kdl` routes
  `research-a` to Codex and `research-b` to Claude.
- **Dependency order holds at every seam.** Research → design → validators →
  authoring → review chain → skill lane, with `book-system-k6` before
  `book-validation-k7` and both before `ordinal-fs-tree-book-k10`.
- **File assignment is complete.** All fifteen files are assigned across
  `orientation-k11`…`syllabus-cli-k17` with none unassigned; the `src/fs/mod.rs`
  split is declared (the manifest split is F4).
- **Every named proof point has exactly one owner.** Whole-source coverage
  (`fragment-validation-k8`, exercised exhaustively by `book-assembly-k18`);
  Markdown and local links (`markdown-validation-k9`, `book-assembly-k18`);
  crate verification (`book-assembly-k18`); technical accuracy (the `review-impl`
  it commissions); editorial quality (the second review — modulo F5); skill
  behaviour (`skill-evaluation-k22`).
- **Chain adjacency is correct.** The book's review chain is created inside
  node-09 as flat siblings appended after `book-assembly-k18`. Pre-order
  finishes that directory — appended leaves included — before visiting
  `walkthrough-skill-delivery-k19`, so plain `leaf-add` is right for every step
  and no later sibling node intervenes.
- **Skill-lane temporal separation holds.** `skill-baseline-k20` forbids writing
  or scaffolding the skill; `skill-evaluation-k22` mandates identical prompts,
  rubric, sample sizes and controls, and states that a rubric change invalidates
  the comparison and requires rerunning both sides.
- **Every brief pointer resolves.** `docs/ordinal-fs-tree/{CONTEXT,ARCHITECTURE,CLI}.md`,
  `models/{structure.als,operations.qnt}` with both runners,
  `docs/formalism-findings.md`, `plugins/linkuistics/PROVENANCE.md`, and
  `plugins/linkuistics/skills/authoring-conventions/SKILL.md`.

## Done when

- F1, F2 and F3 are applied to the named files.
- Each of F4–F11 is either applied as described, or answered in this leaf's
  running log with the reason it was rejected or reshaped.
- No repair introduces a new leaf; every change is an edit to an existing brief
  or task body. Work that cannot be expressed that way is externalised as its
  own leaf rather than absorbed here.
- The tree still satisfies `walkthrough-k2`'s own `Done when` after the edits,
  and the root brief's `## Decomposition` still describes the tree accurately —
  F1's repair changes what the book slices open with, so check that paragraph.
- No book, validator, skill or crate source is written by this leaf.

## Notes

`walkthrough-k23` was findings-only and ran no build, test, lint or format
command; nothing here rests on executed verification. Line counts in F6 and the
file inventory in "Checked and clean" were read from the working tree at this
commit and are the only measured claims.

The reviewer explicitly did not raise chapter naming, slice titles, or the
choice to explain `src/fs/mod.rs` in two halves: each is a defensible editorial
call that creates no dependency, completeness or single-session defect.

## Decisions (running log)

- F1–F4, F7–F9, and F11 were verified as real contract defects and repaired in
  the named existing briefs and task bodies.
- F5 was a contract stated unclearly. The book-node brief now fixes the kinds,
  shared stem, full-handle references, editorial target, and whole-book scope.
- F6 and F10 are real sizing trade-offs. The affected node briefs now make
  decomposition at explicit seams the intended response and state what shared
  ledger or rubric survives across children.
