# walkthroughs-k3

## Goal

Decompose the documentation arm and the publishing pipeline — products P1 and
P2 of the root brief — into leaves that each fit one session and each land
something independently useful.

## Context

The requirements are settled and must not be re-elicited; the root brief and
`plan-k1`'s decision log are the input. What is *not* settled is the shape of
the work, and that is this session's whole job.

## Done when

The subtree is cut, each leaf carries a body written by this session, and the
node briefs say why the split falls where it does.

## Notes

**Place this subtree with `leaf-insert`, ahead of `specification-capture-k4`.**
`leaf-add` appends at the parent's end, which would queue every book behind a
deep research pair and break the root brief's promise that the documentation is
never blocked.

**The critical path is the validator, not the prose.** No book can be proved
until the fragment ledger takes a per-book corpus instead of compiled-in
constants, so that work is cut ahead of the pilot rather than beside it. It is
also where a recorded rejection reopens: the existing book spec rejected a
sidecar manifest when there was one book, and with six the question is live
again. That is ADR-shaped work and probably earns a design leaf of its own.

**Sequence the wide rename expand → migrate → contract.** Relocating the
existing book under `docs/walkthroughs/` touches validator constants, a large
test suite, the crate README and the context map. One leaf per stage, each
landing green.

**The pilot is `jj-workspace` and its deliverable is two things.** A book, and a
measure of which editorial stages paid for themselves. The second is the point:
nothing currently distinguishes six stages from two, and a pipeline that cannot
be falsified is not evidence of anything. Record what each stage changed, not
merely that it ran.

**The stages come from prior art already on disk.** `TheGreatExplainer`'s
requirements §1.6 specifies the pipeline with its feedback edges and human
gates; `Writegood` carries a judging protocol and the argument that measurement
must precede machinery; `grove.gh-issue-12` is a worked example of a
preregistered evaluation with arms, controls and adjudication. Read them before
inventing a stage list.

**Each book takes a human-authored structure brief as an input artifact.** The
source does not carry audience, conceptual order, or what deserves emphasis.
Cut the leaf that asks for it; do not have a session invent it.

**The art phase is new ground.** The existing book has no diagrams, no figures
and no captions anywhere, and the validator has no concept of an asset. Format,
convention and validation are all unbuilt.
