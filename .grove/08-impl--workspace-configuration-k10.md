# workspace-configuration-k10


## Goal

Make Grove launch and tree-mutation admission use modular personal policy and
workspace selections through the existing SessionConfig adapter. Demonstrate
isolated workspace arrangements and reload on the next session.



## Context

Depends on completed `configuration-engine-k6`. SessionConfig currently admits
at most one local candidate before calling Templates::load; migrate it to
Catalog and choose overlay selection, then primary selection, then an empty
list. TemplateSource remains a source, not a cached resolved snapshot.

Keep discovery/trackedness in Grove and the existing jj-workspace seam. The
generic runner must not learn Grove paths, session kinds or the default-list
policy. `configuration-inspection-k11` will use this same adapter and its
structured result/error surface.

## Done when

- The global personal file always participates. A present local selection
  replaces the default list, including explicit empty selection; absent local
  selection inherits it. Direct local values apply last and may complete
  parameters on a personally targeted kind.
- The worktree's candidate wins; only positive absence permits repository
  fallback. Tracked, unreadable, unparseable, structurally invalid, actively
  invalid or unprobeable chosen candidates refuse with source-attributed
  diagnostics. Never read or merge the lower-priority candidate after refusal.
  Existing ignore/trackedness protections and repository-selector scrubbing
  remain intact; preserve the approved jj metadata snapshot behavior.
- SessionConfig exposes the resolved inspection/diagnostic information the
  human CLI needs without maintaining a second resolver. Source discovery and
  admission errors map to the reviewed diagnostic shape, preserving paths even
  when there is no span. Existing kind validation and unconfigured-kind refusals
  still apply; no catch-all or closed kind list appears.
- The driver and mutating verbs retain their validation load points and primary
  kind authority. Invalid active policy fails before tree mutation or launch,
  even when the requested kind itself is valid. Local-only keys and inactive
  personal routes never authorize a mutation or launch by themselves.
- Temporary jj workspaces with fake executables demonstrate both lead/review
  arrangements, local replacement/default/empty selection, parameter-only
  experiments, workspace isolation, and exact argument vectors. Editing a
  selection or shared value during one child changes the next session's command
  without restarting or changing the running child's argv.
- Existing flat personal/local fixtures still launch the same commands. Source
  admission tests and reload tests continue to use the same production seams;
  no real agent process is needed and `.cargo/config.toml` remains intact.

## Verification and documentation

Use public SessionConfig load/require/expand plus the composed loop and fake
executables. Test both the pre-transition and pre-launch error boundaries;
observe no partial tree mutation or accidental fallback. Reuse existing jj
fixtures and loop seams instead of inventing a parallel driver.

Update the delivered configuration grammar and selection/reload behavior in
`docs/CONFIGURATION.md`, relevant usage/architecture prose, public crate docs,
specs/ADRs/glossary and release-facing descriptions. Clearly leave the pending
human CLI/example installation surfaces marked pending until their leaves land.
Update the grove-loop source-exact book, especially configuration discovery and
driver/epoch explanations, plus any other manifest-covered source touched.
Run the root brief's common checks.

## Notes

The binary running the current Grove loop does not acquire edited Rust behavior
until rebuilt and installed. Acceptance tests must execute the new build, using
the repository's cargo signal guard for fake children; actual Grove tree verbs
in this task still use `grove-llm` directly.
