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

- **`06-refusal.md:329` — *"The type itself is sixteen lines, thirteen of which
  argue for the three that declare it."*** **`duplicated-cause-k67` moved this
  sentence and rewrote its first two numbers**; the defect this bullet was cut
  for is in the third, which that leaf carried forward untouched because deciding
  it is this leaf's job. Fragment `refusal-opaque-type` is now `refusal.rs:19-34`,
  sixteen lines: 19–31 are the thirteen-line doc comment, 32 is
  `#[derive(Debug)]`, 33 is `pub struct Refusal(Kind);` and 34 is **blank**. So
  thirteen argue for *two* declaring lines and one blank, not three. Check whether
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

## Seven more, from `duplicated-cause-k67`'s adversarial read

That leaf spent its reviewer on chapters 6 and 7 and the ledgers, and the
uniqueness claims below fell out of it. **None is about a byte `k67` moved** — each
was wrong before it and is wrong after — so they are recorded here rather than
fixed there, on the same rule that put the two above here. Line numbers are as of
`k67`'s commit.

- **`06-refusal.md:269` — *"the only block in the book that is an entire source
  root."*** Three of the four blocks are: `manifest-no-dependencies` is `1-44` of
  a 44-line root, `subprocess-seam-source` is `1-81` of an 81-line root, and
  `refusal-source` is `1-268` of a 268-line root. Only `lib.rs` is split.
  `source-index.md`'s own *Ownership blocks* table is the evidence and it is on
  the same shelf as the claim.
- **`06-refusal.md:640` — *"it is the only generic parameter in the file … and
  `not_scoped` takes it for the same reason."*** The sentence falsifies itself:
  `Refusal::namespace` (`refusal.rs:79`) and `Refusal::not_scoped`
  (`refusal.rs:100`) both take `impl Into<String>`. Two, not one.
- **`06-refusal.md:931` — *"[`CommandFailed`] is the only arm that renders on one
  line."*** `OutputNotText`'s literal contains no `\n` either, and the chapter says
  as much seven lines later (*"`OutputNotText` says what happened and stops"*). If
  the intended subject was the *source* `write!` rather than the rendered message,
  say so — the sentence's subject is currently the message.
- **`concept-index.md:80` — *"The one arm with no remedy paragraph."*** Two arms
  have none, which is what `06-refusal.md`'s summary and its case-analysis table
  both say. This is the summary-layer half of a finding the proof stage already
  fixed in the chapter and missed in the index.
- **`06-refusal.md:35` — *"Most of the file's non-comment bytes are that
  user-facing text."*** Measured on `refusal.rs`: 2,922 bytes of string literal
  against 4,957 of everything else non-comment — **37%**, not most. It was 37%
  before `k67` too (2,945 / 7,914), so the claim has never been true and `k67`
  did not move it. State the structural fact instead of a share.
- **`06-refusal.md:21-22` and `07-what-jj-owns.md:261-262` — *"is in this file
  eleven times"* / *"appears eleven times … Ten of the eleven speak for jj."***
  **Disputed rather than established, and the ambiguity is the defect.** Under
  *every message is caller-independent* the count is eleven and both sentences
  hold; under *every message names jj's offer* it is nine, because `CommandFailed`
  and `OutputNotText` name no remedy — which is what the case-analysis table
  (`06-refusal.md:440-441`, both rows *none*) and the summary (*"absent in exactly
  the two arms"*) already say. `PathNotText`'s remedy is the filesystem's by the
  chapter's own words, so *ten speak for jj* fails under either reading. Pick the
  reading, state it where the shape is first named, and make all three sites obey
  it.
- **`06-refusal.md:447` — *"every entry in it is a statement about jj's offer."***
  Two entries in that column read `none`, and two more are not jj's by the
  chapter's own later text (`PathNotText`'s is the filesystem's,
  `OutsideWorkspace`'s is a call back into this crate). Same root as the bullet
  above; fixing one without the other leaves the pair disagreeing.
- **Two lower-confidence ones, listed so they are not re-found:**
  `06-refusal.md:948` *"Ten of the eleven kinds say something about a command"*
  (three name a command); `06-refusal.md:1035` *"the only line in the file that
  disclaims an action"* (`NotAWorkspace` ends on *Nothing was created or
  changed.*, which the chapter itself calls a claim about the whole call).

**The pattern is worth naming, because it is what makes the list long.** Every one
of these is a *uniqueness or count* claim — *the only*, *most*, *ten of the
eleven* — and `book-check` is structurally blind to all of them: it proves that
the quoted bytes match the source and never that a sentence about those bytes is
true. Sweeping chapters 1–7 for that one grammatical shape is a cheaper and more
complete job than re-reading them for correctness.
