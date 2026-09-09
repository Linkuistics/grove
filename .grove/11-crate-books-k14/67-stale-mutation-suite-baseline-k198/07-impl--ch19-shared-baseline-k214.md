# ch19-shared-baseline-k214

## Goal

Make `docs/walkthroughs/grove-loop/19-the-core.md` agree with the seven chapters
it speaks for. It is the only page in the book that states the mutation
baseline *on behalf of* other pages — the six before it now state their own — so
its one citation, its ten-versus-eleven adjudication and its account of why the
eleven fail all have to close over what chapters 11 to 17 now say. Then decide,
once, whether this class of shared roll-up should be mechanically checked.

## Context

- **One citation, and it is the last `558` in the book.** Line 123, in *The file
  the whole book's baseline is made of*: *seven of them — chapters 11 to 17 —
  cite the same environmental baseline: 558 tests, 547 passed, eleven failed
  before any mutation, in `crates/grove-loop/tests/prompt.rs`.* The re-derived
  control, reproduced identically at all six children of this node, is **560
  tests, 549 passed, 11 failed**. Both the total and the passing count move; the
  eleven does not.
- **The sentence is a cross-chapter claim as well as a number, and it is the
  interesting half** (`assembly-chapters-are-cross-chapter-claims` in miniature:
  this page owns a root, but *this sentence* owns none). It asserts that seven
  named chapters cite one baseline. Chapters 11–17 have now each been
  re-measured and re-worded by a separate leaf — `k208` through `k213` — and the
  clause has to be checked rather than assumed: read each of the seven and
  confirm both that it cites this baseline and that it cites *this* one. The
  wordings deliberately differ; the numbers must not.
- **The ten-versus-eleven adjudication is this page's, and the node brief has
  already ruled on the part that is not.** The page argues that copying
  `.claude-plugin/` gives a clean baseline of ten of one cause, and that a flat
  eleven can hide an observer of anything touching `PLUGIN`. That argument is
  sound and stays. What it must not do is re-describe the seven chapters'
  controls as tens: all seven were re-derived against the **eleven**, and the
  eleventh's separate cause is stated on this very page — so the recommendation
  stands as a recommendation about how a *future* study should copy, not as a
  restatement of what these seven measured.
- **Three enumerations on the page, none derived from a run.** *Ten of this
  file's sixteen tests reach `compose`: nine through it or `compose_with`
  directly, and one through `signalling_contract()`.* *The evidence is sixteen
  tests, and the brief pins four of them.* And *Chapter 10, whose copy was scoped
  differently — 626 tests over three crates — reports ten rather than eleven for
  the same reason.* Count each against
  `crates/grove-loop/tests/prompt.rs` and against chapter 10's own page
  (`uniqueness-and-count-claims-need-enumeration`); the third is a claim about
  another chapter's measurement, so check what chapter 10 actually says as well
  as whether it is right.
- **One forward reference now lands that did not.** The page says a control
  wrong in that direction *hides an observer, which is the same failure mode the
  `cargo build -p grove --bins` step exists to prevent at seventeen.* Chapter 17
  did not state that step until `k213`; it now does, in its own procedure
  paragraph. Confirm the reference resolves to something chapter 17 says, rather
  than to something a reader has to reconstruct.
- **`k213` found one thing the seven share that this page may want to carry.**
  Four of eleven mutant runs in chapter 17's study newly failed a `task_grow`
  test that cannot execute a mutated line at all — a cross-test flake reading
  exactly like a newly attributed observer, settled by running the candidate
  alone under the mutant rather than by a second full run. Chapter 17 owns the
  account. Decide whether the shared-baseline section is the right place to say
  that the baseline is a *set*, not a count, for this reason too — or whether
  saying it twice is worse than saying it once.
- **The mechanical-check decision, with `ledger-rollup-check-k207` named.** That
  leaf owns the condition where an assembly chapter states a ledger roll-up the
  ledger contradicts, and it is explicitly scoped to **ledger-derived** roll-ups.
  This baseline is not one: it is derived from a run in a scratch copy, and
  nothing in the repository holds it. So the question here is a different one and
  should be answered rather than deferred — is there a check that would have
  caught seven pages drifting to `558`, and if the honest answer is that a number
  no artifact holds cannot be mechanically checked, say so and say what the
  cheap human-side substitute is. Decide it once, on this page, and name `k207`
  as the neighbouring case that *is* checkable, so a later reader is not left
  wondering why one got a check and the other did not.

## Done when

- Line 123 states the re-derived total and passing count, and its *seven of them
  — chapters 11 to 17* clause has been checked against all seven pages rather
  than carried forward.
- The ten-versus-eleven passage still recommends the clean ten and no longer
  reads as a description of what the seven chapters measured.
- The three enumerations are counted from the source and from chapter 10, and
  each either holds or is corrected.
- The mechanical-check question is decided on this page, once, with
  `ledger-rollup-check-k207` named and the distinction between the two cases
  stated.
- No `558` or `547` survives anywhere under `docs/walkthroughs/`, checked with a
  positive control that the same pattern still finds something it should.
- `book-check --final --check all` is green over every book touched, and
  `bash scripts/check.sh` is no worse than before.
- **This is the node's last child.** When it retires, the node has no live leaf
  left; close it as `references/retire.md` directs rather than cutting an eighth.

## Notes

**No source change.** The corpus freeze is the parent node's rule and it holds
here too; a defect found becomes its own leaf.

**The control, reproduced six times.** The node brief's *Pointers* carries the
copy recipe, and it has reproduced exactly at `ch11-a-grove-begins-k208` through
`ch17-the-epoch-k213`: 560 tests, 549 passed, 11 failed, matched set for set.
This leaf takes no mutation run of its own — it has no arm to mutate — so it
needs the recipe only if it chooses to re-confirm the control once more.

**Measure with `GROVE_SIGNAL_FILE` unset in the measuring shell** if any run is
taken: this repository is a meta-grove and the session inherits a live signal
path.

## Decisions (running log)

**From `ch17-the-epoch-k213`, before this leaf starts.** The book-wide `558`
sweep in the *Done when* above has exactly two hits today, and only one of them
is yours. `19-the-core.md:123` is the citation to repair.
`11-a-grove-begins-k208`'s line 176 is **not**: it reads *exactly one test of
the 558 the suite* ***then*** *held*, and the paragraph immediately after it
opens *That reading is superseded* and states 560. It is a record of a past run,
frozen by its own stated rule
(`counts-split-current-state-from-record-of-a-run`), and replacing its number
would make the sentence false. Classify, do not substitute.
