# retire-harness-stamp-claim-k23

**Kind:** impl

## Goal

`grove retire --help` claims `--harness` "writes a lasting binding to
`.grove-stamps/<name>`". It does not. Decide whether the doc is wrong or the
behaviour is, and fix whichever it is.

## Context

`RetireArgs::harness`'s doc comment (`src/cli.rs`) is a verbatim copy of
`StartArgs::harness`'s, including the stamping sentence. But `maybe_stamp` has
exactly **one** call site — `launch.rs`, inside `do_grove`, below the no-launch
return — so `grove retire --harness pi` resolves pi for that session and writes
nothing. The claim's *second* half is true: `launch::retire` calls
`resolve_for_launch`, which reads the stamp.

Found by `retire-no-launch-help-k21` while establishing what `grove retire`'s
`--no-launch` may honestly say. That leaf deliberately left the defect standing
and its own new description **omits** the "It writes no stamp" clause `grove
do --no-launch`'s carries — a clause that would be vacuous here, since nothing on
this path stamps at all. So the two options' descriptions currently disagree about
whether this verb stamps, and only one of them is right.

This is not obviously a documentation fix. The behaviour is arguably the correct
one and the doc merely inherited: `grove retire` is a one-off manual verb, and
silently rebinding the grove's harness as a side effect of retiring one node is a
lasting change nobody asked for — which is the same argument `do` uses to keep
`maybe_stamp` below its own `--no-launch` return. But `--harness` on `do` *is*
documented as a deliberate binding, and a user who reaches for the identical flag
on `retire` has no way to learn it behaves differently. Decide it, don't just
reword.

## Done when

- `grove retire --help`'s `--harness` description matches what the code does,
  executed and read.
- If the answer is that the doc was wrong, the reason `retire` does not stamp is
  recorded where the next reader meets it, not merely deleted — the two verbs
  taking the same flag to different effect is the surprising part.
- `cargo test` passes.

## Notes

The two doc comments being byte-identical copies is what let this survive: the
sentence is true of the struct it was written for. Worth a glance at whether any
*other* claim rides the same copy — `resolve_for_launch`'s auto-detection half is
shared and correct, but that is the only other clause.

**Decided: the doc was wrong.** The behaviour stands, under the rule *a lasting
binding is written by the action that asks for one* — already in the codebase as
`do`'s `maybe_stamp` sitting below its `--no-launch` return, now generalised in
ADR *model-per-task-kind* with `grove retire --harness` as its second instance.
Symmetry was rejected on **cost asymmetry**: an unpersisted `--harness` surprises
once, immediately; a retire that rebound the grove would redirect every later
`grove do` and be found sessions later. Recorded in both `--harness` docs (each
naming the other verb), `maybe_stamp`'s "one call site, deliberately", the ADR,
`CONTEXT.md`, `README.md`, `docs/grove.md` and `docs/workflows/start.md`.

The other clause turned out **imprecise rather than correct**: the default is the
stamp *then* auto-detection, and the parenthetical only read as true in the
company of the stamping sentence — so removing that sentence from `RetireArgs`
would have left a false default behind. Both structs now state it in full.

Full carry-forward in the root brief. CHANGELOG-free per `k20`'s rule; no leaf
externalized — the four extra surfaces are the flagged finding's extent.
