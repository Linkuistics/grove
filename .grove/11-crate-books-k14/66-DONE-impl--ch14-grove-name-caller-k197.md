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

1. **`root_init`, confirmed by reading the enclosing `fn` at each call site.**
   `grep -n grove_name crates/grove-loop/src/tree_lifecycle.rs` gives exactly the
   four hits the task file names — 81, 347, 1023 (the definition), 1300 (the
   test) — and a workspace-wide `grep -rn grove_name crates/` adds only unrelated
   identifiers that contain the substring (`concurrent_loops_with_the_same_grove_name…`,
   `every_grove_name_in_the_tree_is_classified`, `grove_names_in`), so the two
   call sites are the whole population. Line 81 is inside `transition_to_current`
   (opens at 76); line 347 is inside `root_init` (opens at 341), whose body is
   `let name = grove_name(vacancy.root()); initialize_grove(vacancy, &name, …)`.
   `initialize_grove` (381) takes `name: &str` and never calls the function — it
   is the *consumer* both callers hand the string to, which is exactly the
   confusion the sentence made. One word changed, `initialize_grove` →
   `root_init`; the "two" was already right and the "chapter 11's" still is,
   since `root_init` is reproduced in chapter 11 at line 93.

2. **The rest of the same sentence was re-derived rather than assumed, because
   the correction has to leave a true enumeration behind.** `default_root_slug`
   and `root_shape` do each have exactly one caller in the workspace, at
   `tree_lifecycle.rs` lines 82 and 87, both inside `transition_to_current` and
   so both in chapter 14's block. "Chapter 11 reproduces and explains all three"
   holds for the three functions whose callers the sentence enumerates —
   `default_root_slug` (11 line 139), `root_shape` (431), `grove_name` (528).

3. **Every other occurrence of `grove_name` in the books was read, and none
   needed changing.** Six pages name it. Chapter 11's *Two callers, one of them
   chapter 14's* already said "line 347, in `root_init` above, and line 81, in
   `transition_to_current`" — correct, and now agreed with rather than
   contradicted. Chapter 11's test note ("`transition_to_current` calls it too"),
   its mutation row 9, its docblock-repair passage, chapter 13's helpers-divider
   paragraph, chapter 14's `Opening::Vacancy` bullet and chapter 1's
   `join(".grove")` passage make no claim about which functions call it.

4. **No source change, so no ledger or fragment moved.** `book-check --repo .
   --book docs/walkthroughs/grove-loop --final --check all` is green (13 files,
   10,557 resolved lines), and `bash scripts/check.sh` reports all 8 principal
   checks passing with 6 books checked and 0 failing.

5. **Not widened.** The book's other caller counts were not swept, per the task
   file's note; the only counts touched are the ones inside the corrected
   sentence.
