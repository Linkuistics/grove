# node-grammar-k3

## Goal

Cut the `impl` leaves that deliver the node grammar `NN-k<key>/_<slug>.md`
(root `_BRIEF.md`) end to end — the library, grove's grammar and verbs, the
skills and docs, the books, the cutover and release, the migration — each a
vertical slice that lands green on its own, ordered so that this repo's own
tree is cut over in the same session as the release that can read it, and the
other groves are migrated after that release is installed.

## Context

- The design at `node-grammar-k2`, reviewed if that session cut a chain.
- The root brief's *Notes*: the meta-grove's sequencing hazard and the list
  of five groves to migrate.
- `plan-k1` decision 9 — the agreed test seams; every leaf names which of
  them it is tested through.
- `docs/specs/walkthrough-books.md`, *An accepted source change*: a source
  edit and its book's fragments and prose land in one commit with
  `scripts/check.sh` green. A leaf that touches a book's root owes that book
  its update in the same commit — no leaf may leave a book red for a sibling.
- `docs/RELEASING.md` — the release procedure. The cutover leaf ends with a
  human: stop the loop, cut and install the release, update the plugin, rename
  this tree, re-run `grove`.
- This is a canonical-grammar cutover with no dual-grammar reader (`plan-k1`
  decision 7), so it is not an expand → migrate → contract refactor: each leaf
  lands the new spelling for its own surface, and the tree that the loop
  reads changes only at the cutover leaf.

## Done when

- Every `impl` leaf exists with a body that names its surface, the seams it is
  tested through, and the book (if any) it owes.
- The positions encode the order: library → grove grammar and verbs → skills
  and docs → cutover and release (the last code leaf, HITL) → migration of
  the four other groves.
- The cutover leaf's body states the human sequence above and says that the
  session ends by stopping the loop rather than by `grove-llm complete` into
  a tree the running driver can no longer read.
- The migration leaf's body lists the five trees, the check (open each with
  the new binary), and what to do with `grove.gh-issue-12/.grove/FORMAT`.

## Notes

- No naming history in anything a leaf writes (`plan-k1` decision 6).
- A leaf that cannot be demoed on its own is a horizontal layer; redraw it.

## Decisions (running log)

**1. Working increments and the agreed workstream boundary.** The useful
handoffs are the library's supplied-name/level-validation API, Grove's complete
grammar implementation, the installed methodology's source, the reader-facing
documentation, the release/cutover, and migration of the remaining trees. Keep
these as ordered leaves in this grove: `plan-k1` decisions 7 and 10 explicitly
put the migration and implementation in this tree, and this leaf's contract
fixes their positions through the human handoff. That specific scope takes
precedence over planning's general preference for separate groves per working
increment. No extra workspace or independently driven tree is needed.

**2. The library lands with every API consumer.** Supplied distinguished names,
their identity, read validation, projected-plan validation and conformance form
one usable library increment. Reference-domain consumers, adversarial fixtures,
the syllabus CLI and Grove's call sites move with that API. Grove keeps its
then-current filename behavior in this increment; the following leaf switches
its parser, handles, every verb and fixtures together. No compatibility API or
dual-grammar reader is introduced.

**3. Books follow their sources.** Each code leaf owns the fragments, ledgers,
indexes and explanatory prose for every source root it changes, discovered
from the book manifests. There is no later book-repair leaf. A separate
documentation leaf owns remaining prose-only examples and the user guide;
the methodology leaf owns the shipped skills and their conformance checks.

**4. Cutover is a stopped-loop handoff.** Prepare the release and a checked
rename plan before asking the human to stop loops and approve publication.
The cutover completes only after installation, plugin refresh, this tree's
rename and verification. It never signals `grove-llm complete` to the driving
20.2.0 process. The other four groves stay stopped across the global install
until the migration leaf verifies them; this grove is verified again there.
Temporary migration tooling stays outside the published codebase.

**5. Planning earns a review before implementation.** The decomposition spans
a public trait, whole-tree refusal behavior, byte-reconstructed books and a
machine-wide executable switch. Once the leaf bodies exist, insert a
`review-planning` step before the first implementation leaf, naming this
producer's handle. It will check these seams and the handoff; no in-session
reviewer is used and no integration is created speculatively.

**6. Verification and handoff.** All seven new handles resolve through the
installed reader, and the migration leaf's brief chain reaches the root charter.
`bash scripts/check.sh` exits 0: all eight principal checks pass, including
the 11 conformance controls, workspace tests and final validation of all six
books. The diff is confined to this task tree; no check input under source,
plugins or durable docs was changed during the run. `node-grammar-k12` is
placed before `distinguished-names-k6`. The root retains live review and
implementation work, so retiring this planning leaf closes no ancestor.
