# 030-hosting-api-spike

**Kind:** work (spike — record direction with evidence)

## Goal

**Build-discover the hosting-API shape** of the [[trellis framework]]:
**library-you-link** (`run(app)`, the consumer app owns `main`) vs
**runtime-you-plug-into**. Spike both against the **real vendored zellij
client/server internals** and **record the chosen direction with evidence**, so
110-native-host-api can build the MVP host API against it.

## Context

- Depends on **010/020** (the fork vendored and launchable). This is the spike
  that ADR-0020 §Notes flags as build-discovered, not decided in the abstract —
  per this tree's precedent (leaf 080 surfaced the `cli_pipe_output` constraint
  empirically).
- **library-idiomatic is preferred** (ADR-0020 Notes) but the call is empirical:
  it hinges on how zellij's `main`/client/server bootstrap is structured and
  whether grove can own `main` and `run(app)` without fighting the runtime.
- The deliverable is *direction + evidence*, not the full trait design — that's
  110's job. Keep spike code throwaway; the durable output is the recorded
  decision (an ADR or a brief note promoted upward at finish).

## Done when

- Both directions tried far enough against real internals to judge fit; the
  chosen hosting-API direction is **recorded with the evidence** that decided it.
- 110-native-host-api has a clear seam to build the MVP API against.

## Notes

- If the evidence is strong and the decision consequential, raise an ADR;
  otherwise a brief note suffices (lazy — constraint 4).
- Throwaway spike code does not need to be production-clean; the decision is the
  artifact.
