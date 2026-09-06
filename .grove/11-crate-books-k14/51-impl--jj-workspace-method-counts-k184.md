# jj-workspace-method-counts-k184

## Goal

Correct two count claims in the `jj-workspace` book that are wrong about the
crate's own shape, and that no source change has ever made wrong — they were
wrong when written.

## Context

Both were found by the adversarial read `lossy-path-rendering-k66` spent its
in-session reviewer on, and both were left there deliberately: that leaf changed
`lib.rs` and `refusal.rs`, and neither of these claims is about a byte it moved.
Neither is covered by `book-check`, which proves reconstruction and says nothing
about prose.

- **`04-namespace.md:365` — *"`control_dir` is one of five methods on
  `Workspace`."*** `Workspace` has six public functions — `resolve`, `root`,
  `main_repo`, `control_dir`, `is_tracked`, `commit` — which is what
  `07-what-jj-owns.md` says (*"the three types and six functions"*), plus two
  private ones, `fileset` and `relative`. Five holds only under the unstated
  reading *public methods taking `&self`*, which excludes the associated function
  `resolve` and the private pair. `01-orientation.md` spells the same set a third
  way — *"four operations … plus two accessors"* — which is six and matches
  neither *five* nor the word *methods*. **Decide the book's vocabulary once**
  (is `resolve` a method? are the private two?), then make all three sites agree;
  the defect is the disagreement, and picking a number without picking a rule
  will not survive the next reader.

- **`06-refusal.md:155` — *"The type itself is eleven lines, eight of which argue
  for the three that declare it."*** Fragment `refusal-opaque-type` is
  `refusal.rs:19-29`, eleven lines: 19–26 are the eight-line doc comment, 27 is
  `#[derive(Debug)]`, 28 is `pub struct Refusal(Kind);` and 29 is **blank**. So
  eight argue for *two* declaring lines and one blank, not three. Check whether
  the intended reading counted the blank line or the `derive`, and say which.

## Done when

- Both claims are true of the source, and the `Workspace`-surface vocabulary is
  consistent across `01-orientation.md`, `04-namespace.md` and
  `07-what-jj-owns.md` — with the chosen rule stated once rather than left to be
  inferred at three sites.
- No source byte changes, so no ledger row and no fragment range moves; this is
  a prose-only fix and `walkthrough.toml` should be untouched.
- `bash scripts/check.sh` passes.

## Notes

**Do not treat `book-check` green as evidence either claim is fixed.** It was
green while both were wrong, and it will be green afterwards whichever number is
written. The check is a re-derivation from `crates/jj-workspace/src/lib.rs` and
`refusal.rs` by hand.

**Expect neighbours.** These two survived one adversarial pass that was aimed at
the pages `lossy-path-rendering-k66` had edited; chapters 1, 2 and 4 were only
swept for the counts that leaf falsified. Counting every *other* quantity those
three chapters assert is in scope here and is the cheaper half of the job.
