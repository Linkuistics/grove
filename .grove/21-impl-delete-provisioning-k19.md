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
- **`src/methodology.rs`, `--content-hash` and the build-pairing report go here**,
  with provisioning — this is the last live build that writes a skill directory,
  and `reverify_installed` is the caller that kept `methodology::identity()` alive
  (`src/provision.rs:53-77`, `src/loop_driver.rs:116-128`). Moved from
  `prompt-names-the-kind-k18`, where deleting them would not have compiled.
- `docs/adr/one-build-owns-a-session.md` **retired** — no build writes a skill
  directory, so there is no pairing to report. True only from this commit onward,
  which is why it is here.
- `docs/adr/skill-delivers-the-methodology.md` **retired** — its delivery path
  ceases to exist.
- **The cost that record existed to prevent is recorded, not argued away.** Grove
  no longer guarantees the methodology is present, so a session can be launched
  pointing at a skill that is not installed. That is a message, not machinery:
  grove states the version it is, names the install command, and stops. The
  provisioned-skill list went with `${locations}` at k18 and the gap is recorded
  where a reader meets it — a harness with a skill-loading affordance is
  unaffected, one without loses its fallback, and the reopen condition is **a
  session that cannot reach the methodology by the affordance alone**.
- **Serving the methodology over MCP is recorded as rejected here**, in the same
  place the registry question is answered by deletion. The spec's `## Out of scope`
  names both, and this is the only leaf whose subject is the delivery path.
- `content/` is deleted, along with the `include_dir` dependency, the embed, the
  content-hash stamp, the staging-and-atomic-rename dance, and `build.rs`'s
  `rerun-if-changed` walk over it. If `build.rs` has nothing left to do, delete
  it and remove it from `Cargo.toml`'s reasoning.
- `tests/provision.rs`, `tests/harness.rs` and `tests/plugin_fallback.rs` are
  deleted or reconciled to what survives.
- **The other harnesses get the plugin here, by a route that works from *this*
  workspace.** `plugins/install.sh` refuses an ordinary run from a secondary jj
  workspace (`plugins/install.sh:113-150`), and this mandated tree is one: the
  default workspace is `/Users/antony/Development/grove`, which does not yet carry
  the plugin. So install with `./plugins/install.sh --force` — the exact case the
  guard's own text sanctions, *testing an unmerged skill live* — and treat the
  consequence as a debt this leaf owns rather than a side effect it ignores: every
  link then points at a disposable tree, and a link whose target disappears reads
  as *skill not installed* rather than as an error. Two obligations follow, both
  written into the commit message: re-run `./plugins/install.sh` from
  `/Users/antony/Development/grove` once this branch lands there, and record that
  repair as a `Done when` of `spec-to-current-state-k23` so the grove cannot finish
  with the links dangling. Remove the stale binary-written directories by hand
  first; they are not symlinks and `install.sh` should refuse to clobber them.
- **The binaries are installed in this session and the session ends without
  signalling** — the root brief's `### The cutover sequence`, run in full. This is
  the leaf where skipping it does visible damage: the *old* driver still runs
  `provision::reverify_installed` before every transition, so a surviving old
  process would rewrite `~/.claude/skills/grove` and its two siblings over the
  symlinks just installed, silently undoing this leaf between iterations. The
  behavioural probe for step 3 is that the new `grove` writes no skill directory:
  delete a stamp file under one provisioned root, run the driver's start path, and
  observe it is not recreated.
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

**Grove's dependency set shrinks here.** `include_dir` goes; `sha2` loses its
last consumer with `methodology.rs` and the content-hash stamp — confirm that
rather than assume it, and delete it if it holds. `tests/library_dependency.rs` holds a claim about the dependency set
grove asks for — reconcile it.
