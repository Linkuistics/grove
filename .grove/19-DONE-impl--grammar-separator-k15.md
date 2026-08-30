# grammar-separator-k15

## Goal

Land the `--` separator between kind and slug, rename every live leaf in this
tree onto it, and **reinstall the binaries in the same session**. Rename and
reinstall are one step with no session between them.

## Context

`docs/specs/module-decomposition.md`, decision 3, and its two requirement
scenarios. The grammar becomes:

    NN-[DONE-|ABANDONED-]<kind>--<slug>-k<key>.md      a leaf
    NN-<slug>-k<key>                                    a node directory

The middle splits at the **first** `--`; neither the kind nor the slug may
contain one. Round-tripping holds, the permanent key stays the terminal token,
node names are untouched, and the kind token stays **byte-identical to the skill
suffix**, which is the property `open-kind-k20` and the plugin exist for.

**Why, and why now.** Once the kind is an open token, today's grammar has no
single parse: `design-decomposition` in the middle of a name reads as kind
`design` + slug `decomposition` **and** as kind `design-decomposition` + empty
slug, and a three-word kind makes it four ways deep. Today only matching against
the closed set resolves it — the very thing `open-kind-k20` removes. This is one
filename naming **two entries**, which is worse than the two-filenames-one-entry
case the store's canonicality obligation forbids, because what differs between
the readings is the **handle**.

Three alternatives are **rejected**, not open: an inner underscore for multi-word
kinds (the filename token and the skill name would then differ by a rule,
reintroducing the second source the open kind deletes); moving the kind after the
key (unseats the terminal-key rule resolution and the glossary both lean on, and
only relocates the delimiter problem); forbidding hyphens in slugs (renames just
as much for a worse read).

## Done when

- The parser and renderer accept **only** the new grammar. A task-shaped filename
  with no `--` between kind and slug is refused, and the refusal names both what
  is on disk and the canonical form.
- `format(parse(f)) == f` holds for a multi-word kind beside a multi-word slug —
  the spec's own scenario is `integrate-review-design` + `module-decomposition`.
  Add it as a test fixture by name.
- Every fixture, test and document that spells a task filename is updated.
- **The binaries are rebuilt and installed before the tree is renamed** (see the
  sequence below).
- Every live leaf in `.grove/` is renamed onto the new grammar, this leaf's own
  file included. Node directory names are unchanged. `DONE` leaves are renamed
  too — the outcome infix sits before the kind, so they wear the separator as
  well.
- `docs/adr/task-names-are-canonical.md` is **amended** for the separator; its
  migration clauses go, migration having been deleted at `delete-migration-k6`.
- `cargo test` and `cargo clippy --all-targets` clean; `CHANGELOG.md` updated.

## Notes

**This leaf must not stop half-way.** A session that renames the tree and stops
has wedged the loop. Run it in this order, and do not interleave:

This is the root brief's `### The cutover sequence`, and that section is the
authority; what follows is its instance for the grammar.

1. Land the parser change; `cargo test`; `cargo clippy --all-targets`.
2. `cargo build --release`, then put the two binaries where this machine's `PATH`
   resolves them. **They are installed by Homebrew** —
   `/opt/homebrew/bin/grove` and `/opt/homebrew/bin/grove-llm` are symlinks into
   `../Cellar/grove/<version>/bin/`. Overwriting the Cellar files is the direct
   route; `cargo install --path .` works **only** if `~/.cargo/bin` outranks
   `/opt/homebrew/bin` on `PATH`, which on this machine it does not — that trap
   is documented at `docs/ARCHITECTURE.md:1540` and `docs/USAGE.md:175`.
3. Prove the install **by behaviour**: `readlink -f "$(command -v grove-llm)"`
   names the file just written, and the new build parses a `--` name the old one
   refuses (a scratch `.grove` outside this tree is the cheap way to ask).
   `grove --version` still prints the unchanged workspace version and witnesses
   nothing — do not use it as the check.
4. Rename the tree **with the new binary in place**, then check it reads its own
   work: `grove-llm pick`, `grove-llm kind`, `grove-llm brief-chain`.
5. Retire this leaf **by its new path** and commit everything as one change.
6. **Do not run `grove-llm complete`.** End the session unsignalled; that is what
   stops the loop, and the commit message says the human restarts `grove`.

**The loop stops because this session declines to signal, not because a guard
catches the reinstall.** The running driver is the *old* build, loaded in memory,
and it would happily take another iteration against a tree it can no longer parse:
`report_build_pairing` prints and returns `()` (`src/loop_driver.rs:550-576`), and
`docs/USAGE.md:164-177` says it reports without refusing. What does stop the loop
is an unsignalled exit (`src/loop_driver.rs:49`). Say so in the commit message; the
human restarts `grove` and the new build takes over. Nothing needs recovering.

**Lands green in the suite**; what it breaks is the pairing between an installed
binary and a tree on disk, which is why the sequence above is part of the
deliverable rather than advice.

**A standing consequence starts here**: from this leaf onward, every later leaf
that changes the verb surface sessions invoke must reinstall in the same session.
The root brief records this.

## This leaf owns the release, and `.grove/FORMAT`'s deletion with it

Added by `delete-migration-k6`, which withdrew its own cutover and pushed the
consequence here.

**This is the first leaf that genuinely cannot defer its tree-visible half.** The
rename onto the new grammar *is* the deliverable, and the installed build cannot
parse the result — there is no *skip the tree half* option of the kind k6 found
for itself. So this leaf owns the first published release of the refactor: cut a
minor release, publish, update through Homebrew, per the root brief's corrected
`### The cutover sequence`. Do **not** overwrite a released version's Cellar
files, and before publishing run the new build read-only against every other live
grove on the machine, confirming it picks what each driver is already on.

**`.grove/FORMAT` is deleted here.** `delete-migration-k6` removed everything that
writes, reads or requires it, but deliberately left the file in this tree so the
installed 19.3.0 could keep driving — a stray `FORMAT` is a foreign entry every
reader ignores. Once this leaf's release is published the file has no reader on
any build, and deleting it is a one-line tree-visible change that belongs with the
rename: one deployment, one cutover.

## What this leaf found, for the leaves that follow

**The pre-publish check earned its place, and it failed.** The root brief's
`### The cutover sequence` step 2 says to run the new build read-only against
every other live grove on the machine before publishing. There were **four** —
`Writegood`, `grove.gh-issue-12`, `grove.code-walkthrough-for-ordinal-fs-tree`
and `APIAnyware.add-ocaml-target`, 505 leaves between them — and 19.4.0 refused
every one at its first filename while 19.3.0 picked a live leaf in each. A
release published without addressing that strands four workstreams.

The measurement needs `env -u GROVE_SIGNAL_FILE`, and a probe from inside a
session also trips the working-tree guard; without stripping the session's own
environment both builds answer *wrong working tree* and the check silently
measures nothing.

**Resolved by renaming all four onto the new grammar in the same window**, on
the human's decision. Three were clean and took a commit of their own;
`grove.gh-issue-12` carried unrelated work in progress, so its rename is
snapshotted in its working copy and its next session commits it with that work.
`.grove/FORMAT` was left in place in all four — nothing on any build reads it,
and a stray `FORMAT` is a foreign entry every reader ignores (`delete-migration-k6`).

**The consequence for later cutover leaves.** Those four trees are now on the
`--` grammar and pinned to 19.4.0 or later. A leaf that publishes a release
which changes how a tree is *read* must repeat this check against all of them,
and budget for renaming them, rather than treating this repo's tree as the
only one the release meets.

**The rename was one function of inference, used once on the way out.** The
script matched the closed kind set longest-label-first to find where the kind
ended — precisely the inference the separator abolishes — and every one of the
535 leaves matched, with `FORMAT` the only unmatched entry in any tree. Node
directory names were untouched, as the grammar says they are.
