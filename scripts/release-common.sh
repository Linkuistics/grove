#!/usr/bin/env bash
#
# Shared release-pipeline definitions. SOURCED, never executed:
#
#   source "$(dirname "${BASH_SOURCE[0]}")/release-common.sh"
#
# Holds the two things release-doctor.sh and release-build.sh must agree on —
# the target list, and which Rust toolchain the build gets. They used to agree
# by comment ("must stay in sync"), and on the toolchain they did not agree at
# all, which is how a green doctor shipped a dying build twice.

if [[ -n "${GROVE_RELEASE_COMMON_SOURCED:-}" ]]; then
  return 0
fi
GROVE_RELEASE_COMMON_SOURCED=1

# The single source of truth for what a release builds. The native host target
# is listed explicitly even though rustup usually auto-installs it — an explicit
# check survives a partial rustup setup.
# shellcheck disable=SC2034  # consumed by the scripts that source this file
TARGETS=(
  aarch64-apple-darwin
  aarch64-unknown-linux-gnu
  x86_64-unknown-linux-gnu
)

# shellcheck source=scripts/pack-record.sh
source "$(dirname "${BASH_SOURCE[0]}")/pack-record.sh"

# The skill sources of the archive. docs/specs/codex-skill-provisioning.md,
# "The pack record and the release", states them. Core is plugins/grove of the
# tree that the release builds. Each pack is the source directory of a member
# of the pack record, in the plain checkout of its sha that check:packs uses.
# These are shell variables that this file sets and never reads from the
# environment. release.test.sh points them at fixtures after it sources this
# file.
ARCHIVE_CORE_REPO="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
ARCHIVE_PACK_RECORD="$ARCHIVE_CORE_REPO/plugins/packs.json"
ARCHIVE_PACK_CACHE="$ARCHIVE_CORE_REPO/target/external-packs"
readonly ARCHIVE_CORE_PATH=plugins/grove
readonly ARCHIVE_CORE_ID=grove-core
# The keg layout: Homebrew links each keg's share/ into the prefix, so each
# source appears in Ravel's default skill directory.
readonly ARCHIVE_SKILLS=share/ravel/skills

# The fixed part of the archive manifest: one `<mode><TAB><path>` line for each
# file, with its Git mode. Grove's executables and its readable caller assets.
# Ravel is a separate runtime dependency, and the caller assets keep their
# complete readable tree and relative imports.
archive_fixed_manifest() {
  tr ' ' '\t' <<'EOF'
100644 LICENSE
100644 README.md
100755 bin/grove
100755 bin/grove-llm
100644 libexec/ravel-policy/README.md
100644 libexec/ravel-policy/examples/grove-review.rhai
100644 libexec/ravel-policy/examples/grove-review.ts
100644 libexec/ravel-policy/examples/grove-static.rhai
100644 libexec/ravel-policy/examples/grove-static.ts
100644 libexec/ravel-policy/examples/review.rhai
100644 libexec/ravel-policy/examples/review.ts
100644 libexec/ravel-policy/examples/static.rhai
100644 libexec/ravel-policy/grove.rhai
100644 libexec/ravel-policy/grove.ts
100644 libexec/ravel-policy/provenance.rhai
100644 libexec/ravel-policy/provenance.ts
100644 libexec/ravel-policy/sample/config.rhai
100644 libexec/ravel-policy/sample/policy.ts
100644 libexec/ravel-policy/typecheck/review.ts
100644 libexec/ravel-policy/typecheck/static.ts
EOF
}

# git_mode PATH: the Git mode of the file or link at PATH. 120000 is a link,
# 100755 a regular file that its owner can execute and 100644 another regular
# file. Anything else prints `other`.
git_mode() {
  if [[ -L "$1" ]]; then
    echo 120000
  elif [[ -f "$1" && -x "$1" ]]; then
    echo 100755
  elif [[ -f "$1" ]]; then
    echo 100644
  else
    echo other
  fi
}

# core_source_files: the files of plugins/grove in ARCHIVE_CORE_REPO, one path
# per line, relative to that repository. They are the files that Git tracks
# and the untracked files that no ignore rule matches, where each still
# exists. In the clean tagged tree of a release, these are the files of the
# tagged tree. In a working copy, they are the files that a commit of it would
# hold. Git's global and system configuration are off, as for a pack checkout.
# A name that holds a newline splits into names that the archive lacks, so the
# manifest check then fails rather than passing.
core_source_files() {
  local listing file
  listing="$(pack_git -C "$ARCHIVE_CORE_REPO" ls-files -z --cached --others --exclude-standard \
    -- "$ARCHIVE_CORE_PATH" | tr '\0' '\n' | LC_ALL=C sort -u)" || return 1
  while IFS= read -r file; do
    if [[ -n "$file" ]] && [[ -e "$ARCHIVE_CORE_REPO/$file" || -L "$ARCHIVE_CORE_REPO/$file" ]]; then
      printf '%s\n' "$file"
    fi
  done <<<"$listing"
}

# pack_source_files DIR PATH: the Git mode, a tab and the path of each file
# that the checkout DIR tracks under PATH, relative to DIR, one per line.
pack_source_files() {
  local listing line
  listing="$(pack_git -C "$1" ls-files -z -s -- "$2" | tr '\0' '\n')" || return 1
  while IFS= read -r line; do
    # `ls-files -s` gives `<mode> <object> <stage><TAB><path>`.
    [[ -z "$line" ]] || printf '%s\t%s\n' "${line%% *}" "${line#*$'\t'}"
  done <<<"$listing"
}

# each_skill_source: one line `<id><TAB><root><TAB><path>` for each skill
# source of the archive. Its files are those under <root>/<path>, and the
# archive holds them at share/ravel/skills/<id>/. Core comes first, then each
# pack in record order. It fails, naming the cause on stderr, when the record
# is unusable or names core's ID, or when a pack has no clean checkout of its
# sha in ARCHIVE_PACK_CACHE. It never makes a checkout. The build makes them
# first, so a missing one here means that the bytes have no recorded source.
each_skill_source() {
  local members status id url path sha dir
  printf '%s\t%s\t%s\n' "$ARCHIVE_CORE_ID" "$ARCHIVE_CORE_REPO" "$ARCHIVE_CORE_PATH"
  members="$(pack_record_members "$ARCHIVE_PACK_RECORD")" || return 1
  if [[ -z "$members" ]]; then
    echo "error: the pack record $ARCHIVE_PACK_RECORD names no pack" >&2
    return 1
  fi
  while IFS=$'\t' read -r status id url path sha; do
    if [[ "$status" != ok ]]; then
      # A bad line holds the reason in the third column.
      echo "error: pack $id in $ARCHIVE_PACK_RECORD: $url" >&2
      return 1
    fi
    if [[ "$id" == "$ARCHIVE_CORE_ID" ]]; then
      echo "error: the pack record $ARCHIVE_PACK_RECORD names $id, which is core's source ID" >&2
      return 1
    fi
    dir="$(pack_checkout_dir "$ARCHIVE_PACK_CACHE" "$id" "$sha")"
    if ! pack_checkout_is_clean "$dir" "$sha"; then
      echo "error: pack $id has no clean checkout of $sha at $dir; run 'task check:packs'" >&2
      return 1
    fi
    printf '%s\t%s\t%s\n' "$id" "$dir" "$path"
  done <<<"$members"
}

# skills_manifest: the archive manifest's line for each file of each skill
# source, with the path that the archive gives it. Core's modes come from the
# working files, and a pack's modes come from Git. It fails as
# each_skill_source does, and when a source holds no file.
skills_manifest() {
  local sources id root path listing file mode
  sources="$(each_skill_source)" || return 1
  while IFS=$'\t' read -r id root path; do
    if [[ "$id" == "$ARCHIVE_CORE_ID" ]]; then
      listing="$(core_source_files)" || return 1
      listing="$(while IFS= read -r file; do
        [[ -z "$file" ]] || printf '%s\t%s\n' "$(git_mode "$root/$file")" "$file"
      done <<<"$listing")"
    else
      listing="$(pack_source_files "$root" "$path")" || return 1
    fi
    if [[ -z "$listing" ]]; then
      echo "error: source $id holds no file under $root/$path" >&2
      return 1
    fi
    while IFS=$'\t' read -r mode file; do
      printf '%s\t%s/%s/%s\n' "$mode" "$ARCHIVE_SKILLS" "$id" "${file#"$path"/}"
    done <<<"$listing"
  done <<<"$sources"
}

# The archive manifest: one `<mode><TAB><path>` line for each file of the
# archive, below its top directory. It fails as skills_manifest does.
archive_manifest() {
  local fixed skills
  fixed="$(archive_fixed_manifest)" || return 1
  skills="$(skills_manifest)" || return 1
  printf '%s\n%s\n' "$fixed" "$skills"
}

# copy_listed ROOT PREFIX DEST: copy each file that stdin names, one path per
# line relative to ROOT and each under PREFIX/, so that DEST/<rest> holds
# ROOT/PREFIX/<rest>. tar keeps each file's mode and each link as a link.
copy_listed() {
  local root="$1" prefix="$2" dest="$3" scratch status=0
  scratch="$(mktemp -d)" || return 1
  if ! COPYFILE_DISABLE=1 tar -C "$root" --no-xattrs -cf - -T - | tar -C "$scratch" -xpf -; then
    status=1
  elif ! mkdir -p "$(dirname "$dest")" || ! mv "$scratch/$prefix" "$dest"; then
    status=1
  fi
  rm -rf "$scratch"
  return "$status"
}

# copy_skill_sources DEST: copy each skill source of the archive to
# DEST/<id>/. Core's files are core_source_files, and a pack's files are those
# that its checkout tracks. It fails as each_skill_source does.
copy_skill_sources() {
  local dest="$1" sources id root path
  sources="$(each_skill_source)" || return 1
  while IFS=$'\t' read -r id root path; do
    if [[ "$id" == "$ARCHIVE_CORE_ID" ]]; then
      core_source_files | copy_listed "$root" "$path" "$dest/$id" || return 1
    else
      pack_git -C "$root" ls-files -z -- "$path" | tr '\0' '\n' | copy_listed "$root" "$path" "$dest/$id" ||
        return 1
    fi
  done <<<"$sources"
}

# The manifest's paths on stdin, as `<mode><TAB><path>` lines, give every file
# and every directory that holds one, directories with a trailing slash.
archive_layout() {
  local mode entry dir
  while IFS=$'\t' read -r mode entry; do
    [[ -n "$entry" ]] || continue
    echo "$entry"
    dir="$entry"
    while [[ "$dir" == */* ]]; do
      dir="${dir%/*}"
      echo "$dir/"
    done
  done | LC_ALL=C sort -u
}

# Pack DIR's one entry TOP into the .tar.xz ARCHIVE, carrying file contents and
# modes only. macOS's bsdtar otherwise adds an AppleDouble `._<name>` member for
# every file with an extended attribute (`com.apple.provenance` marks most of
# them) and pax headers for the attributes themselves. It hides both again when
# it reads, but GNU tar and Homebrew on Linux extract the `._` files beside the
# real ones.
pack_archive() {
  local dir="$1" top="$2" archive="$3"
  COPYFILE_DISABLE=1 tar -C "$dir" --no-xattrs -cJf "$archive" "$top"
}

# Extract ARCHIVE into DIR exactly as stored. bsdtar's tar reader folds
# AppleDouble `._` members into the file they describe unless told not to,
# which would hide the very members `pack_archive` exists to keep out.
extract_exactly() {
  local archive="$1" dir="$2" exact=()
  if tar --version 2>/dev/null | grep -q bsdtar; then
    exact=(--options '!mac-ext')
  fi
  tar -xJf "$archive" -C "$dir" ${exact[@]+"${exact[@]}"}
}

# Extract ARCHIVE and check it holds exactly TOP/, and within it exactly the
# manifest's files and their directories, each with the Git mode that the
# manifest gives it. Prints what is missing, unexpected or wrong and returns
# nonzero. Prints nothing on success. A manifest that cannot be listed, such
# as one whose pack has no checkout, fails the check.
assert_archive() {
  local archive="$1" top="$2" scratch status=0 manifest entry mode actual
  if ! manifest="$(archive_manifest)"; then
    echo "$archive: the archive manifest cannot be listed; see above"
    return 1
  fi
  scratch="$(mktemp -d)"
  if ! extract_exactly "$archive" "$scratch"; then
    echo "$archive: cannot be extracted"
    rm -rf "$scratch"
    return 1
  fi
  if [[ "$(ls -A "$scratch")" != "$top" ]]; then
    echo "$archive: top level is not exactly $top/:"
    ls -A "$scratch"
    status=1
  fi
  local expected actual_layout
  expected="$(archive_layout <<<"$manifest")"
  actual_layout="$(
    cd "$scratch/$top" 2>/dev/null || exit 0
    {
      find . -mindepth 1 -type d | sed 's|$|/|'
      find . -mindepth 1 ! -type d
    } | sed 's|^\./||' | LC_ALL=C sort
  )"
  while IFS= read -r entry; do
    if [[ -n "$entry" ]]; then
      echo "$archive: missing $top/$entry"
      status=1
    fi
  done < <(LC_ALL=C comm -23 <(echo "$expected") <(echo "$actual_layout"))
  while IFS= read -r entry; do
    if [[ -n "$entry" ]]; then
      echo "$archive: unexpected $top/$entry"
      status=1
    fi
  done < <(LC_ALL=C comm -13 <(echo "$expected") <(echo "$actual_layout"))
  while IFS=$'\t' read -r mode entry; do
    local path="$scratch/$top/$entry"
    if [[ -e "$path" || -L "$path" ]]; then
      actual="$(git_mode "$path")"
      if [[ "$actual" != "$mode" ]]; then
        echo "$archive: $top/$entry has Git mode $actual, not $mode"
        status=1
      fi
    fi
  done <<<"$manifest"
  rm -rf "$scratch"
  return "$status"
}

# NO METHODOLOGY PAIRING ASSERTION. Until delete-provisioning-k19 this file
# carried CONTENT_MARKER -- a phrase that existed only in content/ -- and
# assert_methodology_pairing, which grepped each staged binary for it and failed
# the release if either lacked the embed. The check ran here rather than only in
# the Rust suite because a release builds three targets in --release, two of them
# cross-compiled, so a linker or codegen setting that kept the embed out of a
# shipped binary was not something the test-profile pair could speak to.
#
# Neither binary embeds skills now. The content/ marker, the dual-binary pairing
# contract and the later embedded Codex snapshot are all retired.
# crates/grove/tests/native_skill_delivery.rs verifies that startup writes no
# native skill installation.

# Put rustup's shim directory at the FRONT of PATH, so the release build gets a
# coherent rustup toolchain rather than whatever `cargo` happens to resolve to.
#
# WHY THIS SETS RATHER THAN DIAGNOSES. The two Linux targets are rustup-managed;
# Homebrew's `rust` formula ships its own cargo *and* rustc that know nothing
# about them, and on a machine with both installed Homebrew's wins on PATH. The
# result is `error[E0463]: can't find crate for 'std'` several minutes into a
# release build. That has now cost two releases (ship-release-k25,
# observe-mid-turn-live-k31) *despite being known both times* — a remedy the
# operator must remember every time is not a remedy, so the script does it.
# The pin is announced, not silent: the doctor prints the cargo and rustc it
# settled on, and the target check below verifies the outcome rather than
# assuming it.
#
# WHY PATH AND NOT SOMETHING NARROWER. Two narrower remedies were measured and
# rejected:
#   - Resolving `CARGO="$(rustup which cargo)"` and invoking it by absolute path
#     does NOT work. cargo finds `rustc` via $RUSTC or PATH, so rustup's cargo
#     drives Homebrew's rustc and fails identically.
#   - `rustup run <toolchain> cargo …` does NOT work either. It sets only
#     $RUSTUP_TOOLCHAIN and leaves PATH untouched, so on a machine whose PATH
#     lacks the shim directory it resolves Homebrew's binaries and changes
#     nothing.
# Setting $RUSTC alone does work, but leaves a mixed toolchain (one vendor's
# cargo, another's rustc). Prepending the shim directory gives cargo, rustc and
# the cargo-* subcommands from one rustup toolchain, and honours a
# rust-toolchain.toml if this repo ever grows one.
#
# No-op when rustup is not installed; check_rust_toolchain reports the fallout.
pin_rust_toolchain() {
  local shim_dir="${CARGO_HOME:-$HOME/.cargo}/bin"
  [[ -x "$shim_dir/cargo" && -x "$shim_dir/rustc" ]] || return 0
  if [[ "$(command -v cargo || true)" != "$shim_dir/cargo" ]]; then
    PATH="$shim_dir:$PATH"
    export PATH
    hash -r 2>/dev/null || true
  fi
}

# The directory pin_rust_toolchain prefers — also what the doctor reports
# against, so "is this rustup's?" is asked in exactly one place.
rustup_shim_dir() {
  printf '%s\n' "${CARGO_HOME:-$HOME/.cargo}/bin"
}

# The rustc the build will actually compile with. cargo consults $RUSTC first
# and PATH second, so this mirrors cargo's own resolution — asking `rustup`
# instead is what made the doctor green while the build died.
resolved_rustc() {
  if [[ -n "${RUSTC:-}" ]]; then
    printf '%s\n' "$RUSTC"
  else
    command -v rustc || true
  fi
}

# True when $1 (a rustc) has a std for target $2.
#
# `--print target-libdir` is pure path arithmetic off the sysroot: it prints a
# path for any *recognised* target whether or not it is installed. So the probe
# is the directory's existence, not the command's exit status — and that test
# reproduces the E0463 boundary exactly (Homebrew's rustc: dir missing for both
# Linux targets; rustup's: present).
target_std_installed() {
  local rustc="$1" target="$2" libdir
  libdir="$("$rustc" --print target-libdir --target "$target" 2>/dev/null)" || return 1
  [[ -n "$libdir" && -d "$libdir" ]]
}
