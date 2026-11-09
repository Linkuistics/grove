# owner-staging-setup-k248

## Goal

Build the retained staging HOME and its sources. Pass the isolation trial
before any other staging trial launches a session.

## Context

- The cutover contract in `docs/specs/owner-skill-delivery.md` puts three
  steps first in the staging trials, in this order:
  1. Build the candidate keg with `task owner:candidate-keg`.
  2. Write the staging settings with the paths of the candidate keg and of
     the four installed kegs.
  3. Run the isolation trial of "Settings isolation and the staging HOME".
- The candidate is `7b113a28a966`, and its `plugins/grove` has the tree
  `885051a94edd`. The section "The handoff to the owner cutover" of
  `docs/repository-moves-record.md` names each source's commit and
  accepted `resourceDigest`.
- TestAnyware 2.8.0 is installed. The formulae of personal, superpowers
  and quint-llm-kit are in the tap and not yet installed. An installation
  changes no HOME and no discovery, so this leaf installs them.
- owner-policy-conversion-k245 supplies `config.rhai`.
  owner-settings-drafts-k246 supplies the staging settings in
  `docs/owner-cutover/settings-staging.json`.
- The staging settings name Grove's six sources at
  `/Users/antony/.local/state/owner-staging/candidate/brew/share/ravel/skills/<id>`.
  Build the candidate keg with that `OUT`:
  `task owner:candidate-keg COMMIT=7b113a28a966
  OUT=/Users/antony/.local/state/owner-staging/candidate RAVEL_2_0_PREFIX=…`.
  Another `OUT` needs a new staging draft and a new run of
  `task owner:settings-drafts`. The staging HOME must not contain a source,
  and no source may contain it. A sibling such as
  `/Users/antony/.local/state/owner-staging/home` meets that rule.
- The parent brief withdrew the decoy-helper control and the checkout HOME.
  They served the mise clones.
- The simplified switch has no rehearsal admission. The staging HOME needs
  no store of rehearsal records. The idle-parent measurement of
  owner-staging-hooks-k251 and the switch dry run reuse this staging HOME
  and its sign-in contract.

## Done when

- Homebrew installed `personal`, `superpowers` and `quint-llm-kit` from the
  tap. Each keg source gives the accepted `resourceDigest` of the moves
  record, as do the TestAnyware source and the candidate keg's six sources.
- The staging HOME and the candidate keg are at absolute paths that stay
  until the switch receipt. Choose paths outside the scratchpad, and record
  them for the plan's `accepted.staging`.
- The staging HOME holds its own `.claude`, `.claude.json` and `.codex`
  tree, `config.rhai` and the staging settings. Its harness configuration
  is a prototype of the native residue. The skill-free plugins are enabled,
  and no superseded skill is installed. The owner's MCP entries are present
  in both harnesses. Claude Code registers the `personal` marketplace from
  the installed keg and holds `guardrail` installed.
- The isolation trial's five steps pass. The only links in the staging HOME
  are `Library/Keychains` and `.codex/auth.json`. A token refresh keeps the
  real `auth.json` a link, and `codex login status` still passes in the
  real HOME. The staging HOME receives credentials by that narrow sign-in
  contract only.
- The evidence document records each path, digest and result, with the
  Claude Code and Codex versions.

## Notes

Do not register `writing-hooks` here. owner-staging-hooks-k251 measures
that installation beside a signed-in session.

If a refresh replaces the `auth.json` link with a file, stop. The owner
then chooses a separate sign-in inside the staging `CODEX_HOME`.

Use the accepted 2.0 build that `scripts/harness-trials/builds.json` names.
Each staging launch sets `CODEX_HOME` to the staging `.codex` tree and
inherits none of Claude Code's five location variables.

## Decisions (running log)

- Homebrew installed `personal` 2026-11-06, `superpowers` 7.0.0 and
  `quint-llm-kit` 1.0.0 with automatic update, cleanup and dependent
  upgrades off. `ravel` 1.1.0 and `grove` 22.8.0 stayed unchanged.
- The retained paths live under `/Users/antony/.local/state/owner-staging`:
  `candidate` (the candidate keg), `ravel-2.0` (the accepted 2.0 build,
  `bin/ravel` SHA-256 `3a9a9129…`) and `home` (the staging HOME).
- Finding: the second skill formula's link made Homebrew relink TestAnyware
  with `{ :mkpath }` (`Keg#resolve_any_conflicts`). Every directory became
  real and every file a link that leaves the source. Ravel refused the
  source as `skill_resource_invalid`. `brew unlink testanyware && brew link
  testanyware` restored one directory link. The keg-layout claim in
  `docs/specs/codex-skill-provisioning.md` is wrong for the first skill
  formula on any machine. A separate leaf owns that design repair.
- The staging HOME copies the owner's installed `config.rhai`
  (`0ccd5cdaa954…`), not the committed candidate. The owner changed it on
  2026-11-08, and owner-switch-plan-k254 reads that digest.
- Staging sessions run headless through a trial selection script, as the
  real trials do. Codex runs `gpt-6.1-sol` at low effort, because Codex
  refused `gpt-6.1-luna` for a ChatGPT account.
- Step 5 forced the refresh with app-server `account/read` and
  `refreshToken: true`. Codex 0.167.0 resolves the link and replaces the
  real target, so the link survives. Both refreshes kept it.
- The first run listed the stale `grove` because Claude Code reads
  `.claude/skills` in each parent folder of its working directory. The
  specification and the launcher now require a working directory outside
  the real HOME.
- npm's `.bin` links of the Playwright `npx` package resolve inside the
  staging HOME. The specification's step 1 now requires that only the two
  sign-in links lead out, and lists internal links with their owner. This
  is a contract clarification for the owner to confirm.
