# A task name has exactly one spelling

Grove's task-tree grammar is canonical. Parsing and rendering are inverses in
both directions: `parse(format(n)) == n` and `format(parse(f)) == f`. A
position uses at least two digits with no excess leading zero; a key is positive
decimal with no leading zero. Each filename has one reading.

    NN-[DONE-|ABANDONED-]<kind>--<slug>-k<key>.md    leaf
    NN-k<key>/                                    node directory
    _<slug>.md                                    node file
    _BRIEF.md                                     root node file

The first `--` separates a leaf's kind from its slug. Both are nonempty tokens
of lowercase ASCII letters, digits and single hyphens; neither contains the
separator or a reserved word. Node directories carry only position and key.
Every directory, including the root, holds exactly one node file, a regular
file. `_BRIEF.md` belongs only at the root; a titled node file belongs only in
a positioned node. `BRIEF` is reserved and cannot be a slug.

The node file's name supplies the node's title and its body supplies the brief.
A node handle combines that slug with the containing directory's key. The name
module owns that composition, the leaf handle and the handle parser and
renderer. No title or routing field is read from a file's contents.

## Why this shape binds

A directory's spelling is paid again in every descendant path. Keeping titles
on node files bounds that repeated cost to the position and key. `_` also
separates a node's own file from its digit-prefixed children. It sorts first in
common human-facing listings and after digits in byte order; Grove's ordering
excludes it altogether.

Canonicity prevents two spellings from becoming one apparent entry: otherwise a
lookup could choose one file while a retirement marks the other. Recognised
names that cannot be parsed, missing node files and multiple node files halt the
read or mutation, naming the level or offending files and the canonical form.
A read never repairs names. The operator pays for strictness when hand-editing;
a refusal must therefore provide the spelling needed to recover.

A missing positioned-node-file error also advises the operator to check for an
interrupted `leaf-decompose` before creating a brief: if the directory is empty
and a sibling leaf shares its position and key, delete the empty directory to
retain the leaf, or move that leaf into it as its `_<slug>.md` node file.
Giving either half a fresh key or manufacturing a brief does not recover that
operation. This advice is conditional, not a claim that Grove found the sibling.
The level check refuses before a guarded snapshot exists, so exact recognition
would need a new store affordance or a second reader. Accept the less specific
diagnostic instead; reopen a guarded diagnostic interface only if automatic
recognition is required beyond actionable operator recovery.

A `_`-prefixed name belongs to the node-file grammar even when malformed. A
digit-prefixed name belongs to the positioned grammar; neither can be silently
disclaimed as foreign and hide work. Names outside these partitions remain
foreign. Grove supports this grammar only and performs
no migration.

## Considered options

- **Keep a title in each directory.** Rejected because every extra title byte
  consumes path length at every deeper level. Nothing reopens this naming call.
- **Add a separate title marker.** Rejected because the node file already holds
  the work's brief, and two files would split one node's identity and context.
  Nothing reopens it without a requirement for separately owned content.
- **Repeat the key in the node file.** Rejected because two stored keys can
  disagree. The directory alone owns it; nothing reopens this duplication.
- **Accept alternate spellings and repair during reads.** Rejected because a
  read would mutate under a shared lock. An explicitly requested repair product
  would be a different decision.
- **Use a single hyphen between kind and slug.** Rejected because both tokens
  may contain hyphens and kinds form an open set. Reopen only if one of those
  token contracts changes.

The structural model's `witness_two_filenames_name_one_entry` demonstrates the
canonicity failure. `witness_two_distinguished_children` demonstrates why a
canonical grammar still needs validation of each level: two different node-file
names can coexist on disk. The library's architecture states that validation;
[`module-decomposition`](../specs/module-decomposition.md) states Grove's
name and handle ownership.
