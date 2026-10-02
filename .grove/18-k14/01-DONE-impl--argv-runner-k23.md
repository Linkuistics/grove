# argv-runner-k23

## Goal

The runner's template machinery and the last records of Grove configuration are
deleted, and every book still validates. The `keyed-launch` book is left valid
and not yet coherent: what it is for is the next child's.

## Context

- The node brief, `_runner-templates.md`: its *Done when* and notes.

## Done when

- Everything in the node brief's *Done when* except the `keyed-launch` book's
  account of itself: its thesis, its orientation and assembly prose, its
  concept index and its structure specification.
- That book has no chapter, fragment, root or index row for a deleted file,
  and `book-check` validates it.
- `bash scripts/check.sh` passes.

## Decisions (running log)

**The leaf decomposed at the book's thesis.** The book's stated outcome is a
test with three parts, and two of them were about the template half. Deleting
four chapters and rebasing the rest is mechanical. Saying what the book is for
now is a rewrite of its brief, its orientation and its closing chapter, so it
is the second child, as `lifecycle-launch-k12` split at the `grove-llm` book.

**Citations in documents `current-state-documents-k15` rewrites are unlinked
and no more.** The usage guide, the two READMEs, the architecture document, the
release procedure and the configure-grove skill still describe `config.kdl` in
their own prose. Each link into a deleted file now points at the dispatch
README or a specification, or is gone. The skill's two `Source:` lines are
absolute URLs that no check reads, and they are left with the files they head.

**Historical records keep their text.** The changelog, the preservation
baseline, the formalism findings and the routing research name the deleted
files as plain text marked *since deleted*. The baseline gains one exception
note saying Grove's configuration is gone.

**There is no decision 6.** `module-decomposition.md` says so in its
introduction and the number is not reused. Decision 7 lists the runner's
interface as the crate exports it, which adds the observed, noninteractive and
confined entry points the old sketch never carried.

**The glossary keeps one retired entry.** **Grove configuration** stays, marked
retired, because a copy of the file can still be on an owner's disk. Its five
sub-entries are deleted. **Kind routing** and **Review target diversity** are
reworded to the dispatch policy.

**One runner test went as a duplicate.** `arguments_reach_the_child_as_written`
proved a template's quoted word stayed one argument. Built with `Argv::new` it
is a subset of `a_caller_built_argv_is_spawned_whole_and_directly`.

**Chapter 1 of the book was rewritten, not patched.** Its fragments covered the
manifest, the library root and the error module, and all three changed shape.
Prose sits between the fragments it explains, so a valid chapter over the old
prose would have described fragments that are not there. `src/argv.rs` moved
into it from the deleted chapter 5, since it is the type a command arrives in.

**The assembly chapter lost two of its three parts and kept its question.** The
parts about composing a value and re-reading one were about the template half.
What is left is the way out, the stall the crate cannot close and the ledgers,
restated for seven roots and 1,688 lines. Whether the book keeps that question
at all is `book-thesis-k24`'s.

**The structure brief carries a status note and is otherwise untouched.** It
says the book has moved ahead of it. Its section on `docs/CONFIGURATION.md` is
gone with that file.

**The books were rebased by script and the source index was generated.** Two
throwaway scripts: one carries fragment ranges across a diff and re-reads
literal bodies, the other writes the whole source index from the manifest and
the fragment headers. The recorded test run in the assembly chapter was
re-measured. Figures in chapters 2 to 4 and 7 were not.

**Three statements outside the runner followed the code.** `grove-loop`'s
library root and its `complete` module described the runner as holding a key
and a template, and the architecture document described a template reader the
crate *still carries*. Each now says what the crate does. The `grove-loop` book
was rebased for the two source comments and for the three lease-test files.

**`reference_navigation.rs` names four guides.** `docs/CONFIGURATION.md` was
the fifth, and the test's example of a real sibling document.

**One provisioning test failed once and was not reproduced.**
`canonical_legacy_aliases_remove_obsolete_links_once_and_launch` failed in one
full workspace run at its last assertion, that the launched session ran. It
passed six times alone and thirty-two times at eight copies in parallel. This
leaf touches nothing on that path. The assertion prints no diagnostic, so the
cause is not known.
