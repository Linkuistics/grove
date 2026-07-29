# changelog-unreleased-k13

**Kind:** impl

## Goal

Log this grove's shipped, user-facing changes in `CHANGELOG.md`. They are all
post-`v16.2.0` and currently appear nowhere.

## Context

Raised by `install-workspace-guard-k8`, which needed to log its own change and
found there was no section to log it in.

`CHANGELOG.md`'s own preamble states the rule: *"A change to anything the grove
binary does not carry is logged in the section of the grove release it lands
before, prefixed with the component it touched."* Every change this grove has
made is in that category — none of it is carried by the binary — and none of it
has been logged.

`v16.2.0` is **already cut and tagged** (`chore: release v16.2.0`, 17 commits
back at the time of writing; `Cargo.toml` reads `16.2.0`), so the top `##
v16.2.0` heading is closed history, not an accumulating section. There is no
`## Unreleased` heading and this repo has never used one — the release cut has
always written the section. That is the thing to decide: introduce an
`## Unreleased` heading, or leave the entries to be written at the next
`cargo release`. Seven commits in this grove declined to invent one unilaterally,
which is why this is a leaf rather than something `k8` did inline.

Unlogged so far, all `linkuistics`- or repo-level rather than binary:

| Change | Commit |
|---|---|
| `using-codebase-memory` skill — the whole SKILL.md | `skill-k2`, `skill-integrate-k4` |
| registered in the `linkuistics` plugin manifest | `distribution-k5` |
| `install.sh` workspace guard + `install.test.sh` | `install-workspace-guard-k8` |
| review chain / vendor pair become node directories | `chain-as-node-k7` → `chain-node-k9`…`-k11` |

The last row is **grove-binary** work and the only one that fits the versioned
convention directly. `chain-node-k9` already lists `CHANGELOG.md` among the files
it touches, so it may land its own entry first — check before writing, and do not
duplicate it.

## Done when

- The heading question above is decided and the reasoning is in the commit.
- Every row of the table is either logged or explicitly recorded as deliberately
  not logged.
- Component prefixes follow the preamble's shape — e.g. *"`linkuistics` /
  `using-codebase-memory`: …"*.

## Notes

**Sequenced last on purpose.** It can only be written once the changes are all
in, so it appends after the chain work rather than inserting ahead of it.

**A skills entry names no version** — the preamble says so, and both
`plugin.json` files deliberately declare none (*skills-monorepo*). Do not invent
one.
