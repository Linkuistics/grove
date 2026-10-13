# grove-ravel-policy-k28 — brief


## Goal
Deliver Grove-owned policy adapter, sample/examples and provenance through
Ravel's generic routing/context SDK and ordinary readable policy imports.



## Context
Preserve applicable assets and runtime/typecheck fixtures from the old
crates/harness-dispatch package. The generic Ravel SDK has no host.run or durable
run lookup. k27 owns the runtime callers; k5's parent contract covers the complete
adapter/provenance obligations and the legacy original-driver exception.

## Done when
- Grove-readable assets replace embedded Grove module registration. Sample and
  examples import ravel/sdk through documented policy imports and typecheck/run
  against the independent installed worker with fake agents.
- Creator/Reviews/provider needs are supplied as caller-owned context and records;
  no retired host.run, record verb or Ravel store is accessed. Current retirement
  and Creator instructions explain new provenance and the named legacy v22 case.
- Caller context production and consumption are exercised together, including
  missing/invalid creator handling; actual personal policy choices are preserved.
- Changed source-pinned books/manifests/indexes match; focused checks and the
  full relevant adapter/runtime suite pass through reusable tasks.

## Notes
Ravel remains generic and unchanged. Grove owns and will ship its assets through
k6. Keep old package/build fixtures needed by pre-k6 archive checks.

## Execution slices

This migration reaches three independently testable boundaries. Only the first
child runs in the decomposing session:

1. grove-ravel-static-assets-k40 adds readable Grove static examples and the
   sample beside the old package, with public-SDK typechecks and installed-worker
   fake-agent checks. It preserves the sample's choices and workspace grants;
   it neither changes the owner's policy nor claims creator-aware routing.
2. grove-ravel-provenance-k41 delivers caller-owned provider evidence and context
   production through the actual launch/retirement path, with the named original
   v22-driver case and matching guidance. It settles the evidence contract the
   review adapter will consume; Ravel supplies no private run lookup.
3. grove-ravel-review-policy-k42 migrates the review adapter and examples to that
   contract and the generic SDK, exercises production and consumption together,
   and checks this node's complete obligations before closing it.

## Decisions (running log)

- Decompose at the existing static-policy / run-provenance / review-consumer
  seams. The current adapter's `lookUpCreator` calls `host.run`; Ravel 0.1.0's
  `ContextHost` has only measured reads. A spelling change cannot preserve that
  behavior. Keep the old embedded assets for pre-k6 archive checks while the new
  Grove-readable assets expand alongside them.
- Put new readable policy assets under `assets/ravel-policy/`; k6 owns their
  installed Grove layout. Owners copy/import these files normally. Ravel's
  generic embedded `ravel/sdk` and `ravel/examples/static` remain the only SDK
  imports used by the static increment.
