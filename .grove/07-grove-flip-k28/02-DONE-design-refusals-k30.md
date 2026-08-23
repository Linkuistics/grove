# refusals-k30

## Goal

Settle how a library refusal reaches a grove operator, and write the
**reachability table** for grove's verb set before any verb flips. Two outputs,
one leaf, because the table is what turns the decision from a preference into a
count of cases that actually occur.

No verb changes here. The deliverable is a decision with a durable record and a
table the migrate-stage leaves transcribe.

## Context

- `docs/adr/entry-name-is-the-only-seam.md` — the *It costs a second thing*
  paragraph, and the sentence that names this leaf's condition: **reopen if a
  domain appears whose own vocabulary collides with the library's rather than
  merely differing from it — grove is that domain.**
- `docs/ordinal-fs-tree/CLI.md` — *Which refusals this CLI can reach* is the
  table to copy the shape of, and *What `cli-k16` found* is the case, argued from
  a real consumer.
- `docs/formalism-findings.md` entry 017, which **measured** what a second
  wording costs, and entry 019, whose counterfactual is *write the reachability
  table at design time*.
- `crates/ordinal-fs-tree/src/plan.rs` — `Refusal` and its variants;
  `crates/ordinal-fs-tree/src/error.rs` — `Error`, and which variants are generic
  over the name type. `Malformed` and `Reserved` carry `EntryName::Err` and
  therefore already speak grove's words; `Refused` carries `Refusal` and does not.
- grove's `CONTEXT.md`, for *Leaf* and *Node directory* — the two terms that
  collide.
- `src/llm_cli.rs`, for what grove's verbs currently say when they refuse.

## Done when

- **Question 3 is answered**, among: grove prints the library's refusals
  verbatim; grove renders them itself in its own vocabulary; or the seam ADR is
  reopened and the library's error surface changes. The answer is recorded where
  it will outlive `.grove/` — an ADR if it constrains future work, and an
  amendment to `entry-name-is-the-only-seam.md` either way, since that record
  currently ends by naming this leaf's question as open.
- If the answer is to reopen the seam, this leaf **cuts the library-side leaf**
  rather than doing it: a change to a checked library's public error type is not
  a side effect of a flip.
- A refusal-reachability table exists covering every grove verb: each `Refusal`
  variant and each non-`Io` `Error` variant marked reachable or not reachable
  from grove's argument surface, with the argument that reaches it. It lives
  where the migrate leaves will read it — grove's `docs/ARCHITECTURE.md` or a
  document beside it, not in `.grove/`.
- grove builds and the whole suite passes; nothing has changed in `src/` unless
  the answer was to re-word, in which case only wording has.

## Notes

**The collision is worse than the syllabus's foreignness, and that is the whole
question.** `syllabus lesson-add 4 sections` answering *"the entry with key 4 is a
**leaf**, which holds nothing. Children go in a **node**"* is merely foreign — that
domain has lessons and modules and no leaves at all, so an operator reads it as
library vocabulary. In grove the same sentence is *actively misleading*: grove's
`Leaf` is a task file and its `Node directory` is a subtree, so an operator reads
a true statement about regular files as a false statement about their tree.

**Entry 017 measured the alternative going wrong**, so re-wording is not the
obvious answer it looks like. Whatever this leaf decides, it should say what it
expects to pay, in the same terms 017 used, so a later reader can check the
prediction.

**A third option worth pricing rather than assuming away**: grove renders only
the variants that *collide* and prints the rest verbatim. That is smaller than
re-wording everything and larger than nothing, and the reachability table is
exactly what tells you how many variants that is — which is the reason the two
halves of this leaf belong together.

**The table's payoff is mechanical.** `cli-k16` reported that its table predicted
its suite exactly: every refusal marked reachable ended with a contract test
naming its model witness, every one marked unreachable with none, and not one was
reachable after all. That turned an implementation leaf's test suite into a
transcription. The migrate leaves here get the same deal, which is why this leaf
runs before them and not alongside.

**Watch for the variant grove reaches and the syllabus could not.**
`DestinationOccupied` and `KeysExhausted` were reachable there only on a
hand-edited tree; grove's trees *are* hand-edited — the methodology says so. And
`ContentForANode` was discharged there by the verb set; check whether
`leaf-decompose` can hand a node bytes.

## Decisions (running log)

**The table is written against grove's *argument surface*, not against the
library's operation set, and that is what makes it small.** Every grove verb that
mutates has a precondition the library cannot see — an outcome infix, a session
kind, `finish`-reservation, brief-ness — so grove must classify its target before
it calls. Once it has, the library's own species refusals are unreachable behind
checks grove needs anyway. Four `Refusal` variants of ten are reachable; one of
them from an ordinary argument, three only from a tree a hand edit or a failed
rollback has damaged.

**Question 3 is answered: grove prints the library's refusals verbatim, and does
not re-word any of them.** Not because the collision is imaginary — it is real
and this leaf composed the offending message in grove's own vocabulary to
measure it — but because the reachability table shows the collision reaches an
operator through exactly **one** argument, `leaf-add`/`leaf-add-pair` given a
task file as `<parent>`, and grove already refuses that itself and must keep
doing so for the brief case the library cannot be handed at all. What survives
verbatim is the set whose words are already true of a grove tree: a duplicated
key, an exhausted counter, a failed rollback. The seam ADR is **not** reopened
and no library-side leaf is cut.

**No new ADR.** The AND test fails on *hard to reverse* — re-wording later is a
match arm per variant, not an audit of unwritten assumptions the way
`task-names-are-canonical` is. The durable record is three edits to artifacts
that already exist: the table and the pre-empt rule in `docs/ARCHITECTURE.md`,
the amendment closing `entry-name-is-the-only-seam.md`'s reopening clause, and
the two missing terms in `CONTEXT-MAP.md`'s grove→library relationship. A fourth
record restating them would fail `ADR-FORMAT.md`'s minimum-coherent-set rule.
This also matches the precedent inside this workstream: `cli-k16` recorded the
identical decision for the syllabus in `CLI.md`, a design document, not an ADR.

**The translation lives in `CONTEXT-MAP.md`, not in either glossary.**
`CONTEXT-FORMAT.md` forbids one term living in two glossaries, so a bridge from
the library's words to grove's belongs to the relationship between the contexts.
The map already carries two of the four pairs; this leaf adds the two an operator
actually meets in a library message and the map did not have — *distinguished
child* and *entry*.
