# 030-homebrew-sole-gesture

**Kind:** work

## Goal

Make `brew install grove` the **sole** gesture: it lands the binaries, and the
first `grove do` provisions the skill via 010's on-launch extract — eliminating the
separate `grove install` step. Adjust the formula/release artifact for the embed
model and declare any deps.

## Context

- Formula template: `scripts/templates/grove.rb.tmpl` (renders to `grove.rb`;
  installs `grove`, `grove-llm`, `rmux` into `bin/`). Rendered + published into the
  **separate** `Linkuistics/taps` repo by `scripts/release-build.sh` (renders the
  formula, builds per-target `*.tar.xz`) and `scripts/release-publish.sh` (uploads
  tarballs, copies `grove.rb` into the tap's `Formula/`).
- The release **tarball currently ships `content/`** for the old `grove install`
  fetch. With 010's embed, the binary already carries `content/`, so the tarball no
  longer needs it and `grove install` is no longer a user step.
- Two binaries kept (ADR-0006); the formula's `bin.install` shape is unchanged.

Settle here:
- Does the tarball still carry `content/`? (Default: drop it — the binary embeds it.)
- Does the formula need a `caveats`/post-install describing the one-gesture flow?
- Any deps to declare (e.g. a file-watcher if the 040 loop kill uses `notify`)?

## Done when

- `brew install grove` installs the binaries; a first `grove do` provisions the
  skill with **no** `grove install` needed.
- The formula template + release scripts reflect the embed model (no separate
  `content/` fetch in the user path).
- Any deps are declared; a `caveats` documents the one-gesture flow if useful.

## Notes

- The tap is an **external** repo — coordinate the formula change with the template
  + render scripts in *this* repo; publishing happens at a real release.
- The `grove install` / `fetch` / `extract` / `uninstall` **code** deletion is
  **090**; 030 only stops *relying* on it for distribution.
- **Inert for this grove** until 040.
