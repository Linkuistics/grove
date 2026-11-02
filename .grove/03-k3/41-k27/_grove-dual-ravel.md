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

grove-dual-ravel-k28 cut these increments. Each leaves Grove green and the
TypeScript route unchanged:

1. `grove-ravel-runtime-fixtures-k94`: the test seam selects the runtime
   explicitly. Fixture configurations prove the native route on 2.0 for both
   callers.
2. `grove-rhai-static-assets-k95`: the shared case table, the Rhai sample and
   the Rhai static example.
3. `grove-rhai-review-assets-k96`: the Rhai review adapter, review examples and
   provider-evidence wrapper, with cross-runtime receipts.
4. `grove-ravel-two-admission-k97`: admission of 2.x from 2.0.0, its remedy,
   the native-route documentation and the books.
5. `grove-directory-lifecycle-k98`: the directory route through the lifecycle
   caller, settings isolation and lifetime. It then advertises the route.
6. `grove-directory-confinement-k99`: the confined standalone route, and the
   unreadable native case measured separately.
7. `grove-dual-archive-k100`: both asset sets in the archives, formula and
   installed smoke.
8. `grove-dual-target-inputs-k101`: the 2.0 lane of the pinned inputs, target
   and floor smoke, and legacy-driver continuity.
9. `grove-linux-two-lane-k103`: k101 found that the directory build does not
   link at glibc 2.17, because it calls `renameat2`. This leaf repairs Ravel,
   records the Linux 2.0 archives and runs the 2.0 lane on the Linux floors.
   Ravel 2.0.0 cannot be cut before this repair.
10. `grove-dual-ravel-handoff-k102`: full checks with each explicit build, and
    the stage record for k29 and k6.

Admission follows both asset leaves, because the delivery specification admits
2.x only after the Rhai assets pass. The directory leaves follow admission, so
the route is advertised last.

## Builds and homes

- **The 1.1 build.** The Homebrew keg `/opt/homebrew/Cellar/ravel/1.1.0`. Its
  `bin/ravel` SHA-256 is `c52f990b2059c8bbe8efba10a741d3a0311000ac1712d71e882638f54c3ac1ed`, and its worker build identity is
  `fa130cc262cfb22541798debe937ecd9e81488ebef494d873d377978808c9d9c`. It is also the ending executable of the live driver.
  Read it, and copy it into fixture prefixes. Never upgrade, unlink or change it.
- **The 2.0 build.** The glibc 2.17 repair of grove-linux-two-lane-k103:
  build commit `23e50ce90037`, with the parent `4f831feb4d0f`, output commit
  `1ca4aecbeae1` at the tip of the bookmark, and build identity `ef5857e989e6d99da51631ae0295685c8961c1b01bac58d865e6fb789ae71deb`. Its three archives are the 2.0 lane of
  `scripts/ravel_target_inputs.py`, and Grove's ignored
  `target/ravel-archives-2.0/` holds a copy. Install it with `task install
  PREFIX=<scratch prefix>` from the workspace `Ravel.skill-directory-delivery`
  with an empty working copy above the bookmark `skill-directory-delivery`.
  The installed `bin/ravel` SHA-256 on macOS is `8e10e311c50c81ca60ac4f393d17408982d0aafcfe46cccfbc46130465290ae2`. The
  build reports `ravel 1.1.0`, so identify it by commit and build identity. It
  installs one executable and no `libexec/ravel` worker. The earlier directory
  build of ravel-local-sources-k90, identity `9feb86b7…`, did not link on
  Linux.
- **A changed line.** If rollout rebases the bookmark before this node closes,
  record the new commits and identity. Do not move the bookmark here, except
  in grove-linux-two-lane-k103. That leaf adds the glibc 2.17 repair above
  `4f831feb` as a Ravel stage, and it records the new build here.
- **Homes.** Each route runs in its own temporary HOME, never the real one. The
  1.1 HOME holds `policy.ts` with schema 2 and settings with only `timeoutMs`,
  `contextBytes` and `policyEnv`. The native 2.0 HOME holds `config.rhai` with
  `SCHEMA_VERSION` 4 and no `skills`. The directory HOME adds a `skills` object
  whose sources have `type` `local`.

## Pointers

`docs/specs/codex-skill-provisioning.md`, especially Supported routes,
`docs/specs/harness-selection-and-execution.md` and the Rhai contract mapping
of `docs/grove-skill-delivery-design-evidence.md`. The preceding Ravel commits,
and the reviewed repairs ravel-rhai-skill-delivery-k34 and
grove-rhai-skill-delivery-k35.

The reviewed Ravel contract is commit `822c0ed2f67d` on the bookmark
`skill-directory-delivery`, narrowed to local sources at `7a894c782037` by
ravel-local-sources-k78. ravel-local-sources-k90 made the local-source
build: output commit `4f831feb4d0f`, build commit `78332fcc367d` and build
identity `9feb86b7982a3920`. The directory build is now its glibc 2.17
repair: build commit `23e50ce90037` and build identity `ef5857e989e6d99d`. Grove's adoption contract is the commit that
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
