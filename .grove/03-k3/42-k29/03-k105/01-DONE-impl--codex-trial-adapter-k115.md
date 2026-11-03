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

## Decisions (running log)

- One runner with a harness adapter. `run --harness codex` runs any case in
  Codex through the case's own tree, so later Codex leaves reuse the Claude
  Code cases case for case. A case's `model` and `effort` apply only to its
  own `harness`. Another harness takes its own defaults: `gpt-6.1-sol` at
  `medium` for Codex, as in Grove's Rhai sample. Case fields whose Codex
  mechanism belongs to a later leaf refuse for Codex with that leaf's name.
- The route is `codex exec --json --skip-git-repo-check --color never
  --sandbox workspace-write -c approval_policy="never"` with the model and
  `model_reasoning_effort`. The permission classifier of this session refused
  a Codex launch with `danger-full-access`. Under `workspace-write`, reads are
  unrestricted and writes reach the working directory, `TMPDIR` and `/tmp`.
  Ravel allocates its session channel under `TMPDIR`, so `end-session`
  reaches it. In a lifecycle case this sandbox runs in place of Ravel
  confinement. In a standalone case it runs inside Ravel confinement. The
  record states both. No `--add-dir`: Codex reads every path without a grant.
- The pre-launch check of the configuration directory is `codex debug
  prompt-input`, which makes no model call. Its skill roots table must name
  the sandbox `.codex/skills/.system`. `codex login status` must report a
  sign-in. The rendered input is the native startup text before launch.
- The events come from the session file
  `.codex/sessions/YYYY/MM/DD/rollout-*-<thread id>.jsonl`. Each
  `item_completed` `CommandExecution` names its argument vector and its
  working directory, so each command and each read gets absolute paths. The
  stream `--json` is the fallback when no session file exists.
- A real probe in a scratch sandbox home found two classes of
  platform-managed residue: the bundled `.codex/skills/.system` skills and
  the account's curated remote plugins in `.codex/plugins/cache`. The runner
  counts both as residue, not as native copies. The session file holds
  account identifiers and credit balances, so the record keeps only
  extracted fields.
- The first real smoke failed its link check after the launch. Codex links
  `apply_patch` and `applypatch` to its own executable under
  `.codex/tmp/arg0-*`, and it removes them only at a normal exit. Ravel ends
  the group first. The check now admits exactly those links when they point
  at the resolved Codex program, and the record lists them.
- Codex writes scripts of several lines. A newline outside quotes now ends a
  command before tokenising, so a `cd` on a later line moves later relative
  paths. This also applies to Claude Code's Bash calls.
- The second real smoke, `smoke-20261103T005733Z` at Grove `e46154df8d92`,
  passed every check. It credits no acceptance case. Its section in the
  acceptance record is "smoke in Codex".
- No in-session reviewer. Each refusal and the extraction have a control that
  fails against a broken runner, and the real smoke repeated them.
- `bash scripts/check.sh` refused host Bun 1.4.4 at the policy typecheck. The
  stage record's `mise exec bun@1.4.3 -- bash scripts/check.sh` passed all 21
  checks.
