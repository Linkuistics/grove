# duplicated-cause-k67

## Goal

Stop `jj-workspace` printing the same cause twice to an operator: four of the
eleven refusal kinds interpolate a cause into their own `Display` **and** return it from
`Error::source`, so any consumer that renders the chain renders it once inside
the message and once after it — landed as a corpus change the book contract
permits.

## Context

- Four kinds hold a cause and return it from `source()`
  (`crates/jj-workspace/src/refusal.rs:238-244`): `UnresolvablePath`,
  `ControlDir` and `NotRunnable` return their `io::Error`, and
  `CommitNotRecorded` returns its boxed `Refusal`. All four also interpolate that
  same cause into their `Display` arm with `{cause}` (`refusal.rs:153-160`,
  `167-173`, `203-208`, `222-233`).
  **These line numbers were re-derived at `lossy-path-rendering-k66`**, which
  added an eleventh kind and moved `refusal.rs` from 230 lines to 254; the
  citations above are against the new file and the four kinds are unchanged.
- **It is reachable from grove and it was measured, not inferred.**
  `crates/grove-llm/src/main.rs:1` is `fn main() -> anyhow::Result<()>`, so a
  refusal that reaches the top is printed with `anyhow`'s `Debug` — the top
  message, `Caused by:`, then the chain. Measured on jj 0.44.0 by driving a real
  `Workspace::commit` failure (an unreadable directory in the working copy, the
  fixture `a_commit_that_cannot_land_names_the_operation_log_repair` uses) through
  the crate and formatting the result:

  ```text
  {:#}  … Nothing here runs a recovery of its own.: `jj commit -m …` failed in …
  {:?}  … Nothing here runs a recovery of its own.
        Caused by:
            `jj commit -m …` failed in …
  ```

  The `{:#}` join is worse than the repetition: the remedy paragraph's final full
  stop is followed by `: ` and a second message.
- **The trade the current shape buys is real, and a fix must keep it.** A
  consumer that prints only `{}` — which is what a message written for a person is
  for — must still be told *why*. `the commit did not land in /work/atlas` with no
  cause is a worse message than a duplicated one, so simply deleting `{cause}`
  from the four arms is not the answer on its own.
- Candidate shapes, and the decision is this leaf's: keep the interpolation and
  drop the `source()` link for the three `io::Error` kinds (loses machine-readable
  causes); keep `source()` and drop the interpolation, adding a documented
  requirement that consumers render the chain (changes what `{}` says, and grove
  has a site that discards the refusal entirely); or render the cause only when it
  is not already reachable — which is the only option that is a decision about
  `Display` rather than about the crate's contract.
- `docs/walkthroughs/jj-workspace/06-refusal.md`'s *Worked example* states the
  observable and its measurement, and its *What `source()` gives a consumer* names
  the four kinds. The page argues the current shape as a trade rather than as a
  defect, so a fix has to rewrite those two passages rather than only their
  fragments.

## Done when

- A refusal carrying a cause states that cause once in whatever a consumer
  renders, and the decision above is recorded with its rejected alternatives.
- A test asserts it against a real failure rather than a constructed one — the
  existing `a_commit_that_cannot_land_names_the_operation_log_repair` fixture
  already produces a two-link chain and is the cheapest place to assert on both
  renderings.
- The corpus-freeze rule in `.grove/BRIEF.md` is honoured: **one commit** carries
  the source change, every affected book ledger and page, and a green validator
  run over every book it touched. `refusal.rs:1-254` is the `refusal-source`
  block, owned by chapter 6 of the `jj-workspace` book and split into
  twenty-two literal fragments — a line moved in any `Display` arm moves
  `refusal-display-gate`, `refusal-display-namespace`, `refusal-display-seam` or
  `refusal-display-commit`, their fragment-index rows, and the block's own range;
  a change to `source()` moves `refusal-error-source-caused` and
  `refusal-error-source-uncaused`.
- `bash scripts/check.sh` passes.

## Notes

**Placed after every crate book deliberately, for the reason `jj-docs-url-k64`,
`jj-owned-names-k65` and `lossy-path-rendering-k66` carry.** Editing a byte of a
frozen root while a book that quotes it is being written invalidates the ranges
the freeze protects. Run this once the books that own `refusal.rs` exist and can
be re-proved in the same commit.

**This is not a request to adopt an error crate.** `no-dependencies` is the
crate's first refusal and chapter 1 is built on it; `thiserror` would write the
`Display` this leaf is changing and would publish the variants chapter 6 argues
must stay private. The fix is a change to hand-written text and one `match`.
