# chain-construction-integrate-k40

**Kind:** integrate-review-design

## Goal

Apply **chain-construction-review-k39**'s findings to the design from
**chain-construction-k38**, and leave the outcome in its durable home — the
spec, the ADR set, the guidance surfaces, or `grove-llm` itself, as the design
concluded.

## Done when

- Every finding from k39 is **dispositioned**: applied, or explicitly declined
  with a reason. A declined finding is a legitimate outcome and is recorded as
  one, not silently dropped.
- The durable record k38 nominated exists and is coherent —
  `docs/specs/task-kind-taxonomy.md` reworked in place if that is the home, or
  the ADR set reworked as a minimum coherent set (never a superseding ADR), with
  every citation reconciled.
- If the design landed a `grove-llm` verb: it is implemented, tested, and the
  property that matters is **falsified by mutation** rather than asserted —
  this repo's standing bar (*jj-first-coverage-k6*, *codex-grant-refused-k35*,
  *guard-loop-signal-k37*). Whatever guidance surfaces name the old hand-cut
  procedure are updated to name the verb.
- If the design landed prose only: the three cutting-time surfaces
  **compose-task-chains-k29** identified (the bootstrap prompt, `SKILL.md`'s
  Decompose step, `TASK-FORMAT.md`) carry the strengthened wording, and the
  change is described in terms of what a session now *reads at the moment it
  cuts*, not what the reference material says.
- Anything reaching real sessions only through a release is called out as such —
  this repo's recurring in-the-tree-not-in-the-binary gap — and the CHANGELOG
  entry is written even if the release is not cut.

## Notes

Cut together with k38 and k39 as one chain, per **chain-group-unit-k36**'s
operational habit.

The chain's own history is evidence for whatever it concludes: if a mechanism
lands, note whether *this* chain would have been cut differently under it.

## Disposition

All three findings **applied**; none declined. Both verbs are implemented,
tested and documented.

### F1 — one CLI call is not yet one reliable mutation → applied in full

The composite verbs are **not** `leaf_add` in a loop. `tree_grow::add_run` is the
shared engine and implements every clause the review asked for: all slugs
validated and the parent resolved before the first write; positions and keys
allocated from **one snapshot**; every destination checked free before any is
written; a mid-write failure **rolled back** with the error naming any residue it
could not remove; stdout buffered and printed only on success.

The review asked for "all-or-nothing *or* a documented recoverable partial", and
all-or-nothing was the natural promise, so that is what landed. Two arms, tested
separately, because the up-front sweep and the rollback cover different failures:

- **The sweep** is provoked by a *directory* wearing the third leaf's name — the
  tree reconciles parsed names against real filesystem kinds, so it is invisible
  to allocation and still blocks the write. Exactly the gap between the parsed
  tree and the filesystem that a sweep exists to cover.
- **The rollback** is provoked by an over-long third name, which turns out to be
  a hazard **specific to composite verbs**: the derived names are longer than the
  stem the caller validated, so `<stem>-integrate` can cross `NAME_MAX` while the
  first two clear it. `Path::exists` reports false for an over-long path, so the
  sweep waves it through and the failure lands mid-write.

Falsified by mutation, per this repo's bar: deleting the sweep and the rollback
fails exactly the three atomicity tests and nothing else. A fourth test pins the
consequence the machinery is *for* — a retry after a refusal gets the positions
and keys the failed run had planned, rather than appending a duplicate shape.

### F2 — the pair verb does not establish a vendor pair → applied, by a third route

The review offered two remedies: compare both producers' harness facts, or weaken
the verb to a syntactic template and drop the reliability claim. It ruled the
second insufficient, and the first has a trap the finding did not name: producer
A's harness is not a *fact* to compare against. It resolves through leaf → kind →
family → stamp **at launch time**, so comparing it at construction compares an
argument against a **forecast about the environment** — the same inversion
**session-leaf-binding-k28** found in the driver's routing peek, answered there
the same way: the tree wins.

So both producers are **declared**: `--harness-a` and `--harness-b`, both
required, refused when equal. That satisfies the finding's own requirement
("persist and compare both producer harness facts, rejecting an equal pair before
writing") in the only way that makes the property durable — two `**Harness:**`
lines in the tree, and a comparison between two *arguments*, which reads no env,
inspects no tree, and cannot decay when routing policy changes after the leaves
are cut. Declaring A costs nothing it did not already cost: a leaf naming the
stamped harness is not a reroute, so the unscoped model keys still apply.

What the caller gives up is *"the usual vendor, and a different one"* — and that
is the point, because that sentence is not expressible as a fact about a leaf.
`content/driving.md`'s claim that undeclared A "falls through to the grove's
stamp" is gone with the procedure it described.

Falsified by mutation: undeclaring A and permitting an equal pair fails four
tests across the library and the CLI.

### F3 — the normative ADR enumeration disagrees with its adopted option → applied

Both verbs are in ADR *cli-binary-split*'s enumeration, landed in the same change
as the implementation, so the ADR never described a roadmap. While there, the
*Considered options* entry gained the correction the design already conceded: its
closing generalisation *"the verbs stay one leaf per call"* does not survive and
was already false when written (`root-init`, `leaf-decompose`). The rule is leg 3
— the caller decides *whether*, the verb decides *what* — not a file count.

### Named-risk verdicts

The two clean bills (constraints 3 and 5) are unchanged by the implementation:
`leaf-add` is untouched, no tree is inspected, no chain is linted, and every
refusal is authoring-time argument validation with a human present.

The review's one **conditional** verdict — encounter at cutting time, "clean bill
on the design, conditional on k40" — was the binding one, and its condition was
*replace the hand-cut procedure, do not append to it*. Applied that way in all
five surfaces: `content/SKILL.md`'s Decompose step and `content/TASK-FORMAT.md`
now show the one-call form where they showed three `leaf-add` lines,
`content/driving.md`'s two worked examples are rewritten (the review-chain one
kept a hand-cut form **only** for the retrofit case, which has no verb by
design), `content/prompts/start.md` names both verbs, and `docs/grove.md` states
the derivation rather than the encouragement. Both verbs sit immediately after
`leaf-add` in `grove-llm --help`, which is pinned by a test — placement is the
design's whole answer to k29's failure mode, so it is asserted rather than
assumed.

Constraint 4 stays **lightly strained, not violated**, and the spec says so
outright rather than arguing it away: the mechanism was adopted on reasoning
about the error class, not on a measured incident.

### Would this chain have been cut differently?

**No — and that is the honest reading.** `leaf-add-chain . chain-construction
--kind design` emits `chain-construction`, `chain-construction-review`,
`chain-construction-integrate` with kinds `design` / `review-design` /
`integrate-review-design` — which is byte-for-byte what the three hand-cut
`leaf-add` calls produced. A maximally-primed session (cutting a chain about
chains, immediately after k36 concluded on chains) got the transcription right.

So this chain is evidence for the **friction**, which k38 already judged thin,
and *not* for the error class the verb actually closes. The verb's case rests on
the derivation being one a session can get wrong in a way nothing downstream
catches — not on this chain having gone wrong. It did not.
