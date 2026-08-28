# walkthrough-k23

**Reviews:** walkthrough-k2

## Goal

Adversarially review the committed decomposition for missing work, false
independence, incorrect ordering, and Grove composition errors before any
research or implementation leaf runs.

## Context

- Review the `walkthrough-k2` commit, the root brief, every live node brief and
  leaf, and the settled `plan-k1` requirements.
- This is findings-only. Do not edit the tree or run build, test, lint, or format
  commands.

## Done when

- The review checks the exact fifteen-file source corpus and evidence-only
  exclusions against the crate's real module/manifest layout.
- It tests whether the research pair is justified and independent, the design
  precedes its validators, and every authoring slice leaves a useful verified
  increment without waiting on a later slice to become intelligible.
- It checks that whole-source coverage, Markdown/local links, crate verification,
  technical accuracy, editorial quality, and skill behavior each have one clear
  proof point.
- It checks the lazy technical → integration-if-needed → editorial →
  integration-if-needed route for adjacency and correct `review-impl` ownership.
- It checks that the skill lane preserves baseline-before-authoring temporal
  separation and compares enabled runs against the same predeclared rubric.
- Every finding names severity, exact task/brief location, violated requirement,
  and a concrete repair; absence of findings is stated explicitly.
- If findings warrant changes, an `integrate-review-planning` leaf with bare stem
  `walkthrough` is inserted immediately before the first later live root entry
  and carries `**Integrates:** walkthrough-k23`. If no findings exist, no
  integration leaf is created.

## Notes

The plan is intentionally load-bearing enough to earn a fresh-context review.
Do not turn preference about chapter naming into a finding unless it creates a
dependency, completeness, or single-session defect.

## Decisions (running log)

Reviewed the `walkthrough-k2` commit (jj `ttmnnrlrutkv` / `62661ec0`): the root
brief's decomposition, three node briefs, and fourteen leaves. Inspection only —
no build, test, lint or format command run, and no tree file edited other than
the integration leaf this review cut.

Eleven findings, three of them structural: the complete-operation tour that
`plan-k1` settled is required by `book-system-k6` and forbidden by node-09's
sequence (F1); `fragment-validation-k8` and `markdown-validation-k9` are gated
on a book scaffold `orientation-k11` creates two nodes later (F2); and scoped
fragment validation is undefined for a source file split across slices, which
`src/fs/mod.rs` and `Cargo.toml` both are (F3). The remainder cover the
undeclared manifest split, the editorial review's unspecified kind and
`**Reviews:**` target, four consecutive ~1,200-line authoring slices with no
decomposition relief, the absent source-freeze policy, unowned `CONTEXT-MAP.md`
and plugin-manifest registration, baseline contamination controls that do not
name the completed book, evaluation-leaf sizing, and forward-referenced type
definitions.

Verified clean and recorded as such so the integration does not re-derive them:
the fifteen-file corpus and its exclusions against the real crate layout, the
research pair's justification and its configured cross-vendor routing, the
dependency order at every seam, complete file assignment across the seven
authoring slices, one clear owner for each named proof point, review-chain
adjacency under the pre-order walk, the skill lane's baseline-before-authoring
separation and shared rubric, and every brief pointer.

Findings warrant changes, so `integrate-review-planning` leaf `walkthrough-k24`
was inserted at the slot held by `walkthrough-method-k3`, the first later root
entry whose subtree holds live work. It carries the findings verbatim.
