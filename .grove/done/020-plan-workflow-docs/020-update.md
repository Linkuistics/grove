# 020-update

**Kind:** work

## Goal
Write `docs/workflows/update.md` — the lifecycle walkthrough for `grove update` in `acme/orders-api`, refreshing an existing materialisation, showing the auto-commit, and naming the ADR-nudge convention.

## Context
- Assumes `010-install.md`'s walkthrough has shipped — this one references the install walkthrough as the starting state instead of re-establishing it.
- ADR-0001 covers the same auto-commit rules for `update` as for `install`; do not restate them in full — link.
- `docs/grove.md` says: "By discipline, record version bumps in an ADR (`docs/adr/`) so the update decision is traceable — `VERSION.md` only carries the current version, not the history." The walkthrough should show this convention being followed.

## Done when
- `docs/workflows/update.md` exists.
- Walks: starting state (post-install), `brew upgrade grove` (or the CLI-upgrade step the user is assumed to have done — verify the actual recommended path against `README.md` / `docs/grove.md`), `grove update` itself, the resulting commit, and a worked example of authoring `docs/adr/NNNN-update-grove-to-vX.Y.Z.md` afterwards.
- The "what changed" panels show `git log --oneline -2` (the update commit on top of the install commit), and a `git show --stat HEAD` of the update commit.
- Covers `--no-commit` and `--message` *only* as a single "the flags work the same as install — see install.md" pointer, not as duplicated subsections.
- Brief note on what happens if there's nothing to update (no-op skip per ADR-0001).
- Short Codex-equivalent callout where the path differs.

## Notes
- The ADR nudge is `eprintln` output from `grove update` itself (per `010-implement-path-scoped-commit.md` notes). Reproduce its exact wording — read `src/install.rs` rather than paraphrasing.
- Do NOT scaffold an ADR for the user. The walkthrough *shows* what writing one looks like — it is documentation of the convention, not boilerplate the CLI emits.
