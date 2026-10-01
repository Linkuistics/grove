# package-json-autoloading-k66

**Integrates:** package-json-autoloading-k65

## Goal

Triage and integrate k65's two findings on package.json autoloading before
`dispatch-documentation-k41` changes the cited surfaces.

## Context

Read k65's findings and all eleven doubt dispositions. It reviews producer
commit `37b1a0f4023d4dc8801914025952b11535b7ef4f` and leaves the implementation
unchanged. Load the integrate-review-impl skill for this session's procedure.

- F1: the oversized firing arm accepts any post-hello close/crash as firing.
  Distinguish a timeout from unrelated termination and establish a healthy
  baseline for the same probe/plain entry/directory. Reconsider the 4 GiB
  fixture's cost against the causal evidence it actually retains. If a cheaper
  control replaces it, reconcile the spec tables and runtime evidence.
- F2: the public nearest-manifest rule lacks the successfully parsed regular
  file qualification. A malformed nearest manifest lets an outer imports map
  answer. Reconcile the spec, README and changelog with the existing source
  evidence and retain meaningful command-seam coverage of malformed, nameless
  and no-imports manifests as appropriate.

The review found no new runtime authority breach in the examined alias or
TMPDIR paths. The alias does require the admitted entry's package map. k64's
human-accepted single dotenv control and root exception remain settled. Do not
invent another generic resolver or turn the package switch off merely to avoid
triaging these findings.

The review's scratch observations are recorded in k65; their source script and
output remain at `/tmp/grove-review-k65/` on this host, for convenience only.
The task records the fixtures, outputs, pinned source citations and digests;
it must remain understandable if those temporary files disappear.

## Done when

- Each finding has an explicit, evidence-backed disposition.
- Accepted findings are fixed and the affected claims agree across their
  public surfaces, tables and summaries.
- Meaningful regression/mutation checks establish that any changed firing
  control rejects unrelated probe termination and still detects the intended
  manifest effect.
- Required verification runs through the repository Taskfile. Follow the
  project's release-smoke requirement if the worker or release layout changes.
- Durable decisions are reconciled, this leaf is retired and the changes are
  committed through jj before signalling the next session.

## Notes

All implementation, test and documentation fixes belong here. The preceding
review ran no test, build, lint or format command, so its historical check and
release-smoke references are not fresh verification of this integration.

## Decisions (running log)

**The findings were read from k65's commit, `2067cb434d72`, and graded
against the files.** The graph does not cover them: the coverage hook reports
the spec, the runtime evidence and the changelog as excluded, as k65 found for
the dispatch tests. Everything below is from the source and from runs.

**F1 is a real issue, and it reproduces.** The arm asserts
`driven.report.is_none()`, which a closed channel satisfies as well as a
timeout. Seen at entry, with the working copy empty: a stand-in at the
`unmoved` probe's path that is the real probe unless TMPDIR's `package.json`
is a link or past 1 MiB, and otherwise says the probe's hello, reads the
request and exits 1 without touching the file. The test as k62 left it passed,
in 3.5 s. So the oversized arm counted a death it had nothing to do with.

**F2 is a contract stated unclearly, and it reproduces.** Through the
checkout's front, front `59a71a9c6d63…` and worker `c840885f5a45…`, for an
entry one directory below a named `package.json` whose map answers `#x`. The
nearest file answered when it was a nameless or a named map, or a link to
one. It ended the search, refusing the import, as `{}`, as a named file with
no `imports`, with `imports` of the wrong type, with a map lacking the name,
and as an empty file. It was passed over, and the outer map answered, when it
did not parse, when its JSON was an array, `null` or a string, when it was
unreadable, a directory, a FIFO or a dangling link. Node 26.10.0 on the same
layouts refuses the unparsable file and the array with
`ERR_INVALID_PACKAGE_CONFIG`. So "as in Node" holds for a well-formed file
only, and the public rule needs the qualification. No party gains reach: every
file in question is at or above an admitted entry.

**A link to a FIFO replaces the 4 GiB file.** k62 saw that a FIFO named
`package.json` is not opened, which is right. A `package.json` that is a link
to a FIFO is opened. Driven directly on macOS arm64, and on Linux arm64 in a
container with builds cross-compiled from this source at the pinned runtime
(`9ab3970a1966…`): under the `unmoved` probe a reader appears on the FIFO
within about 10 ms, and the probe never reports, whether no writer comes, once
on each host, or one opens and writes nothing, twenty times of twenty on each.
The shipped worker, started in the same directory, never opens it and loads
the policy in about 10 ms.
That is the causal evidence the oversized arm lacked: the test opens the FIFO
for writing without blocking, which fails until a reader has it open, so a
success is the read seen. It costs no memory. It is also the wider control.
The resolver finds a link's kind before it parses, so anything that would
read a regular `package.json` there opens this one first, at any size, where
the 4 GiB file caught only a reader that hit this version's `u32` length.
The 4 GiB readings stay in the runtime evidence as k62's measurements.

**The write end is held, never fed.** Opening and closing it at once left the
macOS probe blocked and let the Linux one through, so a release would race.
Held open with nothing written, the probe stays silent on both. So the firing
arm asserts two things in one run: a reader came, and the probe was still
running and silent when its time ran out. A probe that dies gives neither.

**The case is its own test.** The oversized arm sat at the end of the TMPDIR
case, so a mutant that failed that case's first arm never reached it, and k62
ran it by hand. The new case is
`hostile::a_package_json_that_never_yields_above_the_workers_start_directory_stays_unopened_and_stalls_the_unmoved_probe`.
It has the baseline F1 asked for: the same probe, entry and directory beside
a `package.json` it can read, which loads. `watching_for_a_reader` and
`support::mkfifo` are the two helpers, and `tests/context.rs` now uses the
shared `mkfifo`.

**Each control was seen to fail, and the old one to pass what it should not
have.** The stand-in above failed the new case at "never opened". A second
stand-in, which opens the file without blocking before it exits, failed it at
"was not waiting". With the move removed from `main.ts` and the worker and
probes rebuilt, the new case failed at its front arm with `selection_timeout`,
exit 124, at the five-second bound, in the suite and not by hand, and the
other TMPDIR case failed at its front arm. With the switch off in
`dispatch.sh`, the new case failed at its firing arm, where the probe opened
nothing and loaded the policy, and the three `authority` package cases
failed, the new one at its first control. Both files went back to
`030406f5f9ac…` and `323c445ac685…`, the test files' digests were the same
before and after, and the worker rebuilt byte for byte as `c840885f5a45…`,
build `721aab0848f6…`.

**F2's regression is a command-seam case.**
`authority::the_nearest_package_json_that_reads_as_one_is_a_modules_own_and_one_that_does_not_is_passed_over`
holds nine layouts: no nearer file, a nameless map, a link to one, three that
end the search, and three that are passed over. Its own controls are the
first two: the outer map answers with nothing nearer, and a nearer map
answers when it reads. The empty file, the unreadable file, the FIFO, the
dangling link and the other JSON values are in the runtime evidence as seen
by hand. An unreadable file would not hold under root, and the rest restate
the same rule.

**The public surfaces say one thing.** The spec's `#policy-authority` has the
qualified nearest-file rule and says a `package.json` above the start
directory is not opened. Its seam row and firing-configuration row name the
new fixture. The README has a paragraph on the difference from Node, the
changelog entry carries both changes, and the ADR's two sentences are edited
in place. The existing `authority` case's comment has the qualifier. The
runtime evidence has two new readings, a table of what was changed and what
failed, and two more limits. Left alone, and still true: the comment above
`SHIPPED_SWITCHES` and the one at the move in `main.ts`, which say what was
seen on 2026-10-01 and what the probe still does. Both files are in the
worker's source digest, so an edit there would have changed the build for no
change in behavior.

**One sentence of k62's evidence was wrong, and is corrected.** "A file that
is not a regular file is not opened" does not hold for a link: the resolver
opens it to learn what it is. A link to a FIFO beside an admitted entry
therefore stalls a selection to its bound, which was seen. That is in code an
entry admits, like the rest of F2, and whoever can plant it can plant a
module. It is in the runtime evidence and is no new class.

**No ADR, no reviewer, no further review leaf.** Nothing here is a new
decision: the worker ADR's evidence sentences were corrected in place. The
in-session reviewer was not spent. Both fixes are held by executable cases
whose controls were seen to fail in each direction. The prose goes to
`dispatch-documentation-k41`, which consolidates these paragraphs and cuts the
documentation-acceptance review. No shipped source changed, so the worker and
the release layout are k62's and `task release:smoke` was not rerun.

**`task check` passes, all twelve checks.** It ran once to its end, after the
last edit to anything it reads, on working-copy snapshot `e621c4be8811…`,
which was the same afterwards. It rebuilt the worker and the probes as build
`721aab0848f6…`, the worker byte for byte `c840885f5a45…`, and both new cases
passed in it. An earlier run was stopped by hand at its conformance step, to
reflow one ADR sentence, and is no result. `task release:smoke` was not run,
since no shipped source, worker or layout changed. The comment edit in the
existing `authority` case came after the two rebuild mutations and before
this check. The two stand-in runs were repeated on the final test source.
What changed after the check is this entry.
