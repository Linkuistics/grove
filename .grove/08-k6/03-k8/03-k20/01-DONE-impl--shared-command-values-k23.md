# shared-command-values-k23


## Goal

Deliver primary and local `values` patches for named commands through the public
Catalog/Templates seam and Grove launches, including removal and provenance.



## Context

## Done when

- `values` has the reviewed shape and uniqueness rules; `param` assigns and
  `unset` removes shared overrides, with final-state schema/reference validation.
- Local values complete required declarations, including unused declarations;
  exact opaque/empty strings survive expansion and active NUL values fail.
- Shared changes reach every admitted user. Histories retain overwritten values
  and removals; parameter and word origins identify declarations and winners.
- Public tests cover captured-source independence, convenience equivalence,
  dormant templates and active value diagnostics. Grove launch/mutation seams
  cover local shared values and invalid policy refusal.
- Docs and source-exact books describe this usable increment; route parameter
  patches remain explicit errors. Root checks pass.

## Notes

Route exceptions, target reset histories and missing personal targets belong to
route-parameter-overrides-k24, which audits the complete parent contracts.

## Decisions (running log)

The approved modular specification supplies the design. Add shared parameter
maps behind the existing Catalog seam, preserving the compiled named fragments
and immutable command declarations. Validate surviving shared assignments after
the primary/local fold, even for commands without an admitted route; demand
required values only when instantiating admitted routes. No public patch API or
new dependency is needed.

Four shared-value public tests first failed on unsupported syntax, then passed
after implementation. Their observable assertions cover precedence, removal,
opaque words, source-independent snapshots, structural errors and active-value
diagnostics. Grove acceptance additionally observes exact local-value argv and
refusal before creating a task tree. The bounded independent review found no
actionable findings in final-state validation or provenance. The existing KDL
accessors and splitting behavior are unchanged; no new version-dependent API
decision was introduced.

Verification: `bash scripts/check.sh` passed all eight principal checks, including
workspace tests and final validation of all six books. SHA-256 manifests over
all 1,801 tracked files (source, tests, configuration, scripts, documentation and
task artifacts) matched before and after that run. The keyed-launch book owns
3,591 source lines with no deferred ranges. No corpus exceptions were added.
The shared-value public tests and Grove launch/refusal acceptance passed in both
focused runs and the full suite. Route work remains live in the next child, so
neither parent node closes in this commit.
