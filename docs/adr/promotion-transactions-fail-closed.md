# Promotion transactions fail closed

`leaf-promote-chain` stages its complete replacement node under the reserved
`PROMOTING-<final-node-name>/` prefix, moves the producer into that directory,
and lands the node with one same-parent rename. While the prefix exists every
task-tree reader and mutator refuses to proceed, except promotion recovery for
the named producer, so an interrupted file-to-directory transformation can
neither schedule review before its producer nor let another mutation reuse the
reserved key run.

This binds because portable filesystems do not offer the file-to-differently-
named-directory replacement as one atomic operation, and Grove's normal
lenience toward foreign files would otherwise turn an interrupted staging
directory into invisible workflow state. Reserving one visible prefix and
failing closed preserves the stronger property Grove needs — no partial state is
runnable — without a journal or lock outside the task tree.

## Considered options

- **Create the visible node before moving the producer.** Rejected because an
  interruption makes depth-first pick enter the node and schedule review before
  the still-external producer. Reopen only if visible partial nodes become
  structurally unpickable.
- **Move the producer first and let readers ignore the staging directory.**
  Rejected because an interruption makes pick skip the producer and continue
  into unrelated work, while key allocation can reuse keys hidden inside the
  foreign directory. Reopen only if every affected operation can derive the
  complete transaction without recognising a reserved marker.
- **Use an external journal or lock.** Rejected because Grove's task tree is its
  only workflow state and must remain intelligible without Grove installed.
  Reopen only if that artifact-only constraint changes.
