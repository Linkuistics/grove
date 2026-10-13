# session-result-capture-k32


## Goal
Provide the shared caller-owned stdout capture and bounded continue-result reader
needed by both Grove consumers, without switching their active contracts.



## Context
Read grove-ravel-consumers-k31 and grove-ravel-runtime-k27. k30 supplies
run_observed_with_stdout and run_noninteractive_with_stdout; k26 freezes Ravel's
public stdout value as `grove-continue`. Ravel 0.1.0 returns the exact result
without a newline and sends session output to stderr. Use its public CLI only.

## Done when
- grove-loop exposes one capture that allocates and holds an unlinked regular
  file, supplies owned stdout Files, rewinds after supervision and reads at most
  the continue literal's length plus one byte. Missing and unsupported bytes
  refuse; no trimming, transcript parsing or pathname reopening occurs.
- Deterministic tests catch missing rewinds, prefix/trimming acceptance, output
  contamination and unbounded reads. Both k30 runner APIs pass the same reader
  with installed Ravel 0.1.0 and its worker from an independent temporary prefix,
  with fake agents and no host runtime on PATH. No model launches.
- Existing runtime tests, formatting/lints and changed source-book validation
  pass. Use reusable Taskfile commands and preserve complete logs.
- Leave both actual consumers and their prompts on the existing contract;
  grove-ravel-switch-k33 owns the remaining atomic migration. Do not close k31
  or k27, install Grove, or change the live v22 ending channel.

## Implementation plan

1. Write refusal/acceptance and bounded-read tests and observe the failing exact
   result case. Add the shared capture with the crate's existing opaque Error.
2. Exercise both actual stdout runner APIs with the installed Ravel pair,
   including stderr noise and unsupported/missing results.
3. Update the source-pinned grove-loop book and reusable check entry point;
   run focused checks, then the full relevant nextest/lint/book handoff.
4. Retire this child and seal its jj commit, leaving the migration child live.

## Decisions (running log)

- Keep Grove's result meaning in grove-loop, alongside its prompt/loop contract;
  keyed-launch remains unaware of session results. The descriptor holder hides
  file allocation, rewind and bounded validation from both eventual callers.
  Caller ordering and Ravel's successful cleanup remain explicit obligations.
- The narrow adversarial pass found a Linux portability issue in the permission
  assumption: tempfile 3.27's O_TMPFILE fast path can use mode 0666 before umask.
  Classified actionable; set mode 0600 explicitly during capture allocation,
  keeping the existing metadata assertion as the executable check. No second
  review is needed for that direct, test-covered correction. The reviewer found
  no result-interpretation violation; writer closure remains a caller obligation.

## Verified handoff

- The shared API is `grove_loop::session_result::SessionResult`:
  `allocate() -> Result<Self, grove_loop::Error>`,
  `stdout(&self) -> Result<File, grove_loop::Error>` and
  `read_continue(self) -> Result<(), grove_loop::Error>`. The public
  `CONTINUE_RESULT` literal is `grove-continue`. The capture starts empty, has
  mode 0600 and no pathname, and interpretation reads at most 15 bytes.
  k33 must keep the capture until reap and epoch invalidation; caller teardown,
  status, cancellation and group checks precede its result interpretation.
- The exact-value and bounded-read tests failed against the stub, then passed
  with the actual reader. Missing, unsupported, binary, padded and contaminated
  results refuse. Installed Ravel 0.1.0 and its worker exercised both k30 runner
  APIs in a separate prefix with an isolated managed PATH, no host Bun/Node and
  fake shell agents. All four input cases passed through both APIs.
- `task ravel:result RAVEL=/opt/homebrew/bin/ravel`: 4 reader tests and the
  installed-pair test passed. `task test:grove-loop`: 385 passed, one explicitly
  invoked installed test skipped in the default run. `task test:keyed-launch`:
  64 passed; its 12 fixture roles remain driven by their owning tests.
  `task test:grove`: 145 passed, 3 existing fixture/installed roles skipped.
  Both doctest tasks passed (no examples). Checks ran on macOS; the Linux mode
  correction was established against pinned dependency source and the explicit
  mode operation, rather than a Linux execution in this session.
- Workspace `cargo clippy --locked --workspace --all-targets` and formatting
  passed. Grove's worker build required Bun 1.4.2; host 1.4.3 correctly refused,
  so the successful Grove task used the existing pinned binary from
  `/Users/antony/Development/Ravel/target/tools` on its command-local PATH.
- The final grove-loop book check validates 16 roots and 14,287 exact source
  lines, with no deferred ranges. New capture source, moved dependency,
  library-module ranges, indexes and summary prose agree. Final source-book
  validation followed the dependency/module summary corrections.
- Full logs and per-file digests are saved under `target/session-result-capture/`.
  Runtime checks held all 1,939 tracked repository inputs unchanged. The final
  installed-result/book run held 1,941 inputs unchanged, adding the actual Ravel
  front and compiled worker to the repository input set. That final run followed
  prose corrections; this handoff text is written after measurement.

This child meets the expansion contract. Both actual callers still use the
original locator/report/prompts, and the live v22 ending channel is unchanged.
grove-ravel-switch-k33 owns their atomic migration; k31 and k27 remain live.

## Notes
