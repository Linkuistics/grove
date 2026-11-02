# grove-dual-ravel-k27 — brief

## Goal

Make Grove explicitly compatible with the three supported routes of the
delivery specification. They are native on 1.1, native on 2.0 and directory on
2.0.
Include the installed caller assets. Rhai native compatibility on 2.0.0 does
not establish directory delivery.

## Done when

- Stable 1.x from 1.1.0 and stable 2.x from 2.0.0 pass actual public lifecycle
  and standalone caller acceptance. 2.0.0 is the actual minimum of the
  directory feature. The native route on 2.0 and the directory route have
  separate evidence. Unsupported versions, malformed output and failed probes
  refuse with precise remedies. Refusals keep the canonical selection of the
  first PATH entry.
- Shipped examples, typechecks, caller assets, fixtures and installed checks
  explicitly identify each supported runtime and schema. Port every caller
  asset to Rhai, which satisfies Grove issue 24. Keep receipt bytes and
  provider-safety checks identical, and keep the accepted runtime differences
  of the delivery specification visible. Keep the TypeScript assets at their
  installed paths for 1.x. One case table drives the checks of both sets, with
  the expected outcome for each runtime. No owner configuration is overwritten.
- Directory launches from local sources preserve the mandate, the endings,
  the mandatory order and resource access. One source is a plain Git checkout
  at a commit, and one launch runs without a network. A missing checkout
  refuses as `skill_source_missing` before any worker. Native-only lifecycle needs
  no sources. Confined native access is measured separately, without a
  directory-grant bypass.
- Caller specifications, ADRs and views, the glossary, source-exact books and
  release inputs agree with behaviour. Full Grove checks and the affected
  archive, formula and floor checks pass with explicit matching independent
  Ravel builds and homes.

## Decomposition

The planner consumes delivered Ravel builds before it cuts caller, version and
installed-distribution increments. A version predicate change alone cannot
establish compatibility or justify publication.

## Pointers

`docs/specs/codex-skill-provisioning.md`, especially Supported routes,
`docs/specs/harness-selection-and-execution.md` and the Rhai contract mapping
of `docs/grove-skill-delivery-design-evidence.md`. The preceding Ravel commits,
and the reviewed repairs ravel-rhai-skill-delivery-k34 and
grove-rhai-skill-delivery-k35.

The reviewed Ravel contract is commit `822c0ed2f67d` on the bookmark
`skill-directory-delivery`, narrowed to local sources at `7a894c782037` by
ravel-local-sources-k78. The directory build is the output of
ravel-local-sources-k90: output commit `4f831feb4d0f`, build commit
`78332fcc367d` and build identity `9feb86b7982a3920`. Grove's adoption contract is the commit that
retires grove-rhai-skill-delivery-k35, with the change that
grove-rhai-skill-delivery-k52 made. That integration of the review
grove-rhai-skill-delivery-k51 added the Runtime parity section of the delivery
specification. It accepts Ravel's three runtime differences, the same read
bound for the jj store path and one receipt difference. It keeps receipt bytes
and provider-safety checks identical.

## Notes

Keep separate legacy-driver fixtures and original ending executables. Final tap
publication follows the human's release decision recorded in
common-skill-loading-k32. Do not treat this later directory-adoption stage as
the prerequisite for Rhai-native compatibility. No tap mutation here.

A build of the feature line reports `ravel 1.1.0` until the cut. Identify each
Ravel build by its commit and build identity. Prove the admission of
`ravel 2.0.0` output with stub executables. Run each route in its own HOME, and
prove a native launch on 1.1 beside a directory launch on 2.0. Select the
`ravel@2` prefix through the PATH of the invocation. Keep the unversioned
dependency of Grove's formula.
