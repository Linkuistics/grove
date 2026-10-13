# session-authority-scrub-k34


## Goal
Remove enclosing old/new session authority at both Grove spawn boundaries,
Cargo's child environment and the shared direct-test helper before the switch.



## Context
Read the inherited switch contract. Existing callers still use dispatch; this
is a green guard expansion, with no prompt/result/locator migration. Ravel
0.1.0's inherited ending context is RAVEL_EXIT_FILE. Retain the original live
session's harness-dispatch exit ending.

## Done when
- Lifecycle and standalone scrub RAVEL_EXIT_FILE, GROVE_SIGNAL_FILE,
  GROVE_RUN_SIGNAL_FILE, GROVE_LAUNCH_DIR, HARNESS_DISPATCH_EXIT_FILE,
  HARNESS_DISPATCH_RUN_ID, HARNESS_DISPATCH_STATE_DIR and the retired
  GROVE_HARNESS_PID/GROVE_CLAUDE_PID handles before granting fresh context.
- Cargo force-clears these names; the shared direct-test helper removes them.
  Regression checks prove inherited values cannot override the Cargo guard or
  reach helper children. Real policy/process tests exercise both callers with
  inert enclosing controls, explicit owner grants and observed dirty controls.
- Changed source books, manifests and indexes agree. Focused red/green checks,
  full relevant nextest suites and book validation pass with complete logs.
- Cut the atomic migration successor before retirement; k33/k31/k27 stay live.

## Implementation plan
1. Extend Cargo/helper regression checks and add lifecycle/standalone policy
   refusal probes; run them and confirm the missing scrub causes failures.
2. Expand existing lists without changing their callers; force-clear Cargo's
   matching names. Update explanatory prose and source-pinned fragments.
3. Run focused checks, relevant runtime suites, format/lint/book validation;
   freeze measurement subjects and retain complete logs under target/.
4. Cut the remaining atomic migration child, retire this expansion and seal its
   focused jj change. Signal through the original harness-dispatch binary last.

## Decisions (running log)
- Keep each existing scrub seam and its ownership. Expand its named controls
  without adding a new API or switching either consumer. Test actual policy
  inheritance with an owner-granted ordinary variable as the positive control.
- All three regressions failed before the production/config change. The direct
  policy controls refused as intended; both Grove launch paths then refused
  because inherited authority reached selection, and Cargo omitted the old run
  identity guard. The expanded lists and force-clears pass all three under
  explicitly inherited inert values. Keep the nine old/new names on every
  existing scrub seam, including retired PID handles.
- The host Bun is 1.4.3; use the existing verified Ravel checkout binary at
  /Users/antony/Development/Ravel/target/tools/bun (1.4.2) on the build PATH.
  No installation or runtime pin is changed.
- Decline an in-session reviewer for this mechanical guard expansion: the
  observed failing real-policy probes and Cargo/helper child checks are the
  stronger instrument for inheritance than a second read of the same lists.
  The next child retains the atomic migration's architectural review decision.

## Verification and handoff

- `red.log`: all three new/extended focused cases failed on the expected
  missing guards, after direct policy probes refused as their dirty controls.
- `green.log`: all three passed with all nine controls inherited as inert
  values. `task test:authority` passed all nine authority checks.
- `runtime.log`: `task test:grove` passed 147 cases (three explicitly ignored
  subprocess/installed roles); `task test:grove-loop` passed 385 cases (one
  installed role ignored), and its executable-doc check has zero examples.
- Formatting and strict clippy for both changed packages passed; final book
  validation reconstructs overview's 1,419 lines and grove-loop's 14,291 lines,
  with no deferred source. Logs: `format.log`, `clippy.log`,
  `overview-book.log`, `grove-loop-book.log`.
- All 1,961 per-file before/after digests match. Subjects are all tracked files,
  compiled dispatch assets, installed Grove/dispatch/Ravel commands and the
  build Bun. Files and full logs are in `target/session-authority-scrub-k34/`.
- grove-ravel-migrate-k35 carries all remaining atomic migration work. This
  guard expansion changes no actual locator, argv, result reader or prompt;
  k33, k31 and k27 remain live. No install, release or teardown was performed.

## Notes
