# modular-audit-k16

## Goal
Finish the repository-wide configuration-forms audit and current prose
reconciliation after modular-types-k15, then decide subsystem review.

## Done when
- Enumerate configuration forms across the repository and classify every old-form
  survivor as rejection evidence or historical material; use the controls in
  Grove's execute procedure rather than a selected-symbol search alone.
- Reconcile remaining prose, especially keyed-launch book examples and roll-ups,
  its structure brief, overview chapters 1/5, keyed-launch lib.rs and the Architecture
  configuration-delta section. Include all root/ancestor contract surfaces.
- Run scripts/check.sh and decide a review-impl leaf under Grove's threshold for
  the completed removal, covering load rejection, authority, expansion and provenance.
- Check and close ancestors only when all their acceptance criteria hold.

## Context
modular-types-k15 removes the resolver/inspection variants and updates their
immediate public type contract, renderer consumers and source-exact projections.
The current leaf owns the wider forms audit; do not infer it from that cleanup.
Keep the settled public loading/expansion test seam; no new CLI acceptance suite.

## Handoff from modular-types-k15
The full check passed, including all six books. Public type and renderer cleanup
is complete; lib.rs and the immediate fold/inspection explanations were updated.
The source-exact validator establishes bytes, not the remaining prose claims.
In particular, audit old running examples and the `Templates::source` commentary:
command definitions are personal-only, while inspection carries local binding,
route and parameter origins. Chapter 2 now states that distinction; other book
chapters and the public source accessor comment may still carry the old whole-file
replacement explanation. This is a concrete lead, not the exhaustive forms audit.

## Decisions (running log)

- Reconcile current examples and provenance prose with modular composition;
  `source()` identifies the personal command definition, while inspection carries
  route, binding and parameter origins. Keep comment line counts unchanged so
  source-exact projection coordinates do not churn.
- The audit includes hidden repository files, root documents, shipped plugin
  guidance, Mermaid design sources and all books. Graph generation
  `2026-09-17T05:07:07Z` has stale configuration locations and excludes docs;
  current source and text enumeration are the evidence for this leaf.
- The completed loader/resolver is a load-bearing subsystem. Schedule a
  `review-impl` sibling covering the entire removal and the documentation audit;
  do not spend an in-session review beside that scheduled review.

## Delivered evidence

- `docs/configuration-forms-audit.md` records the repository-wide forms inventory,
  old-form survivor classifications, search controls and graph coverage limits.
  The tracked-file cross-check found only the AGENTS.md symlink omitted by rg;
  its CLAUDE.md target was searched. Packaged KDL, generated fixture documents,
  current guides/specs, diagrams, plugin guidance and all book surfaces were included.
- Reconciled command-definition provenance, active semantic validation and local
  selection policy in source comments and their exact fragments. Converted the
  remaining current flat running examples, corrected book summaries/indexes and
  marked the preservation baseline's old grammar as historical. No runtime
  implementation or test behavior changed in this leaf.
- `bash scripts/check.sh` passed all eight principal checks, including workspace
  tests and all six final books. SHA-256 comparisons of all 1,800 tracked file
  subjects before/after the run matched individually. Final comment/prose fixes
  were followed by passing `cargo fmt --all --check` and final `book-check
  --check all` runs for keyed-launch, grove-loop, grove-llm and overview. Their
  per-book source totals were unchanged and all final-run subject hashes matched.
- Ancestor criteria are represented by the earlier removal/migration leaves and
  this audit; no implementation gap was found at this boundary. The scheduled
  subsystem review keeps modular-representation-k14 and its ancestors live.
  The review must recheck acceptance and settle any findings before their closure.
