# ch14-grove-name-caller-k197

## Goal

Correct chapter 14's naming of `grove_name`'s second caller — it names
`initialize_grove`, which does not call `grove_name` at all — and reconcile
anything the correction falsifies.

## Context

- **The defect.** `docs/walkthroughs/grove-loop/14-finishing.md`, in *Chapter 11
  owns items this chapter is the main consumer of*:

      `grove_name` has two, of which line 81 is one and chapter 11's
      `initialize_grove` is the other.

  The sentence is enumerating **call sites**, as the clause before it does for
  `default_root_slug` and `root_shape` ("both callers are in this chapter's
  block, at lines 82 and 87"). `grove_name`'s two call sites in
  `crates/grove-loop/src/tree_lifecycle.rs` are line 81, inside
  `transition_to_current`, and line 347, inside **`root_init`**.
  `initialize_grove` (line 381) never calls `grove_name`; it *receives* the
  string both callers computed. So the correct name is `root_init`, and it is
  still "chapter 11's" — `root_init` is in chapter 11's block, exactly as the
  sentence claims of the item it misnames.
- **Confirmed by enumeration, not by search.** `grep -n grove_name
  crates/grove-loop/src/tree_lifecycle.rs` gives four hits — the two call sites
  (81, 347), the definition (1023), and the test
  `root_init_root_brief_title_is_the_grove_name` (1300). No other file in the
  workspace names it.
- **Found by `welded-grove-name-summary-k160`** while reconciling chapter 11's
  reproduction of `grove_name`'s docblock. That leaf's repaired comment now says
  *Both callers pass it straight to [`initialize_grove`]* — true, and adjacent
  enough to this sentence that leaving the error in place invites a reader to
  reconcile two pages that cannot be reconciled.
- **No source change is involved.** This is a prose defect on a finished page;
  the frozen corpus is untouched, no fragment moves, and no ledger changes.
  `welded-grove-name-summary-k160` deliberately did not widen into it.
- **Check chapter 11 while you are there.** Its own *Two callers, one of them
  chapter 14's* paragraph names line 347 as `root_init` correctly; confirm the
  two pages agree after the fix rather than assuming they do.

## Done when

- `14-finishing.md` names `root_init` as `grove_name`'s second call site, and the
  sentence still reads as an enumeration of call sites.
- Every other page that enumerates `grove_name`'s callers is checked against the
  same four hits and agrees — chapter 11 at minimum; grep the book for
  `grove_name` and read each occurrence rather than trusting this list.
- `book-check --repo . --book docs/walkthroughs/grove-loop --final --check all`
  is green.
- `bash scripts/check.sh` is no worse than it was before this leaf.

## Notes

**Do not widen this into a sweep of the book's caller counts.** The class it
belongs to — a count that is right and an attribution that is wrong — is worth a
leaf of its own if a second instance turns up; this leaf fixes the one instance
it can name.

## Decisions (running log)
