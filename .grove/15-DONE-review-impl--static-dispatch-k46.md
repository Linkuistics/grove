# static-dispatch-k46

**Reviews:** static-dispatch-k12

## Goal

An adversarial, inspection-only read of the whole `static-dispatch-k12` node:
the standalone `harness-dispatch` walking skeleton that every later increment
extends. Produce findings, not fixes.

## What to doubt

The node brief names these as the foundation of every later increment and of
the trust boundary, so cheaper to correct now than after later increments build
on them:

- **The protocol.** The framed channel on descriptor 3, the hello and evaluate
  exchange, the 1 MiB frame cap, and whether any frame, diagnostic or policy
  output can reach stdout, the report or another frame.
- **The worker's location and identity check.** Found only relative to the
  real front executable, never PATH, cwd or environment. The protocol, package
  version and source digest are checked before any entry is handed over. Can
  `build.rs` and `scripts/dispatch.sh` compute different digests for the same
  source set without a test failing?
- **Policy authority.** Only the personal default or an explicit `--config`.
  Is there any path by which a repository's code, or an environment variable,
  selects or shadows an entry? That includes the embedded specifiers
  (`harness-dispatch/sdk` and the two `harness-dispatch/examples/…`) against a
  `node_modules/harness-dispatch` shadow.
- **The deadline's hard kill and reaping.** It holds from worker start, the TERM
  grace is at most one second, and then KILL. Is the worker always reaped, on
  every path, including a result that arrives just as the bound expires, a
  worker that exits early, and one whose descendants hold its streams?
- **The durability of the pre-exec commit.** One exclusive transaction at
  `synchronous = EXTRA` with `fullfsync`. No lock is held across evaluation.
  Failures exit 4 and launch nothing, and committed launch fields never change.
  Is there any path that execs without a committed attempt, or turns an
  attempt into a success?

The last leaf, `choice-and-refusals-k15`, added `--choice`, the refusal
contract and the static starter examples. Doubt these as well:

- Does an explicit choice ever select anything but the named candidate, or
  fall back to a route when the ID is unknown?
- Does the equivalent `inspect` invocation a refused `run` prints really
  reproduce the selection? Consider quoting, hyphen-leading values, relative
  paths with the cwd, and non-UTF-8 inputs, which it renders lossily. Can it
  ever leak the prompt?
- Does every refusal name an input or a source? `support::Run::refusal`
  enforces that only for refusals some test reads.
- Do the examples' effort explanations stay out of model ranking, and is the
  Grove example's kind list exactly Grove's?

## Done when

The findings are written into this leaf, each with its evidence, and the leaf
is retired.

## Decisions (running log)

**Review boundary.** Read the six producer changes from `a0903787c05d`
through `e0d5c67d5e69`, against their current committed source. The reviewed
tip is `e0d5c67d5e69`; the working change began empty. This is an
inspection-only review: no test, build, lint, format or runtime probe was run,
and no production or test file was changed. The producer's recorded checks are
evidence of its runs, not fresh verification by this review.

**Source evidence replaces an out-of-date graph.** Tier 2 verification queried
project `Users-antony-Development-grove.new-tool-for-harness-selection-and-execution`.
Its generation is `2026-09-29T11:18:53Z`; `search_graph` over the package
returned zero symbols with no more pages. `check_index_coverage` checked all
package paths and the supporting manifests, scripts and contract documents.
Every new package path was `not_tracked`, the root manifest and Taskfile had
changed metadata, and `docs/` and `scripts/` were excluded. Findings therefore
rely on direct source reads, not on graph absence. No repository-wide absence
claim is made.

**Keep scheduled work outside the findings.** Handled cancellation and signal
transparency belong to `evaluation-boundary-k27`; computed context and the
256 KiB diagnostic limit belong to `bounded-context-k22`; archive/CPU delivery
belongs to `dispatch-delivery-k16`. Their absence here is not a defect in this
increment. The current static choice does honor an explicit catalog ID and
refuses unknown IDs without falling back. The digest implementations enumerate
the same current source set, and the command-seam tests require a matching
worker. The starter mappings match the methodology's current 23 kinds.

**Integrate before subsequent implementation.** The findings warrant a paired
integration. Cut `static-dispatch-k47` immediately after this review, before the
first later sibling with live work, `dispatch-delivery-k16`. Its charter points
to this review's handle and leaves every finding open to independent triage.
The root still has live work, so retirement closes no ancestor node.

## Findings

These are source-derived findings; their trigger cases were not executed in
this review. P1 concerns the policy/descriptor authority boundary. P2 concerns
correctness and durability. Every finding is actionable for the paired
integration to triage. Source and test paths below are relative to
`crates/harness-dispatch/`.

### F1 — P1: Do not import a lossy replacement of the admitted policy path

`src/authority.rs:103-132` canonicalizes and hashes the selected native path
without checking that it is UTF-8. `src/worker.rs:368-375` then hands the worker
`entry.to_string_lossy()`, and `worker/src/main.ts:81-86` imports that string.
These can name different files on Linux. For example, an explicit file with
the native basename `policy-\xff.ts` is admitted and hashed, but the worker
imports `policy-\uFFFD.ts`. If both files exist, the second file's code runs;
if only the selected file exists, an otherwise readable policy fails to load.
A UTF-8 personal symlink whose canonical target contains invalid bytes has
the same problem.

This violates the explicit/personal policy authority contract: the host may
execute code the owner did not select, and its reported digest belongs to the
other file. The boundary must preserve the admitted path exactly or refuse it
before evaluating anything; replacement characters are not an authority to
execute a sibling. `tests/authority.rs` covers relative paths and symlinks,
but only UTF-8 paths, so neither case detects this substitution.

### F2 — P1: The descriptor sweep leaves inherited authority above its ceiling

`src/worker.rs:227-236` marks descriptors close-on-exec only below
`descriptor_limit()`. At `src/worker.rs:583-591`, that limit is the caller's
soft `RLIMIT_NOFILE` capped at 65,536, or 1024 after a failed limit query.
An inherited descriptor 65,536 or higher on a Linux caller with a larger limit
is never touched and reaches the policy worker. There is also a lower-numbered
trigger: a caller can open descriptor 100, lower its soft limit to 64, and then
launch the front. Lowering the limit does not close existing descriptors;
descriptor 100 again survives this sweep.

The delivered contract admits only standard streams and descriptor 3 to the
worker. A leaked descriptor can carry an open secret file, terminal or writable
completion channel even though the environment was scrubbed. Cover all
actually open inherited descriptors rather than treating a bounded allocation
limit as their maximum. `tests/authority.rs:177-272` checks descriptor 7 only;
`tests/support/mod.rs` likewise probes only 3 through 9 for the final harness.
Those checks cannot detect either leak.

### F3 — P2: First-use directory creation is outside the durable commit

`src/store.rs:286-297` recursively creates the state directory, but never syncs
the parents that acquire the new directory entries. The following SQLite
transaction uses `synchronous = EXTRA` and `fullfsync`, and `run` proceeds to
exec after its commit. SQLite's EXTRA sync covers the directory containing the
deleted rollback journal, which is the new database directory; it does not
establish the persistence of that directory's entry in its parent.
[SQLite's synchronous contract](https://www.sqlite.org/pragma.html#pragma_synchronous)
describes that scope, and the [Linux fsync contract](https://man7.org/linux/man-pages/man2/fsync.2.html)
requires a separate directory sync for directory-entry persistence.

The resulting durability gap is an inference from those contracts and this
creation path, not a simulated power-loss observation. On first use of a new
`--state-dir`, or a new default `.local/state/harness-dispatch` hierarchy, a
power loss after exec can lose a newly created ancestor entry and make the
committed attempt unreachable. The required receipt is then not guaranteed
durable before launch. Directory creation needs to participate in the
durability boundary, with sync failures refusing the launch. The producer's
k24 evidence explicitly relies on SQLite pragmas for durability;
`tests/records.rs:95-132` checks first-use visibility and permissions, which
does not establish persistence of these parent entries.

### F4 — P2: Reject NUL arguments before reporting a valid proposal or recording it

`src/policy.rs:341-344` accepts every literal string, including `"before\0after"`.
Candidate `model` and `effort` strings also admit NUL, and `src/argv.rs:93-145`
copies literals and slot values into argv without checking them. A routes
policy containing such an argument passes validation, and `inspect` reports
success for an executable program. `run` commits its attempt and announces a
handoff before `Command::exec` rejects the invalid argument. The resulting
`exec_failed` refusal points to the program and offers an environment/program
remedy, even when the malformed input is a catalog argument.

NUL is deterministically unrepresentable in native argv, so this is a policy
validation/expansion failure, not a launch availability race. Refuse it with the
argument or slot's policy location before a proposal or handoff commit. The
same catalog-wide malformed-argument contract applies to unselected literals;
optional slot satisfaction can remain selection-specific. The invalid-shape
table in `tests/inspect.rs` covers argument types and slot counts, while the
NUL case in `tests/run.rs` covers only prompt bytes. Neither covers catalog
arguments or values used by model/effort slots.

### F5 — P2: An empty PATH must still search the caller's current directory

`src/program.rs:147-151` filters out an empty PATH as though it were unset.
That bypasses the immediately following empty-entry handling. With `PATH=""`
and an executable `agent` in the caller's cwd, a configured `program: "agent"`
is refused with exit 127 by both forms. An empty PATH is one empty entry, which
the specified lookup resolves against the cwd; `PATH=":"` already reaches
that code. This is a refusal of an available explicitly selected program.

Preserve the distinction between an absent PATH and a present empty PATH.
The area spec's *Policy and joint choice* and README's *The program* explicitly
promise empty-entry behavior. `tests/run.rs` exercises absolute, relative and
nonempty PATH lookup, but not a wholly empty PATH.

### F6 — P2: Lossy refusal argv can turn a rejected input into an accepted selection

`src/cli.rs:294-300` converts every value in the purported equivalent
inspection lossily. `inspect_invocation` also converts argv[0] lossily, and
`src/refusal.rs` does the same for cwd. A parsed `run` with a non-UTF-8
`--task-id` or `--choice` refuses with exit 2 before policy executes, then
advertises an inspection containing replacement characters instead. Pasting
or executing it can evaluate policy and succeed: `--task-id` now contains valid
UTF-8, or a replacement-character candidate ID can match the catalog. Native
config/state paths and cwd can likewise become different filesystem locations.

The original argument cannot be reconstructed from this JSON string or shell
word. Preserve exact bytes in an explicit reversible representation, or mark
the reproduction unavailable when it cannot be faithfully represented;
do not advertise changed inputs as equivalent. The task specifically asks to
doubt this case. `tests/refusals.rs` reproduces awkward but valid UTF-8 inputs,
and `tests/run.rs` checks non-UTF-8 task-ID refusal, but never executes or
qualifies that refusal's reproduction.

### F7 — P2: A prompt value of `--json` must not choose the refusal format

`src/main.rs:38-41` searches every raw argument for `--json`, then continues
using that result for parsed-command failures at line 77. But
`src/cli.rs:153-159` explicitly permits a hyphen-leading prompt value. In
`harness-dispatch run --kind unrouted --prompt --json`, clap consumes the final
word as prompt data and `args.json` is false. The selection refusal nevertheless
renders as JSON. The same prompt on a routed invocation uses text for the
handoff notice, because `run` correctly consults `args.json`; a subsequent exec
failure switches back to JSON. `--task-id --json` has the same ambiguity.

This breaks the requested text/JSON contract and treats caller data as format
control. Once parsing succeeds, rendering should follow the parsed subcommand's
flag; the pre-parse fallback is only needed when parsing fails. The test
`a_prompt_that_starts_with_hyphens_is_still_the_prompt` in `tests/run.rs` uses
`"--json is not a flag here"`, which does not equal `--json`, and therefore
misses this case. Cover the exact value on both a successful invocation and a
refusal.
