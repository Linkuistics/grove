# usage-inventory-k16

## Goal

Write down, and commit on its own, the set `docs/USAGE.md` must cover: every
command and flag the guide is obliged to document, and every user journey from
scaffolding a grove to teardown. Do not edit the guide in this session.

## Context

- The root brief's *Done when* requires the inventory to precede the guide's
  edit, so that "complete" names a checkable set and an adversarial read has
  something to read against.
- **The inventory's own boundary is yours to settle and to write down.** The
  brief says "every subcommand and flag the binary exposes", and `grove` itself
  exposes only `-h` and `-V` — the loop is what a bare `grove` runs. The commands
  a human actually meets are split across two binaries: `grove`, and the
  `grove-llm` verbs that `docs/USAGE.md` already documents in its *Review
  composition and escalation* section. State which surface the guide owns and
  why, in the inventory itself, before enumerating it. Getting that boundary
  wrong in either direction is the failure mode: too narrow and the guide is
  complete against a set nobody wanted; too wide and it takes
  `docs/CONFIGURATION.md`'s subject.
- Derive the command rows from `--help` output, not from the current guide. The
  guide is the artifact under test; using it as the source makes the inventory
  agree with it by construction.
- The journeys run from `root-init` through pick, session, retire, commit, node
  close, review composition, pruning, and the complete finish cycle. `CONTEXT.md`
  names each of those; the guide's job is the human's path through them.

## Done when

- A committed inventory exists — location yours to choose within the ownership
  table's rules; a section of `docs/USAGE.md` is legitimate if it is genuinely
  the guide's own subject, and a separate file directly under `docs/` is not,
  because the ownership table bounds what may sit there.
- Every command row is traceable to `--help` output, and every flag of every
  covered command appears.
- Every journey row names its start state and its end state.
- The inventory says what it deliberately excludes, and why.
- `bash scripts/check.sh` passes and no link is left dangling.

## Notes

**Do not improve the guide while you are in it.** A single commit that both sets
the standard and meets it cannot show that the standard was met; the two-leaf
split exists for exactly that reason and folding them back defeats it. Anything
you notice about the guide's content goes in the inventory as a gap row, which is
`usage-guide-k23`'s work list.
