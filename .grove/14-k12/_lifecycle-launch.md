# lifecycle-launch-k12 — brief

## Goal

Bare `grove` launches every lifecycle session by running `harness-dispatch run`
itself. Nothing on the launch path and no tree verb reads Grove configuration.

## Context

- `docs/specs/harness-selection-and-execution.md`, *Grove integration*: *A
  lifecycle session* and *A refusal*; and the two Grove launch boundary rows of
  the test seams.
- Root brief, requirements 1, 3, 6 and 9.
- `direct-dispatch-k4`'s note *Where the design lands in the code*, the *Grove*
  entry.

## Done when

- The driver runs `harness-dispatch run` as the foreground job in the
  working-tree root, passing the kind, the task file, the handle, the mandate
  and the three parameters, and nothing else.
- Kind admission is gone from the driver, the tree verbs, root scaffolding and
  the finish sentinel. A refused launch leaves its leaf live and the loop
  stopped, and rerunning `grove` continues.
- Every Grove test that launches a session goes through the real front and its
  compiled worker, with the policy in a temporary HOME. There is no fake
  `harness-dispatch`.
- The Grove launch boundary cases hold, the controlling-terminal ones included.
  A `config.kdl` and a `.grove.kdl` on disk, valid, invalid or tracked, change
  nothing about a launch.
- The `grove-loop`, `grove-llm` and overview books are valid for what changed.
- `bash scripts/check.sh` passes.

## Notes

- This is the wide test migration. Most launching tests take their
  configuration from one shared fixture home, so start there.
- A test of what configuration does to a launch goes here, since it can no
  longer pass. A test of the configuration code itself stays until
  `grove-configuration-k13` deletes that code.
- After this leaf the `grove config` commands still exist and describe a
  configuration nothing launches from. That state is not released.

## Decisions (running log)

**The loop takes the dispatch path, and the session configuration is left
alone.** `grove_loop::run` takes the path `crates/grove/src/dispatch.rs` finds
in place of its `TemplateSource`. The driver builds one `harness-dispatch run`
argv with each value joined to its flag in one word, so a prompt or a path that
begins with a dash is still a value. A value dispatch cannot take is dispatch's
to refuse; Grove converts nothing. `session_config.rs` and its exports are
untouched: `grove config show` still uses them, and `grove-configuration-k13`
deletes both.

**Dispatch is located before the loop starts.** A missing sibling is reported
before anything is scaffolded or launched, and after the lease, so the same
working tree still has one driver.

**A refused launch is reported as any session that ends without a signal.** The
driver adds the kind, the handle, a pointer to the diagnostic above and that
rerunning continues, and only when the status is a failure. It reads nothing
dispatch wrote.

**`--help` changed with the launch.** The Grove example in `harness-dispatch
--help` and `run --help` described a `config.kdl` command definition, which is
false once this leaf lands. It now shows the invocation the driver makes, and
the launch-boundary suite holds its flags to the ones that reach `select`. The
dispatch README, the configure-grove skill and the usage guide still quote the
old definition: they are `current-state-documents-k15`'s, and the same test
holds any Grove invocation they come to quote.

**No direct launch is left to compare against.** The launch-boundary cases that
measured a dispatched session against a direct one now state the expectation
outright. The prompt is compared with `grove_loop::compose`'s own output, byte
for byte. The harness's signal state is compared with the exact sets Grove's
spawn gives its child: the entry's one ignored and one blocked signal, and
nothing else.

**The native-data case runs in a secondary workspace.** In a single workspace
`worktree` and `repo` are one path, and swapping them would pass.

**A session with no run drops its variable.** Every Grove session now has a run
identity, so the stale-creator case's finishing session unsets
`HARNESS_DISPATCH_RUN_ID` before it works, as a harness started by hand in the
working tree has none.

**The old-configuration states were measured while the configuration code
exists.** The fixture's valid pair, its invalid pair and its tracked delta were
each run through `grove config show`: the first resolved the delta's other
command, the second refused as a syntax error, the third refused as tracked.
After `grove-configuration-k13` nothing can check that again, and the case only
needs the files to be on disk.

**Tests of what configuration does to a launch are deleted, not ported.**
`modular_*`, the template-slot and parameter cases, the admission refusals and
`session_kind_presence.rs` went. What replaced admission is asserted from both
sides: `crates/grove-llm/tests/no_kind_admission.rs` writes any kind with every
leaf-writing verb under a HOME with no policy, and the launch-boundary suite
shows the refusal arriving at launch with the leaf live. The shared fixture
home is now empty, so every verb test runs with no policy installed.

**The viewer's launch test keeps one failing launch.** Its helper passes a
dispatch path that does not exist for the case that needs the launch itself to
fail. That is a missing executable, not a stand-in for one.

**Two user-facing strings followed the rule.** `grove-llm`'s `--kind` help said
a kind no launch template declares is refused later, and `Kind`'s documentation
called the label a configuration key. Both now say the kind is what a launch is
selected by, and that a refused launch leaves the leaf live. The methodology's
own text about kinds is `current-state-documents-k15`'s.

**`session_config.rs` keeps its stale comments.** Its doc comments still name
the loop as their caller. Editing a file the next leaf deletes would cost a
second rebase of the book chapter that reproduces it, so the chapter carries one
note saying nothing launches from the module any more.

**The books were rebased by script, then read.** Five sources moved. Each
fragment's range and bytes were regenerated from the current source by mapping
its old first line, with an explicit boundary wherever that line had changed,
and the manifest and the derived index tables followed. Prose was then rewritten
by hand where it described a configuration read, admission or a template. The
measured figures those chapters carry (mutation tables, comment-line counts,
test counts) were not re-measured: several were already out of step with the
source before this leaf, and none is checked.

**The leaf decomposed at the `grove-llm` book.** That book stated three orders,
and the presence rule was the second. Its growing chapter, its fragments and its
index now say what the code does and the book validates, but its orientation and
assembly chapters, its early-use ledger and its structure specification still
say three. Making them say two is a rewrite of the book's thesis and not a
patch, so it is the second child.

**An unrelated flake was cut as a leaf.** One loop unit test failed once in
eleven full runs, at a non-blocking `flock`, by a mechanism the lease tests
already document and isolate against. It is `fork-sensitive-pin-test-k18`.

**A fixture race was fixed where it surfaced.** The first full check failed on
the terminal interrupt case: its policy wrote the worker's PID with a create and
then a write, and the case interrupts as soon as the file exists, so a worker
stopped between the two left an empty file. The fixture now renames the file
into place whole. The race predates this leaf and the case is this leaf's, so it
was repaired here and not cut.

**No review was cut for `dispatch-launch-k19`, and the in-session reviewer was
not spent.** The production change is one invocation built and four checks
deleted, and each case the specification's two launch-boundary rows name has a
test that was run. Whether the node earns a review is `two-orders-k20`'s to
judge when it closes it.

**The node closed with a review cut.** `two-orders-k20` rewrote the `grove-llm`
book to two orders and judged the cutover load-bearing enough for an
adversarial read. `lifecycle-launch-k21` is that review, placed ahead of
`grove-configuration-k13`.
