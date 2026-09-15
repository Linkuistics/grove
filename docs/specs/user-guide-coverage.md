# user-guide-coverage

The coverage inventory for [`USAGE.md`](../USAGE.md): the set that document is
obliged to cover, written down and committed **before** the guide is edited
against it.

## Problem

`docs/ARCHITECTURE.md`'s *Documentation ownership* table makes `USAGE.md` the
canonical source for *human workflow and commands*, and nothing says what that
obliges. "Complete" has therefore been a judgement — the guide covers what its
author thought of — and an adversarial read of it has nothing to read against.

Two failure modes bound the fix, and both are real. Too narrow and the guide is
provably complete against a set nobody wanted: `grove` itself exposes only `-h`
and `-V`, so a guide covering "every subcommand and flag the `grove` binary
exposes" is three rows long and omits every command a human actually meets. Too
wide and the guide takes [`CONFIGURATION.md`](../CONFIGURATION.md)'s subject,
which the ownership table forbids.

## Solution

An inventory in four parts — a stated boundary, a command table, a journey
table, and a named set of stable entry points — plus the exclusions and the
gaps. It is a standard, not a description: every row is derived from `--help`
output or from `CONTEXT.md`'s lifecycle vocabulary, never from the guide's
current contents, because the guide is the artifact under test.

### The boundary: what the guide owns

**The guide owns the whole installed command surface — both binaries, every
subcommand and every flag — described in the human's register.**

The justification is delivery, not intent. `docs/ARCHITECTURE.md`'s *Repository
products* table has Homebrew install `grove` **and** `grove-llm`; both land on
the human's `PATH` from one formula, and no other row of the ownership table
documents either. `grove-llm --help` says its verbs "are [not] meant for direct
human use", and that is a statement about who *drives* them, not about who has
to understand them: a human reading a task tree is reading the result of those
verbs, three of them (`pick`, `resolve`, `brief-chain`) are diagnostics the
guide already offers, and one (`leaf-prune`) is gated on human confirmation and
cannot be documented anywhere else without splitting its subject.

**Register, not depth.** For `grove-llm` the guide documents what a verb does to
the tree, what it prints, whether it commits, and when a human runs it directly —
not the calling contract a session works from. That contract is the `--help`
text and the methodology skills, and restating it here would create a second
source for it.

**Three lines the boundary does not cross.** Session configuration, launch
templates and kind routing are `CONFIGURATION.md`'s. Installation is the
`README`'s. The methodology a session executes — kinds, their disciplines, the
review procedures — belongs to the `grove` plugin skills. The guide may *link*
each; it may not restate any.

## The command inventory

Every row below is traceable to `--help` output at grove 20.1.0 (`grove --help`,
`grove-llm --help`, and each verb's own `--help`). *Worked* means the guide must
show a real invocation, not merely name the command.

### `grove` — the human binary

`Usage: grove [COMMAND]` — bare lifecycle plus `view [WORKTREE]`.

| Row | Surface | Obligation |
|---|---|---|
| G1 | `grove` (no arguments) | Worked. All three dispatch outcomes: no `.grove/` ⇒ `root-init` plus a first `requirements` leaf; live leaves ⇒ launch the first in tree order; no live leaves ⇒ materialize the `finish` leaf. |
| G2 | `-h`, `--help` | Named, with the fact that it stops before touching a repository. |
| G3 | `-V`, `--version` | Named, same stop-before-touching guarantee. |
| G4 | Bare lifecycle has no launch-policy selectors | Stated: bare `grove` takes no tree argument and no confirmation step; its enclosing working tree selects the workstream. The viewer path only selects an observation location. |
| G5 | Exit status | Stated: `0` for a clean finish or a session that ended without signalling; `128 + N` when killed, because the driver dies of the same signal. |

| G6 | `grove view [WORKTREE]` | Worked for current and explicit worktree; no upward search, read-only plain text, manual refresh, navigation/scroll keys, error recovery, non-TTY refusal and temporary blocking-read limit. |

Rows G1–G3 come from `grove --help`. G4 and G5 are behaviours no help text
states: G4 is the absence of lifecycle selector arguments, and G5 is verified
in the source — `grove` re-raises the signal it was killed by
(`keyed_launch::reraise`), which is what makes `128 + N` observable to a wrapper.

### `grove-llm` — the tree interface

`Usage: grove-llm [COMMAND]`, plus `-h`/`--help` and `-V`/`--version` on the
binary itself. Twelve verbs; `help` is clap's own and is covered by the row for
`--help`.

| Row | Verb | Arguments and flags — all of them | Obligation |
|---|---|---|---|
| L1 | `root-init` | `[SLUG]` (default `plan`) | Worked. Creates `.grove/`, the root `_BRIEF.md`, and `01-requirements--<slug>-k1.md`; refuses an existing `.grove/`; no commit. Note that bare `grove` does this for you. |
| L2 | `pick` | none | Worked, as a diagnostic. Depth-first pre-order; empty stdout when nothing is live. |
| L3 | `brief-chain` | `[LEAF_PATH]` | Worked, as a diagnostic. Defaults to `pick`'s leaf; each level must have exactly one correctly placed node file; a missing or competing file refuses the read. |
| L4 | `kind` | `[LEAF_PATH]` | Named, as a diagnostic. Defaults to `pick`'s leaf. |
| L5 | `resolve` | `<REFERENCE>` — `[n]`, `n`, `[n]-slug`, `<slug>-k<key>`, or a bare slug | Worked. Searches live, `DONE` and `ABANDONED` alike; a node resolves to its directory using its node-file slug and directory key; title edits change handles but preserve keys; ambiguity lists the keys; exits zero either way. |
| L6 | `leaf-add` | `<PARENT>` `<SLUG>`, `--kind <KIND>` (required, repeatable) | Worked. `.` for the root; one leaf per `--kind` in order, as one unit; appends at the end; `finish` is refused; no commit. |
| L7 | `leaf-insert` | `<TARGET>` `<SLUG>`, `--kind <KIND>` (required, **not** repeatable — one leaf per call) | Worked. Shifts the target and later siblings up one; subtrees and keys ride along; no file contents are rewritten; no commit. |
| L8 | `leaf-decompose` | `<LEAF_PATH>` `<FIRST_CHILD_SLUG>`, `--kind <KIND>` (optional) | Worked. Leaf becomes a node directory with the key preserved, body becomes `_<slug>.md` inside `NN-k<key>/`, first child inherits the kind unless overridden; no commit. |
| L9 | `leaf-retire` | `<LEAF_PATH>` | Worked. Adds the `DONE` infix in place; refuses a brief, a `DONE` leaf and an `ABANDONED` leaf; no commit. |
| L10 | `leaf-prune` | `<PATH>` — a live leaf **or** a node directory | Worked. HITL: only after explicit human confirmation. On a node it marks every live leaf in the subtree and leaves `DONE` ones alone; refuses the grove root; no commit. |
| L11 | `finish-commit` | `<FINISH_HANDLE>` | Worked. Revalidates the finish leaf and the absence of ordinary work, then deletes and commits only `.grove/`; does not stand in for the human confirmation. |
| L12 | `complete` | `--done`, `--signal-file <SIGNAL_FILE>` (default `$GROVE_SIGNAL_FILE`) | Worked. The last step of a task, after commit and retire; `--done` ends the whole grove instead of relaunching. |
| L13 | `-h`/`--help`, `-V`/`--version`, `help [COMMAND]` | — | Named once, with the note that every verb carries its own `--help` and that the help text is the authority on what exists. |

**Two rows were corrected by `usage-guide-k23`, against the binary.** The
inventory claims traceability to grove 20.1.0, so a row the binary contradicts is
a defect in the standard rather than licence for the guide to cover something
else.

- **L7's `--kind` is not repeatable.** Only `leaf-add` takes an ordered list;
  `grove-llm leaf-insert 3 spike --kind a --kind b` is refused by clap with
  `error: the argument '--kind <KIND>' cannot be used multiple times`. That
  asymmetry is load-bearing — it is why the research triple is spelled through
  `leaf-add` — so the row now states it.
- **J14 was a journey the binary no longer has.** It described a startup refusal
  for a working tree on a different filesystem from its `.jj/`, justified by a
  teardown that moved `.grove/` into `.jj/grove/` in one atomic rename.
  `tree_lifecycle::finish_commit` performs no such rename — it deletes `.grove/`
  and commits the deletion — and `jj_workspace::Refusal` carries no cross-device
  case at all; the `unsupported workspace layout` message survives only in
  `docs/preservation-baseline.md`, the historical ledger of a Git-worktree-era
  binary. The live refusal at that boundary is the control directory's
  (`the control directory … is not usable`), so J14 now names that one and the
  `usage-workspace-layouts` entry point keeps its anchor with its subject
  restated.

**Flag completeness is the checkable part.** Rows L1–L12 name every positional
argument and every flag each verb accepts; a verb gaining one is a change to
this inventory as well as to the guide.

## The journey inventory

Each row names its start state and its end state, so a reader can tell whether
the guide got them from one end to the other. `CONTEXT.md` names the underlying
lifecycle; the guide's subject is the human's path through it.

| Row | Journey | Start state | End state |
|---|---|---|---|
| J1 | Scaffold a grove | A jj working tree with no `.grove/`, configuration written | Root brief and a live `requirements` leaf; first session launched |
| J2 | Prepare a tree that is not jj-enabled | A Git or plain directory | `jj git init --colocate` run; `grove` starts |
| J3 | Run and resume | A grove with live leaves | The first live leaf in tree order launched; resuming is the same command |
| J4 | Watch one session through | A leaf launched | Leaf `DONE`, its work committed under the stable handle, loop relaunched |
| J5 | Interrupt and stop the loop | A session running | Ctrl-C reaches the session and the driver decides; `kill` on `grove` ends the loop and exits `128 + N` |
| J6 | Read the tree by eye | A `.grove/` directory | The next session identified from filenames alone, without running anything |
| J7 | Decompose an oversized leaf | A live leaf too big for one session | A node directory with `_<slug>.md` and a first child |
| J8 | Compose a review chain | A producer at a reviewable boundary | `review-*` leaf beside it, added as the producer's last act; then `integrate-review-*` placed by the insert rule |
| J9 | Cut a research pair | An open question worth two corpora | `research-a`, `research-b` and `combine-research` cut in one all-or-nothing call |
| J10 | Close a node | A node whose children are all terminal | The node closed and committed by the retiring session |
| J11 | Prune abandoned work | Live work that should no longer be done | `ABANDONED` in place, after explicit human confirmation; the whole reviewed path pruned, not just the producer |
| J12 | Recover a mistake | A grove started in the wrong workspace, or a bad commit | `jj op restore` / `jj undo` back to the prior state |
| J13 | Resolve a second-driver refusal | Two `grove` processes in one working tree | The second exits naming the canonical tree; the first stays owner |
| J14 | Fix an unusable control directory | `.jj/grove/` cannot be created or written | Permissions fixed; `grove` starts |
| J15 | Finish | No live leaves left | Knowledge promoted, `.grove/` torn down and committed, loop stopped cleanly |
| J16 | Integrate after finishing | A torn-down grove | Branch or bookmark integration done by the human, after teardown |

## Stable entry points

The guide publishes an explicit `<a id="…"></a>` anchor immediately preceding
the heading of each row below, and **this is the set a walkthrough book may
reserve from**. `docs/specs/walkthrough-books.md`, *Outbound links*, requires
every book whose manifest declares a `[guide]` path to cite one declared guide
anchor from its `README.md` reader contract, and reports a reserved anchor that
the guide does not carry in the explicit form. Renderer-generated heading slugs
change silently when a heading is retitled; these do not.

| Anchor | Entry point | The reader it serves |
|---|---|---|
| `usage-running-grove` | Running grove: start, resume, finish | Anyone meeting the binary for the first time — the overview's entry point |
| `usage-task-tree` | The task tree and its filename grammar | A reader who has a `.grove/` in front of them |
| `usage-session-lifecycle` | What happens in a session | A reader tracing kind ⇒ launch ⇒ signal ⇒ relaunch |
| `usage-tree-verbs` | The `grove-llm` verbs over the tree | A reader tracing what a session does to the tree |
| `usage-review-composition` | Review composition and escalation | A reader following how review chains are built |
| `usage-driver-lease` | One driver per working tree | A reader on ownership, leases and concurrent drivers |
| `usage-workspace-layouts` | Supported workspace layouts | A reader on jj workspaces, the control directory, and one grove per workspace |
| `usage-finish` | The complete finish cycle | A reader on teardown and its guarantees |

The `usage-` prefix is deliberate: these anchors are cited from other documents
as `USAGE.md#…`, and an unprefixed `finish` or `pick` would read as a repository
term rather than a location in this guide.

**The set is closed and additive.** A book may reserve only from this table; a
book needing an entry point that is not here changes this file first. Removing a
row is a breaking change to every book that reserved it, and the books' own
validation is what reports it.

## Out of scope

Named here so that "complete" does not silently mean "everything".

- **Session configuration, launch templates, kind routing, the `.grove.kdl`
  delta.** `CONFIGURATION.md` owns them. The guide states that configuration is
  validated before any tree mutation and links; it does not carry the schema.
- **Installation and the marketplace.** The `README` owns them.
- **The methodology.** Kinds, their disciplines and the seven constraints belong
  to the `grove` plugin skills; the guide names the plugin and links its spine.
- **Grove vocabulary.** `CONTEXT.md` owns it. The guide uses the terms; it does
  not define them.
- **Runtime and repository design.** `docs/ARCHITECTURE.md` owns the *why*; the
  guide covers the *what* and *how* and links for the rest.
- **`book-check` and `syllabus`.** Repository-internal authoring tools, not part
  of the installed product — the *Repository products* table ships `grove` and
  `grove-llm` only.
- **Exit codes and stderr text of individual `grove-llm` verbs**, beyond the
  behaviours the table's rows name. Each verb's `--help` is the authority, and
  transcribing it would create a second source that goes stale.
- **Cutting a release.** `RELEASING.md` owns it.

## Gaps in the guide as `usage-inventory-k16` found it

Measured against the tables above, not against the guide's own structure. This
was the work list for the leaf that edits the guide; it is a finding, not an
instruction about wording. **All fourteen were closed by `usage-guide-k23`**, and
the list is kept as the record of what the standard caught — the live statement
of coverage is the guide's own coverage map, which
`crates/grove/tests/user_guide_coverage.rs` compares against the tables above.

| Gap | Row | What is missing |
|---|---|---|
| 1 | L1 | `root-init` is never named; the guide describes only bare `grove` doing it. |
| 2 | L4 | `kind` is absent entirely. |
| 3 | L8 | `leaf-decompose` is absent; decomposition is described only in prose. |
| 4 | L9 | `leaf-retire` is absent; retirement is described only in prose. |
| 5 | L10 | `leaf-prune` is never named as a verb, though pruning and its HITL gate are covered; node pruning and the refusal of the grove root are missing. |
| 6 | L12 | `complete` is never named, and neither flag appears — the guide says only that a session "signals Grove". |
| 7 | L2, L3, L5 | `pick`, `brief-chain` and `resolve` are named but carry no worked invocation and no argument forms. |
| 8 | L6, L7 | `leaf-add` and `leaf-insert` appear worked, but only inside the review-chain section; their arguments are not stated as a surface. |
| 9 | L11 | `finish-commit` is named without its argument or its revalidation behaviour. |
| 10 | L13 | Per-verb `--help` is never pointed at as the authority. |
| 11 | J2 | Covered in the opening paragraph only; not reachable as a journey. |
| 12 | J6, J9, J10 | Present but scattered; no start-to-end path a reader can follow. |
| 13 | Anchors | The guide carries no explicit `<a id="…"></a>` anchors at all, so no book can validate a guide citation today. |
| 14 | Structure | The guide's current headings are close to the entry-point set but not identical; the anchors above are the contract, and headings follow them rather than the reverse. |

## Test seams

- `crates/grove/tests/user_guide_coverage.rs` — the row-by-row check. It reads
  every `G`/`L`/`J` row id and every stable-entry-point anchor out of *this*
  file, and requires the guide's coverage map to name each row exactly once, the
  guide to carry each anchor in the explicit `<a id="…"></a>`-before-a-heading
  form, and every anchor the map points at to exist. Two documents, one written
  by the standard and one by the guide, have to agree; a row added here and
  forgotten there is a failure rather than a silence.
- `user_documentation_references_resolve` and
  `every_repository_markdown_reference_resolves`
  (`crates/grove/tests/reference_navigation.rs`) — the guide is inside the
  curated user-documentation surface, so every link it grows is checked.
- `book-check` over each book directory, via `scripts/check.sh` — the mechanism
  that reports a reserved guide anchor the guide does not carry, per
  `docs/specs/walkthrough-books.md`.
