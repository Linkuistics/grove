# mandate-delivery-review-k3

**Reviews:** `mandate-delivery-k2`

## Goal

Disprove the mandate-delivery design before `increments-k4` decomposes on top of
it. It is load-bearing for months: the root brief's `Done when` is checked
against it, and it invents a syntax that ~120 kB of `content/` will then be
marked up in.

Inspection only. Read the committed diff of `mandate-delivery-k2`, the source it
claims things about, and the record set. Do not run builds, tests, formatters, or
edit anything; findings go to an `integrate-review-design` leaf you cut yourself
**only if you find something worth acting on**.

## Context

Artifacts under review:

- `docs/specs/mandate-delivered-methodology.md` (new)
- `docs/adr/mandate-delivers-the-methodology.md` (new)
- `docs/adr/one-build-owns-a-session.md` (rewritten in place)
- `docs/adr/complete-session-configuration.md` (one option reworked)
- `CONTEXT.md` (glossary: `Methodology unit` added; four entries reworked)
- `CONTEXT-MAP.md` (ownership list), `docs/ARCHITECTURE.md` (one pointer)

Four calls were put to the human and confirmed, so they are settled inputs, not
findings: one `methodology` module seam; malformed embed fails the **build**;
`continue.md` becomes `content/MANDATE.md` as an ordinary unit; `kinds=` admits
`*` or an explicit list only. Argue with their *consequences* freely — just not
with the choices themselves.

## The specific doubts

Ordered by how much damage each would do if the design is wrong.

1. **Do total partition and the fence rule actually compose?** Markers are
   ignored inside fenced code blocks, and units may begin anywhere. So a marker's
   status depends on the parser's fence state *at that point in the file* — which
   is accumulated across every preceding unit. Two things to test on paper: an
   unterminated fence earlier in a file silently demotes every later marker to
   prose (the file then fails the build for having one giant unit, or does it
   fail at all?); and a fence opened in unit A and closed in unit B means neither
   slice is independently well-formed markdown when inlined into a mandate. The
   spec asserts a unit "must read correctly standing alone" as an authoring rule —
   is that checkable, or is it the design's one unverified claim?

2. **Is `kinds` forbidden on `class=procedural` right?** The spec argues a scope
   there would be a lie because procedural units ship to no mandate. But
   `grove-llm methodology` lists *all* units, and a session may only ever need
   some of them. Does a scope on a procedural unit have a legitimate use the
   design foreclosed, and is "forbidden" (build error) too strong versus
   "optional, and it filters the listing"?

3. **Verify the two source claims the spec makes about the release path and the
   identity constant.** Both are load-bearing scheduling facts and neither was
   run:
   - `scripts/release-common.sh`'s `assert_methodology_pairing` currently fails a
     release if `grove-llm` contains `CONTENT_MARKER`. The spec claims this
     inverts once `grove-llm` links the embed. Confirm the direction, and confirm
     whether `tests/provision.rs`'s pinning of that marker is the only other site.
   - The spec claims `GROVE_CONTENT_HASH`, its `build.rs` emission, and the
     embed-vs-filesystem equality test can all be deleted once both binaries link
     the embed. Check nothing else needs the identity *without* linking
     content — `--version`, the release scripts, `--identity` on a `grove-llm`
     too old to answer.

4. **The reconciliation rule, applied to the whole record set.** The rule this
   session settled on: *a record is reworked now only when the new decision
   supersedes its own; a record merely describing mechanism that has not yet
   changed stays accurate until it does.* Under it, `one-build-owns-a-session`
   and `complete-session-configuration` were reworked; `one-live-driver-per-working-tree`
   was edited and then **reverted**; `config-driven-sessions.md` (12 provisioning
   sites) and `ARCHITECTURE.md` were left to the retirement increment, the latter
   with a pointer so its citation of the reworked ADR does not misrepresent it.
   Is that rule right, and is it applied consistently? The specific risk is the
   glossary, where four entries *were* edited forward while
   `Global skill provisioning` describes live code — check `CONTEXT.md` does not
   now contradict itself.

5. **Composition order needs no special case — verify.** The claim is that
   per-file ordering places `MANDATE.md` first, so the composer has no preamble
   rule. Check nothing else needs to interleave: is there a triggering unit in a
   format guide that must precede part of `SKILL.md` to read correctly?

6. **Is the completeness invariant actually complete?** It asserts triggering
   units appear where their scope admits, procedural units nowhere, and every
   unit is id-reachable. What it does **not** assert is that the classification is
   *right* — that residue is named and handed to a review pass. Is there a third
   mechanical claim available that nobody thought to make? Consider: does anything
   check that a *mandate* is non-empty for every kind, or that no kind receives
   only `kinds=*` units when it has a discipline unit that should have been
   scoped to it?

7. **The size alarm's bound is arbitrary.** 64 KiB is a number, not a
   derivation. Is there a principled one — a multiple of the triggering estimate,
   a fraction of `ARG_MAX` minus a measured environment — or is arbitrary
   honest here given it is explicitly an alarm and not a limit?

8. **Requirements coverage.** The spec's `## Requirements` has no scenario for
   fence handling, and none for the byte-exactness of the marker line itself
   (only the unit's bytes). Are those gaps, or correctly left to `## Test seams`?

## Done when

Every doubt above is either confirmed sound or written up as a finding with the
file and the argument. Findings that are real go to an
`integrate-review-design` leaf, cut with `leaf-insert` against `increments-k4` —
it is the first live sibling after this one, so a plain `leaf-add` would land the
integration *after* planning and let planning decompose on unfixed material.

If nothing survives verification, create nothing and retire.

## Notes

- Read `content/driving.md` § *Verifying a claim about the repo itself* before
  checking doubt 3. Two of those claims are greps against a shell script and a
  test file, and a well-formed pattern that matches nothing reads exactly like
  confirmation.
- The classification pass over `SKILL.md`, `TASK-FORMAT.md` and `driving.md` is
  **not** in scope here — it does not exist yet and earns its own review chain
  inside the first increment. This review is of the design only.
