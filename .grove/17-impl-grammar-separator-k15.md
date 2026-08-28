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
