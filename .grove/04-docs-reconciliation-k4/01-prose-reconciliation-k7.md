# prose-reconciliation-k7

**Kind:** work

## Goal

Make the human-facing prose — `README.md`, `CHANGELOG.md`, and the statements
under `docs/` — read as one repo's documentation. A reader arriving cold can tell
what the repo holds and which install path they want.

## Context

The graft kept grove's `README.md` and dropped the skills one wholesale, so the
skills tree currently has **no** reader-facing entry point beyond one paragraph in
grove's README. The dropped README's per-skill table (skill / *loads when* /
notes) has no equivalent here and is genuinely useful — it is the only place the
`paths:` vs description trigger distinction is visible per skill.

`CHANGELOG.md` was resolved by appending the skills changelog verbatim under a
closed-record heading. That is a sound *historical* resolution; what is missing is
the forward policy — how a skills-only change gets logged now that the file is
versioned by grove's releases.

## Done when

- `README.md` describes a repo with two components: the grove CLI + methodology
  (brew, binary-provisioned to three harnesses) and the skill plugins (marketplace
  for Claude Code, `install.sh` for codex/gemini). A cold reader can tell which
  install path they want.
- The skills README's skill table survives somewhere sensible, with its "loads
  when" column intact.
- `CHANGELOG.md` states the policy for logging a skills-only change alongside a
  `v<N>.0.0` grove release, and its entry style is consistent above the
  carried-in section.
- The `linkuistics`-as-prerequisite framing in `README.md` and `content/SKILL.md`
  says the true thing: in-repo, but still a separate install.
- `docs/workflows/`, `docs/superpowers/`, `docs/grove.md` and `docs/concepts.md`
  are checked for statements the merge invalidated, and any found are fixed.

## Notes

- Where the skill table lands is the one judgement call: a new `plugins/README.md`
  (discoverable when browsing the plugin tree, but a second entry point) or a
  section in the root `README.md` (one entry point, but a longer README that
  buries the CLI). Pick one and say why in the commit.
- The stale *"Looking for grove? It moved to its own repo"* line died with the
  dropped README — verify it did not survive anywhere before restoring table
  content from that file.
- The one surviving instance of that claim is inside the carried-in changelog
  section, which is a **closed historical record** and correct as it stands. Do
  not edit it.
- Out of scope: the ADR set (`02-adr-set-k8`), including any README or docs
  citation that only breaks because `02` moves a slug.
