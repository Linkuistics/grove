# the-channel-k113

## Goal

Draft chapter 6 of the `keyed-launch` book — *Appearance is the event*,
`06-the-channel.md`, slice `appearance-is-the-event` — owning `src/channel.rs`
lines 1–271: 271 lines.

## Context

- Draft stage, child 6 of 10 of `keyed-launch-k107`. Responsibilities are the
  structure brief's *6 · Appearance is the event — the channel*.
- What the channel is and why a launch needs one at all — an interactive child
  returns to its prompt when it finishes rather than exiting, so its own exit is
  not the event anyone is waiting for. **Allocation picks a name and writes
  nothing**, which is what makes *appearance* the event.
- The name grammar — a recognisable prefix plus 128 bits of randomness — and why
  the prefix is named for `signal` and matches what grove's driver already leaves
  behind; `DRAW_RETRY_LIMIT` as a bound rather than an unbounded retry, because at
  128 bits a collision means the randomness source is not random; why `allocate`
  checks the directory rather than leaving it to the child's first write; `Token`
  as opaque to this crate and readable to its caller, and the line framing as
  framing rather than interpretation; `signal` as a free function because the two
  ends are different processes; and `discard_abandoned`, **whose exactness is the
  point** — a looser rule would let this crate's cleanup delete a neighbouring
  file in a directory whose other contents belong to the consumer.
- **This is the chapter that cites `CONTEXT.md` at `loop-control-channel`**, at its
  first use of *channel* for the thing grove's glossary already names. That anchor
  exists in explicit form today.
- **The block stops at line 271.** Lines 272–404 are the inline `#[cfg(test)] mod
  tests` and are chapter 9's, which explains them against this chapter's fragments.
  Do not reach into them; do not add a corpus exception row.
- Required example anchor: `a-path-and-nothing-else` — `Channel::allocate` in the
  control directory through to a path that does not exist, and a token read back
  after `signal`.

## Done when

- `book-check --repo . --book docs/walkthroughs/keyed-launch --through
  appearance-is-the-event --check all` is valid: 1,229 resolved lines, 844
  deferred, `final=false`.
- `scripts/check.sh` is red on `book-check` alone, and this file says so.

## Notes

`src/channel.rs` is the only root in this book split at a `cfg` boundary rather
than a conceptual one. The reason belongs on chapter 9's page, not this one.

## Decisions (running log)

**1 · Chapter 6's third prose obligation is the *do not restate* one, and the
measurement is why.** The structure brief's *What each chapter's prose owes*
assigns the directional third obligation explicitly to chapters 3, 4 and 5
(supply the argument) and to chapters 7 and 8 (do not restate), and names no
chapter in between. `src/channel.rs` lines 1–271 are **85 comment lines of 271,
31%** — measured this session, against `src/templates.rs`'s 13% and
`src/run.rs`'s 53%, and within two points of the corpus-wide 33%. The comments
here *argue*: the prefix, `DRAW_RETRY_LIMIT`, the directory check, the empty file
that is not an empty token, the free function and the exactness of the cleanup
grammar each carry their own case in situ, and the fragment graph quotes every
one of them verbatim on the page. So this chapter takes chapters 7 and 8's
instruction for those six, and spends its prose on the three things a doc comment
structurally cannot do: cite the tests, connect the decisions across roots, and
state what the source is silent about. Rejected: taking chapters 3–5's
instruction, which in a 31% root produces a page that paraphrases the comment
above every fragment it quotes — the exact failure mode the obligation was
written to name.

**2 · What the source is silent about, and this chapter says: `allocate` tests
occupancy with `symlink_metadata`.** The doc comment on `allocate` argues one
choice — why the *directory* is checked here rather than at the child's first
write — and is silent on a second in the same function. Occupancy is
`fs::symlink_metadata`, which describes the link, while the directory check above
it is `fs::metadata`, which describes what the link points at. The two spellings
differ in exactly two cases, both measured this session rather than reasoned
from memory: a **dangling** symlink is `Ok` to the first and `NotFound` to the
second, so `metadata` would call the name free; and a **symlink loop** is `Ok` to
the first and `FilesystemLoop` to the second, so `metadata` would turn a
redrawable name into a hard allocation failure. A relative dangling link was also
measured to land in the *same* directory — the check wrote a token onto a file
called `driver.lease` — which is why the page does not say *outside the directory
the caller named*. The page states this as what the two calls mean and says
plainly that **no test pins it**, per the method's rule against turning an
inference into a source fact.

**2a · The allowance was spent here, and it demoted the claim.** The leaf's one
in-session reviewer was given the argument with its conclusion stripped and an
adversarial *find what is wrong* brief. Findings, classified: the framing *this
line confines the crate's writes* was **overstated and is now cut** — nothing
re-examines the path between the draw and the child's write, `Path::exists` and
`read`'s `fs::read_to_string` both follow links, and a symlink could only stand
at a drawn name if 128 bits had been predicted, which is the condition
`DRAW_RETRY_LIMIT` already reports; *outside the directory the caller named* was
**wrong** and is replaced (a relative link resolves inside it); *the child would
call `signal`* was **overstated** — the child is an arbitrary program and the
crate's own tests use shell redirection; and the loop/`FilesystemLoop` case was a
**better argument the page did not have**, since it is a plain correctness gain
with no security framing at all. Each was re-verified directly before the
rewrite. Noise: the Unix-only caveat, which the book's `README.md` already fixes.
No second reviewer was materialised and no `review-draft` leaf was cut — the fix
narrowed an overclaim to measured facts and introduced no new unverified
assertion.

**3 · The chapter names `run`, not `watch`, for the far end of the through-line.**
The brief's through-line obligation is worded *`Channel::allocate` writing nothing
is why `watch` can treat appearance as an event at all*. `watch` is private and
chapter 8's, and it appears in no early-use row; naming it here would owe a new
row for a symbol the reader does not need in order to follow the claim. Line 38's
own reproduced bytes name `crate::run`, which the ledger already carries from
`01-orientation.md#the-cast`, so the page makes the claim with `run` as the actor
and points at chapter 8 for the poll itself. **No new early-use row is owed by
this chapter**: the only later-slice referent in the 271 reproduced lines is that
one `run` link.

**4 · Sixteen literal fragments, cut at the item, and the name grammar read as
one section.** The file is a thesis, three constants, one type with five methods,
a second type, one free function and four private helpers — sixteen items, and
the cut follows them, merging only `impl Channel {`'s opening line into
`allocate` and `Token`'s two accessors into `Token`. Fifteen sections over
sixteen fragments: `CHANNEL_PREFIX`, `NONCE_BYTES` and `DRAW_RETRY_LIMIT` are
three fragments in one section, because the brief treats *the name grammar* as
one responsibility and the three constants are that grammar's three numbers.

## Result

`book-check --repo . --book docs/walkthroughs/keyed-launch --through
appearance-is-the-event --check all` is **valid: 9 files, 1,229 resolved lines,
844 deferred lines, `final=false`** — the figures this leaf's *Done when* names.
One block moved from `deferred` to `resolved`: `channel-production` (271), over
sixteen literal fragments under one composite. `src/channel.rs` is now the book's
only partly-reconstructed root; its remaining 133 lines are chapter 9's.

`bash scripts/check.sh` exits 1: **FAILED — 1 of 8, and the one is `book-check`**,
as every child of this node but the last is expected to leave it. Under `--final`
the keyed-launch book reports the four unwritten pages (`M101`), the two
navigation and contents entries that name them (`M103` — one of which is this
page's own missing `Next` to `07-the-job.md`), the six source blocks chapters
7–9 own (`F003`, twelve findings), and the ownership and early-use rows still
`pending` for those chapters (`F009`, six findings). **No finding names
`appearance-is-the-event`**, checked by search rather than by reading. The other
seven checks — `cargo fmt`, `shellcheck`, `cargo clippy`, plugin install,
conformance, the conformance suite and the test suite, including
`every_repository_markdown_reference_resolves` and the corpus-inventory tests —
pass, measured after every edit including the review rewrite; the other four
books still validate `final=true`.

The early-use row this chapter owns — `Channel`, `Token`, `signal`, first used at
`01-orientation.md#the-cast` — is now `explained`, and it does not appear in the
`--final` `F009` list, which is the check that it was accepted rather than merely
edited. **No new early-use row was owed**: the only later-slice referent in the
271 reproduced lines is line 38's `[`run`](crate::run)`, which the ledger already
carries from the same anchor.

The glossary citation the brief requires is at `06-the-channel.md:37`, to
`CONTEXT.md#loop-control-channel`, at the chapter's first use of *channel* for the
thing grove's glossary names — the book's first and only use of its one declared
glossary anchor. Nothing outside the book was owed or touched.

**Nothing is handed forward and no leaf was cut.** The corpus was not edited. One
observation is worth a later reader's attention but is not a defect: the two
`std`-semantics claims about `allocate`'s occupancy test are pinned by no test in
this repository, and the page says so in those words. Adding one would be a change
to a frozen crate, so it is left rather than smuggled in.
