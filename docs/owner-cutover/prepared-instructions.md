# Prepared instructions for the owner skill switch

This file is `INSTRUCTIONS.md` in the owner's prepared folder
`/Users/antony/.local/state/owner-switch/prepared`. It stays readable after
Grove removes `.grove/`, after finish ends and after the native skill
installations go. Each command below names absolute paths. No command needs
a Grove workspace.

The design is `docs/specs/owner-skill-delivery.md` in the Grove repository.
Its sections "The native settings inside finish", "The switch inside finish",
"Resuming the switch", "The rehearsed plan" and "Rollback" own each rule.
`docs/owner-cutover/owner-checklist.md` lists the writers to pause.

## The prepared folder

| File | Purpose |
| --- | --- |
| `plan.json` | The real switch plan, schema `owner-switch-plan/2` |
| `owner_switch.py` | The switch program. The plan calls it `$program`. |
| `inventory.py` | The baseline instrument of step 3 |
| `owner_switch_probes.py` | The probe program of step 7 and of restore |
| `harness_trial.py` | The harness adapters, which the probe program imports from its own folder |
| `owner_switch_dry_run.py` | The comparison instrument |
| `staging/` | The frozen staging runner of gate 4, with its helpers, cases and settings at their repository paths |
| `INSTRUCTIONS.md` | This file |

The evidence section "Owner switch plan" of
`docs/owner-skill-delivery-evidence.md` names the SHA-256 of each file. A
changed file needs another dry run before an attempt.

## The interpreter

Each command below uses mac-check's machine interpreter:

```text
/opt/homebrew/opt/python@3.14/bin/python3.14
```

No project's mise setting replaces it. The program runs on Python 3.9 and
later.

## Native delivery: `native-off`

Finish runs this command immediately after it promotes Ravel. The owner
runs the same command after a crash or after a stop before the switch's
backup:

```sh
/opt/homebrew/opt/python@3.14/bin/python3.14 -I /Users/antony/.local/state/owner-switch/prepared/owner_switch.py native-off --home /Users/antony
```

Before the command, pause the writers of that settings file. mise tracks
`~/.config/ravel` through `[dotfiles]`. So run no `mise bootstrap`,
`mise dot apply` or `mise dot sync`. Edit the file through no other program.
The command refuses other Ravel drivers, but it cannot exclude a writer after
its last comparison. Keep the pause until a resolution reports `disabled`.

It writes only `/Users/antony/.config/ravel/settings.json`. Exit 0 means
that the off value is in place. After exit 3 or 4, native delivery is
incomplete, and no new default session may start. Repeat the same command
after the cause is removed.

To print the exact command as JSON, add `--print-command`. That form writes
nothing.

## The current state: `status`

```sh
/opt/homebrew/opt/python@3.14/bin/python3.14 -I /Users/antony/.local/state/owner-switch/prepared/owner_switch.py status --plan /Users/antony/.local/state/owner-switch/prepared/plan.json
/Users/antony/.local/bin/mac-check --only "Owner skill switch"
```

`status` names the delivery health, the governing attempt and each restore
command that is due. It writes nothing.

## The comparison before each attempt

The operator compares the dry-run result with the real prepared inputs
before each real attempt. The comparison reads only:

```sh
/opt/homebrew/opt/python@3.14/bin/python3.14 -I /Users/antony/.local/state/owner-switch/prepared/owner_switch_dry_run.py compare \
  --result /Users/antony/.local/state/owner-switch/dry-run/result.json \
  --prefix grove-keg=/opt/homebrew/opt/grove \
  --prefix ravel=/opt/homebrew/opt/ravel \
  --out /Users/antony/.local/state/owner-switch/comparisons/<UTC time>.json
```

Keep the output file. Exit 0 means that each row is equal or substituted.
Do not start an attempt after another exit status.

The comparison allows only three logical prefix substitutions:

| Name | Path in the dry run | Path at the real attempt |
| --- | --- | --- |
| `home` | The dry run's disposable HOME, `<OUT>/home` | `/Users/antony` |
| `grove-keg` | The Grove keg that the dry run named | The released Grove keg that `/opt/homebrew/opt/grove` reaches |
| `ravel` | The `ravel@2` keg that the dry run named | The promoted Ravel keg that `/opt/homebrew/opt/ravel` reaches |

The comparison resolves each given prefix to its canonical path. A
substitution passes only when the content and the versions are equal.

The comparison has a row for each tool version, including Codex. It has no
row for the planned originals of the write set. Before each attempt, also
run the read-only plan report:

```sh
/opt/homebrew/opt/python@3.14/bin/python3.14 -I /Users/antony/.local/state/owner-switch/prepared/owner_switch.py report --plan /Users/antony/.local/state/owner-switch/prepared/plan.json --home /Users/antony
```

Each `file` and `entry` line must end with "now its original". Before
`native-off`, only `.config/ravel/settings.json` may say
"now differs: absent". Another differing line needs another plan draft and
dry run. The owner's checklist states the steps.

### The result before finish

owner-switch-plan-redraft-k340 retained the current result at
`/Users/antony/.local/state/owner-switch/dry-run/result.json` on
2026-11-13. It replaced the first result of owner-switch-rehearsal-k255,
after an edit of `~/.claude/settings.json` changed a planned original. That
run used a stand-in Homebrew prefix,
`/Users/antony/.local/state/owner-switch-dry-run/prefix`. The prefix holds
clones of the installed skill kegs, the candidate Grove keg 22.8.0 and the
accepted Ravel 2.0 build. Its comparison passes only with
`--path /Users/antony/.local/state/owner-switch-dry-run/prefix/bin` and the
prefixes of that run. With the command above, the rows of `brew`, `ravel`,
`grove` and the ten sources differ, and so does the plan's retained path.

Finish therefore runs the dry run again after the Grove release and the
Ravel promotion. That run uses the real prefix and no stand-in. Move the
earlier `dry-run` folder aside first. Then compare with the command above.
The section "Owner switch rehearsal" of
`docs/owner-skill-delivery-evidence.md` names the full dry-run command.

## The attempt note

The `attempt` entry writes one attempt note before the switch:

```sh
/opt/homebrew/opt/python@3.14/bin/python3.14 -I /Users/antony/.local/state/owner-switch/prepared/owner_switch.py attempt --plan /Users/antony/.local/state/owner-switch/prepared/plan.json --home /Users/antony
```

The note is `/Users/antony/.local/state/owner-switch/attempts/<attempt>.json`,
mode 0444. Before you run its `switch` command, read the note. Its path and
each field must name this attempt. The note records instructions, not progress. Its
fields are these:

| Field | Content |
| --- | --- |
| `schema` | `owner-switch-attempt/1` |
| `kind` | `switch` |
| `attempt` | The attempt ID, which names the note and the backup |
| `home` | The canonical HOME, `/Users/antony` |
| `plan` | `{"path", "digest"}` of the prepared plan |
| `program` | `{"path", "digest"}` of the prepared program |
| `inputs` | `[{"role", "path", "digest"}]`, the planned command inputs |
| `tools` | The observed tool versions |
| `backup` | `/Users/antony/.local/state/owner-switch/backups/<attempt>` |
| `switch` | The literal switch command |
| `restore` | The literal restore command |
| `native_off` | The literal `native-off` command |
| `created` | The UTC time of the note |

## Restore: `restore --backup`

A caught failure restores by itself. After a crash or an interruption after
the backup's publication, run the `restore` command of the interrupted
attempt's note. Choose the note of that attempt. Never choose the latest
backup by inference. Its form is this:

```sh
/opt/homebrew/opt/python@3.14/bin/python3.14 -I /Users/antony/.local/state/owner-switch/prepared/owner_switch.py restore --backup /Users/antony/.local/state/owner-switch/backups/<attempt> --attempt /Users/antony/.local/state/owner-switch/attempts/<attempt>.json --home /Users/antony
```

Add `--guidance PATH` for each guidance file that the operator session read.
After an interrupted restore, repeat the same command under the same quiet
interval. A later restore of a switched layout first writes a recovery note.
Its retry command adds `--recovery <recovery note>`, as `status` prints it.

Exit 0 means that restore found native delivery in place. Exit 4 means that recovery is
incomplete and the backup stays. Exit 3 means a refusal with no write.
Follow the message, and do not start a new attempt while an earlier attempt
is unresolved.
