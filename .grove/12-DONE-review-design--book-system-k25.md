# book-system-k25

**Reviews:** book-system-k6

## Goal

Adversarially review the committed ordinal-filesystem-tree book-system design
for missing requirements, ambiguous formats, misplaced seams, and validator or
authoring contracts that still require redesign downstream.

## Context

- Review the `book-system-k6` commit, especially
  `docs/specs/ordinal-fs-tree-book.md`, against the producer task, `plan-k1`,
  `walkthrough-method-k5`, the `ordinal-fs-tree-book-k10` brief and its eight
  leaf contracts, and the `book-validation-k7` brief and both validator leaves.
- Inspect the frozen fifteen-file source corpus where a design claim depends on
  its actual bytes or structure. Treat repository docs, tests, ADRs, and models
  as evidence, not as substitutes for a self-contained book contract.
- A first in-session adversarial read caused the producer to tighten the shared
  byte lexer, normative tables, diagnostic records, page ownership, repository
  loading, scope enumeration, and prose-example placement. Re-derive whether
  the committed design is sufficient; do not treat that list as findings to
  preserve or as proof that adjacent gaps are closed.
- This is findings-only. Do not edit the design, source, tests, task tree, or
  other production artifacts, and do not run build, test, lint, or format
  commands.

## Done when

- The review tries to disprove that the raw-Markdown grammar has one
  unambiguous byte interpretation, including directive/fence contexts, exact
  source preservation, recursive expansion, deferral, ownership and
  reachability.
- It mechanically checks that the ownership blocks form gapless,
  non-overlapping partitions of exactly the fifteen source roots and 6,618
  lines, and that scoped states can progress through all seven source-owning
  slices before final assembly requires zero deferrals.
- It tests the page table, conceptual order, low-resolution and full-resolution
  insert tours, navigation, early-use ledger, source/fragment indexes, and prose
  review questions against every book-authoring leaf's contract.
- It tests whether fragment and Markdown validation share one lexical seam,
  whether their CLI, scope, input loading, diagnostics, JSON, and link behavior
  are deterministic, and whether `book-validation-k7` can implement them
  without deciding missing policy.
- Every finding names severity, exact artifact location, violated requirement,
  and a concrete repair. Contract ambiguities are distinguished from accepted
  trade-offs and preferences; absence of findings is stated explicitly.
- If findings warrant changes, an `integrate-review-design` leaf with bare stem
  `book-system` is placed according to Grove's directory-local adjacency rule
  and carries `**Integrates:** book-system-k25`. If no findings exist, no
  integration leaf is created.

## Notes

The review is fresh-context and inspection-only. It reviews the committed
artifact and its present requirements rather than validating the producer's
reasoning or repeating the producer's verification commands.

## Findings

Fifteen findings: two High, eight Medium, five Low. The design artifact is
`docs/specs/ordinal-fs-tree-book.md`; bare `:N` citations are that file.

**What held, checked mechanically.** The ownership ledger is sound. Parsing the
spec's own tables: the fifteen source-root line counts (`:504-520`) match the
fifteen files on disk exactly; the 26 top-level blocks (`:521-546`) form a
gapless, non-overlapping partition of every root, each `Count` equals
`last-first+1`, each root's blocks end exactly at its declared length, the
per-slice sums reproduce the `Owned-source totals` table (`:554-565`) row for
row, and the `State` column is `resolved` exactly for `orientation-k11`'s rows.
The corpus total is **6,929**, not the 6,618 this task's own Context carries —
6,618 is a stale planning-era figure surviving in
`.grove/04-DONE-integrate-review-planning--walkthrough-k24.md:330` and
`.grove/08-DONE-design--book-system-k6.md:100`; the design's figure is the
correct one. Scoped progression is clean: at `--through syllabus-cli-k17`
resolved is 6,929 and deferred 0 while `final` is still false, so the seven
source-owning prefixes reach zero deferrals before final assembly adds page 08,
and the `final` flag — not the deferred count — is what stops a prefix reading
as exhaustive. The corpus also satisfies every lexical precondition the design
asserts (`:255-260`, `:372-374`): all fifteen files are valid UTF-8, LF-only,
LF-terminated, and contain runs of three backticks but no run of four, so the
four-backtick literal fence is genuinely collision-free. The single-parent
tree, no-transforms, and read-only-validation decisions are well argued and are
recorded as trade-offs rather than assumed.

Findings 1-2 are contract defects that leave a decision unmade. Findings 3-10
are ambiguities or stale statements with a named repair. Findings 11-15 are
small and local. None of the fifteen is a preference: each names a rule the
design states, a place the design does not state one, or a statement that is
now false.

**Repair cost is not what it would have been.** This review sits at
`.grove/12`, but `book-validation-k7` and all of `ordinal-fs-tree-book-k10`
are already `DONE`. Every finding below that would have cost one edit to a
design nobody had built against now costs edits across the spec, 92 fragment
definitions, `source-index.md`, and a 7,000-line Rust crate. That is a
consequence of the leaf's position, not of the findings, and it is stated so
the integrating session can weigh repairs against churn rather than discovering
the cost mid-fix.

### 1. The ownership ledger has no stated single source of truth, reinstating the drift the sidecar was rejected to avoid

**Severity: High. Class: contract defect — the design leaves a decision unmade.**

`:471-476` states the precedence for the *graph*: "The directives are the graph
authority. The tables are mandatory derived indexes: the validator recomputes
every relationship and rejects a row that disagrees." But the scoped check
demands more than the directives can supply. `:713` requires "all fifteen source
roots and all top-level ledger rows" and `:715-716` requires "an explicit defer,
with **the ledger's** exact ID, owner, and range, for every later-owned block".
Directives cannot be their own authority for those: a book that simply omits a
block produces a shorter root with nothing to disagree with. Some external table
must say which 26 blocks exist, with which owners and ranges. The design never
says which one, or where it lives.

The implementation answered by copying it into Rust:
`crates/book-validation/src/validator.rs:9-18` (`SLICE_ORDER`), `:20-74`
(`ROOTS` — the fifteen paths *and* their line counts), `:87+` (`BLOCKS` — all 26
rows), consumed by `crates/book-validation/src/ledger.rs:4`. The ledger now
exists in four places with no declared precedence: the spec's tables,
`source-index.md`'s tables, `source-index.md`'s directives, and the Rust
constants.

This is precisely the hazard `:1005-1012` rejects the TOML sidecar to avoid —
"Drift would require choosing which representation to trust, and raw Markdown
would no longer be sufficient to reconstruct the code" — and it is worse than
the sidecar, because the fourth copy is compiled. An accepted source change,
which `:9-13` says requires "the affected ownership ranges and fragments to
change", now also requires a Rust edit, and the design's claim at `:476-478`
that "raw Markdown provides outward relationships in parents and inward lookup
without a second file format" is false of the system as specified.

**Repair.** Name the authority and make the rest derive from it. Either (a) the
spec's `Source roots` and `Top-level ownership blocks` tables are normative and
the validator generates its constants from them (a build script or a checked-in
generated module with a test that regenerates and compares), or (b) the
authority moves into `source-index.md`'s tables and the spec cites rather than
restates them. Either way state the precedence explicitly in *Source and
ownership ledger*, and add a diagnostic for a copy that disagrees.

### 2. "One shared byte-level lexer" is stated over a narrower token set than the rules that consume it, and two more scanners exist

**Severity: High. Class: contract defect — the stated property is not
achievable as written.**

`:264-266`: "Fragment and Markdown validation consume the same token stream from
one shared byte-level lexer. Neither check rescans comments or fences
independently." The lexer's token set is fixed at `:268-345`: reserved directive
lines, ordinary fences, literal fences, and lexical findings, over book files.

Two rules the design also states cannot be answered from that token set.

**Inline code.** `:881` requires local links to be recognised "Outside code
fences **and inline code**". Inline code spans appear nowhere in the lexer
section — not as a token, not as a context, not as an accepted form. The
Markdown checker therefore has to recognise them itself, and does:
`crates/book-validation/src/markdown.rs:506` `exact_code_span_end`, called from
`:432`.

**Non-book files.** `:879` permits links into a "deliberately linked repository
artifact" (see finding 4), and `:874` requires anchor targets to be explicit. An
anchor in a file the shared lexer never ran over needs its fences identified
some other way, so a second fence scanner exists:
`crates/book-validation/src/markdown.rs:524` `repository_fence_ranges`. It does
not agree with the shared lexer: `:530-534` strips up to three leading spaces
before matching a fence delimiter, where the shared lexer requires column zero
(`:300-301`). The same bytes get two different fence interpretations inside one
tool, and which one applies depends on whether the file happens to be a book
file.

**Repair.** Add inline code spans to the shared lexer with an exact accepted
form, so the link rule consumes a token rather than a re-scan. Then close the
non-book path: if finding 4 is repaired by forbidding out-of-book relative
links, `repository_fence_ranges` disappears; if such links stay, state the
lexical rules for a non-book target explicitly and require them to be the shared
lexer's, not CommonMark's.

### 3. The staged `--check` surface is stale, and the spec's two runnable commands are the ones it calls a false success

**Severity: Medium. Class: statement that is now false.**

`:676-679`: "In the current staged surface, `--check` accepts only `fragments`
and defaults to it; rejecting `markdown` and `all` prevents a fragment-only
false success. `markdown-validation-k9` adds `markdown` and `all`, changes the
default to `all`, and updates the two commands above to use `--check all`."

`markdown-validation-k9` is `DONE`
(`.grove/09-book-validation-k7/02-DONE-impl--markdown-validation-k9.md`), and
the tool did exactly what the sentence predicted:
`crates/book-validation/src/cli.rs:46` is
`#[arg(long, value_enum, default_value_t = CheckArg::All)]`, and `:14` documents
both examples with `--check all`. The spec was never updated. It still asserts
that `markdown` and `all` are rejected — false — and its two normative "runnable
author command" blocks at `:662` and `:672` still read `--check fragments`,
which by the spec's own sentence is what "prevents a fragment-only false
success" was guarding against.

This is also a shape defect. `SPEC-FORMAT.md` — "The set is current-state" —
puts work-orders and "the input for the next three leaves" in a node's
`BRIEF.md`, which dies with `.grove/`. A staging plan naming a leaf handle does
not belong in a permanent spec, and this is what happens when one does.

**Repair.** Delete the staging paragraph. Set both command blocks to
`--check all`. State the surface as: `--check` accepts `fragments`, `markdown`
or `all` and defaults to `all`.

### 4. "Deliberately linked repository artifact" is undefined, so the containment check collapses to repository containment

**Severity: Medium. Class: contract defect — an undefined term in a normative
check.**

`:875-880` requires Markdown validation to check "paths remaining inside the
repository and inside the permitted book or deliberately linked repository
artifact." The design never enumerates that set, never defines "deliberately",
and gives it no home: `BookSnapshot` at `:644-646` has exactly two categories,
book files and in-scope source files.

The implementation invented a third: `crates/book-validation/src/lib.rs:18` adds
`linked_files`, populated by `crates/book-validation/src/cli.rs:240-283`, which
scans every book file for local links and loads whatever they resolve to. The
permitted set is therefore *defined by the book being checked*. Any repository
file the book links to is permitted by virtue of being linked, so the only
surviving constraint is repository containment — the check the clause was
written to strengthen.

The mechanism is currently dead: `docs/ordinal-fs-tree/book/*.md` contains zero
`../` relative links. It nonetheless costs a snapshot field, a loader pass, and
the second fence scanner of finding 2.

**Repair.** Decide it. The self-containedness requirement at `:889-893` and in
the root brief argues for the strict answer: relative links leave the book
directory only to the fifteen source paths, and anything else is an M-code
finding. That deletes `linked_files`, `load_linked_files`, and
`repository_fence_ranges` together. If out-of-book links are wanted, enumerate
the permitted paths in the spec.

### 5. The early-use ledger covers types only, but the design's own ownership split creates forward references inside reproduced source bytes

**Severity: Medium. Class: contract defect — a gap in the forward-reference
mechanism.**

`:572` sets the ledger's trigger: "When a page first uses a codebase-specific
**type** whose source belongs to a later slice". The obligation it creates — a
minimum local statement, recorded, with a status the owner later flips — exists
for types and for nothing else.

The manifest split at `:521-524` creates the same hazard in a form the trigger
does not reach. `manifest-package-and-library-dependency` (1-42,
`orientation-k11`) reproduces `crates/ordinal-fs-tree/Cargo.toml:29-42`
verbatim, and those bytes say "`clap` is the CLI's and nobody else's, so it is
optional and behind the `cli` feature **below**" and "**The feature** is on by
default, and that is the load-bearing half." The `[features]` stanza both
comments document is lines 43-45 — `manifest-cli-feature`, deferred to
`syllabus-cli-k17`, page 07. Orientation therefore reproduces nine lines of
commentary whose referent is not on the page, and no ledger row records that.
`manifest-library-cli-boundary` (46-61) has the identical shape against
`[[bin]]` at 62-65.

The obligation is real and was met, but only by author judgment:
`docs/ordinal-fs-tree/book/01-orientation.md:62-64` adds "its feature activation
is a CLI-owned range deferred in the source index." Nothing in the design
required it, and nothing would have caught its absence.

**Repair.** Widen the trigger rather than move the boundary. Moving it (1-33 /
34-45) would strip orientation of the library/consumer commentary at 47-61 that
`:174-176` specifically wants there. Instead: a row is required whenever a page
reproduces bytes that reference a source range owned by a later slice, with the
same five columns and the same status flip. Add the two manifest rows to the
initial ledger at `:585-591`.

### 6. Grove leaf handles are the permanent identifier domain for the spec, the book, and the validator

**Severity: Medium. Class: contract defect — ephemeral keys in durable
artifacts.**

`:288-289` fixes the domain: "`SLICE` is one of the eight enumerated slice IDs
in the page table." Those IDs are `.grove/` task handles — `orientation-k11`
through `book-assembly-k18`. They are the page table's `Slice` column (`:57-73`),
the ownership table's `Owner` column (`:519-546`), the `Owned-source totals`
rows (`:554-565`), the early-use ledger's `Owner` column (`:585-591`), the
`owner=` attribute of every `fragment` and `defer` directive (`:283-291`), and
the seven accepted `--through` values (`:682-687`) — which makes a dead task
handle a user-facing CLI argument.

Count: 67 occurrences in the spec, 225 in `docs/ordinal-fs-tree/book/*.md`, 207
in `crates/book-validation` — 499 across three permanent artifacts, for a
`.grove/` that `grove-finish` tears down.

The failure mode is already visible in this repository. The frozen corpus itself
carries handles from an earlier grove: `crates/ordinal-fs-tree/Cargo.toml:82`
cites `reading-k19`, and `src/name.rs` and `src/ops.rs` cite `reading-k20` and
`promote-k25`. Those resolve to nothing today. The design repeats the pattern at
roughly fifty times the scale, and does so in an artifact whose own membership
test (`SPEC-FORMAT.md`) is "would a session on an unrelated future grove need to
read this?" — yes for the book system, no for the leaf keys.

Note the counter-argument, which is partly good: the spec's page table defines
the IDs, so they remain resolvable as opaque tokens after `.grove/` dies. That
is why this is Medium and not High. It does not survive the CLI case — an
operator running `book-check --through orientation-k11` in 2027 is typing a
handle whose only definition is a table explaining that the suffix means
nothing.

**Repair.** Use the eight page IDs, which are already in the page table, already
one per slice, and already durable: `orientation`, `name-seam`,
`reference-domain`, `read-path`, `mutation-algebra`,
`filesystem-interpreter`, `syllabus-cli`, `invariants-and-trade-offs`. Record
the page-ID-to-grove-leaf mapping in `ordinal-fs-tree-book-k10`'s `BRIEF.md`,
where it dies correctly. Mechanically this is a rename of `owner=`, `slice=`,
`defer owner=`, `--through` values, and three table columns. Given the position
of this leaf (see the preamble), the integrating session may reasonably judge
the churn too large and record the decision instead; that judgment is theirs,
but it should be made rather than inherited.

### 7. A `defer` inside a composite is permitted, contradicts two ownership rules, and is never reconciled

**Severity: Medium. Class: contract defect — three rules give different
answers.**

`:408-412`: "A defer is legal only inside a source root **or composite** … The
named owner owns the range immediately; the earlier slice does not receive
credit for it."

Two other rules say otherwise for the composite case. `:431`: "Every descendant
of a top-level ownership block has the same owner" — a defer to a later slice
inside a composite is a descendant with a different owner, so it is forbidden.
`:442-444`: "Line-count credit is assigned to the top-level block only, so
refinement cannot change slice ownership totals" — which gives the range to the
enclosing block's owner, the opposite of what `:410-412` says.

The ownership ledger only describes blocks that are direct children of a source
root, so a composite-nested defer has no row to be checked against, and is not:
`crates/book-validation/src/validator.rs:536` iterates `root.children` only,
while `check_references` at `:496-530` accepts any defer whose owner is a later
slice, and `crates/book-validation/src/parser.rs:302-311` accepts one in any
active construct. A slice can therefore hand part of its own top-level block to
a later slice with no ledger row and no diagnostic, and the coverage counter at
`validator.rs:1335` will bill it as deferred.

**Repair.** One clause: a `defer` is legal only as a direct child of a source
root. A `defer` anywhere else is `P002`. That makes every defer correspond to a
ledger row by construction and removes the contradiction with `:431` and
`:442-444`.

### 8. The library and CLI seams disagree on an out-of-domain scope, and the design specifies only the CLI

**Severity: Medium. Class: contract defect — an unspecified behaviour at the
seam the design calls primary.**

`:644` declares the core as one deep operation,
`validate(BookSnapshot, Request { scope, checks })` with
`scope = Through(slice-id) | Final`, and gives `slice-id` no stated domain.
`:686-687` states the rule only for the command: passing `book-assembly-k18` to
`--through`, "or passing an unknown slice, is an invocation error with exit
status 2."

The two seams now answer differently. The CLI rejects before calling, giving
exit 2 as specified. The core independently emits a diagnostic —
`crates/book-validation/src/validator.rs:285-300`, code `U001`, phase `parse`,
path `<command>` — but `validate` can only return `Valid` or `Findings`
(`:318-327`), so the library reports the same input as *findings*, exit 1 if
rendered, never as an invocation error. The design's own text reserves `U001`
for "command location" (`:774`) and `<command>` for "Command-wide text failures"
(`:862`), so the component the design says has no command is emitting a command
code at a command location.

This matters because `:648` makes the library the tested surface: "Tests
construct snapshots in memory."

**Repair.** Preferably make the bad case unrepresentable: give the core a typed
scope whose domain is the seven scoped slices plus `Final`, so `Through` cannot
carry an arbitrary string. Otherwise state that the core returns
`invocation-error` status for an out-of-domain scope, and that `U001` may
originate there.

### 9. A mechanically checkable rule sits in the un-checked section with no diagnostic code, and cost two rounds of manual review

**Severity: Medium. Class: misplaced seam.**

`:948-951`: "Every literal fragment's opening directive is immediately preceded
by a prose paragraph: one or more adjacent nonblank lines that are not a
heading, list, table, fence, HTML comment, or fragment directive. No other
nonblank block may intervene."

That is a pure predicate over tokens the shared lexer already produces. It sits
under `:901`, "Mechanical validation deliberately does not claim to prove prose
quality or technical truth", and no code in the `P`/`F`/`M`/`U`/`I` tables
covers it — so it is enforced by human reading. The design's own Notes prefer
"a small explicit format whose exactness can be tested over a flexible notation
that requires human interpretation."

The cost is recorded downstream.
`.grove/10-ordinal-fs-tree-book-k10/10-DONE-review-impl--book-assembly-k34.md:152-175`
is finding 3, "Five literal fragments on page 06 have no preceding prose
introduction", listing five exact line numbers — every one of which a five-line
lexer check would have caught at authoring time, on the slice that introduced
them, instead of at whole-book review two leaves later.

**Repair.** Split the rule. The structural half (a paragraph block immediately
precedes every literal `fragment` opening directive) becomes a Markdown check
with its own code — `M105` — listed with the others at `:895-899`. The five
review questions stay with the editorial reviewer, where judgment belongs.

### 10. `concept-index.md` is a mandatory deliverable with no contract, and the repetition rule depends on it

**Severity: Medium. Class: contract defect — a required artifact with no
specification.**

`concept-index.md` is one of the eleven files (`:46`), has a fixed page identity
(`:68`, `:123`), exists from the orientation slice with content that "grows with
the prefix" (`:74-75`), and is written by every slice — authoring step 6 at
`:620` is "Complete or add early-use rows and concept-index entries." The design
defines no entry schema, no ordering, no coverage criterion, and no completeness
test. The only rule touching it is `:874`, that its links resolve. Compare the
four exactly-schematised tables in `source-index.md` at `:452-500` and
`:568-608`, each with a byte-level row format and an `F009` for a malformed
column.

It is load-bearing, not decorative. The local-repetition rule's first
justification at `:929` is "the page's audience may enter through an index at
that point" — so which concepts the index carries determines where a later page
owes the reader a local restatement. An undefined index leaves that rule with an
undefined antecedent.

The gap was found downstream by a human:
`.grove/10-ordinal-fs-tree-book-k10/12-DONE-review-impl--book-assembly-k36.md:375-396`
is finding 11, "`concept-index.md` ordering breaks at the end" — a reviewer
having to invent the ordering expectation the design never wrote, and noting
"The links all resolve, which is why mechanical validation does not see this."

**Repair.** Give it the same treatment as the other lookup surface: an exact row
form, a stated sort (page order, then anchor occurrence within the page, with
non-page entries in a named position), and a coverage rule — at minimum, every
anchor named by an early-use row or a worked-example row appears. Or state
explicitly that the index is author-discretionary and remove "may enter through
an index" from the repetition rule's antecedent. Leaving both as they are makes
one unspecified artifact govern one specified one.

### 11. The normative JSON example contradicts the spec's own corpus total

**Severity: Low. Class: internal inconsistency.**

`:836-846` is the sample envelope. For scope `read-path-k14` it pairs
`"resolved_lines": 3057` with `"deferred_lines": 3561`. The prefix through
`read-path-k14` owns 203 + 700 + 1,191 + 963 = 3,057 lines of a 6,929-line
corpus (`:564`), so deferred is **3,872**. 3,561 is 6,618 − 3,057 — the stale
planning-era total, which appears nowhere else in the spec. Per-root exactness
is required at `:722` ("an exact sum of resolved plus deferred ranges for every
root"), so the example violates a rule three sections above it.

The tool agrees with 6,929, not with the example:
`crates/book-validation/tests/corpus_validation.rs:17` asserts 6,929 resolved in
final mode, and `:59` asserts `deferred_lines: 6_726` for the orientation prefix
— 6,929 − 203.

**Repair.** `3561` → `3872`.

### 12. The normative orientation control path inverts one call nesting

**Severity: Low. Class: factual error in a normative trace.**

`:145-159` gives the path the orientation page must follow "through this exact
control path", and `:181` requires the page to name "the exact functions and
types above". Two steps read:

    → ops::insert returning Decision
    → Plan::guarded

`Plan::guarded` is not a step after `ops::insert`; it is how `ops::insert`
produces the `Decision` it returns. `crates/ordinal-fs-tree/src/ops.rs:176`
opens `pub(crate) fn insert`, and `:250` is its tail expression
`Plan::of(effects).guarded(snapshot)`, ending the function at `:251`. The design
itself knows the range — its fragment-index example at `:481` gives `ops-insert`
as `176-251`.

**Repair.** Collapse to one step: `→ ops::insert, which builds the effects and
returns Plan::of(effects).guarded(snapshot) as a Decision →`.

### 13. `Parts` names two different types on two sides of a slice boundary, unqualified in both places

**Severity: Low. Class: ambiguity in a normative table.**

`:179` lists what page 02 explains: "`Found`, `Verdict`, `Species`, `Parts`,
`Triple`". `:587` is early-use row 2: "`Label`, `Status`, `Parts`,
`SyllabusName`", owner `reference-domain-k13`.

They are different types. `EntryName::Parts` is an associated type at
`crates/ordinal-fs-tree/src/name.rs:410`, in the name-seam slice.
`reference::Parts` is a concrete enum at
`crates/ordinal-fs-tree/src/reference.rs:162`, in the reference-domain slice.
Each statement is correct for its own referent, but the early-use ledger is a
normative table the validator checks for owner order and the technical reviewer
checks for sufficiency, so "is `Parts` explained by its owner?" has two answers.
The book resolved it by qualifying at the point of use
(`docs/ordinal-fs-tree/book/01-orientation.md:321`); the design did not.

**Repair.** Write `EntryName::Parts` at `:179` and `reference::Parts` at `:587`.

### 14. The corpus freeze is asserted with no enforcement named

**Severity: Low. Class: gap in a section the design owns.**

`:9-13` asserts that "The in-scope source corpus is frozen while the book node
is active." The validator contract at `:640-707` never requires the repository's
own verification to run final validation against the real corpus, so nothing in
the design makes an edit to `src/ops.rs` visible. `book-validation-k7`'s brief
asked for exactly this wiring — "The repository's normal verification exercises
the validators or an explicit book-verification command runs both with one
documented invocation" — and the design that owns the validator contract is
silent, so k7 decided it.

It decided well: `book-validation` is a workspace member (`Cargo.toml:13`) and
`crates/book-validation/tests/corpus_validation.rs:6-19` validates the real book
against the real source in final mode, so `cargo test` does enforce the freeze
today. The finding is that nothing records the requirement, so a later refactor
that moved the crate out of the workspace or made the test read fixtures would
silently retire it.

**Repair.** One sentence in the validator contract: the repository's default
test run performs final validation of the committed book against the committed
corpus.

### 15. "The fixed file inventory" is checked against an undefined notion of book file

**Severity: Low. Class: unspecified input policy.**

`:869` requires Markdown validation to check "the fixed file inventory", and
`:644-646` says only that `BookSnapshot` "contains the bytes of the book files".
Nothing states how the set of files actually present is determined, so the
inventory check has no defined subject.

`crates/book-validation/src/cli.rs:182-200` decided: enumerate the `--book`
directory non-recursively and keep entries whose extension is `md`. A stray
`docs/ordinal-fs-tree/book/notes.txt`, an image, or any `.md` in a subdirectory
is therefore invisible to final validation, which the design describes at
`:886-888` as requiring "all eleven book files".

**Repair.** State that the book directory contains exactly the eleven named
files and no other entry, and give any additional entry — of any extension, at
any depth — an `M101` finding.

## Decisions (running log)

Inspection-only, as the kind requires: no build, test, lint, or format command
was run, and no production artifact, source file, test, or task file other than
this one was edited. The validator crate and the delivered book were read as
*evidence about the design* — specifically, as the record of what a downstream
implementer had to decide because the design did not — not as artifacts under
review. Findings 1, 2, 4, 7, 8 and 15 each rest on a place where the
implementation had to invent policy; that is the sharpest available test of the
producer task's requirement that `book-validation-k7` be able to implement the
contracts "without redesigning the book."

The 6,618 figure in this task's own Context is stale. It was checked rather than
assumed: the fifteen files total exactly 6,929 lines on disk, the spec's tables
agree with that to the row, and 6,618 survives only in planning-era task files
and in the spec's one un-updated JSON example (finding 11). The design's number
is right; the review's brief was wrong, and the mechanical partition check it
asked for passes.

The producer's in-session adversarial read was not treated as settled. Its seven
repaired areas — lexer, table schemas, diagnostic records, page mapping,
repository loading, scope enumeration, prose example placement — were
re-derived from the committed text, and four of the fifteen findings land inside
them: finding 2 on the lexer, finding 8 on scope enumeration, finding 15 on
repository loading, finding 9 on prose example placement. Repairing an area is
not the same as closing it.

Findings warrant changes, so an `integrate-review-design` leaf with the bare
stem `book-system` is cut. Placement follows the directory-local adjacency
rule: reading `.grove/` after position 12, the first sibling entry whose subtree
still holds live work is `entry-name-discharge-contract-k32`, so the
integration is inserted at that slot rather than appended, keeping it ahead of
the one live leaf that could move the lines these findings cite.
