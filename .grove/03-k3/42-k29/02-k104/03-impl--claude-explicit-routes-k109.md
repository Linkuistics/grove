# claude-explicit-routes-k109

## Goal

Observe each explicit request row of Ravel's agent discovery rules in Claude
Code, on the directory route on 2.0.

## Context

The request table in the "Agent discovery and native installations" section of
Ravel's `docs/specs/skill-discovery.md`. The Directory bootstrap and Explicit
loading mechanism rows of Grove's delivery specification.

## Done when

Each case below is a separate launch with its own record. Each record compares
the observed route with the row of the spec.

- A bare name request, with native copies absent, loads the skill from the
  supplied directories.
- A plugin-qualified name request, such as `linkuistics:decision-records`,
  with native copies absent, loads from the supplied directories. It does not
  select a native loader.
- A native loader request with a native match uses the loader first.
- A native loader request with no native match says so. It then loads the
  same guidance from the supplied directories, with disclosure.
- A native loader error other than "no match" gives a report and no bypass.
- An express native-only restriction with no native copy gives a report
  before dependent work and no fallback.
- A concrete path loads that file. A failing concrete path gives a report
  before dependent work and no other location.

## Notes

The native loader of Claude Code is its Skill tool. Install a native copy only
inside the sandbox home, and record it.

Establish how to cause a loader error other than "no match" before the case.
If no such error can be caused, record that row as an unmeasured gap.
