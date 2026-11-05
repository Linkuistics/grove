# engineering-pack-plan-k178

## Goal

Cut the implementation leaves of engineering-pack-k177.

## Done when

- Each Done when of the parent brief belongs to a leaf with a concrete,
  verifiable outcome.
- The leaves follow the expand and contract order of the contract. Each
  handoff leaves Grove's `bash scripts/check.sh` green.
- The consumers of `grove-impl` in Grove's tests and trial cases have their
  own leaf before the removal.

## Decisions (running log)

- The node is one move with three leaves: the consumer step (k202), the
  expand (k203) and the contraction (k204). The membership decision gives
  this move the base proofs only. The node therefore has no trial leaf.
- At planning, three conformance controls copied the real `grove-impl`:
  `unreachable`, `kind-without-load` and `two-kinds-one-rule`. They are the
  only named reads of a moving kind outside the removal's own list. Core
  still owns kind rules in `grove-finish` and `grove-planning`, so
  `two-kinds-one-rule` has a core subject.
- The other files that name `grove-impl` are fixture labels or evidence. The
  trial cases stay unchanged, as the membership decision states. k202 still
  runs a scratch removal, because a clean search does not prove that no
  check reads a moving file.
- The doubt check goes into the first pack commit (k203). The Grove halves
  of the eight doubt tests stay, and their skill reads leave in the removal
  (k204). Grove's suite then never loses the check between two commits.
- The parent's Done when maps to the leaves as follows. The layout, the
  openings, the `LICENSES/` file, the craft bytes, the doubt check, the first
  commit and the tool requirements belong to k203. The test and script
  consumers belong to k202 and k204. The kind table, the rows' removal, the
  marketplace pin, the `plugins/linkuistics` cleanup, the aggregate and the
  second commit belong to k204. Each leaf appends to the moves record.
- No moving craft skill names a plugin-qualified skill. The kinds keep the
  `grove:grove` opening, which k203 changes to the bare-name text.
- No in-session reviewer and no `review-planning` leaf. The subtree follows
  the pattern of two finished moves, and the membership specification had its
  review.
