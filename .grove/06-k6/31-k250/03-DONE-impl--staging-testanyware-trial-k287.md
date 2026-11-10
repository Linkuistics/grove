# staging-testanyware-trial-k287

## Goal

Prove on each harness in the staging HOME that the TestAnyware guidance
reaches its command reference and its installed command through Ravel
delivery.

## Context

- The parent brief holds the trial list and the staging layout.
- "Resources and requirements of each source" of
  `docs/repository-moves-record.md` names the tools of the `testanyware`
  source.
- The staging HOME does not isolate SSH. OpenSSH reads the account's real
  `~/.ssh`, as the guidance case of owner-staging-discovery-k249 found.

## Done when

- On each harness, an agent reads `using-testanyware` and its command
  reference from the `testanyware` source.
- On each harness, the agent runs one real VM operation through the
  installed `testanyware` command, and the VM is gone afterwards.
- The evidence document records each trial, its versions and its result.

## Decisions (running log)

- A read-only probe on 2026-11-10 ran `testanyware vm list --json` 2.8.0
  with HOME set to the staging HOME. It failed as `APP_UNREACHABLE`
  `not_running`. The CLI looks for the app's socket at
  `$HOME/Library/Application Support/TestAnyware/testanyware.sock`
  (`default_socket_path` in `testanyware-app-protocol/src/socket.rs` at
  `v2.8.0`). The app runs once for each account, in the real HOME.
- In a scratch HOME, `TESTANYWARE_APP_SOCKET` with the real socket alone
  failed as `APP_INCOMPATIBLE` `directories`. The CLI derived its state and
  data folders from the scratch HOME. A `config.rhai` with the real
  `state_dir` and `data_dir` added to the socket listed the five golden
  images.
- The runner joins the app for the TestAnyware case only. It sets
  `TESTANYWARE_APP_SOCKET` and `TESTANYWARE_CONFIG`, which names a file in
  the trial folder. The retained staging HOME and the launcher's allow-list
  for other cases stay unchanged. A link out of the staging HOME would break
  the isolation rule that only the two sign-in links leave it. Ravel passes
  the caller's environment to the managed process, as
  `docs/specs/session-execution.md` of Ravel states.
- `codex sandbox` 0.167.0 with `sandbox_mode="workspace-write"` denied the
  socket with "Operation not permitted". With
  `sandbox_workspace_write.network_access=true`, or with the owner's
  `default_permissions=":danger-full-access"`, the same command listed the
  five images.
- The case field `codexPermissions` replaces the trial's `--sandbox
  workspace-write` with the owner's `-c default_permissions=<value>`, as the
  owner's `config.rhai` passes it. The first Codex run uses the case
  without the field. It is the negative control of the verdict on real
  records.
- A calibration start of a Linux VM outside Ravel took 18 s. The start
  wrote `<id>.json`, `<id>.meta.json`, `<id>.keeper.log` and
  `<id>.tart.log` to `~/.local/state/testanyware/vms`. The stop removed the
  first two and the VM from `vm list`. The case uses Linux, which takes no
  macOS guest slot.
- The verdict `testanyware` reads runner facts and events. The runner lists
  the state folder and the running VMs before and after the session. A new
  VM ID must be the line of `ANSWER.md`, must not run afterwards and must
  have no spec file. `screen.png` must be a PNG. The events need a read of
  the skill from its source, then a reference command, then a successful
  `vm start` and a later successful `vm stop`. The runner stops each new VM
  that still runs, and records the stop.
- `testanyware-vm-codex-1` ran without `codexPermissions`, as planned. The
  agent read the skill (event 8) and ran `llm-instructions` (event 10).
  `testanyware doctor` then reported that the sandbox denied the socket.
  The verdict failed with "no successful testanyware vm start", and the
  state folder gained no file. The case then gained
  `"codexPermissions": ":danger-full-access"`, SHA-256 `786e0d9f…`.
- Claude Code's auto-mode classifier refused the session
  `testanyware-vm-codex-2` as "Create Unsafe Agents", because that Codex
  agent runs with full access. This session does not work around the
  refusal. The Claude Code case ran next, with the same
  `bypassPermissions` mode as each earlier staging trial.
- `testanyware-vm-claude-1` passed. VM `testanyware-cdd0e247` started and
  stopped in one shell call, and the screenshot was a 1920×1080 PNG.
- A `codex sandbox` probe with a copy of the staging `config.toml` showed
  that `sandbox_workspace_write.network_access=true` still opens the
  socket beside `default_permissions = ":workspace"`. `codexConfig`, a list
  of `-c` overrides that keeps the sandbox and the approval policy,
  replaced `codexPermissions`. The case's SHA-256 became `ce9ac803…`.
- `testanyware-vm-codex-2` passed with the network override. VM
  `testanyware-e98a94a6` started and stopped.
- `staging_trial.py rejudge` judged the first two runs again with the
  committed runner `eec79326…`. Each verdict equalled its first reading.
- `testanyware doctor` fails `login_item` in the real HOME: the app does not
  start at login. The switch's probe accepts a failed doctor check. The
  evidence names the setting as the owner's to change.
