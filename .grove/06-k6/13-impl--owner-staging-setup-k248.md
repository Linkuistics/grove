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
  real HOME.
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
