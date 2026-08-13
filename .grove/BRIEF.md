# grove.continue-prompt-can-specialise-grove-llm-complete-instruction — brief

## Goal

Land the **mandate composer** — the per-kind selection of triggering units that
`docs/specs/mandate-delivered-methodology.md` designs — and use the
session-ending instruction as its first real consumer. The driver already knows
the launched session kind, so a mandate that still tells a session "run
`grove-llm complete`, unless you are a `finish` leaf, in which case `--done`" is
making the session re-derive a fact the driver resolved before the session
existed.

Two things ride on that, and they are different in nature:

1. **A delivery change** — compose `${prompt}` per kind, reduce the launcher to
   framing, and scope the ending instruction so each kind is told exactly its own
   ending and never an exception to someone else's.
2. **A methodology change** — a `finish` session that externalises work instead
   of tearing down currently has no documented way to keep the loop running. The
   mechanics already support one; nothing says so.

## Done when

- The driver composes each session's mandate from triggering units whose scope
  admits the launched kind, byte-exact, joined by blank lines, runtime facts
  last (spec: *Requirement: Triggering units reach every kind they are scoped
  to*, *Requirement: Slices are byte-exact*).
- No composed mandate contains a branch on session kind. Every kind's mandate
  states one session ending, and the eighteen non-`finish` kinds never see
  `--done`.
- A `finish` mandate names its endings as **outcomes** and never mentions
  another kind's rule.
- The reopened-finish ending is documented: a `finish` session that externalised
  work signals a plain relaunch.
- Each of the nineteen composed mandates carries exactly one session-ending
  instruction, asserted by test — so a twentieth kind fails loudly instead of
  launching sessions that never signal the loop.
- Global skill provisioning is **untouched**. Both delivery paths are live at the
  end of this grove; retiring provisioning is the next increment, and the spec
  already sequences it that way.

## Decomposition

- `specialised-ending-k2` **design** — record the four decisions below in
  `docs/specs/mandate-delivered-methodology.md`, and agree the test seams. The
  spec already carries the composer's design; what is new is the launcher's
  reduction, where the ending specialisation lands, how the complement scope is
  spelled, and the reopened-finish ending.
- `composer-k3` **planning** — cut the composer increment into vertical slices,
  each leaving the product working.

Design precedes planning because three of the four decisions change a shipped
spec, and slicing work against an unamended spec would slice against the wrong
contract.

`composer-k3` cut the increment into three slices, in this order:

- `composition-k7` — the composition function on the `methodology` seam and the
  file-ordering directive it consumes. Nothing consumes the composer yet; this
  is the fault line the planning leaf named, so a slice that only adds a
  function is separable from one that changes what sessions receive.
- `mandate-delivery-k8` — the driver composes by the kind it already resolved,
  and `content/prompts/continue.md` becomes `content/MANDATE.md`, framing only.
  One change of what a session receives, not two: the launcher's instructions
  are removable exactly when composition delivers the units that duplicate them.
- `session-ending-k9` — the ending specialisation itself (D2, D4) and its
  all-nineteen guard. It follows the slice that selects by kind, because
  splitting an ending while nothing selects would ship both endings to every
  session.

`specialised-ending-k2` externalised one further leaf, `unit-scope-audit-k4`: a
grilling on whether the `kinds=*` default should be narrowed anywhere beyond the
session ending. It is a **separate increment and must run last** — nothing in it
is measurable until mandates are composed per kind. The three slices above sit
ahead of it; any further leaf for this increment must `leaf-insert` ahead of it
too rather than appending after it, or it will sequence behind an audit that
cannot start.

## Decisions taken during grilling

**D1 — the launcher becomes framing only.** `content/prompts/continue.md`
becomes `content/MANDATE.md`, one `kinds=* class=triggering` unit stating what
the mandate *is*: methodology sliced for this session's kind, complete with
respect to triggering conditions, with `grove-llm methodology <id>` serving any
deferred body. It carries no instructions. All five of its current sentences
duplicate a `kinds=*` unit that composition already delivers — `skill-bootstrap`,
`skill-decompose`, `skill-commit`, `skill-signal`, `skill-finish` — and its first
("use the grove skill") goes false when provisioning retires. A duplicate inside
`content/` is not build-boundary drift, but it is a second source of truth with
nothing keeping it in step, and it costs the reader a decision: *do these two
statements agree?*

**D2 — the ending is specialised where the conditional lives, not only in the
launcher.** `skill-signal` states both endings at `kinds=*`, so deleting the
launcher's copy alone would leave the session resolving the same `if` one slice
later. `skill-signal` and `skill-finish` are re-scoped so the eighteen non-`finish`
kinds get the relaunch ending and nothing else, and `finish` gets its own. One
sentence stays at `kinds=*` as a negative trigger — a session never discovers a
grove is finished; the driver tells it by launching a `finish` session — because
withholding that yields an unasked question against a destructive action.

Scope is bounded to the **session-ending instruction**. A wider audit of every
unit's scope is a separate concern; externalise it if it surfaces.

**D3 — "every kind but `finish`" is spelled as the explicit eighteen.** Not by
reopening `kinds=` for negation: `mandate-delivers-the-methodology.md` admits
that only for a replacement preserving fail-on-kind-addition, which negation does
not. The explicit list's own hazard is the mirror one — a twentieth kind silently
*omitted*, so its sessions never signal the loop and the driver stops on every
one of them — and it is closed by test rather than by grammar (see `Done when`).

**D4 — a `finish` session that reopens the grove signals a relaunch.** The
methodology gives a `finish` session two exits, `--done` or no signal, while
`skill-decompose` at `kinds=*` tells it — like every session — to externalise
surfaced work. A session that does so cannot tear down: ordinary work is live.
The mechanics already handle it and nothing documents it:

| what the session did | ending |
|---|---|
| teardown completed | `grove-llm complete --done` — the loop stops |
| externalised work instead | `grove-llm complete` — the loop relaunches and picks the new leaf; the sentinel waits |
| declined, or no human present | no signal — the loop stops, the leaf stays live and resumable |

The distinction that makes this consistent with D2 rather than a relapse: the
driver can resolve every branch that depends on the **kind**, and cannot resolve
one that depends on **what happened during the session**. Specialisation removes
the first. The second must stay, or it becomes an unasked question.

## Pointers

- ADR: `docs/adr/mandate-delivers-the-methodology.md` — the delivery decision,
  the `kinds=` grammar's rejected alternatives, and the completeness invariant.
- Spec: `docs/specs/mandate-delivered-methodology.md` — the composer's design,
  already written: composition order, the module seam, the 64 KiB alarm, the
  file-ordering comment directive ("arrives with the composer"), and the
  `content/MANDATE.md` rename. The design leaf **amends** this rather than
  writing a new spec.
- Glossary terms in play: *Mandate slice*, *Triggering unit / procedural unit*,
  *Methodology unit*, *Complete finish cycle*, *Session kind*, *Kind routing*
  (`CONTEXT.md`).
- Seam: `methodology`, exposing parse over `(path, text)`, the embed's unit set,
  and composition over `(units, kind)`. The spec already agrees it; the composer
  adds the third function to an existing seam rather than a new one.
- Code the grilling verified, cited for the claims that rest on it:
  - `src/loop_driver.rs:195` `mandate_prompt` — emits the launcher file whole
    via `provision::continue_prompt()`; the kind is in hand at the call site
    (`selection.kind`) and unused.
  - `src/tree_read.rs:90` — `pick` takes the first live **non-`finish`** leaf
    regardless of position, falling back to the sentinel only when nothing
    ordinary is live. This is what makes D4 work: a `finish` session may
    `leaf-add` at the root, landing *after* the sentinel, and the next iteration
    still picks the new leaf.
  - `src/complete.rs` — the verb is not gated by kind, so the relaunch ending in
    D4 is already legal; only the documentation is missing.

## Notes

**The specialisation lands structurally in this grove and behaviourally in the
next.** Provisioning stays live, so every session also receives the whole
unsliced `SKILL.md` as a harness skill, `finish` sessions included. That is the
transient both-paths state the spec admits, and it means this grove cannot be
verified by watching a session behave differently — it is verified by the
composed mandates themselves.

**No ADR is expected.** D1–D3 apply decisions
`mandate-delivers-the-methodology.md` already records; D4 documents a path the
code already supports. If the design leaf finds D4 is a genuine trade-off rather
than a gap being written down, that is the one candidate — the when-to-write test
decides, not this brief.
