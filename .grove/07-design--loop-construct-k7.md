# loop-construct-k7

## Goal

Design how an iterative loop is represented in a grove — in the filesystem
view first, because that is the minimal UI — and settle what it costs in the
name grammar, the lifecycle and the recorded decisions it reopens.

## Context

Commissioned by `plan-k1`, decision 13. Cut deliberately *after* the pilot has
run editorial cycles by hand, so the construct is designed against a loop that
actually ran rather than one imagined in advance.

Today grove has no loop and no rendering surface of any kind: `.grove/` read
with `find` is the entire UI, and a review chain's iteration is emergent — a
session cuts its own follow-on with `leaf-insert`.

## Done when

The filesystem representation is settled and recorded, with the reopened
decisions either amended or explicitly kept; whether any code changes, and
what, follows from that and is not assumed here.

## Notes

**The human's proposed shape, to design against rather than from scratch.** A
loop is a **directory carrying a marker token**, holding the steps of the loop;
inside it, **numbered subdirectories, one per pass**, holding that pass's
artifacts. Two properties are already good and worth preserving through any
revision:

- `pick` is a pre-order walk that descends a node in place, so numbered pass
  directories are visited in pass order with **no change to selection at all**.
  The iteration order falls out of the walk that already exists.
- `find .grove` shows which passes ran and where each stopped, with grove
  uninstalled. That is constraint 6 satisfied by construction, and it is the
  test the representation exists to pass.

**What the design must resolve.**

- **A node is never marked, and there is one node species.** Every node today is
  a leaf that proved bigger, carries a `BRIEF.md`, and is created by
  `leaf-decompose`. A marker token creates a second species and puts an
  outcome-shaped infix where the grammar has only ever had one on leaves. Either
  the marker is not an infix, or the one-species rule changes and says so.
- **What "the steps" are at the loop level.** A declaration the passes
  instantiate, or leaves in their own right? `.grove/` holds task files and
  never the work product, so "the artifacts of each pass" must mean that pass's
  task files unless something else is intended.
- **Keys are permanent and never reused**, so pass two's `edit` is a different
  leaf from pass one's. Confirm the key allocator behaves across a growing
  loop.
- **The exit rule and the cap.** `docs/review-yield.md` measured nine review
  chains and honestly reported that it could not read a yield curve — so
  diminishing findings per pass is *not* an available stopping rule on this
  repository's own evidence. Whatever the exit rule is, it is not that.
- **The reopened records.** *One task is one session*, *entries are never
  removed*, and the outcome-partition decisions. A loop that re-runs work
  touches all three, and they are reopened deliberately, not incidentally.
- **The name grammar is a seam, not a detail.** The filename grammar is
  `grove-loop`'s `task_name` module over the tree library's entry name, and the
  `--` separator is what makes a name have exactly one reading with no set
  consulted. A marker token must not reintroduce ambiguity that separator
  removed.

**Not now:** a render verb, a terminal surface, or the DevTUI direction. The
filesystem shape is settled first, and the rest is on the root brief's horizon.
