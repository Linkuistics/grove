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

## Decisions (running log)

**Spine membership.** The spine carries the five format documents
(`ADR-FORMAT.md`, `BRIEF-FORMAT.md`, `CONTEXT-FORMAT.md`, `SPEC-FORMAT.md`,
`TASK-FORMAT.md`), the seven shared loop-step references (`bootstrap`, `commit`,
`decompose`, `driver`, `execute`, `grove`, `retire`) and the three **family**
files (`review`, `integrate-review`, `research`), plus the two `LICENSES/` files
those cite. The seven single-kind references, `grilling.md` — reachable only from
`references/requirements.md`, so it is that kind's — and the two `SIGNAL*.md`
files stay in `content/`: the first eight are `plugin-kind-skills-k17`'s, and the
signal files become driver-authored prose at `prompt-names-the-kind-k18`.

**`SKILL.md` is split, not moved.** `content/SKILL.md` is a *kind router* — its
`kind-routing-table` row sends a session to one of ten reference files — and the
live binary still provisions it. The spine's `SKILL.md` is a *condition
register*: it states the same rule in the plugin's shape (the driver named your
`grove-<kind>` skill; you select nothing) and names only files the spine
carries. So the two are two documents and are expected to differ. Every **other**
shared file moves byte-identically, and `conformance.sh` asserts that agreement
until `delete-provisioning-k19` deletes `content/`.

**The name collision is answered by the skill's own declaration, not by a
special case.** The spine declares `harnesses: [claude-code]`, so
`plugins/install.sh` skips it for codex/gemini/pi on its own — those are exactly
the three directories the binary still provisions. It flips to `[any]` at
`delete-provisioning-k19`, when the binary stops writing them. On Claude Code the
marketplace namespaces plugin skills, so there is no collision to answer there.

**`install.sh` now refuses rather than warns.** It already left a non-symlink at
a target path untouched, but only printed `warn` among the run's other lines and
exited 0 — the silent-and-delayed failure its own workspace guard refuses for.
A collision is now an `error`, the run continues so every other skill still
installs, and the script exits non-zero naming the paths and the two ways out.

**The token the prompt must name (this is `prompt-names-the-kind-k18`'s input).**
The prompt names the bare skill name and the Claude Code plugin-qualified
spelling in one imperative: ``Load the `grove-<kind>` skill now — on Claude Code,
where plugin skills are namespaced, that is `grove:grove-<kind>`.`` One
imperative, one target, two spellings — so the micro-test's load-bearing element
survives, grove branches on no harness, and no registry comes back. The durable
statement is in `plugins/grove/README.md`, which outlives `.grove/`.

**The runner reads the inventory as data.** `conformance/rules.tsv` carries all
146 rows of `docs/specs/corpus-rule-ownership.md`'s inventory — rule, owner,
test class, load predicate, and the canonical wording where one is pinned. The
loaded-path assertion covers every row; the single-source sweep covers the 69
with a pinned wording, which is the instrument `tests/rule_ownership.rs` already
had and this leaf moves rather than invents. A row whose owner file is not yet
shipped is reported *pending*, not failed.

**No ADR is amended here.** The spec's reconciliation table assigns
`behavioural-coverage-asserts-delivery`, `corpus-rules-have-one-owner`,
`restatement-declares-its-class` and `grove-binds-without-the-plugin` to "the
leaf that ships the plugin", and `plugin-kind-skills-k17`'s `Done when` claims
all four by name. They are also not yet true: `content/` still ships, the Rust
corpus suites still run, and the register still has two copies. The root brief's
rule — no ADR is rewritten ahead of the code that makes it true — settles it the
same way from the other direction.

**Not a cutover leaf.** Re-derived rather than inherited, by the matrix
`delete-migration-k6` ran: this leaf makes no tree-visible change at all — no
filename, no `.grove/` entry, no grammar. The installed 19.x build meets exactly
the tree it met before, and the plugin is inert to it. So no release, no
Homebrew update, and the session signals normally.

**What "verify it there" could and could not be done in this session.** The task
says to install the plugin under Claude Code here and verify it there. The
`linkuistics` marketplace on this machine is configured from
`GitHub (Linkuistics/grove)`, not from this checkout, so `/plugin install
grove@linkuistics` today would fetch the *published* tree, which does not yet
carry this plugin. Installing is therefore a **push-then-install** step for the
human, not something this session can perform, and claiming otherwise would be
a verification that did not happen.

What was verified: `claude plugin validate plugins/grove` and
`claude plugin validate .` both pass (with the missing-`version` warning
`plugins/README.md`'s *Versioning* section documents as deliberate for every
manifest here); the marketplace entry parses and lists three plugins; the spine's
frontmatter carries `name`, `description` and `harnesses:` exactly as the other
sixteen bundled skills do; and Claude Code's own plugins reference states the
`<plugin>:<skill>` namespacing the token decision rests on.

**The runner's controls are the evidence, not the green run.** Nine cases in
`conformance.test.sh`, each watched coming back **dirty** against a skill set
wrong in one named way — a dangling path, a second owner, a deleted rule, the
condition register restating a procedure, a returned paraphrase, a rule no bound
kind can reach, an unreadable load predicate — plus two that require a clean
reading, so *always fails* is ruled out as well as *always passes*. One fixture
was wrong at first and is written up in the file: severing `SKILL.md`'s pointer
to `references/execute.md` proved nothing, because three other spine files name
that path and the closure simply reached it another way. The control now severs
`references/bootstrap.md`, which `SKILL.md` alone names.
