# A task name has exactly one spelling

Grove's task-tree grammar is **canonical**: distinct filenames never name the
same entry, so `format(parse(f)) == f` holds and not merely `parse(format(n))
== n`. A position is zero-padded to at least two digits and carries no other
leading zero — `05` and `100` are names, `5` and `005` are not — and a name
spelled any other way is refused, with the spelling Grove writes named in the
refusal. `src/task_name.rs` discharges this in one line: parse leniently, render
the result, refuse when the rendering differs from the input.

This replaced a deliberate leniency. The grammar Grove carried before the flip —
`src/tree_id.rs`, deleted in the contract stage — accepted a hand-typed `5` while
rendering `05`, so `05-impl-a-k1.md` and `5-impl-a-k1.md` were **one entry
occupying two files**, sharing a key and a position.

## The trade-off

Leniency is kind to the person hand-editing a tree and hostile to every
invariant above them. `docs/ordinal-fs-tree/models/structure.als` draws the
result under `witness_two_filenames_name_one_entry`: a tree satisfying every
stated invariant in which one entry is two files. Downstream of that, key
uniqueness is false, `by_key` picks arbitrarily between two entries, a sibling
shift moves one spelling and leaves the other, and a retire marks one of the two
`DONE` while `pick` keeps returning the other. None of those failures names its
cause, because each of them is reading a tree that is already wrong.

The cost is real and falls on exactly the person leniency was for. Grove's own
methodology invites hand edits — `references/retire.md` offers *reorder by
hand* — and under this decision a hand-typed `5-` makes the **whole tree**
unreadable until it is renamed, since a malformed name halts rather than being
skipped. That is why the refusal is required to name the canonical spelling and
not merely report a parse failure: the recovery is one `mv`, and the message has
to contain it. Detection without advice would have made this decision
indefensible rather than merely expensive.

One tree can still arrive carrying a lenient name, and the record has to say
what happens to it: a **legacy tree being migrated**. Migration re-renders every
leaf it moves through the canonical grammar, so a legacy `5-task-k1.md` lands as
`05-impl-task-k1.md`. It does not rename **directories** — a migration plan's
source and destination share a parent, which is what is left of the format once
the two layouts needing relocation were withdrawn — so a hand-edited `5-node-k1/`
survives the conversion and is then refused by name, with the `mv` in the
message. That is the same bargain as above and not a gap in it: the alternative
is refusing to *recognise* the directory, which classifies the whole tree as
having no work in it.

What makes the trade favourable is a fact about the corpus rather than a
preference. **Grove has never written a lenient position.** Every on-disk name
Grove produces is rendered by one function that formats `{:02}` — the grow
verbs, the lifecycle verbs and the migration's output alike — so the only way a
lenient spelling enters a tree is a hand edit, and no tree in flight contains
one. The flip is therefore still a pure refactor: nothing on disk changes, and
nothing that Grove wrote becomes unreadable.

## Considered options

- **Waive the obligation knowingly** and carry the withdrawn grammar's leniency
  (`src/tree_id.rs`) into the domain implementation, recording what breaks.
  Rejected: the library's operations are
  built on the assumption that a name is an entry's unique spelling, and a
  waiver does not make them tolerant of the alternative — it makes the corruption
  silent and Grove's, rather than loud and the operator's. A waiver would also
  have to be re-litigated at every later flip leaf, since each one rests on the
  same assumption.
- **Accept a lenient spelling and repair it in place** — parse `5-`, rename the
  file to `05-`, carry on. Rejected: a read verb would mutate the tree, which
  breaks the read/write guard split (`src/tree_access.rs`) and would have a
  `pick` taking the write lock. It also repairs under the *shared* lock, where a
  concurrent reader is looking at the name being replaced. Reopen as an explicit
  repair verb, which is a different thing from a parse.
- **Repair the spelling during migration**, which is the one write path that
  already rewrites names and could rename a lenient directory into its canonical
  form. Rejected for the migration planner's own invariant rather than for the
  reasons above: a planned file's source and destination share a parent, and the
  migration transaction leans on that where its directory sweeps used to be.
  Renaming a node means moving its whole subtree, which reintroduces relocation
  into a format that was reduced to in-place renames on purpose. The lenient
  directory is instead recognised, passed through, and refused by name.
- **Keep leniency and add a separate tree-wide duplicate check.** Rejected: it
  is a second mechanism for a property the grammar can hold by construction, it
  can only run over a whole tree while parsing is per-entry, and it answers after
  the fact — the point of canonicity is that the ambiguous state is never
  represented.

## Why this is hard to reverse

Loosening later is not symmetrical with never having tightened. Once the grammar
is canonical, every operation Grove builds on the library is entitled to assume
one entry is one file, and those assumptions are unwritten by construction —
they show up as the absence of a duplicate-key case rather than as code naming
it. Reversing means auditing each of them, not deleting a check. The decision
also reaches the operator: the refusal is an instruction humans learn, and
withdrawing it silently re-admits the trees it was teaching them not to make.

`docs/formalism-findings.md` entry 020 records how the check was verified, and
the fixture defect the verification found.
