# reading-k19

**Reviews:** reading-k9

## Goal

Attack the reading layer before four mutation leaves are built on it. It is the
substrate: `Snapshot`, `Builder`, `Entry`, `Container`, the two guards and the
error type are what `interpreter-k10`, `insert-k11`, `promote-k12` and
`rewrite-k13` all consume, so a defect in its shape costs a subtree rather than a
patch. That is not a forecast — `seam-k17` is the measured precedent, recorded in
`docs/formalism-findings.md` entry 005: six findings, two of which changed the
public surface, and the entry says in as many words that taking them after the
dependent leaves would have been a rework instead of a fix.

## Context

Beyond the brief chain, and beyond `reading-k9`'s own `Context` section, which
lists the document sections and models this artifact answers to:

- The artifact: `crates/ordinal-fs-tree/src/snapshot.rs`, `src/error.rs`,
  `src/fs/` (`mod.rs`, `lock.rs`, `read.rs`), the two new test files, and the
  amended `tests/algebra_has_no_filesystem.rs`.
- `reading-k9`'s `## Decisions (running log)` — eight decisions, each with its
  reasoning. Those are the target: a stated reason is what an adversarial read
  can be *against*.
- `docs/formalism-findings.md` entry 006, and 005 before it. 005's third
  counterfactual is this leaf's method: **name the judgement to attack**, do not
  ask for a review.

## What to attack, named

Five judgements, in the order of what they would cost if wrong. Each was made
deliberately and each is defensible, which is exactly why a fresh reader is worth
paying for — the producer cannot be surprised by its own reasoning.

1. **The public shape the mutation leaves inherit.** `Snapshot` is an arena of
   names with `Entry`/`Container` handles borrowing it, guards `Deref` to it, and
   `Builder`/`Place` are public so a test can build a tree without a directory.
   Ask what `insert` and `promote` will actually need — a level's positioned
   children in ordinal order, an entry's parent, a fresh key from the whole-tree
   maximum — and whether this shape supplies it without a widening. A borrow that
   has to be dropped before a mutation can start is the specific smell.
2. **The walk-order tie-break.** No model checks it. On a level a hand edit left
   carrying a duplicate ordinal, the order is (ordinal, key, rendered name), and
   `by_key` answers the first of it. Attack the claim that this is *total* and
   the claim that it is *the right* total order; and attack the reasoning that
   the alternative — leaving it to listing order — is a real hazard rather than
   an imagined one.
3. **Halting on a filename that is not UTF-8.** A new refusal, in neither model
   and, until this leaf, in no document. The argument is the trichotomy's own: a
   name that cannot be read cannot be disclaimed. The cost is that genuinely
   foreign junk freezes the tree. Attack the direction of that trade, and attack
   the alternative — treating it as `Foreign`, which is what the library's own
   `Verdict` would do if it could see the name at all.
4. **The guard's new carve-out.** `tests/algebra_has_no_filesystem.rs` now
   exempts the `mod fs;` declaration shape. The claim is that this is narrower
   than it looks and that refusing re-exports makes the guard *stronger*. Attack
   the exemption's parser for shapes it wrongly admits, and attack the claim that
   no crate-root alias can now launder a filesystem item into the algebra.
5. **The lock, which no test can fully reach.** Blocking-only with no `try`
   variant, taken on the lexical parent, released by dropping a `File`, retried on
   `EINTR`. Two claims worth doubt: that a lexical parent plus kernel resolution
   really does give two spellings of one tree the same lock, and that a
   `libc` dependency was the right answer to `flock` not being in `std`.

## Done when

- Every judgement above has a verdict: sound, or a specific defect with the
  structure it admits.
- Anything found is a **finding**, not a fix — this session changes no code.
- The mutation controls are checked for the thing they cannot prove: each was
  written by the same session as the test it validates, so ask which of them
  would still fire against a *differently wrong* implementation.
- An entry in `docs/formalism-findings.md`, including if the answer is *nothing
  found* — the log records misses with the same care as hits, and a review that
  finds nothing is H2 evidence.
- An `integrate-review-impl` leaf beside this one **only if** there are findings
  worth acting on. Cut it with `leaf-insert` against the first later sibling
  still holding live work, because an integration consumes `path:line`
  coordinates that any intervening leaf silently moves.

## Notes

**The producer spent none of its in-session review allowance**, so nothing here
has been read adversarially yet. Every claim in the code comments and in the
decisions log is the producer's own.

**Read the tests as an artifact, not as evidence.** `seam-k17`'s sharpest two
findings were tests that cited a model claim and checked something weaker, and
005's counterfactual is to compare the assertion against the claim's *predicate*
rather than its name. Eighteen of this leaf's twenty-three tests say they
discharge no claim at all — check that each of those is honestly claimless rather
than a claim nobody looked for.

## Findings

### High — accepted spellings of one tree take different locks

`crates/ordinal-fs-tree/src/fs/read.rs:110` chooses the lock directory with the
lexical `root.parent()`, and `src/fs/lock.rs:63` opens and locks that directory.
That does not make equivalent spellings of the **root** converge on one inode.
The existing path-preservation test supplies the counterexample itself at
`tests/reading_on_disk.rs:270`: if the direct root is `syllabus`, the accepted
spelling `syllabus/02-linear-algebra-i2/..` reads the same tree, but locks
`syllabus/02-linear-algebra-i2`; the direct spelling locks `syllabus`'s parent.
The kernel resolves both paths exactly as asked, and they are two different
directories, so a writer through one spelling and a reader through the other do
not exclude each other. A final-component symlink admits the same structure.

This breaks the premise that the snapshot is read under the lock and, once
mutations land, can expose the intermediate duplicate ordinal/key states the
lock is meant to hide. The lock mutation control proves that *some* lock is
taken for one spelling; it cannot detect a differently wrong lock identity.
Resolve a lock-only path to the semantic tree's containing directory while
preserving the caller's spelling for reports, or explicitly refuse spellings
whose lexical parent is not that directory, and cover direct-versus-roundabout
and direct-versus-symlink contention.

### Medium — the filesystem guard's comment stripper can hide real Rust code

`crates/ordinal-fs-tree/tests/algebra_has_no_filesystem.rs:148` scans comment
delimiters without recognising string, raw-string, byte-string or character
literals. A valid algebra source containing `const OPEN: &str = "/*";` leaves
the scanner at positive block-comment depth and makes every later filesystem use
disappear; a `"file://"` literal can hide a use later on the same line. This is
an ordinary lexical context, not the deliberately accepted macro/re-export limit
the test header records. The direct-import and real-module mutations all start
outside such a context, so they remain green evidence for a narrower detector
than the asserted boundary. Use a Rust syntax-aware dependency check or make the
stripper lex literals, and add positive controls with comment-shaped text inside
each literal form.

### Medium — the key-before-name tie-break test does not distinguish the order

The implementation at `crates/ordinal-fs-tree/src/snapshot.rs:199` correctly
orders positioned siblings by ordinal, key, then rendered name. The fixture
claiming to hold that rule at `tests/reading_algebra.rs:107` aligns the two
tie-breaks: key `3` has name `alpha` and key `7` has name `beta`. An
implementation that drops the key comparison and orders by rendered name alone
produces the same expected sequence. The duplicate-key test cannot cover this
because the compared keys are equal, and the arrival-order test uses distinct
ordinals. The recorded distinguished-first mutation therefore says nothing
about this differently wrong implementation. Reverse the lexical/key preference
in the fixture (smaller key with a lexically later name) and retain the equal-key
pair for the final tie-break.

### Medium — a `Place` from another builder can silently name this builder's node

`crates/ordinal-fs-tree/src/snapshot.rs:80` represents a public `Place` as only
an `Option<usize>`, while `Builder::add` promises at line 130 to panic if the
place came from another builder. If both builders have a node at the same arena
index, the foreign place indexes the current builder at lines 148–154 and the
child is silently attached there instead. The panic happens only when the index
is absent or happens to name a non-node. This makes a public construction seam
that exists for algebra tests capable of building a different snapshot from the
one its caller described. Bind places to a builder identity and validate it, or
replace the public arena handle with a construction surface on which a foreign
place is unrepresentable.

## Doubt verdicts

1. **Public mutation substrate: mixed.** `Container::positioned`,
   `Entry::container`, and a whole-snapshot walk supply ordered siblings, parent
   level and the whole-tree key maximum without widening the consumer surface.
   An `Entry` borrow ending before an exclusive guard mutates is correct
   invalidation, not a defect. `Builder`/`Place` has the concrete cross-builder
   defect above.
2. **Walk-order rule: accept; its evidence is incomplete.** A listing-order
   fallback is a real machine-dependent hazard and `(ordinal, key, rendered
   name)` is total for on-disk siblings under the canonical-grammar obligation.
   The code implements it. The named test does not hold the key-before-name
   part.
3. **Non-UTF-8 halt: accept.** Given a `&str` parse seam, neither `Foreign` nor a
   domain refusal can be obtained without guessing. Halting is the sound side of
   the stated trade; widening the name seam would be a different design.
4. **Filesystem guard carve-out: mixed.** Refusing crate-root re-exports does
   close the alias it names, and the module-declaration exemption grants no
   filesystem item by itself. The comment stripper makes the wider boundary
   assertion false.
5. **Lock: reject.** Lexical-parent plus kernel resolution converges only while
   the aliasing components occur before the final root component. The accepted
   terminal `..` spelling already in the suite disproves the general claim.

The graph service was reachable but could not index this jj workspace: its
worker refused to start because active-daemon coordination could not be verified
inside the sandbox, and coverage queries consequently required unavailable
approval. These findings therefore use the producer commit's exact diff and
complete direct reads of every cited source/test file; no negative claim relies
on an empty graph result.
