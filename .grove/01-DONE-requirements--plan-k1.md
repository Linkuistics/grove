# plan-k1

## Goal

Establish what the human wants from a TUI for monitoring and viewing `.grove/`,
agree the acceptance criteria and test seams, and leave a precise handoff for
the next session.

## Context

- `CONTEXT.md`: task-tree scheme, driver lease, session epoch and tree access lock.
- `docs/USAGE.md`: the current human workflow and tree-reading verbs.
- `crates/grove-loop/src/lib.rs`: the existing public, shared-lock tree reader.

## Done when

- The human has confirmed a shared understanding of the scope and behavior.
- The agreed requirements and test seams are recorded in the root brief.
- Any further design or planning work has a bounded leaf with the necessary
  evidence and open questions.

## Notes

This is the fresh grove's bootstrap leaf. Its initial task and root brief were
empty; the workstream name is the only initial product description. The
requirements interview starts with the interaction scope, since it determines
what monitoring means, what the UI must show, and which test seams are needed.

Working sequence: inspect existing contracts; settle the dependent requirements
one question at a time; present the complete brief and test seams for agreement;
then grow the next work, retire this leaf and seal its task commit.

Initial code evidence: `grove_loop::read` returns `Reading::Tree` or
`Reading::Vacant`; a tree owns a shared access lock. The driver epoch record
contains process identity and a signal path. A live task in the filename grammar
means unfinished work, so it alone does not establish an active session.

The graph was indexed for this workspace in fast mode (generation
2026-09-15T07:00:01Z). Coverage checks recorded no gaps for the referenced Rust
files; documentation is excluded from that index and was read directly. Graph
call edges for the generic name `read` contain unrelated matches, so the
specific reader behavior above was checked against exact source.

## Decisions (running log)

The human established a permanent scope boundary: "This is only ever read-only."
The TUI monitors the task tree and updates automatically when it changes. This
is a product constraint, not a first-version restriction to revisit later.

The human chose "Selected task or brief": selecting a task displays its own
Markdown file, and selecting a branch displays its node brief. Ancestor briefs
are reachable through tree navigation; selecting a task does not concatenate
its whole brief chain.

The human agreed that automatic refresh preserves the current selection and
reading position rather than following the next live task. Changes update the
tree without interrupting reading; the selected work item's permanent key is
the existing identity available across renumbering and moves.

The human chose formatted Markdown for the selected-file pane, including
headings, lists, tables and code blocks, rather than displaying raw Markdown
source.

The human agreed to include all tasks, with clear labels distinguishing live,
completed and abandoned work, and collapsible branches for larger trees.

The human confirmed that the root requirements brief captures our shared
understanding, including the proposed test approach. The agreed main test seam
drives the viewer with a temporary tree, scripted navigation and a terminal
viewport and checks visible results. Real filesystem change detection,
preservation of selection/scroll/expansion, absence of viewer writes and a
terminal smoke check are part of that agreement. Requirements are settled;
the next session can plan the implementation without repeating this interview.
