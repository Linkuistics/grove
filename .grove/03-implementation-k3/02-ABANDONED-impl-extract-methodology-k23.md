# extract-methodology-k23


## Goal

Extract methodology identity, session kinds, corpus, prompts, and harness semantics into a deep `grove-methodology` crate.



## Context

The crate defines what Grove methodology is and how semantic session metadata is interpreted. Application-specific provisioning side effects and CLI presentation remain in the root runtime.

## Done when

- Contract tests are written against a small proposed API for methodology version/hash, `SessionKind`, corpus/resource lookup, prompt/session requirements, and harness-facing semantics.
- A new workspace crate owns those concepts and their domain errors without importing CLI, workspace discovery, VCS, task-tree storage, or finish.
- Existing include/embed/build-script and install/provisioning behaviour is preserved at the application boundary; published package contents and reproducible hash checks remain correct.
- Root consumers migrate to the public crate API, reach-through imports disappear, and the old methodology modules/tests/dependencies are removed.
- Methodology skew diagnostics and the Grove driver/skill contract continue to work under the baseline release/MSRV constraints.

## Notes

Do not expose a filesystem path or raw embedded-directory API as the primary seam. Callers should ask semantic questions such as session-kind metadata or named methodology resources.
