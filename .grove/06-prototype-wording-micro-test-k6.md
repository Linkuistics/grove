# wording-micro-test-k6

## Goal

Run the two-arm wording micro-test the spec requires **before any of the rewrite
ships**, and record its result. Two arms, five fresh-context repetitions each, run
once as design validation.

## Why this is first

This is an ordering constraint the spec calls a design fact, not a preference:

> The wording micro-test runs before the core's wording ships. It is cheap, it
> needs no part of the rewrite, and its whole value is being able to change the
> design's answer to the first observed failure while changing the design is still
> free. Run after the rewrite lands, it is a post-mortem.

Trigger strength is the *only* thing in this design that answers the **first**
measured failure — sessions did not read the provisioned skill. Everything else
answers the second. If the wording does not work, the design has no answer to the
first failure at all, and that is a stop, not a tweak.

## The two arms

- **Control** — the pre-mandate launcher shape: ~1.1 kB whose single relevant
  clause is *use the grove skill*. This is the arm the field failure was actually
  measured on, so it is the control the house rule asks for rather than a synthetic
  one. Recover its wording from VCS history rather than reconstructing it.
- **Variant** — the designed core: the imperative naming both targets, the
  ordering clause with its enumeration of tempting alternatives, the provisioned
  directories by absolute path, the rationalization table, and *this prompt is not
  a summary*. The spec's *Trigger strength is the design's load-bearing half*
  section is the design being tested; write the actual wording here, because this
  leaf's output is what `guaranteed-core-k9` ships.

Both arms run against a **short stand-in** `SKILL.md` and reference file — the
real corpus is not needed to test whether a session opens one — and against the
configured model and harness targets this workstream's own
`~/.config/grove/config.kdl` launches. **Name the targets in the result**: a
wording result does not transfer across them.

**Observable:** did the session open the skill and name its reference file before
acting.

## Done when

The result is recorded with
[`skill-delivered-methodology`](../docs/specs/skill-delivered-methodology.md) —
arms, targets, repetitions, counts — and the winning wording is written down in a
form `guaranteed-core-k9` can lift verbatim.

Three outcomes, and two of them change the plan rather than confirming it:

- **Variant beats control.** Proceed; the recorded wording is the core's.
- **Variant does not beat control.** The wording is not the fix and the design's
  answer to the first failure is missing. **Stop and escalate to the human** —
  this is not a tweak, and the rest of the tree is built on the claim this arm was
  meant to establish.
- **Control does not exhibit the failure on these targets.** The house rule's own
  stop clause applies: there is nothing to shape, so the prohibition and
  rationalization apparatus is unwarranted and should be **cut** rather than
  defended. Record that, and cut the core's wording down accordingly.

## Notes

The house authoring rule this discharges is
`plugins/linkuistics/skills/authoring-conventions/SKILL.md`, *Test the wording,
cheaply*. The spec's classification argument matters and should not be re-litigated
here: this is a **discipline** failure (the session knows it should read the skill
and skips it), which is why the prohibition/rationalization/red-flag form is the
prescribed one. The same guidance warns those forms backfire on *shaping*
problems — a later reader who has only seen that warning will want to strip the
apparatus, so the reason it is kept belongs in the recorded result.

This is design validation, **not a gate**. It runs once, its result is read by a
human once, and nothing in the suite or the build re-runs it. It is not the
end-to-end acceptance check either — that is `delivery-acceptance-k11`, and it has
two limbs this one does not test.

`prototype` because the observable is a human's reading of ten fresh-context
sessions; there is nothing here for an AFK session to conclude on its own.
