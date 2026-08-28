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

1. Land the parser change; `cargo test`; `cargo clippy --all-targets`.
2. `cargo build --release`, then put the two binaries where this machine's `PATH`
   resolves them. **They are installed by Homebrew** —
   `/opt/homebrew/bin/grove` and `/opt/homebrew/bin/grove-llm` are symlinks into
   `../Cellar/grove/<version>/bin/`. Overwriting the Cellar files is the direct
   route; `cargo install --path .` works **only** if `~/.cargo/bin` outranks
   `/opt/homebrew/bin` on `PATH`, which on this machine it does not — that trap
   is documented at `docs/ARCHITECTURE.md:1540` and `docs/USAGE.md:175`. Verify
   with `command -v grove` and `grove --version` before continuing.
3. Rename the tree. Check the new binary parses it: `grove-llm pick`,
   `grove-llm kind`, `grove-llm brief-chain`.
4. Retire this leaf **by its new path** and commit everything as one change.
5. `grove-llm complete`.

**Expect the loop to stop after this session, and say so in the commit message.**
The running driver is the *old* build, loaded in memory; `release.toml` records
that the driver's version-skew guard is the first thing in the loop body, so a
reinstall under a live loop lets the session that ran it finish normally and
stops the loop before the next one. That is the machinery working. The human
restarts `grove` and the new build takes over. Nothing needs recovering.

**Lands green in the suite**; what it breaks is the pairing between an installed
binary and a tree on disk, which is why the sequence above is part of the
deliverable rather than advice.

**A standing consequence starts here**: from this leaf onward, every later leaf
that changes the verb surface sessions invoke must reinstall in the same session.
The root brief records this.
