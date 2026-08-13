# The mandate delivers the methodology

Grove's methodology reaches a session as **byte-exact slices of the driver's own
embedded `content/`, composed into `${prompt}` and selected by the launched
session kind**. That is the sole delivery path: the binary no longer sweeps
`content/` into any harness's global skill directory, and no session is expected
to locate a skill on disk.

This binds because the fact a session needs is one the driver already holds, and
a session told a fact succinctly never runs the derivation that would have
established it. The saving is reasoning not performed first, methodology prose
never written second, and bytes not read a distant third — turn count alone would
justify inlining the glossary into argv and never justifies anything.

A slice is copied, never composed. Driver-authored prose about the methodology
would make `content/` non-canonical and create a second source of truth that
drifts across the build boundary, while a verbatim projection cannot contradict
what it copies. The driver therefore authors mandate prose only for facts it
resolves at **runtime** — today the selected stable handle and the resolved
version control — and everything static is a slice. Pointing at a location
instead was not available at the granularity that matters: a kind's discipline is
one bullet inside a section of nineteen, so a pointer leaves the session reading
the whole section and performing the selection itself, which is the reasoning
cost being removed.

Selectivity is what makes this dangerous, so what may be withheld is decided by a
rule rather than per unit. **Keep the `if`, defer the `then`.** A rule's
triggering condition — *that a situation exists calling for something other than
what this session is doing* — ships in every mandate its scope admits. Its
procedural body is deferred to `grove-llm methodology`, which serves units from
the invoking binary's own embed. The asymmetry is the entire argument: a
withheld procedure costs a lookup the session knows to make, while a withheld
condition yields an **unasked question** — silent, and Grove's primary failure
mode, a session quietly absorbing work that should have been its own leaf.

The lookup is **addressed, not guessed**. Unit ids are one namespace across both
classes, so a slice's own id addresses the slice; a marker therefore *declares*
the procedural unit it defers to, and because the marker line is part of the
unit's source that declaration arrives with the slice. Without it the asymmetry
would be an assumption rather than a property — a session told there is a deferred
half, and given no way to ask for it.

That silent failure is admissible only because it is converted into a structural
one. The classification is **data marked in `content/` beside the prose it
classifies**, so the question narrows from "did we build the mandate right?" to
"did we classify this one unit right?", and a completeness invariant answers the
first mechanically: every triggering unit appears in the mandate of every kind its
scope admits, every procedural unit in none, and **every procedural unit is
reachable from some kind's mandate** by following the declared deferrals. Units
**partition** a file rather than sitting in it as islands, which is what makes
unclassified prose unreachable and makes a parser that goes blind to a marker
shape produce visibly different bytes instead of a silent hole. Partition and
reachability are the same claim seen from each end — together they say every byte
of the methodology is either in a mandate or reachable from one, so a procedure no
session can be told about is as impossible as prose no parser can see. The
residue — whether a given unit was classified correctly — is judgement, and gets
an adversarial review pass rather than a mechanism.

A malformed embed **fails the build**. Grove guides and does not gate, but that
constraint governs the human's task tree, not Grove's own compile-time artifact,
and here the errors do not cost the same: a hard failure inconveniences a
contributor in the repository where the mistake was made, while a soft one ships
a binary that drops a triggering unit from every session it launches. Nor is
there any proxy involved — the embed is fully observable by the build that
produced it, which is what separates this from the launch-time checks that
deliberately report rather than refuse.

Retiring provisioning removes the shared mutable directory and with it the only
place a build's methodology could be clobbered from outside. What remains of
[build pairing](one-build-owns-a-session.md) is narrower and sharper: the
triggering half of a rule now reaches the session from the driver's embed while
the deferred procedural half comes from whichever `grove-llm` the session
resolves, so a mismatched pair is a split-brain inside one rule rather than two
copies of one document. It is also loud where the old skew was quiet, and the
declared deferral is why: the mandate names the id it wants, so a `grove-llm`
that does not have it errors on that name.

`docs/specs/mandate-delivered-methodology.md` carries the marker grammar, the
composition order, the verb's surface, and the agreed test seams.

## Considered options

- **Keep provisioning and add the mandate on top of it.** Rejected because it
  realizes none of the win: if the session still loads the whole skill, every
  slice is pure duplication that *raises* both token cost and contradiction risk,
  and the two paths can disagree. This was the shape the design most nearly took,
  as a transitional step, and going straight to mandate-only became reachable the
  moment `grove-llm methodology` could serve the deferred half. Reopen only if a
  supported launch target is found that cannot receive a large `${prompt}`, which
  would make a second delivery path necessary rather than merely redundant.
- **Inline the whole methodology into every `${prompt}`.** Rejected because it
  abandons specificity, which is the point: a session handed all of `content/`
  performs exactly the selection the slice exists to remove, and the nineteen
  kinds' disciplines would arrive eighteen-nineteenths irrelevant. It also
  forfeits the completeness invariant, since a mandate containing everything
  makes "is this unit present?" unable to fail. Reopen never; specificity is the
  decision.
- **Point at locations in the methodology instead of slicing.** Rejected because
  Grove's content is not addressable at the granularity the session needs, so a
  pointer delivers the reasoning cost it was meant to remove. Reopen if
  `content/` is ever restructured so that every rule is separately addressable —
  which is close to what unit markers now make true, and is why the pointer
  option is worth re-examining only as a *supplement* for units too large to
  inline, never as a replacement for triggering conditions.
- **Have the driver compose or summarize the methodology in its own prose.**
  Rejected because it makes `content/` non-canonical: a summary can contradict
  its source, and nothing would detect it. The cost of byte-exactness is that
  slices must read correctly standing alone, which is an authoring constraint on
  `content/` rather than a defect. Reopen never.
- **Classify in a manifest beside `content/` rather than in it.** Rejected
  because adjacency is what keeps a classification true when someone edits the
  prose; a separate manifest goes stale silently and in the fail-open direction.
  Reopen only if the markers are shown to damage `content/`'s readability as
  prose, which HTML comments are chosen to avoid.
- **Name the deferred procedure in the triggering prose rather than in the
  marker.** Rejected because it puts the one reference the build must check back
  inside free text: proving every deferral resolves to a real procedural unit
  would mean parsing prose for a citation grammar, which is a second grammar to
  invent, to teach, and to keep the classification honest against — and a
  reference the build cannot check is exactly the manifest failure one entry
  above, relocated. The marker already carries the id at zero cost, because it is
  part of the unit's source and therefore part of the slice. Reopen never; a
  checkable reference is the point.
- **Split `content/` into one file per unit.** Rejected because readability of
  `content/` from the code is the stated reason for keeping the embed, and the
  seven constraints cannot be checked against a spine scattered over dozens of
  files — constraint 7 asks whether the loop fits on a page, and constraint 6
  asks whether `.grove/` survives as a legible folder of notes. Whole documents
  marked in-body get the granularity without the scattering. Reopen if a unit
  ever needs to be delivered to a consumer that cannot parse markdown.
- **Fail the driver rather than the build on a malformed embed.** Rejected
  because it defers a compile-time-visible fact to a stranger's stalled loop; the
  contributor who made the mistake is the one who should see it, and they are the
  one running the build. Reopen never.
- **Warn rather than fail on a malformed embed.** Rejected because the warning's
  audience is a session that has already been handed an incomplete mandate, and
  the failure being prevented is precisely the one nobody notices. Reopen if
  markers ever become author-supplied outside this repository, which would move
  the error into someone else's working tree and change who pays for a refusal.
- **Let `kinds=` express families or negations.** Rejected because a shorthand
  silently absorbs a session kind added later — the same failure the
  [complete session configuration](complete-session-configuration.md) repetition
  rule exists to make visible — and because it moves the answer to *which kinds
  does this unit reach* away from the marker beside the prose. A negation states
  the complement of a set declared elsewhere, so recovering the per-kind
  classification means restating it in a test: the manifest failure one entry
  above, relocated. Complement cases do arise, and the first one — the relaunch
  ending, for every kind but `finish` — is spelled as all eighteen labels rather
  than as a negation. The list's own hazard is the mirror, a kind added later
  silently *omitted*, and what decides the trade is the cost of guarding each: an
  omission is caught by a check derived from the closed kind set alone, while an
  absorption satisfies every such check and is caught only by an exhaustive
  expected-ending mapping that restates each kind's scope away from its marker.
  Reopen if the kind set grows large enough that explicit scopes stop being
  auditable, and only with a replacement that preserves fail-on-kind-addition.
- **Verify the change behaviourally rather than structurally.** Rejected because
  an evaluation is expensive, non-deterministic, measures a model rather than
  Grove's artifact, and localizes nothing when it goes red. The completeness
  invariant localizes to a unit, and the honest behavioural check is the next real
  Grove run after the change lands, with a human watching. Reopen if a
  misclassification is ever shown to survive both the invariant and the review
  pass.
