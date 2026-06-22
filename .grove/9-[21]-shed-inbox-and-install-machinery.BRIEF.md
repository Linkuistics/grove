# 9-[21]-shed-inbox-and-install-machinery — brief

**Kind:** planning (node)

## Goal

Delete the remaining machinery (ADR-0031): the **inbox / `grove-meta` branch**
subsystem and the **install / materialise-into-harness** machinery (incl. the
cli/repo/worktree `VERSION.md` drift model and `grove status`), now replaced by
the global-skill + Homebrew distribution (070). Also sweep the **dead old-verb
modules** the 060 migration left behind (root BRIEF "Dead code to sweep" — folded
in here per its own instruction).

## Context

Read **ADR-0031** (machinery shed) and the **070** brief (the replacement
distribution) first. The inbox/`grove-meta` subsystem is ADR-0002–0005 + 0012 and
the `grove-llm inbox-*` / `grove inbox show` / `grove meta *` verbs; the
install/materialise machinery is `grove install` / `grove uninstall` + the
`VERSION.md` stamping/drift (ADR-0001/0007/0008 + the `cli/repo/worktree version`
glossary) + `grove status`. Both are coordination/distribution that "less in
grove" + the global-skill model make redundant.

## Settled decisions (grilling, 2026-06-22)

This was originally a single work leaf; on contact it proved too big and carried
two open decisions the brief flagged ("raise it rather than deleting blind"). Both
sit *inside* ADR-0031's already-accepted "delete the machinery" decision, so they
are execution confirmations, **not** a new ADR.

- **Inbox → delete, but migrate the seeds first.** Cross-grove coordination *is*
  in live use: 4 seeds on `grove-meta` (`grove-general-improvements`,
  `grove-do-subsumes-…`, `grove-status-should-show-…`, `rmux-web`) and
  `grove-general-improvements` is a live worktree. Per ADR-0031 the subsystem goes,
  but the captured content is re-homed before the verbs are deleted (9.1). The
  built-in cross-grove **capture mechanism** is dropped; future cross-grove notes
  are handled ad hoc / directly in the target grove's tree.
- **`grove status` → delete entirely.** It is built on the dying `VERSION.md`
  drift model *and* counts leaves by walking a `done/` directory the flat scheme
  abolished (already broken for new-format trees). `grove --version` answers the
  version question; the loop derives tree state from `grove-llm pick`. The
  `grove-status-should-show-…` seed is mooted by the shed (one binary, one global
  skill — nothing to show drift for).
- **ADR-0006 survives** (NOT superseded). The `grove`/`grove-llm` audience split is
  *affirmed* by 070; `grove-llm` keeps `pick` / the leaf verbs / `complete`. The
  parent task said "0001–0008" loosely — 0006 is the exception.
- **Latent coupling to fix (9.3):** provisioning (070) writes the skill to the
  *global* `~/.claude/skills/grove/`, but `launch::load_prompt` / `loop_driver`
  still read prompts from the *repo-local* `harness.install_path(repo)`. The
  main-repo mirror was never cleaned, so the loop currently serves **stale**
  prompts. Removing the install machinery forces repointing `load_prompt` to
  `provision::global_skill_dir().join("prompts")`. `harness.rs` survives (struct +
  `HARNESSES` + `detect`/`select`, used by launch/loop_driver); `harness_stamp.rs`
  survives (it stamps *which harness* a grove binds to — `.grove-stamps/<name>` —
  not a version).

## Decomposition

Safety-ordered — re-home the seeds before deleting the inbox verbs; the two big
deletions next; the orthogonal dead-code sweep last:

- `9.1-[30]-migrate-seeds` (work) — re-home the 4 `grove-meta` seeds, then they can
  be removed. Cross-grove prep; produces commits on *other* branches
  (grove-general-improvements) + drains/removes seeds on `grove-meta`, **not** a
  code change on this branch. Must precede 9.2.
- `9.2-[31]-shed-inbox-grove-meta` (work) — delete `inboxes.rs` + `meta.rs` + their
  tests + the `Inbox`/`Meta` commands + `grove-llm inbox-*` verbs + the args
  structs; drop the SKILL.md **Drain** bootstrap step, the "Inboxes and capture"
  section, the `grove-meta` artifact row, and finish-cycle step 4; remove the
  Inbox/Seed/Drain/grove-meta glossary entries; mark ADR-0002–0005 + 0012
  Superseded; remove the `grove-meta` branch + worktree.
- `9.3-[32]-shed-install-and-status` (work) — delete `install.rs`/`fetch.rs`/
  `extract.rs`/`uninstall.rs`/`version_md.rs`/`status.rs` + their tests + the
  `Install`/`Uninstall`/`Status` commands; **repoint `load_prompt` to the global
  skill dir** (the latent fix above); drop `harness.install_path` if it goes dead;
  remove the install-scope / path-scoped-commit / lifecycle-walkthrough / version-
  trio glossary entries and trim "Global skill provisioning" of its VERSION.md
  references; mark ADR-0001/0007/0008 Superseded.
- `9.4-[33]-sweep-dead-old-verb-modules` (work) — delete `pick.rs`/`brief_chain.rs`/
  `root_init.rs`/`leaf_ops.rs` + their old-format integration tests + the dead
  `pub mod` lines; trim `leaf.rs` to keep only `split_prefix` (the old reader
  `migrate.rs` consumes once) and `Kind` (live). Per root BRIEF "Dead code to
  sweep".

## Done when (node)

- All four children retired; `cargo build` + `cargo test` green; `grove`/`grove-llm
  --help` show only the surviving verbs; `lib.rs` has no dead `pub mod`.

## Notes

- Sequence **after** 070 (done) and 040 (done). All prerequisites met.
- The SKILL.md / glossary edits are to **`content/`** (canonical, embedded into the
  binary); they are inert until the binary is rebuilt — correct-at-rest, lands with
  the next release (the 070 inert-until-flip pattern). The 100 leaf must land before
  any release regardless.
- `migrate.rs` itself is shed-able only once no old-format tree can exist; **not**
  in scope here (this grove + grove-general-improvements still flip via it).
