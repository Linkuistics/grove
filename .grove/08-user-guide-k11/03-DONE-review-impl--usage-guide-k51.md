# usage-guide-k51

**Reviews:** usage-guide-k23

## Goal

An adversarial read of `docs/USAGE.md` against
`docs/specs/user-guide-coverage.md`. The root brief makes this read what *closes*
`user-guide-k11`: the inventory exists so that completeness is a checkable claim,
and a standard nobody reads against is a standard nobody tested.

## Context

- The producer is `usage-guide-k23`; its commit names that handle, so its diff is
  where the work is. It rewrote the guide's structure, not just its gaps.
- The inventory is the specification. `usage-guide-k23` **changed two of its
  rows** — L7's `--kind` repeatability, and J14 plus the `usage-workspace-layouts`
  entry-point description — on the ground that the binary contradicted them. Those
  two corrections are the highest-value thing to re-derive: a producer allowed to
  edit its own standard can widen it to fit what it wrote, and the argument
  recorded in the inventory is exactly what an adversary should try to break. The
  claimed evidence is `grove-llm leaf-insert --kind a --kind b` (clap refuses a
  second), `tree_lifecycle::finish_commit` (deletes and commits, renames nothing),
  and `jj_workspace::Refusal` (no cross-device case).
- `crates/grove/tests/user_guide_coverage.rs` is new and enforces three
  agreements between the two documents. A test the producer wrote against its own
  work is itself in scope: ask what it does **not** catch.

## Done when

Findings are written down, anchored to file and line, with no fixes applied. In
particular, say for each:

- **A row covered in name only.** The map answers all 34 rows; the map is not the
  coverage. Sample rows across `G`, `L` and `J` and check the section the map
  names actually gets a reader from the start state to the end state, and that
  every command row carries a *worked* invocation rather than a mention.
- **A transcript that is not what the binary prints.** Every fenced `console`
  block claims to be real output with paths rewritten to `/home/you/app`. Check
  the shapes against `--help` and the source — especially the ones that could not
  be run here: the second-driver refusal, the control-directory refusal, the
  `finish-commit` live-work refusal, and the `128 + N` exit.
- **A boundary crossed.** The guide may link `CONFIGURATION.md`, the `README`,
  `CONTEXT.md` and the plugin skills; it may not restate them. The verbs section
  is the likely offender — the inventory's *register, not depth* rule says what a
  verb does to the tree, not the calling contract a session works from.
- **The summary layer.** The opening paragraphs and the coverage map are
  roll-ups; check they still describe the body after the restructure.
- **The anchor set.** Eight anchors are a published surface the next five books
  reserve from. Are they on the headings a book would actually want to cite, and
  is any of them attached to a heading likely to be retitled?

## Notes

Inspection only — no fixes, and no edits to the guide or the inventory. If the
findings are worth acting on, cut the `integrate-review-impl` step as this
session's last act; place it with `leaf-add` unless a later sibling entry still
holds live work, in which case `leaf-insert` at the first one that does.

## Findings

The two producer-authored corrections survive re-derivation. `LeafInsertArgs`
stores one `String` while `LeafAddArgs` stores `Vec<String>`, so L7 is right that
only `leaf-add` repeats `--kind` (`crates/grove-llm/src/cli.rs:360-387`).
`finish_commit` deletes and commits the tree, and `jj_workspace::Refusal` has no
cross-device variant (`crates/grove-loop/src/tree_lifecycle.rs:195-258`;
`crates/jj-workspace/src/refusal.rs:28-55`), so the replacement of J14 is also
sound. The findings are elsewhere.

### F1 · High — The `128 + N` transcript reports `kill`'s status, not Grove's

The transcript runs `kill -TERM "$(pgrep -x grove)"` and then immediately runs
`echo $?`, claiming `143` (`docs/USAGE.md:127-132`). In a shell, `$?` there is
the status of the `kill` command; a successful signal delivery returns zero.
The implementation really does restore the default disposition and re-raise the
signal, so a process *waiting for Grove* observes signal 15 / status 143
(`crates/keyed-launch/src/run.rs:186-211`), but this transcript never waits for
that process. It therefore cannot be the real output promised at
`docs/USAGE.md:26-27`, and G5's worked evidence teaches wrappers the wrong way to
observe the contract. Show Grove as a foreground job signalled from a second
shell, or retain its PID and `wait` for it before printing the wait status.

### F2 · High — L11 has no successful worked invocation

The inventory requires `finish-commit` to be worked through the revalidation,
deletion and focused commit (`docs/specs/user-guide-coverage.md:97`). The section
the coverage map assigns to L11 shows only a stale-finish refusal
(`docs/USAGE.md:550-563`), and the Finish section adds only the live-work refusal
(`docs/USAGE.md:752-761`). Neither invocation reaches the operation the row is
meant to cover. On success the CLI emits
`finish-commit <handle>: committed as <change-id>`
(`crates/grove-llm/src/cli.rs:438-457`), but that output and a successful command
are absent. Thus L11 is exactly a row covered in name only even though the map at
`docs/USAGE.md:825` is green. Add a successful teardown transcript; keep the
refusals as useful boundary examples rather than substitutes for the worked
path.

### F3 · Medium — The verbs summary contradicts three verbs beneath it

The roll-up says that none of the twelve verbs commits and that each prints
absolute paths on stdout (`docs/USAGE.md:311-314`). `finish-commit` deletes and
commits `.grove/` (`docs/USAGE.md:550-553,742-743`), `kind` prints a token rather
than a path (`docs/USAGE.md:331-337`), and a successful in-loop `complete` prints
nothing before returning (`docs/USAGE.md:530-548`). The summary therefore gives
a reader the wrong answer on both properties it promises to summarize. Replace
the universal with a small per-verb exception statement (or a table) that makes
`finish-commit`, `kind`, and `complete` explicit.

### F4 · Medium — The guide republishes the review procedure it declares out of scope

The inventory limits the guide to what a verb does to the tree, what it prints,
whether it commits, and when a human runs it; the session calling contract and
review procedures must be linked rather than restated
(`docs/specs/user-guide-coverage.md:44-54,186-193`). The guide nevertheless
reproduces the reviewer allowance and its exceptions, the exact integration
placement algorithm, integration-body convention, retire-before-commit rule,
and target-diversity policy (`docs/USAGE.md:568-646`). Those are methodology
instructions for the agent, not the human-visible result of a tree verb, and now
have a second copy that can drift from the plugin skills. Keep the human journey
and observable tree shape here, but replace the session procedure with links to
its owners.

### F5 · Medium — Duplicate inventory row IDs are silently collapsed by the new test

`inventory_rows` inserts row IDs directly into a `BTreeSet`
(`crates/grove/tests/user_guide_coverage.rs:42-63`). If the standard accidentally
adds a second, different obligation under an existing ID, the set collapses the
two rows and one coverage-map entry still satisfies the equality checks at
`crates/grove/tests/user_guide_coverage.rs:234-267`. The test named
`the_guide_answers_every_inventory_row_exactly_once` therefore cannot establish
uniqueness on the standard side, precisely where a duplicate identity would let
one obligation disappear behind another. Preserve occurrences long enough to
reject duplicate inventory IDs, and cover that counterexample directly.
