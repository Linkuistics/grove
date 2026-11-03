# ravel-confinement-code-removal-k147

## Goal

Remove the confinement code, flags, backends and tests from Ravel on the
bookmark `skill-directory-delivery`. Give the two removed flags the refusal
`confinement_withdrawn` of the design.

## Context

- The sections "Guarantees of a confined launch", "The refusal of the removed
  flags", "Source", "Tests" and "Scripts" of Ravel's
  `docs/skill-discovery-design-evidence.md`.
- "No confinement" of Ravel's `docs/specs/session-execution.md`, and the
  cases "Removed confinement flags" and "Read access" of the acceptance table
  in `docs/specs/skill-discovery.md`.

## Done when

- The source, test and script items of the handoff are done. Each open choice
  of the handoff has a recorded decision.
- The CLI seam tests the new acceptance cases.
- `task check`, `task test:cli`, `task harness:smoke:test` and
  `task docs:examples:test` pass.
- `task test:native-linux PLATFORM=arm64` passes with the split container
  options.
- The change is a described commit on the bookmark.

## Notes

Do not publish a release, push the bookmark or install `ravel@2`. The book
changes in ravel-confinement-book-removal-k148.

## Decisions (running log)

- **The refusal keeps two hidden flags.** `--confine` and `--runtime-read`
  stay as hidden arguments that take an optional value and repeat. The front
  parses with `ArgMatches`, and `cli::withdrawn` refuses the flag with the
  lower clap index. So the first flag names the input, and a word that clap
  took as the value of `--prompt` has no index. The check runs before
  `run::run`, so no settings read, cache work, worker or channel precedes it.
- **The remedy names the harness sandbox.** It says to remove both flags, to
  select the sandbox of the harness in the configuration script, such as
  `codex --sandbox workspace-write`, and that Ravel 1.1 keeps `--confine`.
- **`Prepared::settings_path` goes too.** Only the confinement grants read it.
- **Two cleanup tests take new names.** Neither can report a confined
  directory, so `unwritable_result_stdout_refuses_before_launch` and
  `worker_private_cleanup_failure_blocks_admission` state what they still
  test. The fixture `cleanup_fault` now waits until the barrier run removed
  its result channel.
- **The `requires` mechanism of the example checker goes.** Confinement was
  its only requirement, and no book example needs a platform feature. A
  requirement that only a test supplies would keep an unused path. An
  `<!-- example: ... -->` comment now fails as a page error, so a stale
  directive cannot pass unread. The control `a_directive_fails` and the
  mutant `directive-ignored` replace the skip controls. The summary line has
  no skipped count. ravel-confinement-book-removal-k148 must change
  `docs/book/examples.md` and chapter 10 to match, and remove the directive
  in chapter 5.
- **The amd64 helper keeps both relaxed options.** The handoff names
  `seccomp=unconfined` as still needed. It is silent on
  `systempaths=unconfined`, and no amd64 campaign has run without it. The
  arm64 container keeps only `--init`.
- **The `pivot_root` of the emulated route stays.** The handoff makes a
  `chroot` optional, and the measured campaigns used `pivot_root`.
- **The first arm64 campaign failed on a fixture, and the fix runs each
  write try in a subshell.** The campaign `target/native-linux-k147/arm64` of
  commit `29928824` failed the new read access test. The snapshot modes
  denied both changes. But dash, the `sh` of Ubuntu, ended the agent with
  status 2 when the redirection of `:` failed. A control in the campaign image
  showed status 2 for the old form and a complete run with both changes denied
  for the new form. The fix is in commit `dc162664`.
- **The second arm64 campaign passed.** `task test:native-linux
  PLATFORM=arm64 OUT=target/native-linux-k147/arm64-r2` of commit `dc162664`
  passed each of its 12 steps. Its `check` and `test` each ran 687 tests,
  and all 687 passed. The container had `--init`, no security option and no
  `bwrap`, from image `sha256:746f5e9a4aa0`. The output stays in Ravel's
  `target/`, because ravel-confinement-stage-record-k149 records the build
  campaign. On macOS, `task check` ran 694 tests and all passed, and
  `task test:cli`, `task harness:smoke:test`, `task docs:examples:test` and
  `task docs:examples:mutants` passed.
