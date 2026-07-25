# graft-history-k2

**Kind:** work

## Goal

Land the skills repo's tree and its 68 commits in this repo via a two-parent
merge commit, resolving the five colliding root files, and leave the repo green.

## Context

This repo is **jj-native** — a `.jj/` with no colocated `.git/`. The skills
remote is plain git. Consult `linkuistics:using-jujutsu` before touching VCS.

Sketch (verify against the skill; do not run it from memory):

```
jj git remote add skills https://github.com/Linkuistics/skills.git
jj git fetch --remote skills
jj new main <skills-head>      # two-parent merge
<resolve the five root files>
```

**Collision set, measured during planning.** Everything not listed collides with
nothing and lands as-is:

| Path | Status |
|---|---|
| `plugins/` (15 linkuistics skills + testanyware) | net-new |
| `.claude-plugin/marketplace.json`, `install.sh` | net-new |
| `docs/adr/symmetric-vcs-rule.md` | net-new — not among this repo's ADR slugs |
| `docs/research/{grove-recommendations,jj-agent-prior-art,skill-repo-prior-art}.md` | net-new — no overlap with the 7 here |
| `.gitignore` · `CHANGELOG.md` · `CONTEXT.md` · `LICENSE` · `README.md` | **collide** |

## Done when

- The merge commit exists with both parents; `git log` / `jj log` shows two roots.
- `git blame` (or `jj file annotate`) on a file under
  `plugins/linkuistics/skills/` attributes lines to skills-repo commits, not to
  the merge.
- The five collisions are resolved to this **minimal** policy — deeper
  reconciliation belongs to `glossary-partition-k3` and
  `docs-reconciliation-k4`, so do not do their work here:
  - `.gitignore` — union of both.
  - `LICENSE` — diff properly (201L vs 191L, both Apache-2.0 with identical
    heads; the delta is likely the appendix/copyright block) and keep one. Note
    that `content/LICENSES/` carries separate third-party attributions
    (mattpocock, addyosmani) whose terms interact with it.
  - `README.md` — keep this repo's, plus a short paragraph noting the repo now
    also hosts the skill plugins. Full rewrite is k4's.
  - `CHANGELOG.md` — keep this repo's; carry the skills entries in verbatim under
    a clearly labelled heading. Policy for future entries is k4's.
  - `CONTEXT.md` — keep this repo's **unchanged**, and move the skills glossary
    to `plugins/CONTEXT.md` **as-is**. Rewriting it is k3's.
- `cargo test` passes and `cargo build` produces both binaries.
- `content/` is byte-identical to before the merge — the graft must not perturb
  what the binary embeds.

## Notes

- Do **not** archive or modify `Linkuistics/skills` in this session; that is
  `cutover-k6`, and it must not happen before this merge is pushed.
- `build.rs` emits a `rerun-if-changed` per file under `content/`. Adding
  `plugins/` does not enter that walk — if the build starts behaving oddly, that
  is a signal something moved into `content/` that should not have.
- If the merge conflicts more widely than the five files above, stop and
  re-measure rather than resolving blind: the collision set was computed against
  skills at `e0ba6f4`, and a wider set means that assumption broke.
