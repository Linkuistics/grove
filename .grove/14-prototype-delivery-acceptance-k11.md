# delivery-acceptance-k11

## Goal

Run the end-to-end acceptance check this grove's `Done when` names: **a real Grove
run, with a human watching, showing sessions both ending and reading the skill.**

## Both limbs are required

This is the whole point of the leaf, and it is the one thing that cannot be
weakened:

- Sessions that **end and do not read the skill** are a swap — the wall is gone and
  the first measured failure is back.
- Sessions that **read the skill and do not end** are a swap — the skill is read and
  the loop still stalls.

Neither alone clears this. It is the same check
`skill-delivers-the-methodology` itself nominated — *the next real Grove run
after the change lands, with a human watching* — which is why the design accepts
it rather than proposing a softer one.

## The observable, and its honest limit

The core instructs the session to state, in its first message, that it has read the
skill and to name the reference file it read. **That line is the design's only
observable**, and it exists because Grove *structurally cannot see what a session
reads*: the configured command is opaque and owns the real TTY, so the driver never
sees a byte of the session's output. The audience for the line is the human
watching the loop — the same human who reads the pairing report between sessions.

A machine-checked read receipt was designed and **rejected on its merits**, and is
not to be re-proposed here. The available proxy is *did this session run the
bootstrap verbs under its epoch*, and it has a false negative the methodology
itself invites, since constraint 2 says the one command Grove asks for "is a lookup
you could do by eye". A proxy the methodology tells sessions they may skip reports
on the methodology, not on the session. Reopen only if a launch target ever exposes
which skills a session loaded.

## Done when

Enough real sessions have run, on the configured targets, for the human to judge
both limbs — and the judgement is recorded.

Three outcomes, and only one of them ends the grove:

- **Both limbs hold.** Record it against `skill-delivered-methodology`, and the
  grove's `Done when` is met.
- **Sessions do not end.** The core's ending position or wording is the suspect —
  it is delivered twice and trails only ~1.5 KiB, so the recency argument is much
  weaker here than it was under the mandate, and that is a stated consequence rather
  than a surprise. `leaf-add` the follow-up.
- **Sessions do not read the skill.** Trigger strength did not hold at full scale
  even though `wording-micro-test-k6` said it would. That is the design's central
  claim failing on the real corpus, and it is a **finding for the human**, not
  something to patch inline — escalate.

## Notes

This is an **experiment, not a gate**. A standing behavioural eval in the suite or
the build stays out of scope: expensive, non-deterministic, it measures a model
rather than Grove's artifact, and it localizes nothing when red. This run is read
by a human once.

It is distinct from `wording-micro-test-k6` in three ways that matter: that one ran
against a stand-in corpus, tested one limb, and ran *before* the design could still
be changed cheaply. This one runs the real thing, tests both limbs, and is the
acceptance check.

`prototype` because the observable is a human's reading of a live loop. An AFK
session has nothing to conclude here.
