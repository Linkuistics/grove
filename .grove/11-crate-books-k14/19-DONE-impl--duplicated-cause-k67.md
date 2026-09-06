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

## Decisions (running log)

**A refusal's message states its own layer, and the cause is `source()`'s.** The
`{cause}` interpolation leaves the four `Display` arms; `Error::source` is
untouched. A consumer that wants the *why* renders the chain — which is what
every grove call site already does, since all three reach `anyhow`'s `Debug` at
`grove-llm`'s `main` — and one that prints only `{}` gets the layer plus the
remedy paragraph the arm already carries.

*Rejected: the alternate-flag shape* — interpolate the cause only when
`!f.alternate()`, so `{}` stays self-contained and `{:#}` does not repeat. It
cannot work, and this is measured against the locked version rather than
remembered: `anyhow` 1.0.102 renders the **top** error with a plain `write!(f,
"{}", …)` in both `ErrorImpl::display` and `ErrorImpl::debug`
(`anyhow-1.0.102/src/fmt.rs:8` and `:27`), consulting `f.alternate()` only to
decide whether to append the *chain* below it. The flag therefore never reaches this
crate's `Display`, and the shape fixes nothing on the path grove actually uses.

*Rejected: drop the `source()` links.* It is the only shape that also removes
`anyhow`'s `{:#}` join — a remedy paragraph's full stop followed by `: ` and the
cause — and it produces the cleanest output of the three. It is rejected because
the cause chain is chapter 6's stated premise and the compensation the type
offers for being opaque: a consumer cannot branch on a refusal, and `source()` is
the one structured thing it gets instead. Buying tidier output by deleting it
makes the type strictly less useful.

*Not fixed, and stated rather than hidden:* the `{:#}` join survives. It is
`anyhow`'s rendering of any multi-line `Display` and not a property of these four
arms, and after this change the cause it appends is stated once rather than
twice.

*One arm's text moved with the interpolation, and had to.* `NotRunnable`'s `{}`
was the only one whose cause was doing work the remedy did not repeat: without it
the message read *could not run `jj`* and then named installation, which is the
wrong remedy when the binary is present and not executable. The remedy now says
*on `PATH` and runnable*, so the arm states both cases in its own words and the
chain still carries the `io::ErrorKind` that separates them. The other three
arms' remedies already stated theirs — a broken symlink or a removed directory, a
directory that must exist and be writable, and a repair that does not depend on
why the commit failed.

**No ADR.** The three-part test in `ADR-FORMAT.md` is an AND, and this fails
*hard to reverse*: it is one `match`, four string literals, and no published
surface. The durable record is the comment above the `Display` impl, the
contract on `Refusal`'s own doc comment, and chapter 6.

**The leaf's one in-session reviewer was spent on the prose, not the code.** The
byte-level half of this change is proved by `book-check`; the half no instrument
can see is the sentences about it, so that is where the allowance went. Sixteen
findings, classified: seven valid against text this leaf wrote (all fixed — the
corpus total spelled in words at `07-what-jj-owns.md:441`, two self-contradicting
sentences of my own, an overstated *"that is the real output"* that ignored
grove's `with_context` frame at `crates/grove-llm/src/cli.rs:453`, a stale
description of the changed test in chapter 3, an inaccurate comment on the test
itself, and the affordability claim below); nine valid but about claims this
change did not touch, appended to `jj-workspace-method-counts-k184`, which was
already the leaf for that class and whose own citation this leaf had moved; one
noise.

**One finding changed what the chapter says, and it is the honest cost of the
decision.** The first draft justified the trade with *each of the four remedies is
true without the diagnosis*. That is false for `ControlDir`: chapter 4 records a
reservation where the remedy named permissions and the true cause was *File
exists*, and the interpolated `io::Error` was how a `{}`-only reader saw the two
halves disagree. Removing it makes that failure mode invisible to that reader.
The chapter now says so twice — once in the worked example's cost paragraph and
once against the arm itself — rather than claiming a rule that has an exception in
the same book. The decision stands: the cause is one `source()` hop away, and
grove's three call sites either render the chain or never print at all.

**No `review-impl` leaf.** The second-need rule in `references/execute.md` fires on
re-review after a substantive non-mechanical fix, and the round-two edits are prose
corrections the reviewer itself specified, whose every new factual claim was
re-derived here — the three grove call sites, the four `//` comment blocks in the
file, and `anyhow`'s two writers. Nothing was left resting on a claim no context
has checked.
