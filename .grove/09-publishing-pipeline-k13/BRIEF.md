# publishing-pipeline-k13 — brief

## Goal

Turn what the pilot measured into installed machinery: decide from the evidence
which editorial stages become session kinds, then author those `grove-<kind>`
skills so the remaining books are written through a pipeline rather than by hand.
This is product P2, and the scale-out is gated on it.

## Done when

- The figure and asset question is settled from the pilot's verdict — either a
  contract the art kind and the validator implement, or a recorded rejection.
- An ADR records which stages became kinds, which were merged, which were
  dropped, and what the decision rule said about each. A stage that cannot be
  shown to have paid for itself is not extracted.
- The kinds exist as installed skills under `plugins/grove/skills/`, uniform with
  the nineteen already there, reachable on the loaded path, and covered by the
  conformance runner and the plugin install route.
- The human has the launch templates those kinds need, and has confirmed it.

## Decomposition

Three leaves, in evidence order.

1. `figure-contract-k18` — the art question, answered from the pilot's measure.
2. `pipeline-kinds-k27` — which stages become kinds, and what each one's
   discipline is.
3. `pipeline-skills-k28` — author the skills and wire them in.

The art question comes first because the answer to it is an input to the art
kind's discipline, and because it is the only one of the three that can legitimately
end in "nothing is built".

## Pointers

- `docs/adr/a-kind-is-an-open-token.md` — a kind exists **iff** a
  `grove-<kind>` skill exists; grove holds no list of kinds; and a kind for which
  no launch template resolves is refused before the tree is mutated. That last
  clause is why this node ends with a human gate rather than a green suite.
- `docs/adr/corpus-rules-have-one-owner.md` and
  `docs/adr/restatement-declares-its-class.md` — how a rule is filed between the
  spine skill and a kind skill, and what a restatement in `SKILL.md` may be.
  `CONTEXT.md`'s *Spine skill* / *kind skill*, *Family reference file*,
  *Loop-step reference file*, *Loaded path* and *Composed loaded path* entries
  carry the vocabulary.
- The existing kind skills are the shape to be uniform with:
  `plugins/grove/skills/grove-<kind>/SKILL.md`, one per kind, each opening with
  the imperative to load the spine.
- Assurance already in place: `plugins/grove/conformance.sh` and its own test
  script, `plugins/install.sh` and `plugins/install.test.sh`, and the
  `include_str!` content assertions in `crates/grove-llm/tests/` — several of
  which count or enumerate what the methodology ships and will need updating.
- `/Users/antony/Development/TheGreatExplainer` `docs/requirements.md` §1.6 is
  the prior art for the pipeline's feedback edges and human gates.

## Notes

**Scale-out is gated on this node and not on the loop construct.** Decision 12 of
`plan-k1` reads "the kinds and the loop", but only the extracted kinds are
machinery a book is authored through; the loop construct is a representation and
decision 17 already has it retrofitting as it lands. No book waits on
`loop-construct-k7`.

**Nineteen is a count of the methodology's kinds, not of grove's.** Adding kinds
changes prose in `CONTEXT.md`, `docs/ARCHITECTURE.md` and `TASK-FORMAT.md` that
states the current set and its arithmetic. Those are statements about the
installed methodology and they have to move together; a count left stale reads as
a description.

**A new kind cannot run until the human declares its launch template.** That is
`~/.config/grove/config.kdl`, or an untracked `.grove.kdl` delta — the human's
personal configuration, not a repository file. It is outside anything this node
can commit, and `crate-books-k14` is what breaks if it is missing.
