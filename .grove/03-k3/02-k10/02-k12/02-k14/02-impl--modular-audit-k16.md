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
