# Grove dual-Ravel stage record

This record closes Grove's dual-Ravel stage, `grove-dual-ravel-k27`. It names
the Grove commits of the stage and the two Ravel builds that the stage
measured. It gives recipes that reproduce each build, HOME, configuration and
source. It states the final results and the order of publication that
acceptance and rollout follow. This record reports no release.
[Default-directory runs](#default-directory-runs) names the later build that
the 2.0 lane now pins, and its results.

Two later stages consume this record. `common-loading-acceptance-k29` measures
actual Claude Code and Codex sessions against the unpublished 2.0 build.
`discovery-rollout-k6` publishes Ravel 2.0.0 and prepares Grove's release.
The [delivery specification](specs/codex-skill-provisioning.md#supported-routes)
owns the three routes. The [coordinated release](specs/coordinated-release.md)
owns the order of publication between the products.

On 2026-11-03 the human withdrew operating-system confinement from Ravel and
from Grove. The confined results of this record are therefore historical. They
include the commit of `grove-directory-confinement-k99`, the confined directory
case of the release smoke and the outcome row on confined native access. No
build after the removal can repeat them. The
[stage evidence reconciliation](common-loading-acceptance-record.md#stage-evidence-reconciliation)
states the backend of each result and the checks that run again on the builds
after the removal.

## Grove commits

Each commit is on the line of this workstream, above Grove `main` at
`238c3a839ab4`. Nobody pushed the line. Each leaf left `bash scripts/check.sh`
green with the 1.1 build.

| Commit | Leaf | Delivered |
| --- | --- | --- |
| `09032219baa3` | `grove-dual-ravel-k28` | The plan of the stage |
| `45f37cb199a0` | `grove-ravel-runtime-fixtures-k94` | `GROVE_TEST_RAVEL_RUNTIME` and the native route on 2.0 for both callers |
| `513cb46943dc` | `grove-rhai-static-assets-k95` | The shared case table, the Rhai sample and the Rhai static example |
| `76303df88ba3` | `grove-rhai-review-assets-k96` | The Rhai review adapter, review examples and provider-evidence wrapper |
| `10c67dc5f2fa` | `grove-ravel-two-admission-k97` | Admission of 2.x from 2.0.0, its remedy and the native-route documents |
| `cf9a8ec7b418` | `grove-directory-lifecycle-k98` | The directory route through the lifecycle caller |
| `b9296447aef3` | `grove-directory-confinement-k99` | The confined directory route and the unreadable native case |
| `5b79f8ab65d4` | `grove-dual-archive-k100` | Both asset sets in the archives, the formula and the installed smoke |
| `3a909fd828c4` | `grove-dual-target-inputs-k101` | The 2.0 lane of the target inputs, target smoke and release smoke |
| `852b7ec1a058` | `grove-linux-two-lane-k103` | The 2.0 lane on the Linux floors, with the glibc 2.17 build |

The commit that retires `grove-dual-ravel-handoff-k102` adds this record. It
also makes `scripts/release-prepare.test.sh` select the runtime, as
[Final runs](#final-runs) states.

## Ravel builds

Before Ravel's cut, a build of the feature line reports `ravel 1.1.0`. This
record therefore identifies each build by its commit, its build identity and
the SHA-256 of its `bin/ravel`. It never uses the version output.

| Subject | The 1.1 build | The 2.0 build |
| --- | --- | --- |
| Source | Ravel tag `v1.1.0`, peeled `2095cbd879c9` | Build commit `23e50ce90037` |
| Line | Released | Bookmark `skill-directory-delivery` at output commit `1ca4aecbeae1`, unpushed, above Ravel `main` `24a545236185` |
| Build identity | `fa130cc262cfb22541798debe937ecd9e81488ebef494d873d377978808c9d9c` (worker) | `ef5857e989e6d99da51631ae0295685c8961c1b01bac58d865e6fb789ae71deb` |
| macOS prefix | The Homebrew keg `/opt/homebrew/Cellar/ravel/1.1.0` | A scratch prefix from `task install` |
| macOS `bin/ravel` SHA-256 | `c52f990b2059c8bbe8efba10a741d3a0311000ac1712d71e882638f54c3ac1ed` | `8e10e311c50c81ca60ac4f393d17408982d0aafcfe46cccfbc46130465290ae2` |
| Layout | `bin/ravel` and the worker `libexec/ravel/ravel-policy` | `bin/ravel` only. The front is also the Rhai worker |
| Configuration | `policy.ts`, request and policy schema 2 | `config.rhai`, `SCHEMA_VERSION` 4 |
| Archives | The 1.1 lane of `scripts/ravel_target_inputs.py` | The 2.0 lane of `scripts/ravel_target_inputs.py` |

The output commit adds only Ravel's "Linux floor repair record" above the
build commit, so a build at either commit has the same identity. The 2.0
archives hold these executables:

| Target | Archive SHA-256 | `bin/ravel` SHA-256 |
| --- | --- | --- |
| aarch64-apple-darwin | `6fd7e44ac0a1d345456faef2aeeede2bb695944098dc5f42b7ade52944b0d928` | `8e10e311c50c81ca60ac4f393d17408982d0aafcfe46cccfbc46130465290ae2` |
| aarch64-unknown-linux-gnu | `331008d4c2aa005061b6e61d505bac131d4f4473cc9520eeba765e089b1cb32d` | `5bf3ed5bf6892f37cc39d7f2641558c8b35ac3091d101f4ebf7dae6ce7785411` |
| x86_64-unknown-linux-gnu | `2a7e62f78a5a6600e6d4d2186bc2cbf11db7edae57ef4176f06623ceddad3b04` | `6c1ef29cbe1e6a664e0a35bcde7fe258aa92c02e2f819a9d483ce873ae7b76fa` |

Each of the three executables holds the identity `ef5857e9…` twice, and the
identity `9feb86b7…` of the earlier build nowhere. That earlier build, of
`ravel-local-sources-k90`, did not link at glibc 2.17. Every 2.0 result of
k94 to k101 on macOS used it. The final runs below repeat each of those
results with the current build.

The 1.1 keg is also the ending executable of the live driver. Read it, and
copy it into fixture prefixes. Never upgrade, unlink or change it.

## Recipes

### The 2.0 prefix

Install the 2.0 build from its jj workspace, with an empty working copy above
the bookmark:

```sh
cd ~/Development/AgenticOrchestration/Ravel.skill-directory-delivery
jj new skill-directory-delivery
mise exec -- task install PREFIX="$SCRATCH/ravel-2.0"
shasum -a 256 "$SCRATCH/ravel-2.0/bin/ravel"
```

The digest must be `3a9a9129…`, the build of
[the default-directory runs](#default-directory-runs), and
`$SCRATCH/ravel-2.0/libexec` must not exist. `task install` writes no
installation record. If the bookmark moves, the new commits and identity
replace this record's values before any result is credited.

`task release:archives` at the build commit makes the three archives. A
rebuild gives other archive digests, because the tar member times differ, but
every file is byte-identical. Grove's ignored `target/ravel-archives-2.0/`
holds the copies of the build that the 2.0 lane pins. Admission accepts only
those pins.

### The HOME of each run

Each run of the gate uses its own scratch HOME. The toolchain stays at its
real location, because it is not a subject of the run:

```sh
export HOME="$SCRATCH/run-$LANE/home"
export CARGO_HOME=/Users/antony/.cargo RUSTUP_HOME=/Users/antony/.rustup
export MISE_DATA_DIR=/Users/antony/.local/share/mise
export MISE_CONFIG_DIR=/Users/antony/.config/mise
export MISE_CACHE_DIR=/Users/antony/Library/Caches/mise
export MISE_STATE_DIR=/Users/antony/.local/state/mise
```

The 1.1 run sets `GROVE_TEST_RAVEL=/opt/homebrew/Cellar/ravel/1.1.0/bin/ravel`
and `GROVE_TEST_RAVEL_RUNTIME=1.1`. The 2.0 run sets `GROVE_TEST_RAVEL` to the
front of the 2.0 prefix and `GROVE_TEST_RAVEL_RUNTIME=2.0`. It also puts a
guard `ravel` first on PATH. The guard prints `host-ravel-reached` and exits
86, so a check that reaches the host's Ravel fails.

Each run then calls
`mise exec bun@1.4.3 -- bash scripts/check.sh`, and each task with
`RAVEL=<front> RAVEL_RUNTIME=<1.1|2.0>`. Bun 1.4.3 is the pin of
`scripts/ravel-policy-typecheck.sh`. Run the two builds one after the other,
because they share the Cargo target directory.

From Grove `a81b1acc2d1d`, the gate forwards `GROVE_TEST_RAVEL_RUNTIME` as
`RAVEL_RUNTIME` to its caller-asset check. Before that commit, a 2.0 gate
also needed an exported `RAVEL_RUNTIME=2.0`.
[The gate repair](common-loading-acceptance-record.md#the-gate-repair)
gives the cause.

The installed checks run in a third scratch HOME with the same guard. Two
more settings apply there. `DOCKER_CONFIG=/Users/antony/.docker` keeps the
Docker context of the floors. The formula test needs the owner's Homebrew trust
store, which Homebrew reads from `$HOME/.homebrew/trust.json`. The `brew`
wrapper resets `HOMEBREW_USER_CONFIG_HOME` to that path. Copy the owner's
`trust.json` into the scratch HOME with mode 600, and do not change the
original.

### Configuration and settings

The fixtures write each file into a temporary HOME of their own. The
[usage guide](USAGE.md#launch-policy) owns the route table and the examples.

| Route | `~/.config/ravel/` holds |
| --- | --- |
| Native on 1.1 | `policy.ts` with schema 2. Settings, if any, hold only `timeoutMs`, `contextBytes` and `policyEnv` |
| Native on 2.0 | `config.rhai` with `SCHEMA_VERSION` 4. Settings hold no `skills`, and the prefix holds no installation record. Or the settings hold `"sources": []` |
| Directory on 2.0 | As native on 2.0, with a `skills` object of local sources in `settings.json` |
| Default directory on 2.0 | `config.rhai`, and settings without `skills`. The copied prefix holds an installation record that names a fixture directory of links |

### Local sources

`Sources::plant` in `crates/grove/tests/directory_route.rs` makes the two
sources of the directory cases, outside the working tree:

- `core`: a plain directory with `skills/grove/SKILL.md` and its resource
  `skills/grove/references/execute.md`.
- `engineering`: a plain Git checkout at a commit, with
  `skills/grove-impl/SKILL.md`. The fixture runs `git init`, `git add --all`
  and `git commit` with `GIT_CONFIG_GLOBAL=/dev/null`, and it requires a clean
  status.

The settings name both sources with `"type": "local"` and `"roots":
["skills"]`, and the spine as the one mandatory file. The Linux floor case
uses two plain directories, because the floor userland has no Git.

The skills still live in this checkout. A source of a real harness trial
therefore names a plugin directory of a dedicated Grove checkout, such as
`plugins/grove` with the root `skills`. That checkout is at a recorded commit. Ravel captures uncommitted
changes too, so do not name a development working copy.

## Final runs

All runs below are from 2026-11-03, on macOS arm64. Their subject was Grove
`852b7ec1a058` with the working copy of k102, before its results were written.
A digest of every tracked file outside `.grove/` and of both `bin/ravel` files
was the same before and after each run.

| Check | The 1.1 build | The 2.0 build |
| --- | --- | --- |
| `bash scripts/check.sh` | 20 of 20 passed | 20 of 20 passed |
| `task ravel:resolution` | Passed, 10 tests | Passed, 10 tests |
| `task ravel:commands` | Passed, 4 tests | Passed, 4 tests |
| `task ravel:policy` | Passed: typecheck, compiler test and the case table | Passed: the case table. The TypeScript steps do not apply |
| `task ravel:activation` | Passed, 796 tests | Passed, 796 tests |
| `task ravel:fixtures` | Passed, 18 tests | Passed, 18 tests |
| Core conformance | Passed | Passed |
| `task ravel:directory` | Not applicable | Passed: seven directory cases and the native case |

For each log, the sets of passed test names are identical between the two
builds. The workspace suite of the gate ran 1,333 tests on each. No log
reports a case as not run. A green total alone would not show this.

The first 2.0 run of the gate failed release preparation. Its real
`grove run` case always wrote a 1.1 `policy.ts`, and the 2.0 build refused it
as `config_format_retired`. The case now selects `config.rhai` or `policy.ts`
from `GROVE_TEST_RAVEL_RUNTIME`, as `testing/ravel.rs` does. It refuses a build
whose layout does not match the runtime.

The installed checks used the 2.0 prefix and the 1.1 keg explicitly:

| Check | Result |
| --- | --- |
| `task ravel:targets:controls` | Passed. Each lane refused a changed byte, the other lane's archive and an incomplete pair |
| `task ravel:targets:smoke` | Passed both lanes on all three targets, natively and under QEMU |
| `task release:smoke` | Passed all three targets with both lanes. On Linux it ran the native floor environments, both QEMU floors and the confined directory case |
| `task release:formula:test` | Passed with both builds. The Grove archive had SHA-256 `35da914e…` |
| `task release:installed` | Passed both lanes, direct and through a symlink |
| `task release:upgrade` | Passed from the Grove 22.8.0 keg. 286 entries stayed the same, and the active 22.8 reader completed |
| `task legacy:rehearsal` | Passed with the published Grove 22.1.0 archive as the keg and the 2.0 prefix as `ALSO` |

k103 ran the Linux registration test and the native and QEMU floor tasks on
the same archives. The release smoke above repeats their caller cases.

## Default-directory runs

`grove-default-directory-checks-k228` moved the 2.0 lane to a build that
reads the default skill directory. All runs below are from 2026-11-06, on
macOS arm64. A digest of every tracked file outside `.grove/`, of both
`bin/ravel` files and of the three 2.0 archives was the same before and after
the runs.

| Subject | The default-directory build |
| --- | --- |
| Source | Ravel `ff14cba826b0`, the output commit of `ravel-installation-record-k226`, at the tip of the bookmark `skill-directory-delivery`. Its parent `da736c1fc255` is the code of `ravel-default-directory-k225` |
| Build identity | `58d479ed058f1094b8754c72eecf9f0cec5a60b3d22b76baecdb827364abd217`, from the output of Ravel's build script for each of the three targets |
| macOS prefix | A scratch prefix from `task install`, with no `libexec` and no installation record |
| Archives | `task release:archives` at the source commit, from an empty working copy above it |

| Target | Archive SHA-256 | `bin/ravel` SHA-256 |
| --- | --- | --- |
| aarch64-apple-darwin | `289b4ac57ffd148255960507f02809034e8fb35fe97132813230116add804921` | `3a9a9129573e4d827a317f6b793ba990044080a31a7b667b64cc153a2f974a0b` |
| aarch64-unknown-linux-gnu | `2160a92e280f2d6881fd4d72c8196a787abc1b8e0fe20607e5028fa6041e75f3` | `9b0086b6447834257f4f33d00cd510441043cda4af7d83d709714bb2271edf7f` |
| x86_64-unknown-linux-gnu | `ab49d80db83bf4815342d562d8c265ac21bd97d13f4b9df0a7e18afc79dbdb6c` | `27d5df8c5060fc1c5119ec26e4d7c2f8855d0ae30f9999c70724819f86a2a0a9` |

The lane names one identity for every target. So the Linux floor targets
took the new build too, and the general floor route ran on its archives.
Grove's ignored `target/ravel-archives-2.0-23e50ce90037/` keeps the archives
of the earlier build.

| Check | Result |
| --- | --- |
| `task ravel:directory` | Passed, 11 tests: the seven directory cases, the native pair case and the default-directory cases below |
| `bash scripts/check.sh` with the 1.1 keg | 19 of 19 passed. An earlier run of the same files failed one test, `a_timed_out_selection_launches_nothing_and_publishes_nothing`. That test gives the Bun worker one second, and it then passed 20 runs of 20 alone |
| `bash scripts/check.sh` with the 2.0 build | 19 of 19 passed, with the guard `ravel` first on PATH and not reached |
| `task ravel:targets:controls` | Passed. The 2.0 lane refused a changed byte, the other lane's archive and an unrecorded target |
| `task ravel:targets:smoke` | Passed both lanes on all three targets, natively and under QEMU |
| `task release:smoke` | Passed all three targets with both lanes. On each Linux target, the floor container and QEMU passed the directory case on the 2.0 lane |

The two gates passed the same set of 1,381 test names. A copy of one set with
one name removed made the comparison fail.

`crates/grove/tests/directory_route.rs` gained the default-directory cases of
the [test seams](specs/codex-skill-provisioning.md#test-seams). Each copies the
2.0 build into the keg `Cellar/ravel@2/2.0.0` of a private prefix and writes
its installation record. The record names the prefix's `share/ravel/skills`.
That directory holds relative links into the kegs of two formulae. The
resolution runs in a HOME without `skills` settings.

| Case | Row | Result |
| --- | --- | --- |
| `a_launch_without_skills_settings_receives_each_source_of_the_default_directory` | Directory bootstrap | The launch received the three sources in ID order, each at the keg that its link reaches. The preamble named `skills` and `craft` where each exists |
| `empty_sources_beside_the_default_directory_deliver_nothing` | Directory bootstrap, Native route on 2.0 | The agent received the mandate unchanged, `request.skills` was `()` and no descriptor reached it |
| `a_copied_build_without_a_record_delivers_nothing` | Native route on 2.0 | The same, beside a populated fixture directory |
| `a_shared_home_without_skills_delivers_the_default_sources_on_2_0` | Settings isolation | A native launch on 1.1, a 2.0 launch with the default sources and a second native launch on 1.1 shared one HOME |

The leaf saw two controls fail. Against the earlier build `23e50ce90037`,
three of the four cases failed. The record-less case passed there too, as it
should. A mutation that gave the record-less case the copy with a record
failed it at its delivery check.

## Node outcomes

Each outcome of `grove-dual-ravel-k27` has a recorded result:

| Outcome | Result |
| --- | --- |
| Stable 1.x from 1.1.0 and 2.x from 2.0.0 pass lifecycle and standalone acceptance | `ravel:activation` and `ravel:commands` on each build, and the installed smoke on each target |
| 2.0.0 is the minimum of the directory feature, with separate native and directory evidence | `ravel:directory` on the 2.0 build. The native route on 2.0 is every other 2.0 run, with no `skills` |
| Unsupported versions, malformed output and failed probes refuse precisely | `ravel:resolution` on each build. Stub executables prove `ravel 2.0.0` output |
| Examples, typechecks, assets, fixtures and installed checks name each runtime | `ravel:policy`, the target smoke, the release smoke and the formula test |
| Every caller asset has a Rhai port, and receipt bytes match | The case table of `ravel:policy` on each build |
| The TypeScript assets keep their paths, and no owner configuration is overwritten | The release tasks of the gate and `release:upgrade` |
| Directory launches keep the mandate, endings, order and resources | `ravel:directory`, with a plain Git checkout, a launch without a network and a missing checkout |
| Confined native access is measured separately | The native case of `ravel:directory` and the floor case of the release smoke |
| Documents agree with behaviour | The gate's book checks. The Test seams section now states the Linux 2.0 lane |
| Full checks and the archive, formula and floor checks pass | Both gates and the installed checks above |

No outcome needs repair work.

## Publication order

The [coordinated release](specs/coordinated-release.md#publication-ownership-and-order)
owns this order, and its tap states A to D name each published pair. On
2026-11-03 Ravel `main` was `24a545236185`, no tag `v2.0.0` existed and the
tap had no `ravel@2` formula, which is state A.

1. **Acceptance measures the unpublished build.** `common-loading-acceptance-k29`
   uses the 2.0 build of this record from a scratch prefix and a HOME of its
   own. It names the route of each case: native on 1.1, native on 2.0 or
   directory on 2.0. It does not install `ravel@2` or change the owner's
   settings.
2. **Rollout cuts and publishes Ravel 2.0.0 first.** Ravel's release tooling
   cuts 2.0.0 once and publishes only the keg-only `ravel@2`. This is state B.
   The unversioned `ravel` stays 1.1.0.
3. **Rollout replaces the local 2.0 record.** It follows the
   [rollout handoff](ravel-target-inputs.md#replacement-of-the-local-20-record--2026-11-12)
   with the published archives. It then repeats Grove's admission and
   installed checks against the published archives and the `ravel@2` prefix.
   These are the target controls and smoke, the release smoke, the formula
   test, the installed smoke and `ravel:directory`.
4. **Finish releases compatible Grove.** Finish runs the integrated check and
   Grove's minor release while unversioned `ravel` is 1.1.0. This is state C.
5. **Promotion comes last, in finish.** After Grove's publication and its
   installed verification, finish publishes the same Ravel archives as
   unversioned `ravel`. This is state D.

## Limits

- The fixtures prove delivery and Grove's interpretation of it, not the
  behaviour of an LLM. Actual harness behaviour belongs to k29.
- Before the cut, `ravel 2.0.0` output existed only from stub executables.
  On 2026-11-12 `grove-ravel-two-pins-k314` pinned the published 2.0.0
  archives. Admission and the installed checks then ran on a real
  `ravel 2.0.0`, as step 3 of the publication order states.
- k103 ran `task test:native-linux PLATFORM=amd64` under QEMU user mode. 94
  confinement, namespace and signal tests failed on the build commit and on
  its parent alike. Ravel's own campaign lists those rows as not reproducible
  there.
- The plain Git checkout source is not measured at the Linux floor.
