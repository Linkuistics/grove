# Leaving the loop
<!-- book-page id="leaving-the-loop" slice="admit-before-signal" order="6" -->
[Previous: Ending work](05-ending-work.md) | [Contents](README.md) | [Next: What order holds](07-what-order-holds.md)

<a id="admit-before-signal"></a>
## Two separate effects: teardown and dispatch exit

`finish-commit` deletes and commits `.grove/`. `record-teardown` records that
fact in the admitted launch directory. Neither operation ends the harness run.
The session ends through `harness-dispatch exit`, whose per-launch job channel
belongs to dispatch rather than to this binary. An ordinary leaf retires and
commits its work before exiting; a finish commits the deletion, records teardown,
and exits after any requested integration and release.

The order this chapter owns is admission before the teardown write. `run`
obtains a shared epoch guard for the ambient `GROVE_LAUNCH_DIR` and retains it
through dispatch to the handler. `cmd_record_teardown` requires that same
directory before it calls the verb. A replacement driver therefore cannot
invalidate an admitted operation midway through its write.

<a id="worked-complete"></a>
## Worked example: a finish records teardown, then exits

The following is a schematic command sequence, not a measured transcript:

```text
grove-llm finish-commit finish-k9
  -> .grove/ deleted and its deletion committed; change id on stderr
grove-llm record-teardown
  -> teardown record in the admitted launch directory; path on stderr
harness-dispatch exit
  -> dispatch ends its job and returns a checked receipt to Grove
```

The dispatch receipt accounts for how the harness ended. Grove separately checks
whether the launch directory contains the teardown record and `.grove/` is
absent. A receipt alone cannot establish that a finish deleted its tree, and a
teardown record alone cannot establish how the harness ended. The driver reads
those facts together after dispatch returns. The integration specification
`docs/specs/harness-selection-and-execution.md` owns this separation.

<a id="the-teardown"></a>
## `cmd_finish_commit`: a handle becomes a committed deletion

The handler resolves the worktree, parses the canonical finish handle, and passes
the workspace and handle to the lifecycle verb. It quotes the operator's input
in the error context and prints the returned change id. The lifecycle operation
owns the exclusive tree opening, revalidation of the live finish leaf, deletion,
and path-scoped commit; the handler cannot duplicate those checks.

<!-- fragment «handler-finish-commit» owner="admit-before-signal" source="crates/grove-llm/src/cli.rs" lines="413-430" parent="handlers-leaving" -->
````rust
fn cmd_finish_commit(finish_handle: &str) -> Result<()> {
    let worktree = worktree()?;
    // The argument arrives as text from a session's command line, so it is read
    // by the type that owns the grammar rather than compared as a string: a
    // handle that is not one is told *why*, and only a well-formed handle
    // reaches the verb.
    //
    // Parse the canonical handle before opening the tree. The contextual
    // refusal below quotes the command argument; the lifecycle operation
    // compares it with the handle read from the selected live finish leaf.
    let finish = Handle::parse(finish_handle)?;
    let workspace = Workspace::resolve(&worktree).context("cannot commit the finished grove")?;
    let commit = verbs::finish_commit(&workspace, &finish)
        .with_context(|| format!("`grove-llm finish-commit {finish_handle}`"))?;
    eprintln!("finish-commit {finish}: committed as {}", commit.change_id);
    Ok(())
}

````
<!-- /fragment -->

A malformed handle fails in `Handle::parse`; a well-formed handle that does not
name the live finish leaf fails in the lifecycle operation. `finish-commit`
commits only `.grove/`, so unrelated working-copy changes remain uncommitted.
A failed deletion or commit reports the VCS remedy rather than repairing itself.
The help states those guarantees and preserves the distinction between tree/VCS
facts and the finish session's required human confirmation.

<!-- fragment «verbs-finish-commit-help» owner="admit-before-signal" source="crates/grove-llm/src/cli.rs" lines="249-272" parent="verbs-leaving" -->
````rust
    /// Revalidate the live driver-owned finish leaf and the absence of ordinary
    /// work under the exclusive tree lock, then delete and commit only
    /// `.grove/`. This helper enforces tree and VCS facts; it does not infer or
    /// automate the finish session's required human confirmation.
    ///
    /// Teardown is a plain deletion followed by a path-scoped `jj commit`, and
    /// grove implements no transaction around it: no witness, no manifest, no
    /// rollback proof, no quarantine, no recovery path. Jujutsu snapshots the
    /// working copy before every command and its operation log is the
    /// transaction record, so the version control system already owns every
    /// guarantee grove used to hand-build.
    ///
    /// A failure therefore stops with a message rather than repairing itself,
    /// and the message names the command that puts the tree back — `jj restore
    /// .grove` if the deletion is what failed, `jj undo` if the commit is. Once
    /// the tree is back, rerun this same command with the same handle. Grove
    /// never resets, rebases, or rewrites history on your behalf.
    ///
    /// Only `.grove/` is committed: unrelated working-copy changes stay in the
    /// working copy, because the commit is scoped to that fileset.
    FinishCommit {
        /// Stable handle of the launched finish leaf, for example `finish-k42`.
        finish_handle: String,
    },
````
<!-- /fragment -->

<a id="the-order"></a>
## `cmd_record_teardown`: admit, check absence, record

The handler reads `GROVE_LAUNCH_DIR`, treats an empty value as no loop, and checks
a nonempty path against `SessionEpochGuard::require_launch_dir`. The shared guard
remains alive through the call. With no directory the handler avoids workspace
resolution and the verb returns `Recorded::NoLoop`; a manual command can report
the no-op even outside a jj workspace.

<!-- fragment «handler-record-teardown» owner="admit-before-signal" source="crates/grove-llm/src/cli.rs" lines="431-451" parent="handlers-leaving" -->
````rust
fn cmd_record_teardown(session_epoch: Option<&SessionEpochGuard>) -> Result<()> {
    let launch_dir = std::env::var_os("GROVE_LAUNCH_DIR")
        .filter(|value| !value.is_empty())
        .map(PathBuf::from);
    if let Some(launch_dir) = &launch_dir {
        session_epoch
            .context("record-teardown requires session epoch admission")?
            .require_launch_dir(launch_dir)?;
    }
    let worktree = if launch_dir.is_some() {
        worktree()?
    } else {
        PathBuf::new()
    };
    match verbs::record_teardown(&worktree, launch_dir.as_deref())? {
        verbs::Recorded::Wrote(path) => eprintln!("record-teardown: recorded {}", path.display()),
        verbs::Recorded::NoLoop => eprintln!("record-teardown: no GROVE_LAUNCH_DIR — not running under the loop driver; recorded nothing."),
    }
    Ok(())
}

````
<!-- /fragment -->

The verb delegates the filesystem effect to Grove's launch-directory module.
That module checks `.grove/` is absent before creating the teardown record. A
present tree or a failed record creation is an error. `Recorded::Wrote(path)`
becomes a stderr report; `Recorded::NoLoop` becomes the explicit no-loop message.
Both successful cases return without ending the session and print no stdout.

<!-- fragment «verbs-record-teardown-help» owner="admit-before-signal" source="crates/grove-llm/src/cli.rs" lines="273-275" parent="verbs-leaving" -->
````rust
    /// Record that this grove was torn down in the current launch directory.
    /// Run finish-commit first. Does not end the session; outside a loop it is a no-op.
    RecordTeardown,
````
<!-- /fragment -->

<a id="the-two-contracts"></a>
## The help and the exit contract

| Operation | Input | Effect | Adjacent failure |
|---|---|---|---|
| `finish-commit` | canonical live finish handle | delete and commit only `.grove/` | absent/untracked/symlinked root, ordinary work, wrong handle, deletion or commit failure |
| `record-teardown` | admitted ambient launch directory | absent-root check and teardown record | admission mismatch, tree still present, record creation failure |
| `harness-dispatch exit` | dispatch's ambient job context | request that dispatch end its harness job | owned by dispatch, outside this binary's corpus |

The source-order composites below reassemble both variants and both handlers.
They contribute no bytes beyond their explicitly inserted children.

<!-- fragment «verbs-leaving» owner="admit-before-signal" source="crates/grove-llm/src/cli.rs" lines="249-275" parent="source-command-surface" -->
<!-- insert «verbs-finish-commit-help» -->
<!-- insert «verbs-record-teardown-help» -->
<!-- /fragment -->

<!-- fragment «handlers-leaving» owner="admit-before-signal" source="crates/grove-llm/src/cli.rs" lines="413-451" parent="source-command-surface" -->
<!-- insert «handler-finish-commit» -->
<!-- insert «handler-record-teardown» -->
<!-- /fragment -->

The two orders are now visible: grammar validation before tree opening, and
epoch admission before recording teardown. Dispatch exit is the session's final
action; it is not a thirteenth Grove verb. The next chapter compares all twelve
Grove verbs and the checks that hold their boundaries.

[Previous: Ending work](05-ending-work.md) | [Contents](README.md) | [Next: What order holds](07-what-order-holds.md)
