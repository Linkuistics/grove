# jj-first-coverage-k6

**Kind:** work

## Goal

Close the gap between grove's **jj-first behaviour** and its git-first *tests and
docs*. The code already prefers jj; the test fixtures and the prose do not
reflect that.

## Context

Established during planning by reading the source — re-verify, but this is the
shape:

- `src/repo.rs` is **already jj-first**: `vcs_of` checks `.jj/` before `.git`
  and picks jj plumbing even in a colocated repo; the closest marker walking up
  decides.
- `src/tree_rename.rs` is **already jj-first**: a jj-enabled tree gets a plain
  `fs::rename` (jj snapshots the working copy), falling back to `git mv`
  otherwise.
- Every other `Command::new("git")` in `src/` is a **test helper** — `run_git`,
  `indexed` — building git fixtures.

So there is no behavioural change to make. The two real gaps:

1. **The jj path is untested.** Every fixture constructs a git repo, so the
   branch that is now primary has no coverage — including the plain-rename path
   in `tree_rename.rs` and the `.jj`-wins-over-`.git` precedence in `vcs_of`.
2. **The docs lead with git.** `CONTEXT.md`'s **Grove name** entry derives the
   name from `git rev-parse --show-toplevel` with no jj mention, contradicting
   `repo.rs`. `content/SKILL.md` leads with `git mv` in the renumber
   description. (The `CONTEXT.md` entry is corrected by `01-plan-k1`; check
   whether others remain.)

## Done when

- Test fixtures cover the jj path for VCS resolution and for every tree-mutation
  verb, not just the git path — including the colocated case, where `.jj/` must
  win over an adjacent `.git`.
- No doc or glossary entry describes as git-only something the code decides
  jj-first.
- ADR *symmetric-vcs-rule* still describes the current design, or is reworked in
  place if this leaf changes it.

## Notes

The `linkuistics:using-jujutsu` skill is the reference for jj semantics; the
`git-to-jj-mapping` skill translates specific commands.

Deliberately last in position: it touches no other leaf's work and blocks
nothing. Bring it forward if a jj bug surfaces while doing any earlier leaf.

Watch the `--ignore-working-copy` convention already used in `repo.rs` — jj
probes must stay read-only, or a status probe mutates the working copy it is
inspecting.
