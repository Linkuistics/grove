# grove-ravel-two-admission-k97

## Goal

Admit stable Ravel 2.x from 2.0.0 beside stable 1.x from 1.1.0. Make the
caller documentation state the native routes on 1.1 and on 2.0.

## Context

- The "Launch and compatibility" section and the "Compatibility" and "Prefix
  selection" rows of the delivery specification.
- `crates/grove/src/dispatch/ravel.rs`: `locate_on_path` and `check_version`.
- The "Caller/version adoption" and "Source-exact runtime books" rows of the
  handoff table in `docs/grove-skill-delivery-design-evidence.md`. That table
  is a consumer map, not an inventory of stale strings.

## Done when

- The resolver admits stable 1.x from 1.1.0 and stable 2.x from 2.0.0. The
  admission selects no route.
- It refuses 1.0, prereleases, 3.x and other unknown majors, malformed output
  and failed probes. It keeps the canonical first PATH entry and never falls
  through to a later entry.
- The remedy names both admitted majors with their configuration files. It
  names the `ravel@2` prefix until promotion. It recommends no undirected
  shared upgrade.
- Stub executables prove the admission of `ravel 2.0.0` output. A PATH that
  puts a keg-only `ravel@2` bin first selects it. The ending commands then
  quote that canonical path.
- The caller specification, `docs/specs/standalone-invocations.md`, the
  supervision ADR and the route view state both native routes.
- Owner guidance states both native routes with their configuration and
  settings files. It covers `docs/USAGE.md`, the configure-grove skill and
  the settings guidance. Each owner configuration stays untouched.
- The glossary entries of the consumer map, `CONTEXT-MAP.md` and the
  architecture rows for Ravel and the caller assets agree with behaviour.
- Each source-exact book that quotes the changed code is regenerated. Every
  book check passes.

## Notes

Do not advertise the directory route here. grove-directory-lifecycle-k98 does
that after its own acceptance. Controls: an admission of `ravel 3.0.0` or of
`ravel 2.0.0-rc.1` must fail a test. Watch each fail once.

Run the prose checker on each changed document.
