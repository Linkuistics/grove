# Using Grove

Grove drives a long-running workstream as a sequence of fresh agent sessions.
The workstream lives in a `.grove/` task tree inside a Git or Jujutsu working
tree that you create and own.

Before starting, install Grove as described in the [README](../README.md) and
set the required harness and model configuration in
[CONFIGURATION.md](CONFIGURATION.md).

## Start or resume

Run Grove from anywhere inside the working tree:

```sh
grove do
```

`grove do` is the only lifecycle entry command. It inspects the filesystem and
does the appropriate next thing:

- With no `.grove/`, it opens a requirements session. That session creates the
  root brief and the first `requirements` leaf.
- With live leaves, it launches the first one in tree order.
- With no live leaves, it opens the finish cycle.

If the session or terminal is interrupted, run `grove do` again. Grove resumes
from committed files and the task tree; it has no separate progress database.

Use `--harness` when selecting or changing the grove's primary harness:

```sh
grove do --harness codex
```

Use `--no-launch` to check the next leaf, harness, model, prompt, Codex trust,
and VCS access without starting an agent or writing a harness stamp:

```sh
grove do --no-launch
```

## Human-facing commands

| Command | Purpose |
|---|---|
| `grove do [--harness NAME] [--no-launch]` | Start, continue, or finish the workstream. |
| `grove migrate [PATH]` | Migrate an older `.grove/` layout in place. `grove do` also does this automatically when adopting an old tree. |
| `grove retire NODE [--harness NAME] [--no-launch]` | Open a one-off session that promotes a completed node's brief. The harness choice applies only to that session. |

`grove --help` and each command's `--help` output are the authoritative flag
reference. `grove-llm` is a separate internal command surface for the agent; it
is not intended as a human workflow.

## The task tree

A small workstream might look like this:

```text
.grove/
├── BRIEF.md
├── 01-DONE-requirements-k1.md
├── 02-auth-chain-k2/
│   ├── 01-DONE-auth-design-k3.md
│   ├── 02-auth-design-review-k4.md
│   └── 03-auth-design-integrate-k5.md
└── 03-ship-k6.md
```

A leaf is one agent-sized task. A directory is a node containing smaller
tasks; it may have a `BRIEF.md` charter. The filename carries four pieces of
state:

- `NN` is the mutable position among siblings.
- `slug` is the human-readable name.
- `kN` is the permanent, grove-wide identity.
- `DONE` and `ABANDONED` are terminal outcomes kept in place until the grove
  finishes.

Grove picks the first live leaf in depth-first, numeric order. It does not run
a hidden scheduler or infer dependencies from prose. Review chains and research
pairs are conventions represented by nested directories; see
[Architecture: task kinds and composition](ARCHITECTURE.md#task-kind-taxonomy).

## What happens in a task session

The embedded Grove methodology guides the agent through the same loop:

1. Pick one live leaf and read its ancestor briefs.
2. Apply the discipline named by the leaf's task kind.
3. Do and verify the work.
4. Commit the result through the repository's native VCS interface.
5. Mark the leaf `DONE`, retire any completed parent nodes, then signal Grove.

A task that proves too large can be decomposed in place. Work that should no
longer be done can be marked `ABANDONED`, but pruning requires explicit human
confirmation. Grove guides that decision; it does not make it autonomously.

The runtime methodology is [`content/SKILL.md`](../content/SKILL.md). The CLI
embeds that directory and provisions it to the selected harness's personal
skill directory on `grove do`.

## Finish

After the last live leaf is retired, the running session proposes one complete
finish cycle:

1. Promote durable knowledge from briefs into the repository's normal docs,
   decision records, specs, or context files where it still belongs.
2. Delete `.grove/` in a focused commit.
3. Signal the driver that the grove is done.

This is Grove's one routine human confirmation point because it deletes the
workstream tree. Branch or bookmark integration and working-tree teardown remain
your responsibility; Grove never creates, merges, or removes them.

For why these boundaries exist, see [ARCHITECTURE.md](ARCHITECTURE.md).
