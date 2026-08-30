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
session kind, say so explicitly there. Presence is checked
[per kind, when the kind is used](CONFIGURATION.md#when-a-missing-kind-is-reported),
so an added kind does not break every configuration on upgrade — it stops the
first task of that kind, in the middle of a workstream, and the release note is
what lets an owner get ahead of it.

Then run the normal checks. For the complete repository suite — `--workspace` on
both cargo lines, because this root is *also* a package and a bare invocation
tests and lints `grove` alone, leaving `grove-loop` and `grove-llm` unread:

```sh
cargo fmt --all --check
cargo test --locked --workspace
cargo clippy --workspace --all-targets
bash plugins/install.test.sh
bash plugins/grove/conformance.sh
bash plugins/grove/conformance.test.sh
shellcheck plugins/install.sh plugins/install.test.sh \
  plugins/grove/conformance.sh plugins/grove/conformance.test.sh
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

### If an agent session's classifier refuses `cargo release`

`release.toml` records that refusal is not inherent to these invocations — it
depends on the session's own harness and permission configuration — and says to
fall back to the constituent commands. For the publish step those are named in
§3; for the cut they are below. They were used for the v19.0.0 cut, when
`cargo release major --execute --no-confirm` was refused.

Do the three edits `cargo release` would have made, then let jj and git make the
two artifacts it would have created:

```sh
# 1. the version, in both manifests
#    edit Cargo.toml's `version = "<old>"` → "<new>", then:
cargo check                                  # rewrites Cargo.lock's grove entry
# 2. close the changelog heading, exactly as pre-release-replacements would:
#    insert `## v<new>` two lines under the standing `## Unreleased`
# 3. the release change, the bookmark, and git's HEAD
jj describe -m "chore: release v<new>"
jj bookmark set main -r @
jj new                                       # parks git HEAD on the release commit
git tag -a v<new> -m "Release v<new>"
git describe --tags --exact-match HEAD       # the precondition release-build.sh checks
```

Verify the change is exactly `Cargo.toml`, `Cargo.lock`, and two added
`CHANGELOG.md` lines before tagging — that is the whole of what the automated cut
produces. The `jj new` is not optional: a colocated `HEAD` follows the working
copy's **parent**, so without it the tag lands on the wrong commit and
`release-build.sh` refuses.

## 3. Build and publish

Build the three platform archives and render their checksums into a Homebrew
formula:

```sh
scripts/release-build.sh
```

The script used to assert, on each staged pair before archiving, that **both**
binaries carried the embedded methodology — scanned rather than run, because two
of the three targets are cross-compiled. `delete-provisioning-k19` deleted the
embed, so neither binary carries a corpus and there is nothing for a binary scan
to assert; the check and its marker phrase went with it.

**What now needs releasing alongside the binaries is the methodology itself.**
It is the `grove` plugin under `plugins/grove/`, versioned by this repository's
commit SHA for Claude Code and by the checkout for `plugins/install.sh`, so it
reaches users when this commit does rather than when an archive is built. A
release that changes a `grove-llm` verb the methodology instructs therefore has
to land both halves in the same commit — which `tests/instructed_verbs.rs`
asserts — and users on an older plugin can still reach a verb skew. Say so in the
changelog when a release moves the verb surface.

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
