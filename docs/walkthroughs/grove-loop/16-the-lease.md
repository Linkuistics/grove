# One live driver per working tree
<!-- book-page id="the-lease" slice="one-per-working-tree" order="16" -->
[Previous: The twelve verbs, and the two that are not](15-the-verbs.md) | [Contents](README.md) | [Next: Which calls the lease admits](17-the-epoch.md)

<a id="one-per-working-tree"></a>
## The rule: the seam owns *where*, grove owns *whose*

Chapter 15 finished the tree. Everything from chapter 2 to chapter 15 has been
about a store, a grammar over its entry names, and twelve verbs — and every one
of those chapters could answer *what is true now* by looking at `.grove/`. This
chapter cannot, and it is the only part of the crate that cannot.

> **The version control seam owns *where* an untracked coordination directory
> may live; grove owns *whose* it is, and this block is the whole of that
> ownership.** Grove supplies one string — a namespace — and gets back a
> directory guaranteed to be inside this exact workspace, untracked, and shared
> with no other namespace. Everything after that is grove's: two file names, one
> lock apiece, a nonce, and a record format that says which process holds what.

Chapter 13 has already said what that means for the book's thesis, and this
chapter inherits the argument rather than restating it: this file is *locking
whose whole purpose is to hold state the tree must not hold*, and the two
chapters are the thesis and its deliberate exception. The reason that contrast is
chapter 13's and not this one's is ordering — it is worth having before either
page needs it, and the earlier page is where it goes. What is left here is the
mechanism.

**The size, stated once and checkably.** `driver_lease.rs` is 1,383 lines, which
makes it the **fourth** largest of the thirteen roots, behind `tree_lifecycle.rs`
at 2,725, `task_tree.rs` at 2,038 and `task_name.rs` at 1,712. Split at its
`#[cfg(test)]` line it yields this chapter's 819 lines and chapter 17's 564 — and
819 is the **largest single owned block in the book**, ahead of chapter 13's 808
and chapter 12's 775. A rank is easy to get wrong here because the root and the
owned block are two different units and only one of them is 1,383 lines, which is
why this chapter counts the lines itself rather than carrying a rank in from
anywhere else.

**Say which tree.** The store's vocabulary does not reach this block at all.
There is no `Entry`, no `Key`, no `Sought`, no ordinal and no position in these
819 lines; `ordinal-fs-tree` is not imported and neither is `task_tree`. When
this chapter says *tree* it means the **working tree** — a jj workspace root,
identified by a device and an inode — and never the task tree. That is not a
convenience of wording. It is the point of the block: the two things grove has
called a tree are, here, deliberately unrelated, and the lease is keyed to the
one the store knows nothing about.

The carried example reaches the step before any of the others could have
happened. A human types bare `grove` in a working tree.

```text
bare `grove`, in <worktree>

  Workspace::resolve(cwd)              -> the closest .jj/, canonicalised
  workspace.control_dir("grove")       -> <worktree>/.jj/grove/        created if absent
  DriverLease::acquire(&workspace)     -> flock(LOCK_EX|LOCK_NB) on driver.lease
                                          session.epoch written `inactive`
                                          abandoned channels discarded

  a second bare `grove`, same tree:
                                       -> "another Grove driver already owns
                                           <worktree>; the existing Grove driver
                                           must stop before this one can start"
```

Nothing in that sketch is recoverable from `.grove/`. The lease file carries no
ownership once its lock is gone, the epoch file carries none either, and neither
is committed. A reader who has followed the book this far and asks the crate's
own question — *what did not move, and why could it not?* — gets the clearest
answer in the corpus here: what could not move is a fact about **this process, on
this machine, right now**, and there is no name to spell it in.

<a id="twelve-per-cent"></a>
## Twelve per cent, and where the argument has to come from

This chapter is written under a different instruction from every chapter before
it, and the reason is a measurement. Counting lines whose first non-space
characters are `//`, the production half of `driver_lease.rs` is **103 comment
lines in 819**, or 12.6%. Counted the same way, and comparing production halves
against production halves, every other root in this crate is between three and
six times as argued: `tree_lifecycle.rs` 444 in 1,076, `task_name.rs` 429 in
1,020, `task_tree.rs` 432 in 1,015, `loop_driver.rs` 281 in 546, and the four
unsplit roots at 156 in 358, 202 in 363, 171 in 245 and 42 in 57. It is the
thinnest-argued root in the corpus, and it is the largest owned block in the
book.

Those counts are this chapter's own, under the rule just stated, and they
reproduce the structure brief's figures at every root but two: `verbs.rs` comes
out at 55.6% against the brief's 57%, and `driver.rs` at 73.7% against 73%.
Chapter 15 met the same two-root disagreement, gave the counts rather than the
percentages, and cut no leaf for it — the rule behind the brief's figures is not
stated anywhere, and a rounding difference is not a defect in either.

Chapters 1 to 15 could mostly quote and connect, because the comments already
made the argument. Here they mostly do not, so this chapter's job is to **supply
the argument**: for each mechanism, the line that enforces it, the failure it
prevents, and the clause of the decision record it keeps. That record is
`docs/adr/one-live-driver-per-working-tree.md`. It is named in prose throughout
this chapter and never linked, which is the book's outbound-link contract holding
rather than an omission — a book's local targets are its own pages, its own
roots, the guide and the glossary, and a decision record is none of those.

**The 103 lines split three ways, and the split decides what any tool can check
about them.**

| Marker | Lines | What `cargo doc` does with them |
|---|---:|---|
| `//!` module header | 9 | renders, and checks its intra-doc links |
| `///` item documentation | 61 | renders, and checks attachment and links |
| `//` plain comments | 33 | reads nothing at all |

`cargo doc --no-deps --document-private-items -p grove-loop` reports
**twenty-six warnings** over this crate, and **none of them names
`driver_lease.rs`** — the
run was repeated with the file's timestamp touched, so the silence is a fresh
reading and not a cached one. That clean result is true, and it is evidence about
seventy lines. It says nothing whatever about the other thirty-three, and the one
factual defect this chapter found is in them. The section on the two acquisitions
below shows it.

<a id="the-whole-block"></a>
## The block, declared

The 819 lines are declared here as one composite whose children are the file's
own items, in file order. Everything below reads them in a different order —
by mechanism rather than by declaration — so the composite is where the file's
own shape stays visible.

<!-- fragment «lease-and-epoch» owner="one-per-working-tree" source="crates/grove-loop/src/driver_lease.rs" lines="1-819" parent="source-driver-lease" -->
<!-- insert «lease-module-header» -->
<!-- insert «lease-imports» -->
<!-- insert «lease-namespace» -->
<!-- insert «lease-names-and-bounds» -->
<!-- insert «lease-file-identity» -->
<!-- insert «lease-process-record» -->
<!-- insert «lease-epoch-record» -->
<!-- insert «lease-lock-mode» -->
<!-- insert «lease-lock-mode-impl» -->
<!-- insert «lease-file-identity-impl» -->
<!-- insert «lease-driver-lease-type» -->
<!-- insert «lease-session-epoch-guard-type» -->
<!-- insert «lease-require-signal-path» -->
<!-- insert «lease-acquire» -->
<!-- insert «lease-acquire-with» -->
<!-- insert «lease-worktree-root» -->
<!-- insert «lease-control-dir» -->
<!-- insert «lease-epoch-transitions» -->
<!-- insert «lease-revalidate» -->
<!-- insert «lease-write-epoch-record» -->
<!-- insert «lease-initialize-epoch-record» -->
<!-- insert «lease-write-epoch-contents» -->
<!-- insert «lease-acquire-lease-file» -->
<!-- insert «lease-acquire-epoch-file» -->
<!-- insert «lease-contention-diagnostic» -->
<!-- insert «lease-acquire-epoch-file-with» -->
<!-- insert «lease-acquire-lease-file-with-hook» -->
<!-- insert «lease-lock-exclusively» -->
<!-- insert «lease-close-on-exec» -->
<!-- insert «lease-random-nonce» -->
<!-- insert «lease-hex-nonce» -->
<!-- insert «lease-encode-path» -->
<!-- insert «lease-decode-path» -->
<!-- insert «lease-record-field» -->
<!-- insert «lease-parse-process-record» -->
<!-- insert «lease-read-record» -->
<!-- insert «lease-read-epoch-record» -->
<!-- insert «lease-probe-live-lease» -->
<!-- insert «lease-probe-with-hook» -->
<!-- insert «lease-admit-ambient-session» -->
<!-- insert «lease-ambient-signal-path» -->
<!-- insert «lease-signal-path-from» -->
<!-- insert «lease-admit-session» -->
<!-- insert «lease-write-record» -->
<!-- /fragment -->

<a id="the-header-and-the-one-move"></a>
## The header, and the one thing that is not a move

The file's own header is nine lines, and two of them do the work the rest of this
chapter depends on.

<!-- fragment «lease-module-header» owner="one-per-working-tree" source="crates/grove-loop/src/driver_lease.rs" lines="1-9" parent="lease-and-epoch" -->
````rust
//! **One live driver per working tree** — the lease that makes it true, and the
//! session-epoch handoff that decides which `grove-llm` calls it admits.
//!
//! It arrived here with the rest of the driver at `loop-crate-driver-k22`. The
//! one thing that is not a move is its derivation of a control directory: it
//! takes a resolved [`Workspace`] and asks the seam for grove's namespace inside
//! it (`docs/adr/one-live-driver-per-working-tree.md`), rather than resolving a
//! path itself. The caller has already resolved the workspace — `run` takes one
//! — so a second resolution here could only disagree with the first.
````
<!-- /fragment -->

Two claims, and both are checkable here rather than taken on trust.

The first is the division of ownership: **the seam owns where, grove owns
whose.** `jj_workspace::Workspace::control_dir` is fourteen lines long — it
validates the namespace, joins `.jj/<namespace>` onto the resolved root, and
`create_dir_all`s it. The validation is where the guarantee lives: an empty
namespace, one containing a separator or a NUL, `.` or `..`, or any name jj
itself owns inside `.jj` is refused. So *inside that exact workspace*,
*untracked* and *shared with no other namespace* are three properties the seam
can keep because it knows what jj owns; *whose* is the one property it cannot
know, because *where a lease file may live* is not sayable without naming whose
lease it is. Grove supplies the missing word and nothing else.

The second is the sentence about resolution: *the caller has already resolved the
workspace — `run` takes one — so a second resolution here could only disagree
with the first.* That is a claim about a call site in another crate, and it holds.
`crates/grove/src/cli.rs` resolves once and hands the same value to both:

```text
let workspace = Workspace::resolve(&cwd)?;
let lease     = DriverLease::acquire(&workspace)?;
…
grove_loop::run(&workspace, lease, &templates)?
```

One resolution behind the lease, behind the delta search, behind `${repo}`
expansion and behind the prompt's stated version control. The lease is then
*moved* into `run`, which is why the guard's lifetime is the loop's: it is
released exactly when the loop that justified holding it returns. The decision
record states both halves — one resolution, and the move into the loop — and this
signature is where they are enforced. A `&Workspace` parameter rather than a
`&Path` is not a convenience; a `&Path` would have made a second resolution
possible, and a second resolution is exactly the disagreement the record forbids.

**`run` is chapter 20's.** The header names it, three chapters early, because the
lifetime argument cannot be made without it. Chapter 1's cast already states the
minimum: `run` is the loop itself, and how it ends.

<!-- fragment «lease-imports» owner="one-per-working-tree" source="crates/grove-loop/src/driver_lease.rs" lines="10-21" parent="lease-and-epoch" -->
````rust

use anyhow::{bail, Context, Result};
use jj_workspace::Workspace;
use std::ffi::OsString;
use std::fmt::Write as _;
use std::fs::{self, File, OpenOptions};
use std::io::{Read, Seek, SeekFrom, Write};
use std::os::fd::{AsRawFd, RawFd};
use std::os::unix::ffi::{OsStrExt, OsStringExt};
use std::os::unix::fs::{MetadataExt, OpenOptionsExt};
use std::path::{Path, PathBuf};
use std::time::{Duration, Instant};
````
<!-- /fragment -->

Eleven `use` lines, and their shape is the chapter in miniature. Three of them
reach into `std::os` and import six names between them: `AsRawFd` and `RawFd` for
descriptors, `OsStrExt` and `OsStringExt` for byte-level path conversion,
`MetadataExt` for device and inode, and `OpenOptionsExt` for a creation mode.
Five of the six appear nowhere else in this crate; the exception is `AsRawFd`,
which `task_tree.rs` also imports at its line 42, for the store's own locking.

Two crates besides `std` are reached, and neither is reached the way a reader
might expect. `anyhow` supplies `bail`, `Context` and `Result`, which is the
crate-wide convention. `jj_workspace::Workspace` appears in exactly two
signatures — `acquire` and `acquire_with` — and once more as `Workspace::resolve`
inside admission. `libc` is never imported at all and is always spelled in full:
`c_int` in `LockMode`, `fcntl`, `flock`, and the lock and errno constants beside
them. `keyed_launch` is reached the same way, once, at
`keyed_launch::Channel::discard_abandoned`. And nothing from `ordinal-fs-tree`
appears — which, in a crate that spends 7,496 of its lines on a task tree over
exactly that store, is the most consequential thing these eleven lines say.

<a id="the-names-on-disk"></a>
## The names on disk, and the two bounds


Two constants' worth of decision, in two fragments. The first is the only one
with an argument attached, and the argument is the block's own thesis restated as
a rule about spelling.

<!-- fragment «lease-namespace» owner="one-per-working-tree" source="crates/grove-loop/src/driver_lease.rs" lines="22-30" parent="lease-and-epoch" -->
````rust

/// Grove's control namespace inside the workspace's administration area.
///
/// The seam owns *where* an untracked coordination directory may live and
/// guarantees it is not shared; grove owns *whose* it is, and this is the whole
/// of that ownership. One constant rather than a literal per call site, because
/// two spellings of the namespace are two control directories and the second
/// driver would not see the first one's lease.
const CONTROL_NAMESPACE: &str = "grove";
````
<!-- /fragment -->

The argument in this doc comment is the strongest in the block, and it is worth
being precise about what it does and does not claim. *Two spellings of the
namespace are two control directories and the second driver would not see the
first one's lease* — that is the failure: not a crash, not a refusal, but two
drivers each holding a valid exclusive lock on a different file, each correctly
concluding that it is the only one. Mutual exclusion that fails **open** is worse
than no mutual exclusion, because the second driver reports success.

The constant is used at exactly one call site — line 155, `workspace.control_dir(CONTROL_NAMESPACE)`
— which is the whole of its use in the workspace. So the comment's *rather than a
literal per call site* is prophylactic rather than descriptive: there is one call
site today and the constant exists so that a second one cannot be spelled
differently. That is worth saying plainly, because a reader who greps for a second
use and finds none could conclude the comment is stale. It is not; it is a rule
for a case that has not arisen.

<!-- fragment «lease-names-and-bounds» owner="one-per-working-tree" source="crates/grove-loop/src/driver_lease.rs" lines="31-36" parent="lease-and-epoch" -->
````rust

const LEASE_FILE_NAME: &str = "driver.lease";
const EPOCH_FILE_NAME: &str = "session.epoch";
const IDENTITY_RETRY_LIMIT: usize = 8;
const EPOCH_HANDOFF_TIMEOUT: Duration = Duration::from_secs(30);
const EPOCH_WAIT_INTERVAL: Duration = Duration::from_millis(10);
````
<!-- /fragment -->

Five constants, no comment between them, and every one of them is a policy
decision the decision record argues and the code merely spells.

| Constant | Value | The clause it keeps |
|---|---|---|
| `LEASE_FILE_NAME` | `driver.lease` | the record's *fixed lease and epoch files are untracked coordination locations*; the generic name is safe **only** because the namespace is grove's |
| `EPOCH_FILE_NAME` | `session.epoch` | the *stable per-workspace session epoch control file* |
| `IDENTITY_RETRY_LIMIT` | `8` | *retries a bounded number of times on an open/lock replacement race* |
| `EPOCH_HANDOFF_TIMEOUT` | 30s | *waits for a fixed internal 30-second handoff bound* |
| `EPOCH_WAIT_INTERVAL` | 10ms | the poll granularity inside that bound |

The first two are the reason the namespace exists at all. The record's rejected
options include *take the administration directory from the seam and name the
control files inside it* — rejected because `driver.lease` and `session.epoch`
are generic names in a directory jj owns and may extend, so a collision is one
release away and would be silent. Grove asks for a namespace precisely so that
these two names can be this ordinary.

`IDENTITY_RETRY_LIMIT` is used in **three** bounded loops in this block — at
lines 376, 451 and 648 — and the same constant caps all three. The two `Duration`s
are used once each, in `acquire_epoch_file`'s defaults at lines 341 and 343. The
record calls the bound, the clock, the control-path resolver and the randomness
source *internal test seams, not user configuration*, and none of the five is
reachable from any command line, environment variable or configuration file; the
seams are the injected parameters two functions below carry.

<a id="four-records"></a>
## Four types, and what each is a record of


Four small types carry the whole protocol, and none of them has a method beyond
what is needed to build or compare it. Read together they are the answer to
*what does this block actually know*, and each is a record of a different thing.

<!-- fragment «lease-file-identity» owner="one-per-working-tree" source="crates/grove-loop/src/driver_lease.rs" lines="37-42" parent="lease-and-epoch" -->
````rust

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct FileIdentity {
    device: u64,
    inode: u64,
}
````
<!-- /fragment -->

Two `u64`s, and the whole protocol rests on them. A path is not an identity: it
can be replaced, and a replacement is exactly the attack the retry loops exist to
survive. A device-and-inode pair taken from an **already-open descriptor** is an
identity, because the kernel will not reissue it while the descriptor lives.

The record states the primitive directly: *the lease is an exclusive,
nonblocking advisory lock keyed by the filesystem device and inode of an
already-open working-tree-root descriptor.* `Copy` is on this type and on
`LockMode`, and on neither of the two on-disk records below — the two things
small enough to pass by value are the two that are never parsed from a file.

<!-- fragment «lease-process-record» owner="one-per-working-tree" source="crates/grove-loop/src/driver_lease.rs" lines="43-49" parent="lease-and-epoch" -->
````rust

#[derive(Debug, PartialEq, Eq)]
struct ProcessRecord {
    worktree_identity: FileIdentity,
    worktree_root: PathBuf,
    nonce: String,
}
````
<!-- /fragment -->

The second wraps the first, and the wrapping is the difference between a lease
and an epoch.

<!-- fragment «lease-epoch-record» owner="one-per-working-tree" source="crates/grove-loop/src/driver_lease.rs" lines="50-55" parent="lease-and-epoch" -->
````rust

#[derive(Debug, PartialEq, Eq)]
struct EpochRecord {
    process: ProcessRecord,
    signal_path: Option<PathBuf>,
}
````
<!-- /fragment -->

The two on-disk records, as parsed. `ProcessRecord` is what a lease file holds
and is the answer to *which process, in which working tree*; `EpochRecord` wraps
it with the one thing the lease file does not carry — the current signal path, or
its absence. The nesting is the protocol: an epoch is *a process record plus a
state*, and the comparison at line 688 that decides whether a probed lease matches
an admitted epoch is a comparison of the `ProcessRecord` half alone, which is why
that half is a type rather than three fields.

`PartialEq` on `ProcessRecord` is not derived for convenience: line 688's
`lease_record != epoch.process` is the operative check that a probed lease
belongs to the epoch an operation was admitted under, and a hand-written
comparison there could drift from the parser that built both sides.
`EpochRecord`'s own `PartialEq` is a different matter: it is derived and then
never used, in either half of the file, and the honest reading is symmetry with
the type it wraps rather than a comparison anything performs.

<!-- fragment «lease-lock-mode» owner="one-per-working-tree" source="crates/grove-loop/src/driver_lease.rs" lines="56-61" parent="lease-and-epoch" -->
````rust

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum LockMode {
    Shared,
    Exclusive,
}
````
<!-- /fragment -->

Its two methods are the reason the enum exists rather than a bare flag.

<!-- fragment «lease-lock-mode-impl» owner="one-per-working-tree" source="crates/grove-loop/src/driver_lease.rs" lines="62-77" parent="lease-and-epoch" -->
````rust

impl LockMode {
    fn operation(self) -> libc::c_int {
        match self {
            Self::Shared => libc::LOCK_SH,
            Self::Exclusive => libc::LOCK_EX,
        }
    }

    fn label(self) -> &'static str {
        match self {
            Self::Shared => "shared",
            Self::Exclusive => "exclusive",
        }
    }
}
````
<!-- /fragment -->

Two variants and two methods of two lines each, and the type prevents a
class of mistake rather than abstracting anything. `operation` is the only
place in the block that maps a mode to `libc::LOCK_SH` or `libc::LOCK_EX`, and
`label` the only place that maps one to a word for a diagnostic. Without the
type, `acquire_epoch_file_with` would take a raw `c_int` and a `&str`, and a
caller that passed `LOCK_SH` with `"exclusive"` would be well-formed, would
compile, and would print a lie in the one message a human reads while waiting for
a handoff.

The distribution matters and is small enough to enumerate. `LockMode::Exclusive`
is passed at lines 269 and 281 — the two epoch writes the driver performs — and
tested at 379 to decide whether the file is opened for writing at all.
`LockMode::Shared` is passed at exactly one place, line 759, inside
`admit_session`. That single use is the record's *an ambient agent-side
`grove-llm` tree operation takes a shared epoch guard*, and it is the boundary
the whole admission section below turns on: the driver writes under an exclusive
lock, an admitted operation reads and **holds** under a shared one, and the two
cannot overlap.

<!-- fragment «lease-file-identity-impl» owner="one-per-working-tree" source="crates/grove-loop/src/driver_lease.rs" lines="78-86" parent="lease-and-epoch" -->
````rust

impl FileIdentity {
    fn from_metadata(metadata: &fs::Metadata) -> Self {
        Self {
            device: metadata.dev(),
            inode: metadata.ino(),
        }
    }
}
````
<!-- /fragment -->

Six lines, one call, and it is the only constructor `FileIdentity` has. Every
identity in this block therefore comes from a `fs::Metadata`, and there is no way
to build one from a device and an inode a caller invented — except inside
`parse_process_record`, which builds the struct literally from the two parsed
fields. That asymmetry is deliberate and is worth naming: an identity **read from
a file** is a claim, and an identity **taken from a descriptor** is a fact, and
the only place the two meet is the comparison that decides whether to believe the
claim.

<a id="two-guards"></a>
## Two guards, and what dropping one releases


Each of the two guards owns a lock its own declaration never names — the lock
lives in a `File` field, and closing the field is what releases it. The first is
the lease itself, and its doc comment is the most consequential three sentences
in the file.

<!-- fragment «lease-driver-lease-type» owner="one-per-working-tree" source="crates/grove-loop/src/driver_lease.rs" lines="87-102" parent="lease-and-epoch" -->
````rust

/// Process-scoped ownership of one exact Grove working tree.
///
/// Dropping this guard closes both descriptors. The kernel then releases the
/// advisory lease; bytes left in the lease file do not carry ownership.
#[derive(Debug)]
pub struct DriverLease {
    worktree_root: PathBuf,
    control_dir: PathBuf,
    _worktree_directory: File,
    worktree_identity: FileIdentity,
    lease_path: PathBuf,
    lease_file: File,
    lease_identity: FileIdentity,
    nonce: String,
}
````
<!-- /fragment -->

*Dropping this guard closes both descriptors. The kernel then releases the
advisory lease; bytes left in the lease file do not carry ownership.*

Three sentences, and they are the reason the whole design works after a crash.
The struct holds two `File`s — `_worktree_directory` and `lease_file` — and the
leading underscore on the first says its value is never read: it exists to be
open. `flock` is released when the last descriptor referring to the open file
description is closed, and the kernel does that on `Drop`, on unwinding panic,
and on process death alike. There is no cleanup path in this file, no `Drop`
implementation, and no tombstone, and the record says why: *kernel release on
return, panic, or process death makes restart ordinary continuation while
`.grove/` still exists.*

The failure this prevents is the one the record's last rejected option names:
*use a PID or the existence of a control file as ownership — rejected because
PIDs are reused and files survive crashes.* A file-existence lease needs a
cleanup path; a cleanup path needs to distinguish a dead owner from a live one;
and the only honest way to do that is a kernel lock, which is what this is. So
the leftover bytes are deliberately meaningless. They are the record a probe
parses to learn **which** process claimed the lease, never **whether** anyone
still holds it — that question is answered by trying the lock.

`nonce` is the last field and the one with no local justification at all. Its
argument is in the record: each driver writes a fresh 128-bit value from the
operating system's randomness source, *not derived from a PID, clock, address,
iteration counter, or task key*, and the accepted cost of that choice is stated
as at most one in 2^128 per independent draw. The nonce ensures that a
replacement driver's epoch is distinguishable from its predecessor's even when
every other field — device, inode, path — is identical. That is the case that
matters, since a replacement driver by definition owns the same working tree.

<!-- fragment «lease-session-epoch-guard-type» owner="one-per-working-tree" source="crates/grove-loop/src/driver_lease.rs" lines="103-108" parent="lease-and-epoch" -->
````rust

#[derive(Debug)]
pub struct SessionEpochGuard {
    _epoch_file: File,
    signal_path: PathBuf,
}
````
<!-- /fragment -->

The second guard, and it is a guard over a lock this file never names in the
struct. `_epoch_file` is again underscore-prefixed and again exists to be open —
holding the **shared** epoch lock taken at line 759 — and `signal_path` is the
channel that lock admitted. An `admit_session` caller keeps this value alive
across its own tree access, and that lifetime is the protocol's one
non-obvious guarantee, argued below.

<!-- fragment «lease-require-signal-path» owner="one-per-working-tree" source="crates/grove-loop/src/driver_lease.rs" lines="109-133" parent="lease-and-epoch" -->
````rust

impl SessionEpochGuard {
    /// Refuse an operation whose completion channel is not the one this epoch
    /// admitted.
    ///
    /// # Errors
    ///
    /// Any other path, including none.
    pub fn require_signal_path(&self, signal_path: Option<&Path>) -> Result<(), crate::Error> {
        Ok(self.require_signal_path_inner(signal_path)?)
    }

    fn require_signal_path_inner(&self, signal_path: Option<&Path>) -> Result<()> {
        if signal_path != Some(self.signal_path.as_path()) {
            bail!(
                "completion signal path does not match the admitted session epoch: expected {}, got {}",
                self.signal_path.display(),
                signal_path
                    .map(|path| path.display().to_string())
                    .unwrap_or_else(|| "<none>".to_string())
            );
        }
        Ok(())
    }
}
````
<!-- /fragment -->

The guard's only method, and the shape is the crate's uniform one: a public
function that converts into `crate::Error`, and a private inner that does the
work in `anyhow`. `grove-llm`'s CLI calls it at one place, immediately after
admission, to check that the channel the command was *given* is the channel the
epoch *admitted*.

The failure prevented is narrow and easy to miss. Admission has already checked
that the ambient `GROVE_SIGNAL_FILE` matches the active epoch; this checks that
the path an operation is about to **write to** is the same one. Those are two
different values in one process, and the record's scope note is what makes the
distinction load-bearing: the protocol *prevents an old session from resolving,
mutating, or signalling through `grove-llm` after epoch rotation*, and signalling
is the third of those three. `None` fails the comparison exactly as a wrong path
does, and the diagnostic renders it as `<none>` — a command that carries no
channel at all is refused rather than treated as harmless.

<a id="acquisition-is-an-order"></a>
## Acquisition is an order, not a set of steps


Acquisition is six steps in one function, and the only thing that makes them
correct is their order. The public entry is a one-line conversion over the
private form.

<!-- fragment «lease-acquire» owner="one-per-working-tree" source="crates/grove-loop/src/driver_lease.rs" lines="134-145" parent="lease-and-epoch" -->
````rust

impl DriverLease {
    /// Take exclusive driver ownership of `workspace`'s working tree.
    ///
    /// # Errors
    ///
    /// A workspace whose control directory cannot be created, a working tree
    /// root that cannot be opened, or a predecessor whose session epoch does not
    /// hand over.
    pub fn acquire(workspace: &Workspace) -> Result<Self, crate::Error> {
        Ok(Self::acquire_with(workspace, || {})?)
    }
````
<!-- /fragment -->

The public entry point, and the only one in the block that a binary calls
directly. Its one production call site is `crates/grove/src/cli.rs` line 46, on
the fourth line of bare `grove`'s `run` — before configuration is validated and
before `.grove/` is read or created. The record puts that ordering first: *bare
`grove` acquires one process-scoped driver lease for the working tree **before**
it validates configuration or reads or mutates that grove.*

`acquire_with` beneath it takes a closure that does nothing here. That parameter
is the first of five injected seams in this block, and the record reserves the
species: *a test lock/filesystem backend with post-open/post-lock barriers and an
event trace makes the protocol races and guard lifetimes deterministic without
widening the production interface.* Every one of them is `pub`-free and called
with an inert argument from production. What each is *used for* is chapter 17's,
which owns the tests that pass them.

<!-- fragment «lease-acquire-with» owner="one-per-working-tree" source="crates/grove-loop/src/driver_lease.rs" lines="146-199" parent="lease-and-epoch" -->
````rust

    fn acquire_with(
        workspace: &Workspace,
        before_initial_epoch_handoff: impl FnOnce(),
    ) -> Result<Self> {
        // The control directory comes from the resolution the caller already
        // performed: the seam refuses a working tree that is not jj-enabled
        // before it hands one out, and it hands back grove's own namespace
        // inside it, created if absent.
        let control_dir = workspace.control_dir(CONTROL_NAMESPACE)?;
        let worktree_root = workspace.root().to_path_buf();
        let worktree_directory = File::open(&worktree_root).with_context(|| {
            format!(
                "opening working tree root {} for driver ownership",
                worktree_root.display()
            )
        })?;
        ensure_close_on_exec(worktree_directory.as_raw_fd())?;
        let worktree_identity = FileIdentity::from_metadata(
            &worktree_directory
                .metadata()
                .context("reading pinned working-tree identity")?,
        );

        let lease_path = control_dir.join(LEASE_FILE_NAME);
        let (lease_file, lease_identity) = acquire_lease_file(&lease_path, &worktree_root)?;
        let nonce = hex_nonce(random_nonce()?)?;

        let mut lease = Self {
            worktree_root,
            control_dir,
            _worktree_directory: worktree_directory,
            worktree_identity,
            lease_path,
            lease_file,
            lease_identity,
            nonce,
        };
        lease.revalidate_inner()?;
        before_initial_epoch_handoff();
        lease.initialize_epoch_record()?;
        // Only after this lease owns the workspace, and after the replacement
        // driver's inactive epoch record is installed: cleaning first would
        // remove a live predecessor's channel. The grammar of an abandoned
        // channel is the runner's, so the runner recognises them
        // (`keyed_launch::Channel::discard_abandoned`); the lease supplies only
        // the directory.
        if let Err(error) = keyed_launch::Channel::discard_abandoned(&lease.control_dir) {
            eprintln!(
                "grove: warning: could not clean every signal channel abandoned by a previous driver; continuing because fresh channel allocation does not depend on an empty control directory: {error}"
            );
        }
        Ok(lease)
    }
````
<!-- /fragment -->

Fifty-four lines, and they are an **order** rather than a list. Six steps, and
five of the six are wrong if moved.

1. **Ask the seam for the control directory** (155). This is the derivation the
   header called *the one thing that is not a move*, and it is first because
   everything else needs the directory. A working tree that is not jj-enabled and
   a `.jj/` that cannot hold a directory both stop here, which is why the record
   can say a `--help` or `--version` invocation returns without workspace
   resolution or a lease at all: those never reach this function.
2. **Open the working-tree root and pin its identity** (157–168). The descriptor
   is opened *before* the identity is read from it, and the identity comes from
   `worktree_directory.metadata()` — the descriptor's own — rather than from
   `fs::metadata(&worktree_root)`, which would have been a path lookup and could
   have named a different directory by the time the answer came back. This is the
   difference between pinning a thing and pinning a name.
3. **Take the lease lock** (171). One line, and everything about the retry
   protocol is behind it.
4. **Draw the nonce** (172).
5. **Revalidate** (184), then run the injected barrier, then **install the epoch
   record** (186).
6. **Discard abandoned channels** (193), last, and warn rather than fail.

Step 6 carries the only comment in the function that argues rather than
describes, and it argues an ordering: *only after this lease owns the workspace,
and after the replacement driver's inactive epoch record is installed: cleaning
first would remove a live predecessor's channel.* The failure prevented is a
replacement driver deleting the signal file a still-running session is about to
write to — which would not error anywhere, and would present as a session that
completed and a loop that never noticed. The record states the same rule from the
other end: *a replacement driver removes abandoned signal files only after it
owns the lease and has exclusively invalidated the old epoch.*

The comment is also careful about a boundary the book has been tracking since
chapter 1: *the grammar of an abandoned channel is the runner's, so the runner
recognises them; the lease supplies only the directory.* Grove asks
`keyed-launch` to discard what it considers abandoned and passes a path. It does
not enumerate the directory, does not parse a channel name, and does not decide
what abandoned means. Naming the crate in prose is all this book may do here —
its outbound links cannot reach another book.

**And the failure is a warning, not an error.** The `eprintln!` says why in its
own text: fresh channel allocation does not depend on an empty control directory.
A driver that cannot tidy is still a correct driver, and refusing to start over a
housekeeping failure would convert a full disk or a permissions oddity into an
unusable tool.

<a id="what-the-lease-hands-out"></a>
## What the lease hands out, and to whom

Three small accessors and two one-line transitions. Their interest is entirely in
who calls them, so they are read together and their call sites enumerated
afterwards.

<!-- fragment «lease-worktree-root» owner="one-per-working-tree" source="crates/grove-loop/src/driver_lease.rs" lines="200-206" parent="lease-and-epoch" -->
````rust

    /// The working tree this lease owns, in the spelling the workspace resolved
    /// to.
    #[must_use]
    pub fn worktree_root(&self) -> &Path {
        &self.worktree_root
    }
````
<!-- /fragment -->

The second hands out the directory the runner allocates in, and restates the
division of ownership a third time.

<!-- fragment «lease-control-dir» owner="one-per-working-tree" source="crates/grove-loop/src/driver_lease.rs" lines="207-216" parent="lease-and-epoch" -->
````rust

    /// Grove's control directory inside the workspace's administration area —
    /// where the runner allocates each launch's completion channel.
    ///
    /// Handed out rather than used here: the channel's name grammar, allocation
    /// and cleanup are `keyed-launch`'s, and the lease's contribution is the one
    /// thing that is genuinely grove's — *which* directory is ours.
    pub(crate) fn control_dir(&self) -> &Path {
        &self.control_dir
    }
````
<!-- /fragment -->

The last two are the driver's epoch transitions, and they are one line each.

<!-- fragment «lease-epoch-transitions» owner="one-per-working-tree" source="crates/grove-loop/src/driver_lease.rs" lines="217-224" parent="lease-and-epoch" -->
````rust

    pub(crate) fn activate_session_epoch(&self, signal_path: &Path) -> Result<()> {
        self.write_epoch_record(Some(signal_path), "pre-spawn activation")
    }

    pub(crate) fn invalidate_session_epoch(&self) -> Result<()> {
        self.write_epoch_record(None, "post-reap invalidation")
    }
````
<!-- /fragment -->

`control_dir`'s doc comment states the division a third time, and by now it is
the chapter's refrain: *handed out rather than used here — the channel's name
grammar, allocation and cleanup are `keyed-launch`'s, and the lease's
contribution is the one thing that is genuinely grove's, which directory is
ours.* The `pub(crate)` on it and on the two transitions is the boundary made
mechanical: nothing outside this crate can allocate in grove's control directory
or move the epoch, because the type through which one would do it does not expose
the operations.

The two transitions have no doc comments at all, and they do not need one: each
is a single call to `write_epoch_record` with the two things that distinguish it,
a signal path and an operation label. The labels are not decoration — they are
what the contention diagnostic and every timeout message interpolate, so
*pre-spawn activation* and *post-reap invalidation* reach a human's terminal
verbatim.

**The whole outward surface of this block is nine production call sites, in three
files, and two of the three are other crates.**

| Operation | Called from | |
|---|---|---:|
| `DriverLease::acquire` | `crates/grove/src/cli.rs` | 1 |
| `worktree_root` | `loop_driver.rs` | 1 |
| `control_dir` | `loop_driver.rs` | 1 |
| `revalidate` | `loop_driver.rs` | 2 |
| `activate_session_epoch` | `loop_driver.rs` | 1 |
| `invalidate_session_epoch` | `loop_driver.rs` | 1 |
| `admit_ambient_session` | `crates/grove-llm/src/cli.rs` | 1 |
| `SessionEpochGuard::require_signal_path` | `crates/grove-llm/src/cli.rs` | 1 |

Eight operations, nine call sites. `crates/grove-loop/tests/driver_lease.rs`
calls `acquire` fourteen more times, and that file is evidence rather than
corpus.
The six `loop_driver.rs` sites are chapter 20's block; the two `grove-llm` sites
are the `grove-llm` book's. What this table settles is that the block has no
internal consumer at all: every one of its eight operations exists for a caller
in another module or another crate, which is why the split between `pub`,
`pub(crate)` and private here is a statement about the loop rather than about
encapsulation.

<!-- fragment «lease-revalidate» owner="one-per-working-tree" source="crates/grove-loop/src/driver_lease.rs" lines="225-265" parent="lease-and-epoch" -->
````rust

    /// Confirm that the paths still name the descriptors this process owns.
    ///
    /// # Errors
    ///
    /// A working tree root or a lease file that was replaced while this process
    /// held ownership, or either one gone unreadable.
    pub fn revalidate(&self) -> Result<(), crate::Error> {
        Ok(self.revalidate_inner()?)
    }

    fn revalidate_inner(&self) -> Result<()> {
        let current_root =
            FileIdentity::from_metadata(&fs::metadata(&self.worktree_root).with_context(|| {
                format!(
                    "reading working tree root {} during driver lease revalidation",
                    self.worktree_root.display()
                )
            })?);
        if current_root != self.worktree_identity {
            bail!(
                "working tree root was replaced while Grove held its driver lease: {}",
                self.worktree_root.display()
            );
        }

        let current_lease =
            FileIdentity::from_metadata(&fs::metadata(&self.lease_path).with_context(|| {
                format!(
                    "reading driver lease path {} during revalidation",
                    self.lease_path.display()
                )
            })?);
        if current_lease != self.lease_identity {
            bail!(
                "driver lease path was replaced while Grove held ownership: {}",
                self.lease_path.display()
            );
        }
        Ok(())
    }
````
<!-- /fragment -->

The mechanism is a re-read of two identities and a comparison against the two
pinned at acquisition. The failure it prevents is the one the record admits it
cannot prevent in general: *no claim is made that open/lock identity revalidation
survives unlink/recreate outside an acquisition window.* Revalidation is what
narrows that window. It cannot stop another process replacing the working-tree
root or the lease path; it can stop grove from **continuing to act** as though it
still owned what it locked.

The record fixes exactly when: *the driver holds both root and lock descriptors
until the loop has stopped, and revalidates the lock path before every lifecycle
transition and foreground launch.* That is two occasions, and there are two calls,
both in `loop_driver.rs`, and each carries a context string naming its occasion —
*revalidating driver lease before loop transition* at line 239 and *revalidating
driver lease before foreground launch* at line 281. A reader can check the clause
against the code by reading two strings.

The two arms fail differently on purpose. A replaced **working-tree root** means
the tree grove is driving is not the tree it locked; a replaced **lease path**
means the file it locked is no longer the file another driver would find. Both are
fatal, and both name the path in the message, because the human reading it is
about to be told their loop stopped and needs to know which of the two things
moved.

<a id="three-writes"></a>
## Three writes to one record, each under its own guard


Both writers go through one private method, which differs from the transitions
above only in taking the state and the label as arguments.

<!-- fragment «lease-write-epoch-record» owner="one-per-working-tree" source="crates/grove-loop/src/driver_lease.rs" lines="266-277" parent="lease-and-epoch" -->
````rust

    fn write_epoch_record(&self, signal_path: Option<&Path>, operation: &str) -> Result<()> {
        let path = self.control_dir.join(EPOCH_FILE_NAME);
        let mut file = acquire_epoch_file(&path, LockMode::Exclusive, operation)?;
        write_epoch_contents(
            &mut file,
            self.worktree_identity,
            &self.worktree_root,
            &self.nonce,
            signal_path,
        )
    }
````
<!-- /fragment -->

The third writer is acquisition's own, and it is the one with an ordering
argument attached.

<!-- fragment «lease-initialize-epoch-record» owner="one-per-working-tree" source="crates/grove-loop/src/driver_lease.rs" lines="278-300" parent="lease-and-epoch" -->
````rust

    fn initialize_epoch_record(&mut self) -> Result<()> {
        let path = self.control_dir.join(EPOCH_FILE_NAME);
        let mut epoch_file = acquire_epoch_file(&path, LockMode::Exclusive, "driver acquisition")?;
        // Keep the predecessor's lease bytes intact until exclusive epoch
        // handoff succeeds. An operation that already holds shared admission
        // may still probe those bytes while this replacement waits; publishing
        // the new nonce early would reject an operation the old epoch admitted.
        write_epoch_contents(
            &mut epoch_file,
            self.worktree_identity,
            &self.worktree_root,
            &self.nonce,
            None,
        )?;
        write_record(
            &mut self.lease_file,
            self.worktree_identity,
            &self.worktree_root,
            &self.nonce,
        )
    }
}
````
<!-- /fragment -->

The record names three write points and gives each a state: *inactive
immediately after lease acquisition, active immediately before spawn, and
inactive after the child is reaped and before interpreting its signal.* All three
go through `write_epoch_contents`, and the three call paths are visible in this
block and one chapter ahead:

| Point | State | Reached by | Label |
|---|---|---|---|
| after acquisition | `inactive` | `initialize_epoch_record`, line 281 | `driver acquisition` |
| before spawn | `active` | `activate_session_epoch` | `pre-spawn activation` |
| after reap | `inactive` | `invalidate_session_epoch` | `post-reap invalidation` |

Each acquires the file, writes, and drops it — the guard is scoped to the
statement, and the record requires exactly that: *every exclusive guard is
released before another epoch or tree operation begins and before spawn.* A
driver that held the exclusive epoch lock across a spawn would deadlock against
its own child's admission, which takes the shared lock on the same file.

`initialize_epoch_record` is where the ordering argument in this section lives,
and its comment makes it: *keep the predecessor's lease bytes intact until
exclusive epoch handoff succeeds. An operation that already holds shared
admission may still probe those bytes while this replacement waits; publishing
the new nonce early would reject an operation the old epoch admitted.* Read
against the two statements below it, the sequence is: write the **epoch** first,
with the new nonce and no signal path, and only then `write_record` the **lease**
file with the same nonce. The lease bytes are the last thing to change.

The failure prevented is a live, correctly-admitted `grove-llm` operation being
refused mid-flight. Such an operation holds the shared epoch guard and may probe
the lease at any moment; the probe compares the lease record against **its own
admitted epoch**, so a lease file rewritten before that operation finishes turns
`lease_record != epoch.process` true and produces *driver lease record does not
match the admitted session epoch* for a command that did nothing wrong. The
record's own phrasing of the same guarantee is that *an old call admitted before
exclusive invalidation may finish and block handoff* — may finish, not may be
cut off — and this write order is what makes the may true.

<!-- fragment «lease-write-epoch-contents» owner="one-per-working-tree" source="crates/grove-loop/src/driver_lease.rs" lines="301-330" parent="lease-and-epoch" -->
````rust

fn write_epoch_contents(
    file: &mut File,
    worktree_identity: FileIdentity,
    worktree_root: &Path,
    nonce: &str,
    signal_path: Option<&Path>,
) -> Result<()> {
    file.set_len(0)
        .context("truncating previous session epoch record")?;
    file.seek(SeekFrom::Start(0))
        .context("rewinding session epoch record")?;
    writeln!(
        file,
        "state={}",
        if signal_path.is_some() {
            "active"
        } else {
            "inactive"
        }
    )?;
    writeln!(file, "worktree-device={}", worktree_identity.device)?;
    writeln!(file, "worktree-inode={}", worktree_identity.inode)?;
    writeln!(file, "worktree-path-hex={}", encode_path(worktree_root)?)?;
    writeln!(file, "nonce={nonce}")?;
    if let Some(signal_path) = signal_path {
        writeln!(file, "signal-path-hex={}", encode_path(signal_path)?)?;
    }
    file.flush().context("flushing session epoch record")
}
````
<!-- /fragment -->

The record format, and it is the only serialiser in the block for the epoch. Five
or six `key=value` lines, LF-terminated, no framing, no versioning, no checksum.
`set_len(0)` then `seek(0)` — truncate under the lock, never at open — and the
same pair opens `write_record` for the lease file at the end of the chapter.

`state` is first and is what a reader dispatches on, and the presence of
`signal-path-hex` is made a **consequence** of it rather than an independent
field: the `if let Some` at line 326 is the only place that line is written, so an
`inactive` record cannot carry a path by construction. The parser at lines 620 to
624 checks the same invariant from the other side and refuses a record that has one
anyway — belt and braces over a file two processes write.

Paths are hex, and the reason is `OsStr`. A working-tree path is bytes on Unix,
not necessarily UTF-8, and a record format that could not round-trip an arbitrary
path would refuse to protect exactly the working trees whose names are unusual.
Hex costs two bytes per byte and buys a format with no escaping rules, therefore
no escaping bugs, and no ambiguity about where a value ends.

<a id="neither-open-truncates"></a>
## The two acquisitions, and why neither truncates at open


Two files are acquired in this block and the acquisitions are deliberately not
the same function. The lease is taken once and never waited for; the epoch is
taken repeatedly and is the thing a handoff waits on. Each has a thin production
wrapper over a form that takes its seams.

<!-- fragment «lease-acquire-lease-file» owner="one-per-working-tree" source="crates/grove-loop/src/driver_lease.rs" lines="331-334" parent="lease-and-epoch" -->
````rust

fn acquire_lease_file(path: &Path, worktree_root: &Path) -> Result<(File, FileIdentity)> {
    acquire_lease_file_with_hook(path, worktree_root, |_, _| Ok(()))
}
````
<!-- /fragment -->

The epoch's wrapper fixes six defaults rather than one, because the epoch
acquisition is the one that can wait.

<!-- fragment «lease-acquire-epoch-file» owner="one-per-working-tree" source="crates/grove-loop/src/driver_lease.rs" lines="335-350" parent="lease-and-epoch" -->
````rust

fn acquire_epoch_file(path: &Path, mode: LockMode, operation: &str) -> Result<File> {
    acquire_epoch_file_with(
        path,
        mode,
        operation,
        EPOCH_HANDOFF_TIMEOUT,
        Instant::now,
        || std::thread::sleep(EPOCH_WAIT_INTERVAL),
        |_, _| Ok(()),
        |_, _| Ok(()),
        || {
            eprintln!("{}", epoch_contention_diagnostic(mode, operation));
        },
    )
}
````
<!-- /fragment -->

The message it prints on contention is its own function, so that it can be
asserted without capturing a stream.

<!-- fragment «lease-contention-diagnostic» owner="one-per-working-tree" source="crates/grove-loop/src/driver_lease.rs" lines="351-357" parent="lease-and-epoch" -->
````rust

fn epoch_contention_diagnostic(mode: LockMode, operation: &str) -> String {
    format!(
        "waiting for {} session epoch lock for {operation}",
        mode.label()
    )
}
````
<!-- /fragment -->

Two three-line wrappers and a formatter, and their job is to hold the production
defaults so the seams below are invisible to callers. `acquire_epoch_file` fixes
all six: the 30-second bound, `Instant::now`, a 10ms sleep, two inert barriers,
and a contention reporter that prints the diagnostic to stderr. The record
requires the diagnostic and requires it once: *every epoch acquisition first
tries without blocking, emits one diagnostic on contention, and waits for a fixed
internal 30-second handoff bound.*

Splitting `epoch_contention_diagnostic` out of the closure is what makes the text
assertable without capturing stderr, which is the difference between a message a
test can pin and a message that drifts.

<!-- fragment «lease-acquire-epoch-file-with» owner="one-per-working-tree" source="crates/grove-loop/src/driver_lease.rs" lines="358-444" parent="lease-and-epoch" -->
````rust

#[allow(clippy::too_many_arguments)]
fn acquire_epoch_file_with(
    path: &Path,
    mode: LockMode,
    operation: &str,
    timeout: Duration,
    mut now: impl FnMut() -> Instant,
    mut wait: impl FnMut(),
    mut after_open: impl FnMut(usize, &Path) -> Result<()>,
    mut after_lock: impl FnMut(usize, &Path) -> Result<()>,
    mut report_contention: impl FnMut(),
) -> Result<File> {
    let deadline = now()
        .checked_add(timeout)
        .context("computing session epoch handoff deadline")?;
    let mut reported_contention = false;

    for attempt in 1..=IDENTITY_RETRY_LIMIT {
        let mut options = OpenOptions::new();
        options.read(true);
        if mode == LockMode::Exclusive {
            // Same contract as the lease open in `acquire_lease_file_with_hook`:
            // never truncate at open, because this runs before the lock is held;
            // `write_epoch_contents` truncates under the lock instead.
            // `suspicious_open_options` does not fire here only because clippy
            // cannot follow a builder split across statements — stated
            // explicitly so a future refactor into a chained call stays clean.
            options.write(true).create(true).truncate(false).mode(0o600);
        }
        let file = options
            .open(path)
            .with_context(|| format!("opening session epoch file {}", path.display()))?;
        ensure_close_on_exec(file.as_raw_fd())?;
        after_open(attempt, path).context("running session epoch post-open barrier")?;

        loop {
            let result = unsafe { libc::flock(file.as_raw_fd(), mode.operation() | libc::LOCK_NB) };
            if result == 0 {
                break;
            }
            let error = std::io::Error::last_os_error();
            if !matches!(
                error.raw_os_error(),
                Some(code) if code == libc::EWOULDBLOCK || code == libc::EAGAIN
            ) {
                return Err(error).with_context(|| {
                    format!("locking {} session epoch for {operation}", mode.label())
                });
            }
            if !reported_contention {
                report_contention();
                reported_contention = true;
            }
            if now() >= deadline {
                bail!(
                    "timed out after {}s waiting for {} session epoch lock for {operation}",
                    timeout.as_secs(),
                    mode.label()
                );
            }
            wait();
        }

        after_lock(attempt, path).context("running session epoch post-lock barrier")?;
        let descriptor_identity = FileIdentity::from_metadata(
            &file
                .metadata()
                .context("reading locked session epoch identity")?,
        );
        let path_identity = FileIdentity::from_metadata(
            &fs::metadata(path)
                .with_context(|| format!("reading session epoch path {}", path.display()))?,
        );
        if descriptor_identity == path_identity {
            return Ok(file);
        }
        if attempt == IDENTITY_RETRY_LIMIT {
            bail!(
                "session epoch path {} was replaced during acquisition {} times",
                path.display(),
                IDENTITY_RETRY_LIMIT
            );
        }
    }
    bail!("session epoch acquisition exhausted its bounded retries")
}
````
<!-- /fragment -->

Eighty-seven lines, the largest item in the block, and it is two nested loops
doing two different jobs. The outer loop bounds **identity races**; the inner
loop bounds **waiting**.

The outer loop's job is the open/lock replacement race. Between opening a path
and locking the descriptor, the path can be replaced; a lock on the old inode is
then worthless because a second driver will open and lock the new one. The
resolution is to lock first and *then* ask whether the descriptor and the path
still agree — lines 423 to 434 — and to retry from the open when they do not.
Eight attempts, and the eighth failure is a refusal naming the path and the
count, not a ninth attempt. The record's word for this is *fails closed*, and the
alternative it rules out is looping forever against an adversarial or merely
unlucky filesystem.

The inner loop is the handoff. `flock` is called with `LOCK_NB` every time, so
the call never blocks in the kernel; `EWOULDBLOCK` or `EAGAIN` means *someone
holds it*, and anything else is a real error and is returned with context. On
first contention it reports once — `reported_contention` is set outside the outer
loop, so eight attempts still produce one diagnostic — then it checks the
deadline and sleeps. A timeout is a `bail!` and nothing else: the record insists
*a timeout performs no tree access or epoch rewrite*, and this function returns
before its caller can do either.

The write-mode branch at 379 is small and is the reason `LockMode` carries more
than a `c_int`. A shared reader opens read-only; only an exclusive writer adds
`write`, `create`, `truncate(false)` and mode `0o600`. An admitted `grove-llm`
operation therefore cannot create the epoch file, which means a missing epoch
file is a missing driver rather than a file some reader conjured.

The comment at 381 to 386 is the block's most useful piece of maintenance prose
and is worth reading as an instruction rather than an observation. `truncate(false)`
is explicit *because* it is the default, so that a reader — and clippy's
`suspicious_open_options` — can see that not truncating was chosen. The comment
even says why the lint does not currently fire: clippy cannot follow a builder
split across statements. That is a warning to a future refactor, not a claim
about today, and a chained rewrite that dropped the explicit `truncate(false)`
would be silently wrong in exactly the way the next function's comment describes
at length.

<!-- fragment «lease-acquire-lease-file-with-hook» owner="one-per-working-tree" source="crates/grove-loop/src/driver_lease.rs" lines="445-498" parent="lease-and-epoch" -->
````rust

fn acquire_lease_file_with_hook(
    path: &Path,
    worktree_root: &Path,
    mut after_lock: impl FnMut(usize, &Path) -> Result<()>,
) -> Result<(File, FileIdentity)> {
    for attempt in 1..=IDENTITY_RETRY_LIMIT {
        // `truncate(false)` is load-bearing, not decoration. This open happens
        // *before* `lock_exclusively_nonblocking` below, so it runs while the
        // incumbent lease holder may still own the file. Truncating here would
        // destroy a live holder's record before we know whether we can even take
        // the lock — and on the path where the lock attempt then fails, we would
        // have wrecked the record of a lease we do not hold. The reader at
        // `probe_live_lease_with_post_unlock_hook` parses that record, so an
        // emptied file also reads as a corrupt lease rather than an absent one.
        //
        // Truncation is deliberately deferred to `write_record`, which does its
        // own `set_len(0)` + rewind *after* the lock is held. Stating the
        // behaviour explicitly is what clears `suspicious_open_options`; the
        // semantics are unchanged, since `create` alone never truncated.
        let file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .truncate(false)
            .mode(0o600)
            .open(path)
            .with_context(|| format!("opening driver lease file {}", path.display()))?;
        ensure_close_on_exec(file.as_raw_fd())?;
        lock_exclusively_nonblocking(&file, worktree_root)?;
        after_lock(attempt, path).context("running driver lease post-lock check")?;

        let descriptor_identity = FileIdentity::from_metadata(
            &file
                .metadata()
                .context("reading locked driver lease identity")?,
        );
        let path_identity = FileIdentity::from_metadata(
            &fs::metadata(path)
                .with_context(|| format!("reading driver lease path {}", path.display()))?,
        );
        if descriptor_identity == path_identity {
            return Ok((file, descriptor_identity));
        }
        if attempt == IDENTITY_RETRY_LIMIT {
            bail!(
                "driver lease path {} was replaced during acquisition {} times",
                path.display(),
                IDENTITY_RETRY_LIMIT
            );
        }
    }
    bail!("driver lease acquisition exhausted its bounded retries")
}
````
<!-- /fragment -->

The lease's own acquisition, and the same outer-loop shape without the inner one:
the lease lock never waits. `lock_exclusively_nonblocking` either takes it or
refuses, because a second driver is a human error to report rather than a
handoff to await.

Its opening comment is fourteen lines for a two-word argument, and each line is
necessary. The mechanism is `truncate(false)`; the line it is enforced
on is 469; the failure it prevents is **destroying a live incumbent's lease
record before knowing whether this process can take the lock at all** — and the
comment names the compounding case, that on the path where the lock attempt then
fails, the process has wrecked the record of a lease it does not hold. The clause
it keeps is the record's *leftover bytes carry no ownership or cleanup
obligation* read in the only direction that has teeth: bytes carry no ownership,
but they do carry **identity**, and a liveness probe that finds an emptied file
cannot tell a corrupt lease from an absent one.

**And that last sentence once named a function this workspace does not contain.**
The comment as this chapter first read it said *the reader at
`probe_lease_holder` parses that record*, and there was no `probe_lease_holder`
anywhere in the repository: searching for it returned that comment and nothing
else, while the same search for `probe_live_lease` returned five sites in the
same trees. The reader the sentence describes is
`probe_live_lease_with_post_unlock_hook`, a hundred and eighty lines further
down, whose lines 655 and 656 do exactly what the sentence claims — `parse_process_record` over
`read_record` — and whose failure on an emptied file is *missing worktree-device
field*, a corruption diagnostic rather than an absence. So **the argument was
sound and only the address was wrong**, which is the same shape chapter 6 met in
`task_tree.rs`'s reference to a module named `llm_cli`.

The name was wrong when it was written rather than outrun by a rename: the commit
that created this file wrote the comment with it, over a version that already
defined `probe_live_lease` and its hooked twin and no third probe, and the
pre-move copy of the file carries the identical sentence. `lease-stale-reader-name-k169`
wrote the real reader's name into the block above, and because the longer name
re-flowed inside the same three comment lines the file is still 1,383 lines — so
no ownership range, manifest `lines` value or fragment range on this page moved.

**Nothing in this repository could have caught it.** These are plain `//` lines
inside a function body, and `cargo doc --no-deps --document-private-items` reads
only `///` and `//!`. It reports twenty-six warnings over this crate and none
over this file, and that clean result is honest about the seventy lines it can see and
silent about the thirty-three it cannot. A comment written in `//` is checked by
reading it, and by nothing else.

<a id="the-lock-that-fails-immediately"></a>
## The lock that fails immediately, and the message a human reads


The lease's lock is one call and one classification, and it is where a second
driver meets the only sentence about this whole protocol that a human is ever
meant to read.

<!-- fragment «lease-lock-exclusively» owner="one-per-working-tree" source="crates/grove-loop/src/driver_lease.rs" lines="499-516" parent="lease-and-epoch" -->
````rust

fn lock_exclusively_nonblocking(file: &File, worktree_root: &Path) -> Result<()> {
    let result = unsafe { libc::flock(file.as_raw_fd(), libc::LOCK_EX | libc::LOCK_NB) };
    if result == 0 {
        return Ok(());
    }
    let error = std::io::Error::last_os_error();
    if matches!(
        error.raw_os_error(),
        Some(code) if code == libc::EWOULDBLOCK || code == libc::EAGAIN
    ) {
        bail!(
            "another Grove driver already owns {}; the existing Grove driver must stop before this one can start",
            worktree_root.display()
        );
    }
    Err(error).with_context(|| format!("locking driver lease for {}", worktree_root.display()))
}
````
<!-- /fragment -->

Eighteen lines, one `flock`, and one sentence of user interface. The record's
whole rule is here: *a second driver fails immediately.* Not queued, not retried,
not waited out — `LOCK_NB` and a refusal.

The three-way classification is what makes the refusal usable. Success returns.
`EWOULDBLOCK` or `EAGAIN` is the expected contention and becomes a sentence
written for a human rather than for a log: *another Grove driver already owns
&lt;worktree&gt;; the existing Grove driver must stop before this one can start.* It
names the working tree in the spelling the workspace resolved to, which is the
one thing the human needs and the one thing they cannot get from the error class.
Any other errno is a genuine failure — a filesystem that does not support
`flock`, a descriptor problem — and is returned with its own context rather than
being reported as contention, because telling someone another driver holds the
lease when in fact the filesystem cannot lock would send them looking for a
process that does not exist.

The failure this prevents is the record's first rejected option: *keep the status
quo with no lifetime owner — rejected because two bare drivers can select and
launch the same work or consume one another's completion signals.* Two drivers
over one grove is not a merge conflict; it is two sessions launched for the same
leaf, and a signal file that one driver wrote and the other reaped.

The `worktree_root` argument exists only for that message. It is threaded through
`acquire_lease_file` and `acquire_lease_file_with_hook` for no other purpose,
which is why both take a parameter neither of them reads.

<a id="every-descriptor"></a>
## Every descriptor, and the five opens


One helper stands behind every descriptor this block opens, and it is called
immediately after each open rather than once at the end.

<!-- fragment «lease-close-on-exec» owner="one-per-working-tree" source="crates/grove-loop/src/driver_lease.rs" lines="517-531" parent="lease-and-epoch" -->
````rust

fn ensure_close_on_exec(descriptor: RawFd) -> Result<()> {
    let flags = unsafe { libc::fcntl(descriptor, libc::F_GETFD) };
    if flags == -1 {
        return Err(std::io::Error::last_os_error()).context("reading descriptor flags");
    }
    if flags & libc::FD_CLOEXEC == 0 {
        let result = unsafe { libc::fcntl(descriptor, libc::F_SETFD, flags | libc::FD_CLOEXEC) };
        if result == -1 {
            return Err(std::io::Error::last_os_error())
                .context("marking driver descriptor close-on-exec");
        }
    }
    Ok(())
}
````
<!-- /fragment -->

Fifteen lines that read a descriptor's flags, set `FD_CLOEXEC` if it is missing,
and report either `fcntl` failure with its own context. Rust already sets
`O_CLOEXEC` on files it opens, so on the common path the `if` is not taken; the
function exists because *already true* and *checked* are different guarantees,
and this is the one the record makes.

The clause is one sentence — *every descriptor is close-on-exec* — and the
rejected option beneath it is what gives it force: *let the configured command
inherit the driver-lock descriptor — rejected because an opaque harness may pass
it to descendants that outlive the session, wedging the working tree after the
foreground child exits.* The failure is not theoretical for this crate. The whole
loop is a driver spawning an opaque configured harness; if the lease descriptor
survived that `exec`, and the harness handed it to a background process, the lock
would be held by something grove never launched and cannot reap, and every later
`grove` in that tree would refuse to start with no live driver to point at.

**The guarantee is worth stating as an enumeration rather than as a slogan.**
This block opens five file descriptors, and four of them are marked:

| Opened at | What it is | Marked at |
|---:|---|---:|
| 157 | the working-tree root directory | 163 |
| 389 | the session epoch file | 391 |
| 471 | the driver lease file | 473 |
| 534 | `/dev/urandom` | — |
| 652 | the lease file, for a liveness probe | 654 |

The fifth is the randomness source, and its absence is a scope rather than a gap:
`random_nonce` opens it, reads sixteen bytes and drops it inside its own body,
and no spawn happens anywhere between. The two descriptors that can still be open
when the driver `exec`s a session are the two the `DriverLease` retains — and
those are exactly the two chapter 17's `acquired_driver_descriptors_are_close_on_exec`
reads back with `F_GETFD`, by name, after a real acquisition. The record's
*every* is a claim about descriptors the driver holds, and the code is tighter
than the sentence rather than looser than it.

<a id="the-nonce-and-the-encodings"></a>
## The nonce, and the two encodings


Three encoders and one reader sit between the protocol and the bytes on disk.
The nonce comes first, in two functions that are together fifteen lines long and
carry the block's largest unargued decision.

<!-- fragment «lease-random-nonce» owner="one-per-working-tree" source="crates/grove-loop/src/driver_lease.rs" lines="532-540" parent="lease-and-epoch" -->
````rust

fn random_nonce() -> Result<[u8; 16]> {
    let mut source = File::open("/dev/urandom").context("opening OS randomness source")?;
    let mut nonce = [0_u8; 16];
    source
        .read_exact(&mut nonce)
        .context("reading 128-bit driver nonce from OS randomness source")?;
    Ok(nonce)
}
````
<!-- /fragment -->

Its rendering is separate, and is the only spelling the parser below will
accept.

<!-- fragment «lease-hex-nonce» owner="one-per-working-tree" source="crates/grove-loop/src/driver_lease.rs" lines="541-548" parent="lease-and-epoch" -->
````rust

fn hex_nonce(nonce: [u8; 16]) -> Result<String> {
    let mut rendered = String::with_capacity(32);
    for byte in nonce {
        write!(&mut rendered, "{byte:02x}").context("rendering 128-bit driver nonce")?;
    }
    Ok(rendered)
}
````
<!-- /fragment -->

Sixteen bytes read from `/dev/urandom` with `read_exact`, rendered as
thirty-two lowercase hex characters. Neither function has a doc comment and
neither needs one; what they are *for* is entirely in the record, and it is the
part of this chapter that is a policy decision rather than a mechanism.

*Each driver writes a fresh 128-bit nonce from the operating system's
cryptographic randomness source to the lease record… Neither value is derived
from a PID, clock, address, iteration counter, or task key.* The mechanism is
`read_exact` on `/dev/urandom` and the failure it prevents is **predictability**:
a nonce a second process could compute is not an identity, and every one of the
listed alternatives is computable by anything running on the same machine. The
`read_exact` rather than `read` matters for the same reason — a short read that
went unnoticed would leave part of the nonce zero, and a partially-zero nonce is
drawn from a much smaller space than 2^128.

The record is unusually candid about what this buys and what it costs: *signal
paths are not reused intentionally… after cleanup there is no durable tombstone,
so cross-restart nonce or path reuse is not literally impossible; the accepted
probability is at most one in 2^128 per independent draw. This statistical
freshness is the explicit cost of keeping grove generation out of durable workflow
state.* The rejected option it is paid for is *persist a grove-generation
identifier under `.grove/` or add it to every stable handle* — which would have
made uniqueness exact and put opaque lifecycle state into the artifact tree, in a
book whose subject is a crate that keeps state out of it.

<!-- fragment «lease-encode-path» owner="one-per-working-tree" source="crates/grove-loop/src/driver_lease.rs" lines="549-556" parent="lease-and-epoch" -->
````rust

fn encode_path(path: &Path) -> Result<String> {
    let mut rendered = String::with_capacity(path.as_os_str().as_bytes().len() * 2);
    for byte in path.as_os_str().as_bytes() {
        write!(&mut rendered, "{byte:02x}").context("encoding working-tree path")?;
    }
    Ok(rendered)
}
````
<!-- /fragment -->

Decoding is the longer half, because it is the half that reads a file another
process wrote.

<!-- fragment «lease-decode-path» owner="one-per-working-tree" source="crates/grove-loop/src/driver_lease.rs" lines="557-569" parent="lease-and-epoch" -->
````rust

fn decode_path(value: &str) -> Result<PathBuf> {
    if value.len() % 2 != 0 {
        bail!("working-tree path hex has odd length");
    }
    let mut bytes = Vec::with_capacity(value.len() / 2);
    for offset in (0..value.len()).step_by(2) {
        let byte = u8::from_str_radix(&value[offset..offset + 2], 16)
            .context("decoding working-tree path hex")?;
        bytes.push(byte);
    }
    Ok(PathBuf::from(std::ffi::OsString::from_vec(bytes)))
}
````
<!-- /fragment -->

The path codec, and it is a codec rather than a formatter because `Path` on Unix
is bytes. `encode_path` goes through `as_os_str().as_bytes()`, which is the
`OsStrExt` import earning its place, and `decode_path` comes back through
`OsString::from_vec`. Neither goes near `to_str`, `to_string_lossy` or UTF-8
validation, so a working tree whose name is not valid UTF-8 round-trips exactly
like any other.

`decode_path` is the only parser in the block that checks its input's *shape*
before its content — an odd-length hex string is refused at line 559 before any
`from_str_radix` runs — and the reason is that `step_by(2)` over an odd-length
string would slice past the end and panic. A refusal is the correct answer for a
file two processes write and any process can corrupt; a panic in a driver holding
a lease is not.

<a id="the-record-parsers"></a>
## The record parsers, and what they refuse


Three functions read what the two writers wrote, and a fourth composes them into
the epoch's state machine. The first is the field accessor both records go
through.

<!-- fragment «lease-record-field» owner="one-per-working-tree" source="crates/grove-loop/src/driver_lease.rs" lines="570-581" parent="lease-and-epoch" -->
````rust

fn record_field<'a>(record: &'a str, name: &str) -> Result<&'a str> {
    let prefix = format!("{name}=");
    let mut values = record.lines().filter_map(|line| line.strip_prefix(&prefix));
    let value = values
        .next()
        .with_context(|| format!("missing {name} field"))?;
    if values.next().is_some() {
        bail!("duplicate {name} field");
    }
    Ok(value)
}
````
<!-- /fragment -->

Twelve lines, and they are the strictest thing in the block. `record_field`
refuses a **duplicate** field as firmly as a missing one, and that is not
defensive habit: a `key=value` format with no framing is exactly the format in
which a partially-overwritten file produces two `nonce=` lines, one from each
writer. Taking the first and ignoring the rest would let a torn write read as a
valid record for whichever driver happened to have written first.

There is no torn write on the intended path — every write happens under the
exclusive lock, after `set_len(0)` — so this is a check for the case the record
declines to defend against: *nor does Grove defend against another process
deleting or replacing files in the VCS administration area; that is
repository-control corruption.* Not defending is not the same as not detecting,
and the difference is the whole of this function.

<!-- fragment «lease-parse-process-record» owner="one-per-working-tree" source="crates/grove-loop/src/driver_lease.rs" lines="582-604" parent="lease-and-epoch" -->
````rust

fn parse_process_record(record: &str) -> Result<ProcessRecord> {
    let device = record_field(record, "worktree-device")?
        .parse::<u64>()
        .context("parsing working-tree device")?;
    let inode = record_field(record, "worktree-inode")?
        .parse::<u64>()
        .context("parsing working-tree inode")?;
    let worktree_root = decode_path(record_field(record, "worktree-path-hex")?)?;
    let nonce = record_field(record, "nonce")?;
    if nonce.len() != 32
        || !nonce
            .bytes()
            .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte))
    {
        bail!("driver nonce is not 128-bit lowercase hex");
    }
    Ok(ProcessRecord {
        worktree_identity: FileIdentity { device, inode },
        worktree_root,
        nonce: nonce.to_string(),
    })
}
````
<!-- /fragment -->

Four fields parsed and one validated. The nonce check at 592 to 598 is the
interesting one: thirty-two characters, each an ASCII digit or `a`–`f`. It is not
checking that the nonce is *correct* — nothing could — but that it is
**well-formed**, and specifically that it is in the one spelling `hex_nonce`
produces. Uppercase hex is refused. That is deliberate rather than careless,
because the comparison that matters later is a string equality between a parsed
record and a parsed epoch, and two spellings of one nonce would compare unequal
and turn a live driver into a stale one.

The three `.context` calls give a reader the field name rather than a `ParseIntError`,
which is the difference between *parsing working-tree device* and *invalid digit
found in string* in the one place a human is already confused about why their
loop will not start.

<!-- fragment «lease-read-record» owner="one-per-working-tree" source="crates/grove-loop/src/driver_lease.rs" lines="605-613" parent="lease-and-epoch" -->
````rust

fn read_record(file: &mut File, label: &str) -> Result<String> {
    file.seek(SeekFrom::Start(0))
        .with_context(|| format!("rewinding {label}"))?;
    let mut record = String::new();
    file.read_to_string(&mut record)
        .with_context(|| format!("reading {label}"))?;
    Ok(record)
}
````
<!-- /fragment -->

Nine lines, and the `seek(SeekFrom::Start(0))` is the whole point. Every caller
of this function holds a descriptor that has been **locked, and possibly
written**, and a `File`'s cursor is wherever the last operation left it. Reading
without rewinding would return an empty string for a file this process had just
written and would parse as *missing worktree-device field* — a corrupt-lease
diagnostic for a lease this driver holds correctly.

<!-- fragment «lease-read-epoch-record» owner="one-per-working-tree" source="crates/grove-loop/src/driver_lease.rs" lines="614-635" parent="lease-and-epoch" -->
````rust

fn read_epoch_record(file: &mut File) -> Result<EpochRecord> {
    let record = read_record(file, "session epoch record")?;
    let process = parse_process_record(&record)?;
    let signal_path = match record_field(&record, "state")? {
        "inactive" => {
            if record
                .lines()
                .any(|line| line.starts_with("signal-path-hex="))
            {
                bail!("inactive session epoch unexpectedly carries a signal path");
            }
            None
        }
        "active" => Some(decode_path(record_field(&record, "signal-path-hex")?)?),
        state => bail!("unknown session epoch state {state:?}"),
    };
    Ok(EpochRecord {
        process,
        signal_path,
    })
}
````
<!-- /fragment -->

The one place the epoch's state machine is read, and the two `bail!` arms are the
invariant `write_epoch_contents` maintains, checked from the other side. An
`inactive` record carrying a `signal-path-hex=` line is refused at 624 rather
than being read as inactive-with-a-leftover; an unrecognised `state` value is
refused at 629 rather than being defaulted. Both produce *stale* to the caller,
and the record's phrasing covers them together: *inactive, malformed, unlocked,
or mismatched epochs receive a stale-session diagnostic.*

Refusing rather than defaulting is the choice worth naming. A record whose state
this build does not recognise was written by a grove that is not this one, and
the conservative reading — treat what you do not understand as *no live driver* —
would admit an operation into a tree a newer driver is holding. The refusal fails
in the safe direction: the operation stops, and a human reads why.

<a id="the-liveness-probe"></a>
## The liveness probe, and the race it closes


Liveness is asked by trying to take a lock this process does not want. The
production entry passes an inert hook; the work is in the form below it.

<!-- fragment «lease-probe-live-lease» owner="one-per-working-tree" source="crates/grove-loop/src/driver_lease.rs" lines="636-639" parent="lease-and-epoch" -->
````rust

fn probe_live_lease(control_dir: &Path, epoch: &EpochRecord, operation: &str) -> Result<()> {
    probe_live_lease_with_post_unlock_hook(control_dir, epoch, operation, |_| Ok(()))
}
````
<!-- /fragment -->

The hooked form is sixty-four lines and one attempt loop, and every line of it is
ordered against a race.

<!-- fragment «lease-probe-with-hook» owner="one-per-working-tree" source="crates/grove-loop/src/driver_lease.rs" lines="640-703" parent="lease-and-epoch" -->
````rust

fn probe_live_lease_with_post_unlock_hook(
    control_dir: &Path,
    epoch: &EpochRecord,
    operation: &str,
    mut after_successful_probe: impl FnMut(&Path) -> Result<()>,
) -> Result<()> {
    let lease_path = control_dir.join(LEASE_FILE_NAME);
    for attempt in 1..=IDENTITY_RETRY_LIMIT {
        let mut lease_file = OpenOptions::new()
            .read(true)
            .write(true)
            .open(&lease_path)
            .with_context(|| format!("opening driver lease file {}", lease_path.display()))?;
        ensure_close_on_exec(lease_file.as_raw_fd())?;
        let lease_record =
            parse_process_record(&read_record(&mut lease_file, "driver lease record")?)?;
        let probe = unsafe { libc::flock(lease_file.as_raw_fd(), libc::LOCK_EX | libc::LOCK_NB) };
        let probe_error = (probe != 0).then(std::io::Error::last_os_error);
        if probe == 0 {
            let unlock = unsafe { libc::flock(lease_file.as_raw_fd(), libc::LOCK_UN) };
            if unlock != 0 {
                return Err(std::io::Error::last_os_error())
                    .context("releasing successful driver lease liveness probe");
            }
            after_successful_probe(&lease_path)
                .context("running driver lease post-unlock check")?;
        }
        let descriptor_identity = FileIdentity::from_metadata(
            &lease_file
                .metadata()
                .context("reading probed driver lease identity")?,
        );
        let path_identity = FileIdentity::from_metadata(
            &fs::metadata(&lease_path)
                .with_context(|| format!("reading driver lease path {}", lease_path.display()))?,
        );
        if descriptor_identity != path_identity {
            if attempt == IDENTITY_RETRY_LIMIT {
                bail!(
                    "driver lease path {} was replaced during liveness probe {} times",
                    lease_path.display(),
                    IDENTITY_RETRY_LIMIT
                );
            }
            continue;
        }
        if lease_record != epoch.process {
            bail!("driver lease record does not match the admitted session epoch");
        }
        if probe == 0 {
            bail!("driver lease is unlocked");
        }
        let error = probe_error.expect("failed flock records an OS error");
        if matches!(
            error.raw_os_error(),
            Some(code) if code == libc::EWOULDBLOCK || code == libc::EAGAIN
        ) {
            return Ok(());
        }
        return Err(error).with_context(|| format!("probing driver lease for {operation}"));
    }
    bail!("driver lease liveness probe exhausted its bounded retries")
}
````
<!-- /fragment -->

The most counter-intuitive function in the block: it takes a lock **in order to
discover that taking it was bad news**. The record states the inversion plainly —
*a successful probe is closed and released immediately and means no driver is
live; contention plus a matching lease record is the liveness hint.*

Read in order, one attempt does five things. It opens the lease file and parses
its record (649–656). It tries `LOCK_EX | LOCK_NB` and remembers both the result
and, on failure, the errno (657–658). **If the lock succeeded** it unlocks
immediately, before doing anything else, and runs the post-unlock seam (659–667)
— holding a lock it acquired by accident, even for the length of the checks
below, would make this probe indistinguishable from a driver to any *other* probe
running at the same moment. Then it compares descriptor identity against path
identity and retries the whole attempt on a mismatch (668–686). Only then does it
judge: a record that does not match the admitted epoch is a refusal (688), a lock
that *was* free is *driver lease is unlocked* (691), and contention with a
matching record is the single `Ok(())` in the function (729).

The ordering of those last three is the argument. Identity is checked before the
record, and the record before the lock verdict, so a probe that raced a
replacement retries rather than reporting on bytes it no longer trusts, and a
probe of the *wrong* lease says so rather than reporting that lease's liveness.
The `expect` at line 693 is the one place this block asserts rather than checks,
and it is sound by construction: `probe_error` is `Some` exactly when `probe != 0`,
and that branch is only reached after `probe == 0` has already returned.

**What the probe cannot do is what the shared guard is for.** A liveness answer
is stale the moment it is given — the driver could die immediately afterwards.
The record closes that with a lifetime rather than a check: *the operation retains
its shared epoch guard through tree access, which closes the probe's race: if the
driver dies just after the probe, a replacement driver cannot invalidate the
epoch until the admitted operation finishes.* The probe never becomes true; it
becomes **irrelevant**, because a replacement cannot rotate the epoch out from
under an operation that still holds the shared lock. That is why
`SessionEpochGuard` carries a `File` nobody reads, and why `admit_ambient_session`
returns a guard rather than a boolean.

<a id="admission"></a>
## Admission, and the environment read exactly once


Admission is four functions, and the split between the first three is a
testability argument rather than a decomposition. The public entry resolves the
ambient context and hands it on.

<!-- fragment «lease-admit-ambient-session» owner="one-per-working-tree" source="crates/grove-loop/src/driver_lease.rs" lines="704-721" parent="lease-and-epoch" -->
````rust

/// Admit one agent-side operation when it carries a live loop-control context.
///
/// The returned guard owns the shared epoch lock and must remain alive through
/// the operation's separately acquired Tree access guard. With no ambient
/// signal path this is a manual command and no driver epoch is required.
///
/// # Errors
///
/// A stale session: an epoch that is inactive, one belonging to another working
/// tree, one whose channel is not the ambient one, or one whose driver is no
/// longer alive.
pub fn admit_ambient_session(
    path: &Path,
    operation: &str,
) -> Result<Option<SessionEpochGuard>, crate::Error> {
    Ok(admit_session(path, operation, ambient_signal_path())?)
}
````
<!-- /fragment -->

The resolution itself is one line, and its doc comment explains why it is a
function at all.

<!-- fragment «lease-ambient-signal-path» owner="one-per-working-tree" source="crates/grove-loop/src/driver_lease.rs" lines="722-732" parent="lease-and-epoch" -->
````rust

/// The loop-control context this process was launched into, if any.
///
/// Reading the environment is the *whole* of this function, and the only place
/// in the admission path that touches it — [`admit_session`] takes the resolved
/// path as an argument instead. That split is what lets admission be tested
/// without a unit test writing a process-global that production code in a
/// parallel sibling test is reading at the same moment.
fn ambient_signal_path() -> Option<PathBuf> {
    signal_path_from(std::env::var_os("GROVE_SIGNAL_FILE"))
}
````
<!-- /fragment -->

Classification is separated again, because the value it classifies is one this
repository deliberately sets to something surprising.

<!-- fragment «lease-signal-path-from» owner="one-per-working-tree" source="crates/grove-loop/src/driver_lease.rs" lines="733-740" parent="lease-and-epoch" -->
````rust

/// Classify a loop-control value as ambient context or none. Empty is *none*
/// rather than a degenerate path: `.cargo/config.toml` force-clears the variable
/// to the empty string rather than unsetting it (`tests/env_hygiene.rs` owns
/// that claim), so empty is the value every cargo-launched `grove-llm` sees.
fn signal_path_from(value: Option<OsString>) -> Option<PathBuf> {
    value.filter(|value| !value.is_empty()).map(PathBuf::from)
}
````
<!-- /fragment -->

Three small functions, and the split between them is a testability argument the
source states outright: *reading the environment is the whole of this function,
and the only place in the admission path that touches it — `admit_session` takes
the resolved path as an argument instead. That split is what lets admission be
tested without a unit test writing a process-global that production code in a
parallel sibling test is reading at the same moment.*

The claim is checkable and holds. In these 819 lines there is exactly **one**
`std::env` expression, at line 731. The three others in the file — at 854, 855
and 870 — are inside the test module, which is chapter 17's. So the environment
enters this block through one line, and every path below that line takes a value.

`signal_path_from` classifies, and its doc comment carries a fact about this
repository rather than about the code: `.cargo/config.toml` force-clears
`GROVE_SIGNAL_FILE` to the empty string rather than unsetting it, so **empty is
the value every cargo-launched `grove-llm` sees**, and treating it as a degenerate
path would make every test-suite invocation claim session-epoch authority. The
`filter` at line 739 is what stops that.

The comment credits `tests/env_hygiene.rs` with owning the claim, and the file it
means is `crates/grove/tests/env_hygiene.rs` — the other crate's, not this one's,
whose `tests/` directory holds six files and none of that name. It does own it:
`the_suite_cannot_reach_a_live_loop_signal_file` asserts the observed value
`is_empty()`, and `both_guards_are_present_and_neither_subsumes_the_other` asserts
that the configuration entry carries `force = true` — without which an inherited
live value would win.

**One consequence for every measurement taken over this block.** Because cargo
force-clears the variable, `ambient_signal_path` can only ever return `None`
under the test suite, and every arm below it is therefore unreachable *under the
harness* rather than untested. That is a guard, not a gap, and chapter 15
established it by measurement over the identical pattern in `verbs.rs`: deleting
the corresponding fallback changed nothing — 560 tests, 549 passed, 11 failed,
identical to the control — while deleting the emptiness filter turned exactly
three tests red at the same total, which is what makes the first zero a reading
rather than a blind instrument.

<!-- fragment «lease-admit-session» owner="one-per-working-tree" source="crates/grove-loop/src/driver_lease.rs" lines="741-796" parent="lease-and-epoch" -->
````rust

/// Admission proper, with the ambient context already resolved.
fn admit_session(
    path: &Path,
    operation: &str,
    signal_path: Option<PathBuf>,
) -> Result<Option<SessionEpochGuard>> {
    let Some(signal_path) = signal_path else {
        return Ok(None);
    };
    let current_workspace = Workspace::resolve(path)?;
    let control_dir = signal_path.parent().with_context(|| {
        format!(
            "stale Grove session for {operation}: signal path has no control-directory parent: {}",
            signal_path.display()
        )
    })?;
    let epoch_path = control_dir.join(EPOCH_FILE_NAME);
    let mut epoch_file = acquire_epoch_file(&epoch_path, LockMode::Shared, operation)
        .with_context(|| format!("stale Grove session for {operation}"))?;
    let epoch = read_epoch_record(&mut epoch_file)
        .with_context(|| format!("stale Grove session for {operation}"))?;

    if epoch.process.worktree_root != current_workspace.root() {
        bail!(
            "wrong working tree for {operation}: session belongs to {}, command resolved {}",
            epoch.process.worktree_root.display(),
            current_workspace.root().display()
        );
    }
    let current_identity = FileIdentity::from_metadata(
        &fs::metadata(current_workspace.root()).with_context(|| {
            format!(
                "reading current working-tree identity {}",
                current_workspace.root().display()
            )
        })?,
    );
    if epoch.process.worktree_identity != current_identity {
        bail!("stale Grove session for {operation}: working-tree identity changed");
    }
    let Some(epoch_signal_path) = epoch.signal_path.as_deref() else {
        bail!("stale Grove session for {operation}: session epoch is inactive");
    };
    if epoch_signal_path != signal_path {
        bail!(
            "stale Grove session for {operation}: loop-control path does not match the active epoch"
        );
    }
    probe_live_lease(control_dir, &epoch, operation)
        .with_context(|| format!("stale Grove session for {operation}"))?;
    Ok(Some(SessionEpochGuard {
        _epoch_file: epoch_file,
        signal_path,
    }))
}
````
<!-- /fragment -->

Admission proper, and it is a sequence of five refusals with one early return, in
an order chosen so that each check can assume the previous one passed.

The early return is first and is the whole of the manual-command case: **no
ambient signal path, no epoch required.** The record's phrasing is *manual
commands without loop-control context retain their ordinary behavior*, and the
`Ok(None)` at line 749 is that sentence. A human typing `grove-llm pick` in a
terminal is not in a loop, has nothing to be stale against, and is admitted
without the file being read.

Then, in order:

1. **Resolve the current workspace** (751) and derive the control directory from
   the signal path's own parent (752–758). The epoch file is found *relative to
   the channel the caller was given*, not from the workspace — so a command
   carrying a signal path from another tree looks for that tree's epoch and is
   caught by the check below rather than silently reading the local one.
2. **Take the shared epoch lock and read the record** (759–762). This is the
   admission boundary the record names: *shared-guard acquisition is the
   admission boundary: an old call admitted before exclusive invalidation may
   finish and block handoff; calls beginning after invalidation fail against the
   inactive record or new nonce.* Everything after this line is inside the
   window; everything before it is not.
3. **Compare working trees, then working-tree identities** (763–781). Two checks
   over what looks like one fact, and they fail differently on purpose. A path
   mismatch is *wrong working tree*, and the record singles it out — *a wrong
   worktree receives its own location diagnostic* — because the human's mistake is
   locatable and the message names both trees. An identity mismatch under a
   matching path is *stale*, because the path is right and the directory behind it
   is not the one the driver pinned.
4. **Require an active epoch and the matching channel** (782–788). Inactive is
   stale; a different signal path is stale.
5. **Probe the lease** (790). Last, because it is the most expensive and the only
   one that opens a second file, and because the four checks above have already
   established that this is the right epoch to probe for.

Only then is a guard constructed, and it moves the epoch file into itself — which
is what makes the shared lock outlive this function. Dropping the return value
would release admission immediately; `grove-llm`'s CLI binds it and holds it
through its tree access, and that lifetime is the guarantee, not this function's
return value.

<a id="the-record-written-last"></a>
## The record written last


The lease's own serialiser closes the file, and its position there is not
alphabetical.

<!-- fragment «lease-write-record» owner="one-per-working-tree" source="crates/grove-loop/src/driver_lease.rs" lines="797-819" parent="lease-and-epoch" -->
````rust

fn write_record(
    file: &mut File,
    worktree_identity: FileIdentity,
    worktree_root: &Path,
    nonce: &str,
) -> Result<()> {
    file.set_len(0)
        .context("truncating previous driver lease record")?;
    file.seek(SeekFrom::Start(0))
        .context("rewinding driver lease record")?;
    write!(
        file,
        "worktree-device={}\nworktree-inode={}\nworktree-path-hex={}\nnonce={}\n",
        worktree_identity.device,
        worktree_identity.inode,
        encode_path(worktree_root)?,
        nonce
    )
    .context("writing driver lease record")?;
    file.flush().context("flushing driver lease record")
}

````
<!-- /fragment -->

The lease serialiser, and it closes the file for a reason: it is the **last**
write acquisition performs. `initialize_epoch_record` calls it after the epoch
record is installed, and this chapter's ordering section said why — the
predecessor's lease bytes stay intact until exclusive epoch handoff has
succeeded.

Four fields against the epoch record's five or six, in the same `key=value`
format and with the same `set_len(0)` then rewind under the lock. There is no
`state`, because a lease has no states: it is held or it is not, and that
question is answered by `flock` and never by these bytes. `parse_process_record`
reads both files, which is why the four fields it parses are exactly the four
this function writes — the lease record is a `ProcessRecord` and the epoch record
is a `ProcessRecord` with a state on top.

<a id="what-could-not-move-here"></a>
## What could not move

The book's question, asked of the one chapter whose answer is not about meaning.

**On the way in — the names.** This block owns no grammar, and that absence is
the answer rather than a gap in it. Its two file names are constants, its record
format is `key=value` lines nothing parses but itself, and its one identifier
with any structure — the 128-bit nonce — is validated for *spelling* and never for
sense. Compare chapters 2 to 4, where 1,712 lines exist because a name must be
canonical: `format(parse(f)) == f`, a conformance kit, four verdicts. Here
canonicity is free, because nothing round-trips through a human and no entity
occupies two files. What the layer owns on the way in is not a grammar but a
**word**: the namespace `grove`, one string handed to a seam that could not have
guessed it.

**On the way through — the preconditions.** This is where the block spends
everything. It checks what no library beneath it can see — that the descriptor it
locked still names the path it opened, that the record on disk belongs to the
epoch it was admitted under, that the working tree is the same directory it was
when the lease was taken — and it checks each one against a snapshot it is
holding rather than a path it could look up again. The cost is the one the
outcome names: the check must run against the same snapshot the operation then
plans from, or it is a race with a name. Three retry loops at eight attempts, a
probe that unlocks before it judges, and a guard that outlives the function that
built it are all one answer to that.

**On the way out — the policy.** Six values are chosen here that nothing beneath
could have defaulted: the namespace, two file names, a retry count, a handoff
bound and a poll interval. None of them is configurable, and the decision record
says so — *the bound, clock, control-path resolver, and randomness source are
internal test seams, not user configuration.* The layer above owns which harness
runs; the layer below owns where an untracked directory may live; the choice of
*how long to wait for a predecessor before giving up* belongs to neither, and it
is thirty seconds because someone decided.

**And the thing this chapter is really for.** The rest of the crate can be
reconstructed from the tree: `pick` re-derives position, an outcome is a filename
infix, a node is a directory, and a driver that restarts finds everything it knew
still written down. That is the crate's second thesis, and chapter 13 is where it
is nearly literally true. Here it is false, deliberately and expensively. A lease
is a fact about one process on one machine at one moment; it cannot be committed,
cannot be re-derived, and cannot be recovered from anything left on disk after
the process is gone. The 819 lines above are what it costs to hold a fact the tree
must not hold — and the reason they are worth it is in the record's very first
rejected option, which is that without them two bare drivers can select and launch
the same work.

[Previous: The twelve verbs, and the two that are not](15-the-verbs.md) | [Contents](README.md) | [Next: Which calls the lease admits](17-the-epoch.md)
