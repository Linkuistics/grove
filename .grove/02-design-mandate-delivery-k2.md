# mandate-delivery-k2

## Goal

Write the spec and rework the ADR set for mandate-delivered methodology. The
*what* is settled (see the root brief); this leaf settles the *how* precisely
enough that `planning` can cut increments and `impl` can build without
re-deciding anything.

Deliverables:

- `docs/specs/<slug>.md` — the mandate contract, the triggering/procedural rule,
  the unit-marker syntax, and the agreed test seams.
- ADR rework, **in place** (`linkuistics:decision-records`): a new decision
  recording *methodology is delivered by mandate, not by provisioning* — it
  clears the when-to-write test on all three counts — and `one-build-owns-a-session`
  either rewritten down to the surviving `grove-llm`-off-`PATH` half of
  [[Build pairing]] or deleted and merged into the new one. Reconcile every
  citation: `CONTEXT.md`, `CONTEXT-MAP.md` (it lists ADR ownership),
  `docs/ARCHITECTURE.md`, and the other ADRs.
- Cut the `planning` leaf as this session's last act, once the shape is known.

## Context

Settled by grilling in `plan-k1` (retired; its decisions log is the record):

- **Slice, don't paraphrase.** Byte-exact projection of the embedded `content/`.
- **Keep the `if`, defer the `then`.** Triggering conditions ship in every
  mandate; procedural bodies defer to `grove-llm methodology`.
- **Marked in `content/`**, adjacent to the prose: HTML-comment unit markers plus
  per-file KDL frontmatter. Documents stay whole and readable.
- **Mandate-only delivery.** Global skill provisioning retires.
- **Verification** is the completeness invariant + a review chain over the
  classification + golden per-kind snapshots. Behavioural eval was rejected.

## The questions this design must answer

Ordered roughly by how much else depends on them.

1. **Marker syntax and its failure story.** Proposed shape:
   `<!-- unit: retire-cascade kinds=* class=triggering -->`. What delimits a
   unit's *end* — the next marker, the next heading, an explicit close? What
   happens on a malformed, duplicate, or unknown-id marker: does the build fail,
   or the driver? (Grove's constraint 5 says it guides and does not gate — but
   that governs the *human's* work, not its own embed, so a hard failure here is
   probably right. Argue it.)
2. **What `kinds=` may express.** `*`, an explicit list, a producer-family
   shorthand? A shorthand is a second grammar to learn; an explicit list of
   nineteen is unreadable. Recommend one.
3. **How the parser and the prose are prevented from drifting.** This is the
   test obligation created by inventing a syntax. Note the ordinary trap
   (`driving.md`, "verifying a claim about the repo itself"): a well-formed
   pattern matching nothing reads exactly like a clean repo, so the check needs a
   positive control.
4. **`grove-llm methodology <…>`'s surface.** By unit id, by file, by section?
   Does it list what is available? Is it in the agent grammar `tests/provision.rs`
   scans — and does that test survive provisioning's retirement at all?
5. **Does `content/prompts/continue.md` survive?** A mandate that carries the
   loop may not need a launcher.
6. **Mandate assembly order and framing.** Slices are byte-exact, but their
   *arrangement* is the driver's: what order, what separators, and what — if
   anything — introduces them. Keep it minimal; every framing sentence is
   driver-authored prose and therefore a drift candidate.
7. **The classification itself** — the line-by-line pass over `SKILL.md`,
   `TASK-FORMAT.md` and `driving.md`. This may not fit this session. If it does
   not, it is its own leaf, and it is the one that earns a review chain.

## Done when

The spec and the reworked ADR set land, every citation is reconciled, and the
`planning` leaf is cut with the increment shape written into its body.

## Notes

- **The templating engine is a means, not a requirement.** The user accepted one
  ("even if we have to use a templating engine") to keep `content/` readable from
  the code. If plain slicing achieves that, no engine is needed — do not adopt
  one to satisfy a sentence.
- **`ARG_MAX` is 1 MiB, shared with the environment**, and `${prompt}` is argv,
  not stdin. Worth checking whether the largest plausible mandate approaches it,
  and worth deciding now whether that ceiling is a design constraint or merely a
  fact.
- **Test seams**: sketch them and put them to the user before the design is
  committed (`SPEC-FORMAT.md`, `linkuistics:codebase-design`). The obvious
  candidates are the parser (`content/` → units) and the composer (units + kind →
  mandate string); prefer one seam over two if the composer can be tested
  through the parser's output.
- **This is a load-bearing artifact.** Decide at the end of the session whether
  it earns `review-design`; the root brief's `Done when` will be checked against
  it for months.
