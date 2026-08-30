# book-system-k100

**Integrates:** book-system-k25

## Goal

Triage the `book-system-k25` findings against the committed book-system design
and apply the real ones, so the specification, the book, and the validator agree
on one contract.

## Context

- Findings: the `## Findings` section of `book-system-k25`, read from that
  leaf's own commit. Fifteen findings, each naming a location, the rule it
  violates, and a candidate repair. Triage them; the review's severities are
  its judgment, not this leaf's charter.
- Artifact under repair: `docs/specs/ordinal-fs-tree-book.md`, produced by
  `book-system-k6`.
- Requirements: `plan-k1`, the root brief, and `SPEC-FORMAT.md`'s current-state
  rule.
- Downstream consumers that already shipped against this design and may have to
  move with it: `crates/book-validation` (and its tests) and
  `docs/ordinal-fs-tree/book/` — 92 fragment definitions, `source-index.md`,
  and eleven pages, all of them `DONE`. A repair that changes an identifier
  domain, a directive form, or a diagnostic code is a change to all three.
- Frozen corpus: the fifteen production files listed in
  `ordinal-fs-tree-book-k10`'s brief. Repairs may change how the design
  *describes* them; they may not change the files.

## Done when

- Every finding is settled explicitly as accepted, accepted-in-part, or
  rejected, with the reason recorded in this leaf's running log. A rejection
  states what makes the finding wrong or not worth its cost, not merely that it
  was expensive.
- Each accepted finding is repaired in the design, and every artifact the repair
  invalidates is brought back into agreement in the same session — the
  specification, `docs/ordinal-fs-tree/book/`, and `crates/book-validation`
  together, never the spec alone.
- Where a finding names an unmade decision rather than an error, the decision is
  made and recorded, and the losing alternative and its reason are kept.
- Any repair too large to carry here is externalised as a leaf with a named
  scope, rather than left as a note in the spec; the spec does not gain new
  transitional or staging prose.
- Fragment and Markdown validation and the crate's existing verification pass
  after the changes, at the invocation the repaired design names.
- The design reads as one current-state artifact: no rule contradicting another
  rule, no example contradicting a table, and no statement about the tool that
  the tool does not satisfy.

## Notes

The review ran after every consumer of this design had already landed, so the
usual asymmetry is inverted: the cheap repair is no longer cheap, and a finding
that would have been obvious to accept before authoring may now be right to
reject. That trade is this leaf's to make and to record. What it must not do is
leave a finding unsettled because settling it is inconvenient — an accepted
contract defect with a recorded reason is a result; a finding nobody answered is
not.

Findings 1, 2, 4, 7, 8 and 15 each rest on a place where `book-validation-k7`
had to invent policy because the design supplied none. For those, the working
implementation is evidence of what the contract has to say, not evidence that it
already says it: repairing them usually means writing down what the code already
does, and only sometimes means changing the code.

## Decisions (running log)

Finding 1 is accepted. The specification's source-root and top-level ownership
tables are the design authority; `ROOTS` and `BLOCKS` are a checked compiled
copy used at runtime. A repository test will parse the normative tables and
compare every row with the compiled copy, while `F009` continues to reject a
book ledger that disagrees with that copy.

Finding 2 is accepted in part as a contract stated too broadly. The shared
byte-level lexer remains the sole recognizer of directives and fenced ranges.
The one Markdown link scanner additionally recognizes exact inline-code spans;
inline code is not promoted into the fragment token language. Finding 4 removes
the only second fence scanner, so no two components retain competing fence
interpretations.

Finding 3 is accepted. The permanent specification will describe the completed
`fragments | markdown | all` surface, default `all`, and both runnable commands
will request `all`; the obsolete staged-work paragraph will be deleted.

Finding 4 is accepted with the strict alternative. Relative links may target
only the eleven book files or the fifteen frozen source paths. Arbitrary linked
repository files, `BookSnapshot.linked_files`, the loader pass that populated
it, and the fallback repository fence scanner will be removed.

Finding 5 is accepted. The early-use trigger covers reproduced bytes whose
referent is owned by a later slice, not only types. The two manifest handoffs
will become required rows and the final source index will record them as
explained.

Finding 6 is accepted as a visible trade-off without taking the candidate
migration. The eight slice tokens are defined normatively by this specification
and remain opaque book-system identifiers after `.grove/` is removed; their
historical spelling does not require a live Grove handle. Reusing page IDs was
rejected because it would collapse source ownership identity into page identity,
make a page rename an ownership/CLI migration, and churn roughly 500 already
consistent occurrences without changing validated behavior.

Finding 7 is accepted. A `defer` is legal only as a direct child of a source
root; a defer in a composite is `P002`. This makes every defer correspond to one
top-level ownership row and removes the conflicting nested-credit rules.

Finding 8 is accepted with the unrepresentable-state repair. The core receives
a typed scoped-slice value covering exactly the seven accepted prefixes, or
`Final`; arbitrary strings remain a CLI parsing concern and cannot enter
`validate` as a command-shaped finding.

Finding 9 is accepted. The structural predecessor rule becomes mechanical code
`M105`; the five semantic questions remain editorial judgments.

Finding 10 is accepted in part using the review's author-discretionary
alternative. `concept-index.md` is curated optional navigation whose local links
must resolve; it is not a completeness or repetition authority. Authoring will
curate it, and the repetition rule will no longer depend on hypothetical index
entry.

Finding 11 is accepted. The scoped JSON example's deferred count is 3,872.

Finding 12 is accepted. The orientation trace will state that `ops::insert`
returns `Plan::of(effects).guarded(snapshot)` as its `Decision`, rather than
placing `Plan::guarded` after the call.

Finding 13 is accepted. The two normative references become
`EntryName::Parts` and `reference::Parts`.

Finding 14 is accepted. The validator contract will require the repository's
default test run to validate the committed book against the committed frozen
corpus in final mode.

Finding 15 is accepted. The book-directory inventory is recursive and exact:
any additional file or directory entry, regardless of extension or depth, is an
`M101` finding.

The leaf's narrow doubt review produced three valid, actionable refinements to
Finding 15. Snapshot inventory records non-regular entry identity as well as
paths, so a symlink at an expected page path is `M101`; the CLI refuses a
symlink at the book root before canonicalization; and an unreadable unexpected
directory remains represented for `M101` instead of aborting snapshot loading.
Executable core and CLI tests cover each case, including both in-repository and
out-of-repository page symlink targets. No further review leaf is needed because
the repaired behavior is directly falsifiable at those test seams.

Repository-wide verification exposed one compatibility consequence of Finding
2: the root documentation-navigation test consumes the public Markdown scanner.
The API is retained as a thin adapter that obtains opaque ranges from the shared
parser and invokes the single link scanner; it does not restore the removed
fallback fence recognizer.
