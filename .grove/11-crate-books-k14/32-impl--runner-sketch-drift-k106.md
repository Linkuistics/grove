# runner-sketch-drift-k106

## Goal

Bring `docs/specs/module-decomposition.md` decision 7's interface sketch back
into agreement with `crates/keyed-launch`'s actual public surface, in six places
where the crate has grown past it.

## Context

- Found at `keyed-launch-structure-k34` while measuring the corpus for the book's
  structure brief. The sketch is a *specification of the interface*, and it is
  the one place in the repository that states that interface whole — so a reader
  who trusts it is told a smaller surface than the crate has.
- The six drifts, all in decision 7's fenced sketch:

  | The sketch says | The crate has |
  | --- | --- |
  | `Launch { argv, channel, channel_var, scrub, escalation }` | plus `pub cwd: Option<&'a Path>` (`src/run.rs` line 75) |
  | `enum End { Exited, Signalled, Interrupted }` | `Interrupted { signal: i32 }` — the number is carried, not merely noted (`src/run.rs` line 121) |
  | `impl Templates { load, source, expand }` | plus `require` (`src/templates.rs` line 162) and `keys` (line 667) |
  | `impl Channel { allocate, path, read, discard }` | plus `discard_abandoned` (`src/channel.rs` line 141) |
  | no free functions beside `signal` | `take_interrupt` (`src/run.rs` line 163) and `reraise` (line 186), both re-exported from `src/lib.rs` line 66 |
  | `impl Argv { program, args }` | plus `words` (`src/argv.rs` line 42) |

- **`take_interrupt` and `reraise` are the substantive omission**, not a
  cosmetic one: they are the whole of a looping launcher's obligation for a
  signal that arrives *between* launches, and the sketch's `End::Interrupted`
  without its payload cannot express the ending they exist to report. The other
  four are additions the sketch simply predates.
- The prose *below* the sketch is not stale — the runner paragraph and the
  vocabulary-at-load paragraph both hold — so this is a fenced-block edit, not a
  rewrite of the decision.
- The four ADRs (`complete-session-configuration`,
  `untracked-configuration-delta`, `the-launched-child-is-a-job`) are unaffected:
  none of them restates the surface.

## Done when

- Decision 7's sketch names all six, and the sketch's doc comments still say why
  each member exists rather than merely listing it, as the rest of it does.
- No source file under `crates/keyed-launch/` is touched. **This is a document
  edit only**, so the corpus freeze is not engaged and no ledger, page or
  validator run is implicated.
- `bash scripts/check.sh` passes.

## Notes

**Not a blocker for `keyed-launch-book-k35`.** The book cannot cite this document
— `docs/specs/walkthrough-books.md`'s link contract closes a book's local targets
to its own pages, its own roots, the guide and the glossary — so the book neither
repeats the drift nor can adjudicate it at a fragment. The brief records it under
*Known in advance* and points here. Either order works.

**Check for a second reader before editing.** `docs/ARCHITECTURE.md` and
`CONTEXT-MAP.md` both point at decision 7 as the interface's owner; confirm
neither restates a member list that would drift the other way after this edit.
