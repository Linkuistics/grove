## What each kind produces

The closed set of **nineteen** is five producers (`requirements`, `design`,
`planning`, `prototype`, `impl`), each with its own `review-` and
`integrate-review-` step, plus `research-a`, `research-b`, `combine-research`,
and the driver-reserved `finish`. `TASK-FORMAT.md` carries every kind's
discipline and its HITL/AFK mark, and each kind's own reference file under
`references/` carries the discipline that kind runs under.

**`planning` is the only kind with methodological force** — the sole branch
here, and the only kind that grows the tree generatively:

- Every other kind **produces an artifact** and differs in discipline, not in
  what the loop does with it. `requirements` settles *what* is wanted — on the
  threshold and procedure its own reference file states, which is **not** an
  interview every time; `design` turns that into a spec or an ADR set; `impl`
  produces code, docs, or tests; the `review-*` kinds produce findings and the
  `integrate-review-*` kinds apply them.
- A `review-*` session is **inspection-only**: inspect the producer's committed
  changes, source, requirements or specifications, and recorded verification
  evidence. Do not run test, build, lint, or format commands, edit production or
  test code, or redo the implementation. Review output is findings only; the
  paired `integrate-review-*` task owns every fix and all post-fix verification.
- A **planning task** must first find the **smallest independently useful
  working increments** and order them by dependency, creating a separate grove
  for every obvious stage that leaves the product working and delivers useful,
  verifiable behavior for its successor. Changes that cannot independently leave
  the product working stay in one increment even when their code edits are
  separable. It then cuts the current increment into vertical slices and **grows
  the tree** (Decompose). It no longer interrogates — grilling moved to
  `requirements` — but it MAY still sharpen the glossary or raise an ADR inline,
  as any kind may.

## Raising records, and keeping each set minimal

Whichever kind is running: raise ADRs *sparingly* (`ADR-FORMAT.md` for
placement; the `linkuistics:decision-records` skill for the philosophy, format,
and when-to-write test), and write a spec only at a genuine agreement point
(`SPEC-FORMAT.md`). Treat the ADR set as a **minimum coherent set describing the
current design**: when a session *changes* a decision an ADR already records,
**rework the set in place** — merge / split / delete — and reconcile the briefs
that cite it; never append a superseding ADR (the VCS holds the history). The
same rule governs `docs/specs/`, one grain coarser. See `driving.md` for the
field-guide habits that make grilling, research-leaf commissioning, and the
review chain productive (WDYT, pushback, running decision log, citation
discipline).

**Review ownership inside a picked leaf.** This applies only after the driver
launched this session with a selected-leaf mandate in `${prompt}` and the
session adopted that mandate by running Bootstrap — a `.grove/` directory in the
checkout and inherited Grove control variables do not count. A picked plain producer may
materialise at most one reviewer across the **whole picked leaf**; each
independent diverse-lens context counts. A second need — normally re-review
after a substantive non-mechanical fix — is the signal that review has become
tree-sized work: cut a `review-<producer>` leaf with `leaf-add`, writing the
specific doubt into its body; trivia, noise, visible trade-offs, and
test-conclusive fixes do not force it. A producer that already has a review leaf
beside it, `review-*`, and every research-pair leaf
spawn none; `integrate-review-*` may spend one narrow reviewer, then externalises
substantial redesign as a new producer review chain beside the leaf it is
integrating. Outside this predicate doubt
keeps its standalone cycles. Once review is escalated to the tree grove owns the
route: it launches the `review-*` kind's own configured command, and whether that
target differs from the producer's is the configuration owner's policy — grove
records no producer target, compares none, and warns about none.
