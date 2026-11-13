# Finish handoff

<!-- tc: depart synonym-rotation because "check" names the repository check suites and the freeze check, which are task names, and "verify" is the only verb for that action -->

This document is the handoff to the driver-created finish session of the
`gh-issue-25` grove. It names the whole finish sequence in order, with the
command and the pass condition of each step. It also names the recovery for
each failure and the owner's operation after finish.

finish-handoff-k324 wrote this document on 2026-11-13. It is a plan, not a
receipt. On that date, no step below was complete. A request to plan is not the finish
confirmation.

The linked sections own each rule. This document orders the steps and names
their inputs. If a linked section and this document disagree, the linked
section governs, and the disagreement is a defect of this document.

- [The coordinated release design](specs/coordinated-release.md), steps 6
  and 7 of "Publication ownership and order" and "Recovery boundaries"
- [The owner skill delivery design](specs/owner-skill-delivery.md), from "The
  final session and new launches" to "Purge"
- [Finish promotes after the Grove release](adr/finish-promotes-after-the-grove-release.md)
- `CLAUDE.md`, "Finish, integrate, and release", and [the release procedure](RELEASING.md)
- [The owner's checklist](owner-cutover/owner-checklist.md) and
  [the prepared instructions](owner-cutover/prepared-instructions.md)
- Ravel's `docs/specs/release-channels.md`, "Promotion interface" and
  "Consumer readiness receipt"

## The one confirmation

Finish keeps its normal single confirmation. Its proposal names the complete
sequence below, including the promotion, `native-off` and the owner skill
switch. The owner's one explicit confirmation of that proposal authorises
the whole sequence. Finish does not ask again for a single step.

Some steps are owner actions, such as the pause steps of the owner's
checklist. Finish asks the owner to do them when the sequence reaches them.
That request is not a second confirmation of the sequence.

Finish keeps its teardown helper `grove-llm finish-commit` and its exact
original ending command. Its launch prompt supplies that command. For this
grove the command names `/opt/homebrew/Cellar/ravel/1.1.0/bin/ravel`.

If promotion, `native-off` or the switch is incomplete, the finish is
incomplete. Then finish never signals success.

## The inputs

| Subject | Value |
| --- | --- |
| Required Ravel | 2.0.0, script and request schema 4. Promotion publishes the same archives. It cuts no version. |
| Ravel tag | `v2.0.0`, tag commit `ed6cedbf9a14cd088bea13fcfb8b86300cb6aaca` |
| `ravel-v2.0.0-aarch64-apple-darwin.tar.xz` | SHA-256 `5957840ab33f4331b794539dbd455d801f71c6c6485c781dc290cfb48490231d` |
| `ravel-v2.0.0-aarch64-unknown-linux-gnu.tar.xz` | SHA-256 `a4750ea439450e88621be411183e7bf30d2616f9a545b64a30445df3a9f02bee` |
| `ravel-v2.0.0-x86_64-unknown-linux-gnu.tar.xz` | SHA-256 `906e589f72161ed5dce5db683e6af10963c1952bdf177c6b343d1918928c3dd1` |
| The `ravel@2` prefix | `/opt/homebrew/opt/ravel@2`, which reaches `/opt/homebrew/Cellar/ravel@2/2.0.0`. Its `bin/ravel` has SHA-256 `67c92bed7ae2693d71590368a74416c8d4997f4789b79f3984f4f2f3db791f4c`. |
| Ravel's release evidence | `~/Development/AgenticOrchestration/Ravel/target/release-evidence/v2.0.0.json`, SHA-256 `02a120ea4ac083db0ff6f3026749c88d68114052a671dc24cd1771a9def5ad5e`. [`ravel-v2.0.0-release-evidence.json`](ravel-v2.0.0-release-evidence.json) is its byte copy. |
| The original Ravel keg | `/opt/homebrew/Cellar/ravel/1.1.0`. It stays through the final signal. |
| The running driver | Grove 22.8.0 at `/opt/homebrew/Cellar/grove/22.8.0/bin/grove` |
| Cleanup protection | `HOMEBREW_NO_CLEANUP_FORMULAE=ravel` in `~/.homebrew/brew.env`, as [Homebrew cleanup protection](coordinated-release-evidence.md#homebrew-cleanup-protection) records |
| The promotion decision | promotion-upgrade-rehearsal-k318 decided to promote in finish. [Decision](promotion-upgrade-rehearsal-record.md#decision) gives the evidence and two conditions. promotion-upgrade-remedy-docs-k338 met the first condition before finish. Step 6 below meets the second. |
| Held Claude Code | `2.1.322`, held by `DISABLE_AUTOUPDATER` in `~/.claude/settings.json` |
| Frozen marketplaces | `claude-plugins-official`, `linkuistics` and `quint-llm-kit`, through `"autoUpdate": false` |
| Freeze baseline | [`update-freeze-baseline.json`](owner-cutover/update-freeze-baseline.json) |
| The switch plan | [`switch-plan-draft.json`](owner-cutover/switch-plan-draft.json), SHA-256 `8679483c4127f59d94e30d20934292c6142492b3cf93ca9593cbafacfe2f475e`, also at `~/.local/state/owner-switch/prepared/plan.json` |
| The switch program | `~/.local/state/owner-switch/prepared/owner_switch.py`, SHA-256 `76ed17260bb1e0305c3011ec2c6d5eab171b1454a498a2a5e6ceef9cba090f7d` |
| The comparison instrument | `~/.local/state/owner-switch/prepared/owner_switch_dry_run.py`, SHA-256 `324ccbd3dc6cdc9910e840fe8d28fe6a15ec25b79e70fd5e7c37ce56df4cd352` |
| The prepared instructions | `~/.local/state/owner-switch/prepared/INSTRUCTIONS.md`, SHA-256 `c7966156d4d7906b092ab689e59f7b404791e09d6eeb2219e7c5b56233d1f745` |
| The dry-run result before finish | [`dry-run-result.json`](owner-cutover/dry-run-result.json), SHA-256 `e278339106d23516f037ada76c6f45acfd0fb8bd4402849efcbd7e95be8476ba`, run `run-20261113T080616Z` through the stand-in prefix |
| Accepted sources | The ten `accepted.sources` of the plan. [Published source acceptance](owner-skill-delivery-evidence.md#the-ten-sources) lists each ID and `resourceDigest`. |
| The `grove-core` candidate | `2a24d5e61f29`. Its `plugins/grove` tree ID is `961fddd04bb7b271e9896d07dc0225348fa8c378`. |
| The owner's `config.rhai` | `~/.config/ravel/config.rhai`, SHA-256 `21ca2e515718ef638b6a5fecea4e4b099e93c7f4c4a50b6e4b7a0434a4336a6d` |
| Retained staging state | The staging HOME `~/.local/state/owner-staging/home` and the candidate keg `~/.local/state/owner-staging/candidate` |

[Owner switch plan redraft](owner-skill-delivery-evidence.md#owner-switch-plan-redraft)
holds the current plan and dry-run digests. The digests of
[Owner switch rehearsal](owner-skill-delivery-evidence.md#owner-switch-rehearsal)
are history.

## The sequence

Do the steps in this order. Each step names its pass condition. If a step
fails, stop and go to [Recovery](#recovery).

### Teardown and integration

1. **Preserve and commit.** Commit any remaining project changes through jj.
   Verify the work first. `CLAUDE.md` step 1 owns this step.
2. **Tear down.** From the session workspace, run
   `grove-llm finish-commit <finish-handle>`. The helper removes and commits
   `.grove/`. Never remove `.grove/` by hand. After this step, no leaf can
   carry a repair.
3. **Integrate.** Fetch current `main` and rebase the grove's
   changes onto it. Resolve each conflict. Run `bash scripts/check.sh` on
   the integrated result in both lanes of
   [The two lanes](release-readiness-record.md#the-two-lanes). The 1.1 lane
   uses the 1.1.0 keg. The 2.0 lane uses `/opt/homebrew/Cellar/ravel@2/2.0.0/bin/ravel`
   and its guard. Each lane must pass every principal section of the suite. Then move
   `main` to the finished change, and push it through jj.

### The Grove release

4. **Release.** From Grove's default colocated workspace
   `~/Development/AgenticOrchestration/grove`, run this command:

   ```sh
   HOMEBREW_NO_INSTALL_CLEANUP=1 task release:minor
   ```

   The variable keeps the Grove 22.8.0 keg of the running driver. Without
   it, the task's `brew upgrade grove` runs Homebrew's install cleanup for
   `grove`. Homebrew's `cleanup.rb` skips that cleanup and the periodic
   cleanup when the caller sets the variable. "Activation and running readers" of
   the coordinated release design requires this suppression.

   Keep `ravel@2` installed, so that the task's installed smoke runs its
   2.0 lane. Record the released version as V. The release is the next
   minor version after the version on integrated `main`. Do not assume
   22.9.0.
5. **Verify the installed Grove.** This step verifies the released Grove on
   both admitted Ravel majors, the 1.1.0 keg and the `ravel@2` 2.0.0 keg.
   The task's own last steps must pass:
   `grove --version` and `grove-llm --version` report V, `brew test grove`
   passes, and the GitHub release has three archives. The installed smoke
   must report each of these lines:
   - the 1.1 lane, direct and through a link, for the TypeScript set
   - the 2.0 lane, direct and through a link, for the Rhai set
   - "Ravel 2.0 resolved the six keg sources with no settings"

   This step happens while unversioned `ravel` is 1.1.0. If the task stops
   before this step completes, resume it as [Recovery](#recovery) states.

### The promotion

6. **Write the readiness receipt.** Write it only after step 5 passed. The
   receipt then names the released Grove and the routes that finish
   verified. Ravel's `release-promote.sh` compares the receipt only with the
   `version` line of the tap's `grove.rb`. A receipt that names an
   incompatible Grove therefore passes promotion. In Ravel's default
   workspace, run the command of "Consumer readiness receipt", with V in
   place of `22.9.0`:

   ```sh
   cd ~/Development/AgenticOrchestration/Ravel
   jq --arg version V --arg at "$(date -u +%Y-%m-%dT%H:%M:%SZ)" \
     '{schemaVersion: 1, kind: "ravel-promotion-readiness",
       ravel: {version: "2.0.0", tagCommit, assets},
       consumer: {name: "grove", version: $version, formula: "grove.rb", verifiedAt: $at,
         verifiedRoutes: ["native-1.1", "native-2.0-delivery-disabled", "directory-2.0"]}}' \
     target/release-evidence/v2.0.0.json >target/release-evidence/v2.0.0.readiness.json
   ```

   The three routes come from these runs. The 1.1 lane of step 5 covers
   `native-1.1`. The 2.0 lane and the keg-source line of step 5 cover
   `directory-2.0`. The 2.0 lane of step 3 covers
   `native-2.0-delivery-disabled` on the tagged source.
   [Limits](#limits) states what no installed test covers.
7. **Promote.** Make sure that `brew config` reports
   `HOMEBREW_NO_CLEANUP_FORMULAE: ravel`. Then run the promotion from
   Ravel's default colocated workspace:

   ```sh
   RAVEL_TAP_DIR=/Users/antony/Development/Delivery/homebrew-taps task release:promote VERSION=2.0.0
   ```

   If Ravel's evidence file is lost, add
   `EVIDENCE=/Users/antony/Development/AgenticOrchestration/grove/docs/ravel-v2.0.0-release-evidence.json`.
   The task must exit 0 and write
   `target/release-evidence/v2.0.0.promotion.json`. The tap must be a clean
   jj repository whose `main` agrees with `origin`.
8. **Run `native-off` immediately.** Run it right after step 7, before any
   other step and before any default launch. It needs no switch backup. The native
   settings section owns the rule.

   ```sh
   /opt/homebrew/opt/python@3.14/bin/python3.14 -I /Users/antony/.local/state/owner-switch/prepared/owner_switch.py native-off --home /Users/antony
   ```

   The command must exit 0. Then the promoted `ravel skills resolve --json`
   in the real HOME must report `disabled`. `owner_switch.py status` must
   report the health "native". `mac-check --only "Owner skill switch"` must
   report native delivery. After exit 3 or 4, do not launch a default
   session. Remove the cause and repeat the same command.
### Native delivery and the shared directory

9. **Verify the pair.** `ravel --version` on `PATH` reports `ravel 2.0.0`.
   `/opt/homebrew/Cellar/ravel/1.1.0/bin/ravel --version` still reports
   `ravel 1.1.0`. The promotion evidence lists the 1.1.0 keg and the
   `ravel@2` keg as retained and unchanged. `brew info ravel@2` reports the
   formula as deprecated.

10. **Verify Grove's revised formula and the ten default IDs.** Step 4
    installed Grove's revised formula. `brew info --json=v2 grove` must name
    V as the linked keg, and `/opt/homebrew/opt/grove` must reach that keg.
    Then resolve the shared default directory with the promoted Ravel, in a
    new empty HOME:

    ```sh
    H="$(mktemp -d)"
    env -i HOME="$H" PATH=/usr/bin:/bin /opt/homebrew/opt/ravel/bin/ravel skills resolve --json |
      jq -r '.status, (.sources[] | [.id, .resourceDigest, .originPath] | @tsv)'
    chmod -R u+w "$H"; rm -rf "$H"
    ```

    The status must be `resolved`, with exactly the ten accepted IDs. Each
    `resourceDigest` must equal its value in the plan's `accepted.sources`.
    Each `originPath` must canonicalise into `libexec/skill-sources/<id>`
    of its owning keg. Grove's six sources come from the keg of V.

    The real HOME's settings hold the off value now, so a resolution there
    reports `disabled`. The new HOME holds no settings, so Ravel reads its
    default directory. This session ran the same command with `ravel@2` and
    saw the four installed sources resolve.

    If an ID is missing, a link conflict can be the cause. A duplicate ID
    can leave a whole keg unlinked. Resolve the ownership conflict and link
    the intended keg, as "The keg layout" of the
    [delivery design](specs/codex-skill-provisioning.md#the-keg-layout)
    states. Never use `brew link --overwrite`. Then repeat the resolution.

    If a digest differs, stop in native delivery. That source needs its
    staging trials, a revised acceptance and another dry run before an
    attempt, as step 2 of the switch states. Inside finish, end without
    success, as the recovery row for a failed core trial states.
11. **Verify the owner's configuration.** `~/.config/ravel/config.rhai` must
    have the SHA-256 of [The inputs](#the-inputs). `policy.ts` stays,
    because finish's own driver launched through Ravel 1.1.0. If the digest
    differs, ask the owner to review the change before any default launch.

### The switch

Run the tasks of steps 13 and 14 from a Grove checkout at the integrated
`main`, such as the default workspace. Their paths are relative to that
checkout.

12. **Establish the quiet interval.** Ask the owner to do steps 1 to 12 of
    [the owner's checklist](owner-cutover/owner-checklist.md#steps-for-each-attempt).
    They end other sessions, stop the Codex daemons and pause the writers.
    They also pause Obsidian. The quiet interval lasts until the receipt or
    a restore ends. Finish is the operator. Run no search command from the
    operator session while a process inspection runs. Claude Code's
    embedded `ugrep` is a process that the inspection refuses.
13. **Verify the freeze.** Run these commands, as steps 13 and 14 of the
    checklist state:

    ```sh
    task owner:freeze:check BASELINE=docs/owner-cutover/update-freeze-baseline.json
    claude --version
    claude doctor
    ```

    The freeze task must report 0 breaks. It needs no minimum period after the
    baseline. `claude --version` must report `2.1.322 (Claude Code)`.
    `claude doctor` must print
    `Auto-updates: disabled (set by env: DISABLE_AUTOUPDATER)`. A break
    needs another dry run before the switch, and step 14 is that dry run. A
    changed Claude Code version also needs `task owner:quiet-interval`
    before step 14.
14. **Dry-run against the real prefix.** The dry run before finish used a
    stand-in prefix. This run uses the released Grove keg and the promoted
    Ravel, with no stand-in. First run
    `owner_switch.py processes --home "$HOME"`, and find "clear". Move the
    earlier `~/.local/state/owner-switch/dry-run` folder aside. Then run the
    command of [The command](owner-skill-delivery-evidence.md#the-command)
    without `--path` and with the real prefixes:

    ```sh
    NOSHIM=$(echo "$PATH" | tr ':' '\n' | grep -vx "$HOME/.local/share/mise/shims" | paste -sd: -)
    env PATH="$NOSHIM" task owner:switch:dry-run OUT="/private/tmp/owner-switch-dry-run/run-<UTC time>" ARGS="\
      --python /opt/homebrew/opt/python@3.14/bin/python3.14 \
      --prefix grove-keg=$(realpath /opt/homebrew/opt/grove) --prefix ravel=$(realpath /opt/homebrew/opt/ravel) \
      --sign-in --include .config/mise --include .homebrew/trust.json --include .claude/plugins \
      --include .codex/config.toml --include .codex/hooks.json --include .codex/plugins \
      --include .codex/AGENTS.md --include .codex/instructions.md --include .quint-llm-kit/repo \
      --include .local/bin/codebase-memory-mcp"
    ```

    The run must pass `native-off`, `attempt`, the switch and the literal
    restore. Each probe must exit 0, and each restore comparison item must
    be equal. Copy its result to `~/.local/state/owner-switch/dry-run`, as
    the redraft did.
15. **Compare the prepared inputs.** Run the comparison of the prepared
    instructions, and keep its output file:

    ```sh
    /opt/homebrew/opt/python@3.14/bin/python3.14 -I /Users/antony/.local/state/owner-switch/prepared/owner_switch_dry_run.py compare \
      --result /Users/antony/.local/state/owner-switch/dry-run/result.json \
      --prefix grove-keg=/opt/homebrew/opt/grove \
      --prefix ravel=/opt/homebrew/opt/ravel \
      --out /Users/antony/.local/state/owner-switch/comparisons/<UTC time>.json
    ```

    It must exit 0, with each row equal or substituted. The comparison has a
    row for each tool version, Codex included. If a row differs, make no
    attempt.
16. **Run the plan report.** This is step 15 of the owner's checklist. Run
    it before each attempt:

    ```sh
    /opt/homebrew/opt/python@3.14/bin/python3.14 -I /Users/antony/.local/state/owner-switch/prepared/owner_switch.py report --plan /Users/antony/.local/state/owner-switch/prepared/plan.json --home /Users/antony
    ```

    After `native-off`, each `file` and `entry` line must end with "now its
    original". If a line differs, the plan needs another draft and dry run,
    as [Edits after the plan draft](owner-cutover/owner-checklist.md#edits-after-the-plan-draft)
    states.
17. **Write the attempt note.** Run the `attempt` entry of the prepared
    instructions. Read the note
    `~/.local/state/owner-switch/attempts/<attempt>.json`. Its path and each
    field must name this attempt. The note records instructions, not
    progress.
18. **Switch.** Run the literal `switch` command of the note. Add
    `--guidance <path>` for each guidance file that finish read, by its
    absolute plugin-cache path. The finish skill and the spine are two such
    files. The program does each step of
    [The switch inside finish](specs/owner-skill-delivery.md#the-switch-inside-finish)
    in order:
    - it reviews the plan and runs the narrow process inspection
    - it verifies the Grove release, the promoted Ravel, the formulae and
      the ten digests
    - it records the operator's loaded cache paths, registrations and
      guidance reads, and compares each unverified built-in inventory row
      with the operator's skill list
    - it backs up the whole write set in
      `~/.local/state/owner-switch/backups/<attempt>`
    - it installs the hooks: it registers only `writing-hooks` from
      Grove's `opt` path, installs `technical-communication-hooks@writing-hooks`
      and writes its disabled setting
    - it deactivates the superseded entries, enables the two native
      capability plugins and writes the switched Ravel settings last
    - it runs the probes through the promoted Ravel and ends their services
    - it runs `mac-check` and the final comparison
    - it writes the receipt and appends its observation to the maintained
      manual

    The probes are `resolve-switched`, `claude-switched` and
    `codex-switched`. The native probe is `resolve-disabled`,
    `claude-native` and `codex-native`, which restore runs. Exit 0 means
    that the program wrote the receipt and that both records agree with the final
    layout.
19. **Reread the guidance.** Read the finish skill and the spine again by
    their recorded paths. Verify the switched layout, the guidance reads
    and the receipt against the owner design. A receipt alone is no proof
    of the current state.

### Records

<!-- tc: depart synonym-rotation because "correction" is the manual's own name for an appended entry that supersedes an observation, which differs from a repair of an installation -->

20. **Append the manual correction.** After the receipt, append one dated correction
    to the maintained manual `AI Dev Tooling`. Use the manual's own
    "Synchronise the record" workflow. The correction covers each statement
    that the switch made stale:
    - the rows "Claude-specific", "Codex-specific" and "Shared Agent Skills"
      of "System map"
    - the first four items of "Shared skills"
    - the sentence about `skills/` in "Configuration ownership"
    - steps 4 and 5 of "Bootstrap and recovery". Step 5 names Grove's Codex
      provisioning and `plugins/install.sh`, which the integrated `main` no
      longer holds.

    The workflow also covers the skill rows of the maintenance checker,
    which still classify native links.
    [The maintained manual](cross-repo-docs-reconciliation-record.md#the-maintained-manual)
    lists each statement. The manual is append-only. Never rewrite an
    earlier observation. Keep Obsidian paused through the append and its
    readback.
21. **Record the receipts for the paused workstreams.** Fill each row of
    [Promotion and switch receipts](paused-grove-handoffs.md#promotion-and-switch-receipts).
    The rows name these values:
    - the Grove release V
    - the promotion run, its tap commit and the version of `Formula/ravel.rb`
    - `ravel --version` on `PATH`, and the state of the 1.1.0 keg
    - the result of `native-off`
    - the switch result, its receipt path and its time
    - the remaining owner actions of [After finish](#after-finish)

    Commit the change to `main` through jj, and push it. The change touches
    no packaged file, so no release follows it. The paused workstreams
    resume only after finish ends.
22. **End the quiet interval.** Ask the owner to do the resume actions of
    the checklist. The owner opens Obsidian only after the appended entries
    agree with the result.

### Ending

23. **Record the teardown.** Return to the original session workspace. Run
    `grove-llm record-teardown`. The newly installed binary admits the
    session through the 22.8.0 launch directory, as
    [The launch-directory and admission contract](release-readiness-record.md#the-launch-directory-and-admission-contract)
    states.
24. **End.** Run the exact original ending command of finish's launch
    prompt as the last action. Do nothing after it.

## Recovery

[Recovery boundaries](specs/coordinated-release.md#recovery-boundaries)
owns each row. This table names the row for each failure in finish.
`.grove/` no longer exists after step 2, so no leaf can carry a repair. A
failure after step 2 that the rows cannot complete ends finish with the
force-stop command of its launch prompt. Never signal success then.

| Failure | Row and action |
| --- | --- |
| Step 3 fails | Repair on the session's line and repeat the suite. No cut exists. |
| Step 4 stops after the cut | "Grove release partial". Resume the unfinished step of [the release procedure](RELEASING.md). Never rerun `task release:minor`, because a rerun bumps the version again. Unversioned Ravel stays 1.1.0. |
| Step 5 fails on the formula's test | Repair the test in a same-version tap change, then repeat `brew test grove` and the installed smoke, as section 4 of [the release procedure](RELEASING.md#4-check-the-installation) states. Write no receipt before step 5 passes. |
| Step 7 stops | "Promotion partial". Run `task release:promote VERSION=2.0.0` again. It keeps published formulae and pushes a local promotion commit. Cut neither Ravel nor Grove again. |
| Step 8 fails, or the switch stops before its backup | "Promotion complete, switch has no complete backup". Run `native-off` before any default launch or release task. Verify disabled directory delivery and intact native discovery. Repair preparation while native delivery stays active. |
| The switch fails or stops after its backup | "Promotion complete, owner switch failed or was interrupted after backup". Keep the backup, the attempt note and the restore entry. A caught failure restores by itself. After a crash, re-establish the quiet interval and run the note's `restore` command. Verify native delivery before a fresh attempt. |
| The probes passed before the signal | "Promotion complete, switch probes passed before finish's signal". Do step 19, then complete steps 20 to 24. |
| A core trial or a probe of a Grove kind skill fails on the released Grove | "Promotion complete, a core trial or a probe of a Grove kind skill failed on the released Grove". Establish native delivery first: `native-off` without a backup, or the attempt's restore after mutation. Then end finish without success. The owner repairs `main` and publishes a Grove patch through `task release:patch`. Reaccept changed sources, repeat the affected staging trials and dry-run the revised inputs before a fresh switch. Cut and promote no Ravel again. |

The rows from "No cut" to "Installation/live-book failure" cover Ravel's
cut, which is complete. They do not apply in finish. Ravel's cut never
repeats. Its recovery chapter and the release-channels specification own the
promotion's boundaries. A partial Grove release
resumes at its unfinished step and never bumps the version again. The
original Ravel 1.1.0 keg and the ending command stay through the final
signal, whatever row applies.

## After finish

The owner is the operator after finish ends.
[Resuming the switch](specs/owner-skill-delivery.md#resuming-the-switch)
owns the owner's operation. The prepared instructions give each command.

- The attempt note names the backup and the literal `restore --backup`
  command. Choose the note of the interrupted attempt, never the latest
  backup by inference.
- After an unsuccessful finish, one native Claude Code session can be the
  operator. It records its own ancestors, cache paths and built-in listing.
  The owner can also supply equivalent reader evidence from a terminal.
- A later restore first compares every destination with the receipt's
  post-switch identity. Any drift refuses before the first write. The owner
  then prepares a revised native-return plan. Restore performs no merge and
  has no `--force` bypass.
- The maintained manual is append-only. Each later native return appends a
  new observation.
- Release the Claude Code hold after a successful receipt, as
  [Release the holds](owner-cutover/owner-checklist.md#release-the-holds)
  states. Keep the three marketplace `autoUpdate` values until purge. The
  release edits `~/.claude/settings.json`, a file of the switch set. A
  later whole-file restore then refuses through the drift rule.
- The receipt lists the staging HOME and the candidate keg as removable
  after the receipt. It lists the backup, the frozen cache versions, the
  registrations and the old binaries with their purge conditions.
- Run the purge report before purge:

  ```sh
  /opt/homebrew/opt/python@3.14/bin/python3.14 -I /Users/antony/.local/state/owner-switch/prepared/owner_switch.py purge-report --plan /Users/antony/.local/state/owner-switch/prepared/plan.json --home /Users/antony
  ```

  [Purge](specs/owner-skill-delivery.md#purge) owns the conditions. Purge
  is outside this grove's success condition.
- After the reap of the last driver on Ravel 1.1.0, remove
  `~/.config/ravel/policy.ts`. Remove the 1.1.0 keg with
  `brew cleanup ravel`, outside the finish session of its last driver. The
  owner's manual states when the cleanup protection for `ravel` may go. The
  Grove 22.8.0 keg that step 4 kept can go at the same time.
- The paused workstreams resume after finish ends. Each reads its text in
  [the paused-grove handoffs](paused-grove-handoffs.md) and the receipts of
  step 21.

## Limits

- No installed test runs Grove's native 2.0 route with delivery disabled.
  The 2.0 lane of step 3 covers it on the tagged source. Step 8 covers the
  disabled resolution with the promoted Ravel, and restore's native probe
  covers the harness reads.
- No test ran the next release's `record-teardown` under a live 22.8.0
  driver. A source comparison and the working binaries' tests show the
  contract.
- Homebrew's real conflict resolution of the source aliases is not
  measured before finish. Step 10 measures it after promotion.
- The dry run before finish used a stand-in prefix. Step 14 replaces it
  with the real prefix.
- TestAnyware's live VM tests did not run before finish. The switch's
  probes run its command reference and doctor. `TestAnyware.app` must run
  for VM commands, as [TestAnyware after a login](owner-cutover/owner-checklist.md#testanyware-after-a-login)
  states.

## Verification of this document

finish-handoff-k324 extracted every distinct word of this document and of
the prepared instructions on 2026-11-13. It classified each word that can
name an inverse, a merge, a pending record or a resumed run. No instruction
of either document names one of these four. Each occurrence belongs to one of
these classes:

- the resumption of a partial Grove release at its unfinished step, which
  `CLAUDE.md` and the release procedure require
- a repeat of the promotion task, which Ravel's promotion boundary requires
- the paused workstreams, which resume after finish ends
- the resume actions of the owner's checklist, which reopen paused writers
- the title of the section "Resuming the switch", which states that there is
  no automatic resumption
- a negation, such as "Restore performs no merge"
- the retry of a restore through its recovery note, in the prepared
  instructions, which is another operation with one backup
- the refusal of a new attempt while an earlier attempt is unresolved, in the
  prepared instructions
- this section

Do not replace these classes with a count of occurrences. Each edit of this
section changes such a count.

Each relative link and anchor of this document resolved. A copy with one
broken anchor and one broken file name failed the same instrument.
