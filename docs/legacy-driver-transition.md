# Preserve the original driver during the Ravel release

The extraction grove is running under installed Grove **22.1.0** and its
original `harness-dispatch` supervision contract. k13 upgrades public Grove
while this driver stays alive. Preserve the complete original keg at its
canonical path and retain bare ending/admission commands through final finish's
last signal. Ravel is an independent process context and ending command.

This procedure applies to this extraction checkpoint. The older
[signal-contract cutover](RELEASING.md#signal-contract-cutover) describes a
different launch contract; do not substitute `grove-llm complete` here.

## Recorded installation

The original keg is `/opt/homebrew/Cellar/grove/22.1.0`. Its public fronts
resolve from `/opt/homebrew/bin`. The inherited PATH already contains
`/Users/antony/.local/bin` ahead of `/opt/homebrew/bin`; both compatibility
entries are currently absent. No stable-PATH link was created by k50.

| Original relative path | SHA-256 |
|---|---|
| bin/grove | `5a4a00736ede4343fded982563d7d41c744672b0c20d162d2818aa902bc13b57` |
| bin/grove-llm | `7139dece1bfd4c480e85faa1d1963bab970a91e8725178e14a1e8bf4c4524a43` |
| bin/harness-dispatch | `a4746baa2f644da805e40efa482a6bb8311c08c59002a85fdd774a021ea58a45` |
| libexec/harness-dispatch/harness-dispatch-policy | `a1303798a0159505340d0ed5ccd57e92917935f2222b54b07cab728276c7dbb8` |

[legacy-driver-inventory.json](legacy-driver-inventory.json) records **every
entry**, including directory/file modes, symlink targets and file digests.
It includes the fronts, compiled worker, SDK, policy/example/Grove assets,
runtime notices, license, receipt, README and SBOM. The instrument inventories
the entire actual keg; the required-member list is only its admission minimum.
It refuses external symlinks and unsupported entry types rather than producing
an incomplete recovery copy. The recovery copy retains links and file metadata.

The new Ravel front resolves separately to
`/opt/homebrew/Cellar/ravel/0.1.0/bin/ravel`; no equal-version or shared-Cellar
relationship is assumed.

## Before k13's release

Run in the original session workspace, on its inherited PATH. Do not alter the
driver's environment, launch directory, session epoch or exit channel. All test
helpers scrub `GROVE_*`, `HARNESS_DISPATCH_*` and `RAVEL_*` before subprocess
probes. Preparation invokes only the three original `--version` commands.

```sh
task legacy:controls
task legacy:rehearsal KEG=/opt/homebrew/Cellar/grove/22.1.0 \
  RAVEL=/opt/homebrew/bin/ravel OUT=target/k13/legacy-rehearsal-before
task legacy:prepare KEG=/opt/homebrew/Cellar/grove/22.1.0 \
  STABLE_BIN=/Users/antony/.local/bin OUT=.jj/legacy-driver-22.1.0
```

Outputs must be absent. Keep the preparation directory and complete backup
through the driver ending; this workspace must not be deleted. Compare the
fresh inventory with the recorded original before applying:

```sh
python3 - <<'PY'
import json
from pathlib import Path
recorded = json.loads(Path('docs/legacy-driver-inventory.json').read_text())
prepared = json.loads(Path('.jj/legacy-driver-22.1.0/manifest.json').read_text())
assert prepared['keg'] == recorded['keg']
assert prepared['inventory'] == recorded['inventory'], 'original installation changed'
PY
```

The prepared `release-env.sh` exports both cleanup protections and absolute
original executable paths. Source it in the shell running each release or
recovery step, including final finish; a child's export cannot change the
already-running driver's environment. Homebrew documents
[`HOMEBREW_NO_INSTALL_CLEANUP` and `HOMEBREW_NO_CLEANUP_FORMULAE`](https://docs.brew.sh/Manpage).
The first disables automatic install/upgrade/reinstall cleanup; the second
excludes Grove from cleanup/autoremove. Existing formula exclusions are retained.
Do not explicitly uninstall the original version while the driver is live.

```sh
legacy_plan=/Users/antony/Development/grove.extract-harness-dispatch-to-top-level/.jj/legacy-driver-22.1.0
source "$legacy_plan/release-env.sh"
task legacy:apply PLAN="$legacy_plan"
task legacy:verify PLAN="$legacy_plan"
```

Application creates two absolute symlinks, `harness-dispatch` and `grove-llm`,
in the existing stable directory, targeting the original keg's fronts. It
refuses any existing entry, including a dangling or matching symlink. It checks
both destinations before creating either, uses non-overwriting symlink creation,
and rolls its own links back if final verification fails. Its `links.json`
application receipt records each created symlink's device/inode and nanosecond
change/modification times, so a replacement to the same target is refused too.
It also refuses PATH
shadowing, changed keg/backup inventory or missing cleanup protection. Run it
once; verify an already-applied plan instead of repeating apply. Keep the plan
and links under one operator's ownership; do not mutate them concurrently.

The bare old helper remains deliberately pinned for subsequent sessions'
admission/teardown contract. New Grove checks use explicit paths into the newly
installed keg. `GROVE_ORIGINAL_LLM` and `GROVE_ORIGINAL_DISPATCH` expose the
canonical original paths in the release shell; the running driver already holds
its original canonical launch path. Never repoint that path at Ravel or retain
only a copied front.

## Apply the authorized release, then verify

Move to the default colocated workspace for k13's source-only integration and
`task release:minor`, following [RELEASING.md](RELEASING.md). Keep the same sourced
environment. Record the absolute preservation plan in k13's terminal handoff
and the root brief for final finish. Resume a partial release at its unfinished
step; do not repeat a version cut. Re-source protections in every new shell.

After installation, return to this session workspace and run:

```sh
source "$legacy_plan/release-env.sh"
task legacy:verify PLAN="$legacy_plan"
task legacy:rehearsal KEG=/opt/homebrew/Cellar/grove/22.1.0 \
  RAVEL=/opt/homebrew/bin/ravel OUT=target/k13/legacy-rehearsal-after
command -v harness-dispatch grove-llm
"$GROVE_ORIGINAL_DISPATCH" --version
"$GROVE_ORIGINAL_LLM" --version
```

The rehearsal's actual original bytes live in a scratch complete keg. It unlinks
the scratch public fronts, adds a two-front new-version fixture, then launches
the next fake session through the retained old canonical path. Its bare exit
writes only the temporary legacy channel. The old SQLite store exports an
execution-confirmed `exit_signal` observation; both fake process groups and
descendants are gone. A nested independently installed Ravel returns its own
opaque result; its process context carries no legacy authority. Calling legacy
exit there is a no-op, while Ravel end-session outside Ravel refuses. Known-wrong
old/new/Grove channel baits stay unchanged.

The old helper's version and no-loop teardown no-op pass. In a scratch jj
workspace a stale launch refuses teardown without a marker. This rehearsal
creates **no second Grove driver** and does not claim to perform an admitted
finish/teardown. The original helper's entire bytes remain pinned; the real
driver supplies admission when final finish arrives. Missing actual worker or
keg refuses. Post-reap cleanup removes only matching scratch links. Missing
inputs fail; nothing is skipped and no model harness is launched.

Neither k13 nor final finish removes the real compatibility links, plan or
original keg before its last signal. This session's ending remains bare
`harness-dispatch exit`; final finish first invokes original
`grove-llm record-teardown`, then its original ending command. Invoke executables
directly, never through Cargo's authority-scrubbing environment.

## Recovery and post-driver cleanup

If verification refuses, stop before any further upgrade/publication work.
Do not overwrite PATH collisions or replace unexpected keg bytes. Inspect the
named mismatch and recorded inventory. If application itself fails, its links
are rolled back; any concurrently replaced owner entry is retained. If a killed
application left a partial matching link, inspect it against `manifest.json`,
remove only that owned link, then reapply before release. Do not use cleanup's
driver-ended assertion while the driver is live.

If the original canonical keg has vanished, links into it cannot keep future
launches working. The complete backup is `<plan>/keg`. Restore it **to the exact
recorded canonical path**, only after confirming that destination is absent and
cleanup is suppressed; do not overwrite an unexpected directory. Copy the
whole tree with metadata/symlinks intact, then run `task legacy:verify` before
resuming. An existing altered keg requires operator reconciliation, not an
automatic overwrite. The tool never invokes Homebrew or restores/removes a keg.

Only from a separate post-driver shell, after observing that the original Grove
driver and its supervised process groups have exited/reaped, remove the owned
compatibility links:

```sh
task legacy:cleanup PLAN="$legacy_plan" DRIVER_ENDED=yes
```

The tool's cleanup verb checks both links before removing either and refuses a replacement or
missing link. The assertion is the operator's observation, not a process detector.
If another operator already removed one, reconcile manually without deleting a
replacement. Keep the backup until the new installation works. Then remove
Grove from the shell's cleanup exclusions (retaining other exclusions), restore
the prior automatic-cleanup setting. Any subsequent Homebrew cleanup is a
separate operator action: after the original driver has ended and the new
installation works, it may remove the original keg. The preservation tool
never removes a keg. Remove the recovery directory only
when it is no longer needed. No original history import or catch-up work occurs.

## k50 evidence

The preparation-only run saved `target/k50/original-plan` without applying it.
`target/k50/controls.log` holds the refusal/ownership controls;
`target/k50/rehearsal-final` holds the actual-original upgrade/unlink logs, temporary
store/export, ending, fake process/context records and identical per-entry
original/Ravel before/after inventories. Earlier rehearsal attempts retain their
failures: an inherited release variable assertion, then an incorrect observation
schema assertion; the corrected rehearsal checks the actual old schema. No live
Homebrew, PATH, config, history or session authority was modified.

Final `task legacy:controls` passes all 31 subprocess cases, including a
same-target owner replacement and sourceable cleanup protections retaining
other formula exclusions. Ruff formatting/lint and strict Pyright pass.
`target/k50/review.md` records the single fresh review and both classifications;
`replacement-red.log` proves the ownership control failed before its correction.
`cleanup-task-refusal.log` records the Taskfile's refusal without driver-ended
acknowledgement. `linkage.log` shows the actual original fronts/worker link only
system libraries.

`target/k50/check-final.log` records all 15 principal checks passing: 1,378
workspace nextest cases, nine doctests, explicit installed-Ravel assets and
provenance, and six books validated final/all. The original `check.log` retains
the missing release-name classification and host Bun 1.4.3 refusal. The final
run uses preserved pinned Bun 1.4.2 through `target/k50/tools/bun` on a child-only
PATH; neither the host runtime nor the version contract changed.
The initial narrow Rust invocation lacked required `GROVE_TEST_RAVEL`; the
normal full check supplies the installed dependency and all twelve removed-
surface cases pass there.

`final-frozen-before.json`, `final-frozen-after.json` and
`final-digest-comparison.json` match every one of 15,788 named input subjects,
including project/adjacent-checkout files, tools/dependency sources, archive
inputs, complete original/Ravel kegs, owner configuration and legacy history.
The per-entry rehearsal inventories independently match. The measurement
instrument is `target/k50/measure.py`; generated logs/temporary test trees are
excluded. This evidence paragraph and leaf/parent completion notes are written
after measurement, and do not form part of its frozen inputs. No live
application, publication, installation, driver replacement or teardown ran.
