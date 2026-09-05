# resolve-k145

## Goal

Draft chapter 9 of the `grove-loop` book — *Resolve*,
`docs/walkthroughs/grove-loop/09-resolve.md` — and prove the prefix through slice
`wider-than-a-key`.

## Context

- The fifth of `the-walk-k126`'s six chapter children, and **the largest**: its two
  ownership blocks are `resolution` (`task_tree.rs` 747–1015) and `resolve-tests`
  (1,653–1,996) — 613 lines, of which 344 are tests. Completing it resolves the
  last of `task_tree.rs`'s ten blocks and reconstructs the file in full; say so.
- The structure brief's section is *9 · Resolve*. The rule is that **grove's
  reference grammar is wider than a key, and has an ambiguous outcome the library
  has no counterpart for**: `Resolution`, `Located`, `located`, `resolve_in`,
  `Lookup`, `lookup`, `reference`, `existing_path`, `Ref` and `parse_ref` — a
  path, `[n]`, `n`, `<slug>-k<key>` or a bare slug.
- **The test block is two labelled sections, not one.** The source separates
  *resolve* (1,653–1,874) from *resolve: the full `<slug>-k<key>` handle
  (task-tree-scheme §5)* (1,875–1,996); both are inside the single
  `resolve-tests` block. Both take *supply the claim*: per reproduced test, the
  property it establishes **and** what would have to be true for it to pass while
  the property was broken. The production half is 42% prose and takes *do not
  restate*.
- **Two early-use rows close here**, both owned by `wider-than-a-key`: chapter
  1's cast row `verbs::resolve`, `Resolution`, and the `parse_ref` row added at
  `03-kind-slug-handle.md#one-place-the-grammar-is-spelled`. Both move from
  `pending` to `explained`, because `owner_is_complete` computes the expected
  status from the scope.
- `parse_ref`'s leniency on a bare key — `007` is key 7 — is the precedent
  chapter 3 argued `Handle::parse`'s leniency on `a-k007` from. This chapter is
  where that grammar is finally read, so it closes chapter 3's forward reference
  rather than repeating it.
- **The early-use ledger is a floor**: enumerate this chapter's reproduced bytes
  for later-owned symbols named or exercised, and add the rows they owe.
- The chapter's carried-example row is the brief's row 9: `plan-k1`, `[1]`, `1`
  and `plan` — four spellings of one reference — to one `Resolution`, or
  `Ambiguous` listing the keys. **This grove's own tree now poses that case**:
  `the-walk` is both this leaf's parent node `the-walk-k126` and its sibling
  `the-walk-k143`, so the ambiguous arm is live rather than hypothetical. The
  book is told strictly from the crate's side, so cite the behaviour and not this
  repository's `.grove/`.

## Done when

- `book-check --repo . --book docs/walkthroughs/grove-loop --through
  wider-than-a-key --check all` is valid: 13 files, 4,173 resolved lines, 6,360
  deferred, `final=false`.
- Chapter 9 exists, `README.md`'s contents entry and chapter 8's two navigation
  lines are updated, and both `wider-than-a-key` ownership rows read `resolved`.
- `scripts/check.sh` is red on `book-check` alone, and this file says so.

## Notes

**The corpus is frozen.** A defect found while drafting becomes its own leaf and
is not fixed inline.

## Found while drafting

**Three early-use rows closed here, not two.** This file's *Context* named two —
chapter 1's cast row `verbs::resolve`, `Resolution` and the `parse_ref` row at
`03-kind-slug-handle.md#one-place-the-grammar-is-spelled`. The third is
`reset_read_count`, `read_count`, anchored on `07-the-walk.md#nineteen-tests`
and owned by `wider-than-a-key`, which `the-walk-k143` added after this leaf's
body was written; the *parent* brief predicted it in as many words — *chapter 9
moves the row to `explained`* — so the miss was in this file rather than in the
tree. Enumerating the ledger for rows whose owner is this leaf's slice is the
check that catches it, and reading this file's own count is not. Same lesson as
`pick-test-count-k147` and `structure-brief-lexical-pair-k150`: **count before
writing a count, and count from the artifact rather than from the sentence that
describes it.**

**The early-use floor sweep added nothing, and that is a result rather than an
omission.** Both spellings were enumerated over the reproduced bytes of
`747-1015` and `1653-1996`. The only later-owned symbol either block names is
`crate::verbs::resolve` at line 753, and the chapter-1 cast row already covers
it — with `wider-than-a-key` as its owner, so the forward reference closes here
rather than pointing on to chapter 15. `target` at 938 is chapter 6's, which is
earlier and owes nothing. No hyphenated verb spelling appears in either block.

**No unresolved intra-doc link and no detached docblock in this chapter's
block.** `cargo doc --no-deps --document-private-items -p grove-loop` reports
thirty warnings for the crate and **none** of them falls between lines 747 and
1015; the three `task_tree.rs` unresolved links are at 580, 586 and 638, which
chapters 7 and 8 already adjudicated. The five-link tally the parent brief
carries is unchanged, and `unresolved-doc-links-k151` gains nothing from this
chapter.

**Seven refusal arms, measured by mutation; three are unobserved.** The
procedure the parent brief requires for a private resolver with many `bail!`
arms was run in a copy of the workspace: each arm replaced in turn with a
panicking sentinel, and `grove-loop` plus `grove-llm` — 245 inline tests, thirty
targets — run against each, diffed against an unmutated control so the copy's
own constant failures do not confound the result.

| Arm | Refusal | Held by |
|---|---|---|
| 792 | `located`: matched the root brief | none — unreachable through either branch of `lookup` |
| 867 | `slug_match_key`: `unreachable!` | none — already a panic, so a green suite proves it |
| 942 | `reference`: no entry matches | `add_under_nonexistent_parent_errors`, `insert_requires_an_existing_target` |
| 952 | `reference`: ambiguous, re-query by key | `add_refuses_an_ambiguous_parent_slug_and_lists_the_keys` |
| 990 | `parse_ref`: unclosed `[` | `resolve_malformed_bracket_ref_errors` |
| 993 | `parse_ref`: `[…]` not an integer key | `resolve_malformed_bracket_ref_errors` |
| 999 | `parse_ref`: not an integer key | none — reachable only by `u32` overflow |

Each covered arm was mutated separately and produced a small distinct failure
set, which is the control the procedure asks for: it attributes each failure
rather than counting it. **Two of the four covered arms are held from chapter
10's block** — `reference` is the mutating verbs' door — and the remaining two
share a single witness that reads no messages, so swapping the two clauses'
wording would pass unchanged. Recorded on the page; no leaf is owed, because the
corpus is frozen and adding a test would shift every later line of the file.

**One claim in the block is held by the type and not by the suite.**
`Located::kind` is `None` for a node directory. Two tests resolve a node — once
by key, once by handle — and neither inspects `kind`. Stated on the page.

**Four counts in the drafted page were wrong on the first pass and were caught
by enumeration**, which is worth recording because three of the four read
plausibly: *fourteen of fifteen tests use `resolve_fixture`* (twelve do), *the
only fixture built by hand* (four are, one per section-two test and three in
section one), *the absent-root test appears three times in the file* (four —
chapter 8 carries two, not one), and the two negative `terminal_key` rows
described as exercising two clauses (both stop at `peel_key`'s first). The
general form: **a count about a neighbouring chapter is as much a claim as one
about your own block, and the neighbouring page is not the place to check it.**

## How this leaf ended

- `book-check --repo . --book docs/walkthroughs/grove-loop --through
  wider-than-a-key --check all` → **valid: 13 files, 4,173 resolved lines, 6,360
  deferred, `final=false`**, which is this leaf's stated target exactly.
- Chapter 9 is `docs/walkthroughs/grove-loop/09-resolve.md`, 1,700-odd lines over
  thirty-six fragments that tile `747-1015` and `1653-1996` with no gap.
  `README.md`'s contents entry and chapter 8's two navigation lines are updated;
  the `resolution` and `resolve-tests` ownership rows read `resolved`; the three
  `wider-than-a-key` early-use rows read `explained`; the two `defer` directives
  under `source-task-tree` are now `insert`s, so `task_tree.rs` has no deferred
  block left.
- **`scripts/check.sh` is red on `book-check` alone.** All sixty-four of its
  `--final` diagnostics name a later chapter's deferred block or pending
  early-use row — `growing-the-tree`, `verbs::root_init`,
  `tree_lifecycle::leaf_prune` and the rest — and none names chapter 9. That is
  the shape the parent brief describes for every child but the last.
- **One caveat on that sentence, and it resolves in this leaf's favour.** The
  workspace run first came back red on `cargo test` as well, at
  `driver_lease.rs`'s `a_second_driver_refuses_before_tree_access_or_launch` and
  `a_reinitialized_tree_reuses_plan_k1_without_reusing_the_old_session`, both
  with *nothing wrote first-ready … after 120s*. That is
  `driver-lease-fixture-timing-k85` exactly, down to the two fixture names and
  the symptom, and k85 records it as first seen at `the-surface-k78` — another
  documentation leaf whose diff touched nothing those fixtures read. It was
  aggravated here by this session running the mutation sweep concurrently. Re-run
  unloaded, `cargo test -p grove-loop --test driver_lease` is **23 passed, 0
  failed, in 6.02s**. This leaf's diff touches no file under `crates/`, so it
  cannot reach them; the freeze holds and nothing here is owed to k85.

## Decisions (running log)

1. **Kept the source's two labelled test sections as two sections of the page**
   rather than merging twenty-one tests into one run. The source separates
   *resolve* (1,653–1,874) from *resolve: the full `<slug>-k<key>` handle*
   (1,875–1,996) because the fallback was added later and is argued from
   `task-tree-scheme` §5 rather than from the grammar's shape; merging them would
   have lost the reason the six exist as a group. The page says the second
   section's argument is carried by two of its six tests and that the other four
   would pass under implementations the section exists to exclude.
2. **Placed the refusal measurement after both test sections rather than beside
   the production fragments.** Chapter 8 put its equivalent inside the production
   section, but three of this block's seven arms are held from chapter 10's tests
   and two more from a single test in the second section, so the table cannot be
   read before both sections have been. The production half points forward to it
   in one clause instead.
3. **Cut no leaf for the two coverage gaps.** Arm 999 is reachable only by `u32`
   overflow and `Located::kind` is unpinned for nodes; both are gaps in the
   evidence rather than defects in the code, and the freeze makes a test-adding
   leaf a line-shifting change to a 2,023-line root that five finished chapters
   reproduce. Adjudicated on the page, which is what the book's third obligation
   is for.
