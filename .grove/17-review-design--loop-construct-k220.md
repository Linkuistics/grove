# loop-construct-k220

**Reviews:** loop-construct-k7

## Goal

An adversarial read of the pass-series design: `docs/specs/pass-series.md`,
`docs/adr/iteration-is-a-node-of-nodes.md`, and the **Pass series / pass** entry
added to `CONTEXT.md`. Findings, not fixes.

## Context

The producer designed **against a human's proposed shape and departed from it**,
which is the first thing to contest. The human proposed *a directory carrying a
marker token*; the design keeps the directory, drops the marker, and argues the
mark has no reader. If that argument is wrong the whole record is wrong, so read
the six objections in the ADR's trade-off section one at a time and ask which
would survive a reader arriving — and whether the design should have been written
to accommodate one rather than to reject it.

Four places this design is most likely to be wrong, named because the producer
could not check them from inside:

- **The pass/correction-run distinction is the load-bearing seam and it is
  new.** *A pass is a repetition the series declared in advance; a correction run
  is one the finding session decided.* Everything else rests on it — which
  repetitions get a directory, which reuse
  `a-feedback-edge-is-forward-tree-growth`. Is it decidable by a session at the
  moment it must decide, or does it require knowing why an earlier session acted?
- **A pass node's brief has two writers with different jobs.** `leaf-decompose`
  moves the decomposed leaf's body in as `BRIEF.md`, so the pass's charter is
  written by the previous pass's last step. But `references/editorial.md` also
  puts a running `## Handed forward` list in a document node's brief. Under a
  series those are the same file at different levels. Check the design does not
  quietly require one brief to be both.
- **The one-step pass has no directory, which makes the shape non-uniform.** A
  series of one-step passes is a run of sibling leaves; a series of two-step
  passes is a run of directories. A reader of `find .grove` sees two different
  pictures for one construct. Is the exemption right, and does anything in the
  spec break at the boundary?
- **The declarations are unenforced and the spec says so.** Test whether *says
  so* is enough: enumerate what a session could get wrong (a cap chosen after
  pass 1, a sequence never written, an escalation skipped) and ask whether each
  is detectable by any reader at all, including a human with `find`.

**Three claims are measurements and go stale.** The producer reported them as a
frozen reading, deliberately: zero repeated `(parent, kind, slug)` triples in the
tree; one `copy-edit`/`art`/`proof` per book across four books; 36 of 150 leaves
under `crate-books-k14` allocated after `proof--grove-loop-k182`. **Re-derive
them rather than reading them** — the producer's own leaf added entries after
taking them, and the extractor is a shell pipeline whose controls are described
in `loop-construct-k7`'s running log rather than committed as a script.

**One claim about another repository is unverified.** The spec's Problem section
rests on this grove's tree only. The producer looked at `Writegood`'s tree and
found no iterative shape there either, but did not survey `grove.gh-issue-12` or
`TheGreatExplainer`, which the root brief names as live in-house prior art. A
counter-example in either — a loop somebody already ran in a grove — is the
strongest single finding available here.

## Done when

Every finding is written down with the evidence for it, and the read has reached
a verdict on the marker-token rejection specifically: sound, or unsound and why.
Cut an `integrate-review-design` leaf only if there are findings worth acting on.
