# Releasing Grove

Grove releases are published in two repositories:

- `Linkuistics/grove` carries the release commit, version tag, GitHub Release,
  and binary archives.
- `Linkuistics/homebrew-taps` carries the generated Homebrew formula that
  installs those archives.

The release scripts build and connect those pieces. Run them from Grove's
default, colocated Jujutsu workspace, where both `.jj/` and `.git/` exist.

## Prerequisites

Install `cargo-release`, authenticate the GitHub CLI, and keep a clone of the
Homebrew tap at `~/Development/homebrew-taps`:

```sh
cargo install cargo-release
gh auth login
git clone https://github.com/Linkuistics/homebrew-taps.git ~/Development/homebrew-taps
```

Set `GROVE_TAP_DIR` if the tap clone lives elsewhere. The tap checkout must be
clean and on the branch you intend to push. Check all build and publishing
prerequisites before starting:

```sh
scripts/release-doctor.sh
git -C "${GROVE_TAP_DIR:-$HOME/Development/homebrew-taps}" status --short --branch
```

The doctor checks the pinned Rust toolchain, all release targets, Zig,
`cargo-zigbuild`, and GitHub authentication. It installs nothing.

## 1. Prepare the release

Record the shipped behavior under `## Unreleased` in
[`CHANGELOG.md`](../CHANGELOG.md). If the release adds, removes, or renames a
session kind, say so explicitly there. The
[configuration schema](CONFIGURATION.md#the-nineteen-kinds) has no partial
compatibility mode: every complete `~/.config/grove/config.kdl` fails validation
until its owner edits it, so the release note is the only warning users get.

Then run the normal checks. For the complete repository suite:

```sh
cargo fmt --check
cargo test --locked
bash plugins/install.test.sh
shellcheck plugins/install.sh plugins/install.test.sh
```

Describe the finished change and move `main` to it. A change already based
directly on `main` needs no merge commit:

```sh
jj describe -m "<change description>"
jj bookmark set main -r @
```

Inspect `jj status` and `jj log` before continuing. Keep unrelated working-copy
changes out of the release.

## 2. Cut the version

Start an empty change on `main` so `cargo-release`, which drives Git, commits on
the intended parent:

```sh
jj new main
cargo release patch
cargo release patch --execute
```

The first `cargo release` is a dry run. The executed command bumps
`Cargo.toml` and `Cargo.lock`, closes the changelog's `## Unreleased` section,
creates a `chore: release v<version>` commit, and creates the corresponding
`v<version>` tag. Use `minor` or `major` instead of `patch` when appropriate.

Inspect the imported release change, then move and push the bookmark:

```sh
jj log -r 'main::@'
jj bookmark set main -r <release-change>
jj git push -b main
```

Jujutsu does not push tags, so pushing the release tag is the repository's one
intentional Git mutation:

```sh
git push origin v<version>
```

Do not omit the tag. The GitHub Release created below is attached to it.

## 3. Build and publish

Build the three platform archives and render their checksums into a Homebrew
formula:

```sh
scripts/release-build.sh
```

Before archiving each target, the script asserts on the staged pair that **both**
binaries carry the embedded methodology. It scans the binaries rather than
running them, because two of the three targets are cross-compiled and cannot
execute on the building machine. A failure aborts the release rather than
shipping the archive; the phrase it scans for lives in
`scripts/release-common.sh` and is pinned to `tests/provision.rs` by a test
there.

That assertion **inverted** when `grove-llm` began linking the embed. It used to
fail a release when `grove-llm` carried it, because only `grove` extracted
`content/` and `grove-llm` needed nothing but a compile-time identity constant.
`grove-llm` now computes the methodology identity from the embed itself — for
`--content-hash`, and for the per-verb warning about a clobbered skill
directory — so its absence is the fault rather than its presence. The verb that
first made it link the embed, `grove-llm methodology`, is gone; the identity
reason it left behind is independent and survives it.

Inspect `target/dist/`, which should contain three `.tar.xz` archives and
`grove.rb`. Then publish both repositories:

```sh
scripts/release-publish.sh
```

The publish script performs these operations in order:

1. Creates the `Linkuistics/grove` GitHub Release and uploads the archives.
2. Copies `target/dist/grove.rb` to the tap's `Formula/grove.rb`.
3. Commits `grove v<version>` in the Homebrew tap and pushes that repository.

If the first operation succeeds but the tap push fails, do not rerun the whole
script: the GitHub Release already exists. Fix the tap checkout, copy the
generated formula, and commit and push it from the tap repository.

## 4. Verify the installation

Refresh Homebrew and install or upgrade Grove:

```sh
brew update
brew upgrade grove
grove --version
```

If Grove is not installed yet, use `brew install linkuistics/taps/grove` instead
of `brew upgrade grove`.

Confirm that the reported version matches the tag and that the GitHub Release
contains all three archives. For a behavioral smoke test of the installed
binary, use the isolated harness procedure documented in
`scripts/release-publish.sh`.
