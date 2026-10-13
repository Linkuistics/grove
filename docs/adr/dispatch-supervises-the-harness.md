# Ravel supervises the harness

Grove launches the independently installed `ravel start-session` as a job.
Ravel selects the command through the owner's policy, supervises its process
group, restores the terminal and reaps it before returning the opaque argument
published by `ravel end-session RESULT`. The caller supplies the prompt and
ending instructions; Ravel injects none and assigns no meaning to the result.
[Ravel's executable contract](https://github.com/Linkuistics/Ravel/blob/v0.1.0/docs/specs/session-execution.md)
owns selection, ephemeral transport, cleanup and confinement. Grove consumes
only the public CLI and stdout result, never a private worker/report protocol,
run record or persistent store.

Supervision puts the harness's end in one process for every caller, at the cost
of a process for the harness's lifetime and nested job control. Grove's local
`keyed-launch` remains its domain-free runner; Ravel has its own runner. No
shared Rust dependency crosses the independently released package boundary.

## Grove's side of the boundary

Grove selects the leaf, composes the unchanged prompt, gives the session its
launch directory and holds epoch admission. After Ravel returns and is reaped,
Grove invalidates the epoch and interprets exactly `grove-continue` and
`grove-force-stop`. Driver interruption wins first. A successful Ravel
force-stop wins over teardown and exits 1 without completion; it leaves
remaining live work resumable. Teardown otherwise retains its authority over
status, results and surviving groups. Continuation requires successful status
and a gone group. Missing/unsupported results stop without continuation.
Standalone work currently requires continuation before publishing validated outputs.
Session output does not contaminate stdout's held result.

The prompt gives exact canonical continue and force-stop commands. A finish records teardown
with `grove-llm record-teardown` before that command. Ravel discovers enclosing
context; no session identifier is an ending argument. Existing sessions launched
by the original installed-v22 driver keep their own `harness-dispatch exit`
command and admission/teardown authority until that driver ends.

## Resolution and costs

Grove chooses the first executable regular `ravel` in an absolute PATH directory,
canonicalizes it once and probes that path with `--version`. Empty/relative PATH
entries add no cwd fallback. Stable `ravel 0.1.x`, starting at 0.1.0, is the
supported CLI line; other minors/majors, prereleases, malformed/unsuccessful
probes refuse with an upgrade remedy. Refusal never falls through to another
installation. Homebrew presence alone proves no runtime compatibility.

This trusts the owner's selected PATH program even during probing. Version output
is not authentication; canonicalization fixes symlink selection, not an inode
against owner replacement. Launch and confinement reuse the selected path and
Ravel's documented installed layout. Version probes and launches scrub original
and new session authority, including inherited caller provider evidence.

Grove makes Ravel a job and Ravel makes the harness one; each restores the
terminal ([the launched child is a job](the-launched-child-is-a-job.md)). An
uncatchable supervisor death can orphan its harness. Grove gets no supported
result, stops and reclaims a terminal delegated by its signalled child. Processes
escaping a managed group are outside a process-group cleanup guarantee.
The caller names a sandbox-writable `--exit-dir`; it reaches no selection policy.
Grove reads the public result after reap, not an ending file. Selection and
provider receipts remain separate
([policy ownership](harness-selection-is-owned-by-policy.md),
[caller provenance](a-review-carries-its-creator-reference.md)).

## Considered options

- **Find Ravel beside Grove or bind versions/Cellar paths.** Rejected because
  independently released packages need not share a prefix or version. Reopen
  only if Ravel becomes part of Grove's package.
- **Search relative PATH entries or fall through incompatible executables.**
  Rejected because those add cwd trust or silently substitute an installation.
  Reopen with an explicit owner requirement for those resolution semantics.
- **Keep an exec handoff or supervise only optionally.** Rejected because that
  splits ownership of the harness's end and requires each caller to supervise.
  Reopen if supervision prevents a required harness capability.
- **Keep cleanup/channel mechanics in Grove.** Rejected because Ravel serves
  other callers. Grove supplies result meanings and teardown; Ravel manages the
  process and returns the argument without interpreting it.
- **Infer teardown from missing `.grove/`.** Rejected because absence attests
  no confirmation or disposition. Only this driver's teardown record does.
- **Let teardown override an explicit force-stop.** Rejected because a caller
  requesting a stop would receive false completion. Only an exact result from
  a successful Ravel invocation can override teardown; unsupported bytes and
  failed supervisors retain no such authority. Force-stop does not undo teardown.
- **Use exit status alone as completion.** Rejected because a natural child
  exit, even 0, supplies no caller acknowledgement. A supported result after
  cleanup is required.
- **Use a shared runner crate across projects.** Rejected for the independent
  CLI boundary. Each project keeps its local implementation and release checks.
