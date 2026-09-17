# plan-k1


## Goal

Establish the requirements for removing support for non-modular configuration.

## Context

The current modular configuration spec deliberately supports flat top-level
entries and mixed documents. The generic reader and resolver implement that
compatibility, and tests and packaged examples exercise it.

## Done when

- The removal boundary and retained modular behavior are recorded in the root brief.
- The human's expected test coverage is recorded without unresolved questions.
- Concrete implementation leaves carry the fixture migration and final removal.

## Notes

No full grilling is needed: removal of the old format is pre-decided, and no
set of three interdependent product questions has surfaced.

## Decisions (running log)

The human's requested scope is: "Remove support for the non-modular config."

The code and language contract identify that format as flat top-level kind-to-
template entries, including flat local overrides and documents mixing those
entries with a modular wrapper. Simple modular configuration needs neither
profiles nor parameters; that existing behavior remains in scope to preserve.

The human narrowed the initial seam proposal: "We just need to make sure the
config load/expands tests work." Acceptance therefore uses the existing public
configuration load/expansion tests, with fixtures adapted to modular syntax.
No new CLI/fake-executable acceptance suite is requested. Existing downstream
tests still need valid fixtures so their unrelated contracts continue to run.

The migration spans existing runner, adapter and CLI fixtures, with source-exact
walkthroughs coupled to production changes. The bootstrap therefore cuts two
implementation increments directly: `modular-fixtures-k2`, then `modular-only-k3`.
The current modular reader supplies the expansion stage; no new design or
planning session is required. The remaining work is implementation.
