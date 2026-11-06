# homebrew-delivery-design-k216

## Goal

Amend the specifications and the decision records, so that they describe
Homebrew installation and Ravel delivery as `skill-marketplace-decision-k201`
settled. Answer the questions of the parent brief.

## Context

- The parent brief `homebrew-skill-delivery-k215` states the outcomes and the
  open questions.
- The running log of `skill-marketplace-decision-k201` holds the decisions in
  the human's words. Its section "Route effects" holds the evidence behind
  them.
- `docs/specs/owner-skill-delivery.md` names mise checkouts or a new
  marketplace in these sections:
  - "Pack checkouts", "Native residue" and "Resource lifetime".
  - "The final session and new launches" and "The switch inside finish".
  - "Rollback", "Purge" and "Cutover contract".
- `docs/specs/external-pack-contract.md`: the sections "Distribution" and "The
  checker without a copy".
- `docs/specs/codex-skill-provisioning.md`: the supported delivery routes.
- `docs/specs/pack-membership.md`: the lines about `skill-marketplace-k185`.
  The view `docs/design/external-pack-contract/membership-boundaries.mmd`
  shows the same step.
- `docs/adr/skill-delivery-is-the-users-choice.md`: Grove's formula now
  couples Grove's release to the packs, and the native skill routes retire.
- The tap `~/Development/Delivery/homebrew-taps` holds the formulae of Grove,
  Ravel and TestAnyware.
- The tasks `release:formula:test` and `release:smoke` test Grove's formula
  and the installed layout of its archive.

## Done when

- Each document above agrees with the decisions of k201. `ADR-FORMAT.md`
  governs the rework of the decision records.
- Each question of the parent brief has an answer in a specification or a
  decision record.
- Each of the six seams of the root brief has its place in a specification.
- A change to a recovery rule of the owner design changes
  `docs/models/owner-switch.qnt` first, and `task model:owner-switch` stays
  green.

## Notes

The amended owner design carries the cutover inside finish, so it is
load-bearing. At the end of the session, judge whether it earns a review
chain. Cut that chain with `leaf-insert` before `homebrew-delivery-plan-k217`.

`owner-pack-checkouts-k93` now names Homebrew installation at the level that
k201 settled. Its details follow this design.

## Decisions (running log)

**Evidence (2026-11-06).** The scratch readings are in
`docs/homebrew-skill-delivery-evidence.md`. Homebrew 7.0.12's cleaner sets
modes only under `bin`, `sbin` and `lib`, and its prune step skips `libexec`.
Grove 22.8.0's keg shows it: `bin` files have mode 0555, and `libexec` files
keep 0644. The installer moves the `opt` link only in its link step, after
the install and the cleaner.

Ravel canonicalises a source path that is a link
and records it as `originPath`. Its digest covers paths, types, normalised
executable bits, link targets and contents, and no write bit.

In scratch homes, Claude Code 2.1.319 kept a directory marketplace's path as given. A
SessionStart hook received the `opt` path as `CLAUDE_PLUGIN_ROOT`, so the
plugin loads in place. Codex 0.166.0 canonicalised the marketplace path to
the versioned keg and copied the plugin into its cache. After the link moved
and the old keg was removed, `codex plugin list` failed. A second
`marketplace add` and `plugin add` repaired it.

**Layout (2026-11-06).** Each skill formula installs each Ravel source at
`libexec/skills/<source-id>/` in its keg, as a copy of the source directory.
A native plugin outside a source goes to `libexec/plugins/<plugin>/`. The
marketplace files sit at `libexec/.claude-plugin/marketplace.json` and
`libexec/.agents/plugins/marketplace.json`. The stable path is
`$(brew --prefix)/opt/<formula>/libexec/skills/<source-id>`.

The directory
name is the source ID of the owner design, so a provider can scan
`opt/*/libexec/skills/*`. `share/` was rejected, because Homebrew links it
into the shared prefix and prunes it.

**Identity (2026-11-06).** The accepted identity stays the `resourceDigest`.
A keg source has the digest of a clean plain checkout of the same commit made
with Git's global and system configuration off. The cleaner leaves `libexec`
alone, and the digest ignores write bits. The staging trials read kegs, so the
accepted digest comes from a keg. The Git tree ID decides nothing more: the
switch compares the digests of the installed keg.

**Lifetime (2026-11-06).** A Ravel capture through an `opt` path reads one
complete keg or refuses. An advance therefore needs no quiet interval. A
racing launch refuses as `skill_source_missing` or `skill_source_changed`, and
a rerun captures the new keg.

Claude Code registers by the `opt` path once,
and the plugin survives upgrade and cleanup. Codex needs its two commands
again after each Grove installation or upgrade. That departs from the word
"once" in k201 for Codex only. Whether a Codex session runs the cached hooks
while its marketplace path is missing is unmeasured, and a seam owns it. Pi
links the extension through the `opt` path once.

**Release (2026-11-06).** Grove's record is `plugins/packs.json`, keyed by
source ID, with `url`, `path` and `sha`. It names the five domain packs only.
`check:packs` reads it. TestAnyware and PersonalSkills leave Grove's check,
because Grove's formula does not carry them.

The release build copies each
pack's source directory from the isolated checkout of `check:packs`, and core
from the tagged tree. The archive manifest lists each copied file with its
Git mode. A pack change reaches users as a new record entry, a passing
`check:packs` and `task release:patch`.

**Timing (2026-11-06).** Before Grove's release, the staging trials and the
rehearsal read Grove's six sources from a candidate keg. That is the
production formula's install method over an unreleased archive, in a private
prefix with an `opt`-shaped link. The four other formulae publish before the
staging trials. Installing them in the real Homebrew prefix changes no HOME
and no discovery.

After the release, switch step 4 resolves Grove's six
sources from the real keg. It compares each digest with its accepted digest.
Step 5's pin advance goes, because the release task upgrades the keg.

**The hooks installation (2026-11-06).** Inside finish, after promotion and a
process listing, finish registers `writing-hooks` from Grove's keg. It then
installs `technical-communication-hooks` and disables it. A guard skips the
installation when the registration exists, and stops on any other state. The
prepared plan already holds the disabled plugin as the earlier value of its
cell, and its row as an entry outside the switch set.

An interruption between
the installation and the disabling reads as the existing "changed" state. No
state, record or inverse changes, so `owner-switch.qnt` needs no change.

**Rollback (2026-11-06).** The inverse stays the return while the backup and
the frozen plugin caches exist. After purge, no native return exists. The
owner repairs the Ravel route, and a session opened directly can read the
folders that `ravel skills resolve` lists.

**Native users (2026-11-06).** After finish, Grove's
`.claude-plugin/marketplace.json` keeps the name `linkuistics` and lists no
plugin. Its description points to Homebrew and Ravel. `claude plugin validate`
accepts an empty listing with one warning, and the listing refreshes without
an error.

 Deleting the file was rejected, because each automatic update would then fail.

The release notes
give the move: install Grove, configure Ravel's sources, register the hooks,
then remove the marketplace. A seam measures what a refresh of the retired
listing does to an installed plugin.

**Frozen registration (2026-11-06).** Nothing in finish refreshes or updates
`linkuistics`. Each command names the new marketplace. No command runs
`claude plugin marketplace update` without a name.

**Names (2026-11-06).** Grove's keg names both marketplace files
`writing-hooks`, so the Codex identity stays
`technical-communication@writing-hooks`. The personal keg names its file
`personal`, so the plugin is `guardrail@personal`. WritingSkills removes its
root Codex marketplace file.

**Other formulae (2026-11-06).** The formulae of personal, superpowers and
quint-llm-kit pin a GitHub source archive by `sha256`, because mise cannot
evaluate a formula without one. Each keg copies the delivered subtree at its
repository path, so the owner's roots and mandatory path stay. TestAnyware's
release archive carries `plugins/testanyware` at `libexec/skills/testanyware`.

**Wording (2026-11-06).** `authoring-conventions` says "the Linkuistics skill
packs" in place of "this marketplace". Its claim that `install.sh` reads
`harnesses:` becomes that no installer reads the key.

**Records (2026-11-06).** The decision record becomes
`homebrew-installs-skills-and-ravel-delivers-them`. The installer goes, with
its `--pack` option. The pack plugin manifests and the `harnesses:` key stay
as metadata that no route installs.

**The skills directory (2026-11-06).** The human wrote: "Let's make
~/.config/ravel/skills/ the installation directory for skills". Ravel 2.0
refuses a source whose real path lies inside `~/.config/ravel`. A link there
into a keg resolves, and a copied directory refuses as
`skill_access_unavailable`. The human chose "Links into kegs", and added: "I
wouldn't save the skills dir I don't think".

**Nothing beyond Homebrew (2026-11-06).** The human then asked how Homebrew
could make those links, and wrote: "My goal is to have nothing more to do
beyond homebrew install/upgrade". Homebrew's post-install sandbox writes only
the keg, its logs and the prefix's link directories. No formula can therefore
write a link or a setting in HOME. The links in `~/.config/ravel/skills/` are withdrawn.

**Ravel's default directory (2026-11-06).** The human chose "Ravel default in
2.0.0" over explicit settings. Each skill formula installs its sources at
`share/ravel/skills/<source-id>`. Homebrew links them into
`$(brew --prefix)/share/ravel/skills/`, and it merges that directory across
kegs. Ravel 2.0.0 reads that directory with no settings.

This grove changes
Ravel's contract before the 2.0.0 cut. The leaf
`ravel-default-skill-directory` designs that change and its effect on the owner
switch.

**Marketplace files (2026-11-06).** Each keg holds its marketplace files at
its root, outside `share/`. Homebrew links nothing from the keg root, so two
kegs cannot conflict over `.claude-plugin/marketplace.json`. Each harness
registers the file by `$(brew --prefix)/opt/<formula>`.

**No review chain yet (2026-11-06).** The Ravel default changes what the owner
switch's native state means, so `ravel-default-skill-directory` amends the
owner design again. A review now would read a design that is about to change.
That leaf judges one review chain over both amendments.
