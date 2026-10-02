# sample-policy-k10

## Goal

`harness-dispatch init` installs a sample policy that is the owner's pinned
configuration converted, and a supported helper lets one checkout choose among
the selections that policy offers.

## Context

- `docs/specs/harness-selection-and-execution.md`: *The sample policy and the
  choice file*, `init` under *Command interface*, *Delivery and release*, and
  the sample row of the test seams.
- Root brief, requirements 11, 12 and 13.
- `parity-fixture-k7` recorded the fixture the sample is compared against.

## Done when

- The SDK's choice-file helper behaves as the specification states and is
  tested as delivered: an offered name selects, an unoffered name refuses
  naming the file, the name and the names offered, and an absent file leaves
  the default.
- For every kind under every selection it offers, the installed sample selects
  the program and arguments in the parity fixture. Its default is `claude-led`
  with `codex-sol`. It refuses a caller that did not pass a parameter the
  selected command needs.
- `init` writes the sample to the personal default path, refuses when anything
  is already there, and reports that the sample runs `codex` with approvals
  off. `run` and `inspect` with no policy refuse as `policy_missing`, naming
  `init`.
- The sample ships in the release archive and the installed layout, and the
  installed-layout smoke test installs it with `init` and inspects it.
- The dispatch README covers installing the sample and the choice file.
- `bash scripts/check.sh` passes.

## Notes

- The front carries the sample's text, so `init` needs no worker.
- No table helper goes into the SDK. The sample's tables are its own code.

## Decisions (running log)

**The sample's source is `worker/sample/policy.ts`, shipped as
`examples/sample.ts`.** It began in `worker/examples/`, and the first full check
failed twice: `tests/grove.rs` and `tests/hostile.rs` both read that directory
as the list of registered examples and import each file by its specifier. That
invariant is load-bearing, so the sample moved out instead of being exempted in
two tests. The build copies it to where the specification places it, the
worker's type check covers it, and the front embeds it with `include_str!`. It
is not in the worker's source digest, because it decides nothing about the
compiled worker; `tests/sample.rs` fails a readable copy that has gone stale.

**`readChoice` returns the names, `undefined`, or a refusal.** A refusal is a
`Refused` value for `select` to return, never a throw: the command's own
`selection_threw` remedy tells an author to decline by returning one, and a
throw would report the policy as broken when the choice file is. Its codes are
`choice_unoffered` and `choice_unreadable`. The specification now says the
refusal is a returned value.

**The helper stats, then reads.** A file that is not regular, or is over 4 KiB,
is refused without being opened, so a FIFO cannot hold a selection. A dangling
link is an absent file.

**The declarations build admits Node's types, not Bun's.** The helper is the
first SDK code that calls a runtime API, so `"types": []` became
`"types": ["node"]`. The SDK, the adapter and the examples still compile with
no Bun type.

**`init` takes no flag, reads no owner setting, and creates the file
exclusively.** Exclusive creation is the one step that refuses a file, a
directory and a dangling link alike. Its refusals are `policy_exists` and
`policy_unwritable`, stage `authority`, exit 3. A malformed settings file has
nothing to do with installing a policy, so `init` does not read it, and the
specification's *every command reads it* now excepts `init`.

**The sample is tables and one `select`.** Routes name a role, arrangements
bind the roles and patch routes, modifiers patch a harness's own model and
effort and patch routes. A later patch wins, and a route's model or effort wins
over its harness's. `release-notes` routes to a third, fixed harness, which is
why `high-effort` leaves it alone. Its own refusals are `incomplete_mapping`,
`parameter_missing` and `choice_arrangement`.

**The parity sweep was seen to fail.** All 384 commands matched on the first
run, so the sample was mutated four ways (a modifier's route patch, an
arrangement's role override, the default, an argument order). Each failed the
test at the kind and selection it should, and the file was restored to its
digest. The first mutation failed under `codex-led`, where the patched review
is Claude's: `codex-sol` lowers those reviews whichever harness holds them, as
the fixture records.

**The sweep runs serially.** An inspection takes about 17 ms, so 384 of them
take about seven seconds and need no threads.

**No in-session reviewer was spent.** The claim a reviewer could have tested,
that the sample reproduces the configuration, is covered by an executable seam
that was seen to fail.

**No review leaf is cut.** The sample's correctness is the parity sweep's, and
the helper and `init` are small, with their cases at the command seam. Nothing
here is the load-bearing artifact a review chain is earned by.
