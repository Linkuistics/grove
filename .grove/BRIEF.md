# grove.refactor-for-minimalism — brief

## Goal

Decompose grove into **five modules with independent lifetimes**, three of which
are reusable outside grove, and shrink what remains of grove itself to the loop
that composes them. The target is a launcher that owns a loop and a vocabulary,
and nothing else.

| module | owns | domain-free |
|---|---|---|
| **tree store** | the on-filesystem ordered tree: read, mutate, `exists?`, `initialize`, `delete`, and a no-outcome answer to a search | yes |
| **runner** | a config file of `key → complete command template`, whole-word expansion, direct argv spawn, session supervision, the out-of-band completion signal, kill escalation | yes |
| **loop** | compose the two: `exists? → create or find next → determine the command → run → finalise`. Holds grove's vocabulary — kind, handle, prompt composition — and the one-driver-per-workspace lease | no |
| **skills** | the methodology, and the routing from task type to procedure | no |
| **VCS seam** | taking a commit, and the guidance printed when grove declines to proceed | partly |

## Done when

- Each module is testable through its own interface without the other four.
- The tree store is the only thing that touches the task tree.
- The runner is the only thing that spawns or supervises a process.
- Adding a task type is authoring content, not editing and rebuilding a binary.
- No module reimplements a guarantee the VCS already provides.

## Decomposition

Ordering is by what unblocks what. The tree store's new operations are the
precondition for deleting grove's second lock layer; the runner's extraction is
independent of it; the skills question depends on both, because it is what makes
a task type a label rather than a compiled variant.

## Principles

These are settled, and a session that finds itself arguing against one should
raise it rather than quietly design around it.

1. **The version control system owns safety, history and transactionality.**
   It is not this project's concern. Where grove currently hand-builds a
   durable record of a pre-operation state, a proven rollback, or a crash-atomic
   multi-path mutation, the answer is the VCS's own — jj snapshots the working
   copy before every command, and its operation log is the transaction record.
   Grove takes commits; it does not implement transactions.

2. **Anomalies stop with a good message; they are not repaired in code.**
   A misnamed file, a torn tree, an unrecognised state: name what is wrong, name
   how to fix it, and stop. Recovery machinery is not written where a sentence
   and a human will do. An error that only reports detection is unfinished — the
   advice is part of the error.

3. **One type owns a name, end to end.** A name's parsing and unparsing live in
   exactly one type, and nothing else spells or peels one. This holds for the
   *handle* as well as the filename: the handle is a projection of the name and
   belongs to the same owner.

4. **The skills drive; grove is ambient.** Anything a session can do, a session
   does — through verbs it invokes. Grove keeps only what a session physically
   cannot do for itself: relaunch itself with fresh context, kill itself under a
   sandbox, choose its own vendor before it exists, and tell itself to load the
   methodology.

5. **Entries are marked, never removed.** Inherited from the store, whose key
   allocation derives from the names on disk. Deleting the *root* is a different
   operation from removing an *entry*; only the first is on the table.

## Pointers

- `docs/adr/task-tree-transactions-fail-closed.md` — the witness protocol
  principle 1 supersedes. Its own reopen condition is the one being taken:
  *"or a durable finish receipt is introduced"*.
- `docs/adr/complete-session-configuration.md` — the completeness rule that a
  task-type-as-label change reopens.
- `docs/adr/entries-are-never-removed.md` — principle 5, argued from key
  allocation.
- `CONTEXT-MAP.md` — the vocabulary-boundary discipline the first extraction
  used, and the model for the next ones.

## Notes

Migration is **out of scope and to be deleted**, not preserved: no legacy tree
needs it.
