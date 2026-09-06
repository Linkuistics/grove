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
