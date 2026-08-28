# delete-provisioning-k19

## Goal

Delete provisioning. Grove writes no skill directory, embeds no methodology, and
keeps no harness registry: the methodology installs the way this repo's other
skill plugins already do.

## Context

`docs/specs/module-decomposition.md`, decision 11, and `minimalism-k1`'s
`## Deletion list`:

- *Contained* row 3: `src/harness.rs` (48 lines) — **no surviving caller**; its 6
  sites are all in `provision.rs`, which dies with it.
- *Reconciled* row 1: `src/provision.rs` (485 lines), **5 surviving sites**, each
  with its replacement already decided — `launch.rs:14` `provision_installed` →
  gone; `loop_driver.rs:125-126` `reverify_installed` /
  `report_absent_skill_destination` → gone; `llm_cli.rs:453`
  `warn_on_foreign_skill_dirs` → gone, because there are no foreign directories
  once grove writes none; `loop_driver.rs:238` `installed_skill_dirs` → gone with
  `${locations}`, already removed at `prompt-names-the-kind-k18`.

`minimalism-k1` also settled the alternative: **MCP was cut.** It appears nowhere
in this repo, it would not remove provisioning but change what is provisioned,
and `harness.rs` is explicit that a registry row is a place to write files, never
a thing to run. A harness registry row for a further harness is answered by
deletion — there is no registry left to hold a row.

## Done when

- `src/provision.rs` and `src/harness.rs` are gone, with all five surviving sites
  removed rather than rewired.
- `content/` is deleted, along with the `include_dir` dependency, the embed, the
  content-hash stamp, the staging-and-atomic-rename dance, and `build.rs`'s
  `rerun-if-changed` walk over it. If `build.rs` has nothing left to do, delete
  it and remove it from `Cargo.toml`'s reasoning.
- `tests/provision.rs`, `tests/harness.rs` and `tests/plugin_fallback.rs` are
  deleted or reconciled to what survives.
- **The other harnesses get the plugin here.** Run `plugins/install.sh` so the
  symlink farm covers what the binary used to provision — this is the leaf where
  the name collision `plugin-spine-k16` deferred is finally safe, because nothing
  writes `~/.claude/skills/grove`, `~/.codex/skills/grove` or
  `~/.pi/agent/skills/grove` any more. Remove the stale binary-written
  directories by hand first; they are not symlinks and `install.sh` should refuse
  to clobber them.
- **Verify the loop still launches a session that can reach the methodology**,
  on at least one harness that is not Claude Code, before committing. This is the
  one deletion in the tree that can silently break every subsequent session.
- `cargo test` and `cargo clippy --all-targets` clean; `docs/USAGE.md` and
  `docs/ARCHITECTURE.md` describe plugin installation; `CHANGELOG.md` updated.

## Notes

**Lands green**, and it is a **contract** stage: the plugin is the new form,
shipped at k16 and k17, and this is where the old form goes.

**Do not run ahead of it.** Deleting provisioning before the plugin is installed
on every harness this loop routes to leaves the next session with no methodology
at all, and the failure is quiet — a session that cannot load its skill does not
crash, it improvises.

**Grove's dependency set shrinks here.** `include_dir` goes; check whether
`sha2` still has a consumer once the content-hash stamp is gone, and delete it
too if not. `tests/library_dependency.rs` holds a claim about the dependency set
grove asks for — reconcile it.
