# Leaving the loop
<!-- book-page id="leaving-the-loop" slice="admit-before-signal" order="6" -->
[Previous: Ending work](05-ending-work.md) | [Contents](README.md) | [Next: What order holds](07-what-order-holds.md)

<a id="admit-before-signal"></a>
## The channel is checked before it is written

Two verbs leave the loop, and they are the two that open no tree. `complete`
writes one word into the file the loop driver is watching for, and returns.
`finish-commit` revalidates a finished tree, deletes it, and commits the
deletion. Neither takes the shared or the exclusive opening the last four
chapters have been reading: `cmd_complete` resolves no working tree of its own,
and `cmd_finish_commit` resolves one only to hand it to a call that opens the
tree itself, because the teardown's subject is a tree that stops existing.
Under a driver the working tree is still resolved once per verb, by the
admission *The grammar and the openings* read, which every verb passes through
including these two.

The rule this chapter opens on is `complete`'s, and it is the third of the
three orders *Orientation* named: **the completion channel is resolved and
checked against the admitted epoch before it is written.** The check has to
precede the write because the write is what it protects against. A session
whose channel is not the one the live epoch admitted is a session signalling a
loop that did not launch it, and an answer arriving with the signal would
arrive too late — the driver acts on the file's appearance, not on a later
correction.

The premise is the [loop control channel](../../../CONTEXT.md#loop-control-channel):
the per-launch path the driver allocates, hands to the session in
`GROVE_SIGNAL_FILE`, and watches while its harness child runs. Its appearance
ends the session; its content is read only to tell a relaunch from a clean
finish. That path also names the active session epoch, which is what makes it
checkable at all — *The grammar and the openings* admitted this process against
that epoch before dispatch, and this is the chapter where the guard admission
returned is finally consulted. It is consulted by one verb, which is why `run`
passes it to one arm and to no other.


`finish-commit` parses a canonical handle before opening the tree. Leading
zeros, zero and overflow refuse at that boundary. Its contextual error quotes
the operator’s argument; subsequent layers compare with the current handle
read from the selected finish leaf.


The chapter owns three blocks of `cli.rs`: the two variants (248–289),
`CompleteArgs` (311–322), and the two handlers (438–483). It reads `complete`
first, because it is the carried session's own last command and the chapter's
order is its; then `finish-commit`, which no ordinary session runs; then the
two doc comments, as the catalogue of promises the handlers have been seen to
keep.

<a id="worked-complete"></a>
## Worked example: the signal, and the teardown

The session is the one *Orientation* carries, at its last command. It has
resolved its leaf, read the brief chain, done the work, added its review leaf,
retired its own leaf and taken the commit *Ending work*'s reminder asked for.
One step remains, and the reminder named it. Every line below is what the built
binary printed at the frozen corpus, with the scratch paths replaced by the
carried session's. The two invocations that turn on the epoch — the
signal itself, and the refusal that quotes two channels — were measured under a
live `grove` driver over a scratch Jujutsu workspace, because neither is
reachable without one. The rest were measured with no driver running.

```console
$ cd /work/atlas && grove-llm complete
grove complete: signalled; the loop will start the next task.
```

That one line is on stderr; stdout is empty and the exit status is `0`. Between
the prompt and the line, `cmd_complete` resolved the channel from
`GROVE_SIGNAL_FILE`, asked the guard `run` obtained whether that path is the one
the epoch admitted, and only then called `verbs::complete`, which wrote
`relaunch` and a newline into
`/work/atlas/.jj/grove/signal-3f9c2a7e5b1d4c8890aa61e0f27b4d13`. The verb then
returned. What happens next is not this binary's: the driver, which has been
watching that path for the life of the session, sees the file appear, applies
its escalation to the harness child it spawned, removes the channel, and
launches the next task with fresh context. Measured under a live driver, the
file existed with those bytes immediately after the verb exited and was gone
once the driver had acted.

The same verb has two other endings, and each takes a different change to
reach. Dropping the channel from the environment leaves the verb nothing to
signal, and it says so. Reaching the check this chapter is about takes more
than a wrong channel: it takes the live channel still in the environment and
`--signal-file` pointing somewhere else, because that is the only way the flag
and the epoch can disagree. A wrong channel supplied in the environment alone
never reaches this handler, since admission compares it with the epoch record
first and refuses there.

```console
$ env -u GROVE_SIGNAL_FILE grove-llm complete
grove complete: no GROVE_SIGNAL_FILE — not running under the loop driver; exit this session manually.

$ grove-llm complete --signal-file /work/atlas/.jj/grove/signal-22222222222222222222222222222222
Error: completion signal path does not match the admitted session epoch: expected /work/atlas/.jj/grove/signal-3f9c2a7e5b1d4c8890aa61e0f27b4d13, got /work/atlas/.jj/grove/signal-22222222222222222222222222222222
```

The second is the chapter's order, observed. The path named by `--signal-file`
was never created — measured by looking for it afterwards — because the guard
refused before `verbs::complete` was called, and the refusal quotes both paths
so the operator can see which of the two is the live one. The admission refusal named
above has a second form worth showing, because it is what a session meets once
the driver has gone: the channel still in the environment with no epoch record
beside it. `run` refuses that as *stale Grove session for grove-llm complete*,
naming the epoch file it could not open — *The grammar and the openings*'
refusal rather than this chapter's, and reached one call earlier.

The table is the verb's whole outcome space, sorted by what decides each, and
the column to read across is whether the channel was written. The first three
rows are the endings this handler renders; the last two are refusals that never
reach it. The argument vector alone does not predict the row — the first and
the third are the same vector, `grove-llm complete`, and differ only in what
the environment holds.

| Environment and flags | Decided by | Stream and text | Channel written | Exit |
|---|---|---|---|---|
| the live channel in `GROVE_SIGNAL_FILE` | the guard agrees | stderr: `signalled; the loop will start the next task` | yes, `relaunch` | `0` |
| the same, plus `--done` | the guard agrees | stderr: `signalled; the grove is finished — the loop will stop` | yes, `done` | `0` |
| no channel anywhere | `signal_channel` answers `None` | stderr: `no GROVE_SIGNAL_FILE … exit this session manually` | no | `0` |
| a channel the epoch did not admit | the guard refuses | stderr: `does not match the admitted session epoch` | no | `1` |
| a stale channel and no epoch record | admission, before dispatch | stderr: `stale Grove session for grove-llm complete` | no | `1` |

The fourth row is the one the chapter is named for, and the third is the one a
reader is most likely to meet, because it is what a manually run `grove-llm`
does. The exit statuses are the stream contract *Reading the tree* drew: no
loop to signal is an answer and exits `0`, and a channel that fails the check
is a refusal and exits `1`.

The session never runs `finish-commit`. That verb belongs to the finish session
the driver launches when the last live leaf has been retired, and the
[guide's account of the finish cycle](../../USAGE.md#usage-finish) is where its
place in the methodology is stated. The trace below is the carried grove at
that moment: every leaf terminal, including the node *Ending work* pruned, and
one live `finish` leaf the driver materialised with the next key after `k8`.

```text
/work/atlas/.grove/
├── _BRIEF.md
├── 01-DONE-impl--rate-limit-k3.md
├── 02-DONE-review-impl--rate-limit-k4.md
├── 03-k5/
│   ├── _cache.md
│   ├── 01-DONE-impl--warm-k6.md
│   ├── 02-ABANDONED-impl--evict-k7.md
│   └── 03-ABANDONED-impl--ttl-k8.md
└── 04-finish--finish-k9.md
```


The finish session first supplies a different canonical handle, then the
live finish leaf’s handle. The first refuses; the second performs teardown.


```console
$ grove-llm finish-commit other-k7
Error: `grove-llm finish-commit other-k7`

Caused by:
    requested finish handle other-k7 does not match the live finish leaf finish-k9

$ grove-llm finish-commit finish-k9
finish-commit finish-k9: committed as yuntyzvxnwwtzyqovpwvwmlqvwmrtmyz
```


Both arguments are syntactically canonical. `other-k7` does not match the
live finish leaf, so the lifecycle call refuses. `finish-k9` matches and the
commit uses the selected leaf’s handle. A zero-padded argument would instead
fail in `Handle::parse` before opening the tree. Workspace resolution precedes
that parsing, so a command outside a jj workspace can fail at the workspace
boundary first.


The accepted invocation printed one line on stderr and nothing on stdout,
deleted `.grove/`, and took one commit. What that commit contains is the
teardown's whole claim.

```console
$ jj log --no-graph -r @- -T 'change_id ++ "\n" ++ description'
yuntyzvxnwwtzyqovpwvwmlqvwmrtmyz
finish-k9: remove completed grove task tree

$ jj diff -r @- --summary
D .grove/01-DONE-impl--rate-limit-k3.md
D .grove/02-DONE-review-impl--rate-limit-k4.md
D .grove/03-k5/01-DONE-impl--warm-k6.md
D .grove/03-k5/02-ABANDONED-impl--evict-k7.md
D .grove/03-k5/03-ABANDONED-impl--ttl-k8.md
D .grove/03-k5/_cache.md
D .grove/04-finish--finish-k9.md
D .grove/_BRIEF.md

$ jj diff --summary
M crates/gateway/lib.rs
```

Three facts are visible there, and each is a promise the help makes. The
commit's message names the work item by the handle a name on disk wore, not by
the one the operator typed — `finish-k9`, from the tree, where the argument was
`finish-k0009`. Every path in the commit is under `.grove/`, and every one is a
deletion. And the edit outside the tree, made by the finish session while it
promoted material from the briefs, is still uncommitted afterwards: the commit
is scoped to a fileset, so it does not commit on the operator's behalf. Run
again on the same tree the verb refuses with *no Grove task tree at
/work/atlas/.grove* and names `jj op log` and `jj undo`, which is the remedy
belonging to the tool that owns the operation log.

<a id="the-order"></a>
## `cmd_complete`: resolve, ask, then write

The handler is two fragments, cut where the order the chapter is named for
ends and the rendering begins. The first is the order: `cmd_complete` receives
the guard `run` obtained, resolves the channel it is about to signal, and asks
the guard about that path before anything is written. Its comment states why the
channel is resolved in the handler at all, and the page checks that reason
against the seam it names.

<!-- fragment «handler-complete-admit» owner="admit-before-signal" source="crates/grove-llm/src/cli.rs" lines="456-463" parent="handlers-leaving" -->
````rust
fn cmd_complete(args: &CompleteArgs, session_epoch: Option<&SessionEpochGuard>) -> Result<()> {
    // **Asked before the write, which is why the channel is resolved here.** The
    // lease admits this session against the channel it is about to signal, and
    // an answer that came back with the signal would come back too late.
    let channel = verbs::signal_channel(args.signal_file.as_deref());
    if let Some(session_epoch) = session_epoch {
        session_epoch.require_signal_path(channel.as_deref())?;
    }
````
<!-- /fragment -->

`verbs::signal_channel` is the resolution, and it is public so that a caller
can perform it *before* the write. `verbs::complete` calls the same function on
its own way in, so the live path resolves the channel twice; what the public
function buys is not that the verb skips the work but that the answer exists
early enough to be checked. The loop's own documentation gives that as the
reason — the caller has to ask about the path before the write, so the answer
cannot be something the write returns.
The function takes `--signal-file` when it is present and otherwise reads
`GROVE_SIGNAL_FILE`, treating an empty value as no channel at all. That is not
a corner case in this repository, whose `.cargo/config.toml` force-clears the
variable to the empty string rather than unsetting it, so the empty value is
what every `cargo`-launched `grove-llm` here sees;
`an_empty_signal_environment_is_no_loop_context` in
`crates/grove-llm/tests/complete.rs` pins the reading and first asserts that
the empty value is really present when it runs. `an_explicit_channel_passes_through`
beside it pins something narrower than it looks: an explicit path comes back
unchanged. Because the ambient value is empty under cargo, no test in this
crate ever sets a non-empty environment channel against the flag, so the flag's
precedence is not held by a test. It was measured instead, under the driver, by
the refusal above — which names the flag's path as the one it got.

The `if let Some` is the whole of the check, and its shape carries a fact worth
stating. `session_epoch` is `Option<&SessionEpochGuard>` because admission
answers `None` for a manual command, so a `grove-llm complete --signal-file
<anywhere>` run outside a loop writes wherever it is pointed and nothing
objects. That is not a hole: outside a loop there is no epoch to disagree with,
and the thing being protected — a session signalling a loop that did not launch
it — cannot arise. Under a driver the guard is `Some`, and
`require_signal_path` compares the resolved channel with the one the epoch
recorded, refusing any other path including none. The `?` returns that refusal
from the handler before the next line runs.

The second fragment is the write and the two endings, and it is where
`Signalled` — named in *Orientation*'s import block and pending until now — is
finally matched on. `verbs::complete` takes the channel and the flag, writes
the disposition if there is a channel, and answers which of the two happened;
the handler turns each answer into one line of advice on stderr.

<!-- fragment «handler-complete-endings» owner="admit-before-signal" source="crates/grove-llm/src/cli.rs" lines="464-480" parent="handlers-leaving" -->
````rust
    match verbs::complete(channel.as_deref(), args.done)? {
        Signalled::Wrote(_) => {
            let tail = if args.done {
                "the grove is finished — the loop will stop"
            } else {
                "the loop will start the next task"
            };
            eprintln!("grove complete: signalled; {tail}.");
        }
        Signalled::NoLoop => eprintln!(
            "grove complete: no GROVE_SIGNAL_FILE — not running under the loop driver; \
             exit this session manually."
        ),
    }
    Ok(())
}

````
<!-- /fragment -->

`Signalled` has exactly two variants and the `match` is exhaustive over both.
`Wrote` carries the path that was written, and the handler ignores it — the
binding is `_` — because the session already knows where its channel is and the
line it needs is what to expect next, not where the file went. Which line that
is depends on `--done` and on nothing else: `the loop will start the next task`
by default, `the grove is finished — the loop will stop` with the flag. The
`tail` binding exists so the two endings share one `eprintln!` and differ only
in the clause the flag selects. `NoLoop` is the other variant, and its line
carries three clauses where the first carries one: there is no channel, this
session is not under the driver, and whoever started it has to end it. Neither
arm returns anything: both evaluate to `()`, and the `Ok(())` after the `match`
is the function's own, so every ending that reaches this handler exits `0`.
Nothing here writes to stdout, and nothing in this verb ever does.

What the two variants mean at the seam is worth stating once, because the
handler's rendering is all a reader of this module sees of it. `verbs::complete`
maps `--done` to a `Disposition` and hands the token to the runner's channel,
which frames it as a line; the driver reads it back and interprets anything
present that is not `done` as a relaunch, so a stale binary's older token still
relaunches rather than being mistaken for a clean finish. The round trip is
held by `relaunch_signal_is_read_back_as_relaunch` and
`done_signal_is_read_back_as_done` in this crate's `complete.rs`, both of which
allocate a real channel and read the token back the way the driver does, rather
than parsing a file the test wrote itself. The older token's reading has its
own test, `unrecognised_signal_content_is_treated_as_relaunch`, which plants a
stale binary's `complete` in the channel and requires a relaunch.
`no_channel_at_all_is_answered_rather_than_refused` holds the `NoLoop` value.
The three stderr wordings are held by the source and
by no test in either crate.

`CompleteArgs` is the whole of the verb's argument surface, and it is read here
because both of its fields have already been used: `done` chose the tail above,
and `signal_file` was the first thing `signal_channel` looked at. It is the one
argument struct in the module with no positional operand.

<!-- fragment «args-complete» owner="admit-before-signal" source="crates/grove-llm/src/cli.rs" lines="311-322" parent="source-command-surface" -->
````rust
#[derive(Parser)]
pub struct CompleteArgs {
    /// Finish the whole grove instead of relaunching: signal the loop to stop
    /// cleanly. Use as the **last** action of the Finish cycle. Without it
    /// (the per-task default) the loop relaunches with fresh context.
    #[arg(long)]
    pub done: bool,
    /// Relaunch-signal file the loop driver watches for. Default: `$GROVE_SIGNAL_FILE`.
    #[arg(long = "signal-file")]
    pub signal_file: Option<PathBuf>,
}

````
<!-- /fragment -->

Two flags, both optional, and the shapes are the ones their meanings need.
`done` is a `bool` under a bare `#[arg(long)]`, so it is a flag rather than a
value and its absence is the per-task default the help calls out; a session
that forgets it relaunches the loop, which is recoverable, where a session that
passed it by mistake would stop a grove that is not finished. `signal_file` is
an `Option<PathBuf>`, and it is a `PathBuf` for the reason *Ending work* gave
for `leaf_path`: unlike a slug or a kind, a path has no grammar type that owns
it, and what it names is settled by the filesystem rather than by a parser. Its attribute
spells the flag out where `done`'s does not, and the two forms agree here —
clap's derive kebab-cases a field name, so a bare `#[arg(long)]` would produce
the same `--signal-file`. What the explicit form buys is that the flag's
spelling is fixed in this file rather than following the field if it is ever
renamed. Its help names the
environment variable as the default rather than declaring one to clap, which is
what leaves the resolution to `signal_channel` and therefore leaves it ahead of
the check.

<a id="the-teardown"></a>
## `cmd_finish_commit`: the operator's spelling, quoted once

The other handler is longer in comment than in code, and the comment is about
which frame owns which spelling of the handle. The verb takes the finish leaf's
handle as text from a session's command line, reads it by the type that owns
the grammar, resolves the workspace, makes the call, and prints the change id
the call returned. Its comment argues for quoting the operator's own text here
and nowhere deeper, and the page checks that against the refusal the worked
example produced.

<!-- fragment «handler-finish-commit» owner="admit-before-signal" source="crates/grove-llm/src/cli.rs" lines="438-455" parent="handlers-leaving" -->
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


`Handle::parse` owns validation before any tree is opened, just as `Slug`
and `Kind` do for growth. It requires a valid slug and a canonical positive
decimal key fitting in `u32`. `finish-k0009` refuses rather than naming key 9;
`a_noncanonical_key_spelling_refuses_without_mutation` verifies that boundary
through the binary.


The `with_context` on the lifecycle call quotes the command argument when
the requested handle does not match the live finish leaf. A malformed handle
fails earlier in `Handle::parse`. The noncanonical-key regression checks
refusal and nonmutation; the successful teardown tests check that the committed
identity is the handle read from the selected leaf.


`Workspace::resolve` here is the second resolution of the working tree in one
verb, and the page states what the context line beside it does and does not
cover. `worktree()` has already resolved the tree through the same call and
returned its root; this line resolves that root again, because
`verbs::finish_commit` takes a `Workspace` where the tree verbs take an opening
their handler has already obtained. There are three shapes at this seam rather
than two: `root-init` hands over a vacancy, `complete` hands over neither, and
this verb hands over the workspace itself. A tree that is not jj-enabled is therefore refused by the
first call, not this one, and the measured refusal is the seam's *not a Jujutsu
working tree* with no *cannot commit the finished grove* above it. The context
is reachable only for a failure the first resolution did not have, which makes
it a safety net rather than the message a non-jj tree produces.

The call is where the teardown happens, and the book stops at it. What comes
back is a `Commit` carrying a change id — the identity that survives the
rewrites a commit id does not — and line 452 prints one line naming the
canonical handle and that id, on stderr, with stdout left empty. Everything
before the print is the loop's: revalidating the live leaf under the exclusive
lock, refusing when ordinary work has appeared, refusing an untracked tree or a
symlinked root, deleting through the store, and taking the path-scoped commit.
Seven tests in `finish_commit.rs` hold those refusals, and five of them assert
the same second thing beside the message — the tree left exactly as it was,
compared file by file — because a verb that refused after deleting would have
destroyed what it declined to act on. Two do not. The absent-tree case has no
tree to compare, and the symlinked-root case instead requires that the
directory the link pointed at still holds its files, which is the property that
matters when the operand names a tree that is not the verb's own. That
symlinked root is also the one refusal this chapter states from its test rather
than from its own measurement.

The composite that reassembles the two handlers, in source order, is stated
here.

<!-- fragment «handlers-leaving» owner="admit-before-signal" source="crates/grove-llm/src/cli.rs" lines="438-480" parent="source-command-surface" -->
<!-- insert «handler-finish-commit» -->
<!-- insert «handler-complete-admit» -->
<!-- insert «handler-complete-endings» -->
<!-- /fragment -->

<a id="the-two-contracts"></a>
## The two contracts, as `--help` states them

The two variants are these verbs' doc comments, which are the `--help` a
session reads, and the page reproduces them because they are corpus. Each is
read for the line that keeps its promise and the test that would catch its
breach. `FinishCommit`'s comment is almost entirely one
argument: that grove implements no transaction around the teardown and does not
need one.

<!-- fragment «verbs-finish-commit-help» owner="admit-before-signal" source="crates/grove-llm/src/cli.rs" lines="248-271" parent="verbs-leaving" -->
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

Read against the handler and the call, the comment divides three ways. What
this module keeps is one line: the change id printed on stderr. What the call
keeps is the revalidation, the scoped deletion and the scoped commit, together
with the precondition refusals — live work remaining, an untracked tree, an
absent tree, and the symlinked root read from its test below — each of which
leaves the tree exactly as it was.

What is stated from the source and was not measured is the pair of remedies the
third paragraph promises, because reaching either needs a failure injected
between the steps. `jj restore .grove` is emitted when the deletion itself
fails, and its own wording — *restore what was removed* — says the tree may be
part gone by then, so it is the one refusal in this verb that does not leave
the tree standing. `jj undo` for a failed commit comes from the version-control
seam's own refusal rather than from the loop, which is why the loop's context
line there names no jj command and does not need to.

What nothing keeps is the second sentence, and it says so itself: the verb
*does not infer or automate the finish session's required human confirmation*,
and there is no flag, prompt or environment variable anywhere in this path that
could. That is the same shape *Ending work* found in `leaf-prune`'s bold rule,
stated here as a fact about what the binary can check rather than as a rule the
page enforces.


The argument help gives the canonical example `finish-k42`. The parser
requires that same key spelling, including positivity and no leading zeros.


`Complete`'s comment is the module's account of why this verb writes a flag and
returns rather than ending the session itself, and it is the only place in the
corpus that names the driver's escalation.

<!-- fragment «verbs-complete-help» owner="admit-before-signal" source="crates/grove-llm/src/cli.rs" lines="272-289" parent="verbs-leaving" -->
````rust
    /// Signal task completion to the self-driving loop. Run this as
    /// the **last step** of a task, after commit + retire — it is how the loop
    /// ends this harness session and starts the next task with fresh context.
    ///
    /// Writes the disposition flag (relaunch by default, or finish with
    /// `--done`) to the signal file and returns immediately. Ending the
    /// session is the loop driver's job, not this verb's: the driver launched
    /// this session and is watching for the signal file while it runs, so it
    /// applies grace → SIGTERM → kill-grace → SIGKILL to its own child once
    /// the file appears (driver-side watcher) — the driver can always signal
    /// its child, unlike an in-agent self-kill, which some harness sandboxes
    /// (e.g. codex's Seatbelt) silently deny. Do nothing else after running
    /// it. The default relaunches the loop for the next task; `--done` — the
    /// Finish cycle's last action — stops it cleanly. The signal-file default
    /// comes from the loop driver's environment (`GROVE_SIGNAL_FILE`); when
    /// that is absent (a session bare `grove` did not launch) it is a safe
    /// near-no-op that just tells you to exit manually.
    Complete(CompleteArgs),
````
<!-- /fragment -->

The argument is a claim about what a process may do to itself, and the
comment states its evidence: a driver-side watcher is used because an in-agent
self-kill is something a harness sandbox may silently deny, with codex's
Seatbelt named as the case that was met. The driver is the harness's own parent
process and outside whatever sandbox the harness runs under, so it can always
signal its child. Nothing in this module tests any of that — the escalation is
`grove-loop`'s and the channel is `keyed-launch`'s — and what this chapter can
show is the half the comment says is left here: the flag is written, and the
verb returns. The measured transcript is exactly that, and the driver acting on
it afterwards was observed rather than asserted.

One clause in the comment names the driver, and it names it *bare `grove`*
because the spelling it had did not survive a check against this build. It
described the no-channel case as *a session not under `grove do`*, and `do` is
a verb neither binary has: `removed_surface.rs` records it as removed from
both as *bare `grove`'s business*, and that same spelling is the driver the
rest of the comment describes — the binary the live measurement ran under. The
rest of the sentence was accurate as written: an absent channel is the safe
near-no-op the worked example's no-channel invocation showed, so only the
parenthesis moved. Correcting it is a change to a frozen source root and was
not this chapter's to make; `complete-help-grove-do-k101` made it, and the
fragment above reproduces the corrected bytes. The substitution fits the line
it replaced, so the fragment's own `272-289` and every range below it in this
file are unmoved — which is the property that let one commit carry the source,
this page and the closing chapter's tally together. It is the last of the
three comment defects these chapters found while drafting to be corrected,
after the two that *Growing the tree* and *Ending work* found.

The table gathers both verbs' promises against the line that keeps each and the
test that holds it, in the form the closing chapter's rows are built from.

| Promise in the help | Verb | Kept at | Held by |
|---|---|---|---|
| revalidates the live finish leaf and the absence of ordinary work | `finish-commit` | the call, lines 450–451 | `finish_commit_refuses_when_ordinary_work_appeared`, `finish_commit_refuses_a_handle_that_is_not_the_live_finish_leaf` |
| a deletion and a path-scoped commit, with no transaction | `finish-commit` | the call, lines 450–451 | `native_jj_finish_commit_records_only_the_teardown`, `colocated_jj_finish_commit_records_only_the_teardown` |
| a precondition refusal names the way out | `finish-commit` | the call's refusals | `finish_commit_refuses_an_untracked_task_tree_naming_how_to_track_it`, for `jj commit … root:.grove`; `finish_commit_on_an_absent_tree_names_the_operation_log`, for `jj op log` and `jj undo` |
| a failed deletion names `jj restore .grove`, a failed commit `jj undo` | `finish-commit` | the call, and the seam beneath it | no test in either crate — each needs a failure injected between the steps |
| only `.grove/` is committed; unrelated changes stay uncommitted | `finish-commit` | the call, lines 450–451 | `native_jj_finish_commit_preserves_unrelated_working_copy_changes`, and its colocated twin |
| does not infer or automate the human confirmation | `finish-commit` | nowhere — the code cannot keep it | — |
| the change id on stderr | `finish-commit` | line 452 | no test in this crate |
| writes the disposition flag and returns immediately | `complete` | line 464 | `relaunch_signal_is_read_back_as_relaunch`, `done_signal_is_read_back_as_done` |
| the default relaunches; `--done` stops the loop cleanly | `complete` | line 464 where `args.done` is passed to the call | the same two, through the token each reads back; lines 466–470 only choose which of the two the stderr line reports |
| the default channel comes from `GROVE_SIGNAL_FILE` | `complete` | line 460 in `signal_channel` | `an_empty_signal_environment_is_no_loop_context`, for the empty value; the flag's precedence over a non-empty one is measured, not tested |
| an absent channel is a safe near-no-op that says so | `complete` | lines 473–476 | `no_channel_at_all_is_answered_rather_than_refused`, for the value |
| ending the session is the driver's job | `complete` | nothing in this module | the driver's, in `crates/grove-loop` |
| the channel is the one the epoch admitted | `complete` | lines 461–463 | `grove_llm_admits_only_the_live_epoch_while_version_remains_exempt` |

The last row is the chapter's rule and the only one held by a test that drives
this binary as a process under a real driver.
`grove_llm_admits_only_the_live_epoch_while_version_remains_exempt`, in
`crates/grove-loop/tests/driver_lease.rs`, launches a driver over a real
workspace with a harness that reports the channel it was given and then waits,
runs `pick` on the live channel and requires success, runs `complete` with a
different channel and requires both a refusal and that no file appeared at that
path, runs `pick` on the stale channel and requires the *stale Grove session*
refusal, and runs `--version` on the same stale channel and requires exit `0`.
The third of those five is this chapter's order stated as a test. It makes
three assertions on that one invocation — the command failed, no file appeared
at the path the flag named, and stderr carries *does not match the admitted
session epoch* — and the middle one is what would fail on the day the write
moved ahead of the check, for that reason rather than merely on an exit
status.

The composite that reassembles the two variants is stated here.

<!-- fragment «verbs-leaving» owner="admit-before-signal" source="crates/grove-llm/src/cli.rs" lines="248-289" parent="source-command-surface" -->
<!-- insert «verbs-finish-commit-help» -->
<!-- insert «verbs-complete-help» -->
<!-- /fragment -->

The two verbs that leave the loop have now been read, and neither handler opens
a grove: `cmd_complete` touches no tree, and `cmd_finish_commit` leaves the
exclusive opening to the call, which takes it and holds it through the
deletion. `complete` resolves a channel, asks the guard `run` obtained whether it
is the admitted one, writes a word, and names what the loop will do next.
`finish-commit` reads a handle by its type, hands the workspace to a call that
opens the tree it is about to delete, and prints the change id that records the
deletion. With them the twelve verbs are all read, each as one call into
`grove_loop::verbs` plus rendering, and the three orders the header promised
have all been shown where they happen. The next chapter puts them in one table
and asks what the compiler holds, what order holds, and what tests hold.

[Previous: Ending work](05-ending-work.md) | [Contents](README.md) | [Next: What order holds](07-what-order-holds.md)
