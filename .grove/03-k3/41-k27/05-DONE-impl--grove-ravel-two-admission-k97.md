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

## Decisions (running log)

**Admission.** `check_version` parses `ravel <major>.<minor>.<patch>` with
canonical numbers. Major 1 needs a nonzero minor. Major 2 admits every minor.
Every other major refuses. The result is a yes or a no, so the admission
selects no route.

**Remedy.** The remedy names 1.x from 1.1.0 with `policy.ts` and 2.x from
2.0.0 with `config.rhai`. It tells the owner to put the matching `bin` first on
the PATH of the invocation, and it names `$(brew --prefix ravel@2)/bin`. It
warns against an upgrade of a shared installation. The tests assert those
parts and the absence of `brew upgrade`. The missing-Ravel remedy
`brew install linkuistics/taps/ravel` stays, because it installs an admitted
1.x.

**Keg-only proof.** A unit case builds Homebrew's layout: an unversioned
`bin/ravel` link into the 1.1.0 keg and an `opt/ravel@2` link into the 2.0.0
keg. The default order selects 1.1.0, and `opt/ravel@2/bin` first selects the
2.0.0 keg path. The caller case `both_actual_callers_quote_a_keg_only_ravel_2_selected_first_on_path`
runs both callers against stubs. The stub records the `--prompt=` argument,
and both endings quote the keg path, not the `opt` link. The 1.1 stub never
runs.

**Controls, each seen to fail once.** A resolver that admitted major 3 failed
`rejects_the_selected_incompatible_version_without_falling_through`. A resolver
that removed an `-rc` suffix before the parse failed the same test. The source
was restored byte for byte after each.

**Documentation.** The delivery and caller specifications, the standalone
specification, the supervision ADR and the route view now state both native
routes. The routes caption marks the directory route as proposed, and no
document advertises it. `docs/USAGE.md` has a route table, the `ravel@2` PATH
step, the settings split and a Rhai example. This session ran that example
through public `start-session` of the 2.0 build in a temporary HOME, and it
returned `grove-continue`.

The configure-grove skill and its policy reference
state both configuration files, both settings key sets and the 2.x verification.
The glossary entries of the consumer map, `CONTEXT-MAP.md`, the architecture
rows and the changelog agree. The evidence documents keep their observations
as recorded.

**Book.** The overview fragment of `ravel.rs` now covers 434 lines. The ledger
and the roll-up totals moved from 1,090 to 1,173 lines. Chapter 05 changed only
those totals. The prose of chapter 06 states both majors and both runtimes of
the installed-pair test. Chapters 02 and 03 and grove-loop chapter 19 needed no
change.

**Results.** `GROVE_TEST_RAVEL` named the 1.1 keg. Then
`mise exec bun@1.4.3 -- bash scripts/check.sh` passed all 20 principal checks.
The diff digest was the same before and after the run.

The 2.0 build came from
`4f831feb4d0f`. Its `bin/ravel` SHA-256 was
`151635d25169e6a76c7eed0a1de64af661b3d77554c9bbef88ab85a59afb823d`. It ran
behind a host-guard stub that exits 86. `task ravel:resolution` and
`task ravel:activation` with `RAVEL_RUNTIME=2.0` passed: 155 Grove, 402
grove-loop and 227 grove-llm tests, and the resolver and authority sets.

**Review.** The change is one version predicate, its remedy and documentation.
Tests and two observed controls cover the predicate, so the work earns no
`review-impl` leaf, and this session spent no in-session reviewer.
