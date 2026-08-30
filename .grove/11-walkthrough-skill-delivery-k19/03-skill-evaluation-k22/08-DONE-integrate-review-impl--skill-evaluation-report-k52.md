# skill-evaluation-report-k52

**Integrates:** skill-evaluation-report-k51

## Goal

Act on the seven findings of `skill-evaluation-report-k51` against
`docs/evaluations/writing-code-walkthroughs/README.md`, and re-run the
deterministic checks the report claims after the edits land.

## Context

- Findings, with line citations and recommended fixes, are in
  `.grove/11-walkthrough-skill-delivery-k19/03-skill-evaluation-k22/07-DONE-review-impl-skill-evaluation-report-k51.md`.
- Reviewed artifact: `docs/evaluations/writing-code-walkthroughs/README.md` as
  committed by `skill-evaluation-report-k49` (jj change `twxrszsu`, commit
  `338198872cf90648430719ace60dedc7e23dd39a`). Line citations are to that
  commit; the file is unchanged since, so they still land.
- The frozen rubric is `docs/evaluations/writing-code-walkthroughs/baseline/rubric.md`.
  Nothing here reopens it, and no finding proposes a score change — every
  resolved count in the report reproduces from the retained evidence.

## Done when

- Finding 1 is settled: the blanket "all four regression guards remain
  unevaluable" is replaced by the two determinable results — `A14` breaches the
  guard under every completion of the missing sample, and the `10/15` endpoint
  is unreachable under every completion — or the stronger frozen rule the report
  actually applies is stated explicitly in its place.
- Finding 2 is settled: Case A's treatment-correlated invalidation is named
  where the enabled sample is introduced, and carried into the unresolved gap
  that says why the conflict is structural.
- Finding 3 is settled: the refinement scorers' disagreement count (`6 of 240`,
  one of them on preservation row `C09`) replaces the incorrect agreement claim,
  and the two single-scored arms are named.
- Finding 4 is settled: the `Verdict` no longer credits the final skill with
  passing harness checks that pass by refusing it, and the "sealed-template
  delta" clause either tests something or is dropped.
- Finding 5 is settled: both skill digests are stated with the arms each
  governs, and the consequence reaches `Unresolved gaps`.
- Finding 6 is settled: at least
  `plugins/linkuistics/PROVENANCE.md`'s `writing-code-walkthroughs` entry links
  the evaluation report and says the campaign did not establish the acceptance
  rubric.
- Finding 7 is settled or visibly accepted as a stated limitation.
- Every deterministic row the report claims is re-run after the final edit and
  its recorded result matches: `template-test.sh`, both historical
  `harness-test.sh` guards, `freeze-harness-test.sh`, `shellcheck`,
  `plugins/install.test.sh`, `cargo test --test reference_navigation
  every_repository_markdown_reference_resolves`, `cargo test --workspace
  --all-targets`, `cargo fmt --all -- --check`, and
  `cargo clippy --workspace --all-targets`.

## Notes

Findings 1-3 change what the report concludes; 4-6 are omissions and
overstatements; 7 is bounded and may be accepted visibly rather than fixed. A
finding closed as an accepted trade-off should say so in the report itself
rather than be dropped silently.

Finding 3 also holds against upstream records —
`refinement-regression/README.md:107-108` and
`adjudication/resolution.md:29-31` carry the same wrong agreement claim. Correct
them together with the synthesis so the report and the record it cites do not
disagree.

Finding 4's second half touches committed test scripts, not prose. Restoring the
positive assertions that `skill-evaluation-report-k49` deleted from both
`harness-test.sh` files needs a digest-parameterised harness if it is to keep
the drift refusal as well; if that is more than this leaf should carry, keep the
drift guards and make `template-test.sh` build both templates through a harness
pinned to the final digest, so the manifest delta is asserted against something
the repository actually produces.

## Decisions (running log)

Findings 1-6 are verified as real issues. Finding 7 is a real bounded record-shape
limitation and an incomplete static-analysis scope; disclose the former and
expand the latter. The frozen rubric and resolved scores remain unchanged.

For finding 4, retain the historical harness digest guards and their drift
refusals. Stop crediting the final skill with a reusable-harness pass and drop
the tautological sealed-template-delta claim; the remaining template test still
checks the final skill's bytes, digest, frontmatter, and plugin manifest. A new
final-digest campaign harness would add machinery without changing the report's
negative acceptance result.

The narrow adversarial pass found that a prose-only disclaimer left the exact
template command emitting the discarded claim. Remove the tautological
self-created-template block and its output text as part of finding 4; retain
only the script's real final-skill structure, byte, digest, and plugin-manifest
checks.

After retirement, `skill-evaluation-k22` closes because its charter permits an
unresolved acceptance gap to be surfaced. `walkthrough-skill-delivery-k19` does
not close: its stricter charter still requires the unchanged rubric to be met
and material improvement shown. Add `evaluation-recovery-k53` to plan a new,
separately predeclared cycle rather than weakening or reinterpreting the frozen
campaign.
