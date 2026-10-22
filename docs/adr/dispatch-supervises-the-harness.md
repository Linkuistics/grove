# Ravel supervises the harness

Grove launches the independently installed `ravel start-session` as a job.
Ravel selects the command through the owner's policy and supervises its process
group. It restores the terminal and reaps the group. Then it returns the opaque
argument that `ravel end-session RESULT` published. The caller supplies the
prompt and the ending instructions. Ravel injects none and gives the result no
meaning.

[Ravel's executable contract](https://github.com/Linkuistics/Ravel/blob/v1.1.0/docs/specs/session-execution.md)
owns selection, ephemeral transport, cleanup and confinement. Grove consumes
only the public CLI and the stdout result. It never consumes a private worker or
report protocol, a run record or a persistent store.

Supervision puts the end of the harness in one process for every caller. The
cost is one process for the lifetime of the harness, and nested job control.
Grove keeps no runner, and no shared Rust dependency crosses the boundary
between the independently released packages. The
[job decision](the-launched-child-is-a-job.md) records how Grove makes `ravel`
a job.

**Ravel alone gives the cleanup assurance.** A successful Ravel status with an
exact result is the only evidence of cleanup that Grove uses. Grove sends no
KILL to `ravel`, sets no bound on its wait and probes no process group after the
reap. So a stuck `ravel` holds its caller until it exits. An uncatchable death
of `ravel` can orphan its managed job, and Grove then gets no supported result
and stops. The caller contract states these
[limits](../specs/harness-selection-and-execution.md#job-limits).

## Grove's side of the boundary

Grove selects the leaf, composes the unchanged prompt and gives the session its
launch directory. It also holds epoch admission. Grove owns the meanings of
`grove-continue` and `grove-force-stop`, and the teardown record. It interprets
them only after it reaps `ravel` and invalidates the epoch.

The caller contract states the order of that interpretation, the publication
rule of `grove run` and the ending commands of the prompt. Its sections are
[Lifecycle interpretation](../specs/harness-selection-and-execution.md#grove-integration),
[Standalone interpretation](../specs/harness-selection-and-execution.md#confinement)
and
[Ending and returned result](../specs/harness-selection-and-execution.md#run-ending).

## Resolution and costs

Grove resolves an independently installed `ravel` on PATH and probes its
version. The caller contract states the procedure in
[Independent packages](../specs/harness-selection-and-execution.md#package-boundary).
Homebrew presence alone proves no runtime compatibility.

The floor is 1.1.0 because Grove relies on Ravel's failure paths for its cleanup
assurance.
[Ravel 1.1.0](https://github.com/Linkuistics/Ravel/blob/v1.1.0/CHANGELOG.md)
changes the behaviour of five of them. In one, Ravel stops a session when it
cannot check that it reaped its policy worker. Grove's own survivor check of the
group of `ravel` used to catch that case.

This trusts the program that the owner's PATH selects, even during the probe.
Version output is not authentication. Canonicalisation fixes the symlink
selection. It does not fix an inode against replacement by the owner.

The owner's policy selects the command, and Grove's caller assets own the
provider receipts
([policy ownership](harness-selection-is-owned-by-policy.md),
[caller provenance](a-review-carries-its-creator-reference.md)).

## Considered options

- **Find Ravel beside Grove, or bind versions or Cellar paths.** Rejected,
  because independently released packages need not share a prefix or a
  version. Reopen only if Ravel becomes part of Grove's package.
- **Search relative PATH entries, or fall through incompatible executables.**
  Rejected, because these add trust in the current directory or silently
  substitute an installation. Reopen when the owner explicitly requires those
  resolution semantics.
- **Keep an exec handoff, or supervise only optionally.** Rejected, because
  that splits the ownership of the end of the harness. Each caller would then
  have to supervise. Reopen if supervision prevents a required capability of a
  harness.
- **Keep the cleanup and channel mechanics in Grove.** Rejected, because Ravel
  serves other callers. Grove supplies the result meanings and teardown. Ravel
  manages the process and returns the argument without interpreting it.
- **Infer teardown from a missing `.grove/`.** Rejected, because an absence
  attests no confirmation or disposition. Only this driver's teardown record
  does.
- **Let teardown override an explicit force-stop.** Rejected, because a caller
  that requests a stop would receive a false completion. Only an exact result
  from a successful Ravel invocation can override teardown. Unsupported bytes
  and failed supervisors keep no such authority. Force-stop does not undo
  teardown.
- **Use the exit status alone as completion.** Rejected, because a natural exit
  of the child, even with 0, supplies no acknowledgement from the caller. Grove
  needs a supported result after cleanup.
- **Use a shared runner crate across projects.** Rejected for the independent
  CLI boundary. Ravel publishes no runner library.
- **Keep a local copy of the runner in Grove.** Rejected (owner, 2026-10-21).
  Ravel copied the runner from Grove commit `639b9d6`, and the two copies
  diverged. Grove's copy lacked Ravel's corrections of its failure paths and its
  native entry. It also repeated cleanup that Ravel already guarantees. Reopen
  only if Ravel stops supervising its own job.
- **Check the group of `ravel` after the reap, and kill a survivor.** Rejected
  (owner, 2026-10-22). The harness runs in a group that Ravel owns, so a check
  of the group of `ravel` never covered it. A KILL from Grove would also cut
  Ravel's own cleanup short and orphan its managed job. Reopen if Ravel stops
  checking the absence of its groups before it returns a result.
- **Bound the wait with a KILL after a grace.** Rejected (owner, 2026-10-22). A
  KILL to `ravel` cuts its cleanup short and orphans its managed job. That is the
  outcome that the bound was there to prevent. Ravel bounds its own escalation.
  Grove waits until `ravel` exits, however long that takes.
