# current-state-documents-k68

## Goal

Rewrite the design-tense artifacts of harness-dispatch as current state: the
spec, the visual design README and its viewer text, the runtime evidence, the
ADRs and the glossary.

## Context

`acceptance-walk-k67` ran first. It found a passing instrument for every
acceptance case and every seam clause, so the spec may describe the whole
command as current state and nothing stays hedged. Its running log names the
test for each clause, which is the place to look when a sentence of the
notice claims that a suite shows something.

That log also lists where a test holds its clause more narrowly than the
spec's `#test-seams` table words it, and `seam-controls-k70` closes the ones
worth closing. Do not reword the table to match the narrower tests: the
table is the contract, and k70 moves the tests.

What k67 saw while sizing this leaf, as a starting list and not a complete one.
Sweep each document whole, since a list of patterns is complete only as far as
the list.

- The spec opens with a notice of over a hundred lines, from "This is the
  first-release design" to the sentence about the visual document. It lists
  what is delivered, increment by increment. Before deleting it, check each
  sentence for a contract fact the body below does not state. Much of it
  describes what a test suite shows, which `#test-seams` already carries.
- The spec's body still speaks as a design in places. `#grove-integration`
  opens with an instruction, "Add optional whole-argument slots", and ends
  "Usage and configure-grove must explain". `#execution-contract` has "The
  implementation must therefore record". `#delivery` has "Use Bun 1.4.2
  initially", "Each existing archive gains" and "These are
  implementation/release acceptance requirements, not results of this design".
  `#out-of-scope` says "this increment". `#records-and-outcomes` has "a later
  increment supplies".
- Some sentences are history, not hedges: "The `Worker` class in the table
  above was reported here until it was seen to fire", "which is why
  package.json autoloading stayed off until the worker moved" and "The move
  gave that one up to close a reach that was open as shipped". Decide for each
  whether the reason it gives is still needed as current state. The runtime
  evidence is where what was seen belongs.
- The visual design README calls itself "a design for independent review, not
  implemented product behavior", lists "Current changes" from review k9, and
  ends by saying Linux execution, packaging and cancellation tests "belong to
  implementation and release". `diagrams.json`'s `intro` says "nothing here is
  implemented" and names k9 and k10. Check the `.mmd` captions and
  `index.html` too.
- The runtime evidence says, under *Source grounding*, "The three new
  lifecycle slots are proposed, not existing API". Its earlier sections are
  dated observations, which stay as what was seen.
- `docs/adr/harness-selection-is-owned-by-policy.md` has "initially ships" and
  two "follow-up work" sentences. Read all three dispatch ADRs whole:
  that one, `policy-evaluation-precedes-process-replacement.md` and
  `a-review-carries-its-creator-reference.md`. `ADR-FORMAT.md` governs how a
  set is reworked.
- The glossary's dispatch entries, from *Review target diversity* to
  *Delivered context* in `CONTEXT.md`, already read as current state. Confirm
  that, and change nothing that is already right.

No behavior changes here. Tests pin some of these words: the conformance rows,
the composition-guidance pins, and
`the_documented_command_definition_for_dispatch_is_the_one_launched_here`.

## Done when

- The spec carries no "not implemented" notice and no design-era hedge. It
  describes the delivered command as current state, and every anchor another
  document links to still resolves.
- The visual design README, the viewer's text and the runtime evidence are
  current.
- The three ADRs and the glossary entries carry no design-tense wording.
- `task check` passes.

## Notes
