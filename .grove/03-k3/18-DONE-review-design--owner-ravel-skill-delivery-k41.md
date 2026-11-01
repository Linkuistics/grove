# owner-ravel-skill-delivery-k41

**Reviews:** owner-ravel-skill-delivery-k37

## Goal

Challenge the owner skill delivery design before k34 and the later owner
cutover work rely on it.

## Context

Read the producer's committed result and its running decision log. The
artifacts are `docs/specs/owner-skill-delivery.md`, its evidence document, the
baseline `docs/owner-skill-inventory.tsv`, the instrument under
`docs/evaluations/owner-skill-delivery/`, the owner views in the harness
presentation and the reworked `finish-promotes-after-the-grove-release` record.
The baseline describes the owner's machine on 2026-11-02. Rerun the
instrument read-only and reconcile any later change explicitly.

## Done when

- Test the inventory's completeness. Look for a skill discovery root, a plugin
  component or a hook that the instrument does not read. Rerun its control.
- Challenge each disposition, especially `guardrail` as native-only, the
  skill-free technical-communication plugin, and `using-superpowers` as the
  only mandatory file. That file changes Codex behaviour.
- Challenge the pin policy, which gives up automatic updates, and the staging
  HOME's isolation of settings and sign-in.
- Challenge the switch inside finish. Examine reader evidence, the moment of
  the last 1.1 settings read, plugin disabling during a running session,
  Claude Code's cache sweep and the frozen marketplace. Examine nested probe
  launches from finish and the inverse operation.
- Verify that the k34 requirements are complete and that each needed change is
  real. Judge whether an independently opened session's loss of native skills
  is stated clearly enough for the owner.
- Report supported findings with consequence and the smallest useful
  correction. Follow the review skill for any integration step. Do not
  implement repairs.

## Notes

No installation, owner configuration, plugin change or other-workstream tree
edit. The producer changed nothing outside its checkout.

## Decisions (running log)

- Review scope: producer commit `6b2ec873e3fd3acf6e6cf0cec312b9bf4953f481`
  against current parent `391baf5c8f22`. The only intervening change to its
  principal artifacts repairs the specification's viewer links. This review
  uses the current coordinates. It changes findings and task-tree notes only.
- The mandated read-only inventory rerun produced the exact baseline SHA-256,
  `c9af548bdf8d616437e9f46dfa89dac6f97ac23dfb204c107734f85ab5aba4f0`.
  Per-entry comparison found no added, removed or changed row. The requested
  scratch control produced one unclassified skill and one dangling entry.
  The script, baseline and four configuration/registry inputs checked before
  and after the run did not change. This reproduces the instrument, not its
  claim to cover every discovery root.
- Supported defects: the cited Codex sign-in recipe exposes native discovery;
  installed Codex plugin roots are omitted; capability changes are invisible
  to the inventory comparison. These require integration before k34 consumes
  the owner contract. No owner setting or installation changes in this review.
- The official Claude Code loading reference gives an orphan-age cleanup rule,
  rather than active-reader protection. This earns a low-severity correction
  to the lifetime premise. Disabling alone is not evidence of cache deletion.
- Cut `owner-ravel-skill-delivery-k48` as the integration immediately before
  k34, carrying this review's handle rather than its findings. Retired k41.
  The common-loading node still has live work, so no parent closes.

## Findings

High means the stated acceptance procedure cannot satisfy its contract.
Medium means the design can approve an incomplete or changed installation.
Low means a documented guarantee needs a narrower premise. Each finding gives
the consequence and the smallest useful correction. None is an implementation
instruction for this review session.

### 1. The cited Codex sign-in recipe defeats staging isolation (high)

Location: `docs/specs/owner-skill-delivery.md:245`, "Settings isolation and the
staging HOME", and item 8 at line 369.

The staging HOME must have its own harness settings and no superseded native
skills. It is also told to reuse Ravel's documented sandbox-home technique.
The cited Rhai source is concrete:
`Ravel.coordinated-release-design/docs/specs/configuration-evaluation.md:710`
names `.codex` as Codex's linked sign-in store, and
`scripts/harness-smoke.sh:64` executes `codex) link_store .codex`.
That links the entire real `.codex` tree, including `config.toml`, hooks,
plugins and `skills`, into the staging HOME. This machine has superseded
Grove and superpowers entries in that tree.

Following that recipe exposes native copies in a replacement-only trial.
Writing the proposed staging configuration through the link would also write
the real configuration. The recipe isolates Ravel's settings, but it does not
isolate the harness configuration that this design needs to change.

Smallest useful correction: specify a separate Codex configuration tree and
an authentication-only reuse route, measured with the installed harness.
Do not link the whole `.codex` directory. Record every linked path, check both
Codex user skill roots and configuration overrides, and show that staging
configuration writes leave the real files unchanged. Make k34's item 8 require
this narrower recipe rather than preservation of the existing smoke recipe.
The macOS Claude recipe links `Library/Keychains`, so it does not have this
particular whole-configuration defect.

Codebase-design assessment: authentication and mutable harness configuration
have different owners, but the selected directory link joins them. Keep the
sign-in reuse seam below the staging configuration boundary.

### 2. Installed Codex plugin skills are outside the inventory (medium)

Location: `docs/evaluations/owner-skill-delivery/inventory.py:170` and
`docs/owner-skill-delivery-evidence.md`, "Discovery roots" and "Method".

The instrument scans two fixed Codex marketplace trees:
`~/.codex/.tmp/bundled-marketplaces/openai-bundled` and
`~/.cache/codex-runtimes/codex-primary-runtime/plugins/openai-primary-runtime`.
It never scans `~/.codex/plugins/cache`.

On this machine that omitted root holds installed skill files for Pages,
Sites, Work Pets and plugin management under `openai-curated-remote`. For
example, the current session exposes
`~/.codex/plugins/cache/openai-curated-remote/pages/0.1.26/skills/write-page/SKILL.md`.
The corresponding installed paths exist, but none of those four plugin names
has a baseline row. This is an observed omission, not an inference from an
empty graph query. The byte-identical rerun and the scratch-root control both
pass despite it.

Consequence: k38 cannot reconcile the full installed collection against this
baseline. A new skill or name collision in an omitted root is also invisible
at the final switch's comparison gate. These entries may properly remain
platform-managed, but they still need an explicit inventory and disposition.

Smallest useful correction: discover and enumerate the installed Codex plugin
roots, distinguish cached from active entries, and classify each entry without
altering platform-managed resources. Reconcile the baseline and evidence.
Add an omitted-root control; the existing control exercises only a root that
the caller explicitly supplies.

### 3. The comparison gate cannot see changed hooks or capabilities (medium)

Location: `docs/evaluations/owner-skill-delivery/inventory.py:192`, its retained
storage/tool rows at line 226, and
`docs/specs/owner-skill-delivery.md:285`.

The switch promises to stop for classification on any new or changed entry.
For user hooks, the instrument instead emits one row per `(event, guessed
owner)`, with no content digest. Additional hooks with that same pair are
discarded. A changed matcher, command, argument, timeout or hook script can
therefore leave the TSV unchanged. MCP rows retain only server names, and
agent rows retain only filenames. The executable versions and Quint clone
revision at lines 229–233 are literals rather than fresh observations.
For skill trees, `digest` also omits executable permission bits and directory
symlink targets.

Consequence: a byte-identical inventory does not establish that the native
capabilities and resources rehearsed earlier are still the ones finish will
keep or restore. A changed same-name MCP command, a second hook from one
installer or a lost executable bit can pass the stated gate. The current
scratch control tests unknown Claude skill discovery, not these changes.

Smallest useful correction: define the change identity for each entry class.
Enumerate individual hooks and record non-secret fingerprints of their
definitions and owned scripts; fingerprint relevant agent and plugin
components; preserve resource mode/link evidence; obtain fresh tool and
repository identities. Represent unverifiable inputs as needing review.
Keep credentials and raw configuration values out of the public baseline.
Extend the controls to show that a same-name capability change makes the gate
fail.

Codebase-design assessment: the inventory is the enabling point for native
deactivation, but its output hides less state than its caller assumes.
Either deepen that interface to cover the promised comparisons or require
separate, explicit evidence for the omitted state before activation.

### 4. Cache retention is time-based, not reader-based (low)

Location: `docs/specs/owner-skill-delivery.md:239`, "Resource lifetime", and the
final-session view's statement that retained bytes preserve lazy reads.

The specification says Claude Code sweeps cached versions that no session
uses. The current [official loading reference](https://code.claude.com/docs/en/plugins/loading#cleanup-of-previous-versions)
instead describes an `.orphaned_at` marker on update or uninstall and background
removal after 14 days. It states no active-reader check. Disabling a plugin is
distinct from updating or uninstalling it, so this does not establish that
the proposed disable operation itself erases finish's guidance.

The inspected current Grove, Linkuistics and superpowers cache directories
have no orphan marker. The design's frozen marketplace and retained install
registrations can preserve that useful state. However, "a session still uses
it" is not the stated platform protection, and an orphaned absolute path is
not protected merely by keeping a backup elsewhere.

Smallest useful correction: describe the actual cleanup rule and make the
finish reader evidence name its exact loaded cache paths and registration/
orphan state. Keep those versions registered and prevent replacement or
uninstall until the last reader ends, or provide a verified retention route
that preserves the paths the reader actually uses. Rehearse the lazy read
after disabling and after the nested probe starts. Do not claim a session-aware
cache guarantee from a successful short trial.

## Supported choices and limits

- Keeping `guardrail` native is supported by its user-invoked command and
  frontmatter hook. The technical-communication split has a concrete native
  capability to preserve; its skill-free plugin discovery trial is already
  assigned. Neither disposition needs a second generic delivery mechanism.
- The pin policy states its lost automatic updates, deterministic recovery
  benefit and owner pin-advance responsibility. The mandatory superpowers
  file deliberately changes Codex behaviour. The independent-session section
  clearly states the loss of superseded native discovery. These are visible
  trade-offs within the settled requirements, not new approval requests.
- Historical Ravel supplies the first five k34 requirements. The user-invoked
  metadata rule is absent from its loader and is real new work. Ravel 1.1's
  `Settings::read` rejects unknown keys as `settings_invalid`; the last-1.1-read
  ordering is supported. Item 8 needs finding 1's correction.
- Ravel's nested-session contract gives a child its own result context while
  retaining the parent's. The current owner launch is unconfined. No general
  nested-Ravel prohibition supports a finding here. Probe environment,
  live-disable behaviour, rollback and real harness reads still need the
  rehearsals already assigned by the design. This review launched no harness,
  changed no plugin, and ran no test, build, lint or format suite.
- The finish ADR gives the switch a concrete owner after publication and
  records the rejected earlier-switch and post-finish alternatives. The
  extension itself is coherent; the evidence and staging defects above do not
  establish that the release order must change.

## Review evidence

The review used Verify-tier graph discovery for the inventory and Ravel's
settings reader, exact source, and direct reads for non-code artifacts and
machine installation paths. Grove generation `2026-11-01T20:44:50Z` reported
matching metadata for the inventory and principal Markdown artifacts. The TSV
was not tracked by the graph and was read directly. The owner class/cutover
Mermaid files had recorded parse gaps covering their full bodies; both were
read directly. The graph's `walk_root` trace also included an unrelated Rust
`get` edge, which was not used as source evidence.

Ravel's default workspace generation `2026-11-01T07:59:38Z` covers
`crates/ravel/src/settings.rs`; its excluded documentation was read directly.
The Rhai sandbox-home recipe was read at committed input
`2a77ffbda3b2510f0997c740c0642b18afaeece7` in the isolated coordinated-release
workspace named by the producer's inputs. No graph-completeness claim is made
for that separate workspace or the owner's installation directories.

The read-only rerun's per-entry result and controls are recorded in the running
log above. Its TestAnyware positive control found the readable Claude plugin
and both Codex copies. The dangling Pi entry has no readable identity; its
`entry` still records TestAnyware. No current machine drift explains findings
1–3: they concern the committed procedure and instrument.
