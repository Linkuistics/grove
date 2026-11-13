# Release readiness record

`release-readiness-checks-k321` measured the final working result on
2026-11-13. It ran the checks of each affected repository against the
published Ravel 2.0.0. It also states each acceptance limit before the
finish handoff. Its times are UTC.

This leaf changed one test script in Grove, as
[The release preparation test](#the-release-preparation-test) states. It
changed no other repository, keg, link, tap, setting or real HOME. It
launched no harness and no default session.

## Result

- `task check:packs` passed for each of the five domain packs, alone and in
  the aggregate with core, at the commits of `plugins/packs.json`.
- Each pack repository, Ravel and TestAnyware passed its own checks at its
  published commit. The tap has no check suite. Each installed tap formula
  passed its own test.
- `bash scripts/check.sh` passed 18 of 18 on Grove commit `639d690125ab`,
  once with the Ravel 1.1.0 keg and once with the published `ravel@2` keg.
  The archive admission read the published 2.0.0 archives and the retained
  1.1.0 pins.
- The first 2.0 run, on `fe475b42725a`, failed one section after its
  assertions passed. The cause was the test's cleanup, which could not
  remove Ravel 2.0's read-only skill cache. Commit `639d690125ab` repairs
  the cleanup.
- A source comparison with `v22.8.0` shows that the working Grove keeps the
  22.8.0 launch-directory and admission contract. No helper preservation is
  needed.
- The real discovery evidence of the staged installed split is summarised
  below, with the build of each trial and each limit.
- Ravel's live book for 2.0.0 keeps the packaging guidance of
  ravel-packaging-guidance-k301.

No check failed at its final subject. So this record carries no failure as
a live leaf.

## Subjects

Each local `main` equalled its remote `main` after `jj git fetch`, and each
working copy was an empty change on that `main`.

| Subject | Location | Published commit | Accepted source commit | Tree ID of the source directory |
| --- | --- | --- | --- | --- |
| Grove | this workspace | `639d690125ab`, the working result. Its parent is `fe475b42725a` | The candidate `2a24d5e61f29` | `961fddd04bb7b271e9896d07dc0225348fa8c378` |
| Ravel | `~/Development/AgenticOrchestration/Ravel` | `d6d817654daf`. The tag `v2.0.0` is `ed6cedbf9a14` | Not a source | |
| RequirementsSkills | `~/Development/AgenticOrchestration/RequirementsSkills` | `87820e4ca282` | `c51ec8bebd81` | `f3cf8128fd206c48d73b061cc5ef6b536d4f9d61` |
| DesignSkills | `~/Development/AgenticOrchestration/DesignSkills` | `93b85332a8d9` | `aa088156d1a5` | `dd04e268aeb5eba82c527755b1f52bee3565a216` |
| EngineeringSkills | `~/Development/AgenticOrchestration/EngineeringSkills` | `55aff4acbd58` | `8d1a2bc3e07b` | `d8a891cd5c13e9866f7937537afc7979fb3ef0f7` |
| ResearchSkills | `~/Development/AgenticOrchestration/ResearchSkills` | `67cb6c856d93` | `8f321f2e99e0` | `79dc2a530cd2d30cd088073423437bb18d61b401` |
| WritingSkills | `~/Development/AgenticOrchestration/WritingSkills` | `2535513621ff` | `578f84d1f5a9` | `ce8f4633533f9f550d414a474561a77f92cebba0` |
| PersonalSkills | `~/Development/AgenticOrchestration/PersonalSkills` | `d1d650ba7978` | `cddcfde901cd` | `f66e0792b9507194842e5b7d44457cf12d70dd98` |
| TestAnyware | `~/Development/AbstractingComputers/TestAnyware` | `3a7c513bbfc0` | `1809a49068a2`, tag `v2.8.0` | `0701492a5dae89e5b967e0122a5f1085cccec291` |
| The tap | `~/Development/Delivery/homebrew-taps` | `f7f9b9b0bc4d`. Homebrew's tap clone is at the same commit | Not a source | |

cross-repo-docs-reconciliation-k319 published each pack's and TestAnyware's
published commit above its accepted commit. `git rev-parse` gave each source
directory the same tree ID at both commits, and each tree ID equals the
accepted value of
[Staging acceptance](owner-skill-delivery-evidence.md#the-accepted-sources).
So the pack check at the accepted commits and each repository's checks at
its published commit read the same source bytes. The changes outside the
source directories are the READMEs and, in WritingSkills, two more
documents and the docstring of `scripts/hook-count.py`.

## Grove's checks

### The two lanes

Each lane ran the recipe of
[the stage record](grove-dual-ravel-stage-record.md), in a new scratch HOME,
with `mise exec bun@1.4.3 -- bash scripts/check.sh`.

| Lane | `GROVE_TEST_RAVEL` | `GROVE_TEST_RAVEL_RUNTIME` | Guard |
| --- | --- | --- | --- |
| 1.1 | `/opt/homebrew/Cellar/ravel/1.1.0/bin/ravel`, `ravel 1.1.0` | `1.1` | None |
| 2.0 | `/opt/homebrew/Cellar/ravel@2/2.0.0/bin/ravel`, `ravel 2.0.0` | `2.0` | A `ravel` first on PATH that prints `host-ravel-reached` and exits 86 |

The `ravel@2` keg is the published 2.0.0. Its `bin/ravel` has the SHA-256
`67c92bed7ae2…` and the build identity `7f2ff6b3f522…`, as
[Published source acceptance](owner-skill-delivery-evidence.md#the-published-ravel)
records.

| Run | Commit | Time | Result |
| --- | --- | --- | --- |
| 1.1 | `fe475b42725a` | 05:27:06Z to 05:33:38Z | 18 of 18 passed |
| 2.0 | `fe475b42725a` | 05:33:54Z to 05:39:25Z | 17 of 18 passed. "release preparation" failed, as [The release preparation test](#the-release-preparation-test) states |
| 1.1 | `639d690125ab` | 05:43:47Z to 05:49:25Z | 18 of 18 passed |
| 2.0 | `639d690125ab` | 05:49:25Z to 05:54:56Z | 18 of 18 passed |

In each run on `639d690125ab`, `cargo nextest` ran 1344 tests with 1344
passes and 29 skips. The 2.0 run never reached the guard, and no `rm` error
occurred. The working copy above `639d690125ab` held only this leaf's task
file, which no check reads.

The archive admission section runs `scripts/test_ravel_target_inputs.py`
over the recorded pins. The pins of lane 2.0 are the published 2.0.0
archives, and the pins of lane 1.1 are the published 1.1.0 archives. Each
local archive gave its pinned SHA-256:

| Archive | SHA-256 |
| --- | --- |
| `ravel-v2.0.0-aarch64-apple-darwin.tar.xz` | `5957840ab33f4331b794539dbd455d801f71c6c6485c781dc290cfb48490231d` |
| `ravel-v2.0.0-aarch64-unknown-linux-gnu.tar.xz` | `a4750ea439450e88621be411183e7bf30d2616f9a545b64a30445df3a9f02bee` |
| `ravel-v2.0.0-x86_64-unknown-linux-gnu.tar.xz` | `906e589f72161ed5dce5db683e6af10963c1952bdf177c6b343d1918928c3dd1` |
| `ravel-v1.1.0-aarch64-apple-darwin.tar.xz` | `92ec94083495f536136a17310ce1624004d145569f6fbf92da978aee65e7c357` |
| `ravel-v1.1.0-aarch64-unknown-linux-gnu.tar.xz` | `4334bd0adfa97391925b44029e96ce70744ecf3d141a2b5bb21174fd7cfa9c60` |
| `ravel-v1.1.0-x86_64-unknown-linux-gnu.tar.xz` | `2a0f91c793069fa2dd8d73fcbe0fe383d383d029f60adc4a5b7f3726919b60fb` |

`task ravel:targets:controls ARCHIVES=target/ravel-archives
ARCHIVES_2=target/ravel-archives-2.0` passed on `639d690125ab`, from
05:56:12Z to 05:56:37Z. It admitted each real
archive of both lanes. Its changed-byte, cross-lane and unrecorded-target
controls refused as wanted. It reported that the 2.0 lane records every
target.

### The pack check

`task check:packs` ran inside each lane, as the section "external packs".
Each pack's checkout under `target/external-packs/<id>/<sha>` was clean and
at its recorded commit, so the script reused it.

| Pack | Commit | Alone | Aggregate with core |
| --- | --- | --- | --- |
| `requirements` | `c51ec8bebd811c0087ad19a1d66d0aa87bd01386` | passed | passed |
| `design` | `aa088156d1a55e28dd9b59f0330f4ab9f1f3d330` | passed | passed |
| `engineering` | `8d1a2bc3e07bec4ce57e0391a6aa778dff63a017` | passed | passed |
| `research` | `8f321f2e99e09a8ba5e60fa4a08871727e84df62` | passed | passed |
| `writing` | `578f84d1f5a9bb04573a704bca639530d93b6bca` | passed | passed |

The script reported "5 recorded pack(s) pass alone and in the aggregate" in
each lane. The section "conformance" checked core alone.

### The release preparation test

The first 2.0 run, on `fe475b42725a` from 05:33:54Z to 05:39:25Z, passed 17
of 18. The section "release preparation" printed "release preparation
tests: all passed" and then 378 `rm` errors, and it exited 1. The 1.1 run
on the same commit passed that section.

The cause is the published `ravel@2` keg. Its
`share/ravel/installation.json` names `/opt/homebrew/share/ravel/skills`,
which holds the four installed skill kegs. The test's HOME has no Ravel
settings, so Ravel 2.0 resolved those sources and stored their snapshots
read-only under `real-home/.cache/ravel`. The test's cleanup was
`trap 'rm -rf "$scratch"' EXIT`. That `rm` failed, and bash gave its status
to the script.

Ravel 1.1 has no skill cache. The earlier 2.0 runs of
[the stage record](grove-dual-ravel-stage-record.md) used a prefix of
`task install`, which holds no installation record. The accepted build at
`~/.local/state/owner-staging/ravel-2.0` also has only `examples` and
`notices` under `share/ravel`.

Commit `639d690125ab` changes the trap to
`chmod -R u+w "$scratch" 2>/dev/null; rm -rf "$scratch"`. Other scripts of
this repository already use that form, for example
`scripts/grove-installed-smoke.sh` and `scripts/release-formula.test.sh`.

| Control | Result |
| --- | --- |
| The test alone in lane 2.0 before the change | Exit 1 after "all passed", with 378 `rm` errors |
| The test alone in lane 2.0 after the change | Exit 0, no `rm` error |
| The test alone in lane 1.1 after the change | Exit 0 |
| `shellcheck scripts/release-prepare.test.sh` | No finding |

The finish check runs before promotion, while unversioned `ravel` is 1.1.0,
so it would have passed without the change. After promotion, unversioned
`ravel` is 2.0.0 with the same installation record. Each later release's
`scripts/check.sh` would then have failed this section.

The two failed runs left their scratch folders with read-only files. This
leaf made them writable and removed them.

## The other repositories' checks

### The pack repositories

Each repository ran `task check` at its published commit, from 05:28:07Z to
05:28:30Z. Each conformance task checks the pack against the Grove commit
that the repository's `Taskfile.yml` pins in `GROVE_COMMIT`.

| Repository | Tasks | `GROVE_COMMIT` | Result |
| --- | --- | --- | --- |
| RequirementsSkills | `check:plugin`, `check:conformance` alone and in the aggregate | `302f85429e4b` | passed |
| DesignSkills | The same | `84fba18c1f98` | passed |
| EngineeringSkills | The same, and `check:doubt` with its mutation controls | `f45c5a53796f` | passed |
| ResearchSkills | `check:plugin`, `check:conformance` alone and in the aggregate | `57414e00cd87` | passed |
| WritingSkills | `check:plugin` for three plugins, `check:hooks` with its controls, `check:hook-writes`, `check:tc-checker`, `check:tc-hook-lint`, `check:tc-pi`, `check:conformance` | `4cfac6295881` | passed |
| PersonalSkills | `check:plugin` for two plugins, `check:frontmatter`, `check:guardrail-test` with 37 passes, `check:shellcheck` | None | passed |

`claude plugin validate` gave two warnings for each plugin: no `version`
in the manifest and no icon. The packs omit both by design, as
[the moves record](repository-moves-record.md) states.

### Ravel

Ravel's default workspace was an empty change on `main` `d6d817654daf`.
ravel-evidence-step-invocation-k332 ran the same two suites there.

| Command | Time | Result |
| --- | --- | --- |
| `task check` | 05:56:50Z to 05:59:19Z | Passed. The sections formatting, scripts, book-bun-integrity, harness-smoke-fakes, installed-record, lint and terminal-recovery passed. `cargo nextest` ran 713 tests with 713 passes and 48 skips |
| `task release:lifecycle:test` | 05:59:19Z to 06:07:20Z | Passed: "release: lifecycle passed" |

The working copy had no change after the runs.

### TestAnyware

TestAnyware has no aggregate check task. This leaf ran each task that needs
no person, no VM and no Apple service. The working copy was an empty change
on `main` `3a7c513bbfc0`, and it had no change after the runs.

| Task | Time | Result |
| --- | --- | --- |
| `plugin:check` | 05:56:58Z to 05:56:59Z | Passed, with the warnings of no `version` and no icon |
| `scripts:check` | 05:56:59Z to 05:57:00Z | Passed: `shellcheck -x` and `shfmt -i 2 -ci -d` |
| `release:test` | 06:07:39Z to 06:07:52Z | Passed: "release-pipeline-test: all passed". It asserts the keg layout of the skill source |
| `cli:test` | 06:07:52Z to 06:15:44Z | Passed. `cargo test --workspace` ran 59 test binaries with 1655 passes and 12 ignored live-VM tests |
| `agent:macos:test` | 06:15:44Z to 06:16:44Z | Passed. XCTest ran 6 tests and Swift Testing ran 47, with no failure |

The narrower `app:test`, `config:test`, `slots:test` and similar tasks run
subsets of `cli:test`. `cli:clippy` reports and never gates, by its
description. The live tasks need a person and a VM slot.

### The tap

The tap has no Taskfile or test suite. Homebrew's tap clone
`/opt/homebrew/Library/Taps/linkuistics/homebrew-taps` was at
`f7f9b9b0bc4d`, the same commit as the working copy. Homebrew was
7.0.13-83-ga6fea6b. Each command ran with `HOMEBREW_NO_AUTO_UPDATE=1`.

`brew test linkuistics/taps/<name>` passed for each installed formula:

| Formula | Installed keg that the test read |
| --- | --- |
| `personal` | `2026-11-06_1` |
| `superpowers` | `7.0.0_1` |
| `quint-llm-kit` | `1.0.0_1` |
| `testanyware` | `2.8.0_1` |
| `ravel` | `1.1.0` |
| `ravel@2` | `2.0.0` |
| `grove` | `22.8.0` |

Homebrew's linters are advisory here. No repository procedure gates on them,
and the earlier records accepted the same classes of offence.

| Command | Result |
| --- | --- |
| `brew audit --strict` | `personal`, `superpowers` and `quint-llm-kit`: no problem. `ravel`, `ravel@2`, `grove` and `testanyware`: a `version` that the URL repeats. `testanyware`: `on_linux` in `install` and `Formula["python@3.12"]`. `grove` 22.8.0: the order of `depends_on` and two `Formula["ravel"]` paths. `ravel@2`: a redundant `to_s` at line 67 |
| `brew style` on each formula file | The same code offences. `atlas`, `linkuistics-herdr`, `personal`, `quint-llm-kit`, `ravel` and `superpowers` had none. The first word of the `desc` of `ravel-lite` is an article |
| `brew readall --aliases linkuistics/taps` | Exit 1. It simulates each macOS version, and on an Intel or earlier system `grove`, `ravel`, `ravel@2` and `testanyware` have no URL. These formulae publish Apple silicon and Linux archives only, and Ravel 1.1.0 has the same result |

The redundant `to_s` of `ravel@2` is the one offence that a 2.0.0 formula
added. It comes from Ravel's template `scripts/templates/ravel.rb.tmpl`, so
promotion copies it into unversioned `ravel`. It changes no behaviour. Grove's
release replaces `grove` 22.8.0 from `scripts/templates/grove.rb.tmpl`.

## The launch-directory and admission contract

The running driver is Grove 22.8.0. After finish releases the next minor
version, finish runs the newly installed `grove-llm record-teardown`. That
binary must admit the session through the launch directory and epoch that
the 22.8.0 driver wrote.

`git diff --stat v22.8.0 639d690125ab` over the binary's whole input gave
this result. `v22.8.0` is `bacb255aba8a`.

| Input | Change since `v22.8.0` |
| --- | --- |
| `crates/grove-llm/src`, all files, and `crates/grove-llm/Cargo.toml` | None |
| `crates/grove-loop/src/driver_lease.rs`, which holds `admit_ambient_session` and `SessionEpochGuard` | None |
| `crates/grove-loop/src/launch_directory.rs`, `verbs.rs`, which holds `record_teardown`, `lib.rs` and `session_result.rs` | None |
| `crates/ordinal-fs-tree`, `Cargo.toml` and `Cargo.lock` | None |
| `crates/grove-loop/src/loop_driver.rs` | A doc comment only |
| `crates/grove-loop/src/prompt.rs` | The prompt text that a new driver composes: the bare skill name and the force-stop report |
| `crates/grove-loop/src/session_command.rs` | The standalone request without `--confine` |
| `crates/grove-loop/src/ravel_job.rs` | A detached job loses nine repository and multiplexer variables |
| `crates/jj-workspace/src/lib.rs` | `Workspace::resolve` calls the new `find_root`. The walk and the canonical path are the same |

The last four rows change what a new driver sends. None is on the path of
`record-teardown`, which reads `GROVE_LAUNCH_DIR`, requires the session
epoch through `SessionEpochGuard::require_launch_dir`, and writes through
`verbs::record_teardown`. So the installed `record-teardown` of the next
release admits under the running 22.8.0 driver as 22.8.0's own does.

The tests `crates/grove-llm/tests/record_teardown.rs` and
`crates/grove-loop/tests/driver_lease.rs` passed inside `cargo nextest` in
each lane. They run the working binaries, not the 22.8.0 driver.

The finish session's ending command names
`/opt/homebrew/Cellar/ravel/1.1.0/bin/ravel`. homebrew-cleanup-protection-k316
protects that keg, and the release does not touch it.

## Discovery evidence of the staged installed split

The real Claude Code and Codex trials ran in the retained staging HOME
`~/.local/state/owner-staging/home`, with its own `.codex` tree. The
sources came from kegs: Grove's six from the candidate keg, the other four
from their installed kegs. No native copy of a delivered skill was present.
[Staging acceptance](owner-skill-delivery-evidence.md#staging-acceptance)
holds the records. This section summarises them.

### Builds

| Subject | Value |
| --- | --- |
| Ravel | The accepted 2.0 build at `~/.local/state/owner-staging/ravel-2.0`: line `skill-directory-delivery` at `ff14cba826b0`, build identity `58d479ed058f…`, `bin/ravel` SHA-256 `3a9a9129573e…`. It reports `ravel 1.1.0` |
| Grove sources | The candidate keg of `ef8a79dd7621`, `grove-core` digest `8d92cd58…`, for the first trials. The candidate keg of `2a24d5e61f29`, `grove-core` digest `d8eec388…`, for the reruns of core-candidate-restaging-k300 |
| Other sources | `personal` 2026-11-06, `superpowers` 7.0.0, `quint-llm-kit` 1.0.0 and `testanyware` 2.8.0, at their accepted digests |
| Claude Code | 2.1.319 for the early trials, 2.1.322 for the later trials and the reruns |
| Codex | 0.167.0 in each run |
| Models | `claude-sonnet-5-5` at low effort and `gpt-6.1-sol` at low effort, except where the table names another |

### Trials

| Trial | Claude Code | Codex | What it shows |
| --- | --- | --- | --- |
| Isolation | `isolation-2`, 2.1.319 | The same record | No real-HOME subject changed. No superseded skill in either native listing |
| Mandatory read and implicit discovery | `discovery-claude-1` | `discovery-codex-1` | The mandatory file first, then `cli-tool-design` from the design snapshot before the first edit, with no skill named |
| Kind skill with a relative resource | `kind-skill-claude-4`, 2.1.322, candidate `2a24d5e61f29` | `kind-skill-codex-4`, candidate `2a24d5e61f29` | `grove-planning` and the spine resource `references/bootstrap.md` from the core snapshot |
| Delegate | `delegate-claude-1` | `delegate-codex-1` | A delegate read `coding-style-bash` from the engineering snapshot. In Codex the delegate found it through the descriptor. In Claude Code the parent passed the path |
| Compaction | `compaction-claude-opus-1` and `-2`, `claude-opus-5-5` at high effort | `compaction-codex-1` | The skill read again after compaction |
| Stale copy | `stale-copy-claude-2`, 2.1.322, candidate `2a24d5e61f29` | `stale-copy-codex-2`, candidate `2a24d5e61f29` | Ravel's copy of `grove` read, and no file of the planted copy |
| Off value, offline launch, missing source | `sources-3`, no harness | The same | `disabled`, the same view offline, and `skill_source_missing` |
| Guidance probe | `guidance-claude-1` | Not applicable | The guidance of a disabled plugin and a nested probe |
| TestAnyware | `testanyware-vm-claude-1`, 2.1.322 | `testanyware-vm-codex-2` | The command reference and a real VM |
| Quint MCP | `quint-mcp-claude-1`, 2.1.322 | `quint-mcp-codex-2` | The skill with its server |
| codebase-memory MCP | `codebase-memory-mcp-claude-1`, 2.1.322 | `codebase-memory-mcp-codex-2` | The skill with its server |
| Hooks and `/guardrail` | `native-residue-claude-1`, 2.1.322 | Not applicable | The technical-communication hooks run from the source in place |
| Snapshot reads in `auto` mode | `auto-mode-read-claude-1`, 2.1.322 | Not applicable | Read and Bash in the owner's launch shape with no permission denial |

Each verdict reads the session's native listing, and no listing held a
superseded name. The record paths are relative to
`~/.local/state/owner-staging/trials`, except `isolation-2`.

### Limits of the discovery evidence

- No model trial ran on the published Ravel 2.0.0. The trials ran on the
  accepted build `58d479ed…`. The published build differs from it by
  `576811bcb4b0`, the supervision repair of ravel-stop-eperm-k302, and by
  the release commits. Skill resolution is the same, and
  [Published source acceptance](owner-skill-delivery-evidence.md#published-source-acceptance)
  resolved each of the ten sources at its accepted digest with the published
  build.
- Grove's sources came from the candidate keg, not from a released Grove
  keg. Finish's release publishes that keg, and the switch compares its
  installed digests before any change.
- Most model trials ran once on each harness. Each shows that the behaviour
  occurs, not how often.
- The Claude Code compaction case passed with `claude-opus-5-5` at high
  effort. With `claude-sonnet-5-5` at low effort it failed in five of five
  runs. No owner route uses that setting. After promotion the owner's
  `config.rhai` routes most `review-*` kinds to `claude-sonnet-5-5` at
  medium or high effort, and no compaction run measured that setting.
- The Claude Code delegate shows access to the snapshot, not discovery,
  because the parent passed the path.
- No trial reads a skill of `requirements` or `research` by a model. Their
  trials prove resolution, refusal and digest only.
- The guidance probe and the hooks ran on Claude Code only, by the scope
  decisions of the staging record.
- The staging HOME does not isolate SSH.
- The quiet-interval measurement used Claude Code 2.1.322. It needs another
  run if the held version differs.
- The gaps of the common-loading acceptance record keep the dispositions of
  its [Gap register](common-loading-acceptance-record.md#gap-register). They
  include the Codex `token_budget` loss and the absence of a real harness on
  Linux.

## Ravel's live book

Ravel's README sends a reader to the book of each release on GitHub Pages.
The index at `https://linkuistics.github.io/Ravel/` lists "ravel 2.0.0
(newest)" and "ravel 1.1.0".

`https://linkuistics.github.io/Ravel/v2.0.0/06-reference.html` returned
HTTP 200, SHA-256 `6887c77eac44…`. Its section "Default skill directory"
holds the guidance of ravel-packaging-guidance-k301 word for word. It
names the private copy at `libexec/skill-sources/ID/` and the relative
directory symlink `share/ravel/skills/ID`. It warns that a real source
directory under `share/ravel/skills` can become file links, which refuse as
`skill_resource_invalid`. The 1.1.0 page has no such text, so the search
can fail.

On Ravel `main` `d6d817654daf`, `jj diff` from the guidance commit
`5e778ef86b8d` changes neither that paragraph nor its table in
`docs/book/src/06-reference.md`. The tag `v2.0.0` contains the commit.

## Acceptance limits

This list is for the finish handoff.

- Grove's check after the final integration on `main` belongs to finish.
  This record measured the working result `639d690125ab`.
- Finish's check runs with unversioned `ravel` 1.1.0, as `scripts/check.sh`
  resolves it from PATH. This record also ran the 2.0 lane. After
  promotion, a check without `GROVE_TEST_RAVEL` runs on 2.0.0.
- The tap's Homebrew linters report the offences above. The new one is the
  redundant `to_s` of Ravel's template. A later Ravel change can remove it.
- The launch-directory contract is shown by source comparison and by the
  working binaries' tests. No test ran the next release's `record-teardown`
  under a live 22.8.0 driver.
- The discovery evidence has the limits of
  [its section](#limits-of-the-discovery-evidence).
- TestAnyware's live VM checks did not run. They need a person and a VM
  slot, and this workstream changed no VM code.

## Rerun commands

The scratch paths name a new empty directory. `<lane>` is `1.1` or `2.0`.

```sh
# Grove, each lane. See "The two lanes" for the environment.
mise exec bun@1.4.3 -- bash scripts/check.sh

# Grove's pack check alone.
task check:packs

# Each pack repository, at its published main.
task check

# Grove's archive admission on the real archives.
task ravel:targets:controls ARCHIVES=target/ravel-archives ARCHIVES_2=target/ravel-archives-2.0

# Ravel, at main.
task check
task release:lifecycle:test

# TestAnyware, at main.
task plugin:check
task scripts:check
task release:test
task cli:test
task agent:macos:test

# The tap.
HOMEBREW_NO_AUTO_UPDATE=1 brew test linkuistics/taps/<name>
HOMEBREW_NO_AUTO_UPDATE=1 brew audit --strict linkuistics/taps/<name>
HOMEBREW_NO_AUTO_UPDATE=1 brew readall --aliases linkuistics/taps

# The source comparison.
git diff --stat v22.8.0 HEAD -- crates
```
