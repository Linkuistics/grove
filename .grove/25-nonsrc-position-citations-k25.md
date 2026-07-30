# nonsrc-position-citations-k25

**Kind:** impl

## Goal

Five comments **outside the root crate's `src/`** cite dead `.grove/` positions —
four in `tests/`, one in `codex-bridge/src/`. `src-position-citations-k24` swept
`src/` and deliberately did not absorb these: they are a different claim's
surface, not `k24`'s extent.

## Context

Found by `k24` while re-verifying its own claim, and **measured, not sampled** —
the same dereference-pattern grep that returns clean on `src/` returns these.
Every site verified by reading:

- `tests/launch.rs:60` — "*provisions the global skill on launch (070/010)*"
- `tests/launch.rs:140` — "*skill dir (070/010)*"
- `tests/provision.rs:6` — "*(self-extension-core-and-methodology /
  task-tree-scheme, 070/010)*" — the ADR slugs are right; the position is
  vestigial beside them, so this is likely `k24`'s `provision.rs` fix verbatim
  (drop the position, keep the slugs).
- `tests/harness_stamp.rs:85` — "*D5: `.grove-stamps/` must not dirty `git
  status`*". **A live `D<n>`**, the class `k14` reported at zero and `k22`
  re-confirmed at zero — both were right about `src/`, and neither claim ever
  reached `tests/`.
- `codex-bridge/src/main.rs:4-7` — the hard one, see below.

`herdr-plugin/` and `scripts/` are at zero (measured, same instrument).

**`codex-bridge` needs a decision, not a reword** — the `retire-harness-stamp-claim-k23`
shape. Its header points at `.grove/06-success-demonstration-k11/02-codex-bridge-k54.md`
**in the AgentAnyware repo**, and at that node's
`01-DONE-demo-architecture-and-plan-k53.md` "(decisions D2/D3/D6) for why it looks
like this". That referent is unreachable three ways over, not one: a *foreign*
repo, a tree deleted at its own finish cycle, and running-log sections that were
never durable. But the pointer is **load-bearing** — it carries the only stated
reason this binary decides nothing — so the question is whether to promote that
reason into this repo (an ADR, or prose in the header itself) or to drop the
pointer and accept the loss. Do not default; `k22` offered three options and was
right not to.

**Why the `src/`-only scoping is the reusable part, and why it is the third
generation.** `k14` narrowed by comment species (`//!`) and by an enumerated
pattern list. `k24` fixed that but inherited a *directory* narrowing — and note
`codex-bridge/src/main.rs` **is** a `src/` path, just not the root crate's, so
even the word "src/" did not mean what the claim's readers would take it to mean.
Same family as `k11`'s "grep the claim, not the file list": **a claim's scope is
part of the claim, and a scope stated as a path goes stale exactly like a file
list does.**

`k24` also settled the discriminator this leaf needs, so it need not re-derive
it: a position is an **instance** when the reader must *dereference* it to use
the comment, and **not** an instance when the position is the subject — fixture
data (`tree_migrate`'s golden v1 tree), a byte count, or a synthetic grammar
illustration (`01-<slug>-k1.md`). `tests/` has many of the benign kind
(`position 02`, `02-second-k2`, `04-last-k7`); they are correct and must be left
alone.

## Done when

- No comment in `tests/`, `codex-bridge/` or any other non-root-`src/` tree
  dereferences a `.grove/` position; load-bearing referents became an ADR slug,
  a module path, or promoted prose.
- The `codex-bridge` cross-repo pointer is decided explicitly, with the reasoning
  recorded.
- The claim is re-verified by grep over the **whole repo**, not this leaf's
  five-item list — and the grep is checked for having run, against a positive
  control (`k17`/`k20`/`k21`/`k24` each recorded a flag trap that manufactures a
  false clean; `k24`'s method: run the same instrument somewhere it *must* hit).
- `cargo test` passes.

## Notes

**Do not ship a lexical guard test**, and this is `k24`'s settled call rather
than an open question. The discriminator above is *semantic* — does the reader
dereference it? — so any regex encoding either flags the legitimate
position-as-subject sites or narrows to a pattern list, which re-commits `k14`'s
defect while wearing a test's authority. If a guard is wanted anyway, that is a
decision to argue on its own merits, not a Done-when.

CHANGELOG-free unless it changes behaviour, per `retire-help-node-path-k20`'s
rule as inherited by `k21`/`k22`/`k23`/`k24`: stale prose corrected against
already-shipped behaviour earns no entry. `codex-bridge` promotion, if it lands
as an ADR, is *recorded* not *landed* — also free (`k13`).
