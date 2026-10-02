# direct-dispatch-k6

**Integrates:** direct-dispatch-k5


## Goal

Triage the findings in `direct-dispatch-k5` against the settled requirements and
the design delivered by `direct-dispatch-k3`. Apply the findings that hold, with
the smallest coherent design changes, before planning cuts implementation work.



## Context

- Read `direct-dispatch-k5` from its committed review. Its findings are evidence
  to assess, not a list this charter requires accepting.
- The root brief's requirements and simplicity instruction bind. This is design
  integration: the specifications, ADR set, visual views and planning handoff
  must agree on what is to be built.
- The producer's log in `direct-dispatch-k3` records the alternatives it
  considered. Preserve working mechanisms outside the accepted correction.

## Done when

- Every finding has a recorded disposition with supporting evidence.
- Accepted corrections are reflected coherently in the design and its planning
  handoff; no rejected finding remains a hidden implementation obligation.
- The resulting design is concrete enough for `direct-dispatch-k4` to cut its
  increments, and relevant document checks have been performed.

## Notes

Dispositions, graded against the source and not against the review's wording.
The log below holds the evidence for each.

| Finding | Disposition | What changed |
|---|---|---|
| F1, dispatch selects standalone work but does not launch it | Wording unclear, and a trade-off accepted visibly. Its remedy is not built | The spec and the ADR say what each launch does; the ADR records the launcher handoff as a rejected option; cancellation during a standalone selection gains a statement and a test case |
| F2, global settings discard per-kind control | A trade-off accepted visibly. Its remedy is not built | The spec says settings are the same for every kind and what that gives up; the ADR records per-kind settings as a rejected option |
| F3, the confined handoff leaves a second executable lookup | A real issue, fixed | A confined launch takes the resolved executable and looks nothing up; two confinement cases and one runner case are added |

The review's ruling 6 also named one sentence the new contract makes false,
without raising it as a finding. It is corrected.

**For the human.** F1 and F2 each rest on how a requirement of yours is read,
and I kept the design's reading because the other one adds mechanism. Both are
cheap to reverse and add nothing an owner would later have to give up. If
requirement 1 means that harness-dispatch itself must be the process that
launches a confined `grove run`, or requirement 8 means settings must differ by
kind, say so before the implementation leaves run.

## Decisions (running log)

**F3 is real and is fixed (2026-10-02).** Reproduced from the source. The
runner's confinement resolves the program again
(`crates/keyed-launch/src/confinement.rs:59`, `:88`): a path with a separator is
canonicalised against Grove's own cwd, and a name takes the first regular file
on PATH with no execute check. Dispatch resolves against the caller's cwd and
skips unexecutable files (`crates/harness-dispatch/src/program.rs:134`, `:155`).
`grove run` gives its child the staged directory as cwd and does not change its
own (`crates/grove/src/standalone.rs:221`), so a staged `./tool` and a shadowed
PATH name each reach a file inspection did not report. The fix: Grove hands the
runner the reported `executable`, and a confined launch takes an absolute
program path, refuses any other and searches no PATH. That deletes the second
lookup, which has no caller left once the templates go. The harness's `argv[0]`
under confinement is the resolved path, as it is today.

**F3's test cases are the two that can be built (2026-10-02).** An unexecutable
file that shadows the program earlier on PATH, and a relative PATH entry that
holds a file of that name under Grove's own directory. Each fails under today's
second lookup and passes when the runner uses the reported file. The review's
*staged relative program* cannot be built as it describes: an input is staged
without execute permission (`standalone.rs:109`), so dispatch refuses it before
any launch. The relative PATH entry shows the same cwd disagreement. An empty
PATH entry adds nothing to it at this seam.

**F1's remedy is not built (2026-10-02).** The finding is right that the spec
said Grove launches every session through dispatch and then described a path on
which dispatch launches nothing. That wording is fixed. The remedy, dispatch
launching through the confinement boundary, is what `direct-dispatch-k3`
weighed and rejected, and its two reasons hold in the source: the sandbox
command names the resolved program twice, as a read grant and as the command
(`confinement.rs:59`, `:138`, `:197`), so a launcher prefix has to be a Grove
helper executable; and the runner scrubs the child's environment
(`standalone.rs:207`), so that scrub would have to move behind dispatch for
selection to keep the owner's grants. That is a dispatch flag, a Grove
subcommand and a split runner entry, against a JSON object. The review names no
behaviour the difference costs: it says itself that the missing run record is
not a requirement. `plan-k1` left the how to design, and requirement 10, the
one written for this case, says `grove run` *selects* through the policy by a
caller-neutral capability, which inspection is. Requirement 14 is served better
by it: reporting a command says nothing about how it runs. The brief's rule
decides between two designs that meet a requirement.

**F1's cancellation case is taken (2026-10-02).** The review asked for owner
grants, refusal and cancellation on the standalone path. The first two were
already cases. A selection can now run for minutes, and the design did not say
what a signal during it does. It needs no mechanism: Grove runs the inspection
in its own process group, so a signal to the group reaches both, and inspection
cancels as its contract already says.

**F2's remedy is not built (2026-10-02).** The difference is real: two Grove
command definitions can carry different dispatch flags today, and one settings
file cannot. Nothing in the human's words asks for it. Requirement 8 was
derived in `plan-k1` from *what an owner tunes about a launch is set on
dispatch's side*, and the pinned configuration passes no dispatch flag at all.
The remedy is a table of kinds in the tool, which requirement 5 removed. The
review's own counterexample costs little: each value is a ceiling, so one that
admits a deciding agent also admits a table lookup, and what is lost is a
shorter failure bound for a policy that hangs. A record directory per kind
would split the store a run lookup reads. Adding a per-kind key later breaks
nobody. The spec already says how the record commands find the store: every
command reads the settings file.

**The check passes on the edited documents (2026-10-02).** `task check`, all
twelve principal checks, run after the last document edit, with the working
copy the same commit before and after. It establishes that the documents and
the tests that read them agree. Nothing here is implemented, so it says nothing
about the behaviour the design describes.

**No reviewer was spent and no redesign was cut (2026-10-02).** The one fix is
a repair to how a value is consumed, which the two added confinement cases
hold. Nothing here rethinks the design.
