# decision-nine-loop-signature-k177

## Goal

Bring `docs/specs/module-decomposition.md`'s decision 9 back into agreement with
the interface it states as written source: the loop's third parameter type and
`LoopOutcome`'s variant set have both moved under it.

## Context

- **The defect, in two places, both inside decision 9's Rust block.**
  - It declares `pub fn run(workspace: &Workspace, lease: DriverLease, templates:
    &Templates) -> Result<LoopOutcome, Error>`. The shipped third parameter is
    **`&TemplateSource`** (`crates/grove-loop/src/loop_driver.rs` line 196), the
    type `crates/grove-loop/src/session_config.rs` line 98 declares. `Templates`
    is a real type but a **different crate's** — `crates/keyed-launch/src/templates.rs`
    line 17 — and the same record cites `keyed_launch` types elsewhere, so the
    name is actively misleading rather than merely stale. `TemplateSource` appears
    nowhere in the record.
  - It declares `pub enum LoopOutcome { Finished, Stopped }`. The shipped enum has
    **three** variants: `Finished`, `Stopped`, and `Interrupted(i32)`
    (`loop_driver.rs` lines 142-165). `Interrupted` appears once in the record, at
    line 352, and that occurrence is `keyed_launch`'s own `End` enum, not this one.
- **This is a specification defect, not a corpus defect**, which is why no
  chapter fixes it and why the `Done when` below touches no `crates/` file. The
  comment that cites decision 9 — `loop_driver.rs` line 169, for the loop's shape
  *exists? → create or find next → determine the command → run → finalise* — is
  **correct**, and that clause is what the sentence leans on the record for.
  Chapter 20 states the drift in one clause and does not adjudicate further.
- **Check whether the drift is wider than these two before editing.** Decision
  9's block declares roughly a dozen items — `read`, `write`, `Reading`,
  `Writing`, `Reference`, `Selection`, `DriverLease`, `Mandate`, `compose`, `run`,
  `LoopOutcome`, `Error`, and a `verbs` module — and chapter 19 confirmed that
  `Mandate`'s four fields and `compose`'s signature still hold. **Enumerate every
  declaration in the block against the shipped source** rather than repairing the
  two named here; two found by one chapter is not evidence the rest are current,
  and a record that is written source is checked by reading the source.
- **Nothing enforces this record against the code.** It is a `docs/specs/` file,
  so `book-check` never reads it, the Markdown link sweep checks only its links,
  and `every_adr_citation_names_a_decision_record` checks citations *to* records
  rather than the contents *of* one. Decision 9 is cited from at least
  `loop_driver.rs` line 169; enumerate its citers before rewording, since a
  citation may lean on a clause the repair would move.

## Done when

- Decision 9's Rust block agrees with the shipped interface for **every**
  declaration in it, not only the two named above, and the enumeration is recorded
  in this leaf's decision log so a later reader can see what was checked.
- No `crates/` file is modified: this leaf changes a specification only, so the
  corpus freeze is not in play and no book page, ledger or fragment range moves.
- If any declaration cannot be brought into agreement without a design judgement
  — a record deliberately stating an intended rather than a shipped shape — that
  is stated in the record rather than silently reconciled.
- `bash scripts/check.sh` is no worse than it was before this leaf.

## Notes

**This leaf is not deferred behind the book**, unlike its siblings: it changes no
bytes any page reproduces, so it can run whenever it is picked. It is placed here
because it was found while drafting chapter 20 and belongs to the same campaign.

**Do not treat chapter 20 as the inventory.** The page states the drift in one
clause because a record is not corpus and the book does not correct it; the two
sites named above are what one chapter noticed while reading `run`'s signature,
and the enumeration this leaf owes is over the whole block.

## Decisions (running log)

**The block is a selective statement of the surface, so only a stated
declaration can disagree.** This had to be settled before any finding could be
classified, because the block omits public items everywhere: `Reference` ships
`root`, `is_root` and `as_str` beside the declared `parse`; `verbs` ships
`stale_cross_refs` and `signal_channel` beside the twelve; the crate ships
`VERSION`, `interpret`, `Disposition`, `admit_ambient_session` and
`SessionEpochGuard`, none of them here. Read as an inventory the record would be
wrong in a dozen places and the repair would be a rewrite; read as a statement of
the surface it is wrong in four. The block's own idiom settles it — `pub struct
DriverLease;` and `pub struct Error;` are written as unit structs where both ship
fields, so the block was already abbreviating deliberately. The two omissions
from `verbs` are the ones worth naming, and both are documented at their own
definitions as *not* verbs, so a reader meeting them is not left to guess; the
record now says so in one new paragraph.

**The enumeration, all thirteen top-level declarations plus the twelve verbs and
the eight result types declared beside them, read against the shipped source.** Agreeing as written:
`read`, `write`, `Reading`, `Writing`, `Reference`/`parse`, `Selection` (three
fields, `crates/grove-loop/src/task_tree.rs:574-578`), `DriverLease` with
`acquire`, `worktree_root` and `revalidate` — and those are its only three public
methods, counted rather than assumed (`driver_lease.rs`, one `impl` block);
`Mandate`'s four fields and `compose` (`prompt.rs:184-210`, as chapter 19 had it);
`Error`; and in `verbs`, `root_init`/`Initialized`, `pick`, `kind`, `brief_chain`,
`resolve`/`Resolution`, `leaf_add`, `leaf_insert`/`Inserted`,
`leaf_decompose`/`Decomposed`, `leaf_retire`, `leaf_prune`/`Pruned`,
`finish_commit`, `complete`/`Signalled`. Four disagreed, two more than the task
file named:

1. `run`'s third parameter, `&Templates` → `&TemplateSource`
   (`loop_driver.rs:193-197`). Repaired.
2. `LoopOutcome`, two variants → three, `Interrupted(i32)` added
   (`loop_driver.rs:142-166`). Repaired, with a comment carrying why the signal
   number is on the variant.
3. **`Renumber`, two fields → four** — `from_position: u32` and `to_position: u32`
   (`task_grow.rs:207-216`). Not named by the task file. Nothing in the source
   defines itself against the record here, so it is straightforwardly stale.
   Repaired.
4. **`Located`, three fields → four** — `outcome: Outcome`
   (`task_tree.rs:768-786`). Not named by the task file, and **deliberately not
   repaired.**

**Why `Located` is stated rather than reconciled — the one design judgement the
`Done when` anticipated.** The shipped field's own doc comment justifies it *by
reference to its absence from this record*: "Not in
`docs/specs/module-decomposition.md`'s listing of this struct, and deliberately
added back." Adding the field to the listing would have made that sentence false
— and `task_tree.rs` is a `crates/` file, which this leaf may not modify and the
corpus freeze protects. So the divergence is recorded in a new paragraph after
the block, naming the field, its type, its purpose and where its justification
lives, and saying explicitly that the listing stays short on purpose. The record
is now honest about the shipped shape without falsifying the source that explains
it, which is what "stated in the record rather than silently reconciled" asks for.
A later leaf that wants the listing complete owes the source comment a rewording
in the same commit, and must land it under the freeze rule.

**Citers of decision 9, enumerated before rewording, and only one leans on a
clause the repair moves.** Seventeen lines outside `.grove/` and this leaf, from a
repository-wide sweep for the phrase, and they are not seventeen citations: seven
sites in source and configuration — `CONTEXT-MAP.md:85`, `Cargo.toml:25`,
`crates/grove/Cargo.toml:13`, `crates/grove/src/main.rs:7`,
`crates/grove-llm/tests/resolve.rs:264`, and `crates/grove-loop/src/prompt.rs:94`
and `:181` — and ten lines across four book pages, of which four are those source
citations reproduced inside fragments (`overview/01-orientation.md:118`,
`overview/03-three-steps.md:117`, `grove-loop/19-the-core.md:453` and `:680`) and
six are the books' own prose (`overview/01-orientation.md:106`,
`overview/03-three-steps.md:78`, `grove-loop/19-the-core.md:269`, `:494` and
`:704`, and `grove-loop/20-the-loop.md:718`). Six further hits are `CHANGELOG.md`
entries, which are history and are not maintained against the present. What they
cite is the crate's domain-boundedness, the loop's five-step shape,
`Resolution::Root`, `Mandate`'s four fields with `compose`'s signature, and the
prompt's provisioning-gap paragraph — every one of them untouched, because the
repair appends inside the block and adds two paragraphs after it. Chapter 19's
census claim at line 269 was re-read against the repaired record and still holds.

**One citer is falsified by the repair itself, and it is externalised rather than
fixed here.** `docs/walkthroughs/grove-loop/20-the-loop.md:718-724` asserts the
drift in the present tense and names this leaf as holding it — true when drafted,
false the moment the record was repaired. This leaf's `Done when` forbids moving
a book page, and the work is a separate concern from the record, so it is cut as
`chapter-twenty-drift-clause-k201` and **inserted at position 51 rather than
appended**: the falsehood ships with this commit, so the fix must be the next leaf
picked, not the sixty-ninth. The node brief
(`14-grove-loop-book-k37/01-grove-loop-k123/BRIEF.md:542-549`) states the same
drift and is deliberately left alone — a brief records what was true when it was
written, and grove does not rewrite the tree for staleness (constraint 1). k201
says so, so a later session does not "fix" it.

**Checks.** `bash scripts/check.sh` — all 8 principal checks pass, including
`book-check` over all six books (0 failing) and the repository link sweep. The
only file changed outside `.grove/` is `docs/specs/module-decomposition.md`; no
`crates/` file, no book page, no ledger and no fragment range moved.
