# embed-wide-gate-k8

## Goal

Close the build gate over the **whole embed**: the malformation classes no single
file can decide — id uniqueness, `defers=` resolution and its class check,
procedural reachability, and ordering-key uniqueness if a carrier exists — plus
the positive control that pins the unit set complete.

This is the leaf that makes `classification-k9` safe to attempt. Until it lands,
a wrong `defers=` target or an unreachable procedural unit is a silent authoring
error; after it, both are a named build failure with a file and an offset.

Design: `docs/specs/mandate-delivered-methodology.md` (*A unit names the
procedure it defers to*, *A malformed embed fails the build*, *Test seams*).

**This leaf was redrawn by `increments-integrate-k12`.** It was previously
`methodology-verb-k8` — the verb, the relocations and the release-scan inversion.
The verb and everything coupled to linking the embed moved to
`addressable-embed-k7`, which is what makes that leaf vertical; what remains here
is the whole-embed half of the gate, which needs the parser `k7` lands and
nothing else.

## Context

### What lands

- **Id uniqueness across the whole embed.** Ids are the verb's only address
  (`grove-llm methodology <id>`), so a collision between two files is
  unaddressable, not merely untidy.
- **`defers=` resolves, and resolves to the right class.** A deferral naming no
  declared unit is a dangling reference; one naming a *triggering* unit is a
  category error that would deliver a condition where a procedure was promised.
  Both fail the build. This is a contributor's mistake, not a caller's — the
  runtime unknown-id error `k7` shipped is the other species and stays a runtime
  error.
- **Every procedural unit is reachable** by following `defers=` from some
  triggering unit. Reachability is partition seen from the other end: together
  they say every byte of the methodology is either in a mandate or reachable from
  one, so an undiscoverable procedure is as impossible as unclassified prose.
- **Ordering-key uniqueness — if and only if there is a carrier.** Derive this
  from `docs/specs/mandate-delivered-methodology.md` **as
  `ordering-key-placement-k6` settled it**. If that leaf deferred the ordering
  carrier out of this grove (its option 3), there is no key here to be duplicated
  and this bullet is empty. Do not assume a duplicate-ordering-key error into
  existence.
- **A positive control pinned complete**: the full set of unit ids is a test
  constant, so losing a unit fails and gaining one fails until someone names it.
  Under the trivial marking that set is one id per embedded markdown file;
  `classification-k9`'s batches each update it deliberately, which is the moment
  the design intends each classification decision to be confirmed by a human.
- **The two embed-claim relocations out of `tests/provision.rs`**, because the
  successor grove deletes their home and both are claims about the embed rather
  than about provisioning:
  `the_embedded_methodology_instructs_no_verb_the_embedded_cli_lacks` (with
  `collect_instructed_verbs`, `scan_instructed_verbs`, `INSTRUCTED_VERBS`) and
  `the_grove_llm_verb_surface_is_flat` (with `exposed_verbs`), which is what makes
  the first comparison mean what it claims. Their corpus improves in the move:
  they scan the **embed** rather than a provisioned extraction of it.

### `INSTRUCTED_VERBS` stays at eleven — verified, do not re-derive

`increments-integrate-k12` enumerated every `grove-llm <verb>` occurrence in the
embed and found exactly the eleven the constant pins, with **no** `grove-llm
methodology`. The relocation is a move, not a revision: carry the constant across
unchanged. Add the verb **when and only when** `content/` starts naming it, which
is `content/MANDATE.md` in the successor grove.

Watch one interaction, though. If `step-suffix-redundancy-k10` and its
implementation edit `content/SKILL.md` or `content/TASK-FORMAT.md` in a way that
adds or removes a `grove-llm <verb>` mention, this test is what notices. That is
the test working; reconcile the pinned set with what the prose actually
instructs, and do not weaken the scan to make it quiet.

### What is deliberately not here

- The per-file gate classes, the parser itself, the marking, the verb, the
  identity cutover and the release-scan inversion. All `addressable-embed-k7`.
- Any real classification judgement. `classification-k9`.
- The completeness invariant's *mandate* claims and the per-kind size alarm —
  those measure a composed mandate, and there is no composer in this grove.
  Reachability here is the structural half that does not need one.

## Done when

- `cargo build` fails, by name and with the file and offset, on: a duplicate id
  anywhere in the embed, a `defers=` naming no declared unit, a `defers=` naming a
  non-procedural unit, and an unreachable procedural unit — plus a duplicate
  ordering key **if** the settled spec puts a carrier in this grove.
- Each of those is shown able to fail against a synthetic malformed embed, and to
  pass against a synthetic well-formed one. A cross-file case is covered: the
  malformation must be one that no single file's parse could have caught, or the
  test is pinning `k7`'s gate over again.
- The pinned complete id set exists as a test constant and fails in **both**
  directions — a lost unit and an unnamed new one.
- The two relocated checks run against the embed, still fail on the shapes they
  were pinned against, and `INSTRUCTED_VERBS` is unchanged at eleven; the
  flat-verb-surface pin still fails the day a `grove-llm` subcommand grows a
  subcommand.
- `grove-llm methodology` still lists and round-trips the trivial marking — the
  gate hardened around it, it did not change what the verb serves.
- `cargo test` and `cargo build` are green, and the running loop is unaffected.

## Notes

- The demo that earns this leaf its boundary: point a `defers=` at a
  non-existent id, or at a triggering unit, and watch `cargo build` name the file
  and offset. That is behaviour a contributor observes, it is useful the moment it
  lands, and nothing downstream has to arrive first for it to be worth having.
- Splitting the gate across two leaves was `increments-integrate-k12`'s answer to
  a leaf that was too large after merging the verb in. It is safe because the
  trivial marking is valid under the **full** gate — no `defers=`, no procedural
  unit, no duplicate id — so `k7`'s window ships nothing the finished gate would
  reject.
- Decide where the embed-claims test home lives once, in `k7`, and put these
  there rather than starting a second one. If `k7` did not create one, create it
  here and say so.
- If a class turns out to be cheaper to check inside the per-file parse than over
  the assembled set, say so rather than forcing it across the seam — the seam is
  *what information the check needs*, not a quota.
