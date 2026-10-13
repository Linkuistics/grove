#!/usr/bin/env bash
# Test a native archive/template without changing any keg, tap or release.
set -euo pipefail
IFS=$'\n\t'
root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
# shellcheck source=scripts/release-common.sh
source "$root/scripts/release-common.sh"
[[ $# == 1 ]] || { echo 'usage: release-formula.test.sh NATIVE_ARCHIVE' >&2; exit 1; }
archive="$1"
top="$(basename "$archive" .tar.xz)"
version="${top#grove-v}"
version="${version%-aarch64-apple-darwin}"
[[ "$top" == "grove-v$version-aarch64-apple-darwin" ]] || { echo 'expected native Grove archive' >&2; exit 1; }
assert_archive "$archive" "$top"
scratch="$(mktemp -d)"
trap 'rm -rf "$scratch"' EXIT
extract_exactly "$archive" "$scratch"
mkdir "$scratch/prefix" "$scratch/test"
# Tests never fetch a formula URL. Checksum placeholders are valid fixture
# values; the production release build renders all three real archive hashes.
sha="$(shasum -a 256 "$archive" | cut -d' ' -f1)"
sed -e "s/@VERSION@/$version/g" -e "s/@SHA_[A-Z_]*@/$sha/g" \
  "$root/scripts/templates/grove.rb.tmpl" >"$scratch/grove.rb"
HOMEBREW_NO_AUTO_UPDATE=1 brew ruby "$root/scripts/release-formula.test.rb" \
  "$scratch/grove.rb" "$scratch/$top" "$scratch/prefix" "$scratch/test"
