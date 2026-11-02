# codex-trial-adapter-k115

## Goal

Add a Codex adapter to the trial runner of claude-trial-runner-k107. Prove it
with fake-harness tests and one real smoke launch.

## Context

- The runner and its record from claude-trial-runner-k107.
- The Codex row of "Sandbox home for real harness runs" in Ravel's
  `docs/specs/configuration-evaluation.md`.
- The Codex measurements in "Harness observations" of Ravel's
  `docs/skill-discovery-design-evidence.md`.

## Done when

- The opt-in task runs a named case in Codex through the same disposable Grove
  tree and 2.0 build.
- The sandbox `.codex` holds only the link to the real `auth.json`. Each run
  sets `CODEX_HOME` to it. The runner lists every link before and after the
  launch.
- The runner refuses an inherited `CODEX_HOME` in the real home before any
  launch, also an empty value or an alias. It replaces a value outside the
  real home and reports the replacement.
- The allow-list control of the runner holds for Codex, and a planted name
  makes it fail once.
- The event extraction reads the Codex session record. Each read and each
  command appears with its absolute path.
- Fake-harness tests prove the Codex refusals and the extraction. Each test
  fails once against a deliberately broken adapter.
- One real smoke launch reaches Codex with native copies absent. It records
  the configuration directory, the sandbox mode, the model, the reasoning
  effort and the native startup text. It credits no acceptance case.

## Notes

`codex debug prompt-input` renders the model-visible input. It can show the
native startup text without a model call.
