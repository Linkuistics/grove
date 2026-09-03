# what-passes-through-k117

## Goal

Draft chapter 10 of the `keyed-launch` book — *What passes through*,
`10-what-passes-through.md`, slice `assembly` — and take the book to green
**final** validation and a green `bash scripts/check.sh`.

## Context

- Draft stage, child 10 of 10 of `keyed-launch-k107`, and the only one that owns
  no source. It resolves no legitimate hole: every source-owning slice has already
  replaced its defers, and this slice is final-only under
  `docs/specs/walkthrough-books.md`, *Authoring workflow and scoped proof*.
- Responsibilities are the structure brief's *10 · What passes through —
  assembly*: state the pass-through test in the form under *Audience and intended
  outcome*, and apply it to all nine source-owning chapters — for each of the three
  places a layer learns a meaning it was never given (**on the way in**, **on the
  way through**, **on the way out**), which chapters proved the crate does not, and
  by which test. The six tests the brief names are the evidence.
- **It closes on the one thing the crate cannot do anything about**: a child that
  finishes and never signals, which no observable here can distinguish from one
  still working, and which is the caller's to close at the layer that instructs the
  child. Chapter 8 stated it; this page does not soften it either.
- Required example anchor: `where-does-it-learn` — the three parts of the test,
  each answered against the chapters that proved it.
- Close the indexes, move every early-use row to `explained`, and record the final
  evidence.

## Done when

- `book-check --repo . --book docs/walkthroughs/keyed-launch --final --check all`
  is valid: 10 pages, 9 source roots, 2,073 resolved lines, no deferred holes,
  `final=true`.
- `bash scripts/check.sh` passes, all 8 principal checks, the book gated by
  discovery rather than by a hand-added line.
- **Last act:** `grove-llm leaf-add keyed-launch-book-k35 keyed-launch --kind
  copy-edit`, unless a live later sibling under `keyed-launch-book-k35` already
  holds that stage — read that condition off the node's live entries before
  cutting. A defect an earlier stage owns is instead one contiguous correction run
  in pipeline order, replacing this last act.

## Notes

Retiring this leaf leaves `keyed-launch-k107` with no live leaf: close the node
under the spine's four steps, promote what survives into
`keyed-launch-book-k35`'s brief, and name both handles in the commit message.

## Decisions (running log)

**1 · The brief names seven tests, not six, and the page states seven.** This
leaf's `## Context` and `keyed-launch-k107`'s brief both say *the six tests the
brief names are the evidence*. Enumerated from
`docs/specs/keyed-launch-book-structure.md`, *Audience and intended outcome*, the
count is **seven**: two on the way in
(`a_key_only_the_overlay_declares_does_not_resolve`,
`an_overlay_replaces_a_whole_template_and_reports_its_own_path`), three on the way
through (`a_slot_value_is_one_argument_whatever_it_contains`,
`shell_metacharacters_stay_literal`,
`an_unquoted_hash_is_refused_rather_than_truncating_the_argv`) and two on the way
out (`a_child_that_never_signals_ends_with_no_token`,
`a_scrubbed_variable_is_removed_from_an_inherited_environment`). The human-authored
brief is right and the derived grove artifacts miscounted it, so the page follows
the brief. All seven exist and pass; each was read for its assertions rather than
its name or its doc comment before being described. The node brief is corrected on
retirement.

**2 · Chapters 1 and 9 are placed as the claim's statement and its external check,
not forced into one of the three arms.** The brief asks the page to apply the test
to all nine source-owning chapters. Seven of them close one arm each. Chapter 1
owns the manifest, the library root and the two error types — it *states* that the
crate understands neither half and proves no arm; chapter 9 holds a configuration
to obligations about a document's shape from outside the consumer, which reaches
all three arms and none of them exclusively. Both are in the assembly table with
that answer written down, because a table that assigned them an arm apiece would
have been a tidier table and a false one. Rejected: dropping them from the table,
which would have made *all nine* untrue.

**3 · One in-session reviewer spent on chapter-attribution truth.** The page's
distinctive risk is not its own argument but its ninety-odd claims *about other
chapters* — what each proved and by which test — which no validator checks and
which a drafting session is least able to see, having just written none of them.
That is the narrow claim class the allowance exists for.

**4 · The reviewer returned ten findings and all ten were real; every one was a
claim about another chapter, the source, or a count.** Classified under the
execute procedure's four ways, none landed as noise, as an unclearly stated
contract, or as a visible trade-off. Each was verified against the file that
disproves it before being fixed, and the fixes were: the stall paragraph is
chapter 8's *second*, not its first; chapter 7 carries it in `run`'s **function**
doc comment, not beside a field (`src/run.rs` 336–366); *grove's entire presence
in 2,073 lines is one sentence* was chapter 1's claim about the **book**
relocated onto the **source**, which names grove in four of its nine files; the
ownership table is `source-index.md#ownership-blocks`, not anything on chapter
1's page, and `#the-map` was simply the wrong anchor; `src/templates.rs`'s eight
blocks hold **four** in two non-adjacent pairs, chapter 5's as well as chapter
3's; the nine inline tests do not all exist because `is_channel_name` is private
— the *module* does, and one of the nine takes it up, the other eight using only
the public surface; chapter 9's assembly row named 104 of its 237 owned lines and
now names both roots; the second arm names three brief tests plus a fourth that
prices the third rule; and *the seven tests this page named* was false by
enumeration — the page names nine, of which seven are the ones the stated outcome
names. A tenth count I introduced while fixing the sixth (*five times across four
files*) was itself wrong by occurrence and was replaced with an enumeration.

**No re-review was cut.** Every fix is a local correction of fact with a file and
line that settles it, not a redesign, so the execute procedure's *second need*
signal did not fire; and this family has no `review-draft` kind to escalate to.
The class the findings came from is recorded for the later stages instead.

**5 · Chapter 5's forward promise is now kept where it belongs.** `05-to-an-argv.md`
line 744 promises that the closing chapter revisits the `Argv` seam *only as one
of the nine answers*. The draft introduced it fresh in the take-away section and
nowhere in the test itself, which read as a tenth argument. The seam is the
compile-time form of the **on the way out / adding** arm — chapter 7's
`#nothing-added` is its runtime form — so it is now part 3's answer, and the
take-away refers back to it rather than introducing it.
