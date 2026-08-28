# plugin-spine-k16

## Goal

Stand up `plugins/grove/` — the marketplace entry, the shared `grove` spine
skill, and the dependency-free shell conformance runner — beside the binary's
`content/`, which keeps shipping until `delete-provisioning-k19`.

## Context

`docs/specs/module-decomposition.md`, decision 11 and test seam 4.

**The fatness rule**, which this leaf implements the spine half of:

- **In the shared spine**: every rule shared across families — the seven
  constraints, the bootstrap, execution, decomposition, retirement and commit
  procedures, and the four format documents.
- **Inline in `grove-<kind>`**: every rule owned by that kind or its family.
  That is `plugin-kind-skills-k17`'s.
- **Nowhere twice.** `docs/adr/corpus-rules-have-one-owner.md` and
  `docs/adr/restatement-declares-its-class.md` bind unchanged and are what make
  this checkable.

The installation model already exists in this repo twice over: a marketplace
entry in `.claude-plugin/marketplace.json`, and `plugins/install.sh`'s symlink
farm for harnesses without a package manager, where **a skill declares its own
harness eligibility** (`harnesses:` frontmatter) rather than a registry deciding
for it. `plugins/install.test.sh` is the shape the conformance runner follows —
dependency-free bash, isolated `HOME`, testing its own installer.

## Done when

- `plugins/grove/` exists with a `grove` spine skill carrying the shared rules,
  and `.claude-plugin/marketplace.json` lists it as a third plugin.
- The spine's content is **moved** from `content/`, not copied — but `content/`
  is still what the binary provisions, so the two must not disagree while both
  exist. The honest arrangement is that the spine is the source and `content/`'s
  shared files are what will be deleted at `delete-provisioning-k19`; state which
  is which in the plugin's own README.
- `plugins/grove/conformance.sh` (name it as the repo's convention prefers)
  asserts, over the shipped skill set: every behavioural rule is present on the
  **composed loaded path** of every kind that binds it; **no rule has two
  owners**; and **every file a skill names by path exists**. It asserts nothing
  about how many kinds there are.
- The runner runs green on what exists after this leaf — a spine with no kind
  skills yet is a legitimate intermediate state and the runner must say so
  rather than fail.
- `cargo test` and `cargo clippy --all-targets` still clean (nothing in `src/`
  changed); `CHANGELOG.md` updated.

## Notes

**Lands green**, and it removes nothing — this is the **expand** stage of the
methodology's move out of the binary.

**One name collision to handle deliberately, and it is the reason this leaf comes
before `delete-provisioning-k19` rather than after.** The binary provisions
`~/.claude/skills/grove`, `~/.codex/skills/grove` and `~/.pi/agent/skills/grove`;
the spine skill is also called `grove`. On Claude Code the marketplace namespaces
plugin skills, so installing the plugin there does **not** collide. The symlink
farm does: `plugins/install.sh` would want to write the same three paths the
binary owns. So install the plugin under Claude Code here and verify it there;
run `plugins/install.sh` for the other harnesses only at
`delete-provisioning-k19`, once the binary has stopped writing those directories.
If `install.sh` cannot yet refuse to clobber a non-symlink at those paths, make
it refuse — that is a real guard, not a workaround.

**A second, smaller collision to name and answer:** on Claude Code a plugin skill
is invoked as `<plugin>:<skill>`, so the token the prompt must name may not be
the bare `grove-<kind>`. Decide the answer here, while the spine is the only
skill, and write it down where `prompt-names-the-kind-k18` will read it — that
leaf composes the imperative and cannot be left to guess.

**The methodology corpus is part of the work, not a follow-on.** It is roughly
6,300 lines of test-side corpus assertion in the Rust suite today
(`loaded_path_budgets` 1,841, `methodology` 1,254, `session_kind_guidance` 1,076,
`composition_guidance` 798, `reference_navigation` 676, `rule_ownership` 656).
Those move to the shell runner across this leaf and the next; do not delete them
before their assertion exists somewhere.
