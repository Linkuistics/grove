# grove-ravel-runtime-k27 — brief


## Goal
Switch lifecycle and standalone execution to installed Ravel's public two-verb
boundary, with Grove-owned continue results and ending instructions.



## Context
Consume grove-ravel-resolution-k26's staged resolver. Remove its dead-code
expectation and the old sibling locator when switching both consumers.
`cli::execute` and `standalone::run` are the old locator's direct callers.
Installed Ravel 0.1.0's help and docs/session-cli.md describe stdout as the opaque
result, managed output on stderr, and --exit-dir/--runtime-read as public flags.
Use no private worker protocol. k5's full Done when remains the migration contract.

## Done when
- Both callers select and canonicalize Ravel once and use that same executable
  for launch, prompt ending commands and documented runtime confinement reads.
- Both result readers accept the frozen Grove continue value and refuse missing
  or unsupported results; session output cannot pollute the value. Grove retains
  teardown/admission authority, continuation, refusal/resume and cancellation.
  The k26 installed round-trip fixture freezes this value as `grove-continue`;
  use that same literal in both readers and prompt ending instructions.
- New prompts and grove-llm help use ravel end-session RESULT without an ID.
  Preserve explicit compatibility for the still-live v22 original prompt/channel.
  Scrub old/new authority in loop, standalone, Cargo and direct test helpers.
- Existing test support uses isolated PATH/separate prefixes. Real installed
  Ravel/worker fixtures cover both launch/read paths, different package versions,
  old sibling/cwd/old-Ravel/malformed decoys, process cleanup and PTY recovery.
- Every changed source-pinned walkthrough block, manifest and index agrees;
  focused and full relevant runtime nextest checks pass. Keep bundled delivery
  green until k6; do not install Grove or replace the live ending binary.

## Notes
If this fan-out exceeds one focused session, decompose at green expand/migrate
seams. k28 owns generic policy/provenance assets; coordinate public caller context
fields through the parent brief rather than inventing Ravel storage.

## Decomposition

The current runner sends both output streams to the terminal or transcript. Ravel
returns its public result on stdout, so capturing that stream without changing
job/terminal supervision is the prerequisite green seam.

- session-stdout-capture-k30 expands keyed-launch with separate regular-file
  stdout capture for interactive and noninteractive jobs, verified at process
  and PTY boundaries. Existing launches and APIs remain active.
- grove-ravel-consumers-k31 performs the atomic migration of both launch/read
  paths, prompts, confinement and installed-pair/lifecycle fixtures, using that
  capture seam. It owns all remaining Done when above and changed source books.

The node remains live until actual consumer migration passes. No Grove runtime
or source prompt is switched by the expansion child.
