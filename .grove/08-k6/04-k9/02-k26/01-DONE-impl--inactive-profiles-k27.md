# inactive-profiles-k27

## Goal
Allow structurally valid inactive personal profiles beside working base commands,
while refusing explicit profile resolution until the occurrence fold lands.

## Done when
- Profile names, bodies, includes and patch shapes/duplicates validate at load,
  including inactive profiles; local profile definitions are refused.
- Empty selection ignores profile semantics and preserves base argv, authority,
  inspection and conformance after source capture. Known selected profiles fail
  explicitly as unsupported, distinct from unknown names; no silent base fallback.
- Grove launches base commands with inactive profiles and keeps refusing either
  source's selection declaration, including empty lists, before mutation/spawn.
- Affected documentation and source-exact books agree; scripts/check.sh passes.

## Plan
1. Extend Catalog tests for inactive semantic failures, structural refusals,
   namespace separation, captured-source independence and selected refusal.
2. Reuse patch validation in profile scope; retain original KDL and profile
   declaration spans. Keep the base resolver and Grove guard intact.
3. Extend the existing Grove launch/declaration fixture with real profiles.
4. Update boundary prose and source-exact fragments, then run focused tests and
   the principal checks before retirement and the task commit.

## Decisions (running log)
- The approved modular spec remains the design. This is its inactive-profile
  vertical slice, not a new grammar. The graph generation 2026-09-16T11:54:39Z
  predates the named resolver; coverage reported changed/untracked metadata, so
  source inspection supplies the implementation evidence.
- Profiles retain their source in captured KDL. This increment validates their
  patch structure without applying it and records definition spans to distinguish
  known-but-unsupported selections from unknown names. The next leaf owns the
  occurrence-aware fold and all selected semantics.
- Focused public tests establish the isolation property directly: leaking inactive
  bindings/values would invalidate the base fixture, and leaking routes would
  admit the local-only key. Structural cases exercise independent namespaces and
  duplicate/shape rejection. These executable checks are the doubt instrument;
  a fresh read of the same small parser branch adds less evidence here. No
  in-session reviewer was materialized.
- Known selected profiles use the existing shape category with a temporary
  unsupported-composition remedy, selection occurrence, and related definition
  span. Unknown names retain unknown_profile. k28 replaces this interim refusal.
- The principal run found the repository's seven-argument Clippy limit on the
  extracted helper. Grouped the path/text/role in ParseSource; the failed run was
  stopped before editing, and final verification starts again on frozen inputs.

## Verification
- Catalog profile tests failed first on the old unsupported-profile parser, then
  passed with structural validation and explicit selected refusal.
- Focused Grove launch/root-creation and leaf-add acceptance passed with real
  alternate profiles, personal/local declarations and explicit empty lists.
- The old named-command rejection fixture now uses a malformed profile without
  a body; valid empty profiles are positively covered by Catalog acceptance.
- Final `bash scripts/check.sh` passed all eight principal checks, including
  workspace tests and all six final source-exact books. The keyed-launch book
  reconstructs 3,922 lines with no deferred source. SHA-256 comparison of all
  1,805 tracked input files before/after the run found no changes.
- k27's Done when is satisfied. k26 remains live through k28; occurrence
  composition, selected validation/provenance and the k9/k6 closure audits are
  explicitly owned there. No ancestor closes in this session.
