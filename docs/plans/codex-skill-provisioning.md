# Codex Startup Provisioning Implementation Plan

> Implemented with independent filesystem-ownership, technical and editorial
> reviews. The verification record below closes the plan.

**Goal:** Restore offline startup installation and repair of every bundled
Codex-compatible skill while preserving Claude marketplace delivery.

**Architecture:** Generate an embedded plugin inventory at build time. A
binary-owned provisioning adapter stages complete snapshots, reconciles owned
skill links under a shared lock, and refuses collisions before lifecycle launch.

**Constraints:** The contract is `docs/specs/codex-skill-provisioning.md`.
Unix/macOS/Linux; Rust 1.85 floor; no additional runtime dependencies; tests use
isolated homes; no publication. Preserve the existing loop-driver terminal fix.

## Task 1: Specify the process boundary with a failing test

- [x] Add `crates/grove/tests/skill_provisioning.rs`, with a temporary jj
  workspace, personal config, fake configured child, and temporary Codex home.
- [x] Start the real binary and require an installed `grove/SKILL.md` under
  `.agents/skills` before the fake child starts. Verify the bundled Linkuistics
  and Testanyware skills and the absence of Claude-only `guardrail`.
- [x] Run `cargo test -p grove --test skill_provisioning` and observe failure
  from absent installed skills.

## Task 2: Implement delivery and ownership

- [x] Add `crates/grove/build.rs`: recursively enumerate regular plugin files,
  emit `include_bytes!` references and executable flags, select Codex skill
  directories from leading frontmatter, reject duplicate skill names, and
  register Cargo rebuild inputs. Generated constants remain implementation-only.
- [x] Add `crates/grove/src/provision.rs` with a single startup entry point
  `pub(crate) fn ensure_codex_skills() -> anyhow::Result<()>`. Internal functions
  accept explicit destination paths so tests need no process-environment mutation.
- [x] Resolve HOME/CODEX_HOME, detect installed Codex, acquire an installation
  lock, preflight all destinations, compare the complete active snapshot,
  stage missing or changed bytes, and publish the current snapshot atomically.
- [x] Install canonical links; repair recognized Grove links; refresh existing
  legacy links; remove only obsolete owned links. Preserve foreign entries and
  explain collisions. Repeated matching runs do not write.
- [x] Declare the adapter in `main.rs`; call it in `cli.rs` after acquiring the
  driver lease and before `grove_loop::run`.
- [x] Extend process/module tests for repeated runs, damaged snapshots, missing
  and obsolete links, supporting scripts, foreign occupants, legacy links,
  custom Codex home, absent Codex, non-launch commands, and concurrent starts.
- [x] Run focused tests, rustfmt, and clippy; review the resulting filesystem
  protocol against the spec and reconcile review findings.

## Task 3: Deliver a coherent public contract

- [x] Update `scripts/templates/grove.rb.tmpl` caveats: Codex provisions on
  startup; Claude installs the three plugins and enables marketplace auto-update.
- [x] Update README, plugin docs, usage/configuration, architecture, release
  guidance, and current-state references affected by restored provisioning.
- [x] Add Unreleased notes and update the source-exact overview walkthrough,
  its manifest, inventories, and navigation for the new adapter and startup call.
- [x] Run `bash scripts/check.sh` and resolve any integration failures.
- [x] Describe changes through jj, integrate the completed branch onto local
  main, preserve a linear graph, and leave a clean empty working copy.

## Verification record

The full `bash scripts/check.sh` run passed all ten non-book checks, including
workspace tests and the previously failing orphaned-epoch integration test.
All five unaffected books passed. The overview check ran during the final
diagnostic-source update; after its fragments were synchronized, final
validation passed with nine roots, 1,843 source lines and no deferred content.

Final focused verification also passed: five provisioning module tests, seven
startup integration tests, formatting, and all-target Grove clippy with warnings
rejected. Foreign-file diagnostics, legacy directory aliases and destination-local
symlink publication have regression coverage. Actual cross-volume rename was
not exercised on this host; writable test locations share one filesystem.
