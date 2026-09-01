# lossy-path-rendering-k66

## Goal

Stop `jj-workspace` from silently converting a path it cannot render into a
scope that matches nothing, and cover the one branch of `relative` no test
reaches — landed as a corpus change the book contract permits.

## Context

- `crates/jj-workspace/src/lib.rs:265-272` renders a workspace-relative path into
  a jj fileset by joining its components with `/` and converting each with
  `to_string_lossy`. That is the crate's only lossy conversion, and it has to
  happen somewhere: a fileset is an argument in a command line, and jj's
  arguments are text.
- **The consequence is silent, and it is the one shape this crate refuses
  everywhere else.** A path whose bytes are not valid UTF-8 becomes a string
  containing replacement characters, so the fileset names a file that does not
  exist. Measured on jj 0.44.0 against a fileset that matches nothing:
  `jj file list` prints nothing on stdout and exits 0, and `jj commit` warns on
  stderr, takes an **empty** commit and exits 0. So `is_tracked` answers `false`
  for a file that is tracked, and `commit` returns a `Commit` naming an empty
  change while the caller believes its work is committed. No refusal is raised
  at any point.
- The crate's own rule is the argument for changing it. `control_dir` refuses a
  namespace it cannot honour *rather than quietly reinterpreting it*
  (`lib.rs:130-133`), and `validated_namespace` refuses `'\0'` precisely so an
  obscure operating-system error becomes a refusal that names the problem. A path
  that cannot be rendered is the same class and is handled the other way.
- **The reachability is platform-dependent and that is not a reason to leave
  it.** APFS rejects a filename that is not valid UTF-8, so the case is hard to
  construct on macOS; Linux filesystems accept arbitrary bytes, and grove runs
  there. Whether to refuse (a new `Refusal` case, or `Refusal::outside_workspace`'s
  sibling) or to pass the path to jj as bytes is the decision this leaf makes;
  refusing is the shape consistent with the rest of the crate, and passing bytes
  is not available while the fileset is a `&str` in an argv array.
- **A second, adjacent gap belongs in the same commit.** `relative`'s fallback
  branch (`lib.rs:245-259`) canonicalises the *parent* so a path the caller has
  just deleted still resolves. The crate's suite reaches that branch through no
  test: `a_deletion_is_committable_after_the_path_is_gone` passes a relative
  path, which takes the textual `strip_prefix` branch instead. The behaviour was
  measured while drafting `scope-and-commit-k59` — an absolute path through a
  symlinked ancestor whose leaf is deleted commits the deletion, and one whose
  *parent* is also deleted is refused with `UnresolvablePath`, whose message
  talks about broken symlinks rather than about a removed directory — and
  `05-scope-and-commit.md` records both. Both belong to the same two functions
  and the same fragments.

## Done when

- A path that cannot be rendered as a fileset is refused rather than turned into
  a scope that matches nothing, with a message naming the path and the reason,
  and a test asserts it (`#[cfg(unix)]`, constructing the name with
  `OsStr::from_bytes`).
- The fallback branch of `relative` has a test: an absolute path reached through
  a symlinked ancestor, whose leaf has been deleted, commits the deletion.
- If the `UnresolvablePath` message is changed to fit the removed-parent case,
  the change lands here too; if it is left alone, the reason is recorded.
- The corpus-freeze rule in `.grove/BRIEF.md` is honoured: **one commit** carries
  the source change, every affected book ledger and page, and a green validator
  run over every book it touched. `lib.rs:146-274` is the
  `scope-tracking-and-commit` block, owned by chapter 5 of the `jj-workspace`
  book and split into fourteen literal fragments — a line added anywhere in
  `relative` moves the boundaries of `relative-absolute`,
  `relative-strip-or-canonical-parent`, `relative-root-is-not-a-scope` and
  `relative-render`, their fragment-index rows, and the block's own range. A new
  `Refusal` case moves `refusal.rs` and chapter 6 with it.
- `bash scripts/check.sh` passes.

## Notes

**Placed after every crate book deliberately, for the reason `jj-docs-url-k64`
and `jj-owned-names-k65` carry.** Editing a byte of a frozen root while a book
that quotes it is being written invalidates the ranges the freeze protects. Run
this once the books that own `lib.rs` and `refusal.rs` exist and can be re-proved
in the same commit.

**This is not a request to make paths non-UTF-8-safe throughout.** The crate
speaks to jj through a command line and will keep doing so. What is wrong is not
the conversion; it is that the conversion cannot fail.
