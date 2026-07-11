# release-k5

**Kind:** work

## Goal

Ship it: cut the release, **verify the live binary actually prunes**, and close
issue #2.

## Context

- Releases are cut **manually**: `scripts/release-doctor.sh`, `release-build.sh`,
  `release-publish.sh`. There is no GitHub Actions release workflow.
- Distribution is Homebrew (`brew install Linkuistics/taps/grove`), and the binary
  **embeds `content/`**, extracting it to `~/.claude/skills/grove/` on `grove do`
  against a content-hash stamp. So the methodology only reaches a user *through a
  release* — `methodology-k3`'s prose is inert until this leaf runs.
- Issue #2 (`Linkuistics/grove#2`) is this grove's charter and closes here.

## Done when

- Version bumped, `CHANGELOG.md` updated, released via the three scripts.
- **The live binary is verified by behaviour, not by version.** In a scratch grove:
  `leaf-prune` a leaf, confirm the `ABANDONED` file appears, then `leaf-add` and
  confirm the new key is `max + 1` and **not** the pruned one. A green build and a
  bumped version are *not* evidence the feature is wired — that exact trap has been
  hit before in this repo.
- The extracted global skill (`~/.claude/skills/grove/SKILL.md`) actually contains
  the pruning prose — i.e. the content-hash stamp refreshed rather than short-circuiting.
- **Issue #2 closed**, with a comment saying what shipped: the mark, the verb, the
  one-mark decision, and where the durable record now lives.
- Issue #3 (`leaf-insert` on an untracked source) is **left open** — a different
  verb, deliberately not absorbed.

## Notes

If the review leaf (`review-k6`) surfaced accepted trade-offs, name them in the
release notes rather than letting them disappear.
